use omegaflow::archivar::footprint::{
    band_code, decode_rec, encode_rec, parse_header, write_header, FootprintRecord, HEADER_LEN,
    REC_BYTES,
};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn u64_arg(args: &[String], name: &str) -> Option<u64> {
    arg_value(args, name).and_then(|s| s.parse().ok())
}

fn record_key(r: &FootprintRecord) -> (u8, u32, u8, u32) {
    (r.order, r.ipix, band_code(r.band), r.frac.to_bits())
}

fn group_key(r: &FootprintRecord) -> (u8, u32, u8) {
    (r.order, r.ipix, band_code(r.band))
}

struct PartSource {
    name: String,
    reader: BufReader<File>,
    remaining: u64,
    current: Option<FootprintRecord>,
    prev: Option<(u8, u32, u8, u32)>,
}

impl PartSource {
    fn open(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
        let len = std::fs::metadata(path)
            .map_err(|e| format!("stat {path} returned void: {e}"))?
            .len();
        let mut reader = BufReader::with_capacity(1 << 20, file);
        let mut head = [0u8; HEADER_LEN];
        reader
            .read_exact(&mut head)
            .map_err(|e| format!("read {path} header returned void: {e}"))?;
        let rows =
            parse_header(&head).ok_or_else(|| format!("{path}: the FP01 header stays unread"))?;
        let expect = HEADER_LEN as u64 + rows * REC_BYTES as u64;
        if len != expect {
            return Err(format!(
                "{path}: {len} bytes, {expect} expected from a {rows}-row header — the partial is not whole"
            ));
        }
        Ok(PartSource {
            name: path.to_string(),
            reader,
            remaining: rows,
            current: None,
            prev: None,
        })
    }

    fn read_next(&mut self) -> Result<(), String> {
        if self.remaining == 0 {
            self.current = None;
            return Ok(());
        }
        let mut buf = [0u8; REC_BYTES];
        self.reader
            .read_exact(&mut buf)
            .map_err(|e| format!("{}: read a record returned void: {e}", self.name))?;
        let rec =
            decode_rec(&buf).ok_or_else(|| format!("{}: a record stays undecodable", self.name))?;
        let key = record_key(&rec);
        if let Some(prev) = self.prev {
            if key < prev {
                return Err(format!(
                    "{}: a record precedes a larger one — the partial is not sorted by (order, ipix, band, frac)",
                    self.name
                ));
            }
        }
        self.prev = Some(key);
        self.remaining -= 1;
        self.current = Some(rec);
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
struct MergeNode {
    key: (u8, u32, u8, u32),
    stream: usize,
}

impl PartialOrd for MergeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MergeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key
            .cmp(&other.key)
            .then(self.stream.cmp(&other.stream))
    }
}

fn part_range(name: &str) -> Option<(u64, u64)> {
    let stem = name.strip_prefix("allwise_part_")?;
    let digits = stem.strip_suffix(".fp01")?;
    let (lo, hi) = digits.split_once('_')?;
    if lo.is_empty() || hi.is_empty() {
        return None;
    }
    if !lo.bytes().all(|b| b.is_ascii_digit()) || !hi.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some((lo.parse().ok()?, hi.parse().ok()?))
}

