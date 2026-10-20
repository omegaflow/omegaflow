use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5CompoundMember, Hdf5Datatype, Hdf5File, decode_f32, decode_f64};
use omegaflow::spectral::{parse_spectral_bin, write_spectral_bin};

const DEFAULT_OUT: &str = "leda.bin";
const DEFAULT_FIELD: &str = "ant252_x";
const LEDA_FREQ_START_HZ: f64 = 30_000_000.0;
const LEDA_CHANNEL_WIDTH_HZ: f64 = 24_000.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

struct Table<'a> {
    name: String,
    datatype: &'a Hdf5Datatype,
    rows: usize,
    raw: Vec<u8>,
}

fn member<'a>(dt: &'a Hdf5Datatype, name: &str) -> Option<&'a Hdf5CompoundMember> {
    dt.members.iter().find(|m| m.name == name)
}

fn has_antenna_member(dt: &Hdf5Datatype) -> bool {
    dt.class == 6 && dt.members.iter().any(|m| m.name.starts_with("ant"))
}

fn find_table<'f, 'b>(file: &'f Hdf5File<'b>) -> Result<Table<'f>, String> {
    let mut names: Vec<String> = vec!["Spectral data".to_string(), "data".to_string()];
    for link in file.links_of("") {
        names.push(link.name.clone());
    }
    for name in names {
        let Ok((_obj, ds, dt)) = file.dataset(&name) else {
            continue;
        };
        if !has_antenna_member(dt) {
            continue;
        }
        let rows = match ds.dims.first().copied() {
            Some(n) => n as usize,
            None => continue,
        };
        if rows == 0 {
            continue;
        }
        let raw = match file.read_dataset(&name) {
            Ok(b) => b,
            Err(note) => {
                eprintln!("{name}: dataset read void: {note:?}");
                continue;
            }
        };
        if raw.len() < rows.saturating_mul(dt.size) {
            eprintln!(
                "{name}: {} B carry fewer than {} rows of {} B",
                raw.len(),
                rows,
                dt.size
            );
            continue;
        }
        return Ok(Table {
            name,
            datatype: dt,
            rows,
            raw,
        });
    }
    Err("no compound spectral table (members ant*) — the file stays unread".into())
}

fn read_text(raw: &[u8], off: usize, len: usize) -> String {
    let end = off.saturating_add(len).min(raw.len());
    String::from_utf8_lossy(&raw[off.min(raw.len())..end])
        .trim_end_matches('\0')
        .trim()
        .to_string()
}

fn channel_layout(m: &Hdf5CompoundMember) -> Result<(usize, usize, Endian), String> {
    if m.datatype.class == 10 {
        let base = m
            .datatype
            .base
            .as_ref()
            .ok_or_else(|| format!("{}: array member carries no base type", m.name))?;
        let nchan = m.datatype.array_dims.iter().fold(1u64, |a, d| a * *d) as usize;
        if base.class != 1 || (base.size != 4 && base.size != 8) {
            return Err(format!(
                "{}: array base is class {} size {} — only f32/f64 spectra are read",
                m.name, base.class, base.size
            ));
        }
        Ok((nchan, base.size, base.endian))
    } else if m.datatype.class == 1 && (m.datatype.size == 4 || m.datatype.size == 8) {
        Ok((1, m.datatype.size, m.datatype.endian))
    } else {
        Err(format!(
            "{}: member is class {} size {} — not a f32/f64 spectrum",
            m.name, m.datatype.class, m.datatype.size
        ))
    }
}

