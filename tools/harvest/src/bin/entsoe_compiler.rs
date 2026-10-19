use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::env;
use std::path::PathBuf;
use std::process::Command;

const API: &str = "https://web-api.tp.entsoe.eu/api";
const TAG: &str = "transparency.entsoe.eu";
const KEY_NAME: &str = "ENTSOE_API_TOKEN";
const MAGIC: &[u8; 4] = b"ESOE";
const DOCUMENT_TYPE: &str = "A78";
const BUSINESS_PLANNED: u8 = 1;
const BUSINESS_FORCED: u8 = 2;
const BUSINESS_OTHER: u8 = 3;
const NO_SECONDS: i64 = 0;

#[derive(Clone, Debug, PartialEq)]
struct EsRecord {
    start_unix: f64,
    end_unix: f64,
    business_type_code: u8,
    quantity: Option<f64>,
    domain: String,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == name {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

fn find_repo_root() -> Option<PathBuf> {
    if let Ok(env_root) = env::var("OMEGAFLOW_REPO") {
        if !env_root.is_empty() {
            let root = PathBuf::from(env_root);
            if root.join("phi").join("sources.\u{3c6}").is_file() {
                return Some(root);
            }
            return None;
        }
    }
    let mut starts: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = env::current_dir() {
        starts.push(cwd);
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            starts.push(dir.to_path_buf());
        }
    }
    for start in starts {
        let mut dir = start;
        loop {
            if dir.join("phi").join("sources.\u{3c6}").is_file() {
                return Some(dir);
            }
            match dir.parent() {
                Some(parent) => dir = parent.to_path_buf(),
                None => break,
            }
        }
    }
    None
}

fn read_secret(name: &str) -> Option<String> {
    let root = find_repo_root()?;
    let text = std::fs::read_to_string(root.join(".secrets.local")).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(eq) = line.find('=') {
            if line[..eq].trim() == name {
                let value = line[eq + 1..].trim().trim_matches(['"', '\'']);
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .args([
            "-sSL",
            "--retry",
            "3",
            "--max-time",
            "120",
            "--connect-timeout",
            "15",
            "--write-out",
            "\n%{http_code}",
            url,
        ])
        .output()
        .map_err(|e| format!("curl invocation void: {e}"))?;
    if !out.status.success() {
        return Err(format!("curl status {:?}", out.status.code()));
    }
    let raw = out.stdout;
    let Some(split) = raw.iter().rposition(|b| *b == b'\n') else {
        return Err("response carries no status line".to_string());
    };
    let code = String::from_utf8_lossy(&raw[split + 1..])
        .trim()
        .to_string();
    let body = &raw[..split];
    if !code.starts_with('2') {
        return Err(format!(
            "HTTP {code} ({} B): {}",
            body.len(),
            error_head(body, 1500)
        ));
    }
    if body.is_empty() {
        return Err("response carries no bytes".to_string());
    }
    Ok(body.to_vec())
}

const LENGTH_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LENGTH_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    bit_buf: u32,
    bit_cnt: u32,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        BitReader {
            data,
            pos: 0,
            bit_buf: 0,
            bit_cnt: 0,
        }
    }

    fn read_bit(&mut self) -> Option<u32> {
        if self.bit_cnt == 0 {
            if self.pos >= self.data.len() {
                return None;
            }
            self.bit_buf = self.data[self.pos] as u32;
            self.pos += 1;
            self.bit_cnt = 8;
        }
        let bit = self.bit_buf & 1;
        self.bit_buf >>= 1;
        self.bit_cnt -= 1;
        Some(bit)
    }

    fn read_bits(&mut self, n: u32) -> Option<u32> {
        let mut value = 0u32;
        for i in 0..n {
            value |= self.read_bit()? << i;
        }
        Some(value)
    }

    fn read_byte(&mut self) -> Option<u8> {
        self.read_bits(8).map(|v| v as u8)
    }

    fn read_u16_le(&mut self) -> Option<u16> {
        let lo = self.read_byte()? as u16;
        let hi = self.read_byte()? as u16;
        Some(lo | (hi << 8))
    }

    fn align_byte(&mut self) {
        self.bit_buf = 0;
        self.bit_cnt = 0;
    }
}

struct Huffman {
    counts: [u16; 16],
    symbols: Vec<u16>,
}

impl Huffman {
    fn from_lengths(lengths: &[u8]) -> Option<Huffman> {
        let mut counts = [0u16; 16];
        for &len in lengths {
            if len as usize > 15 {
                return None;
            }
            counts[len as usize] += 1;
        }
        counts[0] = 0;
        let mut left = 1i32;
        for len in 1..16 {
            left <<= 1;
            left -= counts[len] as i32;
            if left < 0 {
                return None;
            }
        }
        let mut offsets = [0usize; 16];
        let mut total = 0usize;
        for len in 1..16 {
            offsets[len] = total;
            total += counts[len] as usize;
        }
        let mut symbols = vec![0u16; total];
        for (symbol, &len) in lengths.iter().enumerate() {
            if len != 0 {
                symbols[offsets[len as usize]] = symbol as u16;
                offsets[len as usize] += 1;
            }
        }
        Some(Huffman { counts, symbols })
    }

