use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};

use omegaflow::bsp_reader::daf::{DafFile, RECORD_BYTES};
use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::cdn::upload_asset;
use omegaflow::ephemeris::{
    body_table, chebyshev_polys, extract_granules, state_ssb_multi, write_binary,
    ASTEROID_GRANULE_DAYS, J2000_EPOCH,
};
use omegaflow::fk::FkFile;

const ET_MIN: f64 = -315_538_891_200.0;
const ET_MAX: f64 = 220_898_664_000.0;
const SEGMENTS_PER_BODY: usize = 4;
const ROUNDTRIP_LIMIT_M: f64 = 100.0;
const COMPARE_LIMIT_M: f64 = 1e-6;
const DATA_START_ADDR: u32 = 385;
const CERES_GM_REF: f64 = 6.2628888644e10;
const VESTA_GM_REF: f64 = 1.7288232879e10;

#[derive(Clone)]
struct SegInfo {
    target: i32,
    center: i32,
    frame: i32,
    data_type: i32,
    start_et: f64,
    end_et: f64,
    start_addr: u32,
    end_addr: u32,
    name: String,
}

struct XorShift64(u64);

impl XorShift64 {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn die(msg: String) -> ! {
    eprintln!("spk_split: {}", msg);
    std::process::exit(1);
}

fn read_record(src: &mut dyn Read, buf: &mut [u8; RECORD_BYTES], rec_no: u32) -> usize {
    let mut filled = 0;
    while filled < RECORD_BYTES {
        match src.read(&mut buf[filled..]) {
            Ok(0) => break,
            Ok(n) => filled += n,
            Err(e) => die(format!("reading record {}: {}", rec_no, e)),
        }
    }
    if filled < RECORD_BYTES {
        die(format!(
            "stream ended inside record {}: {} of {} bytes arrived",
            rec_no, filled, RECORD_BYTES
        ));
    }
    filled
}

fn read_record_soft(src: &mut dyn Read, buf: &mut [u8; RECORD_BYTES], rec_no: u32) -> Option<()> {
    let mut filled = 0;
    while filled < RECORD_BYTES {
        match src.read(&mut buf[filled..]) {
            Ok(0) => break,
            Ok(n) => filled += n,
            Err(e) => die(format!("reading record {}: {}", rec_no, e)),
        }
    }
    if filled == 0 {
        return None;
    }
    if filled < RECORD_BYTES {
        die(format!(
            "stream ended inside record {}: {} of {} bytes arrived",
            rec_no, filled, RECORD_BYTES
        ));
    }
    Some(())
}

fn parse_file_record(file_record: &[u8; RECORD_BYTES]) -> (usize, usize, u32) {
    let idword = &file_record[0..8];
    if !idword.starts_with(b"DAF/") {
        die(format!(
            "the stream's first record carries the identifier {:?}",
            idword
        ));
    }
    let locfmt = &file_record[88..96];
    if locfmt != b"LTL-IEEE" {
        die(format!(
            "the stream's first record carries the format {:?}",
            locfmt
        ));
    }
    let nd = u32::from_le_bytes(file_record[8..12].try_into().unwrap()) as usize;
    let ni = u32::from_le_bytes(file_record[12..16].try_into().unwrap()) as usize;
    let fward = u32::from_le_bytes(file_record[76..80].try_into().unwrap());
    (nd, ni, fward)
}

struct SummaryRecord {
    next: u32,
    summaries: Vec<(Vec<f64>, Vec<i32>)>,
}

fn parse_summary_record(bytes: &[u8; RECORD_BYTES], nd: usize, ni: usize) -> SummaryRecord {
    let next = f64::from_le_bytes(bytes[0..8].try_into().unwrap()) as u32;
    let nsum = f64::from_le_bytes(bytes[16..24].try_into().unwrap()) as usize;
    let ss = nd + ni.div_ceil(2);
    let mut summaries = Vec::with_capacity(nsum);
    for i in 0..nsum {
        let off = 24 + i * ss * 8;
        if off + ss * 8 > RECORD_BYTES {
            die(format!(
                "summary record carries {} summaries past the record edge",
                nsum
            ));
        }
        let mut doubles = Vec::with_capacity(nd);
        for k in 0..nd {
            doubles.push(f64::from_le_bytes(
                bytes[off + k * 8..off + (k + 1) * 8].try_into().unwrap(),
            ));
        }
        let mut integers = Vec::with_capacity(ni);
        let int_start = off + nd * 8;
        for k in 0..ni {
            integers.push(i32::from_le_bytes(
                bytes[int_start + k * 4..int_start + (k + 1) * 4]
                    .try_into()
                    .unwrap(),
            ));
        }
        summaries.push((doubles, integers));
    }
    SummaryRecord { next, summaries }
}

fn parse_name_record(bytes: &[u8; RECORD_BYTES], name_chars: usize, nsum: usize) -> Vec<String> {
    let mut names = Vec::with_capacity(nsum);
    for i in 0..nsum {
        let off = i * name_chars;
        if off + name_chars > RECORD_BYTES {
            die("name record carries names past the record edge".to_string());
        }
        let name = std::str::from_utf8(&bytes[off..off + name_chars])
            .unwrap_or("")
            .trim_end_matches('\0')
            .trim_end()
            .to_string();
        names.push(name);
    }
    names
}

fn seg_from_summary(summary: &(Vec<f64>, Vec<i32>), name: String) -> Option<SegInfo> {
    let (doubles, integers) = summary;
    if doubles.len() < 2 || integers.len() < 6 {
        return None;
    }
    Some(SegInfo {
        target: integers[0],
        center: integers[1],
        frame: integers[2],
        data_type: integers[3],
        start_et: doubles[0],
        end_et: doubles[1],
        start_addr: integers[4] as u32,
        end_addr: integers[5] as u32,
        name,
    })
}

fn build_body_daf(
    file_record: &[u8; RECORD_BYTES],
    ss: usize,
    segs: &[SegInfo],
    span: &[u8],
) -> Vec<u8> {
    let mut out = Vec::with_capacity(3 * RECORD_BYTES + span.len());
    let mut fr = [0u8; RECORD_BYTES];
    fr.copy_from_slice(file_record);
    fr[76..80].copy_from_slice(&2u32.to_le_bytes());
    fr[80..84].copy_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&fr);
    let mut summary_rec = [0u8; RECORD_BYTES];
    summary_rec[16..24].copy_from_slice(&(segs.len() as f64).to_le_bytes());
    let delta = DATA_START_ADDR as i64 - segs[0].start_addr as i64;
    let mut names_rec = [0u8; RECORD_BYTES];
    for (i, seg) in segs.iter().enumerate() {
        let off = 24 + i * ss * 8;
        summary_rec[off..off + 8].copy_from_slice(&seg.start_et.to_le_bytes());
        summary_rec[off + 8..off + 16].copy_from_slice(&seg.end_et.to_le_bytes());
        let ints: [i64; 6] = [
            seg.target as i64,
            seg.center as i64,
            seg.frame as i64,
            seg.data_type as i64,
            seg.start_addr as i64 + delta,
            seg.end_addr as i64 + delta,
        ];
        for (k, v) in ints.iter().enumerate() {
            summary_rec[off + 16 + k * 4..off + 20 + k * 4]
                .copy_from_slice(&(*v as i32).to_le_bytes());
        }
        let noff = i * ss * 8;
        let name = seg.name.as_bytes();
        let n = name.len().min(ss * 8);
        names_rec[noff..noff + n].copy_from_slice(&name[..n]);
    }
    out.extend_from_slice(&summary_rec);
    out.extend_from_slice(&names_rec);
    out.extend_from_slice(span);
    out
}

