use omegaflow::archivar::inflate::inflate;
use omegaflow::archivar::parquet::{ParquetValue, parse_parquet};
use omegaflow::cdn::upload_release;
use omegaflow_harvest::table::{Table, arg_or, arg_value, emit, has_flag, load_source, stat};

const MAGIC: [u8; 4] = *b"ZPQT";
const NETLOC: &str = "zenodo.org";
const FORMAT: &str = "zip_parquet";
const COMPILER: &str = "tools/harvest/src/bin/zip_parquet_compiler.rs";
const DEFAULT_URL: &str = "https://zenodo.org/api/records/4036589/files/year=2019.zip/content";
const DEFAULT_OUT: &str = "fink_ztf_alerts_2019.bin";
const GIB: u64 = 1 << 32;

fn le16(data: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(data.get(off..off + 2)?.try_into().ok()?))
}

fn le32(data: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(off..off + 4)?.try_into().ok()?))
}

fn find_eocd(data: &[u8]) -> Option<usize> {
    if data.len() < 22 {
        return None;
    }
    let tail = data.len() - 22;
    let floor = tail.saturating_sub(65535);
    (floor..=tail)
        .rev()
        .find(|&i| data.get(i..i + 4) == Some(b"PK\x05\x06"))
}

fn central_dir(data: &[u8]) -> Option<(usize, usize)> {
    let eocd = find_eocd(data)?;
    let count = le16(data, eocd + 10)? as usize;
    let cd_size = le32(data, eocd + 12)? as usize;
    let start = eocd.checked_sub(cd_size)?;
    if data.get(start..start + 4)? != b"PK\x01\x02" {
        return None;
    }
    Some((start, count))
}

struct Member {
    name: String,
    method: u16,
    comp_size: u64,
    local_off: u64,
}

fn members(data: &[u8]) -> Option<Vec<Member>> {
    let (mut off, count) = central_dir(data)?;
    let mut out = Vec::with_capacity(count.min(1 << 16));
    for _ in 0..count {
        if data.get(off..off + 4)? != b"PK\x01\x02" {
            return None;
        }
        let method = le16(data, off + 10)?;
        let comp_size = le32(data, off + 20)? as u64;
        let name_len = le16(data, off + 28)? as usize;
        let extra_len = le16(data, off + 30)? as usize;
        let comment_len = le16(data, off + 32)? as usize;
        let local_off = le32(data, off + 42)? as u64;
        let name = std::str::from_utf8(data.get(off + 46..off + 46 + name_len)?)
            .ok()?
            .to_string();
        out.push(Member {
            name,
            method,
            comp_size,
            local_off,
        });
        off = off.checked_add(46 + name_len + extra_len + comment_len)?;
    }
    Some(out)
}

fn local_payload<'a>(data: &'a [u8], m: &Member) -> Option<&'a [u8]> {
    let mut base = m.local_off;
    for _ in 0..4 {
        let off = usize::try_from(base).ok()?;
        if data.get(off..off + 4) == Some(b"PK\x03\x04") {
            let name_len = le16(data, off + 26)? as usize;
            let extra_len = le16(data, off + 28)? as usize;
            let mut comp = m.comp_size;
            if comp == 0 || comp == 0xFFFF_FFFF {
                comp = le32(data, off + 18)? as u64;
            }
            let start = off.checked_add(30 + name_len + extra_len)?;
            let end = start.checked_add(usize::try_from(comp).ok()?)?;
            return data.get(start..end);
        }
        base = base.checked_add(GIB)?;
    }
    None
}

fn member_bytes(data: &[u8], m: &Member) -> Option<Vec<u8>> {
    let payload = local_payload(data, m)?;
    match m.method {
        0 => Some(payload.to_vec()),
        8 => inflate(payload),
        _ => None,
    }
}

fn value_num(value: &ParquetValue) -> Option<f64> {
    match value {
        ParquetValue::I32(x) => Some(*x as f64),
        ParquetValue::I64(x) => Some(*x as f64),
        ParquetValue::Float(x) => {
            let y = *x as f64;
            y.is_finite().then_some(y)
        }
        ParquetValue::Double(x) => x.is_finite().then_some(*x),
        ParquetValue::Bytes(b) => std::str::from_utf8(b)
            .ok()
            .and_then(omegaflow_harvest::table::cell_num),
        ParquetValue::Bool(_) | ParquetValue::Int96(_) | ParquetValue::Unhandled(_) => None,
    }
}