    fn decode(&self, reader: &mut BitReader<'_>) -> Option<u16> {
        let mut code = 0i32;
        let mut first = 0i32;
        let mut index = 0i32;
        for len in 1..16 {
            code |= reader.read_bit()? as i32;
            let count = self.counts[len] as i32;
            if code - first < count {
                return Some(self.symbols[(index + (code - first)) as usize]);
            }
            index += count;
            first += count;
            first <<= 1;
            code <<= 1;
        }
        None
    }
}

fn fixed_tables() -> Option<(Huffman, Huffman)> {
    let mut lit_lengths = [0u8; 288];
    for (symbol, len) in lit_lengths.iter_mut().enumerate() {
        *len = match symbol {
            0..=143 => 8,
            144..=255 => 9,
            256..=279 => 7,
            _ => 8,
        };
    }
    let dist_lengths = [5u8; 32];
    let lit = Huffman::from_lengths(&lit_lengths)?;
    let dist = Huffman::from_lengths(&dist_lengths)?;
    Some((lit, dist))
}

fn dynamic_tables(reader: &mut BitReader<'_>) -> Option<(Huffman, Huffman)> {
    const ORDER: [usize; 19] = [
        16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
    ];
    let hlit = reader.read_bits(5)? as usize + 257;
    let hdist = reader.read_bits(5)? as usize + 1;
    let hclen = reader.read_bits(4)? as usize + 4;
    let mut cl_lengths = [0u8; 19];
    for &slot in ORDER.iter().take(hclen) {
        cl_lengths[slot] = reader.read_bits(3)? as u8;
    }
    let cl = Huffman::from_lengths(&cl_lengths)?;
    let total = hlit + hdist;
    let mut lengths: Vec<u8> = Vec::with_capacity(total);
    while lengths.len() < total {
        match cl.decode(reader)? {
            symbol @ 0..=15 => lengths.push(symbol as u8),
            16 => {
                let prev = *lengths.last()?;
                let repeat = reader.read_bits(2)? as usize + 3;
                for _ in 0..repeat {
                    lengths.push(prev);
                }
            }
            17 => {
                let repeat = reader.read_bits(3)? as usize + 3;
                for _ in 0..repeat {
                    lengths.push(0);
                }
            }
            18 => {
                let repeat = reader.read_bits(7)? as usize + 11;
                for _ in 0..repeat {
                    lengths.push(0);
                }
            }
            _ => return None,
        }
    }
    if lengths.len() != total {
        return None;
    }
    let lit = Huffman::from_lengths(&lengths[..hlit])?;
    let dist = Huffman::from_lengths(&lengths[hlit..])?;
    Some((lit, dist))
}

fn inflate_stored(reader: &mut BitReader<'_>, out: &mut Vec<u8>) -> Option<()> {
    reader.align_byte();
    let len = reader.read_u16_le()? as usize;
    let nlen = reader.read_u16_le()?;
    if len as u16 != !nlen {
        return None;
    }
    for _ in 0..len {
        out.push(reader.read_byte()?);
    }
    Some(())
}

fn inflate_block(
    reader: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    lit: &Huffman,
    dist: &Huffman,
) -> Option<()> {
    loop {
        let symbol = lit.decode(reader)?;
        if symbol < 256 {
            out.push(symbol as u8);
        } else if symbol == 256 {
            return Some(());
        } else {
            let length_index = symbol as usize - 257;
            if length_index >= LENGTH_BASE.len() {
                return None;
            }
            let length = LENGTH_BASE[length_index] as usize
                + reader.read_bits(LENGTH_EXTRA[length_index] as u32)? as usize;
            let dist_symbol = dist.decode(reader)? as usize;
            if dist_symbol >= DIST_BASE.len() {
                return None;
            }
            let distance = DIST_BASE[dist_symbol] as usize
                + reader.read_bits(DIST_EXTRA[dist_symbol] as u32)? as usize;
            if distance == 0 || distance > out.len() {
                return None;
            }
            let start = out.len() - distance;
            for offset in 0..length {
                let byte = out[start + offset];
                out.push(byte);
            }
        }
    }
}

fn inflate(data: &[u8]) -> Option<Vec<u8>> {
    let mut reader = BitReader::new(data);
    let mut out = Vec::new();
    loop {
        let final_block = reader.read_bit()?;
        let block_type = reader.read_bits(2)?;
        match block_type {
            0 => inflate_stored(&mut reader, &mut out)?,
            1 => {
                let (lit, dist) = fixed_tables()?;
                inflate_block(&mut reader, &mut out, &lit, &dist)?;
            }
            2 => {
                let (lit, dist) = dynamic_tables(&mut reader)?;
                inflate_block(&mut reader, &mut out, &lit, &dist)?;
            }
            _ => return None,
        }
        if final_block == 1 {
            break;
        }
    }
    Some(out)
}

fn next_local_header(bytes: &[u8], from: usize) -> Option<usize> {
    let mut i = from;
    while i + 4 <= bytes.len() {
        if &bytes[i..i + 4] == b"PK\x03\x04" {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn zip_entries(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos + 30 <= bytes.len() && &bytes[pos..pos + 4] == b"PK\x03\x04" {
        let method = u16::from_le_bytes([bytes[pos + 8], bytes[pos + 9]]);
        let compressed_size = u32::from_le_bytes([
            bytes[pos + 18],
            bytes[pos + 19],
            bytes[pos + 20],
            bytes[pos + 21],
        ]) as usize;
        let name_len = u16::from_le_bytes([bytes[pos + 26], bytes[pos + 27]]) as usize;
        let extra_len = u16::from_le_bytes([bytes[pos + 28], bytes[pos + 29]]) as usize;
        let data_start = pos + 30 + name_len + extra_len;
        if data_start > bytes.len() {
            break;
        }
        let body = &bytes[data_start..];
        let entry = match method {
            0 => {
                if compressed_size == 0 || compressed_size > body.len() {
                    break;
                }
                body[..compressed_size].to_vec()
            }
            8 => {
                let slice = if compressed_size == 0 {
                    body
                } else if compressed_size <= body.len() {
                    &body[..compressed_size]
                } else {
                    break;
                };
                match inflate(slice) {
                    Some(entry) => entry,
                    None => break,
                }
            }
            _ => break,
        };
        out.push(entry);
        if compressed_size > 0 {
            pos = data_start + compressed_size;
        } else if let Some(next) = next_local_header(bytes, data_start + 1) {
            pos = next;
        } else {
            break;
        }
    }
    out
}

fn xml_from_response(bytes: &[u8]) -> Option<String> {
    if bytes.starts_with(b"PK\x03\x04") {
        let entries = zip_entries(bytes);
        if entries.is_empty() {
            return None;
        }
        let mut text = String::new();
        for (i, entry) in entries.iter().enumerate() {
            if i > 0 {
                text.push('\n');
            }
            text.push_str(&String::from_utf8_lossy(entry));
        }
        return Some(text);
    }
    Some(String::from_utf8_lossy(bytes).into_owned())
}

fn error_head(body: &[u8], cap: usize) -> String {
    let mut rest = String::from_utf8_lossy(body).into_owned();
    for (open, close) in [("<style", "</style>"), ("<script", "</script>")] {
        while let Some(start) = rest.find(open) {
            match rest[start..].find(close) {
                Some(rel) => {
                    let end = start + rel + close.len();
                    rest.replace_range(start..end, " ");
                }
                None => {
                    rest.truncate(start);
                    break;
                }
            }
        }
    }
    let mut visible = String::new();
    let mut in_tag = false;
    for ch in rest.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                visible.push(' ');
            }
            _ if !in_tag => visible.push(ch),
            _ => {}
        }
    }
    let collapsed = visible.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut capped = String::new();
    for ch in collapsed.chars() {
        if capped.len() + ch.len_utf8() > cap {
            break;
        }
        capped.push(ch);
    }
    capped.trim().to_string()
}

fn tag_text(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}");
    let mut search = 0usize;
    while let Some(rel) = xml[search..].find(&open) {
        let at = search + rel;
        let after = &xml[at + open.len()..];
        match after.chars().next() {
            Some('>') | Some(' ') | Some('\t') | Some('\n') | Some('\r') => {}
            _ => {
                search = at + 1;
                continue;
            }
        }
        let gt = xml[at..].find('>')?;
        let text_start = at + gt + 1;
        let lt = xml[text_start..].find('<')?;
        let text = xml[text_start..text_start + lt].trim();
        if text.is_empty() {
            return None;
        }
        return Some(text.to_string());
    }
    None
}