fn list_parts(dir: &str) -> Result<Vec<(u64, u64, String)>, String> {
    let rd = std::fs::read_dir(dir)
        .map_err(|e| format!("read the parts directory {dir} returned void: {e}"))?;
    let mut found = Vec::new();
    for entry in rd {
        let entry =
            entry.map_err(|e| format!("read a parts-directory entry returned void: {e}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some((lo, hi)) = part_range(&name) {
            found.push((lo, hi, entry.path().to_string_lossy().into_owned()));
        }
    }
    found.sort_unstable();
    Ok(found)
}

fn merge_partials(parts_dir: &str, out_path: &str, expect_tiles: u64) -> Result<u64, String> {
    if expect_tiles == 0 {
        return Err("--expect-tiles 0: no tile span — the combine stays unwritten".into());
    }
    let found = list_parts(parts_dir)?;
    if found.is_empty() {
        return Err(
            "no allwise_part_* partials in the parts directory — the combine stays unwritten"
                .into(),
        );
    }
    let mut expect = 0u64;
    for (lo, hi, _) in &found {
        if *lo != expect {
            return Err(format!(
                "allwise_part_{lo}_{hi}.fp01 starts at tile {lo}, the enumeration holds {expect} — a gap in the tile span, the combine stays unwritten"
            ));
        }
        if *hi < *lo {
            return Err(format!(
                "allwise_part_{lo}_{hi}.fp01 descends (lo {lo} > hi {hi}) — the combine stays unwritten"
            ));
        }
        expect = *hi + 1;
    }
    if expect != expect_tiles {
        return Err(format!(
            "the partials tile 0..{} but the survey holds {expect_tiles} tiles — the combine stays unwritten",
            expect.saturating_sub(1)
        ));
    }

    let mut sources = Vec::new();
    for (_, _, path) in &found {
        let mut src = PartSource::open(path)?;
        src.read_next()?;
        sources.push(src);
    }

    let mut heap = BinaryHeap::new();
    for (i, src) in sources.iter().enumerate() {
        if let Some(rec) = &src.current {
            heap.push(Reverse(MergeNode {
                key: record_key(rec),
                stream: i,
            }));
        }
    }

    let mut file =
        File::create(out_path).map_err(|e| format!("create {out_path} returned void: {e}"))?;
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, 0);
    file.write_all(&hbuf)
        .map_err(|e| format!("write {out_path} header returned void: {e}"))?;
    let mut out = BufWriter::with_capacity(1 << 20, file);

    let mut total: u64 = 0;
    let mut held: Option<(FootprintRecord, f64)> = None;
    let mut rec = [0u8; REC_BYTES];
    while let Some(Reverse(node)) = heap.pop() {
        let stream = node.stream;
        let current = sources[stream]
            .current
            .take()
            .ok_or_else(|| format!("{}: the stream's record went missing", sources[stream].name))?;
        sources[stream].read_next()?;
        if let Some(next) = &sources[stream].current {
            heap.push(Reverse(MergeNode {
                key: record_key(next),
                stream,
            }));
        }
        match held {
            None => held = Some((current, current.frac as f64)),
            Some((ref r, ref mut s)) if group_key(r) == group_key(&current) => {
                *s += current.frac as f64;
            }
            Some((r, s)) => {
                let merged = FootprintRecord {
                    frac: s.clamp(0.0, 1.0) as f32,
                    ..r
                };
                encode_rec(&mut rec, &merged);
                out.write_all(&rec)
                    .map_err(|e| format!("write {out_path} records returned void: {e}"))?;
                total += 1;
                held = Some((current, current.frac as f64));
            }
        }
    }
    if let Some((r, s)) = held {
        let merged = FootprintRecord {
            frac: s.clamp(0.0, 1.0) as f32,
            ..r
        };
        encode_rec(&mut rec, &merged);
        out.write_all(&rec)
            .map_err(|e| format!("write {out_path} tail record returned void: {e}"))?;
        total += 1;
    }
    out.flush()
        .map_err(|e| format!("flush {out_path} returned void: {e}"))?;

    let mut patch = std::fs::OpenOptions::new()
        .write(true)
        .open(out_path)
        .map_err(|e| format!("reopen {out_path} returned void: {e}"))?;
    patch
        .seek(SeekFrom::Start(5))
        .map_err(|e| format!("seek {out_path} header returned void: {e}"))?;
    patch
        .write_all(&total.to_le_bytes())
        .map_err(|e| format!("patch {out_path} header returned void: {e}"))?;
    patch
        .flush()
        .map_err(|e| format!("flush {out_path} header returned void: {e}"))?;

    let expect = HEADER_LEN as u64 + total * REC_BYTES as u64;
    let actual = std::fs::metadata(out_path)
        .map_err(|e| format!("stat {out_path} returned void: {e}"))?
        .len();
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }

    let mut vf = File::open(out_path).map_err(|e| format!("open {out_path} returned void: {e}"))?;
    let mut head = [0u8; HEADER_LEN];
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {out_path} header returned void: {e}"))?;
    let rows = parse_header(&head).ok_or_else(|| format!("{out_path}: the header stays unread"))?;
    if rows != total {
        return Err(format!(
            "{out_path}: header {rows} rows, {total} written — the asset stays unwritten"
        ));
    }
    if total > 0 {
        let last_off = HEADER_LEN as u64 + (total - 1) * REC_BYTES as u64;
        vf.seek(SeekFrom::Start(last_off))
            .map_err(|e| format!("seek {out_path} tail returned void: {e}"))?;
        let mut tail = vec![0u8; REC_BYTES];
        vf.read_exact(&mut tail)
            .map_err(|e| format!("read {out_path} tail returned void: {e}"))?;
        decode_rec(&tail).ok_or_else(|| format!("{out_path}: the last record stays unread"))?;
    }

    eprintln!(
        "{out_path}: {} partials summed into {} records (fractions summed and clamped, sorted by ipix) — roundtrip verified",
        found.len(),
        total
    );
    Ok(total)
}

