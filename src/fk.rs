// Text-FK-Reader: FRAME_*/TKFRAME_-Blöcke aus SPICE-Textkerneln (.tf).
// Quelle der Frame-Assoziationen (K05): ersetzt die hartcodierten
// PA-Frame-Tabellen im Flattener. TKFRAME SPEC='ANGLES' → Rotationsmatrix
// (SPICE-Konvention: Rotation um AXIS1 um ANGLE1, dann AXIS2, dann AXIS3).

use std::collections::HashMap;
use std::path::Path;

use crate::mat::matmul;

#[derive(Clone, Debug)]
pub struct TkFrame {
    pub spec: Option<String>,
    pub relative: Option<String>,
    pub angles: Option<[f64; 3]>,
    pub axes: Option<[i32; 3]>,
    pub units: Option<String>,
    pub matrix: Option<[f64; 9]>,
}

#[derive(Clone, Debug)]
pub struct FkFrame {
    pub id: i32,
    pub name: String,
    pub class: Option<i32>,
    pub class_id: Option<i32>,
    pub center: Option<i32>,
    pub tk: Option<TkFrame>,
}

pub struct FkFile {
    pub frames: Vec<FkFrame>,
    pub by_id: HashMap<i32, usize>,
}

fn parse_number_list(s: &str) -> Option<Vec<f64>> {
    let cleaned: String = s.chars().filter(|c| !matches!(c, '(' | ')')).collect();
    let mut out = Vec::new();
    for part in cleaned.split(',') {
        for tok in part.split_whitespace() {
            match tok.parse::<f64>() {
                Ok(v) => out.push(v),
                Err(_) => return None,
            }
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn parse_value(raw: &str) -> String {
    raw.trim()
        .trim_start_matches('\'')
        .trim_end_matches('\'')
        .to_string()
}

impl FkFile {
    pub fn parse(text: &str) -> FkFile {
        let mut frames: Vec<FkFrame> = Vec::new();
        let mut pending: HashMap<i32, TkFrame> = HashMap::new();
        let mut tk_by_frame: HashMap<i32, TkFrame> = HashMap::new();
        let lines: Vec<&str> = text.lines().collect();
        let mut li = 0usize;
        while li < lines.len() {
            let line = lines[li].trim();
            li += 1;
            if line.is_empty() || line.starts_with("\\") || line.starts_with("--") {
                continue;
            }
            let Some(eq) = line.find('=') else {
                continue;
            };
            let key = line[..eq].trim();
            let mut value = parse_value(&line[eq + 1..]);
            let mut balance: i64 = value
                .chars()
                .map(|c| match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                })
                .sum();
            while balance > 0 && li < lines.len() {
                let cont = lines[li].trim();
                li += 1;
                balance += cont
                    .chars()
                    .map(|c| match c {
                        '(' => 1,
                        ')' => -1,
                        _ => 0,
                    })
                    .sum::<i64>();
                value.push(' ');
                value.push_str(cont);
            }
            if key == "TKFRAME" {
                continue;
            }
            if let Some(frame_name) = key.strip_prefix("FRAME_") {
                let is_field = match frame_name.find('_') {
                    Some(i) => frame_name[..i].parse::<i32>().is_ok(),
                    None => false,
                };
                if is_field {
                    let i = frame_name.find('_').unwrap();
                    let id: i32 = frame_name[..i].parse().unwrap();
                    let field = &frame_name[i + 1..];
                    let slot = frames.iter_mut().find(|f| f.id == id);
                    match field {
                        "NAME" => {
                            if let Some(f) = slot {
                                f.name = value;
                            }
                        }
                        "CLASS" => {
                            if let Some(f) = slot {
                                f.class = value.parse().ok();
                            }
                        }
                        "CLASS_ID" => {
                            if let Some(f) = slot {
                                f.class_id = value.parse().ok();
                            }
                        }
                        "CENTER" => {
                            if let Some(f) = slot {
                                f.center = value.parse().ok();
                            }
                        }
                        _ => {}
                    }
                    continue;
                }
                if let Ok(id) = value.parse::<i32>() {
                    if let Some(tk) = pending.remove(&id) {
                        tk_by_frame.insert(id, tk);
                    }
                    if !frames.iter().any(|f| f.id == id) {
                        frames.push(FkFrame {
                            id,
                            name: frame_name.to_string(),
                            class: None,
                            class_id: None,
                            center: None,
                            tk: None,
                        });
                    }
                }
                continue;
            }
            if let Some(tk_part) = key.strip_prefix("TKFRAME_") {
                let rest = tk_part;
                let (id_part, field) = match rest.find('_') {
                    Some(i) => (&rest[..i], &rest[i + 1..]),
                    None => continue,
                };
                let id: i32 = match id_part.parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let tk = pending.entry(id).or_insert_with(|| TkFrame {
                    spec: None,
                    relative: None,
                    angles: None,
                    axes: None,
                    units: None,
                    matrix: None,
                });
                match field {
                    "SPEC" => tk.spec = Some(value),
                    "RELATIVE" => tk.relative = Some(value),
                    "ANGLES" => {
                        if let Some(v) = parse_number_list(&value) {
                            if v.len() == 3 {
                                tk.angles = Some([v[0], v[1], v[2]]);
                            }
                        }
                    }
                    "AXES" => {
                        if let Some(v) = parse_number_list(&value) {
                            if v.len() == 3 {
                                tk.axes = Some([v[0] as i32, v[1] as i32, v[2] as i32]);
                            }
                        }
                    }
                    "UNITS" => tk.units = Some(value),
                    "MATRIX" => {
                        if let Some(v) = parse_number_list(&value) {
                            if v.len() == 9 {
                                let mut m = [0.0f64; 9];
                                m.copy_from_slice(&v);
                                tk.matrix = Some(m);
                            }
                        }
                    }
                    _ => {}
                }
                continue;
            }
        }
        for f in &mut frames {
            let tk = tk_by_frame.remove(&f.id).or_else(|| pending.remove(&f.id));
            if let Some(tk) = tk {
                f.tk = Some(tk);
            }
        }
        let by_id: HashMap<i32, usize> =
            frames.iter().enumerate().map(|(i, f)| (f.id, i)).collect();
        FkFile { frames, by_id }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<FkFile, String> {
        std::fs::read_to_string(path.as_ref())
            .map(|t| FkFile::parse(&t))
            .map_err(|e| e.to_string())
    }

    pub fn frame(&self, id: i32) -> Option<&FkFrame> {
        self.by_id.get(&id).and_then(|&i| self.frames.get(i))
    }

    pub fn frame_by_name(&self, name: &str) -> Option<&FkFrame> {
        self.frames.iter().find(|f| f.name == name)
    }

    pub fn tkframe_child_of(&self, relative_name: &str) -> Option<&FkFrame> {
        self.frames.iter().find(|f| {
            f.class == Some(4)
                && f.tk.as_ref().and_then(|t| t.relative.as_deref()) == Some(relative_name)
                && f.tk.as_ref().and_then(|t| t.spec.as_deref()) == Some("ANGLES")
        })
    }

    pub fn insert_file(&mut self, other: FkFile) {
        for f in other.frames {
            match self.by_id.get(&f.id) {
                Some(&i) => {
                    eprintln!(
                        "fk: frame {} ({}) already present as {} — first file carries it",
                        f.id, f.name, self.frames[i].name
                    );
                }
                None => {
                    let i = self.frames.len();
                    self.by_id.insert(f.id, i);
                    self.frames.push(f);
                }
            }
        }
    }

    pub fn tkframe_rotation(&self, frame_id: i32) -> Option<([f64; 9], String)> {
        let frame = self.frame(frame_id)?;
        let tk = frame.tk.as_ref()?;
        let relative_name = tk.relative.as_ref()?;
        let rel = self.frame_by_name(relative_name)?;
        if rel.class != Some(2) {
            eprintln!(
                "fk: frame {} ({}) relative {} is class {:?} — chain not resolved",
                frame.id, frame.name, relative_name, rel.class
            );
            return None;
        }
        if let Some(m) = tk.matrix {
            return Some((m, format!("MATRIX {}", relative_name)));
        }
        let angles = tk.angles?;
        let axes = tk.axes?;
        let units = tk.units.as_deref()?;
        let scale: f64 = if units.eq_ignore_ascii_case("ARCSECONDS") {
            std::f64::consts::PI / (180.0 * 3600.0)
        } else {
            std::f64::consts::PI / 180.0
        };
        let r = |axis: i32, ang: f64| -> [f64; 9] {
            let (s, c) = (ang * scale).sin_cos();
            match axis {
                1 => [1.0, 0.0, 0.0, 0.0, c, s, 0.0, -s, c],
                2 => [c, 0.0, -s, 0.0, 1.0, 0.0, s, 0.0, c],
                _ => [c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0],
            }
        };
        let m1 = r(axes[0], angles[0]);
        let m2 = r(axes[1], angles[1]);
        let m3 = r(axes[2], angles[2]);
        let rot = matmul(&matmul(&m3, &m2), &m1);
        Some((rot, format!("ANGLES via {}", relative_name)))
    }
}

#[cfg(test)]
mod tests {
    use super::FkFile;

    const MOON_TF: &str = "\\begindata\n\
FRAME_MOON_ME                 = 31001\n\
FRAME_31001_NAME              = 'MOON_ME'\n\
FRAME_31001_CLASS             = 4\n\
FRAME_31001_CLASS_ID          = 31001\n\
FRAME_31001_CENTER            = 301\n\
TKFRAME_31001_SPEC            = 'MATRIX'\n\
TKFRAME_31001_RELATIVE        = 'MOON_ME_DE440_ME421'\n\
TKFRAME_31001_MATRIX          = ( 1 0 0\n\
                                  0 1 0\n\
                                  0 0 1 )\n\
\\begindata\n\
FRAME_MOON_PA_DE440           = 31008\n\
FRAME_31008_NAME              = 'MOON_PA_DE440'\n\
FRAME_31008_CLASS             = 2\n\
FRAME_31008_CLASS_ID          = 31008\n\
FRAME_31008_CENTER            = 301\n\
\\begindata\n\
FRAME_MOON_ME_DE440_ME421     = 31009\n\
FRAME_31009_NAME              = 'MOON_ME_DE440_ME421'\n\
FRAME_31009_CLASS             = 4\n\
FRAME_31009_CLASS_ID          = 31009\n\
FRAME_31009_CENTER            = 301\n\
TKFRAME_31009_SPEC            = 'ANGLES'\n\
TKFRAME_31009_RELATIVE        = 'MOON_PA_DE440'\n\
TKFRAME_31009_ANGLES          = (   67.8526   78.6944   0.2785  )\n\
TKFRAME_31009_AXES            = (   3,        2,        1       )\n\
TKFRAME_31009_UNITS           = 'ARCSECONDS'\n";

    #[test]
    fn moon_tf_frames_parsed() {
        let fk = FkFile::parse(MOON_TF);
        assert_eq!(fk.frames.len(), 3);
        let pa = fk.frame(31008).unwrap();
        assert_eq!(pa.name, "MOON_PA_DE440");
        assert_eq!(pa.class, Some(2));
        assert_eq!(pa.center, Some(301));
        let me = fk.frame(31009).unwrap();
        let tk = me.tk.as_ref().unwrap();
        assert_eq!(tk.spec.as_deref(), Some("ANGLES"));
        assert_eq!(tk.relative.as_deref(), Some("MOON_PA_DE440"));
        assert_eq!(tk.axes, Some([3, 2, 1]));
        let a = tk.angles.unwrap();
        assert!((a[0] - 67.8526).abs() < 1e-9);
        assert!((a[1] - 78.6944).abs() < 1e-9);
        assert!((a[2] - 0.2785).abs() < 1e-9);
    }

    #[test]
    fn me_frame_chain_resolves() {
        let fk = FkFile::parse(MOON_TF);
        let (rot, via) = fk.tkframe_rotation(31009).unwrap();
        assert_eq!(via, "ANGLES via MOON_PA_DE440");
        let a1 = 67.8526f64.to_radians() / 3600.0;
        let a2 = 78.6944f64.to_radians() / 3600.0;
        let a3 = 0.2785f64.to_radians() / 3600.0;
        assert!((rot[0] - 1.0).abs() < 1e-4);
        assert!((rot[4] - 1.0).abs() < 1e-4);
        assert!((rot[8] - 1.0).abs() < 1e-4);
        assert!((rot[1] - a1).abs() < 1e-4);
        assert!((rot[2] + a2).abs() < 1e-4);
        assert!((rot[3] + a1).abs() < 1e-4);
        assert!((rot[5] - a3).abs() < 1e-4);
        assert!((rot[6] - a2).abs() < 1e-4);
        assert!((rot[7] + a3).abs() < 1e-4);
    }

    #[test]
    fn matrix_to_class4_chain_unresolved() {
        let fk = FkFile::parse(MOON_TF);
        assert!(fk.tkframe_rotation(31001).is_none());
    }
}
