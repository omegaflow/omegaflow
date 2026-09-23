use super::*;

const MESG_RECORD: u16 = 20;
const MESG_HR: u16 = 132;

const FIELD_RECORD_TEMPERATURE: u8 = 13;
const FIELD_HR_EVENT_TIMESTAMP: u8 = 9;
const FIELD_HR_EVENT_TIMESTAMP_12: u8 = 10;

const BASE_SINT8: u8 = 0x01;
const BASE_UINT32: u8 = 0x86;
const BASE_BYTE: u8 = 0x0D;

const EVENT_TIMESTAMP_SCALE: f64 = 1024.0;

const EVENT_TIMESTAMP_12_BITS: u32 = 12;
const EVENT_TIMESTAMP_12_CARRY_BITS: u32 = 18;
const EVENT_TIMESTAMP_12_MASK: u32 = (1u32 << EVENT_TIMESTAMP_12_BITS) - 1;
const EVENT_TIMESTAMP_12_COUNTER_MASK: u32 =
    (1u32 << (EVENT_TIMESTAMP_12_BITS + EVENT_TIMESTAMP_12_CARRY_BITS)) - 1;

const CRC_TABLE: [u16; 16] = [
    0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001, 0x6C00, 0x7800, 0xB401,
    0x5000, 0x9C01, 0x8801, 0x4400,
];