fn decode_table(
    dt: &Hdf5Datatype,
    raw: &[u8],
    rows: usize,
    field: &str,
    state: Option<&str>,
    only_row: Option<usize>,
    freq_start: f64,
    df: f64,
    epoch_override: Option<f64>,
) -> Result<(f64, Vec<(f64, f64, f64)>), String> {
    let ant = member(dt, field).ok_or_else(|| format!("member {field} absent"))?;
    let (nchan, elem_size, endian) = channel_layout(ant)?;
    if nchan == 0 {
        return Err(format!("{field}: zero channels"));
    }
    let state_member = member(dt, "state");
    let ts_member = member(dt, "timestamp");

    let mut sums = vec![0.0f64; nchan];
    let mut counts = vec![0usize; nchan];
    let mut epoch = epoch_override;
    let mut selected = 0usize;
    let row_start = match only_row {
        Some(r) => r,
        None => 0usize,
    };
    let row_end = match only_row {
        Some(r) => {
            if r >= rows {
                return Err(format!("row {r} lies outside {rows} rows"));
            }
            r + 1
        }
        None => rows,
    };
    for r in row_start..row_end {
        let base = r * dt.size;
        if base + dt.size > raw.len() {
            return Err(format!("row {r} lies outside {} B", raw.len()));
        }
        if let (Some(m), Some(want)) = (state_member, state) {
            let got = read_text(raw, base + m.offset as usize, m.datatype.size);
            if got != want {
                continue;
            }
        }
        if epoch.is_none() {
            if let Some(m) = ts_member {
                if m.datatype.class == 1 && m.datatype.size == 8 {
                    epoch = decode_f64(raw, base + m.offset as usize, m.datatype.endian);
                }
            }
        }
        let start = base + ant.offset as usize;
        for i in 0..nchan {
            let off = start + i * elem_size;
            let v = if elem_size == 4 {
                decode_f32(raw, off, endian).map(|v| v as f64)
            } else {
                decode_f64(raw, off, endian)
            };
            if let Some(v) = v {
                if v.is_finite() {
                    sums[i] += v;
                    counts[i] += 1;
                }
            }
        }
        selected += 1;
    }
    if selected == 0 {
        return Err("no row survived the state/row filter".into());
    }
    let epoch = epoch.ok_or("timestamp absent and --epoch absent — epoch stays void")?;
    let mut bins = Vec::with_capacity(nchan);
    for (i, (&sum, &count)) in sums.iter().zip(counts.iter()).enumerate() {
        if count == 0 {
            continue;
        }
        let freq = freq_start + i as f64 * df;
        bins.push((freq, df, sum / count as f64));
    }
    if bins.is_empty() {
        return Err("no finite channel survived — the bin stays unwritten".into());
    }
    Ok((epoch, bins))
}

