const MAGIC: [u8; 4] = *b"GRIB";
const EDITION: u8 = 2;
const HEADER: usize = 16;
const TERMINATOR: [u8; 4] = *b"7777";

fn be_u16(b: &[u8]) -> u16 {
    u16::from_be_bytes([b[0], b[1]])
}

fn be_u32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

fn be_u64(b: &[u8]) -> u64 {
    u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Grib2Section {
    pub number: u8,
    pub offset: usize,
    pub length: usize,
}

#[derive(Clone, Debug)]
pub struct Grib2Indicator {
    pub discipline: u8,
    pub edition: u8,
    pub total_length: u64,
}

#[derive(Clone, Debug)]
pub struct Grib2Identification {
    pub centre: u16,
    pub subcentre: u16,
    pub tables_version: u8,
    pub local_tables_version: u8,
    pub ref_time_significance: u8,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub production_status: u8,
    pub processed_data_type: u8,
}

#[derive(Clone, Debug)]
pub struct Grib2GridDefinition {
    pub source: u8,
    pub num_points: u32,
    pub list_octets: u8,
    pub list: Vec<u8>,
    pub list_interpretation: u8,
    pub template_number: u16,
    pub template: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Grib2ProductDefinition {
    pub num_coord_values: u16,
    pub template_number: u16,
    pub parameter_category: Option<u8>,
    pub parameter_number: Option<u8>,
    pub template: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Grib2DataRepresentation {
    pub num_points: u32,
    pub template_number: u16,
    pub template: Vec<u8>,
}

impl Grib2DataRepresentation {
    pub fn packing_name(&self) -> Option<&'static str> {
        match self.template_number {
            0 => Some("simple packing"),
            2 => Some("complex packing"),
            3 => Some("complex packing & spatial differencing"),
            40 | 40000 => Some("JPEG2000"),
            41 | 40010 => Some("PNG"),
            42 | 40020 => Some("CCITT-G4"),
            200 | 20000 => Some("run-length"),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Grib2BitMap {
    pub indicator: u8,
}

#[derive(Clone, Debug)]
pub struct Grib2Message {
    pub indicator: Grib2Indicator,
    pub identification: Option<Grib2Identification>,
    pub local_use_length: Option<usize>,
    pub grid: Option<Grib2GridDefinition>,
    pub product: Option<Grib2ProductDefinition>,
    pub representation: Option<Grib2DataRepresentation>,
    pub bitmap: Option<Grib2BitMap>,
    pub data_length: Option<usize>,
    pub values: Option<Vec<f32>>,
    pub sections: Vec<Grib2Section>,
}

#[derive(Clone, Debug)]
pub enum Grib2Note {
    Magic { bytes: [u8; 4] },
    Edition { edition: u8 },
    Length { total: u64, bytes: usize },
    SectionLength { number: u8, off: usize },
    Terminator { off: usize },
    EndAtByte { off: usize },
}

impl Grib2Message {
    pub fn parse(bytes: &[u8]) -> Result<Grib2Message, Grib2Note> {
        if bytes.len() < 4 {
            return Err(Grib2Note::EndAtByte { off: bytes.len() });
        }
        let magic = [bytes[0], bytes[1], bytes[2], bytes[3]];
        if magic != MAGIC {
            return Err(Grib2Note::Magic { bytes: magic });
        }
        if bytes.len() < HEADER {
            return Err(Grib2Note::EndAtByte { off: bytes.len() });
        }
        let discipline = bytes[6];
        let edition = bytes[7];
        if edition != EDITION {
            return Err(Grib2Note::Edition { edition });
        }
        let total_length = be_u64(&bytes[8..16]);
        if total_length < HEADER as u64 || total_length as usize > bytes.len() {
            return Err(Grib2Note::Length {
                total: total_length,
                bytes: bytes.len(),
            });
        }
        let end = total_length as usize;

        let mut identification = None;
        let mut local_use_length = None;
        let mut grid = None;
        let mut product = None;
        let mut representation = None;
        let mut bitmap = None;
        let mut data_length = None;
        let mut values = None;
        let mut sections = Vec::new();
        let mut pos = HEADER;
        let mut terminator = None;

        while pos < end {
            if end - pos >= 4 && bytes[pos..pos + 4] == TERMINATOR {
                sections.push(Grib2Section {
                    number: 8,
                    offset: pos,
                    length: 4,
                });
                terminator = Some(pos);
                pos += 4;
                break;
            }
            if end - pos < 5 {
                return Err(Grib2Note::Terminator { off: pos });
            }
            let length = be_u32(&bytes[pos..pos + 4]) as usize;
            let number = bytes[pos + 4];
            let section_end = pos
                .checked_add(length)
                .ok_or(Grib2Note::SectionLength { number, off: pos })?;
            if length < 5 || section_end > end {
                return Err(Grib2Note::SectionLength { number, off: pos });
            }
            let payload = &bytes[pos + 5..section_end];
            match number {
                1 => identification = Some(parse_identification(payload)?),
                2 => local_use_length = Some(length),
                3 => grid = Some(parse_grid(payload)?),
                4 => product = Some(parse_product(payload)?),
                5 => representation = Some(parse_representation(payload)?),
                6 => {
                    if payload.is_empty() {
                        return Err(Grib2Note::EndAtByte { off: pos + 5 });
                    }
                    bitmap = Some(Grib2BitMap {
                        indicator: payload[0],
                    });
                }
                7 => {
                    data_length = Some(length);
                    values = representation
                        .as_ref()
                        .and_then(|r| match r.template_number {
                            0 => decode_simple_packing(r, payload),
                            2 => decode_complex_packing(r, payload),
                            3 => decode_complex_spatial_packing(r, payload),
                            42 | 40020 => decode_ccitt_g4(r, payload),
                            _ => None,
                        });
                }
                _ => {}
            }
            sections.push(Grib2Section {
                number,
                offset: pos,
                length,
            });
            pos = section_end;
        }

        if terminator.is_none() {
            return Err(Grib2Note::Terminator { off: pos });
        }

        Ok(Grib2Message {
            indicator: Grib2Indicator {
                discipline,
                edition,
                total_length,
            },
            identification,
            local_use_length,
            grid,
            product,
            representation,
            bitmap,
            data_length,
            values,
            sections,
        })
    }

    pub fn parse_all(bytes: &[u8]) -> Result<Vec<Grib2Message>, Grib2Note> {
        let mut out = Vec::new();
        let mut off = 0usize;
        while off < bytes.len() {
            let message = Grib2Message::parse(&bytes[off..])?;
            off += message.indicator.total_length as usize;
            out.push(message);
        }
        Ok(out)
    }
}

fn parse_identification(p: &[u8]) -> Result<Grib2Identification, Grib2Note> {
    if p.len() < 16 {
        return Err(Grib2Note::EndAtByte { off: p.len() });
    }
    Ok(Grib2Identification {
        centre: be_u16(&p[0..2]),
        subcentre: be_u16(&p[2..4]),
        tables_version: p[4],
        local_tables_version: p[5],
        ref_time_significance: p[6],
        year: be_u16(&p[7..9]),
        month: p[9],
        day: p[10],
        hour: p[11],
        minute: p[12],
        second: p[13],
        production_status: p[14],
        processed_data_type: p[15],
    })
}

fn parse_grid(p: &[u8]) -> Result<Grib2GridDefinition, Grib2Note> {
    if p.len() < 9 {
        return Err(Grib2Note::EndAtByte { off: p.len() });
    }
    let source = p[0];
    let num_points = be_u32(&p[1..5]);
    let list_octets = p[5];
    let list_interpretation = p[6];
    let template_number = be_u16(&p[7..9]);
    let n = list_octets as usize;
    if p.len() < 9 + n {
        return Err(Grib2Note::EndAtByte { off: p.len() });
    }
    let template = p[9..p.len() - n].to_vec();
    let list = p[p.len() - n..].to_vec();
    Ok(Grib2GridDefinition {
        source,
        num_points,
        list_octets,
        list,
        list_interpretation,
        template_number,
        template,
    })
}

fn parse_product(p: &[u8]) -> Result<Grib2ProductDefinition, Grib2Note> {
    if p.len() < 4 {
        return Err(Grib2Note::EndAtByte { off: p.len() });
    }
    let num_coord_values = be_u16(&p[0..2]);
    let template_number = be_u16(&p[2..4]);
    let template = p[4..].to_vec();
    let (parameter_category, parameter_number) = if template.len() >= 2 {
        (Some(template[0]), Some(template[1]))
    } else {
        (None, None)
    };
    Ok(Grib2ProductDefinition {
        num_coord_values,
        template_number,
        parameter_category,
        parameter_number,
        template,
    })
}

fn parse_representation(p: &[u8]) -> Result<Grib2DataRepresentation, Grib2Note> {
    if p.len() < 6 {
        return Err(Grib2Note::EndAtByte { off: p.len() });
    }
    Ok(Grib2DataRepresentation {
        num_points: be_u32(&p[0..4]),
        template_number: be_u16(&p[4..6]),
        template: p[6..].to_vec(),
    })
}

struct MsbBitReader<'a> {
    data: &'a [u8],
    pos: usize,
    bitpos: u8,
}

impl<'a> MsbBitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        MsbBitReader {
            data,
            pos: 0,
            bitpos: 0,
        }
    }

    fn read_bit(&mut self) -> Option<u32> {
        let byte = *self.data.get(self.pos)?;
        let bit = (byte >> (7 - self.bitpos)) & 1;
        self.bitpos += 1;
        if self.bitpos == 8 {
            self.bitpos = 0;
            self.pos += 1;
        }
        Some(bit as u32)
    }

    fn read_bits(&mut self, n: u32) -> Option<u32> {
        let mut v = 0u32;
        for _ in 0..n {
            v = (v << 1) | self.read_bit()?;
        }
        Some(v)
    }
}

const CCITT_WHITE: &[(u16, u8, u16)] = &[
    (0, 8, 0x35),
    (1, 6, 0x7),
    (2, 4, 0x7),
    (3, 4, 0x8),
    (4, 4, 0xB),
    (5, 4, 0xC),
    (6, 4, 0xE),
    (7, 4, 0xF),
    (8, 5, 0x13),
    (9, 5, 0x14),
    (10, 5, 0x7),
    (11, 5, 0x8),
    (12, 6, 0x8),
    (13, 6, 0x3),
    (14, 6, 0x34),
    (15, 6, 0x35),
    (16, 6, 0x2A),
    (17, 6, 0x2B),
    (18, 7, 0x27),
    (19, 7, 0xC),
    (20, 7, 0x8),
    (21, 7, 0x17),
    (22, 7, 0x3),
    (23, 7, 0x4),
    (24, 7, 0x28),
    (25, 7, 0x2B),
    (26, 7, 0x13),
    (27, 7, 0x24),
    (28, 7, 0x18),
    (29, 8, 0x2),
    (30, 8, 0x3),
    (31, 8, 0x1A),
    (32, 8, 0x1B),
    (33, 8, 0x12),
    (34, 8, 0x13),
    (35, 8, 0x14),
    (36, 8, 0x15),
    (37, 8, 0x16),
    (38, 8, 0x17),
    (39, 8, 0x28),
    (40, 8, 0x29),
    (41, 8, 0x2A),
    (42, 8, 0x2B),
    (43, 8, 0x2C),
    (44, 8, 0x2D),
    (45, 8, 0x4),
    (46, 8, 0x5),
    (47, 8, 0xA),
    (48, 8, 0xB),
    (49, 8, 0x52),
    (50, 8, 0x53),
    (51, 8, 0x54),
    (52, 8, 0x55),
    (53, 8, 0x24),
    (54, 8, 0x25),
    (55, 8, 0x58),
    (56, 8, 0x59),
    (57, 8, 0x5A),
    (58, 8, 0x5B),
    (59, 8, 0x4A),
    (60, 8, 0x4B),
    (61, 8, 0x32),
    (62, 8, 0x33),
    (63, 8, 0x34),
    (64, 5, 0x1B),
    (128, 5, 0x12),
    (192, 6, 0x17),
    (256, 7, 0x37),
    (320, 8, 0x36),
    (384, 8, 0x37),
    (448, 8, 0x64),
    (512, 8, 0x65),
    (576, 8, 0x68),
    (640, 8, 0x67),
    (704, 9, 0xCC),
    (768, 9, 0xCD),
    (832, 9, 0xD2),
    (896, 9, 0xD3),
    (960, 9, 0xD4),
    (1024, 9, 0xD5),
    (1088, 9, 0xD6),
    (1152, 9, 0xD7),
    (1216, 9, 0xD8),
    (1280, 9, 0xD9),
    (1344, 9, 0xDA),
    (1408, 9, 0xDB),
    (1472, 9, 0x98),
    (1536, 9, 0x99),
    (1600, 9, 0x9A),
    (1664, 6, 0x18),
    (1728, 9, 0x9B),
    (1792, 11, 0x8),
    (1856, 11, 0xC),
    (1920, 11, 0xD),
    (1984, 12, 0x12),
    (2048, 12, 0x13),
    (2112, 12, 0x14),
    (2176, 12, 0x15),
    (2240, 12, 0x16),
    (2304, 12, 0x17),
    (2368, 12, 0x1C),
    (2432, 12, 0x1D),
    (2496, 12, 0x1E),
    (2560, 12, 0x1F),
];

const CCITT_BLACK: &[(u16, u8, u16)] = &[
    (0, 10, 0x37),
    (1, 3, 0x2),
    (2, 2, 0x3),
    (3, 2, 0x2),
    (4, 3, 0x3),
    (5, 4, 0x3),
    (6, 4, 0x2),
    (7, 5, 0x3),
    (8, 6, 0x5),
    (9, 6, 0x4),
    (10, 7, 0x4),
    (11, 7, 0x5),
    (12, 7, 0x7),
    (13, 8, 0x4),
    (14, 8, 0x7),
    (15, 9, 0x18),
    (16, 10, 0x17),
    (17, 10, 0x18),
    (18, 10, 0x8),
    (19, 11, 0x67),
    (20, 11, 0x68),
    (21, 11, 0x6C),
    (22, 11, 0x37),
    (23, 11, 0x28),
    (24, 11, 0x17),
    (25, 11, 0x18),
    (26, 12, 0xCA),
    (27, 12, 0xCB),
    (28, 12, 0xCC),
    (29, 12, 0xCD),
    (30, 12, 0x68),
    (31, 12, 0x69),
    (32, 12, 0x6A),
    (33, 12, 0x6B),
    (34, 12, 0xD2),
    (35, 12, 0xD3),
    (36, 12, 0xD4),
    (37, 12, 0xD5),
    (38, 12, 0xD6),
    (39, 12, 0xD7),
    (40, 12, 0x6C),
    (41, 12, 0x6D),
    (42, 12, 0xDA),
    (43, 12, 0xDB),
    (44, 12, 0x54),
    (45, 12, 0x55),
    (46, 12, 0x56),
    (47, 12, 0x57),
    (48, 12, 0x64),
    (49, 12, 0x65),
    (50, 12, 0x52),
    (51, 12, 0x53),
    (52, 12, 0x24),
    (53, 12, 0x37),
    (54, 12, 0x38),
    (55, 12, 0x27),
    (56, 12, 0x28),
    (57, 12, 0x58),
    (58, 12, 0x59),
    (59, 12, 0x2B),
    (60, 12, 0x2C),
    (61, 12, 0x5A),
    (62, 12, 0x66),
    (63, 12, 0x67),
    (64, 10, 0xF),
    (128, 12, 0xC8),
    (192, 12, 0xC9),
    (256, 12, 0x5B),
    (320, 12, 0x33),
    (384, 12, 0x34),
    (448, 12, 0x35),
    (512, 13, 0x6C),
    (576, 13, 0x6D),
    (640, 13, 0x4A),
    (704, 13, 0x4B),
    (768, 13, 0x4C),
    (832, 13, 0x4D),
    (896, 13, 0x72),
    (960, 13, 0x73),
    (1024, 13, 0x74),
    (1088, 13, 0x75),
    (1152, 13, 0x76),
    (1216, 13, 0x77),
    (1280, 13, 0x52),
    (1344, 13, 0x53),
    (1408, 13, 0x54),
    (1472, 13, 0x55),
    (1536, 13, 0x5A),
    (1600, 13, 0x5B),
    (1664, 13, 0x64),
    (1728, 13, 0x65),
    (1792, 11, 0x8),
    (1856, 11, 0xC),
    (1920, 11, 0xD),
    (1984, 12, 0x12),
    (2048, 12, 0x13),
    (2112, 12, 0x14),
    (2176, 12, 0x15),
    (2240, 12, 0x16),
    (2304, 12, 0x17),
    (2368, 12, 0x1C),
    (2432, 12, 0x1D),
    (2496, 12, 0x1E),
    (2560, 12, 0x1F),
];

fn ccitt_run(table: &[(u16, u8, u16)], bits: u8, code: u16) -> Option<u16> {
    table
        .iter()
        .find(|&&(_, b, c)| b == bits && c == code)
        .map(|&(run, _, _)| run)
}

fn decode_ccitt_g4(rep: &Grib2DataRepresentation, data: &[u8]) -> Option<Vec<f32>> {
    if rep.template_number != 42 && rep.template_number != 40020 {
        return None;
    }
    let mut rdr = MsbBitReader::new(data);
    let mut values = Vec::new();
    let mut black = false;
    let mut eols = 0u32;
    loop {
        let first = match rdr.read_bit() {
            Some(bit) => bit,
            None => break,
        };
        let mut code = first as u16;
        let mut bits = 1u8;
        let mut run = None;
        loop {
            if bits == 12 && code == 0x1 {
                break;
            }
            let table = if black { CCITT_BLACK } else { CCITT_WHITE };
            if let Some(r) = ccitt_run(table, bits, code) {
                run = Some(r);
                break;
            }
            if bits >= 13 {
                return None;
            }
            code = (code << 1) | rdr.read_bit()? as u16;
            bits += 1;
        }
        match run {
            Some(r) => {
                values.push(r as f32);
                black = !black;
                eols = 0;
            }
            None => {
                eols += 1;
                if eols >= 6 {
                    break;
                }
                black = false;
            }
        }
    }
    Some(values)
}

fn decode_simple_packing(rep: &Grib2DataRepresentation, data: &[u8]) -> Option<Vec<f32>> {
    if rep.template_number != 0 || rep.template.len() < 10 {
        return None;
    }
    let r = f32::from_be_bytes([
        rep.template[0],
        rep.template[1],
        rep.template[2],
        rep.template[3],
    ]);
    let e = i16::from_be_bytes([rep.template[4], rep.template[5]]);
    let d = i16::from_be_bytes([rep.template[6], rep.template[7]]);
    let bits = rep.template[8] as u32;
    if bits == 0 || bits > 31 {
        return None;
    }
    let two_pow_e = 2f64.powi(e as i32);
    let ten_pow_neg_d = 10f64.powi(-(d as i32));
    let mut rdr = MsbBitReader::new(data);
    let mut out = Vec::with_capacity(rep.num_points as usize);
    for _ in 0..rep.num_points {
        let x = rdr.read_bits(bits)?;
        let scaled = if x == 0 { 0.0f64 } else { x as f64 * two_pow_e };
        let v = (r as f64 + scaled) * ten_pow_neg_d;
        if !v.is_finite() {
            return None;
        }
        out.push(v as f32);
    }
    Some(out)
}

fn decode_complex_groups(template: &[u8], num_points: u32, data: &[u8]) -> Option<Vec<f32>> {
    if template.len() < 36 {
        return None;
    }
    let r = f32::from_be_bytes([template[0], template[1], template[2], template[3]]);
    let e = i16::from_be_bytes([template[4], template[5]]);
    let d = i16::from_be_bytes([template[6], template[7]]);
    let bits_group_refs = template[8] as u32;
    let ng = be_u32(&template[20..24]);
    let ref_group_widths = template[24] as u32;
    let bits_group_widths = template[25] as u32;
    let ref_group_lengths = be_u32(&template[26..30]);
    let length_increment = template[30] as u32;
    let bits_scaled_group_lengths = template[35] as u32;

    if ng == 0 || ng as usize > num_points as usize {
        return None;
    }
    for bits in [
        bits_group_refs,
        bits_group_widths,
        bits_scaled_group_lengths,
    ] {
        if bits > 31 {
            return None;
        }
    }
    let two_pow_e = 2f64.powi(e as i32);
    let ten_pow_neg_d = 10f64.powi(-(d as i32));
    let increment = if length_increment == 0 {
        1
    } else {
        length_increment
    };
    let mut rdr = MsbBitReader::new(data);

    let mut group_refs = Vec::with_capacity(ng as usize);
    let mut group_widths = Vec::with_capacity(ng as usize);
    let mut group_lengths = Vec::with_capacity(ng as usize);
    for _ in 0..ng {
        group_refs.push(rdr.read_bits(bits_group_refs)?);
        let width = ref_group_widths + rdr.read_bits(bits_group_widths)?;
        if width > 31 {
            return None;
        }
        group_widths.push(width);
        let scaled = ref_group_lengths + rdr.read_bits(bits_scaled_group_lengths)?;
        group_lengths.push(scaled.saturating_mul(increment));
    }

    let mut out = Vec::with_capacity(num_points as usize);
    let mut remaining = num_points;
    for g in 0..ng as usize {
        if remaining == 0 {
            break;
        }
        let n = group_lengths[g].min(remaining);
        remaining -= n;
        for _ in 0..n {
            let x = rdr.read_bits(group_widths[g])?;
            let v = (r as f64 + (group_refs[g] as f64 + x as f64) * two_pow_e) * ten_pow_neg_d;
            if !v.is_finite() {
                return None;
            }
            out.push(v as f32);
        }
    }
    if remaining != 0 {
        return None;
    }
    Some(out)
}

fn decode_complex_packing(rep: &Grib2DataRepresentation, data: &[u8]) -> Option<Vec<f32>> {
    if rep.template_number != 2 {
        return None;
    }
    decode_complex_groups(&rep.template, rep.num_points, data)
}

fn read_sign_magnitude(b: &[u8]) -> i64 {
    let mut raw = 0u64;
    for &byte in b {
        raw = (raw << 8) | byte as u64;
    }
    let sign_bit = 1u64 << (b.len() * 8 - 1);
    let magnitude = raw & (sign_bit - 1);
    if raw & sign_bit != 0 {
        -(magnitude as i64)
    } else {
        magnitude as i64
    }
}

fn decode_complex_spatial_packing(
    rep: &Grib2DataRepresentation,
    data: &[u8],
) -> Option<Vec<f32>> {
    if rep.template_number != 3 || rep.template.len() < 38 {
        return None;
    }
    let order = rep.template[36];
    let extra_octets = rep.template[37] as usize;
    if order != 1 && order != 2 {
        return None;
    }
    if extra_octets == 0 || extra_octets > 8 {
        return None;
    }
    let extra_len = order as usize * extra_octets;
    if data.len() < extra_len {
        return None;
    }
    let mut seeds = [0f32; 2];
    for (i, seed) in seeds.iter_mut().enumerate().take(order as usize) {
        let off = i * extra_octets;
        *seed = read_sign_magnitude(&data[off..off + extra_octets]) as f32;
    }

    let mut values = decode_complex_groups(&rep.template, rep.num_points, &data[extra_len..])?;
    if values.len() < order as usize {
        return None;
    }
    if order == 1 {
        values[0] = seeds[0];
        for i in 1..values.len() {
            values[i] += values[i - 1];
        }
    } else {
        values[0] = seeds[0];
        values[1] = seeds[1];
        for i in 2..values.len() {
            values[i] += 2.0 * values[i - 1] - values[i - 2];
        }
    }
    for v in &values {
        if !v.is_finite() {
            return None;
        }
    }
    Some(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section(number: u8, payload: &[u8]) -> Vec<u8> {
        let len = 5 + payload.len();
        let mut b = (len as u32).to_be_bytes().to_vec();
        b.push(number);
        b.extend_from_slice(payload);
        b
    }

    fn identification_payload() -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&7u16.to_be_bytes());
        p.extend_from_slice(&0u16.to_be_bytes());
        p.push(2);
        p.push(1);
        p.push(1);
        p.extend_from_slice(&2024u16.to_be_bytes());
        p.push(1);
        p.push(2);
        p.push(3);
        p.push(4);
        p.push(5);
        p.push(0);
        p.push(1);
        p
    }

    fn grid_payload() -> Vec<u8> {
        let mut p = Vec::new();
        p.push(0);
        p.extend_from_slice(&100u32.to_be_bytes());
        p.push(0);
        p.push(0);
        p.extend_from_slice(&0u16.to_be_bytes());
        p.extend_from_slice(&[1, 2, 3, 4]);
        p
    }

    fn product_payload() -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&0u16.to_be_bytes());
        p.extend_from_slice(&0u16.to_be_bytes());
        p.push(3);
        p.push(1);
        p.extend_from_slice(&[0u8; 23]);
        p
    }

    fn representation_payload() -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&100u32.to_be_bytes());
        p.extend_from_slice(&0u16.to_be_bytes());
        p.extend_from_slice(&[0u8; 10]);
        p
    }

    fn minimal_message() -> Vec<u8> {
        let s1 = section(1, &identification_payload());
        let s3 = section(3, &grid_payload());
        let s4 = section(4, &product_payload());
        let s5 = section(5, &representation_payload());
        let s7 = section(7, &[0xAB, 0xCD, 0xEF, 0x12]);
        let total = 16 + s1.len() + s3.len() + s4.len() + s5.len() + s7.len() + 4;
        let mut b = Vec::new();
        b.extend_from_slice(&MAGIC);
        b.extend_from_slice(&[0, 0]);
        b.push(0);
        b.push(EDITION);
        b.extend_from_slice(&(total as u64).to_be_bytes());
        b.extend_from_slice(&s1);
        b.extend_from_slice(&s3);
        b.extend_from_slice(&s4);
        b.extend_from_slice(&s5);
        b.extend_from_slice(&s7);
        b.extend_from_slice(&TERMINATOR);
        b
    }

    #[test]
    fn minimal_message_layout() {
        let bytes = minimal_message();
        let m = Grib2Message::parse(&bytes).unwrap();
        assert_eq!(m.indicator.edition, 2);
        assert_eq!(m.indicator.discipline, 0);
        assert_eq!(m.indicator.total_length, bytes.len() as u64);
        let numbers: Vec<u8> = m.sections.iter().map(|s| s.number).collect();
        assert_eq!(numbers, vec![1, 3, 4, 5, 7, 8]);
        assert_eq!(m.sections[0].offset, 16);
        assert_eq!(m.sections[0].length, 21);
        assert_eq!(m.sections[5].offset, bytes.len() - 4);
    }

    #[test]
    fn identification_fields() {
        let bytes = minimal_message();
        let m = Grib2Message::parse(&bytes).unwrap();
        let id = m.identification.as_ref().unwrap();
        assert_eq!(id.centre, 7);
        assert_eq!(id.subcentre, 0);
        assert_eq!(id.tables_version, 2);
        assert_eq!(id.local_tables_version, 1);
        assert_eq!(id.ref_time_significance, 1);
        assert_eq!(id.year, 2024);
        assert_eq!(id.month, 1);
        assert_eq!(id.day, 2);
        assert_eq!(id.hour, 3);
        assert_eq!(id.minute, 4);
        assert_eq!(id.second, 5);
        assert_eq!(id.production_status, 0);
        assert_eq!(id.processed_data_type, 1);
    }

    #[test]
    fn grid_and_product_fields() {
        let bytes = minimal_message();
        let m = Grib2Message::parse(&bytes).unwrap();
        let g = m.grid.as_ref().unwrap();
        assert_eq!(g.source, 0);
        assert_eq!(g.num_points, 100);
        assert_eq!(g.list_octets, 0);
        assert_eq!(g.list_interpretation, 0);
        assert_eq!(g.template_number, 0);
        assert_eq!(g.template, vec![1, 2, 3, 4]);
        let p = m.product.as_ref().unwrap();
        assert_eq!(p.num_coord_values, 0);
        assert_eq!(p.template_number, 0);
        assert_eq!(p.parameter_category, Some(3));
        assert_eq!(p.parameter_number, Some(1));
        let r = m.representation.as_ref().unwrap();
        assert_eq!(r.num_points, 100);
        assert_eq!(r.packing_name(), Some("simple packing"));
        assert_eq!(m.data_length, Some(9));
        assert!(m.bitmap.is_none());
    }

    #[test]
    fn packing_names_map() {
        let rep = |template_number| Grib2DataRepresentation {
            num_points: 0,
            template_number,
            template: Vec::new(),
        };
        assert_eq!(rep(2).packing_name(), Some("complex packing"));
        assert_eq!(
            rep(3).packing_name(),
            Some("complex packing & spatial differencing")
        );
        assert_eq!(rep(40000).packing_name(), Some("JPEG2000"));
        assert_eq!(rep(40010).packing_name(), Some("PNG"));
        assert_eq!(rep(40020).packing_name(), Some("CCITT-G4"));
        assert_eq!(rep(20000).packing_name(), Some("run-length"));
        assert_eq!(rep(7).packing_name(), None);
    }

    #[test]
    fn simple_packing_round_trip() {
        let mut template = Vec::new();
        template.extend_from_slice(&10.0f32.to_be_bytes());
        template.extend_from_slice(&2i16.to_be_bytes());
        template.extend_from_slice(&1i16.to_be_bytes());
        template.push(5);
        template.push(0);
        let mut s5_payload = Vec::new();
        s5_payload.extend_from_slice(&3u32.to_be_bytes());
        s5_payload.extend_from_slice(&0u16.to_be_bytes());
        s5_payload.extend_from_slice(&template);
        let s5 = section(5, &s5_payload);
        let s7 = section(7, &[0x07, 0xCA]);
        let total = 16 + s5.len() + s7.len() + 4;
        let mut b = Vec::new();
        b.extend_from_slice(&MAGIC);
        b.extend_from_slice(&[0, 0, 0, EDITION]);
        b.extend_from_slice(&(total as u64).to_be_bytes());
        b.extend_from_slice(&s5);
        b.extend_from_slice(&s7);
        b.extend_from_slice(&TERMINATOR);
        let m = Grib2Message::parse(&b).unwrap();
        let values = m.values.as_ref().unwrap();
        assert_eq!(values.len(), 3);
        for (got, want) in values.iter().zip([1.0f32, 13.4, 3.0]) {
            assert!((got - want).abs() < 1e-5, "got {}, want {}", got, want);
        }
    }

    #[test]
    fn complex_packing_two_groups() {
        let mut template = Vec::new();
        template.extend_from_slice(&0.0f32.to_be_bytes());
        template.extend_from_slice(&2i16.to_be_bytes());
        template.extend_from_slice(&0i16.to_be_bytes());
        template.push(5);
        template.push(0);
        template.push(1);
        template.push(0);
        template.extend_from_slice(&0u32.to_be_bytes());
        template.extend_from_slice(&0u32.to_be_bytes());
        template.extend_from_slice(&2u32.to_be_bytes());
        template.push(0);
        template.push(2);
        template.extend_from_slice(&0u32.to_be_bytes());
        template.push(1);
        template.extend_from_slice(&2u32.to_be_bytes());
        template.push(2);
        let mut s5_payload = Vec::new();
        s5_payload.extend_from_slice(&5u32.to_be_bytes());
        s5_payload.extend_from_slice(&2u16.to_be_bytes());
        s5_payload.extend_from_slice(&template);
        let s5 = section(5, &s5_payload);
        let s7 = section(7, &[0x06, 0x6B, 0x86, 0x20]);
        let total = 16 + s5.len() + s7.len() + 4;
        let mut b = Vec::new();
        b.extend_from_slice(&MAGIC);
        b.extend_from_slice(&[0, 0, 0, EDITION]);
        b.extend_from_slice(&(total as u64).to_be_bytes());
        b.extend_from_slice(&s5);
        b.extend_from_slice(&s7);
        b.extend_from_slice(&TERMINATOR);
        let m = Grib2Message::parse(&b).unwrap();
        let values = m.values.as_ref().unwrap();
        assert_eq!(values.len(), 5);
        for (got, want) in values.iter().zip([0.0f32, 4.0, 8.0, 100.0, 108.0]) {
            assert!((got - want).abs() < 1e-5, "got {}, want {}", got, want);
        }
    }

    #[test]
    fn complex_spatial_packing_order_one() {
        let mut template = Vec::new();
        template.extend_from_slice(&0.0f32.to_be_bytes());
        template.extend_from_slice(&0i16.to_be_bytes());
        template.extend_from_slice(&0i16.to_be_bytes());
        template.push(1);
        template.push(0);
        template.push(1);
        template.push(0);
        template.extend_from_slice(&0u32.to_be_bytes());
        template.extend_from_slice(&0u32.to_be_bytes());
        template.extend_from_slice(&1u32.to_be_bytes());
        template.push(2);
        template.push(0);
        template.extend_from_slice(&4u32.to_be_bytes());
        template.push(1);
        template.extend_from_slice(&4u32.to_be_bytes());
        template.push(0);
        template.push(1);
        template.push(1);
        let mut s5_payload = Vec::new();
        s5_payload.extend_from_slice(&4u32.to_be_bytes());
        s5_payload.extend_from_slice(&3u16.to_be_bytes());
        s5_payload.extend_from_slice(&template);
        let s5 = section(5, &s5_payload);
        let s7 = section(7, &[0x0A, 0x15, 0x00]);
        let total = 16 + s5.len() + s7.len() + 4;
        let mut b = Vec::new();
        b.extend_from_slice(&MAGIC);
        b.extend_from_slice(&[0, 0, 0, EDITION]);
        b.extend_from_slice(&(total as u64).to_be_bytes());
        b.extend_from_slice(&s5);
        b.extend_from_slice(&s7);
        b.extend_from_slice(&TERMINATOR);
        let m = Grib2Message::parse(&b).unwrap();
        let values = m.values.as_ref().unwrap();
        assert_eq!(values.len(), 4);
        for (got, want) in values.iter().zip([10.0f32, 12.0, 14.0, 16.0]) {
            assert!((got - want).abs() < 1e-5, "got {}, want {}", got, want);
        }
    }

    #[test]
    fn bitmap_section_visible() {
        let s6 = section(6, &[255]);
        let s1 = section(1, &identification_payload());
        let s7 = section(7, &[0u8; 4]);
        let total = 16 + s1.len() + s6.len() + s7.len() + 4;
        let mut b = Vec::new();
        b.extend_from_slice(&MAGIC);
        b.extend_from_slice(&[0, 0, 0, EDITION]);
        b.extend_from_slice(&(total as u64).to_be_bytes());
        b.extend_from_slice(&s1);
        b.extend_from_slice(&s6);
        b.extend_from_slice(&s7);
        b.extend_from_slice(&TERMINATOR);
        let m = Grib2Message::parse(&b).unwrap();
        assert_eq!(m.bitmap.unwrap().indicator, 255);
    }

    #[test]
    fn parse_all_reads_one_message() {
        let bytes = minimal_message();
        let ms = Grib2Message::parse_all(&bytes).unwrap();
        assert_eq!(ms.len(), 1);
        assert_eq!(ms[0].sections.len(), 6);
    }

    #[test]
    fn wrong_magic() {
        let mut bytes = minimal_message();
        bytes[0] = b'X';
        assert!(matches!(
            Grib2Message::parse(&bytes),
            Err(Grib2Note::Magic { .. })
        ));
    }

    #[test]
    fn wrong_edition() {
        let mut bytes = minimal_message();
        bytes[7] = 1;
        assert!(matches!(
            Grib2Message::parse(&bytes),
            Err(Grib2Note::Edition { edition: 1 })
        ));
    }

    #[test]
    fn truncated_indicator() {
        let bytes = b"GRIB";
        assert!(matches!(
            Grib2Message::parse(bytes),
            Err(Grib2Note::EndAtByte { .. })
        ));
    }

    #[test]
    fn truncated_message() {
        let mut bytes = minimal_message();
        bytes.truncate(bytes.len() - 4);
        assert!(matches!(
            Grib2Message::parse(&bytes),
            Err(Grib2Note::Length { .. })
        ));
    }

    #[test]
    fn section_overruns_message() {
        let mut bytes = minimal_message();
        let total = bytes.len() as u32;
        bytes[16..20].copy_from_slice(&total.to_be_bytes());
        assert!(matches!(
            Grib2Message::parse(&bytes),
            Err(Grib2Note::SectionLength { .. })
        ));
    }

    #[test]
    fn missing_terminator() {
        let mut bytes = minimal_message();
        let n = bytes.len();
        bytes[n - 4..].copy_from_slice(b"8888");
        assert!(matches!(
            Grib2Message::parse(&bytes),
            Err(Grib2Note::Terminator { .. })
        ));
    }

    #[test]
    fn ccitt_g4_one_line() {
        let rep = Grib2DataRepresentation {
            num_points: 0,
            template_number: 42,
            template: Vec::new(),
        };
        let values = decode_ccitt_g4(&rep, &[0x70, 0x70, 0x01]).unwrap();
        assert_eq!(values, vec![2.0f32, 14.0]);
    }
}