fn run(args: &[String]) -> Result<(), String> {
    let parts_dir = match arg_value(args, "--parts-dir") {
        Some(v) => v,
        None => ".".to_string(),
    };
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => "allwise_coverage.fp01".to_string(),
    };
    let expect_tiles = match u64_arg(args, "--expect-tiles") {
        Some(v) => v,
        None => {
            return Err("--expect-tiles names the survey tile count the parts must tile".into())
        }
    };
    let total = merge_partials(&parts_dir, &out_path, expect_tiles)?;
    if total == 0 {
        return Err("no coverage record merged — the asset stays unwritten (0 honored)".into());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("wise_coverage_combiner: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::footprint::FootprintBand;
    use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

    static SEQ: AtomicU64 = AtomicU64::new(0);

    struct TempDir {
        path: std::path::PathBuf,
    }

    impl TempDir {
        fn new() -> Self {
            let n = SEQ.fetch_add(1, AtomicOrdering::Relaxed);
            let path =
                std::env::temp_dir().join(format!("wise_combine_test_{}_{n}", std::process::id()));
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir { path }
        }

        fn file(&self, name: &str) -> String {
            self.path.join(name).to_string_lossy().into_owned()
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    fn rec(ipix: u32, band: FootprintBand, frac: f32) -> FootprintRecord {
        FootprintRecord {
            order: 10,
            band,
            ipix,
            frac,
        }
    }

    fn write_part(path: &str, records: &[FootprintRecord]) {
        let mut buf = Vec::new();
        write_header(&mut buf, records.len() as u64);
        let mut rec = [0u8; REC_BYTES];
        for r in records {
            encode_rec(&mut rec, r);
            buf.extend_from_slice(&rec);
        }
        std::fs::write(path, buf).expect("write part");
    }

    fn read_rows(path: &str) -> Vec<FootprintRecord> {
        let bytes = std::fs::read(path).expect("read asset");
        let mut out = Vec::new();
        let mut off = HEADER_LEN;
        while off + REC_BYTES <= bytes.len() {
            out.push(decode_rec(&bytes[off..off + REC_BYTES]).expect("decode row"));
            off += REC_BYTES;
        }
        out
    }

    fn assert_sorted_by_ipix_and_band(rows: &[FootprintRecord]) {
        let mut prev: Option<(u8, u32, u8)> = None;
        for r in rows {
            let k = group_key(r);
            if let Some(p) = prev {
                assert!(k > p, "the merged asset stays sorted");
            }
            prev = Some(k);
        }
    }

    #[test]
    fn sums_overlapping_pixels_across_partials_and_clamps_to_one() {
        let dir = TempDir::new();
        let a = [
            rec(7, FootprintBand::W1, 0.4),
            rec(7, FootprintBand::W2, 0.2),
        ];
        let b = [
            rec(7, FootprintBand::W1, 0.7),
            rec(8, FootprintBand::W1, 0.1),
        ];
        write_part(&dir.file("allwise_part_0_99.fp01"), &a);
        write_part(&dir.file("allwise_part_100_199.fp01"), &b);
        let out = dir.file("out.fp01");
        let total = merge_partials(&dir.file(""), &out, 200).expect("merge");
        assert_eq!(total, 3);
        let rows = read_rows(&out);
        assert_sorted_by_ipix_and_band(&rows);
        let w1_7 = rows
            .iter()
            .find(|r| r.ipix == 7 && r.band == FootprintBand::W1)
            .expect("the w1 pixel");
        assert_eq!(w1_7.frac, 1.0);
        let w2_7 = rows
            .iter()
            .find(|r| r.ipix == 7 && r.band == FootprintBand::W2)
            .expect("the w2 pixel");
        assert_eq!(w2_7.frac, 0.2);
    }

    #[test]
    fn sums_fractions_that_stay_below_one() {
        let dir = TempDir::new();
        let a = [rec(3, FootprintBand::W3, 0.3)];
        let b = [rec(3, FootprintBand::W3, 0.4)];
        write_part(&dir.file("allwise_part_0_99.fp01"), &a);
        write_part(&dir.file("allwise_part_100_199.fp01"), &b);
        let out = dir.file("out.fp01");
        let total = merge_partials(&dir.file(""), &out, 200).expect("merge");
        assert_eq!(total, 1);
        let rows = read_rows(&out);
        assert_eq!(rows[0].ipix, 3);
        assert_eq!(rows[0].band, FootprintBand::W3);
        assert!((rows[0].frac - 0.7).abs() < 1e-6);
    }

    #[test]
    fn refuses_a_gap_in_the_tile_span() {
        let dir = TempDir::new();
        let a = [rec(1, FootprintBand::W1, 0.5)];
        let b = [rec(3, FootprintBand::W1, 0.5)];
        write_part(&dir.file("allwise_part_0_99.fp01"), &a);
        write_part(&dir.file("allwise_part_200_299.fp01"), &b);
        let out = dir.file("out.fp01");
        let err = merge_partials(&dir.file(""), &out, 300).expect_err("gap refused");
        assert!(err.contains("gap"));
    }

    #[test]
    fn refuses_an_incomplete_tile_span() {
        let dir = TempDir::new();
        let a = [rec(1, FootprintBand::W1, 0.5)];
        let b = [rec(3, FootprintBand::W1, 0.5)];
        write_part(&dir.file("allwise_part_0_99.fp01"), &a);
        write_part(&dir.file("allwise_part_100_199.fp01"), &b);
        let out = dir.file("out.fp01");
        let err = merge_partials(&dir.file(""), &out, 300).expect_err("incomplete refused");
        assert!(err.contains("300"));
    }

    #[test]
    fn refuses_an_unsorted_partial() {
        let dir = TempDir::new();
        let a = [
            rec(9, FootprintBand::W1, 0.5),
            rec(1, FootprintBand::W1, 0.5),
        ];
        write_part(&dir.file("allwise_part_0_99.fp01"), &a);
        let out = dir.file("out.fp01");
        let err = merge_partials(&dir.file(""), &out, 100).expect_err("unsorted refused");
        assert!(err.contains("not sorted"));
    }

    #[test]
    fn tolerates_an_empty_partial_among_measured_ones() {
        let dir = TempDir::new();
        let a: [FootprintRecord; 0] = [];
        let b = [rec(2, FootprintBand::W2, 0.8)];
        write_part(&dir.file("allwise_part_0_99.fp01"), &a);
        write_part(&dir.file("allwise_part_100_199.fp01"), &b);
        let out = dir.file("out.fp01");
        let total = merge_partials(&dir.file(""), &out, 200).expect("merge");
        assert_eq!(total, 1);
        let rows = read_rows(&out);
        assert_eq!(rows[0].ipix, 2);
        assert_eq!(rows[0].band, FootprintBand::W2);
        assert_eq!(rows[0].frac, 0.8);
    }

    #[test]
    fn merged_asset_header_roundtrips_the_row_count() {
        let dir = TempDir::new();
        let a = [rec(4, FootprintBand::W1, 0.3)];
        write_part(&dir.file("allwise_part_0_99.fp01"), &a);
        let out = dir.file("out.fp01");
        merge_partials(&dir.file(""), &out, 100).expect("merge");
        let bytes = std::fs::read(&out).expect("read asset");
        assert_eq!(bytes.len(), HEADER_LEN + 1 * REC_BYTES);
        assert_eq!(parse_header(&bytes[..HEADER_LEN]), Some(1));
        let decoded = decode_rec(&bytes[HEADER_LEN..]).expect("the row decodes");
        assert_eq!(decoded.frac, 0.3);
    }
}