fn eval_granule(granule: &(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>), jd: f64) -> Option<[f64; 3]> {
    let (t0, dt, cx, cy, cz) = granule;
    let x = (jd - t0) / dt;
    if !(-1.0..=1.0).contains(&x) {
        return None;
    }
    let polys = chebyshev_polys(cx.len(), x);
    Some([
        cx.iter().zip(&polys).map(|(c, p)| c * p).sum(),
        cy.iter().zip(&polys).map(|(c, p)| c * p).sum(),
        cz.iter().zip(&polys).map(|(c, p)| c * p).sum(),
    ])
}

fn eval_bin_at(granules: &[(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)], jd: f64) -> Option<[f64; 3]> {
    let idx = granules.partition_point(|g| g.0 <= jd);
    for &i in &[idx.saturating_sub(1), idx] {
        if i >= granules.len() {
            continue;
        }
        if let Some(p) = eval_granule(&granules[i], jd) {
            return Some(p);
        }
    }
    None
}

fn load_gm_catalog(path: &str) -> HashMap<i32, f64> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => die(format!("reading GM catalog {}: {}", path, e)),
    };
    let mut map = HashMap::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let (id_s, gm_s) = match t.split_once('|') {
            Some(p) => p,
            None => die(format!("GM catalog line carries no separator: {:?}", t)),
        };
        let id: i32 = match id_s.trim().parse() {
            Ok(id) => id,
            Err(_) => die(format!("GM catalog line carries no id: {:?}", t)),
        };
        if !(1_000_000..10_000_000).contains(&id) {
            die(format!("GM catalog id {} is not seven-digit", id));
        }
        let gm: f64 = match gm_s.trim().parse() {
            Ok(g) => g,
            Err(_) => die(format!("GM catalog line carries no gm: {:?}", t)),
        };
        if !gm.is_finite() || gm <= 0.0 {
            die(format!(
                "GM catalog id {} carries gm {} outside the positive reals",
                id, gm
            ));
        }
        if map.insert(id, gm).is_some() {
            die(format!("GM catalog id {} appears twice", id));
        }
    }
    if map.len() != 373 {
        die(format!(
            "GM catalog carries {} lines — the IOM table carries 373",
            map.len()
        ));
    }
    let ceres = map[&2_000_001];
    let vesta = map[&2_000_004];
    let rel = |a: f64, b: f64| ((a - b) / b).abs();
    if rel(ceres, CERES_GM_REF) > 1e-3 {
        die(format!(
            "GM catalog ceres {} does not match the IOM crosscheck {}",
            ceres, CERES_GM_REF
        ));
    }
    if rel(vesta, VESTA_GM_REF) > 1e-3 {
        die(format!(
            "GM catalog vesta {} does not match the IOM crosscheck {}",
            vesta, VESTA_GM_REF
        ));
    }
    eprintln!(
        "gm catalog: 373 lines, ceres {:.6e} m3/s2, vesta {:.6e} m3/s2 — IOM crosscheck holds",
        ceres, vesta
    );
    map
}