fn time_series_blocks(xml: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut search = 0usize;
    while let Some(rel) = xml[search..].find("<TimeSeries") {
        let start = search + rel;
        let after = &xml[start + "<TimeSeries".len()..];
        match after.chars().next() {
            Some('>') | Some(' ') | Some('\t') | Some('\n') | Some('\r') | Some('/') => {}
            _ => {
                search = start + 1;
                continue;
            }
        }
        let Some(rel_end) = xml[start..].find("</TimeSeries>") else {
            break;
        };
        let end = start + rel_end + "</TimeSeries>".len();
        out.push(&xml[start..end]);
        search = end;
    }
    out
}

fn parse_date(date: &str) -> Option<(i64, i64, i64)> {
    let mut parts = date.trim().split('-');
    let year = parts.next()?.parse::<i64>().ok()?;
    let month = parts.next()?.parse::<i64>().ok()?;
    let day = parts.next()?.parse::<i64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some((year, month, day))
}

fn parse_time(time: &str) -> Option<(i64, i64, i64)> {
    let t = time.trim().trim_end_matches('Z');
    let mut parts = t.split(':');
    let hour = parts.next()?.parse::<i64>().ok()?;
    let minute = parts.next()?.parse::<i64>().ok()?;
    let second = match parts.next() {
        Some(s) => s.parse::<i64>().ok()?,
        None => NO_SECONDS,
    };
    if !(0..=23).contains(&hour) || !(0..=59).contains(&minute) || !(0..=59).contains(&second) {
        return None;
    }
    Some((hour, minute, second))
}

