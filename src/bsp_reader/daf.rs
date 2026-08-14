use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub enum DafError {
    Io(std::io::Error),
    TooSmall(usize),
    BadIdword([u8; 8]),
    UnsupportedFormat([u8; 8]),
    BadSummary {
        record: u32,
        reason: &'static str,
    },
    AddressOutOfBounds {
        start: u32,
        end: u32,
        file_doubles: u64,
    },
}

impl std::fmt::Display for DafError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DafError::Io(e) => write!(f, "I/O error: {e}"),
            DafError::TooSmall(n) => {
                write!(f, "file too small ({n} bytes) to contain a DAF header")
            }
            DafError::BadIdword(w) => write!(f, "unrecognized DAF identifier: {w:?}"),
            DafError::UnsupportedFormat(fmt) => {
                write!(
                    f,
                    "unsupported binary format: {fmt:?} (only LTL-IEEE is supported)"
                )
            }
            DafError::BadSummary { record, reason } => {
                write!(f, "malformed summary at record {record}: {reason}")
            }
            DafError::AddressOutOfBounds {
                start,
                end,
                file_doubles,
            } => {
                write!(
                    f,
                    "address range [{start},{end}] out of file (file has {file_doubles} doubles)"
                )
            }
        }
    }
}

impl std::error::Error for DafError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DafError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DafError {
    fn from(e: std::io::Error) -> Self {
        DafError::Io(e)
    }
}

pub const RECORD_BYTES: usize = 1024;
pub const DOUBLE_BYTES: usize = 8;

#[derive(Clone)]
pub struct DafFile {
    inner: Arc<DafInner>,
}

struct DafInner {
    data: Vec<u8>,
    pub idword: [u8; 8],
    pub nd: u32,
    pub ni: u32,
    pub fward: u32,
}

#[derive(Debug, Clone)]
pub struct Summary {
    pub doubles: Vec<f64>,  // length == nd
    pub integers: Vec<i32>, // length == ni
    pub name: String,
}

