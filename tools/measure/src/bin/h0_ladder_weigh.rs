use std::io::Write;
use std::process::{Command, Stdio};

const ARXIV_EPRINT: &str = "https://arxiv.org/e-print/2012.08534";
const PANTHEON_DAT: &str =
    "https://raw.githubusercontent.com/PantheonPlusSH0ES/DataRelease/main/Pantheon+_Data/4_DISTANCES_AND_COVAR/Pantheon+SH0ES.dat";
const PANTHEON_COV_STATSYS: &str =
    "https://raw.githubusercontent.com/PantheonPlusSH0ES/DataRelease/main/Pantheon+_Data/4_DISTANCES_AND_COVAR/Pantheon+SH0ES_STAT+SYS.cov";
const PANTHEON_COV_STATONLY: &str =
    "https://raw.githubusercontent.com/PantheonPlusSH0ES/DataRelease/main/Pantheon+_Data/4_DISTANCES_AND_COVAR/Pantheon+SH0ES_STATONLY.cov";

const C_KM_S: f64 = 299792.458;
const LN10: f64 = std::f64::consts::LN_10;
const OM_M: f64 = 0.3;

fn lcdm_integral(z: f64, om: f64) -> f64 {
    let n = 400;
    let dz = z / n as f64;
    let mut sum = 0.0;
    for i in 0..n {
        let zt = (i as f64 + 0.5) * dz;
        let e = (om * (1.0 + zt).powi(3) + (1.0 - om)).sqrt();
        sum += dz / e;
    }
    sum
}

fn lcdm_eff_cz(z: f64, om: f64) -> f64 {
    C_KM_S * (1.0 + z) * lcdm_integral(z, om)
}

const PUB_MW1: f64 = -5.915;
const PUB_MW1_ERR: f64 = 0.022;
const PUB_ZP: f64 = -14.0;
const PUB_ZP_ERR: f64 = 6.0;
const PUB_BW_4: f64 = -3.28;
const PUB_BW_4_ERR: f64 = 0.06;
const PUB_ZW_4: f64 = -0.20;
const PUB_ZW_4_ERR: f64 = 0.13;
const PUB_H0: f64 = 73.0;
const PUB_H0_ERR: f64 = 1.4;
const R19_BW: f64 = -3.26;
const R19_ZW: f64 = -0.17;

fn unmeasured(msg: &str) -> ! {
    eprintln!("h0_ladder_weigh: {msg} — the weighing stays unmeasured");
    std::process::exit(1);
}

fn or_unmeasured<T>(opt: Option<T>, msg: &str) -> T {
    match opt {
        Some(v) => v,
        None => unmeasured(msg),
    }
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("600")
        .arg("-A")
        .arg("omegaflow-h0-ladder-weigh/1.0")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(out.stdout)
}

fn has_arg(flag: &str) -> bool {
    std::env::args().any(|a| a == flag)
}

fn upload_asset_bytes(netloc: &str, name: &str, bytes: &[u8]) {
    let dir = std::env::temp_dir().join(format!("omegaflow_h0_ladder_cdn_{}", std::process::id()));
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let path = dir.join(name);
    if std::fs::write(&path, bytes).is_err() {
        let _ = std::fs::remove_dir_all(&dir);
        return;
    }
    let path_str = match path.to_str() {
        Some(s) => s,
        None => {
            let _ = std::fs::remove_dir_all(&dir);
            return;
        }
    };
    omegaflow::cdn::upload_release(netloc, path_str);
    let _ = std::fs::remove_dir_all(&dir);
}

fn sha256_stdin(bytes: &[u8]) -> Option<String> {
    let mut child = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;
    child.stdin.take()?.write_all(bytes).ok()?;
    let out = child.wait_with_output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?;
    text.split_whitespace().next().map(|s| s.to_string())
}

