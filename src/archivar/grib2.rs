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
                7 => data_length = Some(length),
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
}