fn crc16(bytes: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &b in bytes {
        let mut tmp = CRC_TABLE[(crc & 0xF) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc ^= tmp ^ CRC_TABLE[(b & 0xF) as usize];

        tmp = CRC_TABLE[(crc & 0xF) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc ^= tmp ^ CRC_TABLE[((b >> 4) & 0xF) as usize];
    }
    crc
}

#[derive(Clone, Copy)]
enum Endian {
    Little,
    Big,
}

impl Endian {
    fn u16(self, b: &[u8]) -> u16 {
        match self {
            Endian::Little => u16::from_le_bytes([b[0], b[1]]),
            Endian::Big => u16::from_be_bytes([b[0], b[1]]),
        }
    }

    fn u32(self, b: &[u8]) -> u32 {
        match self {
            Endian::Little => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            Endian::Big => u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
        }
    }
}

#[derive(Clone, Copy)]
struct FieldDef {
    num: u8,
    size: u8,
    base_type: u8,
}

#[derive(Clone)]
struct DefMessage {
    global: u16,
    endian: Endian,
    fields: Vec<FieldDef>,
    dev_fields: Vec<FieldDef>,
}

pub fn parse_fit(bytes: &[u8]) -> Option<Vec<(String, f64, Option<f64>)>> {
    if bytes.len() < 12 {
        return None;
    }
    if &bytes[8..12] != b".FIT" {
        return None;
    }
    let header_size = bytes[0] as usize;
    if header_size < 12 || header_size > bytes.len() {
        return None;
    }
    let data_size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    if header_size >= 14 {
        let stored = u16::from_le_bytes([bytes[12], bytes[13]]);
        if stored != 0 && crc16(&bytes[..12]) != stored {
            return None;
        }
    }
    let record_end = header_size + data_size;
    if record_end > bytes.len() {
        return None;
    }
    if record_end + 2 <= bytes.len() {
        let stored = u16::from_le_bytes([bytes[record_end], bytes[record_end + 1]]);
        if crc16(&bytes[..record_end]) != stored {
            return None;
        }
    }
    let mut out = Vec::new();
    decode_records(bytes, header_size, record_end, &mut out);
    Some(out)
}

fn decode_records(
    bytes: &[u8],
    start: usize,
    end: usize,
    out: &mut Vec<(String, f64, Option<f64>)>,
) {
    let mut defs: [Option<DefMessage>; 16] = std::array::from_fn(|_| None);
    let mut pos = start;
    while pos < end {
        let header = match bytes.get(pos) {
            Some(&h) => h,
            None => return,
        };
        pos += 1;
        if header & 0x80 != 0 {
            let local = ((header >> 5) & 0x03) as usize;
            let Some(def) = defs[local].clone() else {
                return;
            };
            parse_data(bytes, &mut pos, &def, out);
        } else if header & 0x40 != 0 {
            let local = (header & 0x0F) as usize;
            if let Some(def) = parse_definition(bytes, &mut pos, header) {
                defs[local] = Some(def);
            } else {
                return;
            }
        } else {
            let local = (header & 0x0F) as usize;
            let Some(def) = defs[local].clone() else {
                return;
            };
            parse_data(bytes, &mut pos, &def, out);
        }
    }
}

fn parse_definition(bytes: &[u8], pos: &mut usize, header: u8) -> Option<DefMessage> {
    *pos += 1; // reserved byte
    let arch = match bytes.get(*pos)? {
        0 => Endian::Little,
        1 => Endian::Big,
        _ => return None,
    };
    *pos += 1;
    let global = arch.u16(bytes.get(*pos..*pos + 2)?);
    *pos += 2;
    let num_fields = *bytes.get(*pos)?;
    *pos += 1;
    let mut fields = Vec::new();
    for _ in 0..num_fields {
        let num = *bytes.get(*pos)?;
        *pos += 1;
        let size = *bytes.get(*pos)?;
        *pos += 1;
        let base_type = *bytes.get(*pos)?;
        *pos += 1;
        fields.push(FieldDef {
            num,
            size,
            base_type,
        });
    }
    let mut dev_fields = Vec::new();
    if header & 0x20 != 0 {
        let num_dev = *bytes.get(*pos)?;
        *pos += 1;
        for _ in 0..num_dev {
            let num = *bytes.get(*pos)?;
            *pos += 1;
            let size = *bytes.get(*pos)?;
            *pos += 1;
            *pos += 1; // developer data index
            dev_fields.push(FieldDef {
                num,
                size,
                base_type: 0,
            });
        }
    }
    Some(DefMessage {
        global,
        endian: arch,
        fields,
        dev_fields,
    })
}

fn parse_data(
    bytes: &[u8],
    pos: &mut usize,
    def: &DefMessage,
    out: &mut Vec<(String, f64, Option<f64>)>,
) {
    for field in &def.fields {
        let size = field.size as usize;
        let field_bytes = match bytes.get(*pos..*pos + size) {
            Some(b) => b,
            None => return,
        };
        *pos += size;
        match (def.global, field.num) {
            (MESG_RECORD, FIELD_RECORD_TEMPERATURE) => {
                if field.base_type == BASE_SINT8 && size == 1 {
                    let raw = field_bytes[0];
                    if raw != 0x7F {
                        out.push(("temperature".to_string(), raw as i8 as f64, None));
                    }
                }
            }
            (MESG_HR, FIELD_HR_EVENT_TIMESTAMP) if field.base_type == BASE_UINT32 => {
                let mut beats = Vec::new();
                for chunk in field_bytes.as_chunks::<4>().0 {
                    let v = def.endian.u32(chunk);
                    if v != 0xFFFF_FFFF {
                        beats.push(v);
                    }
                }
                emit_nn(&beats, out);
            }
            (MESG_HR, FIELD_HR_EVENT_TIMESTAMP_12) if field.base_type == BASE_BYTE => {
                let samples = unpack_timestamp_12(field_bytes);
                emit_nn(&cumulative_timestamp_12(&samples), out);
            }
            _ => {}
        }
    }
    for field in &def.dev_fields {
        *pos += field.size as usize;
    }
}

fn emit_nn(beats: &[u32], out: &mut Vec<(String, f64, Option<f64>)>) {
    for pair in beats.windows(2) {
        if pair[1] <= pair[0] {
            continue;
        }
        let ms = (pair[1] - pair[0]) as f64 * 1000.0 / EVENT_TIMESTAMP_SCALE;
        if ms.is_finite() && ms > 0.0 {
            out.push(("nn".to_string(), ms, None));
        }
    }
}

fn unpack_timestamp_12(field_bytes: &[u8]) -> Vec<u16> {
    let mut samples = Vec::with_capacity(field_bytes.len() / 3 * 2);
    for chunk in field_bytes.as_chunks::<3>().0 {
        samples.push((chunk[0] as u16) | (((chunk[1] & 0x0F) as u16) << 8));
        samples.push(((chunk[1] >> 4) as u16) | ((chunk[2] as u16) << 4));
    }
    samples
}

fn cumulative_timestamp_12(samples: &[u16]) -> Vec<u32> {
    let mut accumulated: u32 = 0;
    let mut last: u32 = 0;
    let mut timestamps = Vec::with_capacity(samples.len());
    for &sample in samples {
        let raw = sample as u32 & EVENT_TIMESTAMP_12_MASK;
        let delta = raw.wrapping_sub(last) & EVENT_TIMESTAMP_12_MASK;
        accumulated = accumulated.wrapping_add(delta) & EVENT_TIMESTAMP_12_COUNTER_MASK;
        last = raw;
        timestamps.push(accumulated);
    }
    timestamps
}

pub fn fit_ingress(tx: mpsc::Sender<Vec<(String, f64, Option<f64>)>>) {
    let Ok(dir) = std::env::var("FIT_DIR") else {
        return;
    };
    let mut seen: HashSet<String> = HashSet::new();
    loop {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if !name.to_lowercase().ends_with(".fit") {
                    continue;
                }
                if seen.contains(&name) {
                    continue;
                }
                if let Ok(bytes) = std::fs::read(entry.path()) {
                    seen.insert(name);
                    if let Some(batch) = parse_fit(&bytes)
                        && !batch.is_empty()
                    {
                        let _ = tx.send(batch);
                    }
                }
            }
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(local: u8, global: u16, fields: &[(u8, u8, u8)]) -> Vec<u8> {
        let mut d = Vec::new();
        d.push(0x40 | (local & 0x0F));
        d.push(0x00); // reserved
        d.push(0x00); // little-endian architecture
        d.extend_from_slice(&global.to_le_bytes());
        d.push(fields.len() as u8);
        for &(num, size, base_type) in fields {
            d.push(num);
            d.push(size);
            d.push(base_type);
        }
        d
    }

    fn data(local: u8, payload: &[u8]) -> Vec<u8> {
        let mut d = Vec::new();
        d.push(local & 0x0F);
        d.extend_from_slice(payload);
        d
    }

    fn compressed_data(local: u8, offset: u8, payload: &[u8]) -> Vec<u8> {
        let mut d = Vec::new();
        d.push(0x80 | ((local & 0x03) << 5) | (offset & 0x1F));
        d.extend_from_slice(payload);
        d
    }

    fn make_fit(records: &[u8]) -> Vec<u8> {
        let mut f = vec![
            0x0C, // header size 12
            0x10, // protocol 1.0
            0x00, // profile version low
            0x00, // profile version high
        ];
        f.extend_from_slice(&(records.len() as u32).to_le_bytes());
        f.extend_from_slice(b".FIT");
        f.extend_from_slice(records);
        let crc = crc16(&f);
        f.extend_from_slice(&crc.to_le_bytes());
        f
    }

    #[test]
    fn crc_reproduces_real_garmin_header_crcs() {
        let a = [
            0x0e, 0x10, 0xf0, 0x07, 0x45, 0xe6, 0x00, 0x00, 0x2e, 0x46, 0x49, 0x54,
        ];
        assert_eq!(crc16(&a), 0xB703);
        let b = [
            0x0e, 0x10, 0x77, 0x08, 0x00, 0xd9, 0x00, 0x00, 0x2e, 0x46, 0x49, 0x54,
        ];
        assert_eq!(crc16(&b), 0x79FF);
    }

    #[test]
    fn record_temperature_and_hr_nn_parse() {
        let mut records = definition(0, MESG_RECORD, &[(FIELD_RECORD_TEMPERATURE, 1, BASE_SINT8)]);
        records.extend(data(0, &[23]));
        records.extend(definition(
            1,
            MESG_HR,
            &[(FIELD_HR_EVENT_TIMESTAMP, 12, BASE_UINT32)],
        ));
        let mut beats = Vec::new();
        for v in [0u32, 1024, 2048] {
            beats.extend_from_slice(&v.to_le_bytes());
        }
        records.extend(data(1, &beats));

        let out = parse_fit(&make_fit(&records)).expect("valid fit");
        assert!(out.iter().any(|(k, v, _)| k == "temperature" && *v == 23.0));
        let nn: Vec<f64> = out
            .iter()
            .filter(|(k, _, _)| k == "nn")
            .map(|(_, v, _)| *v)
            .collect();
        assert_eq!(nn, vec![1000.0, 1000.0]);
    }

    #[test]
    fn record_temperature_sint8_is_signed() {
        let mut records = definition(0, MESG_RECORD, &[(FIELD_RECORD_TEMPERATURE, 1, BASE_SINT8)]);
        records.extend(data(0, &[0xFB])); // -5
        let out = parse_fit(&make_fit(&records)).expect("valid fit");
        assert!(out.iter().any(|(k, v, _)| k == "temperature" && *v == -5.0));
    }

    #[test]
    fn record_temperature_invalid_sentinel_is_absent() {
        let mut records = definition(0, MESG_RECORD, &[(FIELD_RECORD_TEMPERATURE, 1, BASE_SINT8)]);
        records.extend(data(0, &[0x7F])); // sint8 invalid
        let out = parse_fit(&make_fit(&records)).expect("valid fit");
        assert!(out.is_empty());
    }

    #[test]
    fn compressed_timestamp_header_links_without_timestamp_field() {
        let mut records = definition(
            0,
            MESG_RECORD,
            &[(253, 4, BASE_UINT32), (13, 1, BASE_SINT8)],
        );
        let mut msg0 = Vec::new();
        msg0.extend_from_slice(&42u32.to_le_bytes());
        msg0.push(23);
        records.extend(data(0, &msg0));

        records.extend(definition(1, MESG_RECORD, &[(13, 1, BASE_SINT8)]));
        records.extend(compressed_data(1, 5, &[24]));

        let out = parse_fit(&make_fit(&records)).expect("valid fit");
        let temps: Vec<f64> = out
            .iter()
            .filter(|(k, _, _)| k == "temperature")
            .map(|(_, v, _)| *v)
            .collect();
        assert_eq!(temps, vec![23.0, 24.0]);
    }

    #[test]
    fn hr_event_timestamp_yields_nn_differences() {
        let mut records = definition(0, MESG_HR, &[(FIELD_HR_EVENT_TIMESTAMP, 20, BASE_UINT32)]);
        let mut beats = Vec::new();
        for v in [0u32, 1024, 2048, 3072, 4096] {
            beats.extend_from_slice(&v.to_le_bytes());
        }
        records.extend(data(0, &beats));
        let out = parse_fit(&make_fit(&records)).expect("valid fit");
        let nn: Vec<f64> = out
            .iter()
            .filter(|(k, _, _)| k == "nn")
            .map(|(_, v, _)| *v)
            .collect();
        assert_eq!(nn, vec![1000.0, 1000.0, 1000.0, 1000.0]);
    }

    #[test]
    fn hr_event_timestamp_skips_invalid_sentinel() {
        let mut records = definition(0, MESG_HR, &[(FIELD_HR_EVENT_TIMESTAMP, 16, BASE_UINT32)]);
        let mut beats = Vec::new();
        for v in [0u32, 1024, u32::MAX, 2048] {
            beats.extend_from_slice(&v.to_le_bytes());
        }
        records.extend(data(0, &beats));
        let out = parse_fit(&make_fit(&records)).expect("valid fit");
        let nn: Vec<f64> = out
            .iter()
            .filter(|(k, _, _)| k == "nn")
            .map(|(_, v, _)| *v)
            .collect();
        assert_eq!(nn, vec![1000.0, 1000.0]);
    }

    #[test]
    fn hr_event_timestamp_12_unpacks_and_carries_rollover() {
        let packed = [0x00, 0x0F, 0x30, 0x00, 0x07, 0xB0];
        assert_eq!(
            unpack_timestamp_12(&packed),
            vec![0x0F00, 0x0300, 0x0700, 0x0B00]
        );

        let mut records = definition(0, MESG_HR, &[(FIELD_HR_EVENT_TIMESTAMP_12, 6, BASE_BYTE)]);
        records.extend(data(0, &packed));

        let out = parse_fit(&make_fit(&records)).expect("valid fit");
        let nn: Vec<f64> = out
            .iter()
            .filter(|(k, _, _)| k == "nn")
            .map(|(_, v, _)| *v)
            .collect();
        assert_eq!(nn, vec![1000.0, 1000.0, 1000.0]);
    }

    #[test]
    fn fit_rejects_a_corrupted_crc() {
        let mut records = definition(0, MESG_RECORD, &[(FIELD_RECORD_TEMPERATURE, 1, BASE_SINT8)]);
        records.extend(data(0, &[23]));
        let mut file = make_fit(&records);
        let idx = file.len() - 4; // a record byte, before the trailing 2 CRC bytes
        file[idx] ^= 0xFF;
        assert_eq!(parse_fit(&file), None);
    }

    #[test]
    fn fit_rejects_an_absent_magic() {
        let mut file = make_fit(&[]);
        file[8] = b'X';
        assert_eq!(parse_fit(&file), None);
    }
}