fn gzip_tar_member(tarball: &[u8], member: &str) -> Option<String> {
    let dir = std::env::temp_dir().join(format!("omegaflow_h0_ladder_{}", std::process::id()));
    std::fs::create_dir_all(&dir).ok()?;
    let tar_path = dir.join("source.tar.gz");
    std::fs::write(&tar_path, tarball).ok()?;
    let out = Command::new("tar")
        .arg("-xzOf")
        .arg(&tar_path)
        .arg(member)
        .output()
        .ok()?;
    let _ = std::fs::remove_dir_all(&dir);
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

#[derive(Clone)]
struct Cepheid {
    name: String,
    logp: f64,
    mw: f64,
    sig_mw: f64,
    feh: Option<f64>,
    pi_edr3: Option<f64>,
    sig_edr3: Option<f64>,
    marked: bool,
}

fn parse_cepheids(tex: &str) -> (Vec<Cepheid>, usize) {
    let mut stars = Vec::new();
    let mut absent = 0usize;
    let mut in_data = false;
    for line in tex.lines() {
        let t = line.trim();
        if t.starts_with("\\startdata") {
            in_data = true;
            continue;
        }
        if t.starts_with("\\enddata") {
            break;
        }
        if !in_data {
            continue;
        }
        if t.is_empty() || t.starts_with("\\multicolumn") || t.starts_with("\\table") {
            continue;
        }
        let body = t.trim_end_matches('\\').trim();
        let cells: Vec<&str> = body.split('&').map(|c| c.trim()).collect();
        if cells.len() != 15 {
            continue;
        }
        let marked = cells[0].contains('$');
        let name = cells[0]
            .chars()
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        let logp = match cells[1].parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let mw = match cells[8].parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let sig_mw = match cells[9].parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let feh = cells[10].parse::<f64>().ok();
        let pi = cells[13].parse::<f64>().ok();
        let sig_pi = cells[14].parse::<f64>().ok();
        if pi.is_none() || sig_pi.is_none() {
            absent += 1;
            stars.push(Cepheid {
                name,
                logp,
                mw,
                sig_mw,
                feh,
                pi_edr3: None,
                sig_edr3: None,
                marked,
            });
            continue;
        }
        stars.push(Cepheid {
            name,
            logp,
            mw,
            sig_mw,
            feh,
            pi_edr3: pi,
            sig_edr3: sig_pi,
            marked,
        });
    }
    (stars, absent)
}

#[derive(Clone)]
struct SnRow {
    cid: String,
    zhd: f64,
    mbcorr: f64,
    ceph_dist: Option<f64>,
    is_cal: bool,
    used_hf: bool,
}

fn parse_dat(dat: &str) -> Option<Vec<SnRow>> {
    let mut rows = Vec::new();
    for line in dat.lines().skip(1) {
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() < 15 {
            continue;
        }
        let cid = t[0].to_string();
        let zhd = t[2].parse::<f64>().ok()?;
        let mbcorr = t[8].parse::<f64>().ok()?;
        let is_cal = t[13].parse::<i64>().ok()? == 1;
        let used_hf = t[14].parse::<i64>().ok()? == 1;
        let ceph_dist = if is_cal {
            let v = t[12].parse::<f64>().ok()?;
            if v.is_finite() && v > 0.0 {
                Some(v)
            } else {
                None
            }
        } else {
            None
        };
        if !zhd.is_finite() || !mbcorr.is_finite() {
            return None;
        }
        rows.push(SnRow {
            cid,
            zhd,
            mbcorr,
            ceph_dist,
            is_cal,
            used_hf,
        });
    }
    Some(rows)
}

fn parse_cov(cov: &str) -> Option<(usize, Vec<f64>)> {
    let mut lines = cov.lines();
    let n: usize = lines.next()?.trim().parse().ok()?;
    let mut m = Vec::with_capacity(n * n);
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let v = line.parse::<f64>().ok()?;
        if !v.is_finite() {
            return None;
        }
        m.push(v);
    }
    if m.len() != n * n {
        return None;
    }
    Some((n, m))
}

fn cov_get(n: usize, m: &[f64], i: usize, j: usize) -> f64 {
    m[i * n + j]
}

fn duplicate_pairs(rows: &[SnRow]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..rows.len() {
        for j in (i + 1)..rows.len() {
            if rows[i].cid == rows[j].cid {
                pairs.push((i, j));
            }
        }
    }
    pairs
}

fn pl_weight(sig_mw: f64, sig_pi: f64, pi: f64) -> f64 {
    let mu_err_par = 5.0 / LN10 * sig_pi / pi;
    1.0 / (sig_mw * sig_mw + mu_err_par * mu_err_par)
}

