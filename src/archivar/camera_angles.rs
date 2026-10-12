pub struct CameraAngleFrame {
    pub jd: f64,
    pub fds: String,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
}

const JD_FDS_LABEL: &str = "JULIAN_DATE&FDS";
const SXSYSZ_LABEL: &str = "SXSYSZ";
const C1C2C3_LABEL: &str = "C1C2C3";

fn fortran_f64(token: &str) -> Option<f64> {
    let normalized = token.replace(['D', 'd'], "E");
    normalized.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn record(lines: &[&str; 3]) -> Option<CameraAngleFrame> {
    let jd_tokens: Vec<&str> = lines[0].split_whitespace().collect();
    if jd_tokens.len() < 3 || jd_tokens[jd_tokens.len() - 1] != JD_FDS_LABEL {
        return None;
    }
    let jd = fortran_f64(jd_tokens[0])?;
    let fds = jd_tokens[1].to_string();

    let s_tokens: Vec<&str> = lines[1].split_whitespace().collect();
    if s_tokens.len() < 4 || s_tokens[s_tokens.len() - 1] != SXSYSZ_LABEL {
        return None;
    }
    let sx = fortran_f64(s_tokens[0])?;
    let sy = fortran_f64(s_tokens[1])?;
    let sz = fortran_f64(s_tokens[2])?;

    let c_tokens: Vec<&str> = lines[2].split_whitespace().collect();
    if c_tokens.len() < 4 || c_tokens[c_tokens.len() - 1] != C1C2C3_LABEL {
        return None;
    }
    let c1 = fortran_f64(c_tokens[0])?;
    let c2 = fortran_f64(c_tokens[1])?;
    let c3 = fortran_f64(c_tokens[2])?;

    Some(CameraAngleFrame {
        jd,
        fds,
        sx,
        sy,
        sz,
        c1,
        c2,
        c3,
    })
}

pub fn parse_camera_angles(text: &str) -> Vec<CameraAngleFrame> {
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    let mut frames = Vec::new();
    let mut i = 0;
    while i + 2 < lines.len() {
        if let Some(frame) = record(&[lines[i], lines[i + 1], lines[i + 2]]) {
            frames.push(frame);
        }
        i += 3;
    }
    frames
}

#[cfg(test)]
mod tests {
    use super::*;

    const SYNTHETIC: &str = "\
  0.2442136377841061D+07    00000050                            JULIAN_DATE&FDS
  0.1074370232710200D+05 -0.2377632467018400D+04  0.1995419166206000D+04 SXSYSZ
  0.1602669234243153D+03 -0.6646872148909559D+01  0.3298682679340727D+03 C1C2C3
  0.2442136378813480D+07    00000052                            JULIAN_DATE&FDS
  0.1158681297920100D+05 -0.2343260054703900D+04  0.2296242218930500D+04 SXSYSZ
  0.1615501867295870D+03 -0.9314645872279716D+01  0.3303297993822147D+03 C1C2C3
";

    #[test]
    fn parses_two_synthetic_records() {
        let frames = parse_camera_angles(SYNTHETIC);
        assert_eq!(frames.len(), 2);
        assert!((frames[0].jd - 2442136.377841061).abs() < 0.001);
        assert_eq!(frames[0].fds, "00000050");
        assert!((frames[0].sx - 10743.70232710200).abs() < 1e-6);
        assert!((frames[0].c1 - 160.2669234243153).abs() < 1e-9);
        assert_eq!(frames[1].fds, "00000052");
        assert!((frames[1].sy - (-2343.260054703900)).abs() < 1e-6);
    }

    #[test]
    fn skips_mismatched_label() {
        let broken = "\
  0.2442136377841061D+07    00000050                            JULIAN_DATE&FDS
  0.1074370232710200D+05 -0.2377632467018400D+04  0.1995419166206000D+04 WRONGLABEL
  0.1602669234243153D+03 -0.6646872148909559D+01  0.3298682679340727D+03 C1C2C3
";
        assert!(parse_camera_angles(broken).is_empty());
    }
}
