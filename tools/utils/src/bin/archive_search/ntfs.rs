use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::Path;

const ATTR_STANDARD_INFORMATION: u32 = 0x10;
const ATTR_FILE_NAME: u32 = 0x30;
const ATTR_VOLUME_NAME: u32 = 0x60;
const ATTR_DATA: u32 = 0x80;
const ATTR_END: u32 = 0xFFFF_FFFF;
const INDEX_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

fn filetime_to_unix(filetime: u64) -> i64 {
    if filetime == 0 {
        0
    } else {
        (filetime / 10_000_000) as i64 - 11_644_473_600
    }
}

fn u16le(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

fn u32le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

fn u64le(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes([
        b[o],
        b[o + 1],
        b[o + 2],
        b[o + 3],
        b[o + 4],
        b[o + 5],
        b[o + 6],
        b[o + 7],
    ])
}

struct Boot {
    bytes_per_sector: u64,
    cluster_size: u64,
    record_size: u64,
}

pub struct Run {
    pub lcn: Option<u64>,
    pub len: u64,
}

struct Segment {
    stream_start: u64,
    lcn: Option<u64>,
    len: u64,
}

struct Attr {
    typ: u32,
    non_resident: bool,
    name: String,
    content: Vec<u8>,
    runs: Vec<Run>,
    real_size: u64,
    compressed: bool,
    encrypted: bool,
}

pub struct FileEntry {
    pub index: u64,
    pub parent: u64,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub mtime: i64,
}

pub struct Volume {
    file: File,
    boot: Boot,
    mft_segs: Vec<Segment>,
    pub mft_record_count: u64,
}

fn parse_runlist(bytes: &[u8]) -> Vec<Run> {
    let mut runs: Vec<Run> = Vec::new();
    let mut pos = 0usize;
    let mut prev_lcn: i64 = 0;
    while pos < bytes.len() {
        let header = bytes[pos];
        pos += 1;
        if header == 0 {
            break;
        }
        let len_size = (header & 0x0F) as usize;
        let off_size = ((header >> 4) & 0x0F) as usize;
        if len_size == 0 || len_size > 8 || off_size > 8 || pos + len_size + off_size > bytes.len()
        {
            break;
        }
        let mut len: u64 = 0;
        for k in 0..len_size {
            len |= (bytes[pos + k] as u64) << (8 * k);
        }
        pos += len_size;
        let mut off: i64 = 0;
        if off_size > 0 {
            let mut raw: i64 = 0;
            for k in 0..off_size {
                raw |= (bytes[pos + k] as i64) << (8 * k);
            }
            let shift = 64 - 8 * off_size;
            off = (raw << shift) >> shift;
        }
        pos += off_size;
        let lcn = if off_size == 0 {
            None
        } else {
            prev_lcn += off;
            Some(prev_lcn as u64)
        };
        runs.push(Run { lcn, len });
    }
    runs
}

fn segments(runs: &[Run], cluster_size: u64) -> Vec<Segment> {
    let mut segs: Vec<Segment> = Vec::with_capacity(runs.len());
    let mut start: u64 = 0;
    for run in runs {
        let len = run.len * cluster_size;
        segs.push(Segment {
            stream_start: start,
            lcn: run.lcn,
            len,
        });
        start += len;
    }
    segs
}

fn read_segments(
    file: &File,
    cluster_size: u64,
    segs: &[Segment],
    offset: u64,
    len: u64,
) -> Result<Vec<u8>, String> {
    let mut out: Vec<u8> = Vec::with_capacity(len as usize);
    let mut cur = offset;
    let mut remaining = len;
    while remaining > 0 {
        let idx = match segs.binary_search_by(|s| s.stream_start.cmp(&cur)) {
            Ok(i) => i,
            Err(0) => return Err(format!("the stream carries no extent at {cur}")),
            Err(i) => i - 1,
        };
        let seg = &segs[idx];
        if cur >= seg.stream_start + seg.len {
            return Err(format!("the stream carries no extent at {cur}"));
        }
        let within = cur - seg.stream_start;
        let take = remaining.min(seg.len - within);
        match seg.lcn {
            Some(lcn) => {
                let device = lcn * cluster_size + within;
                let mut buf = vec![0u8; take as usize];
                file.read_exact_at(&mut buf, device)
                    .map_err(|e| format!("the extent at device offset {device} reads not: {e}"))?;
                out.extend_from_slice(&buf);
            }
            None => out.extend(std::iter::repeat(0u8).take(take as usize)),
        }
        cur += take;
        remaining -= take;
    }
    Ok(out)
}

fn apply_fixup(rec: &mut [u8], sector_size: usize) -> Result<(), String> {
    if rec.len() < 8 || sector_size == 0 {
        return Err("the record is shorter than its header".to_string());
    }
    let usa_off = u16le(rec, 0x04) as usize;
    let usa_count = u16le(rec, 0x06) as usize;
    if usa_count < 2 || usa_off + usa_count * 2 > rec.len() {
        return Err("the update sequence array lies outside the record".to_string());
    }
    let sectors = rec.len() / sector_size;
    for i in 0..usa_count - 1 {
        if i >= sectors {
            break;
        }
        let end = (i + 1) * sector_size;
        if end > rec.len() {
            break;
        }
        let src = usa_off + (i + 1) * 2;
        rec[end - 2] = rec[src];
        rec[end - 1] = rec[src + 1];
    }
    Ok(())
}

fn utf16le_to_string(bytes: &[u8]) -> String {
    let mut units: Vec<u16> = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    while i + 1 < bytes.len() {
        units.push(u16::from_le_bytes([bytes[i], bytes[i + 1]]));
        i += 2;
    }
    String::from_utf16_lossy(&units)
}

fn parse_attributes(rec: &[u8]) -> Vec<Attr> {
    let mut attrs: Vec<Attr> = Vec::new();
    if rec.len() < 0x18 {
        return attrs;
    }
    let mut off = u16le(rec, 0x14) as usize;
    while off + 0x18 <= rec.len() {
        let typ = u32le(rec, off);
        if typ == ATTR_END || typ == 0 {
            break;
        }
        let len = u32le(rec, off + 0x04) as usize;
        if len < 0x18 || off + len > rec.len() {
            break;
        }
        let non_resident = rec[off + 0x08] != 0;
        let name_len = rec[off + 0x09] as usize;
        let name_off = u16le(rec, off + 0x0A) as usize;
        let flags = u16le(rec, off + 0x0C);
        let name = if name_len > 0 && off + name_off + name_len * 2 <= rec.len() {
            utf16le_to_string(&rec[off + name_off..off + name_off + name_len * 2])
        } else {
            String::new()
        };
        let mut attr = Attr {
            typ,
            non_resident,
            name,
            content: Vec::new(),
            runs: Vec::new(),
            real_size: 0,
            compressed: flags & 0x0001 != 0,
            encrypted: flags & 0x4000 != 0,
        };
        if non_resident {
            let run_off = u16le(rec, off + 0x20) as usize;
            attr.real_size = u64le(rec, off + 0x30);
            if off + run_off < off + len {
                attr.runs = parse_runlist(&rec[off + run_off..off + len]);
            }
        } else {
            let content_size = u32le(rec, off + 0x10) as usize;
            let content_off = u16le(rec, off + 0x14) as usize;
            if off + content_off + content_size <= rec.len() {
                attr.content = rec[off + content_off..off + content_off + content_size].to_vec();
            }
        }
        attrs.push(attr);
        off += len;
    }
    attrs
}

fn namespace_priority(namespace: u8) -> Option<u8> {
    match namespace {
        3 => Some(3),
        1 => Some(2),
        0 => Some(1),
        2 => Some(0),
        _ => None,
    }
}

impl Volume {
    pub fn open(path: &Path) -> Result<Volume, String> {
        let file = File::open(path)
            .map_err(|e| format!("the volume {} reads not: {e}", path.display()))?;
        let mut boot_bytes = [0u8; 512];
        file.read_exact_at(&mut boot_bytes, 0)
            .map_err(|e| format!("the boot sector of {} reads not: {e}", path.display()))?;
        if &boot_bytes[3..11] != b"NTFS    " {
            return Err(format!(
                "{} carries no NTFS signature in the boot sector",
                path.display()
            ));
        }
        let bytes_per_sector = u16le(&boot_bytes, 0x0B) as u64;
        let sectors_per_cluster = boot_bytes[0x0D] as u64;
        let cluster_size = bytes_per_sector * sectors_per_cluster;
        if bytes_per_sector == 0 || cluster_size == 0 {
            return Err("the boot sector carries no sector geometry".to_string());
        }
        let mft_lcn = u64le(&boot_bytes, 0x30);
        let record_code = boot_bytes[0x40] as i8;
        let record_size = if record_code > 0 {
            record_code as u64 * cluster_size
        } else {
            1u64 << ((-record_code) as u32)
        };
        if record_size < 0x30 {
            return Err("the boot sector carries no usable MFT record size".to_string());
        }
        let mut rec0 = vec![0u8; record_size as usize];
        file.read_exact_at(&mut rec0, mft_lcn * cluster_size)
            .map_err(|e| format!("the $MFT record reads not: {e}"))?;
        apply_fixup(&mut rec0, bytes_per_sector as usize)?;
        if &rec0[0..4] != b"FILE" {
            return Err("the $MFT record carries no FILE signature".to_string());
        }
        let attrs = parse_attributes(&rec0);
        let data = attrs
            .iter()
            .find(|a| a.typ == ATTR_DATA && a.name.is_empty())
            .ok_or_else(|| "the $MFT record carries no unnamed $DATA attribute".to_string())?;
        if !data.non_resident {
            return Err("the $MFT $DATA attribute is resident".to_string());
        }
        let mft_segs = segments(&data.runs, cluster_size);
        let mft_record_count = data.real_size / record_size;
        Ok(Volume {
            file,
            boot: Boot {
                bytes_per_sector,
                cluster_size,
                record_size,
            },
            mft_segs,
            mft_record_count,
        })
    }

    fn read_record(&self, index: u64) -> Result<Vec<u8>, String> {
        let mut rec = read_segments(
            &self.file,
            self.boot.cluster_size,
            &self.mft_segs,
            index * self.boot.record_size,
            self.boot.record_size,
        )?;
        apply_fixup(&mut rec, self.boot.bytes_per_sector as usize)?;
        Ok(rec)
    }

    pub fn build_index(&self) -> (Vec<FileEntry>, u64) {
        let mut entries: Vec<FileEntry> = Vec::new();
        let mut unread: u64 = 0;
        let mut i = 0u64;
        while i < self.mft_record_count {
            let rec = match self.read_record(i) {
                Ok(r) => r,
                Err(_) => {
                    unread += 1;
                    i += 1;
                    continue;
                }
            };
            if rec.len() < 0x30 || &rec[0..4] != b"FILE" {
                i += 1;
                continue;
            }
            let flags = u16le(&rec, 0x16);
            if flags & 0x0001 == 0 {
                i += 1;
                continue;
            }
            let is_dir = flags & 0x0002 != 0;
            let attrs = parse_attributes(&rec);
            let mut best: Option<(u8, String, u64)> = None;
            let mut size: u64 = 0;
            let mut mtime: i64 = 0;
            for attr in &attrs {
                if attr.typ == ATTR_FILE_NAME && !attr.non_resident {
                    let c = &attr.content;
                    if c.len() < 0x42 {
                        continue;
                    }
                    let parent = u64le(c, 0) & INDEX_MASK;
                    let name_len = c[0x40] as usize;
                    let namespace = c[0x41];
                    if 0x42 + name_len * 2 > c.len() {
                        continue;
                    }
                    let name = utf16le_to_string(&c[0x42..0x42 + name_len * 2]);
                    let Some(prio) = namespace_priority(namespace) else {
                        continue;
                    };
                    if best.as_ref().map_or(true, |(p, _, _)| prio > *p) {
                        best = Some((prio, name, parent));
                    }
                } else if attr.typ == ATTR_DATA && attr.name.is_empty() {
                    size = if attr.non_resident {
                        attr.real_size
                    } else {
                        attr.content.len() as u64
                    };
                } else if attr.typ == ATTR_STANDARD_INFORMATION
                    && !attr.non_resident
                    && attr.content.len() >= 0x10
                {
                    mtime = filetime_to_unix(u64le(&attr.content, 0x08));
                }
            }
            if let Some((_, name, parent)) = best {
                entries.push(FileEntry {
                    index: i,
                    parent,
                    name,
                    is_dir,
                    size,
                    mtime,
                });
            }
            i += 1;
        }
        (entries, unread)
    }

    pub fn volume_label(&self) -> Option<String> {
        let rec = self.read_record(3).ok()?;
        let attrs = parse_attributes(&rec);
        attrs
            .iter()
            .find(|a| a.typ == ATTR_VOLUME_NAME && !a.non_resident)
            .map(|a| utf16le_to_string(&a.content))
    }

    pub fn read_data(&self, index: u64, cap: u64) -> Result<Vec<u8>, String> {
        let rec = self.read_record(index)?;
        let attrs = parse_attributes(&rec);
        let data = attrs
            .iter()
            .find(|a| a.typ == ATTR_DATA && a.name.is_empty())
            .ok_or_else(|| "the record carries no unnamed $DATA attribute".to_string())?;
        if data.encrypted {
            return Err("the $DATA attribute is encrypted".to_string());
        }
        if data.compressed {
            return Err("the $DATA attribute is compressed".to_string());
        }
        if data.non_resident {
            let to_read = data.real_size.min(cap);
            let segs = segments(&data.runs, self.boot.cluster_size);
            read_segments(&self.file, self.boot.cluster_size, &segs, 0, to_read)
        } else {
            let mut content = data.content.clone();
            content.truncate(cap as usize);
            Ok(content)
        }
    }
}

pub fn run_lines(
    device: &Path,
    keywords: &[String],
    limit: usize,
    max_mb: u64,
    content: bool,
) -> Vec<String> {
    let vol = match Volume::open(device) {
        Ok(v) => v,
        Err(msg) => return vec![msg],
    };
    let mut lines = Vec::new();
    if let Some(label) = vol.volume_label() {
        lines.push(format!("volume {}", label));
    }
    let (entries, unread) = vol.build_index();
    lines.push(format!(
        "mft records {} | entries {} | unread {}",
        vol.mft_record_count,
        entries.len(),
        unread
    ));
    let needle: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();
    let cap = max_mb * 1024 * 1024;
    let mut shown = 0usize;
    let mut reads = 0usize;
    for e in &entries {
        if e.is_dir {
            continue;
        }
        if shown >= limit {
            break;
        }
        let mut hit = needle.is_empty() || needle.iter().any(|n| e.name.to_lowercase().contains(n));
        if !hit && content && !needle.is_empty() && e.size <= cap && reads < 2000 {
            reads += 1;
            if let Ok(data) = vol.read_data(e.index, cap) {
                let text = String::from_utf8_lossy(&data).to_lowercase();
                hit = needle.iter().any(|n| text.contains(n));
            }
        }
        if hit {
            lines.push(format!(
                "file index={} parent={} size={} mtime={} {}",
                e.index, e.parent, e.size, e.mtime, e.name
            ));
            shown += 1;
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runlist_decodes_relative_runs() {
        let bytes = [0x11, 0x08, 0x10, 0x11, 0x04, 0x02, 0x00];
        let runs = parse_runlist(&bytes);
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0].lcn, Some(16));
        assert_eq!(runs[0].len, 8);
        assert_eq!(runs[1].lcn, Some(18));
        assert_eq!(runs[1].len, 4);
    }

    #[test]
    fn runlist_marks_sparse_runs() {
        let bytes = [0x01, 0x05, 0x00];
        let runs = parse_runlist(&bytes);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].lcn, None);
        assert_eq!(runs[0].len, 5);
    }

    #[test]
    fn fixup_restores_sector_tails() {
        let mut rec = vec![0u8; 1024];
        rec[0x04] = 0x30;
        rec[0x06] = 0x03;
        rec[0x30] = 0xAB;
        rec[0x31] = 0xCD;
        rec[0x32] = 0x11;
        rec[0x33] = 0x22;
        rec[0x34] = 0x33;
        rec[0x35] = 0x44;
        rec[510] = 0xAB;
        rec[511] = 0xCD;
        rec[1022] = 0xAB;
        rec[1023] = 0xCD;
        apply_fixup(&mut rec, 512).unwrap();
        assert_eq!(&rec[510..512], &[0x11, 0x22]);
        assert_eq!(&rec[1022..1024], &[0x33, 0x44]);
    }

    #[test]
    fn utf16_decodes_ascii_names() {
        let bytes = [b'$', 0, b'M', 0, b'F', 0, b'T', 0];
        assert_eq!(utf16le_to_string(&bytes), "$MFT");
    }

    #[test]
    fn namespace_prefers_win32_over_dos() {
        assert!(namespace_priority(1) > namespace_priority(2));
        assert!(namespace_priority(3) > namespace_priority(1));
    }
}