fn solve_small(n: usize, a: &[f64], b: &[f64]) -> Option<Vec<f64>> {
    let mut aug = vec![0.0f64; n * (n + 1)];
    for i in 0..n {
        for j in 0..n {
            aug[i * (n + 1) + j] = a[i * n + j];
        }
        aug[i * (n + 1) + n] = b[i];
    }
    for col in 0..n {
        let mut pivot = col;
        for r in (col + 1)..n {
            if aug[r * (n + 1) + col].abs() > aug[pivot * (n + 1) + col].abs() {
                pivot = r;
            }
        }
        if aug[pivot * (n + 1) + col].abs() < 1e-15 {
            return None;
        }
        if pivot != col {
            for k in 0..=n {
                aug.swap(col * (n + 1) + k, pivot * (n + 1) + k);
            }
        }
        let d = aug[col * (n + 1) + col];
        for k in 0..=n {
            aug[col * (n + 1) + k] /= d;
        }
        for r in 0..n {
            if r == col {
                continue;
            }
            let f = aug[r * (n + 1) + col];
            for k in 0..=n {
                aug[r * (n + 1) + k] -= f * aug[col * (n + 1) + k];
            }
        }
    }
    Some((0..n).map(|i| aug[i * (n + 1) + n]).collect())
}

struct PlFit {
    mw1: f64,
    sig_mw1: f64,
    bw: f64,
    sig_bw: f64,
    zw: f64,
    sig_zw: f64,
    zp: f64,
    sig_zp: f64,
    chisq: f64,
    ndf: usize,
}

fn pl_fit(stars: &[Cepheid], free_slope: bool) -> Option<PlFit> {
    let fit_rows: Vec<&Cepheid> = stars
        .iter()
        .filter(|s| s.pi_edr3.is_some() && s.sig_edr3.is_some() && s.feh.is_some())
        .collect();
    if fit_rows.is_empty() {
        return None;
    }
    let n = fit_rows.len();
    let np = if free_slope { 4 } else { 2 };
    let mut bw = R19_BW;
    let mut zw = R19_ZW;
    let mut mw1 = 0.0;
    let mut zp = 0.0;
    for _ in 0..40 {
        let mut ata = vec![0.0f64; np * np];
        let mut atb = vec![0.0f64; np];
        let mut chisq = 0.0;
        for s in &fit_rows {
            let pi = s.pi_edr3?;
            let sig_pi = s.sig_edr3?;
            let feh = s.feh?;
            let w = pl_weight(s.sig_mw, sig_pi, pi);
            let mu_obs = 10.0 - 5.0 * (pi + zp * 1e-3).log10();
            let mu_mod = s.mw - (mw1 + bw * (s.logp - 1.0) + zw * feh);
            let r = mu_mod - mu_obs;
            let dr_dzp = 5.0 / LN10 * 1e-3 / (pi + zp * 1e-3);
            let mut j = vec![0.0f64; np];
            if free_slope {
                j[0] = -1.0;
                j[1] = -(s.logp - 1.0);
                j[2] = -feh;
                j[3] = dr_dzp;
            } else {
                j[0] = -1.0;
                j[1] = dr_dzp;
            }
            chisq += w * r * r;
            for a in 0..np {
                for b in 0..np {
                    ata[a * np + b] += w * j[a] * j[b];
                }
                atb[a] += -w * j[a] * r;
            }
        }
        let step = solve_small(np, &ata, &atb)?;
        if free_slope {
            mw1 += step[0];
            bw += step[1];
            zw += step[2];
            zp += step[3];
        } else {
            mw1 += step[0];
            zp += step[1];
        }
        let max_step = step.iter().map(|v| v.abs()).fold(0.0, f64::max);
        if max_step < 1e-9 {
            let inv = invert_small(np, &ata)?;
            let ndf = n - np;
            return Some(PlFit {
                mw1,
                sig_mw1: inv[0].sqrt(),
                bw,
                sig_bw: if free_slope { inv[np + 1].sqrt() } else { 0.0 },
                zw,
                sig_zw: if free_slope {
                    inv[2 * (np + 1)].sqrt()
                } else {
                    0.0
                },
                zp,
                sig_zp: inv[(np - 1) * (np + 1)].sqrt(),
                chisq,
                ndf,
            });
        }
    }
    None
}

fn invert_small(n: usize, a: &[f64]) -> Option<Vec<f64>> {
    let mut b = vec![0.0f64; n * n];
    for i in 0..n {
        b[i * n + i] = 1.0;
    }
    let mut aug = vec![0.0f64; n * (2 * n)];
    for i in 0..n {
        for j in 0..n {
            aug[i * 2 * n + j] = a[i * n + j];
            aug[i * 2 * n + n + j] = b[i * n + j];
        }
    }
    for col in 0..n {
        let mut pivot = col;
        for r in (col + 1)..n {
            if aug[r * 2 * n + col].abs() > aug[pivot * 2 * n + col].abs() {
                pivot = r;
            }
        }
        if aug[pivot * 2 * n + col].abs() < 1e-15 {
            return None;
        }
        if pivot != col {
            for k in 0..(2 * n) {
                aug.swap(col * 2 * n + k, pivot * 2 * n + k);
            }
        }
        let d = aug[col * 2 * n + col];
        for k in 0..(2 * n) {
            aug[col * 2 * n + k] /= d;
        }
        for r in 0..n {
            if r == col {
                continue;
            }
            let f = aug[r * 2 * n + col];
            for k in 0..(2 * n) {
                aug[r * 2 * n + k] -= f * aug[col * 2 * n + k];
            }
        }
    }
    let mut inv = vec![0.0f64; n * n];
    for i in 0..n {
        for j in 0..n {
            inv[i * n + j] = aug[i * 2 * n + n + j];
        }
    }
    Some(inv)
}