struct Splitter {
    targets: HashSet<i32>,
    pending: HashMap<i32, Vec<SegInfo>>,
    front_target: Option<i32>,
    front_segs: Vec<SegInfo>,
    span_start: u32,
    span_end: u32,
    buf: Vec<u8>,
    buf_base_addr: u32,
    buf_last_record: u32,
    file_record: [u8; RECORD_BYTES],
    ss: usize,
    carrier: SpkFile,
    gm_map: HashMap<i32, f64>,
    dest: String,
    ci_mode: bool,
    roundtrip_n: usize,
    compare: Option<SpkFile>,
    processed: HashSet<i32>,
    all_targets: BTreeMap<i32, Vec<SegInfo>>,
    total_segments: usize,
}

impl Splitter {
    fn note_summary(&mut self, seg: SegInfo) {
        if seg.target >= 2_000_000 {
            self.all_targets
                .entry(seg.target)
                .or_default()
                .push(seg.clone());
        }
        self.total_segments += 1;
        if !self.targets.contains(&seg.target) {
            return;
        }
        if seg.data_type != 2 {
            die(format!(
                "target {} carries a type-{} segment — the split reads type 2 only",
                seg.target, seg.data_type
            ));
        }
        if seg.center != 10 {
            die(format!(
                "target {} carries a segment centered on {} — the split reads center 10",
                seg.target, seg.center
            ));
        }
        let list = self.pending.entry(seg.target).or_default();
        if list.len() >= SEGMENTS_PER_BODY {
            die(format!("target {} carries a fifth segment", seg.target));
        }
        list.push(seg);
    }

    fn maybe_set_front(&mut self, avail_addr: u32) {
        if self.front_target.is_some() {
            return;
        }
        let complete: Vec<(i32, Vec<SegInfo>)> = self
            .pending
            .iter()
            .filter(|(_, v)| v.len() == SEGMENTS_PER_BODY)
            .map(|(k, v)| (*k, v.clone()))
            .collect();
        let mut best: Option<(u32, i32, Vec<SegInfo>)> = None;
        for (target, mut segs) in complete {
            segs.sort_by_key(|s| s.start_addr);
            let start = segs[0].start_addr;
            if start < avail_addr {
                die(format!(
                    "target {}: segment data begins before address {} — the segment's summary arrived after its data",
                    target, avail_addr
                ));
            }
            let candidate = (start, target, segs);
            match &best {
                None => best = Some(candidate),
                Some((b_start, _, _)) if *b_start > start => best = Some(candidate),
                _ => {}
            }
        }
        if let Some((start, target, segs)) = best {
            let rec_start = (start - 1) / 128 + 1;
            if !self.buf.is_empty() && self.buf_last_record != rec_start {
                self.buf.clear();
                self.buf_base_addr = (rec_start - 1) * 128 + 1;
            }
            if self.buf.is_empty() {
                self.buf_base_addr = (rec_start - 1) * 128 + 1;
            }
            self.front_target = Some(target);
            self.front_segs = segs;
            self.span_start = start;
            self.span_end = self.front_segs[SEGMENTS_PER_BODY - 1].end_addr;
            eprintln!(
                "front: target {} span [{},{}] records [{},{}] avail {}",
                target,
                self.span_start,
                self.span_end,
                rec_start,
                (self.span_end - 1) / 128 + 1,
                avail_addr
            );
            self.pending.remove(&target);
        }
    }

