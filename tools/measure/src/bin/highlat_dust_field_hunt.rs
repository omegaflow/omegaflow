use omegaflow::bayestar::{
    build_index, decode_rec, ebv_at, index_add, index_sort, leaf_record, mu_of_r_pc, parse_header,
    Be19Row, ASSET_HEADER_LEN, BE19_BINS, REC_BYTES,
};
use omegaflow::healpix::{galactic_to_icrs, pix2ang_nest};
use std::io::{Read, Seek, SeekFrom};

const RV: f64 = 3.1;
const VALID_B_MIN_DEG: f64 = 20.0;
const VALID_B_MAX_DEG: f64 = 80.0;
const AV_FULL_MIN: f64 = 0.3;
const NSIDE_HUNT: i64 = 64;
const SHOW_TOP: usize = 40;
const PROBE_DISTANCES_PC: [f64; 5] = [200.0, 400.0, 700.0, 1200.0, 2500.0];

struct MapFile {
    file: std::fs::File,
    last_idx: u64,
    last: Option<Be19Row>,
}

impl MapFile {
    fn read(&mut self, idx: u64) -> Option<&Be19Row> {
        if self.last_idx != idx {
            let off = ASSET_HEADER_LEN as u64 + idx * REC_BYTES as u64;
            self.file.seek(SeekFrom::Start(off)).ok()?;
            let mut buf = vec![0u8; REC_BYTES];
            self.file.read_exact(&mut buf).ok()?;
            self.last = decode_rec(&buf);
            self.last_idx = idx;
        }
        self.last.as_ref()
    }
}

fn load_map(path: &str, idx: &mut omegaflow::bayestar::MapQuery) -> Result<MapFile, String> {
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    let mut head = [0u8; ASSET_HEADER_LEN];
    f.read_exact(&mut head)
        .map_err(|e| format!("read {path} header returned void: {e}"))?;
    let h = parse_header(&head).ok_or_else(|| format!("{path}: the header stays unread"))?;
    if h.mu0 != omegaflow::bayestar::BE19_MU0
        || h.dmu != omegaflow::bayestar::BE19_DMU
        || h.bins as usize != BE19_BINS
    {
        return Err(format!(
            "{path}: grid mu0 {} dmu {} bins {} disagrees with the reader grid — refused",
            h.mu0, h.dmu, h.bins
        ));
    }
    let mut rec = vec![0u8; REC_BYTES];
    let mut row_no = 0u64;
    while row_no < h.n_rows {
        f.read_exact(&mut rec)
            .map_err(|e| format!("read {path} record returned void: {e}"))?;
        let r = decode_rec(&rec).ok_or_else(|| format!("{path}: record {row_no} stays unread"))?;
        index_add(idx, row_no, r.nside.trailing_zeros() as u8, r.ipix);
        row_no += 1;
    }
    index_sort(idx);
    Ok(MapFile {
        file: f,
        last_idx: u64::MAX,
        last: None,
    })
}

struct Sight {
    l_deg: f64,
    b_deg: f64,
    ra_deg: f64,
    dec_deg: f64,
    av_d: [f64; PROBE_DISTANCES_PC.len()],
    av_full: f64,
    converged: bool,
    n_leaf_pc: u32,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(map_path) = args
        .iter()
        .position(|a| a == "--map")
        .and_then(|i| args.get(i + 1))
        .cloned()
    else {
        eprintln!("usage: highlat_dust_field_hunt --map <bayestar.be19>");
        return;
    };
    let mut mq = build_index();
    let mf = match load_map(&map_path, &mut mq) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let mut file = mf;
    let npix = (12 * NSIDE_HUNT * NSIDE_HUNT) as i64;
    let mut hits: Vec<Sight> = Vec::new();
    let mut n_leaf = 0usize;
    for pix in 0..npix {
        let Some((theta, phi)) = pix2ang_nest(NSIDE_HUNT, pix) else {
            continue;
        };
        let b = 90.0 - theta.to_degrees();
        if !(b.abs() >= VALID_B_MIN_DEG && b.abs() <= VALID_B_MAX_DEG) {
            continue;
        }
        let Some(leaf) = leaf_record(&mq, theta, phi) else {
            continue;
        };
        n_leaf += 1;
        let Some(row) = file.read(leaf) else {
            continue;
        };
        let av_full = RV * row.best_fit[BE19_BINS - 1] as f64;
        if !(av_full.is_finite() && av_full >= AV_FULL_MIN) {
            continue;
        }
        let mut av_d = [0.0f64; PROBE_DISTANCES_PC.len()];
        let mut ok = true;
        for (j, r) in PROBE_DISTANCES_PC.iter().enumerate() {
            match mu_of_r_pc(*r).and_then(|mu| ebv_at(&row.best_fit, mu)) {
                Some(ebv) if ebv.is_finite() && ebv > 0.0 => av_d[j] = RV * ebv,
                _ => {
                    av_d[j] = 0.0;
                }
            }
            if !av_d[j].is_finite() {
                ok = false;
            }
        }
        if !ok {
            continue;
        }
        let (ra, dec) = galactic_to_icrs(theta, phi);
        hits.push(Sight {
            l_deg: phi.to_degrees(),
            b_deg: b,
            ra_deg: ra,
            dec_deg: dec,
            av_d,
            av_full,
            converged: row.converged,
            n_leaf_pc: row.nside,
        });
    }
    hits.sort_by(|a, b| {
        b.av_d[2]
            .partial_cmp(&a.av_d[2])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    println!(
        "highlat_dust_field_hunt — {npix} healpix-{NSIDE_HUNT} sightlines, |b| in [{VALID_B_MIN_DEG}, {VALID_B_MAX_DEG}] deg"
    );
    println!(
        "sightlines with a leaf: {n_leaf}; with full line-of-sight A_V >= {AV_FULL_MIN} mag: {}; showing the {SHOW_TOP} strongest (sorted by A_V at {:.0} pc)",
        hits.len(),
        PROBE_DISTANCES_PC[2]
    );
    for h in hits.iter().take(SHOW_TOP) {
        let d = h.av_d;
        println!(
            "l {:.2} b {:.2} | ICRS ra {:.3} dec {:.3} | A_V @ {:.0}/{:.0}/{:.0}/{:.0}/{:.0} pc = {:.3}/{:.3}/{:.3}/{:.3}/{:.3} | full {:.3} | converged {} nside {}",
            h.l_deg,
            h.b_deg,
            h.ra_deg,
            h.dec_deg,
            PROBE_DISTANCES_PC[0],
            PROBE_DISTANCES_PC[1],
            PROBE_DISTANCES_PC[2],
            PROBE_DISTANCES_PC[3],
            PROBE_DISTANCES_PC[4],
            d[0],
            d[1],
            d[2],
            d[3],
            d[4],
            h.av_full,
            h.converged,
            h.n_leaf_pc
        );
    }
    if hits.is_empty() {
        println!(
            "no sightline at |b| >= {VALID_B_MIN_DEG} deg reaches full A_V >= {AV_FULL_MIN} mag — the dust column is too thin to change a crossmatch identification (0 honored)"
        );
    }
}
