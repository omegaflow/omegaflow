const MAGIC: [u8; 4] = *b"PAR1";

const CT_STOP: u8 = 0;
const CT_BOOL_TRUE: u8 = 1;
const CT_BOOL_FALSE: u8 = 2;
const CT_BYTE: u8 = 3;
const CT_I16: u8 = 4;
const CT_I32: u8 = 5;
const CT_I64: u8 = 6;
const CT_DOUBLE: u8 = 7;
const CT_BINARY: u8 = 8;
const CT_LIST: u8 = 9;
const CT_SET: u8 = 10;
const CT_MAP: u8 = 11;
const CT_STRUCT: u8 = 12;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParquetNote {
    Magic { bytes: [u8; 4] },
    FooterLength { len: u64, file: usize },
    Truncated { off: usize },
    Type { tag: u8, off: usize },
    FieldId { off: usize },
    AbsentField { id: i16 },
}

pub fn parquet_type_name(tag: i32) -> Option<&'static str> {
    match tag {
        0 => Some("BOOLEAN"),
        1 => Some("INT32"),
        2 => Some("INT64"),
        3 => Some("INT96"),
        4 => Some("FLOAT"),
        5 => Some("DOUBLE"),
        6 => Some("BYTE_ARRAY"),
        7 => Some("FIXED_LEN_BYTE_ARRAY"),
        _ => None,
    }
}

pub fn repetition_name(tag: i32) -> Option<&'static str> {
    match tag {
        0 => Some("REQUIRED"),
        1 => Some("OPTIONAL"),
        2 => Some("REPEATED"),
        _ => None,
    }
}

pub fn converted_type_name(tag: i32) -> Option<&'static str> {
    match tag {
        0 => Some("UTF8"),
        1 => Some("MAP"),
        2 => Some("MAP_KEY_VALUE"),
        3 => Some("LIST"),
        4 => Some("ENUM"),
        5 => Some("DECIMAL"),
        6 => Some("DATE"),
        7 => Some("TIME_MILLIS"),
        8 => Some("TIME_MICROS"),
        9 => Some("TIMESTAMP_MILLIS"),
        10 => Some("TIMESTAMP_MICROS"),
        11 => Some("UINT_8"),
        12 => Some("UINT_16"),
        13 => Some("UINT_32"),
        14 => Some("UINT_64"),
        15 => Some("INT_8"),
        16 => Some("INT_16"),
        17 => Some("INT_32"),
        18 => Some("INT_64"),
        19 => Some("JSON"),
        20 => Some("BSON"),
        21 => Some("INTERVAL"),
        _ => None,
    }
}

pub fn codec_name(tag: i32) -> Option<&'static str> {
    match tag {
        0 => Some("UNCOMPRESSED"),
        1 => Some("SNAPPY"),
        2 => Some("GZIP"),
        3 => Some("LZO"),
        4 => Some("BROTLI"),
        5 => Some("LZ4"),
        6 => Some("ZSTD"),
        7 => Some("LZ4_RAW"),
        _ => None,
    }
}

pub fn encoding_name(tag: i32) -> Option<&'static str> {
    match tag {
        0 => Some("PLAIN"),
        1 => Some("GROUP_VAR_INT"),
        2 => Some("PLAIN_DICTIONARY"),
        3 => Some("RLE"),
        4 => Some("BIT_PACKED"),
        5 => Some("DELTA_BINARY_PACKED"),
        6 => Some("DELTA_LENGTH_BYTE_ARRAY"),
        7 => Some("DELTA_BYTE_ARRAY"),
        8 => Some("RLE_DICTIONARY"),
        9 => Some("BYTE_STREAM_SPLIT"),
        _ => None,
    }
}

#[derive(Clone, Debug)]
pub struct SchemaElement {
    pub name: String,
    pub type_tag: Option<i32>,
    pub type_length: Option<i32>,
    pub repetition_type: Option<i32>,
    pub num_children: Option<i32>,
    pub converted_type: Option<i32>,
}