    fn on_data(&mut self, rec_no: u32, bytes: &[u8; RECORD_BYTES]) {
        if let Some(target) = self.front_target {
            let rec_start = (self.span_start - 1) / 128 + 1;
            let rec_end = (self.span_end - 1) / 128 + 1;
            if rec_no >= rec_start && rec_no <= rec_end {
                self.buf.extend_from_slice(bytes);
                self.buf_last_record = rec_no;
            }
            if rec_no == rec_end {
                let segs = std::mem::take(&mut self.front_segs);
                self.extract(target, segs);
                self.front_target = None;
                self.maybe_set_front(self.buf_base_addr);
            }
            return;
        }
        self.maybe_set_front(rec_no * 128 + 1);
    }

    fn extract(&mut self, target: i32, mut segs: Vec<SegInfo>) {
        segs.sort_by_key(|s| s.start_addr);
        if segs.len() != SEGMENTS_PER_BODY {
            die(format!(
                "target {}: {} segments arrived, the stream carries {}",
                target,
                segs.len(),
                SEGMENTS_PER_BODY
            ));
        }
        let (start, end) = (segs[0].start_addr, segs[3].end_addr);
        for w in segs.windows(2) {
            if w[1].start_addr != w[0].end_addr + 1 {
                die(format!(
                    "target {}: segments [{}..{}] and [{}..{}] do not touch",
                    target, w[0].start_addr, w[0].end_addr, w[1].start_addr, w[1].end_addr
                ));
            }
        }
        let byte_off = (start - self.buf_base_addr) as usize * 8;
        let span_len = (end - start + 1) as usize * 8;
        if byte_off + span_len > self.buf.len() {
            die(format!(
                "target {}: {} bytes buffered, the segment span needs {}",
                target,
                self.buf.len(),
                byte_off + span_len
            ));
        }
        let span = self.buf[byte_off..byte_off + span_len].to_vec();
        let tail = self.buf.split_off(byte_off + span_len);
        self.buf = tail;
        self.buf_base_addr = end + 1;
        let (min_et, max_et) = (
            segs.iter().map(|s| s.start_et).fold(f64::MAX, f64::min),
            segs.iter().map(|s| s.end_et).fold(f64::MIN, f64::max),
        );
        if min_et != ET_MIN || max_et != ET_MAX {
            die(format!(
                "target {}: segment window [{:.3}, {:.3}] s does not equal the longbow window [{:.3}, {:.3}]",
                target, min_et, max_et, ET_MIN, ET_MAX
            ));
        }
        let table = body_table();
        let body_name = match table.get(&target) {
            Some(b) => b.name.clone(),
            None => die(format!(
                "target {} carries no row in naif_body_ids.tsv",
                target
            )),
        };
        let gm = match self.gm_map.get(&target) {
            Some(g) => *g,
            None => die(format!("target {} carries no GM line", target)),
        };
        eprintln!(
            "target {} ({}): span [{},{}] ({} B), segments: {}",
            target,
            body_name,
            start,
            end,
            span_len,
            segs.len()
        );
        let daf_bytes = build_body_daf(&self.file_record, self.ss, &segs, &span);
        let daf = match DafFile::from_data(daf_bytes) {
            Ok(d) => d,
            Err(e) => die(format!("target {}: DAF-in-RAM refused: {}", target, e)),
        };
        let body_spk = match SpkFile::from_daf(daf) {
            Ok(s) => s,
            Err(e) => die(format!("target {}: SPK refused: {}", target, e)),
        };
        let all_kernels = vec![body_spk.clone(), self.carrier.clone()];
        let gm_text = format!("BODY{}_GM = ( {:.17e} )", target, gm / 1.0e9);
        let pck_bodies = omegaflow::pck::parse(Some(&gm_text), None);
        let wgccre = match pck_bodies.get(&target) {
            Some(w) => w.clone(),
            None => die(format!("target {}: the GM line built no PCK body", target)),
        };
        let fk = FkFile::parse("");
        let (mut granules, rotations, nutation) = extract_granules(
            &body_spk,
            &all_kernels,
            target,
            &wgccre,
            &[],
            &fk,
            ASTEROID_GRANULE_DAYS,
        );
        if granules.is_empty() {
            die(format!("target {}: no granules fit", target));
        }
        granules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let path = if self.dest.is_empty() {
            format!("ephemeris_{}.bin", body_name)
        } else {
            format!("{}/ephemeris_{}.bin", self.dest, body_name)
        };
        if !write_binary(
            &path, &body_name, &granules, &rotations, &nutation, &wgccre, None,
        ) {
            die(format!("target {}: {} returned void", target, path));
        }
        let mut rng = XorShift64(0x5eed_2026_0821_0219 ^ target as u64);
        let lo = ET_MIN + 86400.0;
        let hi = ET_MAX - 86400.0;
        let mut worst = 0.0_f64;
        let mut worst_compare = 0.0_f64;
        let mut covered = 0usize;
        for _ in 0..self.roundtrip_n {
            let et = lo + rng.unit() * (hi - lo);
            let jd = et / 86400.0 + J2000_EPOCH;
            let Some(bin_pos) = eval_bin_at(&granules, jd) else {
                continue;
            };
            covered += 1;
            let spk_pos = match state_ssb_multi(&all_kernels, target, et) {
                Some(s) => [s[0] * 1000.0, s[1] * 1000.0, s[2] * 1000.0],
                None => die(format!("target {}: no SPK state at et {}", target, et)),
            };
            let d = (0..3)
                .map(|k| (bin_pos[k] - spk_pos[k]).powi(2))
                .sum::<f64>()
                .sqrt();
            if d > worst {
                worst = d;
            }
            if let Some(orig) = &self.compare {
                let orig_kernels = vec![orig.clone(), self.carrier.clone()];
                if let Some(orig_pos) = state_ssb_multi(&orig_kernels, target, et) {
                    let d2 = (0..3)
                        .map(|k| (spk_pos[k] - orig_pos[k] * 1000.0).powi(2))
                        .sum::<f64>()
                        .sqrt();
                    if d2 > worst_compare {
                        worst_compare = d2;
                    }
                }
            }
        }
        if covered == 0 {
            die(format!(
                "target {}: no granule covers a roundtrip epoch",
                target
            ));
        }
        eprintln!(
            "  {}: roundtrip {} epochs, max {} m vs the split stream",
            body_name, covered, worst
        );
        if worst > ROUNDTRIP_LIMIT_M {
            die(format!(
                "target {}: roundtrip {} m passes the {} m gate",
                target, worst, ROUNDTRIP_LIMIT_M
            ));
        }
        if self.compare.is_some() {
            eprintln!(
                "  {}: compare-input {} epochs, max {} m vs the original file",
                body_name, covered, worst_compare
            );
            if worst_compare > COMPARE_LIMIT_M {
                die(format!(
                    "target {}: compare-input {} m does not match the original file",
                    target, worst_compare
                ));
            }
        }
        if self.ci_mode && !upload_asset(&path) {
            die(format!("target {}: {} did not reach the CDN", target, path));
        }
        self.processed.insert(target);
    }

