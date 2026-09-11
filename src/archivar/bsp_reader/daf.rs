use std::os::unix::fs::FileExt;
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

enum DafSource {
    Owned(Vec<u8>),
    File(std::fs::File),
}

struct DafInner {
    source: DafSource,
    len: u64,
    pub idword: [u8; 8],
    pub nd: u32,
    pub ni: u32,
    pub fward: u32,
}

#[derive(Debug, Clone)]
pub struct Summary {
    pub doubles: Vec<f64>,
    pub integers: Vec<i32>,
    pub name: String,
}

impl DafFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DafError> {
        let file = std::fs::File::open(path)?;
        let len = file.metadata()?.len();
        if len < RECORD_BYTES as u64 {
            return Err(DafError::TooSmall(len as usize));
        }
        let mut header = vec![0u8; RECORD_BYTES];
        file.read_exact_at(&mut header, 0)?;
        let (idword, nd, ni, fward) = Self::parse_header(&header)?;
        Ok(DafFile {
            inner: Arc::new(DafInner {
                source: DafSource::File(file),
                len,
                idword,
                nd,
                ni,
                fward,
            }),
        })
    }

    pub fn from_data(data: Vec<u8>) -> Result<Self, DafError> {
        if data.len() < RECORD_BYTES {
            return Err(DafError::TooSmall(data.len()));
        }
        let len = data.len() as u64;
        let (idword, nd, ni, fward) = Self::parse_header(&data)?;
        Ok(DafFile {
            inner: Arc::new(DafInner {
                source: DafSource::Owned(data),
                len,
                idword,
                nd,
                ni,
                fward,
            }),
        })
    }

    fn parse_header(bytes: &[u8]) -> Result<([u8; 8], u32, u32, u32), DafError> {
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

        Ok((idword, nd, ni, fward))
    }

    fn read_range(&self, byte_start: usize, byte_len: usize) -> Result<Vec<u8>, DafError> {
        if byte_start + byte_len > self.inner.len as usize {
            return Err(DafError::TooSmall(byte_start + byte_len));
        }
        match &self.inner.source {
            DafSource::Owned(data) => Ok(data[byte_start..byte_start + byte_len].to_vec()),
            DafSource::File(file) => {
                let mut buf = vec![0u8; byte_len];
                file.read_exact_at(&mut buf, byte_start as u64)?;
                Ok(buf)
            }
        }
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

    fn record_bytes(&self, rec: u32) -> Result<Vec<u8>, DafError> {
        let start = (rec as usize - 1) * RECORD_BYTES;
        let end = start + RECORD_BYTES;
        if end > self.inner.len as usize {
            return Err(DafError::BadSummary {
                record: rec,
                reason: "record extends past end of file",
            });
        }
        self.read_range(start, RECORD_BYTES)
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

    fn double_byte_range(
        &self,
        start_addr: u32,
        end_addr: u32,
    ) -> Result<(usize, usize), DafError> {
        if start_addr == 0 || end_addr < start_addr {
            return Err(DafError::AddressOutOfBounds {
                start: start_addr,
                end: end_addr,
                file_doubles: self.inner.len / DOUBLE_BYTES as u64,
            });
        }
        let byte_start = (start_addr as usize - 1) * DOUBLE_BYTES;
        let byte_end = end_addr as usize * DOUBLE_BYTES;
        if byte_end > self.inner.len as usize {
            return Err(DafError::AddressOutOfBounds {
                start: start_addr,
                end: end_addr,
                file_doubles: self.inner.len / DOUBLE_BYTES as u64,
            });
        }
        Ok((byte_start, byte_end))
    }

    pub fn read_doubles(&self, start_addr: u32, end_addr: u32) -> Result<Vec<f64>, DafError> {
        let (byte_start, byte_end) = self.double_byte_range(start_addr, end_addr)?;
        let bytes = self.read_range(byte_start, byte_end - byte_start)?;
        let n = (end_addr - start_addr + 1) as usize;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&bytes[i * 8..(i + 1) * 8]);
            out.push(f64::from_le_bytes(buf));
        }
        Ok(out)
    }

    pub fn doubles_native(&self, start_addr: u32, end_addr: u32) -> Result<Vec<f64>, DafError> {
        let (byte_start, byte_end) = self.double_byte_range(start_addr, end_addr)?;
        let bytes = self.read_range(byte_start, byte_end - byte_start)?;
        let n = bytes.len() / DOUBLE_BYTES;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&bytes[i * 8..(i + 1) * 8]);
            out.push(f64::from_le_bytes(buf));
        }
        Ok(out)
    }

    pub fn read_n_doubles(&self, start_addr: u32, count: usize) -> Result<Vec<f64>, DafError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let end_addr = start_addr + count as u32 - 1;
        self.read_doubles(start_addr, end_addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_daf() -> Vec<u8> {
        let records = 4usize;
        let mut buf = vec![0u8; records * RECORD_BYTES];
        buf[0..8].copy_from_slice(b"DAF/SPK ");
        let nd: u32 = 2;
        let ni: u32 = 2;
        buf[8..12].copy_from_slice(&nd.to_le_bytes());
        buf[12..16].copy_from_slice(&ni.to_le_bytes());
        let fward: u32 = 2;
        buf[76..80].copy_from_slice(&fward.to_le_bytes());
        buf[88..96].copy_from_slice(b"LTL-IEEE");

        let sum_rec = RECORD_BYTES;
        let nsum: f64 = 1.0;
        buf[sum_rec + 16..sum_rec + 24].copy_from_slice(&nsum.to_le_bytes());
        let d0: f64 = 1.5;
        let d1: f64 = -2.25;
        buf[sum_rec + 24..sum_rec + 32].copy_from_slice(&d0.to_le_bytes());
        buf[sum_rec + 32..sum_rec + 40].copy_from_slice(&d1.to_le_bytes());
        let i0: i32 = 10;
        let i1: i32 = -20;
        buf[sum_rec + 40..sum_rec + 44].copy_from_slice(&i0.to_le_bytes());
        buf[sum_rec + 44..sum_rec + 48].copy_from_slice(&i1.to_le_bytes());

        let name_rec = 2 * RECORD_BYTES;
        buf[name_rec..name_rec + 4].copy_from_slice(b"TEST");

        let data_rec = 3 * RECORD_BYTES;
        let v0: f64 = 3.25;
        let v1: f64 = -7.5;
        buf[data_rec..data_rec + 8].copy_from_slice(&v0.to_le_bytes());
        buf[data_rec + 8..data_rec + 16].copy_from_slice(&v1.to_le_bytes());
        buf
    }

    fn check(daf: &DafFile) {
        let summaries = daf.summaries().unwrap();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].doubles, vec![1.5, -2.25]);
        assert_eq!(summaries[0].integers, vec![10, -20]);
        assert_eq!(summaries[0].name, "TEST");

        let addr = (3 * RECORD_BYTES / DOUBLE_BYTES + 1) as u32;
        assert_eq!(daf.read_doubles(addr, addr + 1).unwrap(), vec![3.25, -7.5]);
        assert_eq!(
            daf.doubles_native(addr, addr + 1).unwrap(),
            vec![3.25, -7.5]
        );
    }

    #[test]
    fn open_matches_from_data_byte_for_byte() {
        let data = synthetic_daf();
        let from_data = DafFile::from_data(data.clone()).unwrap();
        check(&from_data);

        let path = std::env::temp_dir().join(format!("daf_parity_{}.bsp", std::process::id()));
        std::fs::write(&path, &data).unwrap();
        let opened = DafFile::open(&path).unwrap();
        check(&opened);
        let _ = std::fs::remove_file(&path);
    }
}