#[derive(Clone, Debug)]
pub struct ColumnMetaData {
    pub type_tag: i32,
    pub encodings: Vec<i32>,
    pub path_in_schema: Vec<String>,
    pub codec: i32,
    pub num_values: i64,
    pub total_uncompressed_size: i64,
    pub total_compressed_size: i64,
    pub data_page_offset: i64,
    pub dictionary_page_offset: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct ColumnChunk {
    pub file_path: Option<String>,
    pub file_offset: i64,
    pub meta_data: Option<ColumnMetaData>,
}

#[derive(Clone, Debug)]
pub struct RowGroup {
    pub columns: Vec<ColumnChunk>,
    pub total_byte_size: i64,
    pub num_rows: i64,
}

#[derive(Clone, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug)]
pub struct FileMetaData {
    pub version: i32,
    pub schema: Vec<SchemaElement>,
    pub num_rows: i64,
    pub row_groups: Vec<RowGroup>,
    pub key_value_metadata: Option<Vec<KeyValue>>,
    pub created_by: Option<String>,
}

struct Compact<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Compact<'a> {
    fn new(buf: &'a [u8]) -> Compact<'a> {
        Compact { buf, pos: 0 }
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], ParquetNote> {
        if self.pos + n <= self.buf.len() {
            let s = &self.buf[self.pos..self.pos + n];
            self.pos += n;
            Ok(s)
        } else {
            Err(ParquetNote::Truncated { off: self.pos })
        }
    }

    fn byte(&mut self) -> Result<u8, ParquetNote> {
        Ok(self.take(1)?[0])
    }

    fn uvarint(&mut self) -> Result<u64, ParquetNote> {
        let mut result = 0u64;
        let mut shift = 0u32;
        loop {
            let b = self.byte()?;
            result |= ((b & 0x7f) as u64) << shift;
            if b & 0x80 == 0 {
                return Ok(result);
            }
            shift += 7;
            if shift >= 64 {
                return Err(ParquetNote::Truncated { off: self.pos });
            }
        }
    }

    fn zigzag(&mut self) -> Result<i64, ParquetNote> {
        let n = self.uvarint()?;
        Ok(((n >> 1) as i64) ^ -((n & 1) as i64))
    }

    fn i16(&mut self) -> Result<i16, ParquetNote> {
        Ok(self.zigzag()? as i16)
    }

    fn i32(&mut self) -> Result<i32, ParquetNote> {
        Ok(self.zigzag()? as i32)
    }

    fn i64(&mut self) -> Result<i64, ParquetNote> {
        self.zigzag()
    }

    fn string(&mut self) -> Result<String, ParquetNote> {
        let n = self.uvarint()? as usize;
        let b = self.take(n)?;
        Ok(String::from_utf8_lossy(b).into_owned())
    }

    fn field(&mut self, last: i16) -> Result<(u8, i16), ParquetNote> {
        let off = self.pos;
        let b = self.byte()?;
        let ctype = b & 0x0f;
        if ctype == CT_STOP {
            return Ok((CT_STOP, 0));
        }
        let delta = (b >> 4) as i16;
        let id = if delta == 0 {
            self.i16()?
        } else {
            last.checked_add(delta)
                .ok_or(ParquetNote::FieldId { off })?
        };
        Ok((ctype, id))
    }

    fn list_header(&mut self) -> Result<(u8, usize), ParquetNote> {
        let b = self.byte()?;
        let etype = b & 0x0f;
        let mut size = (b >> 4) as usize;
        if size == 15 {
            size = self.uvarint()? as usize;
        }
        Ok((etype, size))
    }

    fn skip(&mut self, ctype: u8) -> Result<(), ParquetNote> {
        match ctype {
            CT_BOOL_TRUE | CT_BOOL_FALSE => Ok(()),
            CT_BYTE => {
                self.take(1)?;
                Ok(())
            }
            CT_I16 | CT_I32 | CT_I64 => {
                self.uvarint()?;
                Ok(())
            }
            CT_DOUBLE => {
                self.take(8)?;
                Ok(())
            }
            CT_BINARY => {
                let n = self.uvarint()? as usize;
                self.take(n)?;
                Ok(())
            }
            CT_LIST | CT_SET => self.skip_list(),
            CT_MAP => self.skip_map(),
            CT_STRUCT => self.skip_struct(),
            tag => Err(ParquetNote::Type { tag, off: self.pos }),
        }
    }

    fn skip_elem(&mut self, ctype: u8) -> Result<(), ParquetNote> {
        if ctype == CT_BOOL_TRUE || ctype == CT_BOOL_FALSE {
            self.take(1)?;
            Ok(())
        } else {
            self.skip(ctype)
        }
    }

    fn skip_list(&mut self) -> Result<(), ParquetNote> {
        let (etype, size) = self.list_header()?;
        for _ in 0..size {
            self.skip_elem(etype)?;
        }
        Ok(())
    }

    fn skip_map(&mut self) -> Result<(), ParquetNote> {
        let size = self.uvarint()?;
        if size == 0 {
            return Ok(());
        }
        let types = self.byte()?;
        let kt = types >> 4;
        let vt = types & 0x0f;
        for _ in 0..size {
            self.skip_elem(kt)?;
            self.skip_elem(vt)?;
        }
        Ok(())
    }

    fn skip_struct(&mut self) -> Result<(), ParquetNote> {
        let mut last = 0i16;
        loop {
            let (ctype, id) = self.field(last)?;
            if ctype == CT_STOP {
                return Ok(());
            }
            last = id;
            self.skip(ctype)?;
        }
    }

    fn struct_list<T>(
        &mut self,
        mut read: impl FnMut(&mut Self) -> Result<T, ParquetNote>,
    ) -> Result<Vec<T>, ParquetNote> {
        let (etype, size) = self.list_header()?;
        if etype != CT_STRUCT {
            return Err(ParquetNote::Type {
                tag: etype,
                off: self.pos,
            });
        }
        let mut out = Vec::new();
        for _ in 0..size {
            out.push(read(self)?);
        }
        Ok(out)
    }

    fn i32_list(&mut self) -> Result<Vec<i32>, ParquetNote> {
        let (etype, size) = self.list_header()?;
        if etype != CT_I32 {
            return Err(ParquetNote::Type {
                tag: etype,
                off: self.pos,
            });
        }
        let mut out = Vec::new();
        for _ in 0..size {
            out.push(self.i32()?);
        }
        Ok(out)
    }

    fn string_list(&mut self) -> Result<Vec<String>, ParquetNote> {
        let (etype, size) = self.list_header()?;
        if etype != CT_BINARY {
            return Err(ParquetNote::Type {
                tag: etype,
                off: self.pos,
            });
        }
        let mut out = Vec::new();
        for _ in 0..size {
            out.push(self.string()?);
        }
        Ok(out)
    }

    fn schema_element(&mut self) -> Result<SchemaElement, ParquetNote> {
        let mut name = None;
        let mut type_tag = None;
        let mut type_length = None;
        let mut repetition_type = None;
        let mut num_children = None;
        let mut converted_type = None;
        let mut last = 0i16;
        loop {
            let (ctype, id) = self.field(last)?;
            if ctype == CT_STOP {
                break;
            }
            last = id;
            match id {
                1 => type_tag = Some(self.i32()?),
                2 => type_length = Some(self.i32()?),
                3 => repetition_type = Some(self.i32()?),
                4 => name = Some(self.string()?),
                5 => num_children = Some(self.i32()?),
                6 => converted_type = Some(self.i32()?),
                _ => self.skip(ctype)?,
            }
        }
        Ok(SchemaElement {
            name: name.ok_or(ParquetNote::AbsentField { id: 4 })?,
            type_tag,
            type_length,
            repetition_type,
            num_children,
            converted_type,
        })
    }

    fn column_meta_data(&mut self) -> Result<ColumnMetaData, ParquetNote> {
        let mut type_tag = None;
        let mut encodings = None;
        let mut path = None;
        let mut codec = None;
        let mut num_values = None;
        let mut uncompressed = None;
        let mut compressed = None;
        let mut data_page = None;
        let mut dict_page = None;
        let mut last = 0i16;
        loop {
            let (ctype, id) = self.field(last)?;
            if ctype == CT_STOP {
                break;
            }
            last = id;
            match id {
                1 => type_tag = Some(self.i32()?),
                2 => encodings = Some(self.i32_list()?),
                3 => path = Some(self.string_list()?),
                4 => codec = Some(self.i32()?),
                5 => num_values = Some(self.i64()?),
                6 => uncompressed = Some(self.i64()?),
                7 => compressed = Some(self.i64()?),
                9 => data_page = Some(self.i64()?),
                11 => dict_page = Some(self.i64()?),
                _ => self.skip(ctype)?,
            }
        }
        Ok(ColumnMetaData {
            type_tag: type_tag.ok_or(ParquetNote::AbsentField { id: 1 })?,
            encodings: encodings.ok_or(ParquetNote::AbsentField { id: 2 })?,
            path_in_schema: path.ok_or(ParquetNote::AbsentField { id: 3 })?,
            codec: codec.ok_or(ParquetNote::AbsentField { id: 4 })?,
            num_values: num_values.ok_or(ParquetNote::AbsentField { id: 5 })?,
            total_uncompressed_size: uncompressed.ok_or(ParquetNote::AbsentField { id: 6 })?,
            total_compressed_size: compressed.ok_or(ParquetNote::AbsentField { id: 7 })?,
            data_page_offset: data_page.ok_or(ParquetNote::AbsentField { id: 9 })?,
            dictionary_page_offset: dict_page,
        })
    }

    fn column_chunk(&mut self) -> Result<ColumnChunk, ParquetNote> {
        let mut file_path = None;
        let mut file_offset = None;
        let mut meta_data = None;
        let mut last = 0i16;
        loop {
            let (ctype, id) = self.field(last)?;
            if ctype == CT_STOP {
                break;
            }
            last = id;
            match id {
                1 => file_path = Some(self.string()?),
                2 => file_offset = Some(self.i64()?),
                3 => meta_data = Some(self.column_meta_data()?),
                _ => self.skip(ctype)?,
            }
        }
        Ok(ColumnChunk {
            file_path,
            file_offset: file_offset.ok_or(ParquetNote::AbsentField { id: 2 })?,
            meta_data,
        })
    }

    fn row_group(&mut self) -> Result<RowGroup, ParquetNote> {
        let mut columns = None;
        let mut total_byte_size = None;
        let mut num_rows = None;
        let mut last = 0i16;
        loop {
            let (ctype, id) = self.field(last)?;
            if ctype == CT_STOP {
                break;
            }
            last = id;
            match id {
                1 => columns = Some(self.struct_list(|s| s.column_chunk())?),
                2 => total_byte_size = Some(self.i64()?),
                3 => num_rows = Some(self.i64()?),
                _ => self.skip(ctype)?,
            }
        }
        Ok(RowGroup {
            columns: columns.ok_or(ParquetNote::AbsentField { id: 1 })?,
            total_byte_size: total_byte_size.ok_or(ParquetNote::AbsentField { id: 2 })?,
            num_rows: num_rows.ok_or(ParquetNote::AbsentField { id: 3 })?,
        })
    }

    fn key_value(&mut self) -> Result<KeyValue, ParquetNote> {
        let mut key = None;
        let mut value = None;
        let mut last = 0i16;
        loop {
            let (ctype, id) = self.field(last)?;
            if ctype == CT_STOP {
                break;
            }
            last = id;
            match id {
                1 => key = Some(self.string()?),
                2 => value = Some(self.string()?),
                _ => self.skip(ctype)?,
            }
        }
        Ok(KeyValue {
            key: key.ok_or(ParquetNote::AbsentField { id: 1 })?,
            value,
        })
    }

    fn file_metadata(&mut self) -> Result<FileMetaData, ParquetNote> {
        let mut version = None;
        let mut schema = None;
        let mut num_rows = None;
        let mut row_groups = None;
        let mut key_value_metadata = None;
        let mut created_by = None;
        let mut last = 0i16;
        loop {
            let (ctype, id) = self.field(last)?;
            if ctype == CT_STOP {
                break;
            }
            last = id;
            match id {
                1 => version = Some(self.i32()?),
                2 => schema = Some(self.struct_list(|s| s.schema_element())?),
                3 => num_rows = Some(self.i64()?),
                4 => row_groups = Some(self.struct_list(|s| s.row_group())?),
                5 => key_value_metadata = Some(self.struct_list(|s| s.key_value())?),
                6 => created_by = Some(self.string()?),
                _ => self.skip(ctype)?,
            }
        }
        Ok(FileMetaData {
            version: version.ok_or(ParquetNote::AbsentField { id: 1 })?,
            schema: schema.ok_or(ParquetNote::AbsentField { id: 2 })?,
            num_rows: num_rows.ok_or(ParquetNote::AbsentField { id: 3 })?,
            row_groups: row_groups.ok_or(ParquetNote::AbsentField { id: 4 })?,
            key_value_metadata,
            created_by,
        })
    }
}

impl FileMetaData {
    pub fn parse(bytes: &[u8]) -> Result<FileMetaData, ParquetNote> {
        if bytes.len() < 12 {
            return Err(ParquetNote::Truncated { off: bytes.len() });
        }
        let head = [bytes[0], bytes[1], bytes[2], bytes[3]];
        if head != MAGIC {
            return Err(ParquetNote::Magic { bytes: head });
        }
        let tail = [
            bytes[bytes.len() - 4],
            bytes[bytes.len() - 3],
            bytes[bytes.len() - 2],
            bytes[bytes.len() - 1],
        ];
        if tail != MAGIC {
            return Err(ParquetNote::Magic { bytes: tail });
        }
        let len_off = bytes.len() - 8;
        let flen = u32::from_le_bytes([
            bytes[len_off],
            bytes[len_off + 1],
            bytes[len_off + 2],
            bytes[len_off + 3],
        ]) as usize;
        if flen > len_off {
            return Err(ParquetNote::FooterLength {
                len: flen as u64,
                file: bytes.len(),
            });
        }
        let start = len_off - flen;
        let mut cur = Compact::new(&bytes[start..len_off]);
        cur.file_metadata()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParquetValue {
    Bool(bool),
    I32(i32),
    I64(i64),
    Float(f32),
    Double(f64),
    Bytes(Vec<u8>),
    Unhandled(String),
}

#[derive(Clone, Debug)]
pub struct ParquetColumn {
    pub name: String,
    pub values: Vec<ParquetValue>,
}

struct PageHeaderInfo {
    page_type: i32,
    compressed_page_size: i32,
    num_values: Option<i32>,
    data_encoding: Option<i32>,
    dict_num_values: Option<i32>,
    data_page_v2: bool,
}

impl<'a> Compact<'a> {
    fn page_header(&mut self) -> Result<PageHeaderInfo, ParquetNote> {
        let mut page_type = None;
        let mut compressed = None;
        let mut num_values = None;
        let mut data_encoding = None;
        let mut dict_num_values = None;
        let mut data_page_v2 = false;
        let mut last = 0i16;
        loop {
            let (ctype, id) = self.field(last)?;
            if ctype == CT_STOP {
                break;
            }
            last = id;
            match id {
                1 => page_type = Some(self.i32()?),
                2 | 4 => self.skip(ctype)?,
                3 => compressed = Some(self.i32()?),
                5 => {
                    let mut l = 0i16;
                    loop {
                        let (ct, i) = self.field(l)?;
                        if ct == CT_STOP {
                            break;
                        }
                        l = i;
                        match i {
                            1 => num_values = Some(self.i32()?),
                            2 => data_encoding = Some(self.i32()?),
                            _ => self.skip(ct)?,
                        }
                    }
                }
                7 => {
                    let mut l = 0i16;
                    loop {
                        let (ct, i) = self.field(l)?;
                        if ct == CT_STOP {
                            break;
                        }
                        l = i;
                        match i {
                            1 => dict_num_values = Some(self.i32()?),
                            2 => {
                                self.i32()?;
                            }
                            _ => self.skip(ct)?,
                        }
                    }
                }
                8 => {
                    data_page_v2 = true;
                    self.skip(ctype)?;
                }
                _ => self.skip(ctype)?,
            }
        }
        Ok(PageHeaderInfo {
            page_type: page_type.ok_or(ParquetNote::AbsentField { id: 1 })?,
            compressed_page_size: compressed.ok_or(ParquetNote::AbsentField { id: 3 })?,
            num_values,
            data_encoding,
            dict_num_values,
            data_page_v2,
        })
    }
}

fn slice_at(data: &[u8], start: usize, len: usize) -> Result<&[u8], ParquetNote> {
    let end = start
        .checked_add(len)
        .ok_or(ParquetNote::Truncated { off: start })?;
    if end > data.len() {
        return Err(ParquetNote::Truncated { off: start });
    }
    Ok(&data[start..end])
}

struct Bits<'a> {
    data: &'a [u8],
    bit_pos: usize,
}

impl<'a> Bits<'a> {
    fn read(&mut self, width: u8) -> Result<u32, ParquetNote> {
        let mut val = 0u64;
        for i in 0..width as usize {
            let bit_index = self.bit_pos + i;
            let byte = self.data[bit_index / 8];
            val |= (((byte >> (bit_index % 8)) & 1) as u64) << i;
        }
        self.bit_pos += width as usize;
        Ok(val as u32)
    }
}

fn uvarint_at(data: &[u8], pos: usize) -> Result<(u64, usize), ParquetNote> {
    let mut result = 0u64;
    let mut shift = 0u32;
    let mut i = pos;
    loop {
        let b = *data.get(i).ok_or(ParquetNote::Truncated { off: i })?;
        result |= ((b & 0x7f) as u64) << shift;
        i += 1;
        if b & 0x80 == 0 {
            return Ok((result, i - pos));
        }
        shift += 7;
        if shift >= 64 {
            return Err(ParquetNote::Truncated { off: i });
        }
    }
}

fn rle_bitpacked(data: &[u8], bit_width: u8, target: usize) -> Result<Vec<u32>, ParquetNote> {
    let mut out = Vec::with_capacity(target);
    let bytes_per_val = (bit_width as usize).div_ceil(8);
    let mut pos = 0usize;
    while out.len() < target {
        let (header, used) = uvarint_at(data, pos)?;
        pos += used;
        let run = (header >> 1) as usize;
        if header & 1 == 1 {
            let bytes_needed = run
                .checked_mul(bit_width as usize)
                .ok_or(ParquetNote::Truncated { off: pos })?;
            let body = slice_at(data, pos, bytes_needed)?;
            let mut bits = Bits {
                data: body,
                bit_pos: 0,
            };
            for _ in 0..(run * 8) {
                if out.len() >= target {
                    break;
                }
                out.push(bits.read(bit_width)?);
            }
            pos += bytes_needed;
        } else {
            let body = slice_at(data, pos, bytes_per_val)?;
            let mut val = 0u32;
            for (k, byte) in body.iter().enumerate() {
                val |= (*byte as u32) << (8 * k);
            }
            pos += bytes_per_val;
            for _ in 0..run {
                if out.len() >= target {
                    break;
                }
                out.push(val);
            }
        }
    }
    Ok(out)
}

fn decode_plain(
    data: &[u8],
    type_tag: i32,
    type_length: Option<i32>,
    count: usize,
) -> Result<Vec<ParquetValue>, ParquetNote> {
    let mut out = Vec::with_capacity(count);
    match type_tag {
        0 => {
            let body = slice_at(data, 0, count.div_ceil(8))?;
            for i in 0..count {
                out.push(ParquetValue::Bool((body[i / 8] >> (i % 8)) & 1 == 1));
            }
        }
        1 => {
            let body = slice_at(data, 0, count * 4)?;
            for i in 0..count {
                let o = i * 4;
                out.push(ParquetValue::I32(i32::from_le_bytes([
                    body[o],
                    body[o + 1],
                    body[o + 2],
                    body[o + 3],
                ])));
            }
        }
        2 => {
            let body = slice_at(data, 0, count * 8)?;
            for i in 0..count {
                let o = i * 8;
                out.push(ParquetValue::I64(i64::from_le_bytes([
                    body[o],
                    body[o + 1],
                    body[o + 2],
                    body[o + 3],
                    body[o + 4],
                    body[o + 5],
                    body[o + 6],
                    body[o + 7],
                ])));
            }
        }
        4 => {
            let body = slice_at(data, 0, count * 4)?;
            for i in 0..count {
                let o = i * 4;
                out.push(ParquetValue::Float(f32::from_le_bytes([
                    body[o],
                    body[o + 1],
                    body[o + 2],
                    body[o + 3],
                ])));
            }
        }
        5 => {
            let body = slice_at(data, 0, count * 8)?;
            for i in 0..count {
                let o = i * 8;
                out.push(ParquetValue::Double(f64::from_le_bytes([
                    body[o],
                    body[o + 1],
                    body[o + 2],
                    body[o + 3],
                    body[o + 4],
                    body[o + 5],
                    body[o + 6],
                    body[o + 7],
                ])));
            }
        }
        6 => {
            let mut pos = 0usize;
            for _ in 0..count {
                let lb = slice_at(data, pos, 4)?;
                let len = u32::from_le_bytes([lb[0], lb[1], lb[2], lb[3]]) as usize;
                pos += 4;
                let body = slice_at(data, pos, len)?;
                out.push(ParquetValue::Bytes(body.to_vec()));
                pos += len;
            }
        }
        7 => {
            let len = type_length.ok_or(ParquetNote::AbsentField { id: 2 })? as usize;
            let body = slice_at(data, 0, count * len)?;
            for i in 0..count {
                out.push(ParquetValue::Bytes(body[i * len..(i + 1) * len].to_vec()));
            }
        }
        tag => {
            return Ok(vec![ParquetValue::Unhandled(format!("type {}", tag))]);
        }
    }
    Ok(out)
}

fn decode_dict(
    data: &[u8],
    dictionary: &[ParquetValue],
    count: usize,
) -> Result<Vec<ParquetValue>, ParquetNote> {
    if data.is_empty() {
        return Err(ParquetNote::Truncated { off: 0 });
    }
    let bit_width = data[0];
    let mut out = Vec::with_capacity(count);
    if bit_width == 0 {
        for _ in 0..count {
            match dictionary.first() {
                Some(v) => out.push(v.clone()),
                None => out.push(ParquetValue::Unhandled("dict index 0".to_string())),
            }
        }
        return Ok(out);
    }
    let indices = rle_bitpacked(&data[1..], bit_width, count)?;
    for idx in indices {
        match dictionary.get(idx as usize) {
            Some(v) => out.push(v.clone()),
            None => out.push(ParquetValue::Unhandled(format!("dict index {}", idx))),
        }
    }
    Ok(out)
}

fn read_page_header(data: &[u8], off: usize) -> Result<(PageHeaderInfo, usize), ParquetNote> {
    if off >= data.len() {
        return Err(ParquetNote::Truncated { off });
    }
    let mut cur = Compact::new(&data[off..]);
    let hdr = cur.page_header()?;
    Ok((hdr, cur.pos))
}

fn skip_levels(body: &[u8], max_rep: usize, max_def: usize) -> Result<&[u8], ParquetNote> {
    let mut pos = 0usize;
    if max_rep > 0 {
        let lb = slice_at(body, pos, 4)?;
        let len = u32::from_le_bytes([lb[0], lb[1], lb[2], lb[3]]) as usize;
        pos += 4 + len;
    }
    if max_def > 0 {
        let lb = slice_at(body, pos, 4)?;
        let len = u32::from_le_bytes([lb[0], lb[1], lb[2], lb[3]]) as usize;
        pos += 4 + len;
    }
    Ok(&body[pos..])
}

fn decode_column(
    cm: &ColumnMetaData,
    leaf: &SchemaElement,
    data: &[u8],
) -> Result<Vec<ParquetValue>, ParquetNote> {
    if cm.codec != 0 {
        return Ok(vec![ParquetValue::Unhandled(format!("codec {}", cm.codec))]);
    }
    let type_tag = leaf.type_tag.ok_or(ParquetNote::AbsentField { id: 1 })?;
    let type_length = leaf.type_length;
    let (max_rep, max_def) = match leaf.repetition_type {
        Some(0) => (0usize, 0usize),
        Some(1) => (0usize, 1usize),
        Some(2) => (1usize, 1usize),
        _ => (0usize, 0usize),
    };
    let mut dictionary: Vec<ParquetValue> = Vec::new();
    if let Some(dict_off) = cm.dictionary_page_offset {
        let (hdr, hlen) = read_page_header(data, dict_off as usize)?;
        let n = hdr
            .dict_num_values
            .ok_or(ParquetNote::AbsentField { id: 1 })? as usize;
        let body = slice_at(
            data,
            dict_off as usize + hlen,
            hdr.compressed_page_size as usize,
        )?;
        dictionary = decode_plain(body, type_tag, type_length, n)?;
    }
    let mut out = Vec::new();
    let mut off = cm.data_page_offset as usize;
    let mut remaining = cm.num_values;
    while remaining > 0 {
        let (hdr, hlen) = read_page_header(data, off)?;
        let body = slice_at(data, off + hlen, hdr.compressed_page_size as usize)?;
        if hdr.data_page_v2 {
            out.push(ParquetValue::Unhandled("data_page_v2".to_string()));
            break;
        }
        if hdr.page_type != 0 {
            out.push(ParquetValue::Unhandled(format!(
                "page_type {}",
                hdr.page_type
            )));
            break;
        }
        let n = hdr.num_values.ok_or(ParquetNote::AbsentField { id: 1 })? as usize;
        let enc = hdr
            .data_encoding
            .ok_or(ParquetNote::AbsentField { id: 2 })?;
        let values_body = skip_levels(body, max_rep, max_def)?;
        let vals = match enc {
            0 => decode_plain(values_body, type_tag, type_length, n)?,
            2 | 8 => decode_dict(values_body, &dictionary, n)?,
            e => vec![ParquetValue::Unhandled(format!("encoding {}", e))],
        };
        out.extend(vals);
        remaining -= n as i64;
        off += hlen + hdr.compressed_page_size as usize;
    }
    Ok(out)
}

fn leaf_columns(schema: &[SchemaElement]) -> Vec<SchemaElement> {
    schema
        .iter()
        .filter(|e| e.num_children.is_none() && e.type_tag.is_some())
        .cloned()
        .collect()
}

pub fn parse_parquet(data: &[u8]) -> Option<Vec<ParquetColumn>> {
    let meta = FileMetaData::parse(data).ok()?;
    let leaves = leaf_columns(&meta.schema);
    let mut columns: Vec<ParquetColumn> = leaves
        .iter()
        .map(|l| ParquetColumn {
            name: l.name.clone(),
            values: Vec::new(),
        })
        .collect();
    for rg in &meta.row_groups {
        for (i, chunk) in rg.columns.iter().enumerate() {
            let leaf = leaves.get(i)?;
            let cm = match &chunk.meta_data {
                Some(cm) => cm,
                None => {
                    columns[i]
                        .values
                        .push(ParquetValue::Unhandled("meta_data absent".to_string()));
                    continue;
                }
            };
            columns[i].name = if cm.path_in_schema.is_empty() {
                leaf.name.clone()
            } else {
                cm.path_in_schema.join(".")
            };
            let values = decode_column(cm, leaf, data).ok()?;
            columns[i].values.extend(values);
        }
    }
    Some(columns)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Enc {
        buf: Vec<u8>,
        last: i16,
    }

    impl Enc {
        fn new() -> Enc {
            Enc {
                buf: Vec::new(),
                last: 0,
            }
        }

        fn field(&mut self, id: i16, ctype: u8) {
            let delta = id - self.last;
            if delta > 0 && delta <= 15 {
                self.buf.push(((delta as u8) << 4) | ctype);
            } else {
                self.buf.push(ctype);
                self.buf.extend(zigzag(id as i64));
            }
            self.last = id;
        }

        fn i32(&mut self, id: i16, v: i32) {
            self.field(id, CT_I32);
            self.buf.extend(zigzag(v as i64));
        }

        fn i64(&mut self, id: i16, v: i64) {
            self.field(id, CT_I64);
            self.buf.extend(zigzag(v));
        }

        fn binary(&mut self, id: i16, s: &[u8]) {
            self.field(id, CT_BINARY);
            self.buf.extend(uvarint(s.len() as u64));
            self.buf.extend(s);
        }

        fn list_struct(&mut self, id: i16, items: &[Vec<u8>]) {
            self.field(id, CT_LIST);
            self.buf.push(((items.len() as u8) << 4) | CT_STRUCT);
            for it in items {
                self.buf.extend(it.iter().copied());
            }
        }

        fn stop(&mut self) {
            self.buf.push(CT_STOP);
        }
    }

    fn uvarint(mut n: u64) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            let b = (n & 0x7f) as u8;
            n >>= 7;
            if n == 0 {
                out.push(b);
                break;
            }
            out.push(b | 0x80);
        }
        out
    }

