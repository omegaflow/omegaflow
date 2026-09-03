use crate::archivar::types::C_LIGHT;

pub const JWST_MAGIC: [u8; 4] = *b"JWS1";
pub const JWST_HEADER_BYTES: usize = 8;
pub const JWST_HOST_BYTES: usize = 32;
pub const JWST_OBSID_BYTES: usize = 64;
pub const JWST_RECORD_BYTES: usize = 8 + 8 + 4 + 8 + JWST_HOST_BYTES + JWST_OBSID_BYTES + 4;
pub const JWST_BIN_BYTES: usize = 24;

pub const JY_TO_W_M2_HZ: f64 = 1e-26;

#[derive(Clone)]
pub struct JwstSpectrum {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub plx_mas: f64,
    pub epoch_tdb: f64,
    pub host: String,
    pub obs_id: String,
    pub bins: Vec<(f64, f64, f64)>,
}

pub fn bins_from_jwst_rows(rows: &[(f64, f64)]) -> Vec<(f64, f64, f64)> {
    let valid =
        |&(lam, flux): &(f64, f64)| lam.is_finite() && flux.is_finite() && lam > 0.0 && flux > 0.0;
    let mut out = Vec::new();
    for (i, &(lam_um, flux_jy)) in rows.iter().enumerate() {
        if !valid(&(lam_um, flux_jy)) {
            continue;
        }
        let lam_m = lam_um * 1e-6;
        let freq = C_LIGHT / lam_m;
        let prev_nu = rows[..i]
            .iter()
            .rev()
            .find(|r| valid(r))
            .map(|&(l, _)| C_LIGHT / (l * 1e-6));
        let next_nu = rows[i + 1..]
            .iter()
            .find(|r| valid(r))
            .map(|&(l, _)| C_LIGHT / (l * 1e-6));
        let bin_width = match (prev_nu, next_nu) {
            (Some(hi), Some(lo)) => (hi - lo) * 0.5,
            (Some(hi), None) => (hi - freq).abs(),
            (None, Some(lo)) => (freq - lo).abs(),
            (None, None) => 0.0,
        };
        out.push((freq, bin_width, flux_jy * JY_TO_W_M2_HZ));
    }
    out
}

fn push_padded(buf: &mut Vec<u8>, text: &str, len: usize) -> bool {
    if text.len() > len {
        return false;
    }
    let pad = [0u8; 256];
    buf.extend_from_slice(text.as_bytes());
    buf.extend_from_slice(&pad[..len - text.len()]);
    true
}

fn read_padded(bytes: &[u8], off: &mut usize, len: usize) -> Option<String> {
    let end = off.checked_add(len)?;
    if end > bytes.len() {
        return None;
    }
    let slice = &bytes[*off..end];
    *off = end;
    let cut = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
    match std::str::from_utf8(&slice[..cut]) {
        Ok(s) if !s.is_empty() => Some(s.to_string()),
        _ => None,
    }
}

pub fn write_jwst_bin(specs: &[JwstSpectrum]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(
        JWST_HEADER_BYTES + specs.len() * (JWST_RECORD_BYTES + 512 * JWST_BIN_BYTES),
    );
    out.extend_from_slice(&JWST_MAGIC);
    out.extend_from_slice(&(specs.len() as u32).to_le_bytes());
    for s in specs {
        if !s.ra_deg.is_finite() || !s.dec_deg.is_finite() || !s.epoch_tdb.is_finite() {
            return None;
        }
        if !s.plx_mas.is_finite() {
            return None;
        }
        out.extend_from_slice(&s.ra_deg.to_le_bytes());
        out.extend_from_slice(&s.dec_deg.to_le_bytes());
        out.extend_from_slice(&(s.plx_mas as f32).to_le_bytes());
        out.extend_from_slice(&s.epoch_tdb.to_le_bytes());
        if !push_padded(&mut out, &s.host, JWST_HOST_BYTES) {
            return None;
        }
        if !push_padded(&mut out, &s.obs_id, JWST_OBSID_BYTES) {
            return None;
        }
        out.extend_from_slice(&(s.bins.len() as u32).to_le_bytes());
        for &(freq, bin_width, val) in &s.bins {
            if !freq.is_finite() || !bin_width.is_finite() || !val.is_finite() {
                return None;
            }
            out.extend_from_slice(&freq.to_le_bytes());
            out.extend_from_slice(&bin_width.to_le_bytes());
            out.extend_from_slice(&val.to_le_bytes());
        }
    }
    Some(out)
}