impl DafFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DafError> {
        let data = std::fs::read(path)?;
        Self::from_data(data)
    }

    fn from_data(data: Vec<u8>) -> Result<Self, DafError> {
        if data.len() < RECORD_BYTES {
            return Err(DafError::TooSmall(data.len()));
        }
        let bytes = &data[..];

        let mut idword = [0u8; 8];
        idword.copy_from_slice(&bytes[0..8]);
        if !idword.starts_with(b"DAF/") {
            return Err(DafError::BadIdword(idword));
        }

        let mut locfmt = [0u8; 8];
        locfmt.copy_from_slice(&bytes[88..96]);
        if &locfmt != b"LTL-IEEE" {
            return Err(DafError::UnsupportedFormat(locfmt));
        }

        let nd = {
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&bytes[8..12]);
            u32::from_le_bytes(buf)
        };
        let ni = {
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&bytes[12..16]);
            u32::from_le_bytes(buf)
        };
        let fward = {
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&bytes[76..80]);
            u32::from_le_bytes(buf)
        };

        Ok(DafFile {
            inner: Arc::new(DafInner {
                data,
                idword,
                nd,
                ni,
                fward,
            }),
        })
    }

    pub fn nd(&self) -> u32 {
        self.inner.nd
    }
    pub fn ni(&self) -> u32 {
        self.inner.ni
    }
    pub fn idword(&self) -> [u8; 8] {
        self.inner.idword
    }

    pub fn summary_size_doubles(&self) -> usize {
        self.inner.nd as usize + (self.inner.ni as usize).div_ceil(2)
    }

    pub fn summaries(&self) -> Result<Vec<Summary>, DafError> {
        let mut out = Vec::new();
        let mut rec = self.inner.fward;
        while rec != 0 {
            self.read_summary_record(rec, &mut out)?;
            rec = self.next_record(rec)?;
        }
        Ok(out)
    }

    fn next_record(&self, rec: u32) -> Result<u32, DafError> {
        let bytes = self.record_bytes(rec)?;
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[0..8]);
        let next = f64::from_le_bytes(buf);
        Ok(next as u32)
    }

    fn record_bytes(&self, rec: u32) -> Result<&[u8], DafError> {
        let start = (rec as usize - 1) * RECORD_BYTES;
        let end = start + RECORD_BYTES;
        if end > self.inner.data.len() {
            return Err(DafError::BadSummary {
                record: rec,
                reason: "record extends past end of file",
            });
        }
        Ok(&self.inner.data[start..end])
    }

    fn read_summary_record(&self, rec: u32, out: &mut Vec<Summary>) -> Result<(), DafError> {
        let sbytes = self.record_bytes(rec)?;
        let name_rec = rec + 1;
        let nbytes = self.record_bytes(name_rec)?;

        let nsum_f = {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&sbytes[16..24]);
            f64::from_le_bytes(buf)
        };
        let nsum = nsum_f as usize;
        let ss = self.summary_size_doubles();
        let nd = self.inner.nd as usize;
        let ni = self.inner.ni as usize;
        let name_chars = ss * DOUBLE_BYTES;

        for i in 0..nsum {
            let soff = 24 + i * ss * DOUBLE_BYTES;
            if soff + ss * DOUBLE_BYTES > sbytes.len() {
                return Err(DafError::BadSummary {
                    record: rec,
                    reason: "summary past end of record",
                });
            }
            let sslice = &sbytes[soff..soff + ss * DOUBLE_BYTES];
            let mut doubles = Vec::with_capacity(nd);
            for k in 0..nd {
                let off = k * DOUBLE_BYTES;
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&sslice[off..off + 8]);
                doubles.push(f64::from_le_bytes(buf));
            }
            let mut integers = Vec::with_capacity(ni);
            let int_start = nd * DOUBLE_BYTES;
            for k in 0..ni {
                let off = int_start + k * 4;
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&sslice[off..off + 4]);
                integers.push(i32::from_le_bytes(buf));
            }
            let noff = i * name_chars;
            let name_slice = &nbytes[noff..noff + name_chars];
            let name = std::str::from_utf8(name_slice)
                .unwrap_or("")
                .trim_end_matches('\0')
                .trim_end()
                .to_string();
            out.push(Summary {
                doubles,
                integers,
                name,
            });
        }
        Ok(())
    }

    pub fn read_doubles(&self, start_addr: u32, end_addr: u32) -> Result<Vec<f64>, DafError> {
        if start_addr == 0 || end_addr < start_addr {
            return Err(DafError::AddressOutOfBounds {
                start: start_addr,
                end: end_addr,
                file_doubles: (self.inner.data.len() / DOUBLE_BYTES) as u64,
            });
        }
        let byte_start = (start_addr as usize - 1) * DOUBLE_BYTES;
        let byte_end = end_addr as usize * DOUBLE_BYTES;
        if byte_end > self.inner.data.len() {
            return Err(DafError::AddressOutOfBounds {
                start: start_addr,
                end: end_addr,
                file_doubles: (self.inner.data.len() / DOUBLE_BYTES) as u64,
            });
        }
        let n = (end_addr - start_addr + 1) as usize;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let off = byte_start + i * DOUBLE_BYTES;
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&self.inner.data[off..off + 8]);
            out.push(f64::from_le_bytes(buf));
        }
        Ok(out)
    }

    pub fn doubles_native(&self, start_addr: u32, end_addr: u32) -> Result<Vec<f64>, DafError> {
        let bytes = self.double_slice(start_addr, end_addr)?;
        let n = bytes.len() / DOUBLE_BYTES;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&bytes[i * 8..(i + 1) * 8]);
            out.push(f64::from_le_bytes(buf));
        }
        Ok(out)
    }

    pub fn double_slice(&self, start_addr: u32, end_addr: u32) -> Result<&[u8], DafError> {
        if start_addr == 0 || end_addr < start_addr {
            return Err(DafError::AddressOutOfBounds {
                start: start_addr,
                end: end_addr,
                file_doubles: (self.inner.data.len() / DOUBLE_BYTES) as u64,
            });
        }
        let byte_start = (start_addr as usize - 1) * DOUBLE_BYTES;
        let byte_end = end_addr as usize * DOUBLE_BYTES;
        if byte_end > self.inner.data.len() {
            return Err(DafError::AddressOutOfBounds {
                start: start_addr,
                end: end_addr,
                file_doubles: (self.inner.data.len() / DOUBLE_BYTES) as u64,
            });
        }
        Ok(&self.inner.data[byte_start..byte_end])
    }

    pub fn read_n_doubles(&self, start_addr: u32, count: usize) -> Result<Vec<f64>, DafError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let end_addr = start_addr + count as u32 - 1;
        self.read_doubles(start_addr, end_addr)
    }
}
