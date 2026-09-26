#[derive(Clone, Debug)]
pub struct TdatColumn {
    pub name: String,
    pub base: String,
    pub ttype: String,
    pub ucd: Option<String>,
    pub indexed: bool,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct TdatTable {
    pub table_name: Option<String>,
    pub description: Option<String>,
    pub columns: Vec<TdatColumn>,
    pub row_order: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
}

impl TdatTable {
    pub fn column(&self, name: &str) -> Option<&TdatColumn> {
        self.columns.iter().find(|c| c.name == name)
    }

    pub fn row_index(&self, name: &str) -> Option<usize> {
        self.row_order.iter().position(|n| n == name)
    }

    pub fn cell(&self, row: usize, name: &str) -> Option<Option<&str>> {
        let ci = self.row_index(name)?;
        self.rows.get(row)?.get(ci).map(|c| c.as_deref())
    }

    pub fn f64_cell(&self, row: usize, name: &str) -> Option<f64> {
        self.cell(row, name)?.and_then(|s| s.parse::<f64>().ok())
    }

    pub fn i64_cell(&self, row: usize, name: &str) -> Option<i64> {
        self.cell(row, name)?.and_then(|s| s.parse::<i64>().ok())
    }
}

fn split_word(s: &str) -> Option<(&str, &str)> {
    let s = s.trim_start();
    let end = s.find(char::is_whitespace).unwrap_or(s.len());
    if end == 0 {
        return None;
    }
    Some((&s[..end], &s[end..]))
}

fn parse_field_line(line: &str) -> Option<TdatColumn> {
    let rest = line.strip_prefix("field[")?;
    let name_end = rest.find(']')?;
    let name = &rest[..name_end];
    let mut after = rest[name_end + 1..].trim_start();
    after = after.strip_prefix('=')?.trim_start();
    let (ttype, rest) = split_word(after)?;
    let mut rest = rest.trim_start();
    let ucd = if let Some(inner) = rest.strip_prefix('[') {
        let close = inner.find(']')?;
        let u = inner[..close].to_string();
        rest = inner[close + 1..].trim_start();
        Some(u)
    } else {
        None
    };
    let indexed = if let Some(inner) = rest.strip_prefix("(index)") {
        rest = inner.trim_start();
        true
    } else {
        false
    };
    let description = rest
        .split_once("//")
        .map(|(_, d)| d.trim().to_string())
        .filter(|d| !d.is_empty());
    let base = match ttype.find(':') {
        Some(i) => ttype[..i].to_string(),
        None => ttype.to_string(),
    };
    Some(TdatColumn {
        name: name.to_string(),
        base,
        ttype: ttype.to_string(),
        ucd,
        indexed,
        description,
    })
}

pub fn parse_tdat(bytes: &[u8]) -> Option<TdatTable> {
    let text = std::str::from_utf8(bytes).ok()?;
    const HEADER_OPEN: &str = "<HEADER>";
    const DATA_OPEN: &str = "<DATA>";
    const DATA_CLOSE: &str = "</DATA>";
    let hs = text.find(HEADER_OPEN)?;
    let he = text.find(DATA_OPEN)?;
    let header = &text[hs + HEADER_OPEN.len()..he];
    let ds = he + DATA_OPEN.len();
    let de = match text[ds..].find(DATA_CLOSE) {
        Some(i) => ds + i,
        None => text.len(),
    };
    let data = &text[ds..de];

    let mut table_name = None;
    let mut description = None;
    let mut columns = Vec::new();
    let mut row_order = Vec::new();
    for raw in header.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("table_name = ") {
            table_name = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("table_description = ") {
            description = Some(rest.trim().trim_matches('"').to_string());
        } else if let Some(rest) = line.strip_prefix("line[1] = ") {
            row_order = rest.split_whitespace().map(str::to_string).collect();
        } else if let Some(col) = parse_field_line(line) {
            columns.push(col);
        }
    }

    let mut rows = Vec::new();
    for raw in data.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts: Vec<&str> = line.split('|').collect();
        if parts.last() == Some(&"") {
            parts.pop();
        }
        let cells: Vec<Option<String>> = parts
            .into_iter()
            .map(|p| {
                if p.is_empty() {
                    None
                } else {
                    Some(p.to_string())
                }
            })
            .collect();
        rows.push(cells);
    }

    Some(TdatTable {
        table_name,
        description,
        columns,
        row_order,
        rows,
    })
}

pub const AMS02_SPECIES: [&str; 23] = [
    "aluminum",
    "antiproton",
    "beryllium",
    "boron",
    "carbon",
    "electron",
    "electron_positron",
    "fluorine",
    "helium",
    "helium3",
    "helium4",
    "hydrogen",
    "iron",
    "lithium",
    "magnesium",
    "neon",
    "nitrogen",
    "oxygen",
    "positron",
    "proton",
    "silicon",
    "sodium",
    "sulfur",
];

