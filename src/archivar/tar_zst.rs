pub const MAGIC: [u8; 4] = *b"TZM1";
pub const VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Member {
    pub name: String,
    pub size: u64,
    pub offset: u64,
    pub mtime: i64,
    pub order: u32,
    pub kind: u8,
}

impl Member {
    pub fn is_file(&self) -> bool {
        matches!(self.kind, b'0' | 0)
    }

    pub fn is_dir(&self) -> bool {
        self.kind == b'5'
    }

    pub fn kind_name(&self) -> &'static str {
        match self.kind {
            b'0' | 0 => "file",
            b'1' => "hardlink",
            b'2' => "symlink",
            b'3' => "chardev",
            b'4' => "blockdev",
            b'5' => "dir",
            b'6' => "fifo",
            b'7' => "contiguous",
            _ => "other",
        }
    }
}

pub fn encode(members: &[Member]) -> Option<Vec<u8>> {
    let count = u32::try_from(members.len()).ok()?;
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    for m in members {
        let name = m.name.as_bytes();
        let len = u16::try_from(name.len()).ok()?;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(name);
        out.extend_from_slice(&m.size.to_le_bytes());
        out.extend_from_slice(&m.offset.to_le_bytes());
        out.extend_from_slice(&m.mtime.to_le_bytes());
        out.extend_from_slice(&m.order.to_le_bytes());
        out.push(m.kind);
    }
    Some(out)
}

pub fn parse(bytes: &[u8]) -> Option<Vec<Member>> {
    if bytes.len() < 12 || bytes[0..4] != MAGIC {
        return None;
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
    if version != VERSION {
        return None;
    }
    let count = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let mut pos = 12usize;
    let mut members = Vec::with_capacity(count.min(1 << 16));
    for order in 0..count {
        let len = u16::from_le_bytes(bytes.get(pos..pos + 2)?.try_into().ok()?) as usize;
        pos += 2;
        let name = std::str::from_utf8(bytes.get(pos..pos + len)?)
            .ok()?
            .to_string();
        pos += len;
        let size = u64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
        pos += 8;
        let offset = u64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
        pos += 8;
        let mtime = i64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
        pos += 8;
        let stored_order = u32::from_le_bytes(bytes.get(pos..pos + 4)?.try_into().ok()?);
        pos += 4;
        if stored_order != order as u32 {
            return None;
        }
        let kind = *bytes.get(pos)?;
        pos += 1;
        members.push(Member {
            name,
            size,
            offset,
            mtime,
            order: stored_order,
            kind,
        });
    }
    if pos != bytes.len() {
        return None;
    }
    Some(members)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<Member>> {
    parse(bytes)
}

pub fn find<'a>(members: &'a [Member], needle: &str) -> Option<&'a Member> {
    members
        .iter()
        .find(|m| m.name == needle)
        .or_else(|| members.iter().find(|m| m.name.contains(needle)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Member> {
        vec![
            Member {
                name: "lp_dataset_patched".into(),
                size: 0,
                offset: 512,
                mtime: 1_770_753_600,
                order: 0,
                kind: b'5',
            },
            Member {
                name: "lp_dataset_patched/06/ph012_lc2_172.npy".into(),
                size: 131_200,
                offset: 1536,
                mtime: 1_770_753_600,
                order: 1,
                kind: b'0',
            },
        ]
    }

    #[test]
    fn roundtrip_preserves_members() {
        let members = sample();
        let bin = encode(&members).expect("the manifest encodes");
        let back = parse(&bin).expect("the manifest parses");
        assert_eq!(back, members);
        assert!(back[0].is_dir());
        assert!(back[1].is_file());
        assert_eq!(back[1].kind_name(), "file");
    }

    #[test]
    fn rejects_foreign_and_truncated_manifests() {
        assert!(parse(b"").is_none());
        assert!(parse(b"ZIPT\x01\x00\x00\x00").is_none());
        let bin = encode(&sample()).unwrap();
        assert!(parse(&bin[..bin.len() - 1]).is_none());
        let mut bad_order = bin.clone();
        let last = bad_order.len() - 1;
        bad_order[last - 4] ^= 0xff;
        assert!(parse(&bad_order).is_none());
    }

    #[test]
    fn find_takes_the_exact_name_first() {
        let members = sample();
        assert_eq!(
            find(&members, "lp_dataset_patched/06/ph012_lc2_172.npy").map(|m| m.order),
            Some(1)
        );
        assert_eq!(find(&members, "ph012").map(|m| m.order), Some(1));
        assert!(find(&members, "absent").is_none());
    }
}