    fn zigzag(n: i64) -> Vec<u8> {
        uvarint(((n << 1) ^ (n >> 63)) as u64)
    }

    fn schema_root() -> Vec<u8> {
        let mut e = Enc::new();
        e.binary(4, b"schema");
        e.i32(5, 1);
        e.stop();
        e.buf
    }

    fn schema_leaf() -> Vec<u8> {
        let mut e = Enc::new();
        e.i32(1, 0);
        e.i32(3, 0);
        e.binary(4, b"col");
        e.stop();
        e.buf
    }

    fn column_meta() -> Vec<u8> {
        let mut e = Enc::new();
        e.i32(1, 0);
        e.field(2, CT_LIST);
        e.buf.push((1 << 4) | CT_I32);
        e.buf.extend(zigzag(0));
        e.field(3, CT_LIST);
        e.buf.push((1 << 4) | CT_BINARY);
        e.buf.extend(uvarint(3));
        e.buf.extend(b"col");
        e.i32(4, 0);
        e.i64(5, 3);
        e.i64(6, 100);
        e.i64(7, 80);
        e.i64(9, 4);
        e.i64(11, 8);
        e.stop();
        e.buf
    }

    fn column_chunk() -> Vec<u8> {
        let mut e = Enc::new();
        e.i64(2, 1000);
        e.field(3, CT_STRUCT);
        e.buf.extend(column_meta());
        e.stop();
        e.buf
    }