pub const AMS02_KIND_ENERGY: u32 = 0;
pub const AMS02_KIND_RIGIDITY: u32 = 1;

pub fn ams02_species_id(name: &str) -> Option<u32> {
    AMS02_SPECIES
        .iter()
        .position(|s| *s == name)
        .map(|i| i as u32)
}

pub fn ams02_comp(species_id: u32, kind: u32) -> u32 {
    species_id * 2 + kind
}

pub const AMS02_ROW_TDB: usize = 0;
pub const AMS02_ROW_LOW: usize = 1;
pub const AMS02_ROW_HIGH: usize = 2;
pub const AMS02_ROW_SPECIES: usize = 3;
pub const AMS02_ROW_KIND: usize = 5;
pub const AMS02_ROW_TDB_END: usize = 7;

pub fn ams02_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = crate::archivar::odf::parse_podf_bin(bytes)?;
    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let t = r[AMS02_ROW_TDB];
        let v = r[AMS02_ROW_LOW];
        if !t.is_finite() || !v.is_finite() {
            continue;
        }
        let species = r[AMS02_ROW_SPECIES];
        let kind = r[AMS02_ROW_KIND];
        if !species.is_finite() || !kind.is_finite() {
            continue;
        }
        out.push((t, v, ams02_comp(species as u32, kind as u32)));
    }
    Some(out)
}

pub const AMS02_COMP_NAMES: [&str; 46] = [
    "ams02_aluminum_energy_min_gev",
    "ams02_aluminum_rigidity_min_gv",
    "ams02_antiproton_energy_min_gev",
    "ams02_antiproton_rigidity_min_gv",
    "ams02_beryllium_energy_min_gev",
    "ams02_beryllium_rigidity_min_gv",
    "ams02_boron_energy_min_gev",
    "ams02_boron_rigidity_min_gv",
    "ams02_carbon_energy_min_gev",
    "ams02_carbon_rigidity_min_gv",
    "ams02_electron_energy_min_gev",
    "ams02_electron_rigidity_min_gv",
    "ams02_electron_positron_energy_min_gev",
    "ams02_electron_positron_rigidity_min_gv",
    "ams02_fluorine_energy_min_gev",
    "ams02_fluorine_rigidity_min_gv",
    "ams02_helium_energy_min_gev",
    "ams02_helium_rigidity_min_gv",
    "ams02_helium3_energy_min_gev",
    "ams02_helium3_rigidity_min_gv",
    "ams02_helium4_energy_min_gev",
    "ams02_helium4_rigidity_min_gv",
    "ams02_hydrogen_energy_min_gev",
    "ams02_hydrogen_rigidity_min_gv",
    "ams02_iron_energy_min_gev",
    "ams02_iron_rigidity_min_gv",
    "ams02_lithium_energy_min_gev",
    "ams02_lithium_rigidity_min_gv",
    "ams02_magnesium_energy_min_gev",
    "ams02_magnesium_rigidity_min_gv",
    "ams02_neon_energy_min_gev",
    "ams02_neon_rigidity_min_gv",
    "ams02_nitrogen_energy_min_gev",
    "ams02_nitrogen_rigidity_min_gv",
    "ams02_oxygen_energy_min_gev",
    "ams02_oxygen_rigidity_min_gv",
    "ams02_positron_energy_min_gev",
    "ams02_positron_rigidity_min_gv",
    "ams02_proton_energy_min_gev",
    "ams02_proton_rigidity_min_gv",
    "ams02_silicon_energy_min_gev",
    "ams02_silicon_rigidity_min_gv",
    "ams02_sodium_energy_min_gev",
    "ams02_sodium_rigidity_min_gv",
    "ams02_sulfur_energy_min_gev",
    "ams02_sulfur_rigidity_min_gv",
];