    fn finish(&self) {
        let missing: Vec<i32> = self
            .targets
            .iter()
            .filter(|t| !self.processed.contains(t))
            .copied()
            .collect();
        if !missing.is_empty() {
            die(format!(
                "targets without a segment in the stream: {:?}",
                missing
            ));
        }
        eprintln!(
            "stream: {} targets, {} segments, processed: {:?}",
            self.all_targets.len(),
            self.total_segments,
            self.processed
        );
    }
}

fn read_record_partial(src: &mut dyn Read, buf: &mut [u8; RECORD_BYTES], rec_no: u32) -> usize {
    let mut filled = 0;
    while filled < RECORD_BYTES {
        match src.read(&mut buf[filled..]) {
            Ok(0) => break,
            Ok(n) => filled += n,
            Err(e) => die(format!("reading record {}: {}", rec_no, e)),
        }
    }
    filled
}

fn run_list(src: &mut dyn Read) {
    let mut reader = BufReader::with_capacity(1 << 20, src);
    let mut rec = [0u8; RECORD_BYTES];
    if read_record_partial(&mut reader, &mut rec, 1) < RECORD_BYTES {
        die("the stream carries no full first record".to_string());
    }
    let (nd, ni, fward) = parse_file_record(&rec);
    let ss = nd + ni.div_ceil(2);
    let name_chars = ss * 8;
    let mut rec_no: u32 = 1;
    let mut next_summary: u32 = fward;
    let mut targets: BTreeMap<i32, Vec<SegInfo>> = BTreeMap::new();
    let mut total_segments = 0usize;
    let mut truncated: Option<u32> = None;
    loop {
        if truncated.is_some() {
            break;
        }
        if next_summary != 0 {
            while rec_no + 1 < next_summary {
                let got = read_record_partial(&mut reader, &mut rec, rec_no + 1);
                rec_no += 1;
                if got < RECORD_BYTES {
                    if got > 0 {
                        truncated = Some(rec_no);
                    }
                    break;
                }
            }
            if truncated.is_some() {
                break;
            }
            if read_record_partial(&mut reader, &mut rec, rec_no + 1) < RECORD_BYTES {
                truncated = Some(rec_no + 1);
                break;
            }
            rec_no += 1;
            let summary = parse_summary_record(&rec, nd, ni);
            {
                let mut min_a = u32::MAX;
                let mut max_a = 0u32;
                for (d, i) in &summary.summaries {
                    if d.len() >= 2 && i.len() >= 6 {
                        min_a = min_a.min(i[4] as u32);
                        max_a = max_a.max(i[5] as u32);
                    }
                }
                eprintln!(
                    "summary record {}: {} summaries, next {}, segment addresses [{},{}]",
                    rec_no,
                    summary.summaries.len(),
                    summary.next,
                    min_a,
                    max_a
                );
            }
            if read_record_partial(&mut reader, &mut rec, rec_no + 1) < RECORD_BYTES {
                truncated = Some(rec_no + 1);
                break;
            }
            rec_no += 1;
            let names = parse_name_record(&rec, name_chars, summary.summaries.len());
            for (i, sum) in summary.summaries.iter().enumerate() {
                let name = names.get(i).cloned().unwrap_or_default();
                if let Some(seg) = seg_from_summary(sum, name) {
                    targets.entry(seg.target).or_default().push(seg);
                    total_segments += 1;
                }
            }
            if summary.next == 0 {
                next_summary = 0;
                continue;
            }
            if summary.next <= rec_no {
                die(format!(
                    "summary chain pointer {} does not rise past record {}",
                    summary.next, rec_no
                ));
            }
            next_summary = summary.next;
        } else {
            match read_record_soft(&mut reader, &mut rec, rec_no + 1) {
                Some(()) => rec_no += 1,
                None => break,
            }
        }
    }
    if let Some(rec_no) = truncated {
        eprintln!("list: the stream ended inside record {}", rec_no);
    }
    eprintln!(
        "stream: {} targets, {} segments, {} summaries per target",
        targets.len(),
        total_segments,
        SEGMENTS_PER_BODY
    );
    for (target, segs) in &targets {
        let mut segs = segs.clone();
        segs.sort_by_key(|s| s.start_addr);
        let addr_str: Vec<String> = segs
            .iter()
            .map(|s| format!("[{},{}]", s.start_addr, s.end_addr))
            .collect();
        eprintln!("  target {}: {}", target, addr_str.join(" "));
    }
    let mut contiguous = 0usize;
    let mut full_window = 0usize;
    for (target, segs) in &targets {
        let mut segs = segs.clone();
        segs.sort_by_key(|s| s.start_addr);
        let mut touching = true;
        for w in segs.windows(2) {
            if w[1].start_addr != w[0].end_addr + 1 {
                touching = false;
            }
        }
        if touching {
            contiguous += 1;
        }
        let min_et = segs.iter().map(|s| s.start_et).fold(f64::MAX, f64::min);
        let max_et = segs.iter().map(|s| s.end_et).fold(f64::MIN, f64::max);
        if min_et == ET_MIN && max_et == ET_MAX {
            full_window += 1;
        }
        if segs.len() != SEGMENTS_PER_BODY || !touching || min_et != ET_MIN || max_et != ET_MAX {
            eprintln!(
                "target {}: {} segments, contiguous {}, window [{:.3}, {:.3}]",
                target,
                segs.len(),
                touching,
                min_et,
                max_et
            );
        }
    }
    eprintln!(
        "list: {} targets contiguous, {} with the full longbow window",
        contiguous, full_window
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: spk_split --input <spk> | (stdin) --bodies a,b,c --gm-catalog <φ> --carrier <de441.bsp> [--dest <dir>] [--ci-mode] [--roundtrip N] [--compare-input <spk>]");
        eprintln!("       spk_split --input <spk> | (stdin) --list");
        std::process::exit(1);
    }
    let mut input: Option<String> = None;
    let mut bodies: Option<String> = None;
    let mut gm_catalog: Option<String> = None;
    let mut gm_pck: Option<String> = None;
    let mut carrier_path: Option<String> = None;
    let mut dest = String::new();
    let mut ci_mode = false;
    let mut roundtrip_n = 400usize;
    let mut compare_input: Option<String> = None;
    let mut list_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--bodies" => {
                bodies = args.get(i + 1).cloned();
                i += 1;
            }
            "--gm-catalog" => {
                gm_catalog = args.get(i + 1).cloned();
                i += 1;
            }
            "--gm-pck" => {
                gm_pck = args.get(i + 1).cloned();
                i += 1;
            }
            "--carrier" => {
                carrier_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--dest" => {
                dest = args.get(i + 1).cloned().unwrap_or_default();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--roundtrip" => {
                roundtrip_n = args.get(i + 1).and_then(|v| v.parse().ok()).unwrap_or(400);
                i += 1;
            }
            "--compare-input" => {
                compare_input = args.get(i + 1).cloned();
                i += 1;
            }
            "--list" => list_mode = true,
            _ => {}
        }
        i += 1;
    }
    if list_mode {
        if let Some(path) = &input {
            let file = match File::open(path) {
                Ok(f) => f,
                Err(e) => die(format!("opening {}: {}", path, e)),
            };
            let mut boxed: Box<dyn Read> = Box::new(file);
            run_list(&mut *boxed);
        } else {
            let stdin = std::io::stdin();
            let mut boxed: Box<dyn Read> = Box::new(stdin);
            run_list(&mut *boxed);
        }
        return;
    }
    let target_ids: HashSet<i32> = match &bodies {
        Some(b) => {
            let mut set = HashSet::new();
            for tok in b.split(',') {
                match tok.trim().parse() {
                    Ok(id) => {
                        if !(1_000_000..10_000_000).contains(&id) {
                            die(format!("--bodies {} is not seven-digit", tok));
                        }
                        set.insert(id);
                    }
                    Err(_) => die(format!("--bodies token {} is no NAIF id", tok)),
                }
            }
            if set.is_empty() {
                die("--bodies carries no id".to_string());
            }
            set
        }
        None => die("--bodies absent — the split has no target list".to_string()),
    };
    let gm_map: HashMap<i32, f64> = match (&gm_catalog, &gm_pck) {
        (Some(c), None) => load_gm_catalog(c),
        (None, Some(p)) => {
            let text = match std::fs::read_to_string(p) {
                Ok(t) => t,
                Err(e) => die(format!("reading {}: {}", p, e)),
            };
            let pck = omegaflow::pck::parse(Some(&text), None);
            let mut map = HashMap::new();
            for (id, body) in &pck {
                if let Some(gm) = body.gm_m3_s2 {
                    map.insert(*id, gm);
                }
            }
            if map.is_empty() {
                die(format!("{} carries no GM line", p));
            }
            eprintln!("gm pck: {} carries {} GM lines", p, map.len());
            map
        }
        (None, None) => die("no GM source given (--gm-catalog or --gm-pck)".to_string()),
        (Some(_), Some(_)) => die("--gm-catalog and --gm-pck exclude each other".to_string()),
    };
    for id in &target_ids {
        if !gm_map.contains_key(id) {
            die(format!(
                "target {} carries no GM line in the given source",
                id
            ));
        }
    }
    let carrier_path = match carrier_path {
        Some(p) => p,
        None => die("--carrier absent — the sun's SSB state has no carrier".to_string()),
    };
    let carrier = match SpkFile::open(&carrier_path) {
        Ok(s) => s,
        Err(e) => die(format!("opening {}: {}", carrier_path, e)),
    };
    let compare = match &compare_input {
        Some(p) => match SpkFile::open(p) {
            Ok(s) => Some(s),
            Err(e) => die(format!("opening {}: {}", p, e)),
        },
        None => None,
    };
    let mut splitter = Splitter {
        targets: target_ids,
        pending: HashMap::new(),
        front_target: None,
        front_segs: Vec::new(),
        span_start: 0,
        span_end: 0,
        buf: Vec::new(),
        buf_base_addr: 0,
        buf_last_record: 0,
        file_record: [0u8; RECORD_BYTES],
        ss: 0,
        carrier,
        gm_map,
        dest,
        ci_mode,
        roundtrip_n,
        compare,
        processed: HashSet::new(),
        all_targets: BTreeMap::new(),
        total_segments: 0,
    };
    let mut source: Box<dyn Read> = match &input {
        Some(path) => match File::open(path) {
            Ok(f) => Box::new(f),
            Err(e) => die(format!("opening {}: {}", path, e)),
        },
        None => {
            let stdin = std::io::stdin();
            Box::new(stdin)
        }
    };
    let mut rec = [0u8; RECORD_BYTES];
    read_record(&mut *source, &mut rec, 1);
    let (nd, ni, fward) = parse_file_record(&rec);
    splitter.file_record.copy_from_slice(&rec);
    splitter.ss = nd + ni.div_ceil(2);
    let name_chars = splitter.ss * 8;
    let mut rec_no: u32 = 1;
    let mut next_summary: u32 = fward;
    loop {
        if next_summary != 0 {
            while rec_no + 1 < next_summary {
                let mut rec = [0u8; RECORD_BYTES];
                read_record(&mut *source, &mut rec, rec_no + 1);
                rec_no += 1;
                splitter.on_data(rec_no, &rec);
            }
            let mut rec = [0u8; RECORD_BYTES];
            read_record(&mut *source, &mut rec, rec_no + 1);
            rec_no += 1;
            let summary = parse_summary_record(&rec, nd, ni);
            let mut rec2 = [0u8; RECORD_BYTES];
            read_record(&mut *source, &mut rec2, rec_no + 1);
            rec_no += 1;
            let names = parse_name_record(&rec2, name_chars, summary.summaries.len());
            for (i, sum) in summary.summaries.iter().enumerate() {
                let name = names.get(i).cloned().unwrap_or_default();
                if let Some(seg) = seg_from_summary(sum, name) {
                    splitter.note_summary(seg);
                }
            }
            splitter.maybe_set_front(rec_no * 128 + 1);
            if summary.next == 0 {
                next_summary = 0;
                continue;
            }
            if summary.next <= rec_no {
                die(format!(
                    "summary chain pointer {} does not rise past record {}",
                    summary.next, rec_no
                ));
            }
            next_summary = summary.next;
        } else {
            let mut rec = [0u8; RECORD_BYTES];
            match read_record_soft(&mut *source, &mut rec, rec_no + 1) {
                Some(()) => {
                    rec_no += 1;
                    splitter.on_data(rec_no, &rec);
                }
                None => break,
            }
        }
    }
    if splitter.front_target.is_some() {
        eprintln!(
            "pending at stream end: {:?}",
            splitter.pending.keys().collect::<Vec<_>>()
        );
        die("the stream ended with a buffered target unextracted".to_string());
    }
    splitter.finish();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type2_payload(mid: f64, radius: f64, coeffs: [f64; 3]) -> Vec<u8> {
        let mut d = Vec::with_capacity(128 * 8);
        d.extend_from_slice(&mid.to_le_bytes());
        d.extend_from_slice(&radius.to_le_bytes());
        for c in coeffs {
            d.extend_from_slice(&c.to_le_bytes());
        }
        while d.len() < (128 - 4) * 8 {
            d.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        d.extend_from_slice(&mid.to_le_bytes());
        d.extend_from_slice(&radius.to_le_bytes());
        d.extend_from_slice(&5.0_f64.to_le_bytes());
        d.extend_from_slice(&1.0_f64.to_le_bytes());
        d
    }

    #[test]
    fn body_daf_roundtrips_segment_addresses() {
        let mut file_record = [0u8; RECORD_BYTES];
        file_record[0..4].copy_from_slice(b"DAF/");
        file_record[88..96].copy_from_slice(b"LTL-IEEE");
        file_record[8..12].copy_from_slice(&2u32.to_le_bytes());
        file_record[12..16].copy_from_slice(&6u32.to_le_bytes());
        file_record[76..80].copy_from_slice(&2u32.to_le_bytes());
        let seg_span: u32 = 128;
        let segs: Vec<SegInfo> = (0..4)
            .map(|k| {
                let start = 1 + k * seg_span;
                SegInfo {
                    target: 2136199,
                    center: 10,
                    frame: 1,
                    data_type: 2,
                    start_et: ET_MIN,
                    end_et: ET_MAX,
                    start_addr: start,
                    end_addr: start + seg_span - 1,
                    name: format!("SEG{}", k + 1),
                }
            })
            .collect();
        let mut span = Vec::new();
        for k in 0..4 {
            span.extend_from_slice(&type2_payload(
                0.0,
                32.0,
                [1.0 + k as f64, 2.0 + 2.0 * k as f64, 3.0 + 3.0 * k as f64],
            ));
        }
        assert_eq!(
            span.len(),
            (segs[3].end_addr - segs[0].start_addr + 1) as usize * 8
        );
        let daf_bytes = build_body_daf(&file_record, 5, &segs, &span);
        let daf = DafFile::from_data(daf_bytes).unwrap();
        let spk = SpkFile::from_daf(daf).unwrap();
        assert_eq!(spk.segments().len(), 4);
        for seg in spk.segments() {
            assert_eq!(seg.target, 2136199);
            assert_eq!(seg.center, 10);
            assert_eq!(seg.data_type, 2);
            assert!(seg.start_addr >= DATA_START_ADDR);
            assert!(seg.end_addr >= seg.start_addr);
        }
        let state = spk.state(2136199, 10, 0.0).unwrap();
        assert_eq!(state[0], 1.0);
        assert_eq!(state[1], 2.0);
        assert_eq!(state[2], 3.0);
        let state0 = spk.state(2136199, 10, -1.0e9).unwrap();
        assert_eq!(state0, [1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
    }
}