    fn row_group() -> Vec<u8> {
        let mut e = Enc::new();
        e.list_struct(1, &[column_chunk()]);
        e.i64(2, 1000);
        e.i64(3, 3);
        e.stop();
        e.buf
    }

    fn key_value() -> Vec<u8> {
        let mut e = Enc::new();
        e.binary(1, b"key");
        e.binary(2, b"value");
        e.stop();
        e.buf
    }

    fn unknown_struct() -> Vec<u8> {
        let mut e = Enc::new();
        e.i32(1, 42);
        e.binary(2, b"x");
        e.field(3, CT_DOUBLE);
        e.buf.extend(1.5f64.to_le_bytes());
        e.field(4, CT_LIST);
        e.buf.push((2 << 4) | CT_I64);
        e.buf.extend(zigzag(7));
        e.buf.extend(zigzag(9));
        e.stop();
        e.buf
    }

    fn metadata(extra: bool) -> Vec<u8> {
        let mut e = Enc::new();
        e.i32(1, 1);
        e.list_struct(2, &[schema_root(), schema_leaf()]);
        e.i64(3, 3);
        e.list_struct(4, &[row_group()]);
        e.list_struct(5, &[key_value()]);
        e.binary(6, b"test-writer");
        if extra {
            e.field(7, CT_STRUCT);
            e.buf.extend(unknown_struct());
            e.field(8, CT_MAP);
            e.buf.extend(uvarint(1));
            e.buf.push((CT_I32 << 4) | CT_BINARY);
            e.buf.extend(zigzag(3));
            e.buf.extend(uvarint(1));
            e.buf.extend(b"m");
        }
        e.stop();
        e.buf
    }