fn parse_datetime(date: &str, time: &str) -> Option<f64> {
    let (year, month, day) = parse_date(date)?;
    let (hour, minute, second) = parse_time(time)?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second as f64)
}

fn parse_series(block: &str) -> Option<EsRecord> {
    let business_type_code = match tag_text(block, "businessType").as_deref() {
        Some("A53") => BUSINESS_PLANNED,
        Some("A54") => BUSINESS_FORCED,
        _ => BUSINESS_OTHER,
    };
    let domain = tag_text(block, "in_Domain.mRID")
        .or_else(|| tag_text(block, "out_Domain.mRID"))
        .or_else(|| tag_text(block, "biddingZone_Domain.mRID"))?;
    let start_date = tag_text(block, "start_DateAndOrTime.date")?;
    let start_time = tag_text(block, "start_DateAndOrTime.time")?;
    let end_date = tag_text(block, "end_DateAndOrTime.date")?;
    let end_time = tag_text(block, "end_DateAndOrTime.time")?;
    let start_unix = parse_datetime(&start_date, &start_time)?;
    let end_unix = parse_datetime(&end_date, &end_time)?;
    let quantity = tag_text(block, "quantity")
        .and_then(|t| t.parse::<f64>().ok())
        .filter(|v| v.is_finite());
    Some(EsRecord {
        start_unix,
        end_unix,
        business_type_code,
        quantity,
        domain,
    })
}