fn cholesky(n: usize, a: &[f64]) -> Option<Vec<f64>> {
    let mut l = vec![0.0f64; n * n];
    for i in 0..n {
        for j in 0..=i {
            let mut sum = a[i * n + j];
            for k in 0..j {
                sum -= l[i * n + k] * l[j * n + k];
            }
            if i == j {
                if sum <= 0.0 {
                    return None;
                }
                l[i * n + i] = sum.sqrt();
            } else {
                l[i * n + j] = sum / l[j * n + j];
            }
        }
    }
    Some(l)
}

fn cholesky_solve(n: usize, l: &[f64], b: &[f64]) -> Vec<f64> {
    let mut y = vec![0.0f64; n];
    for i in 0..n {
        let mut sum = b[i];
        for k in 0..i {
            sum -= l[i * n + k] * y[k];
        }
        y[i] = sum / l[i * n + i];
    }
    let mut x = vec![0.0f64; n];
    for i in (0..n).rev() {
        let mut sum = y[i];
        for k in (i + 1)..n {
            sum -= l[k * n + i] * x[k];
        }
        x[i] = sum / l[i * n + i];
    }
    x
}

struct GlsResult {
    mb: f64,
    sig_mb: f64,
    ab: f64,
    sig_ab: f64,
    cov_mb_ab: f64,
}

fn gls_combined(
    n: usize,
    cov: &[f64],
    cal_idx: &[usize],
    hf_idx: &[usize],
    mb_vals: &[f64],
    ab_vals: &[f64],
) -> Option<GlsResult> {
    let nc = cal_idx.len();
    let nh = hf_idx.len();
    let total = nc + nh;
    let mut c = vec![0.0f64; total * total];
    for a in 0..nc {
        for b in 0..nc {
            c[a * total + b] = cov_get(n, cov, cal_idx[a], cal_idx[b]);
        }
        for b in 0..nh {
            c[a * total + nc + b] = -0.2 * cov_get(n, cov, cal_idx[a], hf_idx[b]);
            c[(nc + b) * total + a] = c[a * total + nc + b];
        }
    }
    for a in 0..nh {
        for b in 0..nh {
            c[(nc + a) * total + nc + b] = 0.04 * cov_get(n, cov, hf_idx[a], hf_idx[b]);
        }
    }
    let l = cholesky(total, &c)?;
    let ones_cal: Vec<f64> = {
        let mut v = vec![0.0; total];
        for a in 0..nc {
            v[a] = 1.0;
        }
        v
    };
    let ones_hf: Vec<f64> = {
        let mut v = vec![0.0; total];
        for b in 0..nh {
            v[nc + b] = 1.0;
        }
        v
    };
    let d: Vec<f64> = {
        let mut v = vec![0.0; total];
        for a in 0..nc {
            v[a] = mb_vals[a];
        }
        for b in 0..nh {
            v[nc + b] = ab_vals[b];
        }
        v
    };
    let cinv_ones_cal = cholesky_solve(total, &l, &ones_cal);
    let cinv_ones_hf = cholesky_solve(total, &l, &ones_hf);
    let cinv_d = cholesky_solve(total, &l, &d);
    let s11 = dot(total, &ones_cal, &cinv_ones_cal);
    let s22 = dot(total, &ones_hf, &cinv_ones_hf);
    let s12 = dot(total, &ones_cal, &cinv_ones_hf);
    let y1 = dot(total, &ones_cal, &cinv_d);
    let y2 = dot(total, &ones_hf, &cinv_d);
    let ata = [s11, s12, s12, s22];
    let atb = [y1, y2];
    let sol = solve_small(2, &ata, &atb)?;
    let inv = invert_small(2, &ata)?;
    Some(GlsResult {
        mb: sol[0],
        sig_mb: inv[0].sqrt(),
        ab: sol[1],
        sig_ab: inv[3].sqrt(),
        cov_mb_ab: inv[1],
    })
}