    fn wrap(footer: &[u8]) -> Vec<u8> {
        let mut f = Vec::new();
        f.extend(b"PAR1");
        f.extend_from_slice(footer);
        f.extend((footer.len() as u32).to_le_bytes());
        f.extend(b"PAR1");
        f
    }

    #[test]
    fn minimal_file_metadata_reads() {
        let bytes = wrap(&metadata(false));
        let m = FileMetaData::parse(&bytes).expect("parse");
        assert_eq!(m.version, 1);
        assert_eq!(m.num_rows, 3);
        assert_eq!(m.created_by.as_deref(), Some("test-writer"));
        assert_eq!(m.schema.len(), 2);
        assert_eq!(m.schema[0].name, "schema");
        assert_eq!(m.schema[0].num_children, Some(1));
        assert_eq!(m.schema[1].name, "col");
        assert_eq!(m.schema[1].type_tag, Some(0));
        assert_eq!(parquet_type_name(0), Some("BOOLEAN"));
        assert_eq!(m.schema[1].repetition_type, Some(0));
        assert_eq!(repetition_name(0), Some("REQUIRED"));
        assert_eq!(m.row_groups.len(), 1);
        assert_eq!(m.row_groups[0].total_byte_size, 1000);
        assert_eq!(m.row_groups[0].num_rows, 3);
        assert_eq!(m.row_groups[0].columns.len(), 1);
        let c = &m.row_groups[0].columns[0];
        assert_eq!(c.file_offset, 1000);
        assert!(c.file_path.is_none());
        let meta = c.meta_data.as_ref().expect("meta_data");
        assert_eq!(meta.type_tag, 0);
        assert_eq!(meta.encodings, vec![0]);
        assert_eq!(meta.path_in_schema, vec!["col".to_string()]);
        assert_eq!(meta.codec, 0);
        assert_eq!(codec_name(0), Some("UNCOMPRESSED"));
        assert_eq!(meta.num_values, 3);
        assert_eq!(meta.total_uncompressed_size, 100);
        assert_eq!(meta.total_compressed_size, 80);
        assert_eq!(meta.data_page_offset, 4);
        assert_eq!(meta.dictionary_page_offset, Some(8));
        let kv = m.key_value_metadata.as_ref().expect("kv");
        assert_eq!(kv.len(), 1);
        assert_eq!(kv[0].key, "key");
        assert_eq!(kv[0].value.as_deref(), Some("value"));
    }