fn parse_document(xml: &str) -> Vec<EsRecord> {
    let mut out = Vec::new();
    for block in time_series_blocks(xml) {
        if let Some(record) = parse_series(block) {
            out.push(record);
        }
    }
    out
}

fn compact_period(date: &str) -> Option<String> {
    let (year, month, day) = parse_date(date)?;
    Some(format!("{year:04}{month:02}{day:02}0000"))
}

fn write_bin(records: &[EsRecord]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for record in records {
        out.extend_from_slice(&record.start_unix.to_le_bytes());
        out.extend_from_slice(&record.end_unix.to_le_bytes());
        out.push(record.business_type_code);
        match record.quantity {
            Some(q) => {
                out.push(1);
                out.extend_from_slice(&q.to_le_bytes());
            }
            None => out.push(0),
        }
        let bytes = record.domain.as_bytes();
        let len = bytes.len().min(u16::MAX as usize) as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&bytes[..len as usize]);
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<EsRecord>> {
    if bytes.len() < 8 || &bytes[0..4] != &MAGIC[..] {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = 8usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        if off + 20 > bytes.len() {
            return None;
        }
        let start_unix = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
        off += 8;
        let end_unix = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
        off += 8;
        let business_type_code = bytes[off];
        off += 1;
        let quantity_flag = bytes[off];
        off += 1;
        let quantity = if quantity_flag == 0 {
            None
        } else {
            if off + 8 > bytes.len() {
                return None;
            }
            let q = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
            off += 8;
            Some(q)
        };
        let domain_len = u16::from_le_bytes(bytes[off..off + 2].try_into().ok()?) as usize;
        off += 2;
        if off + domain_len > bytes.len() {
            return None;
        }
        let domain = String::from_utf8(bytes[off..off + domain_len].to_vec()).ok()?;
        off += domain_len;
        out.push(EsRecord {
            start_unix,
            end_unix,
            business_type_code,
            quantity,
            domain,
        });
    }
    Some(out)
}

fn build_url(
    token: &str,
    in_domain: &str,
    out_domain: &str,
    period_start: &str,
    period_end: &str,
) -> String {
    format!(
        "{API}?securityToken={token}&documentType={DOCUMENT_TYPE}&in_Domain={in_domain}&out_Domain={out_domain}&periodStart={period_start}&periodEnd={period_end}"
    )
}

fn usage() -> String {
    "entsoe_compiler --domain <EIC> [--domain <EIC>...] [--out-domain <EIC>...] --start YYYY-MM-DD --stop YYYY-MM-DD [--out-bin name] [--ci-mode]".to_string()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let domains = arg_values(&args, "--domain");
    if domains.is_empty() {
        eprintln!("entsoe_compiler: --domain <EIC> required (in_Domain is never fabricated)");
        eprintln!("usage: {}", usage());
        std::process::exit(2);
    }
    let out_domains = arg_values(&args, "--out-domain");
    if out_domains.is_empty() {
        eprintln!(
            "entsoe_compiler: --out-domain absent — A78 carries out_Domain mirrored from in_Domain \
             (a single-zone transmission unavailability has in_Domain == out_Domain)"
        );
    }

    let Some(start_arg) = arg_value(&args, "--start") else {
        eprintln!("entsoe_compiler: --start <YYYY-MM-DD> required");
        eprintln!("usage: {}", usage());
        std::process::exit(2);
    };
    let Some(stop_arg) = arg_value(&args, "--stop") else {
        eprintln!("entsoe_compiler: --stop <YYYY-MM-DD> required");
        eprintln!("usage: {}", usage());
        std::process::exit(2);
    };
    let Some(period_start) = compact_period(&start_arg) else {
        eprintln!("entsoe_compiler: --start {start_arg} is no YYYY-MM-DD date");
        std::process::exit(2);
    };
    let Some(period_end) = compact_period(&stop_arg) else {
        eprintln!("entsoe_compiler: --stop {stop_arg} is no YYYY-MM-DD date");
        std::process::exit(2);
    };

    let out_bin = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => "entsoe_a78.bin".to_string(),
    };

    let Some(token) = read_secret(KEY_NAME) else {
        eprintln!("{KEY_NAME} absent");
        std::process::exit(1);
    };

    let mut records: Vec<EsRecord> = Vec::new();
    for (i, domain) in domains.iter().enumerate() {
        let out_domain = out_domains.get(i).map(String::as_str).unwrap_or(domain);
        let url = build_url(&token, domain, out_domain, &period_start, &period_end);
        match fetch(&url) {
            Ok(bytes) => {
                let Some(text) = xml_from_response(&bytes) else {
                    eprintln!("{domain}: archive unreadable — the domain stays unharvested");
                    continue;
                };
                let series = parse_document(&text);
                eprintln!(
                    "{domain} -> {out_domain}: {} TimeSeries parsed",
                    series.len()
                );
                records.extend(series);
            }
            Err(msg) => {
                eprintln!("{domain}: fetch void — {msg}, the domain stays unharvested");
            }
        }
    }

    if records.is_empty() {
        eprintln!("entsoe_a78: no TimeSeries parsed — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.start_unix.total_cmp(&b.start_unix));

    let bytes = write_bin(&records);
    if std::fs::write(&out_bin, &bytes).is_err() {
        eprintln!("write {out_bin} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!(
            "{out_bin}: {} unavailability records, {} B, roundtrip parses",
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{out_bin}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(TAG, &out_bin) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "<Unavailability_MarketDocument>\
<unavailability_Time_Period.timeInterval><start>2024-01-01T00:00Z</start><end>2024-02-01T00:00Z</end></unavailability_Time_Period.timeInterval>\
<TimeSeries><mRID>1</mRID><businessType>A54</businessType>\
<biddingZone_Domain.mRID codingScheme=\"A01\">10YFI-1--------U</biddingZone_Domain.mRID>\
<start_DateAndOrTime.date>2024-01-05</start_DateAndOrTime.date>\
<start_DateAndOrTime.time>08:30:00Z</start_DateAndOrTime.time>\
<end_DateAndOrTime.date>2024-01-05</end_DateAndOrTime.date>\
<end_DateAndOrTime.time>12:00:00Z</end_DateAndOrTime.time>\
<quantity_Measure_Unit.name>MAW</quantity_Measure_Unit.name>\
<quantity>250.0</quantity></TimeSeries>\
<TimeSeries><businessType>A53</businessType>\
<biddingZone_Domain.mRID codingScheme=\"A01\">10YFI-1--------U</biddingZone_Domain.mRID>\
<start_DateAndOrTime.date>2024-01-10</start_DateAndOrTime.date>\
<start_DateAndOrTime.time>00:00:00Z</start_DateAndOrTime.time>\
<end_DateAndOrTime.date>2024-01-10</end_DateAndOrTime.date>\
<end_DateAndOrTime.time>06:15:00Z</end_DateAndOrTime.time></TimeSeries>\
</Unavailability_MarketDocument>";

    #[test]
    fn parses_two_time_series_with_codes() {
        let records = parse_document(SAMPLE);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].business_type_code, 2);
        assert_eq!(records[1].business_type_code, 1);
        assert_eq!(records[0].domain, "10YFI-1--------U");
        assert_eq!(records[0].quantity, Some(250.0));
        assert_eq!(records[1].quantity, None);
        let expected =
            days_from_civil(2024, 1, 5).unwrap() as f64 * 86400.0 + 8.0 * 3600.0 + 30.0 * 60.0;
        assert!((records[0].start_unix - expected).abs() < 1e-9);
    }

    #[test]
    fn a_missing_tag_yields_none_not_zero() {
        let block = "<TimeSeries><businessType>A54</businessType></TimeSeries>";
        assert!(parse_series(block).is_none());
        assert!(tag_text(block, "biddingZone_Domain.mRID").is_none());
    }

    #[test]
    fn bin_roundtrip_restores_the_records() {
        let records = parse_document(SAMPLE);
        let bytes = write_bin(&records);
        assert_eq!(&bytes[0..4], &MAGIC[..]);
        let parsed = parse_bin(&bytes).expect("roundtrip parses");
        assert_eq!(parsed, records);
    }

    #[test]
    fn a_foreign_magic_yields_none() {
        assert!(parse_bin(b"XXXX\x00\x00\x00\x00").is_none());
    }

    #[test]
    fn error_head_hides_the_stylesheet() {
        let html = "<!DOCTYPE html><html><head><title>Blocked</title>\
<style>.x{color:red}</style></head><body><h1>Access denied</h1><p>contact</p></body></html>";
        let head = error_head(html.as_bytes(), 1500);
        assert!(head.contains("Blocked"));
        assert!(head.contains("Access denied"));
        assert!(!head.contains("color:red"));
    }

    #[test]
    fn compact_period_is_yyyymmddhhmm() {
        assert_eq!(
            compact_period("2024-01-05").as_deref(),
            Some("202401050000")
        );
        assert!(compact_period("2024-13-05").is_none());
    }

    #[test]
    fn inflate_reads_a_stored_block() {
        let deflate: &[u8] = &[0x01, 0x05, 0x00, 0xfa, 0xff, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(inflate(deflate).as_deref(), Some(&b"hello"[..]));
    }

    #[test]
    fn inflate_reads_a_fixed_huffman_block() {
        let deflate: &[u8] = &[0x73, 0x74, 0x02, 0x00];
        assert_eq!(inflate(deflate).as_deref(), Some(&b"AB"[..]));
    }

    #[test]
    fn zip_local_header_returns_the_stored_payload() {
        const STORED_ZIP: &[u8] = &[
            0x50, 0x4b, 0x03, 0x04, // local file header
            0x14, 0x00, // version needed
            0x00, 0x00, // flags
            0x00, 0x00, // method 0 = stored
            0x00, 0x00, // modification time
            0x00, 0x00, // modification date
            0x00, 0x00, 0x00, 0x00, // crc32
            0x05, 0x00, 0x00, 0x00, // compressed size
            0x05, 0x00, 0x00, 0x00, // uncompressed size
            0x05, 0x00, // filename length
            0x00, 0x00, // extra length
            b'a', b'.', b'x', b'm', b'l', // filename
            b'h', b'e', b'l', b'l', b'o', // payload
        ];
        assert_eq!(zip_entries(STORED_ZIP), vec![b"hello".to_vec()]);
        assert_eq!(xml_from_response(STORED_ZIP).as_deref(), Some("hello"));
    }
}