fn dot(n: usize, a: &[f64], b: &[f64]) -> f64 {
    (0..n).map(|i| a[i] * b[i]).sum()
}

struct Gate {
    name: &'static str,
    own: f64,
    own_err: f64,
    published: f64,
    published_err: f64,
}

fn within_one_sigma(g: &Gate) -> bool {
    let delta = (g.own - g.published).abs();
    let sigma = (g.own_err * g.own_err + g.published_err * g.published_err).sqrt();
    delta <= sigma
}

fn diagonal_mean(vals: &[(f64, f64)]) -> Option<(f64, f64)> {
    let mut num = 0.0;
    let mut den = 0.0;
    for (v, e) in vals {
        let w = 1.0 / (e * e);
        num += w * v;
        den += w;
    }
    if den <= 0.0 || !den.is_finite() {
        return None;
    }
    let mean = num / den;
    let err = 1.0 / den.sqrt();
    if !mean.is_finite() || !err.is_finite() {
        return None;
    }
    Some((mean, err))
}

fn main() {
    let ci_mode = has_arg("--ci-mode");
    let tarball = or_unmeasured(
        curl_bytes(ARXIV_EPRINT),
        "the arXiv e-print returned no bytes",
    );
    let tarball_sha = or_unmeasured(sha256_stdin(&tarball), "the arXiv tarball hash is absent");
    let tex = or_unmeasured(
        gzip_tar_member(&tarball, "bigtable_redux3.tex"),
        "bigtable_redux3.tex is absent from the arXiv package",
    );
    let (stars, absent) = parse_cepheids(&tex);
    if stars.is_empty() {
        unmeasured("the Cepheid table carries no parsed rows");
    }

    let dat_bytes = or_unmeasured(
        curl_bytes(PANTHEON_DAT),
        "Pantheon+SH0ES.dat returned no bytes",
    );
    let dat_sha = or_unmeasured(
        sha256_stdin(&dat_bytes),
        "the Pantheon+SH0ES.dat hash is absent",
    );
    let dat_text = or_unmeasured(
        String::from_utf8(dat_bytes.clone()).ok(),
        "Pantheon+SH0ES.dat is not utf-8",
    );
    let rows = or_unmeasured(
        parse_dat(&dat_text),
        "Pantheon+SH0ES.dat carries no parseable rows",
    );

    let cov_bytes = or_unmeasured(
        curl_bytes(PANTHEON_COV_STATSYS),
        "the STAT+SYS covariance returned no bytes",
    );
    let cov_sha = or_unmeasured(
        sha256_stdin(&cov_bytes),
        "the STAT+SYS covariance hash is absent",
    );
    let cov_text = or_unmeasured(
        String::from_utf8(cov_bytes.clone()).ok(),
        "the STAT+SYS covariance is not utf-8",
    );
    let (n, cov) = or_unmeasured(
        parse_cov(&cov_text),
        "the STAT+SYS covariance is not a square matrix",
    );
    if n != rows.len() {
        unmeasured("the covariance dimension does not match the SN table");
    }

    let statonly_bytes = or_unmeasured(
        curl_bytes(PANTHEON_COV_STATONLY),
        "the STATONLY covariance returned no bytes",
    );
    let statonly_text = or_unmeasured(
        String::from_utf8(statonly_bytes).ok(),
        "the STATONLY covariance is not utf-8",
    );
    let (sn, statonly) = or_unmeasured(
        parse_cov(&statonly_text),
        "the STATONLY covariance is not a square matrix",
    );
    if sn != n {
        unmeasured("the STATONLY covariance dimension differs from the SN table");
    }

    let pairs = duplicate_pairs(&rows);
    let aligned = pairs
        .iter()
        .filter(|&&(i, j)| cov_get(n, &statonly, i, j) != 0.0)
        .count();
    if pairs.is_empty() {
        unmeasured("the SN table carries no duplicate CIDs to certify the covariance row order");
    }
    let aligned_frac = aligned as f64 / pairs.len() as f64;
    if aligned_frac < 0.9 {
        unmeasured(&format!(
            "the covariance row order is uncertified (duplicate-CID alignment {aligned}/{} )",
            pairs.len()
        ));
    }

    let fit2 = or_unmeasured(
        pl_fit(&stars, false),
        "the parallax-space anchor fit (2-param) did not converge",
    );
    let fit4 = or_unmeasured(
        pl_fit(&stars, true),
        "the parallax-space anchor fit (4-param) did not converge",
    );

    let cal_idx: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, r)| r.is_cal)
        .map(|(i, _)| i)
        .collect();
    let hf_idx: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, r)| r.used_hf)
        .map(|(i, _)| i)
        .collect();
    if cal_idx.is_empty() || hf_idx.is_empty() {
        unmeasured("the calibrator or Hubble-flow subset is empty");
    }

    let mb_vals: Vec<f64> = cal_idx
        .iter()
        .map(|&i| match rows[i].ceph_dist {
            Some(d) => rows[i].mbcorr - d,
            None => unmeasured(
                "a calibrator carries no Cepheid distance — the calibration stays unmeasured",
            ),
        })
        .collect();
    let ab_vals: Vec<f64> = hf_idx
        .iter()
        .map(|&i| lcdm_eff_cz(rows[i].zhd, OM_M).log10() - 0.2 * rows[i].mbcorr)
        .collect();

    let gls = or_unmeasured(
        gls_combined(n, &cov, &cal_idx, &hf_idx, &mb_vals, &ab_vals),
        "the combined covariance fit is not positive definite",
    );

    let mb_diag: Vec<(f64, f64)> = cal_idx
        .iter()
        .map(|&i| {
            let mb = match rows[i].ceph_dist {
                Some(d) => rows[i].mbcorr - d,
                None => unmeasured(
                    "a calibrator carries no Cepheid distance — the calibration stays unmeasured",
                ),
            };
            let e = cov_get(n, &cov, i, i).sqrt();
            (mb, e)
        })
        .collect();
    let ab_diag: Vec<(f64, f64)> = hf_idx
        .iter()
        .map(|&i| {
            let ab = lcdm_eff_cz(rows[i].zhd, OM_M).log10() - 0.2 * rows[i].mbcorr;
            let e = 0.2 * cov_get(n, &cov, i, i).sqrt();
            (ab, e)
        })
        .collect();
    let (mb_diag_mean, mb_diag_err) =
        or_unmeasured(diagonal_mean(&mb_diag), "the diagonal M_B collapses");
    let (ab_diag_mean, ab_diag_err) =
        or_unmeasured(diagonal_mean(&ab_diag), "the diagonal a_B collapses");

    let anchor_shift = fit2.mw1 - PUB_MW1;
    let anchor_shift_err = (fit2.sig_mw1 * fit2.sig_mw1 + PUB_MW1_ERR * PUB_MW1_ERR).sqrt();
    let mb_own = gls.mb + anchor_shift;
    let mb_own_err = (gls.sig_mb * gls.sig_mb + anchor_shift_err * anchor_shift_err).sqrt();
    let log_h0 = 0.2 * mb_own + gls.ab + 5.0;
    let log_h0_err =
        (0.04 * mb_own_err * mb_own_err + gls.sig_ab * gls.sig_ab + 2.0 * 0.2 * gls.cov_mb_ab)
            .sqrt();
    let h0 = 10f64.powf(log_h0);
    let h0_err = h0 * LN10 * log_h0_err;

    let log_h0_diag = 0.2 * (mb_diag_mean + anchor_shift) + ab_diag_mean + 5.0;
    let log_h0_diag_err = (0.04
        * (mb_diag_err * mb_diag_err + anchor_shift_err * anchor_shift_err)
        + ab_diag_err * ab_diag_err)
        .sqrt();
    let h0_diag = 10f64.powf(log_h0_diag);
    let h0_diag_err = h0_diag * LN10 * log_h0_diag_err;

    let n_fit = stars
        .iter()
        .filter(|s| s.pi_edr3.is_some() && s.feh.is_some())
        .count();
    let n_marked = stars.iter().filter(|s| s.marked).count();
    let absent_names: Vec<&str> = stars
        .iter()
        .filter(|s| s.pi_edr3.is_none())
        .map(|s| s.name.as_str())
        .collect();
    let marked_names: Vec<&str> = stars
        .iter()
        .filter(|s| s.marked)
        .map(|s| s.name.as_str())
        .collect();
    let unique_cal: std::collections::HashSet<&str> =
        cal_idx.iter().map(|&i| rows[i].cid.as_str()).collect();
    let zmin = hf_idx
        .iter()
        .map(|&i| rows[i].zhd)
        .fold(f64::INFINITY, f64::min);
    let zmax = hf_idx
        .iter()
        .map(|&i| rows[i].zhd)
        .fold(f64::NEG_INFINITY, f64::max);

    println!("h0_ladder_weigh: arXiv 2012.08534 sha256={tarball_sha} | Pantheon+SH0ES.dat sha256={dat_sha} | STAT+SYS.cov sha256={cov_sha}");
    println!(
        "h0_ladder_weigh: Cepheid table N={} rows (prose cites 75), {} π_EDR3 absent, {} fitted, {} footnote-marked",
        stars.len(), absent, n_fit, n_marked
    );
    println!(
        "h0_ladder_weigh: π_EDR3 absent stars: {} | footnote-marked stars: {}",
        absent_names.join(", "),
        marked_names.join(", ")
    );
    println!(
        "h0_ladder_weigh: covariance row order certified — duplicate-CID alignment {aligned}/{}",
        pairs.len()
    );
    println!(
        "h0_ladder_weigh: anchor 2-param (R19-fixed b_W={R19_BW} Z_W={R19_ZW}) M_W1={:.3} ± {:.3} | zp={:.0} ± {:.0} µas | χ²/ndf={:.1}/{} | published M_W1={PUB_MW1} ± {PUB_MW1_ERR}, zp={PUB_ZP} ± {PUB_ZP_ERR}",
        fit2.mw1, fit2.sig_mw1, fit2.zp, fit2.sig_zp, fit2.chisq, fit2.ndf
    );
    println!(
        "h0_ladder_weigh: anchor 4-param M_W1={:.3} ± {:.3} | b_W={:.2} ± {:.2} | Z_W={:.2} ± {:.2} | zp={:.0} ± {:.0} µas | χ²/ndf={:.1}/{} | published b_W={PUB_BW_4} ± {PUB_BW_4_ERR}, Z_W={PUB_ZW_4} ± {PUB_ZW_4_ERR}",
        fit4.mw1, fit4.sig_mw1, fit4.bw, fit4.sig_bw, fit4.zw, fit4.sig_zw, fit4.zp, fit4.sig_zp, fit4.chisq, fit4.ndf
    );
    println!(
        "h0_ladder_weigh: anchor shift Δ = M_W1 − ({PUB_MW1}) = {:.3} ± {:.3} mag",
        anchor_shift, anchor_shift_err
    );
    println!(
        "h0_ladder_weigh: calibrators N={} ({} unique CIDs, paper cites 42) | M_B={:.4} ± {:.4} mag (covariance) | diagonal checkpoint M_B={:.4} ± {:.4}",
        cal_idx.len(), unique_cal.len(), gls.mb, gls.sig_mb, mb_diag_mean, mb_diag_err
    );
    println!(
        "h0_ladder_weigh: Hubble flow N={}, zHD {:.5}..{:.5} | a_B={:.4} ± {:.4} (covariance, flat ΛCDM Ωm={OM_M}) | diagonal checkpoint a_B={:.4} ± {:.4}",
        hf_idx.len(), zmin, zmax, gls.ab, gls.sig_ab, ab_diag_mean, ab_diag_err
    );
    println!(
        "h0_ladder_weigh: H₀ = {:.2} ± {:.2} km/s/Mpc (covariance) | diagonal H₀ = {:.2} ± {:.2} | published {PUB_H0} ± {PUB_H0_ERR}",
        h0, h0_err, h0_diag, h0_diag_err
    );

    let gates = [
        Gate {
            name: "M_W1",
            own: fit2.mw1,
            own_err: fit2.sig_mw1,
            published: PUB_MW1,
            published_err: PUB_MW1_ERR,
        },
        Gate {
            name: "zp",
            own: fit2.zp,
            own_err: fit2.sig_zp,
            published: PUB_ZP,
            published_err: PUB_ZP_ERR,
        },
        Gate {
            name: "b_W",
            own: fit4.bw,
            own_err: fit4.sig_bw,
            published: PUB_BW_4,
            published_err: PUB_BW_4_ERR,
        },
        Gate {
            name: "Z_W",
            own: fit4.zw,
            own_err: fit4.sig_zw,
            published: PUB_ZW_4,
            published_err: PUB_ZW_4_ERR,
        },
        Gate {
            name: "H₀",
            own: h0,
            own_err: h0_err,
            published: PUB_H0,
            published_err: PUB_H0_ERR,
        },
    ];
    let rifts: Vec<&Gate> = gates.iter().filter(|g| !within_one_sigma(g)).collect();
    if rifts.is_empty() {
        println!(
            "h0_ladder_weigh: reproduction gate PASS — every parameter within 1σ of the published value"
        );
    } else {
        for g in &rifts {
            let delta = (g.own - g.published).abs();
            let sigma = (g.own_err * g.own_err + g.published_err * g.published_err).sqrt();
            println!(
                "h0_ladder_weigh: reproduction gate RIFT — {} own {:.4} ± {:.4} vs published {:.4} ± {:.4} ({}σ) — the deviation is the finding, registered, not hidden",
                g.name, g.own, g.own_err, g.published, g.published_err, delta / sigma
            );
        }
        println!(
            "h0_ladder_weigh: suspicion order on a rift — weigh the own chain first (units → parser → fit), then the published value"
        );
    }

    if ci_mode {
        upload_asset_bytes("arxiv.org", "2012.08534.tar.gz", &tarball);
        upload_asset_bytes(
            "raw.githubusercontent.com",
            "Pantheon+SH0ES.dat",
            &dat_bytes,
        );
        upload_asset_bytes(
            "raw.githubusercontent.com",
            "Pantheon+SH0ES_STAT+SYS.cov",
            &cov_bytes,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cepheid_rows_and_counts_absent() {
        let tex = "\\startdata\nAA-GEM & 1.053 & 9.9130 & 0.029 & 8.542 & 0.025 & 7.348 & 0.017 & 6.860 & 0.023 & -0.080 & 0.259 & 0.008 & 0.311 & 0.019 \\\\\nCY-AUR & 0.9 & 8.0 & 0.02 & 7.0 & 0.02 & 6.0 & 0.02 & 5.0 & 0.02 & -0.1 & 0.2 & 0.01 & \\nd & \\nd \\\\\n\\enddata\n";
        let (stars, absent) = parse_cepheids(tex);
        assert_eq!(stars.len(), 2);
        assert_eq!(absent, 1);
        assert_eq!(stars[0].name, "AA-GEM");
        assert!(stars[0].pi_edr3.is_some());
        assert!(stars[1].pi_edr3.is_none());
    }

    #[test]
    fn parses_dat_rows() {
        let dat = "CID IDSURVEY zHD zHDERR zCMB zCMBERR zHEL zHELERR m_b_corr m_b_corr_err_DIAG MU_SH0ES MU_SH0ES_ERR_DIAG CEPH_DIST IS_CALIBRATOR USED_IN_SH0ES_HF\n2011fe 51 0.00122 0.00084 0.00122 2e-05 0.00082 2e-05 9.74571 1.51621 28.9987 1.51645 29.177 1 0\n123 100 0.05 0.0 0.05 0.0 0.05 0.0 15.0 0.1 35.0 0.1 -9 0 1\n";
        let rows = parse_dat(dat).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows[0].is_cal);
        assert_eq!(rows[0].ceph_dist, Some(29.177));
        assert!(!rows[1].is_cal);
        assert!(rows[1].used_hf);
        assert_eq!(rows[1].ceph_dist, None);
    }

    #[test]
    fn cov_parse_square_matrix() {
        let cov = "2\n0.04\n0.01\n0.01\n0.09\n";
        let (n, m) = parse_cov(cov).unwrap();
        assert_eq!(n, 2);
        assert_eq!(cov_get(2, &m, 0, 0), 0.04);
        assert_eq!(cov_get(2, &m, 1, 1), 0.09);
    }

    #[test]
    fn cholesky_solve_inverts_diagonal() {
        let a = [4.0, 0.0, 0.0, 9.0];
        let l = cholesky(2, &a).unwrap();
        let x = cholesky_solve(2, &l, &[1.0, 1.0]);
        assert!((x[0] - 0.25).abs() < 1e-12);
        assert!((x[1] - 1.0 / 9.0).abs() < 1e-12);
    }

    #[test]
    fn gls_recovers_known_means() {
        let n = 2;
        let cov = [0.04, 0.0, 0.0, 0.09];
        let gls = gls_combined(n, &cov, &[0], &[1], &[10.0], &[20.0]).unwrap();
        assert!((gls.mb - 10.0).abs() < 1e-9);
        assert!((gls.ab - 20.0).abs() < 1e-9);
        assert!((gls.sig_mb - 0.2).abs() < 1e-9);
        assert!((gls.sig_ab - 0.06).abs() < 1e-9);
    }

    #[test]
    fn diagonal_mean_matches_inverse_variance() {
        let (m, e) = diagonal_mean(&[(1.0, 0.1), (2.0, 0.2)]).unwrap();
        assert!((m - 1.2).abs() < 1e-9);
        let expect = 1.0f64 / (100.0f64 + 25.0f64).sqrt();
        assert!((e - expect).abs() < 1e-12);
    }
}