    #[test]
    fn unknown_fields_are_consumed() {
        let bytes = wrap(&metadata(true));
        let m = FileMetaData::parse(&bytes).expect("parse");
        assert_eq!(m.version, 1);
        assert_eq!(m.created_by.as_deref(), Some("test-writer"));
        assert_eq!(m.row_groups.len(), 1);
        assert_eq!(m.key_value_metadata.as_ref().map(Vec::len), Some(1));
    }

    #[test]
    fn not_parquet() {
        let bytes = b"XXXX00000000PAR1";
        assert!(matches!(
            FileMetaData::parse(bytes),
            Err(ParquetNote::Magic {
                bytes: [b'X', b'X', b'X', b'X']
            })
        ));
    }

    #[test]
    fn truncated_file() {
        assert!(matches!(
            FileMetaData::parse(b"PAR1"),
            Err(ParquetNote::Truncated { .. })
        ));
    }

    #[test]
    fn footer_length_past_end() {
        let mut f = Vec::new();
        f.extend(b"PAR1");
        f.push(0);
        f.extend(1000u32.to_le_bytes());
        f.extend(b"PAR1");
        assert!(matches!(
            FileMetaData::parse(&f),
            Err(ParquetNote::FooterLength { .. })
        ));
    }

    #[test]
    fn truncated_footer_body() {
        let bytes = wrap(&[0x15, 0x02]);
        assert!(matches!(
            FileMetaData::parse(&bytes),
            Err(ParquetNote::Truncated { .. })
        ));
    }

