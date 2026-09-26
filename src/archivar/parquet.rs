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
    pub scale: Option<i32>,
    pub precision: Option<i32>,
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
        let mut scale = None;
        let mut precision = None;
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
                7 => scale = Some(self.i32()?),
                8 => precision = Some(self.i32()?),
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
            scale,
            precision,
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
    Int96([u8; 12]),
    Unhandled(String),
}

#[derive(Clone, Debug)]
pub struct ParquetColumn {
    pub name: String,
    pub scale: Option<i32>,
    pub values: Vec<ParquetValue>,
}

struct PageHeaderInfo {
    page_type: i32,
    uncompressed_page_size: i32,
    compressed_page_size: i32,
    num_values: Option<i32>,
    data_encoding: Option<i32>,
    dict_num_values: Option<i32>,
    data_page_v2: bool,
}

impl<'a> Compact<'a> {
    fn page_header(&mut self) -> Result<PageHeaderInfo, ParquetNote> {
        let mut page_type = None;
        let mut uncompressed = None;
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
                2 => uncompressed = Some(self.i32()?),
                4 => self.skip(ctype)?,
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
            uncompressed_page_size: uncompressed.ok_or(ParquetNote::AbsentField { id: 2 })?,
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
        3 => {
            let len = 12usize;
            let body = slice_at(data, 0, count * len)?;
            for i in 0..count {
                let o = i * len;
                let mut b = [0u8; 12];
                b.copy_from_slice(&body[o..o + 12]);
                out.push(ParquetValue::Int96(b));
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

fn copy_span(out: &mut Vec<u8>, off: usize, len: usize) -> Result<(), ParquetNote> {
    if off == 0 || off > out.len() {
        return Err(ParquetNote::Truncated { off: out.len() });
    }
    let start = out.len() - off;
    for i in 0..len {
        let b = *out
            .get(start + i)
            .ok_or(ParquetNote::Truncated { off: out.len() })?;
        out.push(b);
    }
    Ok(())
}

fn snappy_decode(data: &[u8]) -> Option<Vec<u8>> {
    let (ulen, mut pos) = uvarint_at(data, 0).ok()?;
    let mut out: Vec<u8> = Vec::with_capacity(ulen.min(1 << 20) as usize);
    while pos < data.len() {
        let tag = *data.get(pos)?;
        pos += 1;
        match tag & 0x03 {
            0 => {
                let mut len = (tag >> 2) as usize + 1;
                if len > 60 {
                    let n = len - 60;
                    let mut extra = 0usize;
                    for i in 0..n {
                        extra |= (*data.get(pos + i)? as usize) << (8 * i);
                    }
                    pos += n;
                    len = extra + 1;
                }
                out.extend_from_slice(data.get(pos..pos + len)?);
                pos += len;
            }
            1 => {
                let len = ((tag >> 2) & 0x07) as usize + 4;
                let off = (((tag >> 5) & 0x07) as usize) << 8 | (*data.get(pos)? as usize);
                pos += 1;
                copy_span(&mut out, off, len).ok()?;
            }
            2 => {
                let len = (tag >> 2) as usize + 1;
                let b = data.get(pos..pos + 2)?;
                let off = u16::from_le_bytes([b[0], b[1]]) as usize;
                pos += 2;
                copy_span(&mut out, off, len).ok()?;
            }
            3 => {
                let len = (tag >> 2) as usize + 1;
                let b = data.get(pos..pos + 4)?;
                let off = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize;
                pos += 4;
                copy_span(&mut out, off, len).ok()?;
            }
            _ => return None,
        }
    }
    (out.len() as u64 == ulen).then_some(out)
}

fn zigzag_decode(n: u64) -> i64 {
    ((n >> 1) as i64) ^ -((n & 1) as i64)
}

fn lz4_block(data: &[u8], expected: usize) -> Option<(Vec<u8>, usize)> {
    let mut out: Vec<u8> = Vec::with_capacity(expected.min(1 << 20));
    let mut pos = 0usize;
    while pos < data.len() {
        let token = *data.get(pos)?;
        pos += 1;
        let mut lit_len = (token >> 4) as usize;
        if lit_len == 15 {
            loop {
                let b = *data.get(pos)? as usize;
                pos += 1;
                lit_len += b;
                if b != 255 {
                    break;
                }
            }
        }
        let lit_end = pos.checked_add(lit_len)?;
        let lit = data.get(pos..lit_end)?;
        pos = lit_end;
        out.extend_from_slice(lit);
        if pos == data.len() {
            if token & 0x0f != 0 {
                return None;
            }
            return (out.len() == expected).then_some((out, pos));
        }
        let ob = data.get(pos..pos + 2)?;
        pos += 2;
        let off = u16::from_le_bytes([ob[0], ob[1]]) as usize;
        if off == 0 || off > out.len() {
            return None;
        }
        let mut match_len = (token & 0x0f) as usize + 4;
        if token & 0x0f == 15 {
            loop {
                let b = *data.get(pos)? as usize;
                pos += 1;
                match_len += b;
                if b != 255 {
                    break;
                }
            }
        }
        if out.len() > expected || match_len > expected - out.len() {
            return None;
        }
        let start = out.len() - off;
        for i in 0..match_len {
            let b = *out.get(start + i)?;
            out.push(b);
        }
    }
    (out.len() == expected).then_some((out, pos))
}

fn lz4_hadoop_decode(data: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos < data.len() {
        let b = data.get(pos..pos + 4)?;
        pos += 4;
        let ulen = u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize;
        if ulen == 0 {
            return None;
        }
        let (chunk, used) = lz4_block(&data[pos..], ulen)?;
        pos += used;
        out.extend_from_slice(&chunk);
    }
    Some(out)
}

fn lz4_raw_decode(data: &[u8], expected: i32) -> Option<Vec<u8>> {
    let ulen = usize::try_from(expected).ok()?;
    let (out, _) = lz4_block(data, ulen)?;
    Some(out)
}

fn delta_binary_packed(data: &[u8]) -> Result<(Vec<i64>, usize), ParquetNote> {
    let (block_size, mut pos) = uvarint_at(data, 0)?;
    let (miniblocks_per_block, used) = uvarint_at(data, pos)?;
    pos += used;
    let (total_count, used) = uvarint_at(data, pos)?;
    pos += used;
    let (first_raw, used) = uvarint_at(data, pos)?;
    pos += used;
    let block_size = block_size as usize;
    let miniblocks_per_block = miniblocks_per_block as usize;
    let total_count = total_count as usize;
    if block_size == 0
        || miniblocks_per_block == 0
        || block_size % miniblocks_per_block != 0
        || total_count == 0
    {
        return Err(ParquetNote::Truncated { off: pos });
    }
    let values_per_miniblock = block_size / miniblocks_per_block;
    let num_miniblocks = (total_count - 1).div_ceil(values_per_miniblock);
    let mut min_deltas = Vec::with_capacity(num_miniblocks);
    for _ in 0..num_miniblocks {
        let (v, used) = uvarint_at(data, pos)?;
        pos += used;
        min_deltas.push(zigzag_decode(v));
    }
    let mut bit_widths = Vec::with_capacity(num_miniblocks);
    for _ in 0..num_miniblocks {
        bit_widths.push(*data.get(pos).ok_or(ParquetNote::Truncated { off: pos })?);
        pos += 1;
    }
    let mut values = Vec::with_capacity(total_count.min(1 << 20));
    let mut previous = zigzag_decode(first_raw);
    values.push(previous);
    let mut deltas_left = total_count - 1;
    for mb in 0..num_miniblocks {
        let count = deltas_left.min(values_per_miniblock);
        let bit_width = bit_widths[mb];
        let bits_needed = count
            .checked_mul(bit_width as usize)
            .ok_or(ParquetNote::Truncated { off: pos })?;
        let bytes_needed = bits_needed.div_ceil(8);
        let body = slice_at(data, pos, bytes_needed)?;
        pos += bytes_needed;
        let mut bits = Bits {
            data: body,
            bit_pos: 0,
        };
        for _ in 0..count {
            let packed = bits.read(bit_width)?;
            previous += min_deltas[mb] + zigzag_decode(packed as u64);
            values.push(previous);
        }
        deltas_left -= count;
    }
    Ok((values, pos))
}

fn decode_delta_binary_packed(
    data: &[u8],
    type_tag: i32,
    count: usize,
) -> Result<Vec<ParquetValue>, ParquetNote> {
    if !matches!(type_tag, 1 | 2) {
        return Ok(vec![ParquetValue::Unhandled(format!(
            "delta type {}",
            type_tag
        ))]);
    }
    let (values, _) = delta_binary_packed(data)?;
    if values.len() < count {
        return Err(ParquetNote::Truncated { off: data.len() });
    }
    let mut out = Vec::with_capacity(count);
    for v in values.into_iter().take(count) {
        out.push(match type_tag {
            1 => ParquetValue::I32(v as i32),
            _ => ParquetValue::I64(v),
        });
    }
    Ok(out)
}

fn decode_delta_length_byte_array(
    data: &[u8],
    count: usize,
) -> Result<Vec<ParquetValue>, ParquetNote> {
    let (lengths, pos) = delta_binary_packed(data)?;
    if lengths.len() < count {
        return Err(ParquetNote::Truncated { off: data.len() });
    }
    let payload = &data[pos..];
    let mut out = Vec::with_capacity(count);
    let mut offset = 0usize;
    for len in lengths.into_iter().take(count) {
        if len < 0 {
            return Err(ParquetNote::Truncated { off: data.len() });
        }
        let len = len as usize;
        let body = slice_at(payload, offset, len)?;
        offset += len;
        out.push(ParquetValue::Bytes(body.to_vec()));
    }
    Ok(out)
}

fn decode_delta_byte_array(data: &[u8], count: usize) -> Result<Vec<ParquetValue>, ParquetNote> {
    let (prefix_lengths, pos) = delta_binary_packed(data)?;
    if prefix_lengths.len() < count {
        return Err(ParquetNote::Truncated { off: data.len() });
    }
    let suffixes = decode_delta_length_byte_array(&data[pos..], count)?;
    let mut out = Vec::with_capacity(count);
    let mut previous: Vec<u8> = Vec::new();
    for i in 0..count {
        let plen = prefix_lengths[i];
        if plen < 0 || plen as usize > previous.len() {
            return Err(ParquetNote::Truncated { off: data.len() });
        }
        let plen = plen as usize;
        let suffix = match suffixes.get(i) {
            Some(ParquetValue::Bytes(b)) => b,
            _ => return Err(ParquetNote::Truncated { off: data.len() }),
        };
        let mut value = Vec::with_capacity(plen + suffix.len());
        value.extend_from_slice(&previous[..plen]);
        value.extend_from_slice(suffix);
        previous = value.clone();
        out.push(ParquetValue::Bytes(value));
    }
    Ok(out)
}

fn decode_byte_stream_split(
    data: &[u8],
    type_tag: i32,
    count: usize,
) -> Result<Vec<ParquetValue>, ParquetNote> {
    let width = match type_tag {
        4 => 4usize,
        5 => 8usize,
        tag => {
            return Ok(vec![ParquetValue::Unhandled(format!("split type {}", tag))]);
        }
    };
    let total = count
        .checked_mul(width)
        .ok_or(ParquetNote::Truncated { off: 0 })?;
    let body = slice_at(data, 0, total)?;
    let mut unsplit = Vec::with_capacity(total);
    for v in 0..count {
        for s in 0..width {
            unsplit.push(body[s * count + v]);
        }
    }
    decode_plain(&unsplit, type_tag, None, count)
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

fn page_body(codec: i32, raw: &[u8], header: &PageHeaderInfo) -> Option<Vec<u8>> {
    match codec {
        0 => Some(raw.to_vec()),
        1 => snappy_decode(raw),
        2 => super::inflate::gunzip(raw),
        5 => lz4_hadoop_decode(raw),
        7 => lz4_raw_decode(raw, header.uncompressed_page_size),
        _ => None,
    }
}

fn decode_column(
    cm: &ColumnMetaData,
    leaf: &SchemaElement,
    data: &[u8],
) -> Result<Vec<ParquetValue>, ParquetNote> {
    if !matches!(cm.codec, 0 | 1 | 2 | 5 | 7) {
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
        let Some(plain) = page_body(cm.codec, body, &hdr) else {
            return Ok(vec![ParquetValue::Unhandled(format!(
                "codec {} page",
                cm.codec
            ))]);
        };
        dictionary = decode_plain(&plain, type_tag, type_length, n)?;
    }
    let mut out = Vec::new();
    let mut off = cm.data_page_offset as usize;
    let mut remaining = cm.num_values;
    while remaining > 0 {
        let (hdr, hlen) = read_page_header(data, off)?;
        let body = slice_at(data, off + hlen, hdr.compressed_page_size as usize)?;
        if hdr.page_type == 2 {
            let n = hdr
                .dict_num_values
                .ok_or(ParquetNote::AbsentField { id: 1 })? as usize;
            let Some(plain) = page_body(cm.codec, body, &hdr) else {
                out.push(ParquetValue::Unhandled(format!("codec {} page", cm.codec)));
                break;
            };
            dictionary = decode_plain(&plain, type_tag, type_length, n)?;
            off += hlen + hdr.compressed_page_size as usize;
            continue;
        }
        if hdr.page_type == 1 {
            off += hlen + hdr.compressed_page_size as usize;
            continue;
        }
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
        let Some(plain) = page_body(cm.codec, body, &hdr) else {
            out.push(ParquetValue::Unhandled(format!("codec {} page", cm.codec)));
            break;
        };
        let values_body = skip_levels(&plain, max_rep, max_def)?;
        let vals = match enc {
            0 => decode_plain(values_body, type_tag, type_length, n)?,
            2 | 8 => decode_dict(values_body, &dictionary, n)?,
            5 => decode_delta_binary_packed(values_body, type_tag, n)?,
            6 => decode_delta_length_byte_array(values_body, n)?,
            7 => decode_delta_byte_array(values_body, n)?,
            9 => decode_byte_stream_split(values_body, type_tag, n)?,
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
            scale: l.scale,
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
pub(crate) mod testkit {
    use super::*;

    pub struct Enc {
        pub buf: Vec<u8>,
        last: i16,
    }

    impl Enc {
        pub fn new() -> Enc {
            Enc {
                buf: Vec::new(),
                last: 0,
            }
        }

        pub fn field(&mut self, id: i16, ctype: u8) {
            let delta = id - self.last;
            if delta > 0 && delta <= 15 {
                self.buf.push(((delta as u8) << 4) | ctype);
            } else {
                self.buf.push(ctype);
                self.buf.extend(zigzag(id as i64));
            }
            self.last = id;
        }

        pub fn i32(&mut self, id: i16, v: i32) {
            self.field(id, CT_I32);
            self.buf.extend(zigzag(v as i64));
        }

        pub fn i64(&mut self, id: i16, v: i64) {
            self.field(id, CT_I64);
            self.buf.extend(zigzag(v));
        }

        pub fn binary(&mut self, id: i16, s: &[u8]) {
            self.field(id, CT_BINARY);
            self.buf.extend(uvarint(s.len() as u64));
            self.buf.extend(s);
        }

        pub fn list_struct(&mut self, id: i16, items: &[Vec<u8>]) {
            self.field(id, CT_LIST);
            self.buf.push(((items.len() as u8) << 4) | CT_STRUCT);
            for it in items {
                self.buf.extend(it.iter().copied());
            }
        }

        pub fn stop(&mut self) {
            self.buf.push(CT_STOP);
        }
    }

    pub fn uvarint(mut n: u64) -> Vec<u8> {
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

    pub fn zigzag(n: i64) -> Vec<u8> {
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

    pub fn file_metadata_footer(schema: &[Vec<u8>], row_groups: &[Vec<u8>]) -> Vec<u8> {
        let mut fm = Enc::new();
        fm.i32(1, 1);
        fm.list_struct(2, schema);
        fm.i64(3, 3);
        fm.list_struct(4, row_groups);
        fm.binary(6, b"omegaflow-test");
        fm.stop();
        fm.buf
    }

    pub fn schema_root_bytes() -> Vec<u8> {
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

    fn snappy_literal(bytes: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(uvarint(bytes.len() as u64));
        out.push(((bytes.len() - 1) << 2) as u8);
        out.extend_from_slice(bytes);
        out
    }

    fn crc32(bytes: &[u8]) -> u32 {
        let mut crc = 0xffff_ffffu32;
        for &b in bytes {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0xedb8_8320u32 & (0u32.wrapping_sub(crc & 1)));
            }
        }
        !crc
    }

    fn gzip_stored(bytes: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend([0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff]);
        let len = bytes.len() as u32;
        out.push(0x01);
        out.extend((len as u16).to_le_bytes());
        out.extend((!(len as u16)).to_le_bytes());
        out.extend_from_slice(bytes);
        out.extend(crc32(bytes).to_le_bytes());
        out.extend(len.to_le_bytes());
        out
    }

    fn lz4_literal_block(bytes: &[u8]) -> Option<Vec<u8>> {
        if bytes.len() >= 15 {
            return None;
        }
        let mut out = Vec::with_capacity(bytes.len() + 1);
        out.push((bytes.len() << 4) as u8);
        out.extend_from_slice(bytes);
        Some(out)
    }

    fn lz4_framed(chunks: &[&[u8]]) -> Vec<u8> {
        let mut out = Vec::new();
        for chunk in chunks {
            out.extend((chunk.len() as u32).to_be_bytes());
            out.extend(lz4_literal_block(chunk).expect("small literal block"));
        }
        out
    }

    fn zigzag_u64(n: i64) -> u64 {
        ((n << 1) ^ (n >> 63)) as u64
    }

    fn bit_pack(values: &[u64], width: u8) -> Vec<u8> {
        let mut out = Vec::new();
        let mut acc = 0u64;
        let mut bits = 0u32;
        for &v in values {
            acc |= v << bits;
            bits += width as u32;
            while bits >= 8 {
                out.push((acc & 0xff) as u8);
                acc >>= 8;
                bits -= 8;
            }
        }
        if bits > 0 {
            out.push((acc & 0xff) as u8);
        }
        out
    }

    fn delta_stream(values: &[i64], block_size: u64, miniblocks: u64) -> Vec<u8> {
        let vpm = (block_size / miniblocks) as usize;
        let deltas: Vec<i64> = values.windows(2).map(|w| w[1] - w[0]).collect();
        let num_miniblocks = deltas.len().div_ceil(vpm);
        let mut min_deltas = Vec::with_capacity(num_miniblocks);
        let mut widths = Vec::with_capacity(num_miniblocks);
        let mut packed = Vec::new();
        for chunk in deltas.chunks(vpm) {
            let min = *chunk.iter().min().expect("non-empty miniblock chunk");
            min_deltas.push(min);
            let mut max_z = 0u64;
            for &d in chunk {
                max_z = max_z.max(zigzag_u64(d - min));
            }
            let mut width = 0u32;
            while width < 64 && max_z >= (1u64 << width) {
                width += 1;
            }
            widths.push(width as u8);
            let vals: Vec<u64> = chunk.iter().map(|&d| zigzag_u64(d - min)).collect();
            packed.extend(bit_pack(&vals, width as u8));
        }
        let mut out = Vec::new();
        out.extend(uvarint(block_size));
        out.extend(uvarint(miniblocks));
        out.extend(uvarint(values.len() as u64));
        out.extend(zigzag(values[0]));
        for &md in &min_deltas {
            out.extend(zigzag(md));
        }
        out.extend(widths);
        out.extend(packed);
        out
    }

    fn dictionary_byte_array_file(codec: i32) -> Vec<u8> {
        dictionary_byte_array_file_layout(codec, false)
    }

    fn dictionary_byte_array_file_net(codec: i32) -> Vec<u8> {
        dictionary_byte_array_file_layout(codec, true)
    }

    fn dictionary_byte_array_file_layout(codec: i32, net_layout: bool) -> Vec<u8> {
        let wrap = |b: Vec<u8>| -> Vec<u8> { if codec == 1 { snappy_literal(&b) } else { b } };
        let mut plain_dict = Vec::new();
        plain_dict.extend(2u32.to_le_bytes());
        plain_dict.extend(b"aa");
        plain_dict.extend(3u32.to_le_bytes());
        plain_dict.extend(b"bbb");
        let dict_body = wrap(plain_dict);

        let mut plain_data = vec![0x01u8];
        for v in [0u8, 1u8, 0u8] {
            plain_data.push(0x02u8);
            plain_data.push(v);
        }
        let data_body = wrap(plain_data);

        let mut dh = Enc::new();
        dh.i32(1, 2);
        dh.i32(2, dict_body.len() as i32);
        dh.i32(3, dict_body.len() as i32);
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

        let mut ph = Enc::new();
        ph.i32(1, 0);
        ph.i32(2, data_body.len() as i32);
        ph.i32(3, data_body.len() as i32);
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
        if net_layout {
            cm.i64(9, dict_offset as i64);
        } else {
            cm.i64(9, data_offset as i64);
            cm.i64(11, dict_offset as i64);
        }
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
        let bytes = dictionary_byte_array_file(3);
        let cols = parse_parquet(&bytes).expect("columns");
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].name, "col");
        assert_eq!(
            cols[0].values,
            vec![ParquetValue::Unhandled("codec 3".to_string())]
        );
    }

    #[test]
    fn parquet_reads_snappy_dictionary_byte_array_column() {
        let bytes = dictionary_byte_array_file(1);
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
    fn parquet_reads_a_dictionary_page_at_the_data_offset() {
        let bytes = dictionary_byte_array_file_net(1);
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
    fn snappy_decode_reads_literal_blocks() {
        let mut s = Vec::new();
        s.extend(uvarint(5));
        s.push((4 << 2) as u8);
        s.extend(b"hello");
        assert_eq!(snappy_decode(&s), Some(b"hello".to_vec()));
    }

    #[test]
    fn snappy_decode_reads_one_byte_offset_copies() {
        let mut s = Vec::new();
        s.extend(uvarint(6));
        s.push((1 << 2) as u8);
        s.extend(b"ab");
        s.push(0x01);
        s.push(0x02);
        assert_eq!(snappy_decode(&s), Some(b"ababab".to_vec()));
    }

    #[test]
    fn snappy_decode_reads_two_byte_offset_copies() {
        let mut s = Vec::new();
        s.extend(uvarint(10));
        s.push((4 << 2) as u8);
        s.extend(b"abcde");
        s.push(0x12);
        s.extend(5u16.to_le_bytes());
        assert_eq!(snappy_decode(&s), Some(b"abcdeabcde".to_vec()));
    }

    #[test]
    fn snappy_decode_refuses_zero_offset() {
        let mut s = Vec::new();
        s.extend(uvarint(4));
        s.push(0x01);
        s.push(0x00);
        assert_eq!(snappy_decode(&s), None);
    }

    #[test]
    fn snappy_decode_refuses_truncated_body() {
        let mut s = Vec::new();
        s.extend(uvarint(5));
        s.push((4 << 2) as u8);
        s.extend(b"hi");
        assert_eq!(snappy_decode(&s), None);
    }

    #[test]
    fn parquet_reads_gzip_dictionary_byte_array_column() {
        let bytes = dictionary_byte_array_file(2);
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
    fn parquet_reads_lz4_dictionary_byte_array_column() {
        let bytes = dictionary_byte_array_file(5);
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

    fn header_with(uncompressed: i32) -> PageHeaderInfo {
        PageHeaderInfo {
            page_type: 0,
            uncompressed_page_size: uncompressed,
            compressed_page_size: uncompressed,
            num_values: None,
            data_encoding: None,
            dict_num_values: None,
            data_page_v2: false,
        }
    }

    #[test]
    fn page_body_gunzips_stored_block() {
        let compressed = gzip_stored(b"hello");
        assert_eq!(
            page_body(2, &compressed, &header_with(5)),
            Some(b"hello".to_vec())
        );
    }

    #[test]
    fn page_body_decodes_lz4() {
        let framed = lz4_framed(&[b"abc"]);
        assert_eq!(
            page_body(5, &framed, &header_with(3)),
            Some(b"abc".to_vec())
        );
    }

    #[test]
    fn page_body_decodes_lz4_raw() {
        let block = lz4_literal_block(b"hello world").expect("small literal block");
        assert_eq!(
            page_body(7, &block, &header_with(11)),
            Some(b"hello world".to_vec())
        );
    }

    #[test]
    fn lz4_block_decodes_literals_only() {
        let block = lz4_literal_block(b"hello world").expect("small literal block");
        let (out, used) = lz4_block(&block, 11).expect("decoded");
        assert_eq!(out, b"hello world");
        assert_eq!(used, block.len());
    }

    #[test]
    fn lz4_block_decodes_match_copy() {
        let mut block = vec![0x41u8];
        block.extend(b"abcd");
        block.extend(4u16.to_le_bytes());
        let (out, used) = lz4_block(&block, 9).expect("decoded");
        assert_eq!(out, b"abcdabcda");
        assert_eq!(used, block.len());
    }

    #[test]
    fn lz4_block_decodes_literal_length_extension() {
        let mut block = vec![0xF1u8, 0x01];
        block.extend(b"abcdefghijklmnop");
        block.extend(16u16.to_le_bytes());
        let (out, used) = lz4_block(&block, 21).expect("decoded");
        assert_eq!(out, b"abcdefghijklmnopabcde");
        assert_eq!(used, block.len());
    }

    #[test]
    fn lz4_block_refuses_zero_offset() {
        let mut block = vec![0x40u8];
        block.extend(b"abcd");
        block.extend(0u16.to_le_bytes());
        assert_eq!(lz4_block(&block, 4), None);
    }

    #[test]
    fn lz4_block_refuses_oversized_output() {
        let mut block = vec![0x43u8];
        block.extend(b"abcd");
        block.extend(1u16.to_le_bytes());
        assert_eq!(lz4_block(&block, 5), None);
    }

    #[test]
    fn lz4_hadoop_decode_concatenates_chunks() {
        let framed = lz4_framed(&[b"abc", b"defgh"]);
        let out = lz4_hadoop_decode(&framed).expect("decoded");
        assert_eq!(out, b"abcdefgh");
    }

    #[test]
    fn delta_binary_packed_decodes_hand_built_stream() {
        let mut body = Vec::new();
        body.extend(uvarint(8));
        body.extend(uvarint(2));
        body.extend(uvarint(4));
        body.extend(zigzag(10));
        body.extend(zigzag(2));
        body.push(2);
        body.push(0x08);
        let (values, used) = delta_binary_packed(&body).expect("delta values");
        assert_eq!(values, vec![10, 12, 15, 17]);
        assert_eq!(used, body.len());
    }

    #[test]
    fn delta_binary_packed_decodes_two_miniblocks() {
        let values = [1i64, 5, 2, 9, 3, 1];
        let body = delta_stream(&values, 8, 2);
        let (out, used) = delta_binary_packed(&body).expect("delta values");
        assert_eq!(out, values.to_vec());
        assert_eq!(used, body.len());
    }

    #[test]
    fn delta_binary_packed_refuses_zero_block_size() {
        let mut body = Vec::new();
        body.extend(uvarint(0));
        assert!(delta_binary_packed(&body).is_err());
    }

    #[test]
    fn delta_binary_packed_refuses_truncated_body() {
        let values = [1i64, 5, 2, 9, 3, 1];
        let mut body = delta_stream(&values, 8, 2);
        body.pop();
        assert!(delta_binary_packed(&body).is_err());
    }

    #[test]
    fn delta_length_byte_array_decodes() {
        let lengths = [2i64, 3, 2];
        let mut body = delta_stream(&lengths, 8, 2);
        body.extend(b"aabbbcc");
        let vals = decode_delta_length_byte_array(&body, 3).expect("values");
        assert_eq!(
            vals,
            vec![
                ParquetValue::Bytes(b"aa".to_vec()),
                ParquetValue::Bytes(b"bbb".to_vec()),
                ParquetValue::Bytes(b"cc".to_vec()),
            ]
        );
    }

    #[test]
    fn delta_byte_array_decodes_prefixes() {
        let prefix_lengths = [0i64, 2, 0];
        let suffix_lengths = [2i64, 1, 2];
        let mut body = delta_stream(&prefix_lengths, 8, 2);
        body.extend(delta_stream(&suffix_lengths, 8, 2));
        body.extend(b"aaxzz");
        let vals = decode_delta_byte_array(&body, 3).expect("values");
        assert_eq!(
            vals,
            vec![
                ParquetValue::Bytes(b"aa".to_vec()),
                ParquetValue::Bytes(b"aax".to_vec()),
                ParquetValue::Bytes(b"zz".to_vec()),
            ]
        );
    }

    #[test]
    fn byte_stream_split_decodes_floats() {
        let floats = [1.0f32, 2.5f32, -3.25f32];
        let plain: Vec<u8> = floats.iter().flat_map(|f| f.to_le_bytes()).collect();
        let mut split = vec![0u8; plain.len()];
        for (v, chunk) in plain.chunks(4).enumerate() {
            for (s, b) in chunk.iter().enumerate() {
                split[s * 3 + v] = *b;
            }
        }
        let vals = decode_byte_stream_split(&split, 4, 3).expect("values");
        assert_eq!(
            vals,
            vec![
                ParquetValue::Float(1.0),
                ParquetValue::Float(2.5),
                ParquetValue::Float(-3.25),
            ]
        );
    }
}