fn run(
    input: &str,
    field: &str,
    state: Option<&str>,
    only_row: Option<usize>,
    out: &str,
    freq_start: f64,
    df: f64,
    epoch_override: Option<f64>,
    ci: bool,
) -> Result<usize, String> {
    let bytes = std::fs::read(input).map_err(|e| format!("read {input}: {e}"))?;
    let file =
        Hdf5File::parse(&bytes).map_err(|note| format!("{input}: HDF5 parse void: {note:?}"))?;
    let table = find_table(&file)?;
    eprintln!(
        "{}: table {} ({} rows, {} B), field {}",
        input,
        table.name,
        table.rows,
        table.raw.len(),
        field
    );
    let (epoch, bins) = decode_table(
        table.datatype,
        &table.raw,
        table.rows,
        field,
        state,
        only_row,
        freq_start,
        df,
        epoch_override,
    )?;
    let out_bytes = write_spectral_bin(epoch, &bins);
    std::fs::write(out, &out_bytes).map_err(|e| format!("write {out}: {e}"))?;
    match parse_spectral_bin(&out_bytes) {
        Some((parsed_epoch, parsed)) => {
            eprintln!(
                "{out}: {} bins, epoch {parsed_epoch} — roundtrip parses ({} B)",
                parsed.len(),
                out_bytes.len()
            );
        }
        None => return Err(format!("{out}: roundtrip parse void")),
    }
    if ci && !upload_release("zenodo.org", out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(bins.len())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci = args.iter().any(|a| a == "--ci-mode");
    let input = match arg_value(&args, "--input") {
        Some(v) => v,
        None => {
            eprintln!("--input absent — the LEDA HDF5 is undeclared");
            std::process::exit(1);
        }
    };
    let field = match arg_value(&args, "--field") {
        Some(v) => v,
        None => DEFAULT_FIELD.to_string(),
    };
    let state = arg_value(&args, "--state");
    let only_row = match arg_value(&args, "--row") {
        Some(v) => match v.parse::<usize>() {
            Ok(r) => Some(r),
            Err(_) => {
                eprintln!("--row {v} is not a row index");
                std::process::exit(1);
            }
        },
        None => None,
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let freq_start = match arg_value(&args, "--freq-start") {
        Some(v) => match v.parse::<f64>() {
            Ok(f) if f.is_finite() => f,
            _ => {
                eprintln!("--freq-start {v} is not finite");
                std::process::exit(1);
            }
        },
        None => LEDA_FREQ_START_HZ,
    };
    let df = match arg_value(&args, "--df") {
        Some(v) => match v.parse::<f64>() {
            Ok(f) if f.is_finite() && f > 0.0 => f,
            _ => {
                eprintln!("--df {v} is not a positive finite width");
                std::process::exit(1);
            }
        },
        None => LEDA_CHANNEL_WIDTH_HZ,
    };
    let epoch_override = match arg_value(&args, "--epoch") {
        Some(v) => match v.parse::<f64>() {
            Ok(e) if e.is_finite() => Some(e),
            _ => {
                eprintln!("--epoch {v} is not a finite unix second");
                std::process::exit(1);
            }
        },
        None => None,
    };
    if let Err(msg) = run(
        &input,
        &field,
        state.as_deref(),
        only_row,
        &out,
        freq_start,
        df,
        epoch_override,
        ci,
    ) {
        eprintln!("{msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &[u8] = include_bytes!("../../testdata/leda_table.h5");

    fn scalar(class: u8, size: usize, endian: Endian) -> Hdf5Datatype {
        Hdf5Datatype {
            class,
            size,
            endian,
            signed: false,
            bit_offset: 0,
            precision: 0,
            string_pad: 0,
            string_charset: 0,
            members: Vec::new(),
            array_dims: Vec::new(),
            base: None,
            reference_type: 0,
            vlen_is_string: false,
        }
    }

    fn array_of(base: Hdf5Datatype, n: u64) -> Hdf5Datatype {
        let mut dt = scalar(10, base.size * n as usize, base.endian);
        dt.array_dims = vec![n];
        dt.base = Some(Box::new(base));
        dt
    }

    fn compound(members: Vec<Hdf5CompoundMember>, size: usize) -> Hdf5Datatype {
        let mut dt = scalar(6, size, Endian::Le);
        dt.members = members;
        dt
    }

    #[test]
    fn leda_fixture_reads_the_compound_table() {
        let file = Hdf5File::parse(FIXTURE).expect("fixture parses");
        let table = find_table(&file).expect("table found");
        assert_eq!(table.rows, 3);
        assert_eq!(table.datatype.members.len(), 9);
        assert!(table.datatype.members.iter().any(|m| m.name == "state"));
        let (epoch, bins) = decode_table(
            table.datatype,
            &table.raw,
            table.rows,
            "ant252_x",
            None,
            None,
            LEDA_FREQ_START_HZ,
            LEDA_CHANNEL_WIDTH_HZ,
            None,
        )
        .expect("fixture decodes");
        assert_eq!(bins.len(), 1, "the fixture carries one scalar channel");
        assert!((bins[0].0 - 30_000_000.0).abs() < 1e-6);
        assert!((bins[0].1 - 24_000.0).abs() < 1e-9);
        assert!((bins[0].2 - 0.6).abs() < 1e-6, "mean of 0.5, 0.6, 0.7");
        assert!((epoch - 1_453_932_642.31).abs() < 1e-3);
    }

    #[test]
    fn leda_fixture_state_filter_uses_the_fixed_length_string() {
        let file = Hdf5File::parse(FIXTURE).expect("fixture parses");
        let table = find_table(&file).expect("table found");
        let (_, bins) = decode_table(
            table.datatype,
            &table.raw,
            table.rows,
            "ant252_x",
            Some("3ss"),
            None,
            LEDA_FREQ_START_HZ,
            LEDA_CHANNEL_WIDTH_HZ,
            None,
        )
        .expect("fixture state filter decodes");
        assert_eq!(bins.len(), 1);
        assert!(
            (bins[0].2 - 0.55).abs() < 1e-6,
            "mean of the two 3ss rows (0.5, 0.6), load row excluded"
        );
    }

    #[test]
    fn leda_array_member_decodes_all_channels() {
        let base = scalar(1, 4, Endian::Le);
        let ant = array_of(base, 4);
        let dt = compound(
            vec![
                Hdf5CompoundMember {
                    name: "ant252_x".into(),
                    offset: 0,
                    datatype: ant,
                },
                Hdf5CompoundMember {
                    name: "timestamp".into(),
                    offset: 16,
                    datatype: scalar(1, 8, Endian::Le),
                },
            ],
            24,
        );
        let mut raw = Vec::new();
        for v in [1.0f32, 2.0, 3.0, 4.0] {
            raw.extend_from_slice(&v.to_le_bytes());
        }
        raw.extend_from_slice(&1_453_932_642.0f64.to_le_bytes());
        let (epoch, bins) = decode_table(
            &dt,
            &raw,
            1,
            "ant252_x",
            None,
            None,
            30_000_000.0,
            24_000.0,
            None,
        )
        .expect("array member decodes");
        assert_eq!(bins.len(), 4);
        for (i, b) in bins.iter().enumerate() {
            assert!((b.0 - (30_000_000.0 + i as f64 * 24_000.0)).abs() < 1e-6);
            assert!((b.2 - (i as f64 + 1.0)).abs() < 1e-9);
        }
        assert!((epoch - 1_453_932_642.0).abs() < 1e-6);
    }

    #[test]
    fn leda_state_filter_keeps_only_the_named_state() {
        let base = scalar(1, 4, Endian::Le);
        let dt = compound(
            vec![
                Hdf5CompoundMember {
                    name: "ant252_x".into(),
                    offset: 0,
                    datatype: base,
                },
                Hdf5CompoundMember {
                    name: "state".into(),
                    offset: 4,
                    datatype: scalar(3, 8, Endian::Le),
                },
                Hdf5CompoundMember {
                    name: "timestamp".into(),
                    offset: 12,
                    datatype: scalar(1, 8, Endian::Le),
                },
            ],
            20,
        );
        let mut raw = Vec::new();
        for (v, state) in [(1.0f32, "3ss"), (2.0, "load"), (3.0, "3ss")] {
            raw.extend_from_slice(&v.to_le_bytes());
            let mut s = [0u8; 8];
            s[..state.len()].copy_from_slice(state.as_bytes());
            raw.extend_from_slice(&s);
            raw.extend_from_slice(&1_453_932_642.0f64.to_le_bytes());
        }
        let (_, bins) = decode_table(
            &dt,
            &raw,
            3,
            "ant252_x",
            Some("3ss"),
            None,
            30_000_000.0,
            24_000.0,
            None,
        )
        .expect("state filter decodes");
        assert_eq!(bins.len(), 1);
        assert!((bins[0].2 - 2.0).abs() < 1e-6, "mean of 1 and 3");
    }

    #[test]
    fn leda_2291_channel_axis_spans_30_to_84_96_mhz() {
        let nchan = 2291usize;
        let first = LEDA_FREQ_START_HZ;
        let last = LEDA_FREQ_START_HZ + (nchan - 1) as f64 * LEDA_CHANNEL_WIDTH_HZ;
        assert!((first - 30_000_000.0).abs() < 1e-6);
        assert!((last - 84_960_000.0).abs() < 1e-6);
    }

    #[test]
    fn leda_non_hdf5_bytes_are_refused() {
        assert!(Hdf5File::parse(b"not hdf5").is_err());
    }
}