    #[test]
    fn absent_required_field() {
        let bytes = wrap(&[CT_STOP]);
        assert!(matches!(
            FileMetaData::parse(&bytes),
            Err(ParquetNote::AbsentField { id: 1 })
        ));
    }

    fn file_metadata_footer(schema: &[Vec<u8>], row_groups: &[Vec<u8>]) -> Vec<u8> {
        let mut fm = Enc::new();
        fm.i32(1, 1);
        fm.list_struct(2, schema);
        fm.i64(3, 3);
        fm.list_struct(4, row_groups);
        fm.binary(6, b"omegaflow-test");
        fm.stop();
        fm.buf
    }

    fn schema_root_bytes() -> Vec<u8> {
        let mut e = Enc::new();
        e.binary(4, b"schema");
        e.i32(5, 1);
        e.stop();
        e.buf
    }

    fn plain_int32_file() -> Vec<u8> {
        let mut ph = Enc::new();
        ph.i32(1, 0);
        ph.i32(2, 12);
        ph.i32(3, 12);
        ph.field(5, CT_STRUCT);
        {
            let mut d = Enc::new();
            d.i32(1, 3);
            d.i32(2, 0);
            d.i32(3, 3);
            d.i32(4, 3);
            d.stop();
            ph.buf.extend(d.buf);
        }
        ph.stop();
        let page_header = ph.buf;

        let mut body = Vec::new();
        for v in [1i32, 2, 3] {
            body.extend(v.to_le_bytes());
        }
        let total = (page_header.len() + body.len()) as i64;
        let data_page_offset = 4usize;

        let mut leaf = Enc::new();
        leaf.i32(1, 1);
        leaf.i32(3, 0);
        leaf.binary(4, b"col");
        leaf.stop();

        let mut cm = Enc::new();
        cm.i32(1, 1);
        cm.field(2, CT_LIST);
        cm.buf.push((1 << 4) | CT_I32);
        cm.buf.extend(zigzag(0));
        cm.field(3, CT_LIST);
        cm.buf.push((1 << 4) | CT_BINARY);
        cm.buf.extend(uvarint(3));
        cm.buf.extend(b"col");
        cm.i32(4, 0);
        cm.i64(5, 3);
        cm.i64(6, total);
        cm.i64(7, total);
        cm.i64(9, data_page_offset as i64);
        cm.stop();

        let mut cc = Enc::new();
        cc.i64(2, data_page_offset as i64);
        cc.field(3, CT_STRUCT);
        cc.buf.extend(cm.buf);
        cc.stop();

        let mut rg = Enc::new();
        rg.list_struct(1, &[cc.buf]);
        rg.i64(2, total);
        rg.i64(3, 3);
        rg.stop();

        let footer = file_metadata_footer(&[schema_root_bytes(), leaf.buf], &[rg.buf]);

        let mut file = Vec::new();
        file.extend(b"PAR1");
        file.extend(&page_header);
        file.extend(&body);
        file.extend(&footer);
        file.extend((footer.len() as u32).to_le_bytes());
        file.extend(b"PAR1");
        file
    }