pub fn parse_jwst_bin(bytes: &[u8]) -> Option<Vec<JwstSpectrum>> {
    if bytes.len() < JWST_HEADER_BYTES || bytes[0..4] != JWST_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = JWST_HEADER_BYTES;
    let mut specs = Vec::with_capacity(count);
    for _ in 0..count {
        if off + JWST_RECORD_BYTES > bytes.len() {
            return None;
        }
        let ra = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
        let dec = f64::from_le_bytes(bytes[off + 8..off + 16].try_into().ok()?);
        let plx = f32::from_le_bytes(bytes[off + 16..off + 20].try_into().ok()?) as f64;
        let epoch = f64::from_le_bytes(bytes[off + 20..off + 28].try_into().ok()?);
        off += 28;
        let host = read_padded(bytes, &mut off, JWST_HOST_BYTES)?;
        let obs_id = read_padded(bytes, &mut off, JWST_OBSID_BYTES)?;
        if off + 4 > bytes.len() {
            return None;
        }
        let n_bins = u32::from_le_bytes(bytes[off..off + 4].try_into().ok()?) as usize;
        off += 4;
        if !ra.is_finite() || !dec.is_finite() || !plx.is_finite() || !epoch.is_finite() {
            return None;
        }
        if off + n_bins * JWST_BIN_BYTES > bytes.len() {
            return None;
        }
        let mut bins = Vec::with_capacity(n_bins);
        for _ in 0..n_bins {
            let freq = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
            let bin_width = f64::from_le_bytes(bytes[off + 8..off + 16].try_into().ok()?);
            let val = f64::from_le_bytes(bytes[off + 16..off + 24].try_into().ok()?);
            off += JWST_BIN_BYTES;
            if !freq.is_finite() || !bin_width.is_finite() || !val.is_finite() {
                return None;
            }
            bins.push((freq, bin_width, val));
        }
        specs.push(JwstSpectrum {
            ra_deg: ra,
            dec_deg: dec,
            plx_mas: plx,
            epoch_tdb: epoch,
            host,
            obs_id,
            bins,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(specs)
}

pub fn mjd_to_unix(mjd: f64) -> f64 {
    (mjd - 40587.0) * 86400.0
}

pub const JWST_LEDGER: &str = "ledger.tsv";
pub const JWST_SIDECAR_DIR: &str = "sidecars";

pub fn sidecar_path(workdir: &std::path::Path, obs_id: &str) -> std::path::PathBuf {
    workdir
        .join(JWST_SIDECAR_DIR)
        .join(format!("{}.bin", obs_id))
}

pub fn ledger_done(workdir: &std::path::Path) -> std::collections::HashSet<String> {
    let mut done = std::collections::HashSet::new();
    if let Ok(body) = std::fs::read_to_string(workdir.join(JWST_LEDGER)) {
        for line in body.lines() {
            if let Some((obs_id, _)) = line.split_once('\t') {
                if !obs_id.is_empty() {
                    done.insert(obs_id.to_string());
                }
            }
        }
    }
    done
}

fn append_fsync(path: &std::path::Path, bytes: &[u8]) -> bool {
    use std::io::Write;
    let f = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        Ok(f) => f,
        Err(_) => return false,
    };
    let mut f = f;
    if f.write_all(bytes).is_err() {
        return false;
    }
    f.sync_all().is_ok()
}

pub fn ledger_append(
    workdir: &std::path::Path,
    obs_id: &str,
    host: &str,
    n_bins: usize,
    epoch: f64,
) -> bool {
    let line = format!("{}\t{}\t{}\t{}\n", obs_id, host, n_bins, epoch);
    append_fsync(&workdir.join(JWST_LEDGER), line.as_bytes())
}

pub fn write_sidecar(workdir: &std::path::Path, spec: &JwstSpectrum) -> bool {
    let dir = workdir.join(JWST_SIDECAR_DIR);
    if std::fs::create_dir_all(&dir).is_err() {
        return false;
    }
    let Some(bytes) = write_jwst_bin(std::slice::from_ref(spec)) else {
        return false;
    };
    let target = sidecar_path(workdir, &spec.obs_id);
    let tmp = dir.join(format!("{}.tmp", spec.obs_id));
    if std::fs::write(&tmp, &bytes).is_err() {
        return false;
    }
    if let Ok(f) = std::fs::File::open(&tmp) {
        let _ = f.sync_all();
    }
    if std::fs::rename(&tmp, &target).is_err() {
        let _ = std::fs::remove_file(&tmp);
        return false;
    }
    true
}

pub fn finalize_workdir(workdir: &std::path::Path) -> Option<Vec<u8>> {
    let dir = workdir.join(JWST_SIDECAR_DIR);
    let mut specs: Vec<JwstSpectrum> = Vec::new();
    for entry in std::fs::read_dir(&dir).ok()?.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("bin") {
            continue;
        }
        let bytes = std::fs::read(&path).ok()?;
        specs.extend(parse_jwst_bin(&bytes)?);
    }
    if specs.is_empty() {
        return None;
    }
    specs.sort_by(|a, b| a.obs_id.cmp(&b.obs_id));
    write_jwst_bin(&specs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(host: &str, obs_id: &str) -> JwstSpectrum {
        JwstSpectrum {
            ra_deg: 217.32673,
            dec_deg: -3.4445,
            plx_mas: 4.6,
            epoch_tdb: 8.4e8,
            host: host.to_string(),
            obs_id: obs_id.to_string(),
            bins: vec![(2.5e14, 1.0e13, 4.2e-24), (2.6e14, 1.0e13, 4.0e-24)],
        }
    }

    #[test]
    fn jwst_bin_roundtrip() {
        let specs = vec![
            spec("WASP-39", "jw01366-o001_t001_niriss"),
            spec("K2-18", "jw02722"),
        ];
        let bytes = write_jwst_bin(&specs).unwrap();
        let parsed = parse_jwst_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].host, "WASP-39");
        assert_eq!(parsed[0].obs_id, "jw01366-o001_t001_niriss");
        assert_eq!(parsed[0].plx_mas, 4.6f32 as f64);
        assert_eq!(parsed[0].bins, specs[0].bins);
        assert_eq!(parsed[1].bins, specs[1].bins);
    }

    #[test]
    fn jwst_bin_refuses_malformed() {
        assert!(parse_jwst_bin(&[0xCF, 0x86, 0x01]).is_none());
        assert!(parse_jwst_bin(b"JWS").is_none());
        let specs = vec![spec("WASP-39", "obs")];
        let bytes = write_jwst_bin(&specs).unwrap();
        assert!(parse_jwst_bin(&bytes[..bytes.len() - 3]).is_none());
        let truncated = &bytes[..bytes.len() - 1];
        assert!(parse_jwst_bin(truncated).is_none());
    }

    #[test]
    fn jwst_bin_refuses_overlong_names() {
        let mut s = spec("WASP-39", "obs");
        s.host = "x".repeat(JWST_HOST_BYTES + 1);
        assert!(write_jwst_bin(&[s]).is_none());
        let mut s2 = spec("WASP-39", "obs");
        s2.obs_id = "y".repeat(JWST_OBSID_BYTES + 1);
        assert!(write_jwst_bin(&[s2]).is_none());
    }

    #[test]
    fn jwst_bin_carries_full_length_obs_ids() {
        let s = spec(
            "WASP-39",
            "jw08017-o001_t001_nirspec_f290lp-g395h-s1600a1-sub2048",
        );
        assert_eq!(s.obs_id.len(), 54);
        let bytes = write_jwst_bin(&[s]).unwrap();
        let parsed = parse_jwst_bin(&bytes).unwrap();
        assert_eq!(
            parsed[0].obs_id,
            "jw08017-o001_t001_nirspec_f290lp-g395h-s1600a1-sub2048"
        );
    }

    #[test]
    fn bins_convert_jy_rows_to_frequency() {
        let rows = [(0.8, 150.0), (1.0, 200.0), (1.2, 250.0)];
        let bins = bins_from_jwst_rows(&rows);
        assert_eq!(bins.len(), 3);
        let (f, w, v) = bins[1];
        assert!((f - C_LIGHT / 1.0e-6).abs() / f < 1e-12);
        assert!((v - 200.0e-26).abs() / v < 1e-12);
        let w_mid = (C_LIGHT / 0.8e-6 - C_LIGHT / 1.2e-6) * 0.5;
        assert!((w - w_mid).abs() / w_mid < 1e-12);
    }

    #[test]
    fn bins_drop_nonpositive_flux_and_bad_lambda() {
        let rows = [
            (1.0, 200.0),
            (2.0, 0.0),
            (3.0, -5.0),
            (4.0, f64::NAN),
            (0.0, 10.0),
            (5.0, 300.0),
        ];
        let bins = bins_from_jwst_rows(&rows);
        assert_eq!(bins.len(), 2);
        assert!(bins.iter().all(|(f, _, v)| f.is_finite() && *v > 0.0));
    }

    #[test]
    fn mjd_midpoint_epoch() {
        assert_eq!(mjd_to_unix(40587.0), 0.0);
        assert!((mjd_to_unix(59786.5) - 1658836800.0).abs() < 1e-9);
    }

    fn tmp_workdir(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("jwst_test_{}_{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn sidecar_and_ledger_roundtrip_and_idempotent_finalize() {
        let wd = tmp_workdir("sidecar");
        let s1 = spec("WASP-39", "obs_b");
        let s2 = spec("K2-18", "obs_a");
        assert!(write_sidecar(&wd, &s1));
        assert!(ledger_append(
            &wd,
            &s1.obs_id,
            &s1.host,
            s1.bins.len(),
            s1.epoch_tdb
        ));
        assert!(write_sidecar(&wd, &s2));
        assert!(ledger_append(
            &wd,
            &s2.obs_id,
            &s2.host,
            s2.bins.len(),
            s2.epoch_tdb
        ));
        let done = ledger_done(&wd);
        assert!(done.contains("obs_a") && done.contains("obs_b"));
        let first = finalize_workdir(&wd).unwrap();
        let second = finalize_workdir(&wd).unwrap();
        assert_eq!(first, second, "finalize must be byte-identical");
        let parsed = parse_jwst_bin(&first).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].obs_id, "obs_a", "finalize sorts by obs_id");
        assert_eq!(parsed[1].obs_id, "obs_b");
        let _ = std::fs::remove_dir_all(&wd);
    }

    #[test]
    fn sidecar_without_ledger_line_stays_out_of_done() {
        let wd = tmp_workdir("orphan");
        let s = spec("WASP-39", "orphan_obs");
        assert!(write_sidecar(&wd, &s));
        assert!(
            ledger_done(&wd).is_empty(),
            "a sidecar without a ledger line is re-harvested, never trusted"
        );
        let _ = std::fs::remove_dir_all(&wd);
    }

    #[test]
    fn empty_workdir_finalizes_void() {
        let wd = tmp_workdir("empty");
        assert!(finalize_workdir(&wd).is_none());
        let _ = std::fs::remove_dir_all(&wd);
    }
}