pub fn ams02_component_name(comp: u32) -> Option<&'static str> {
    AMS02_COMP_NAMES.get(comp as usize).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "<HEADER>\n\
#\n\
table_name = heasarc_ams02spec\n\
table_description = \"AMS-02 Spectral Results Catalog\"\n\
#\n\
field[record_id] = int4  [meta.id;meta.main] (index) // Unique Identifier for Results\n\
field[time] = float8:.6f_mjd [time.start;obs] (index) // Start Date of the Results\n\
field[species] = char30  [phys.atmol.element] (index) // Species\n\
field[energy_min] = float8:.2e_GeV [em.energy] (index) // Minimum Energy per Nucleon\n\
field[data_file] = char60  [meta.ref.url;meta.file] (index) // File Name of FITS Data Product\n\
#\n\
line[1] = record_id time species energy_min data_file\n\
#\n\
<DATA>\n\
1|55701|hydrogen|0.433|ams_hydrogen_energy_180528.fits|\n\
2|55701|helium||ams_helium_energy_180528.fits|\n\
</DATA>\n";

    #[test]
    fn parses_the_header_into_columns_and_metadata() {
        let t = parse_tdat(FIXTURE.as_bytes()).unwrap();
        assert_eq!(t.table_name.as_deref(), Some("heasarc_ams02spec"));
        assert_eq!(
            t.description.as_deref(),
            Some("AMS-02 Spectral Results Catalog")
        );
        assert_eq!(t.columns.len(), 5);
        assert_eq!(t.columns[0].name, "record_id");
        assert_eq!(t.columns[0].base, "int4");
        assert_eq!(t.columns[0].ucd.as_deref(), Some("meta.id;meta.main"));
        assert!(t.columns[0].indexed);
        assert_eq!(
            t.columns[0].description.as_deref(),
            Some("Unique Identifier for Results")
        );
        assert_eq!(t.columns[1].ttype, "float8:.6f_mjd");
        assert_eq!(t.columns[1].base, "float8");
        assert_eq!(t.columns[3].ttype, "float8:.2e_GeV");
        assert_eq!(t.columns[3].base, "float8");
    }

    #[test]
    fn parses_the_row_order_and_data_cells() {
        let t = parse_tdat(FIXTURE.as_bytes()).unwrap();
        assert_eq!(
            t.row_order,
            ["record_id", "time", "species", "energy_min", "data_file"]
        );
        assert_eq!(t.rows.len(), 2);
        let r0 = &t.rows[0];
        assert_eq!(r0.len(), 5);
        assert_eq!(r0[0].as_deref(), Some("1"));
        assert_eq!(r0[4].as_deref(), Some("ams_hydrogen_energy_180528.fits"));
        assert_eq!(t.rows[1][3], None);
    }

    #[test]
    fn typed_cells_absent_and_present() {
        let t = parse_tdat(FIXTURE.as_bytes()).unwrap();
        assert_eq!(t.i64_cell(0, "record_id"), Some(1));
        assert_eq!(t.f64_cell(0, "energy_min"), Some(0.433));
        assert_eq!(t.f64_cell(1, "energy_min"), None);
        assert_eq!(t.f64_cell(0, "species"), None);
        assert_eq!(t.cell(0, "absent_column"), None);
        assert_eq!(t.cell(9, "record_id"), None);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_tdat(b"not a tdat file").is_none());
        assert!(parse_tdat(b"\xff\xfe\xfd").is_none());
    }

    #[test]
    fn ams02_species_table_covers_the_catalog_species() {
        assert_eq!(ams02_species_id("proton"), Some(19));
        assert_eq!(ams02_species_id("electron"), Some(5));
        assert_eq!(ams02_species_id("helium3"), Some(9));
        assert_eq!(ams02_species_id("absent"), None);
        assert_eq!(ams02_comp(19, AMS02_KIND_ENERGY), 38);
        assert_eq!(ams02_comp(19, AMS02_KIND_RIGIDITY), 39);
        assert_eq!(
            ams02_component_name(38),
            Some("ams02_proton_energy_min_gev")
        );
        assert_eq!(
            ams02_component_name(39),
            Some("ams02_proton_rigidity_min_gv")
        );
        assert_eq!(ams02_component_name(46), None);
    }

    #[test]
    fn ams02_series_reads_podf_rows_into_wire_triples() {
        let rows = [
            [1.0e9, 0.433, 1799.0, 19.0, 0.0, 0.0, 0.0, 1.1e9, 0.0],
            [2.0e9, 1.0, 1800.0, 19.0, 0.0, 1.0, 0.0, 2.2e9, 0.0],
        ];
        let bytes = crate::archivar::odf::write_podf_bin(&rows);
        let series = ams02_series(&bytes).unwrap();
        assert_eq!(series.len(), 2);
        assert_eq!(series[0], (1.0e9, 0.433, ams02_comp(19, AMS02_KIND_ENERGY)));
        assert_eq!(series[1], (2.0e9, 1.0, ams02_comp(19, AMS02_KIND_RIGIDITY)));
        assert!(ams02_series(b"X").is_none());
    }

    #[test]
    fn ams02_series_skips_non_finite_or_absent_slots() {
        let rows = [
            [1.0e9, 0.433, 1799.0, 19.0, 0.0, 0.0, 0.0, 1.1e9, 0.0],
            [f64::NAN, 0.5, 1.0, 19.0, 0.0, 0.0, 0.0, 1.1e9, 0.0],
            [3.0e9, 0.5, 1.0, f64::NAN, 0.0, 0.0, 0.0, 1.1e9, 0.0],
        ];
        let bytes = crate::archivar::odf::write_podf_bin(&rows);
        let series = ams02_series(&bytes).unwrap();
        assert_eq!(series.len(), 1);
        assert_eq!(series[0].0, 1.0e9);
    }
}