    fn dictionary_byte_array_file(codec: i32) -> Vec<u8> {
        let mut dh = Enc::new();
        dh.i32(1, 2);
        dh.i32(2, 13);
        dh.i32(3, 13);
        dh.field(7, CT_STRUCT);
        {
            let mut d = Enc::new();
            d.i32(1, 2);
            d.i32(2, 0);
            d.stop();
            dh.buf.extend(d.buf);
        }
        dh.stop();
        let dict_header = dh.buf;

        let mut dict_body = Vec::new();
        dict_body.extend(2u32.to_le_bytes());
        dict_body.extend(b"aa");
        dict_body.extend(3u32.to_le_bytes());
        dict_body.extend(b"bbb");

        let mut ph = Enc::new();
        ph.i32(1, 0);
        ph.i32(2, 16);
        ph.i32(3, 16);
        ph.field(5, CT_STRUCT);
        {
            let mut d = Enc::new();
            d.i32(1, 3);
            d.i32(2, 8);
            d.i32(3, 3);
            d.i32(4, 3);
            d.stop();
            ph.buf.extend(d.buf);
        }
        ph.stop();
        let data_header = ph.buf;

        let mut data_body = vec![0x01u8];
        for v in [0u8, 1u8, 0u8] {
            data_body.push(0x02u8);
            data_body.push(v);
        }

        let dict_offset = 4usize;
        let data_offset = dict_offset + dict_header.len() + dict_body.len();
        let total =
            (dict_header.len() + dict_body.len() + data_header.len() + data_body.len()) as i64;

        let mut leaf = Enc::new();
        leaf.i32(1, 6);
        leaf.i32(3, 0);
        leaf.binary(4, b"col");
        leaf.stop();

        let mut cm = Enc::new();
        cm.i32(1, 6);
        cm.field(2, CT_LIST);
        cm.buf.push((1 << 4) | CT_I32);
        cm.buf.extend(zigzag(8));
        cm.field(3, CT_LIST);
        cm.buf.push((1 << 4) | CT_BINARY);
        cm.buf.extend(uvarint(3));
        cm.buf.extend(b"col");
        cm.i32(4, codec);
        cm.i64(5, 3);
        cm.i64(6, total);
        cm.i64(7, total);
        cm.i64(9, data_offset as i64);
        cm.i64(11, dict_offset as i64);
        cm.stop();

        let mut cc = Enc::new();
        cc.i64(2, dict_offset as i64);
        cc.field(3, CT_STRUCT);
        cc.buf.extend(cm.buf);
        cc.stop();

        let mut rg = Enc::new();
        rg.list_struct(1, &[cc.buf]);
        rg.i64(2, total);
        rg.i64(3, 3);
        rg.stop();

        let footer = file_metadata_footer(&[schema_root_bytes(), leaf.buf], &[rg.buf]);

        let mut file = Vec::new();
        file.extend(b"PAR1");
        file.extend(&dict_header);
        file.extend(&dict_body);
        file.extend(&data_header);
        file.extend(&data_body);
        file.extend(&footer);
        file.extend((footer.len() as u32).to_le_bytes());
        file.extend(b"PAR1");
        file
    }

    #[test]
    fn parquet_reads_plain_int32_column() {
        let bytes = plain_int32_file();
        let cols = parse_parquet(&bytes).expect("columns");
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].name, "col");
        assert_eq!(
            cols[0].values,
            vec![
                ParquetValue::I32(1),
                ParquetValue::I32(2),
                ParquetValue::I32(3),
            ]
        );
    }

    #[test]
    fn parquet_reads_rle_dictionary_byte_array_column() {
        let bytes = dictionary_byte_array_file(0);
        let cols = parse_parquet(&bytes).expect("columns");
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].name, "col");
        assert_eq!(
            cols[0].values,
            vec![
                ParquetValue::Bytes(b"aa".to_vec()),
                ParquetValue::Bytes(b"bbb".to_vec()),
                ParquetValue::Bytes(b"aa".to_vec()),
            ]
        );
    }

    #[test]
    fn parquet_names_compressed_codec_unhandled() {
        let bytes = dictionary_byte_array_file(1);
        let cols = parse_parquet(&bytes).expect("columns");
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].name, "col");
        assert_eq!(
            cols[0].values,
            vec![ParquetValue::Unhandled("codec 1".to_string())]
        );
    }
}
