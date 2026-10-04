use crate::archivar::json::{jstr, parse_json};

#[derive(Clone, Debug, PartialEq)]
pub struct Electrode {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub size: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoordSystem {
    pub system: String,
    pub units: String,
}

fn cell_number(cells: &[&str], index: usize) -> Option<f64> {
    let raw = cells.get(index)?.trim();
    if raw.is_empty() || raw.eq_ignore_ascii_case("n/a") || raw.eq_ignore_ascii_case("nan") {
        return None;
    }
    raw.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn parse_electrodes_tsv(bytes: &[u8]) -> Vec<Electrode> {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return Vec::new();
    };
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let cols: Vec<&str> = header.split('\t').map(str::trim).collect();
    let find = |name: &str| cols.iter().position(|c| c.eq_ignore_ascii_case(name));
    let (Some(name_i), Some(x_i), Some(y_i), Some(z_i)) =
        (find("name"), find("x"), find("y"), find("z"))
    else {
        return Vec::new();
    };
    let size_i = find("size");
    let mut out = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split('\t').map(str::trim).collect();
        let Some(name) = cells.get(name_i).filter(|n| !n.is_empty()) else {
            continue;
        };
        let (Some(x), Some(y), Some(z)) = (
            cell_number(&cells, x_i),
            cell_number(&cells, y_i),
            cell_number(&cells, z_i),
        ) else {
            continue;
        };
        if x == 0.0 && y == 0.0 && z == 0.0 {
            continue;
        }
        out.push(Electrode {
            name: (*name).to_string(),
            x,
            y,
            z,
            size: size_i.and_then(|i| cell_number(&cells, i)),
        });
    }
    out
}

pub fn parse_coordsystem_json(bytes: &[u8]) -> Option<CoordSystem> {
    let text = std::str::from_utf8(bytes).ok()?;
    let json = parse_json(text)?;
    let system = jstr(&json, "iEEGCoordinateSystem")?;
    if system.is_empty() {
        return None;
    }
    let units = jstr(&json, "iEEGCoordinateUnits")?;
    if units.is_empty() {
        return None;
    }
    Some(CoordSystem { system, units })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TSV: &str = "name\tx\ty\tz\tsize\nLFF1\t-35.0\t0.0\t-5.5\t1.2\nLFF2\tn/a\tn/a\tn/a\tn/a\nEMPTY\t0.0\t0.0\t0.0\t3.0\nLFF3\t12.5\t-3.25\t40.0\n";

    #[test]
    fn reads_electrodes_and_honors_a_single_axis_zero() {
        let electrodes = parse_electrodes_tsv(TSV.as_bytes());
        assert_eq!(electrodes.len(), 2);
        assert_eq!(electrodes[0].name, "LFF1");
        assert_eq!(electrodes[0].x, -35.0);
        assert_eq!(electrodes[0].y, 0.0);
        assert_eq!(electrodes[0].z, -5.5);
        assert_eq!(electrodes[0].size, Some(1.2));
        assert_eq!(electrodes[1].name, "LFF3");
        assert_eq!(electrodes[1].y, -3.25);
        assert_eq!(electrodes[1].size, None);
    }

    #[test]
    fn absent_coordinates_skip_the_row_and_the_origin_stays_out() {
        let electrodes = parse_electrodes_tsv(TSV.as_bytes());
        assert!(!electrodes.iter().any(|e| e.name == "LFF2"));
        assert!(!electrodes.iter().any(|e| e.name == "EMPTY"));
    }

    #[test]
    fn a_header_without_coordinate_columns_is_empty() {
        assert!(parse_electrodes_tsv(b"name\tsize\nA\t1.0\n").is_empty());
        assert!(parse_electrodes_tsv(b"").is_empty());
    }

    #[test]
    fn coordsystem_reads_system_and_units() {
        let json = br#"{"iEEGCoordinateSystem":"MNI305","iEEGCoordinateUnits":"mm"}"#;
        let cs = parse_coordsystem_json(json).expect("coordsystem parses");
        assert_eq!(cs.system, "MNI305");
        assert_eq!(cs.units, "mm");
    }

    #[test]
    fn coordsystem_without_the_keys_stays_absent() {
        assert!(parse_coordsystem_json(br#"{"iEEGCoordinateUnits":"mm"}"#).is_none());
        assert!(parse_coordsystem_json(br#"{"iEEGCoordinateSystem":""}"#).is_none());
        assert!(parse_coordsystem_json(b"not json").is_none());
    }
}