fn parse_table(bytes: &[u8]) -> Option<Table> {
    let columns = parse_parquet(bytes)?;
    if columns.is_empty() {
        return None;
    }
    let n_rows = columns.iter().map(|c| c.values.len()).max()?;
    if n_rows == 0 {
        return None;
    }
    let names = columns.iter().map(|c| c.name.clone()).collect::<Vec<_>>();
    let mut rows = vec![vec![None; columns.len()]; n_rows];
    for (j, column) in columns.iter().enumerate() {
        for (i, value) in column.values.iter().enumerate() {
            if let Some(slot) = rows.get_mut(i).and_then(|r| r.get_mut(j)) {
                *slot = value_num(value);
            }
        }
    }
    Some(Table { names, rows })
}

fn select<'a>(ms: &'a [Member], want: Option<&str>) -> Option<&'a Member> {
    match want {
        Some(w) => ms
            .iter()
            .find(|m| m.name.contains(w) && m.name.ends_with(".parquet"))
            .or_else(|| ms.iter().find(|m| m.name.contains(w))),
        None => ms.iter().find(|m| m.name.ends_with(".parquet")),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((src, bytes)) = load_source(&args, DEFAULT_URL) else {
        std::process::exit(1);
    };
    let Some(ms) = members(&bytes) else {
        eprintln!("{src}: no zip central directory read — absent (0 honored)");
        std::process::exit(1);
    };
    let want = arg_value(&args, "--member");
    let Some(m) = select(&ms, want.as_deref()) else {
        eprintln!(
            "{src}: no parquet member selected from {} member(s) — absent (0 honored)",
            ms.len()
        );
        std::process::exit(1);
    };
    let Some(member) = member_bytes(&bytes, m) else {
        eprintln!(
            "{src}:{}: the member stays unread — absent (0 honored)",
            m.name
        );
        std::process::exit(1);
    };
    let Some(table) = parse_table(&member) else {
        eprintln!(
            "{src}:{}: {} B carry no parquet contract — absent (0 honored)",
            m.name,
            member.len()
        );
        std::process::exit(1);
    };
    eprintln!(
        "{src}: member {} ({} B) read as parquet",
        m.name,
        member.len()
    );
    if has_flag(&args, "--stat") {
        stat(&src, &table);
        return;
    }
    let out = arg_or(&args, "--out", DEFAULT_OUT);
    let netloc = arg_or(&args, "--netloc", NETLOC);
    if !emit(&src, &out, &table, MAGIC, &netloc, FORMAT, COMPILER) {
        std::process::exit(1);
    }
    if has_flag(&args, "--ci-mode") && !upload_release(&netloc, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_zip(name: &str, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(data);
        let cd_start = out.len();
        out.extend_from_slice(b"PK\x01\x02");
        out.extend_from_slice(&0x0315u16.to_le_bytes());
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(name.as_bytes());
        let cd_size = out.len() - cd_start;
        out.extend_from_slice(b"PK\x05\x06");
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&(cd_size as u32).to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out
    }

    #[test]
    fn central_dir_reads_despite_a_written_offset_field() {
        let zip = make_zip("year=2019/month=11/day=1/part.parquet", b"payload");
        let (start, count) = central_dir(&zip).expect("central dir");
        assert_eq!(count, 1);
        assert_eq!(zip.get(start..start + 4), Some(b"PK\x01\x02".as_slice()));
    }

    #[test]
    fn a_member_reads_by_name_and_bytes() {
        let zip = make_zip("year=2019/month=11/day=1/part.parquet", b"payload");
        let ms = members(&zip).expect("members");
        assert_eq!(ms.len(), 1);
        assert_eq!(ms[0].name, "year=2019/month=11/day=1/part.parquet");
        assert_eq!(member_bytes(&zip, &ms[0]), Some(b"payload".to_vec()));
    }

    #[test]
    fn selection_prefers_a_parquet_member() {
        let zip = make_zip("a/readme.txt", b"x");
        let ms = members(&zip).expect("members");
        assert!(select(&ms, None).is_none());
        assert!(select(&ms, Some("readme")).is_some());
    }

    #[test]
    fn value_num_refuses_non_finite_and_bool() {
        assert_eq!(value_num(&ParquetValue::I32(-3)), Some(-3.0));
        assert_eq!(value_num(&ParquetValue::Double(2.5)), Some(2.5));
        assert_eq!(value_num(&ParquetValue::Float(f32::NAN)), None);
        assert_eq!(value_num(&ParquetValue::Double(f64::INFINITY)), None);
        assert_eq!(value_num(&ParquetValue::Bool(true)), None);
    }
}
