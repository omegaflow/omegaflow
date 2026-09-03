pub const EQUILIBRIUM_MAGIC: [u8; 4] = *b"JWE1";
pub const EQUILIBRIUM_HOST_BYTES: usize = 32;
pub const EQUILIBRIUM_OBSID_BYTES: usize = 64;
pub const EQUILIBRIUM_SPECIES_NAME_BYTES: usize = 8;
pub const EQUILIBRIUM_NSPECIES: usize = 16;

#[derive(Clone)]
pub struct EquilibriumRecord {
    pub host: String,
    pub obs_id: String,
    pub teq: f64,
    pub x: [f64; EQUILIBRIUM_NSPECIES],
}

pub fn species_names() -> Vec<String> {
    crate::thermochem::species()
        .into_iter()
        .map(|s| s.name.to_string())
        .collect()
}

fn push_padded(buf: &mut Vec<u8>, text: &str, len: usize) -> bool {
    if text.len() > len {
        return false;
    }
    let pad = [0u8; 256];
    buf.extend_from_slice(text.as_bytes());
    buf.extend_from_slice(&pad[..len - text.len()]);
    true
}

fn read_padded(bytes: &[u8], off: &mut usize, len: usize) -> Option<String> {
    let end = off.checked_add(len)?;
    if end > bytes.len() {
        return None;
    }
    let slice = &bytes[*off..end];
    *off = end;
    let cut = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
    match std::str::from_utf8(&slice[..cut]) {
        Ok(s) if !s.is_empty() => Some(s.to_string()),
        _ => None,
    }
}

pub fn write_equilibrium_bin(records: &[EquilibriumRecord]) -> Option<Vec<u8>> {
    let names = species_names();
    let mut out = Vec::with_capacity(
        140 + records.len()
            * (EQUILIBRIUM_HOST_BYTES + EQUILIBRIUM_OBSID_BYTES + 8 + EQUILIBRIUM_NSPECIES * 8),
    );
    out.extend_from_slice(&EQUILIBRIUM_MAGIC);
    out.extend_from_slice(&(EQUILIBRIUM_NSPECIES as u32).to_le_bytes());
    for name in &names {
        if name.len() > EQUILIBRIUM_SPECIES_NAME_BYTES {
            return None;
        }
        let pad = [0u8; EQUILIBRIUM_SPECIES_NAME_BYTES];
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&pad[..EQUILIBRIUM_SPECIES_NAME_BYTES - name.len()]);
    }
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        if !r.teq.is_finite() {
            return None;
        }
        if !push_padded(&mut out, &r.host, EQUILIBRIUM_HOST_BYTES) {
            return None;
        }
        if !push_padded(&mut out, &r.obs_id, EQUILIBRIUM_OBSID_BYTES) {
            return None;
        }
        out.extend_from_slice(&r.teq.to_le_bytes());
        for xi in r.x.iter() {
            if !xi.is_finite() {
                return None;
            }
            out.extend_from_slice(&xi.to_le_bytes());
        }
    }
    Some(out)
}

pub fn parse_equilibrium_bin(bytes: &[u8]) -> Option<Vec<EquilibriumRecord>> {
    if bytes.len() < 8 || bytes[0..4] != EQUILIBRIUM_MAGIC {
        return None;
    }
    let mut off = 4usize;
    let n_species = u32::from_le_bytes(bytes[off..off + 4].try_into().ok()?) as usize;
    off += 4;
    if n_species != EQUILIBRIUM_NSPECIES {
        return None;
    }
    let mut names = Vec::with_capacity(n_species);
    for _ in 0..n_species {
        names.push(read_padded(
            bytes,
            &mut off,
            EQUILIBRIUM_SPECIES_NAME_BYTES,
        )?);
    }
    let canonical = species_names();
    if names != canonical {
        return None;
    }
    if off + 4 > bytes.len() {
        return None;
    }
    let count = u32::from_le_bytes(bytes[off..off + 4].try_into().ok()?) as usize;
    off += 4;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        let host = read_padded(bytes, &mut off, EQUILIBRIUM_HOST_BYTES)?;
        let obs_id = read_padded(bytes, &mut off, EQUILIBRIUM_OBSID_BYTES)?;
        if off + 8 + EQUILIBRIUM_NSPECIES * 8 > bytes.len() {
            return None;
        }
        let teq = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
        off += 8;
        if !teq.is_finite() {
            return None;
        }
        let mut x = [0.0f64; EQUILIBRIUM_NSPECIES];
        for slot in x.iter_mut() {
            let v = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
            off += 8;
            if !v.is_finite() {
                return None;
            }
            *slot = v;
        }
        records.push(EquilibriumRecord {
            host,
            obs_id,
            teq,
            x,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(host: &str, obs_id: &str) -> EquilibriumRecord {
        let mut x = [0.0f64; EQUILIBRIUM_NSPECIES];
        x[0] = 0.98;
        x[7] = 0.02;
        EquilibriumRecord {
            host: host.to_string(),
            obs_id: obs_id.to_string(),
            teq: 1166.0,
            x,
        }
    }

    #[test]
    fn equilibrium_bin_roundtrip() {
        let records = vec![
            record("WASP-39", "jw01366-o001_t001_niriss"),
            record("K2-18", "jw02722"),
        ];
        let bytes = write_equilibrium_bin(&records).unwrap();
        let parsed = parse_equilibrium_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].host, "WASP-39");
        assert_eq!(parsed[0].obs_id, "jw01366-o001_t001_niriss");
        assert_eq!(parsed[0].teq, 1166.0);
        assert_eq!(parsed[0].x, records[0].x);
        assert_eq!(parsed[1].x, records[1].x);
    }

    #[test]
    fn equilibrium_bin_refuses_malformed() {
        assert!(parse_equilibrium_bin(&[0xCF, 0x86]).is_none());
        assert!(parse_equilibrium_bin(b"JWE").is_none());
        let records = vec![record("WASP-39", "obs")];
        let bytes = write_equilibrium_bin(&records).unwrap();
        assert!(parse_equilibrium_bin(&bytes[..bytes.len() - 1]).is_none());
        let mut truncated = bytes.clone();
        truncated[4..8].copy_from_slice(&(99u32).to_le_bytes());
        assert!(parse_equilibrium_bin(&truncated).is_none());
    }

    #[test]
    fn equilibrium_bin_refuses_overlong_names() {
        let mut r = record("WASP-39", "obs");
        r.host = "x".repeat(EQUILIBRIUM_HOST_BYTES + 1);
        assert!(write_equilibrium_bin(&[r]).is_none());
        let mut r2 = record("WASP-39", "obs");
        r2.obs_id = "y".repeat(EQUILIBRIUM_OBSID_BYTES + 1);
        assert!(write_equilibrium_bin(&[r2]).is_none());
    }

    #[test]
    fn equilibrium_bin_refuses_nonfinite() {
        let mut r = record("WASP-39", "obs");
        r.teq = f64::NAN;
        assert!(write_equilibrium_bin(&[r]).is_none());
        let mut r2 = record("WASP-39", "obs");
        r2.x[3] = f64::INFINITY;
        assert!(write_equilibrium_bin(&[r2]).is_none());
    }
}
