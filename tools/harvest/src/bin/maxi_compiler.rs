use omegaflow::archivar::embedded_lsk;
use omegaflow::cdn::upload_release;
use omegaflow::maxi::{
    band_freq_width, mjd_to_tdb, parse_bin, parse_dat, write_bin, MaxiCurve, MaxiSample,
    BAND_10_20, BAND_2_20, BAND_2_4, BAND_4_10,
};
use std::io::Write;

const NETLOC: &str = "maxi.riken.jp";

const BANDS: [u32; 4] = [BAND_2_20, BAND_2_4, BAND_4_10, BAND_10_20];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn angle_deg(raw: &str, what: &str) -> Result<f64, String> {
    match raw.parse::<f64>() {
        Ok(v) if v.is_finite() => Ok(v),
        _ => Err(format!("--{what} {raw}: not a finite angle in degrees")),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let Some(input) = arg_value(args, "--input") else {
        return Err(
            "usage: maxi_compiler --input <maxi_g_lc_1day_all.dat> --out <maxi_curves.max1> --ra <deg> --dec <deg> [--ci-mode] — refused"
                .into(),
        );
    };
    let Some(out_path) = arg_value(args, "--out") else {
        return Err("--out <maxi_curves.max1>: the asset path is never silent — refused".into());
    };
    let Some(ra_raw) = arg_value(args, "--ra") else {
        return Err(
            "--ra <deg>: the source right ascension is absent from the .dat — refused".into(),
        );
    };
    let Some(dec_raw) = arg_value(args, "--dec") else {
        return Err("--dec <deg>: the source declination is absent from the .dat — refused".into());
    };
    let ra_deg = angle_deg(&ra_raw, "ra")?;
    let dec_deg = angle_deg(&dec_raw, "dec")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let text =
        std::fs::read_to_string(&input).map_err(|e| format!("read {input} returned void: {e}"))?;
    let rows = parse_dat(&text);
    if rows.is_empty() {
        return Err(format!(
            "{input}: no headerless row parsed — the asset stays unwritten (0 honored)"
        ));
    }
    let Some(lsk) = embedded_lsk() else {
        return Err("naif0012 table void — the TDB epoch stays void (no fabricated epoch)".into());
    };

    let mut curves: Vec<MaxiCurve> = Vec::with_capacity(BANDS.len());
    let mut epoch_dropped = 0u64;
    for band in BANDS {
        let Some((freq_hz, bin_width_hz)) = band_freq_width(band) else {
            continue;
        };
        let mut samples: Vec<MaxiSample> = Vec::with_capacity(rows.len());
        for (mjd, flux, err) in &rows {
            let Some(t_tdb) = mjd_to_tdb(*mjd, &lsk) else {
                epoch_dropped += 1;
                continue;
            };
            samples.push(MaxiSample {
                t_tdb,
                flux: flux[band as usize] as f32,
                err: err[band as usize] as f32,
            });
        }
        curves.push(MaxiCurve {
            ra_deg,
            dec_deg,
            band,
            freq_hz,
            bin_width_hz,
            samples,
        });
    }

    let Some(bytes) = write_bin(&curves) else {
        return Err(format!(
            "{out_path}: a curve value is non-finite — the asset stays unwritten (0 honored)"
        ));
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut f = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    f.write_all(&bytes)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    f.flush()
        .map_err(|e| format!("flush {out_path} returned void: {e}"))?;

    let parsed =
        parse_bin(&bytes).ok_or_else(|| format!("{out_path}: the roundtrip parse stays unread"))?;
    let total_samples: usize = parsed.iter().map(|c| c.samples.len()).sum();
    eprintln!(
        "{out_path}: {} curves, {} samples from {} rows at ra {ra_deg} dec {dec_deg} — roundtrip verified",
        parsed.len(),
        total_samples,
        rows.len()
    );
    if epoch_dropped > 0 {
        eprintln!("MJD epochs outside the leap-second table: {epoch_dropped}");
    }
    let first = parsed
        .iter()
        .find(|c| c.band == BAND_2_20)
        .and_then(|c| c.samples.first());
    let last = parsed
        .iter()
        .find(|c| c.band == BAND_2_20)
        .and_then(|c| c.samples.last());
    if let (Some(f), Some(l)) = (first, last) {
        eprintln!(
            "2-20 keV flux: first {:.6} at tdb {:.0}, last {:.6} at tdb {:.0}",
            f.flux, f.t_tdb, l.flux, l.t_tdb
        );
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("maxi_compiler: {msg}");
        std::process::exit(1);
    }
}
