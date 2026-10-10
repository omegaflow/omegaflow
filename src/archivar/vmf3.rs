pub const MAGIC: [u8; 4] = *b"VMF3";
pub const GRID_MAGIC: [u8; 4] = *b"VMFG";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 84;
pub const GRID_RECORD_BYTES: usize = 48;

const STATION_BYTES: usize = 16;

const P_PRESSURE: u32 = 1 << 0;
const P_TEMP: u32 = 1 << 1;
const P_E: u32 = 1 << 2;

#[derive(Clone, Debug, PartialEq)]
pub struct SiteMf {
    pub station: String,
    pub mjd: f64,
    pub ah: f64,
    pub aw: f64,
    pub zhd_m: f64,
    pub zwd_m: f64,
    pub pressure_hpa: Option<f64>,
    pub temp_c: Option<f64>,
    pub e_hpa: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridCell {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub ah: f64,
    pub aw: f64,
    pub zhd_m: f64,
    pub zwd_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridHeader {
    pub epoch_year: i32,
    pub epoch_month: u32,
    pub epoch_day: u32,
    pub epoch_hour: u32,
    pub epoch_minute: u32,
    pub epoch_second: f64,
    pub scale_factor: f64,
}

fn req_f64(tok: &str) -> Option<f64> {
    let v: f64 = tok.parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn opt_f64(tok: &str) -> Option<f64> {
    req_f64(tok)
}

pub fn parse_site(bytes: &[u8]) -> Option<Vec<SiteMf>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut out: Vec<SiteMf> = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('!') {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 6 {
            continue;
        }
        let station = toks[0];
        if station.is_empty() || station.parse::<f64>().is_ok() {
            continue;
        }
        let (Some(mjd), Some(ah), Some(aw), Some(zhd_m), Some(zwd_m)) = (
            req_f64(toks[1]),
            req_f64(toks[2]),
            req_f64(toks[3]),
            req_f64(toks[4]),
            req_f64(toks[5]),
        ) else {
            continue;
        };
        out.push(SiteMf {
            station: station.to_string(),
            mjd,
            ah,
            aw,
            zhd_m,
            zwd_m,
            pressure_hpa: toks.get(6).and_then(|t| opt_f64(t)),
            temp_c: toks.get(7).and_then(|t| opt_f64(t)),
            e_hpa: toks.get(8).and_then(|t| opt_f64(t)),
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_grid(bytes: &[u8]) -> Option<Vec<GridCell>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut out: Vec<GridCell> = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('!') {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 6 {
            continue;
        }
        let (Some(lat_deg), Some(lon_deg), Some(ah), Some(aw), Some(zhd_m), Some(zwd_m)) = (
            req_f64(toks[0]),
            req_f64(toks[1]),
            req_f64(toks[2]),
            req_f64(toks[3]),
            req_f64(toks[4]),
            req_f64(toks[5]),
        ) else {
            continue;
        };
        out.push(GridCell {
            lat_deg,
            lon_deg,
            ah,
            aw,
            zhd_m,
            zwd_m,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_grid_header(bytes: &[u8]) -> Option<GridHeader> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut epoch: Option<(i32, u32, u32, u32, u32, f64)> = None;
    let mut scale_factor = 1.0f64;
    for raw in text.lines() {
        let line = raw.trim();
        if !line.starts_with('!') {
            continue;
        }
        let rest = line.trim_start_matches('!').trim();
        if let Some(v) = rest.strip_prefix("Epoch:") {
            let toks: Vec<&str> = v.split_whitespace().collect();
            if toks.len() >= 6 {
                let y = toks[0].parse::<i32>().ok();
                let mo = toks[1].parse::<u32>().ok();
                let d = toks[2].parse::<u32>().ok();
                let h = toks[3].parse::<u32>().ok();
                let mi = toks[4].parse::<u32>().ok();
                let s = toks[5].parse::<f64>().ok();
                if let (Some(y), Some(mo), Some(d), Some(h), Some(mi), Some(s)) =
                    (y, mo, d, h, mi, s)
                {
                    if s.is_finite() {
                        epoch = Some((y, mo, d, h, mi, s));
                    }
                }
            }
        } else if let Some(v) = rest.strip_prefix("Scale_factor:") {
            if let Some(x) = v.split_whitespace().next().and_then(req_f64) {
                scale_factor = x;
            }
        }
    }
    let (epoch_year, epoch_month, epoch_day, epoch_hour, epoch_minute, epoch_second) = epoch?;
    Some(GridHeader {
        epoch_year,
        epoch_month,
        epoch_day,
        epoch_hour,
        epoch_minute,
        epoch_second,
        scale_factor,
    })
}

fn put_f64(out: &mut Vec<u8>, v: Option<f64>, bit: u32, present: &mut u32) -> bool {
    match v {
        Some(x) => {
            if !x.is_finite() {
                return false;
            }
            *present |= bit;
            out.extend_from_slice(&x.to_le_bytes());
        }
        None => out.extend_from_slice(&0.0f64.to_le_bytes()),
    }
    true
}

fn put_station(out: &mut Vec<u8>, name: &str) -> bool {
    if !name.is_ascii() || name.len() > STATION_BYTES {
        return false;
    }
    let start = out.len();
    out.resize(start + STATION_BYTES, 0);
    out[start..start + name.len()].copy_from_slice(name.as_bytes());
    true
}

pub fn write_bin(records: &[SiteMf]) -> Option<Vec<u8>> {
    if records.is_empty() {
        return None;
    }
    let count = u32::try_from(records.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        if !put_station(&mut out, &r.station) {
            return None;
        }
        for v in [r.mjd, r.ah, r.aw, r.zhd_m, r.zwd_m] {
            if !v.is_finite() {
                return None;
            }
            out.extend_from_slice(&v.to_le_bytes());
        }
        let mut present = 0u32;
        if !put_f64(&mut out, r.pressure_hpa, P_PRESSURE, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.temp_c, P_TEMP, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.e_hpa, P_E, &mut present) {
            return None;
        }
        out.extend_from_slice(&present.to_le_bytes());
    }
    Some(out)
}

pub fn write_grid_bin(cells: &[GridCell]) -> Option<Vec<u8>> {
    if cells.is_empty() {
        return None;
    }
    let count = u32::try_from(cells.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + cells.len() * GRID_RECORD_BYTES);
    out.extend_from_slice(&GRID_MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for c in cells {
        for v in [c.lat_deg, c.lon_deg, c.ah, c.aw, c.zhd_m, c.zwd_m] {
            if !v.is_finite() {
                return None;
            }
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    Some(out)
}

struct Reader<'a> {
    bytes: &'a [u8],
    off: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let v = self.bytes.get(self.off..self.off.checked_add(n)?)?;
        self.off += n;
        Some(v)
    }

    fn f64(&mut self) -> Option<f64> {
        let v = f64::from_le_bytes(self.take(8)?.try_into().ok()?);
        Some(v)
    }

    fn u32(&mut self) -> Option<u32> {
        let v = u32::from_le_bytes(self.take(4)?.try_into().ok()?);
        Some(v)
    }
}

fn present_f64(raw: f64, present: u32, bit: u32) -> Option<Option<f64>> {
    if present & bit != 0 {
        if raw.is_finite() {
            Some(Some(raw))
        } else {
            None
        }
    } else if raw == 0.0 {
        Some(None)
    } else {
        None
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<SiteMf>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(RECORD_BYTES)? {
        return None;
    }
    let mut rd = Reader {
        bytes,
        off: HEADER_BYTES,
    };
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let station = std::str::from_utf8(rd.take(STATION_BYTES)?)
            .ok()?
            .trim_end_matches('\0')
            .to_string();
        let mjd = rd.f64()?;
        let ah = rd.f64()?;
        let aw = rd.f64()?;
        let zhd_m = rd.f64()?;
        let zwd_m = rd.f64()?;
        let pressure_raw = rd.f64()?;
        let temp_raw = rd.f64()?;
        let e_raw = rd.f64()?;
        let present = rd.u32()?;
        for v in [mjd, ah, aw, zhd_m, zwd_m] {
            if !v.is_finite() {
                return None;
            }
        }
        out.push(SiteMf {
            station,
            mjd,
            ah,
            aw,
            zhd_m,
            zwd_m,
            pressure_hpa: present_f64(pressure_raw, present, P_PRESSURE)?,
            temp_c: present_f64(temp_raw, present, P_TEMP)?,
            e_hpa: present_f64(e_raw, present, P_E)?,
        });
    }
    Some(out)
}

pub fn parse_grid_bin(bytes: &[u8]) -> Option<Vec<GridCell>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != GRID_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(GRID_RECORD_BYTES)? {
        return None;
    }
    let mut rd = Reader {
        bytes,
        off: HEADER_BYTES,
    };
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let lat_deg = rd.f64()?;
        let lon_deg = rd.f64()?;
        let ah = rd.f64()?;
        let aw = rd.f64()?;
        let zhd_m = rd.f64()?;
        let zwd_m = rd.f64()?;
        for v in [lat_deg, lon_deg, ah, aw, zhd_m, zwd_m] {
            if !v.is_finite() {
                return None;
            }
        }
        out.push(GridCell {
            lat_deg,
            lon_deg,
            ah,
            aw,
            zhd_m,
            zwd_m,
        });
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::sha256::sha256_hex;

    const SITE_SAMPLE: &str = "AGGO      61042.00  0.00126424  0.00043626  2.2940  0.1199  1005.73  22.79  16.62\n\
AIRA      61042.00  0.00121746  0.00041399  2.2444  0.0500   983.52   1.96   5.46\n";

    const GRID_SAMPLE: &str = "! Version:            1.0\n\
! Epoch:              2026 01 01 00 00  0.0\n\
! Scale_factor:       1.e+00\n\
 89.5    0.5  0.00114741  0.00069930  2.2862  0.0158\n\
 89.5    1.5  0.00114740  0.00069526  2.2861  0.0158\n";

    const SITE_PATH: &str = "/tmp/opencode/2026002.vmf3_r";
    const GRID_PATH: &str = "/tmp/opencode/VMF3_20260101.H00";
    const SITE_SHA: &str = "16397c37eaeaf55abcfe72b4d5a0449b08d8be3201d1e266c731f110f87c91f8";

    #[test]
    fn site_sample_parses_and_roundtrips() {
        let rows = parse_site(SITE_SAMPLE.as_bytes()).expect("the site sample parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].station, "AGGO");
        assert_eq!(rows[0].mjd, 61042.0);
        assert_eq!(rows[0].ah, 0.00126424);
        assert_eq!(rows[0].aw, 0.00043626);
        assert_eq!(rows[0].zhd_m, 2.2940);
        assert_eq!(rows[0].zwd_m, 0.1199);
        assert_eq!(rows[0].pressure_hpa, Some(1005.73));
        assert_eq!(rows[0].temp_c, Some(22.79));
        assert_eq!(rows[0].e_hpa, Some(16.62));
        let bin = write_bin(&rows).expect("finite site rows encode");
        assert_eq!(bin.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bin), Some(rows));
    }

    #[test]
    fn grid_sample_parses_and_roundtrips() {
        let header = parse_grid_header(GRID_SAMPLE.as_bytes()).expect("the grid header parses");
        assert_eq!(header.epoch_year, 2026);
        assert_eq!(header.epoch_month, 1);
        assert_eq!(header.epoch_day, 1);
        assert_eq!(header.epoch_hour, 0);
        assert_eq!(header.epoch_minute, 0);
        assert_eq!(header.epoch_second, 0.0);
        assert_eq!(header.scale_factor, 1.0);
        let cells = parse_grid(GRID_SAMPLE.as_bytes()).expect("the grid sample parses");
        assert_eq!(cells.len(), 2);
        assert_eq!(
            cells[0],
            GridCell {
                lat_deg: 89.5,
                lon_deg: 0.5,
                ah: 0.00114741,
                aw: 0.00069930,
                zhd_m: 2.2862,
                zwd_m: 0.0158,
            }
        );
        let bin = write_grid_bin(&cells).expect("finite grid cells encode");
        assert_eq!(bin.len(), HEADER_BYTES + 2 * GRID_RECORD_BYTES);
        assert_eq!(parse_grid_bin(&bin), Some(cells));
    }

    #[test]
    fn site_missing_trailing_columns_stays_absent() {
        let rows = parse_site(b"AGGO      61042.00  0.00126424  0.00043626  2.2940  0.1199\n")
            .expect("the required six columns parse");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].pressure_hpa, None);
        assert_eq!(rows[0].temp_c, None);
        assert_eq!(rows[0].e_hpa, None);
        let bin = write_bin(&rows).expect("finite site row encodes");
        assert_eq!(parse_bin(&bin), Some(rows));
    }

    #[test]
    fn parse_refuses_foreign_bytes() {
        assert_eq!(parse_site(b"not a vmf3 site file"), None);
        assert_eq!(parse_site(b""), None);
        assert_eq!(parse_grid(b"! only a header\n"), None);
        assert_eq!(parse_grid(b""), None);
        assert_eq!(parse_grid_header(b"! Version: 1.0\n"), None);
    }

    #[test]
    fn parse_bin_refuses_foreign_and_truncated_bytes() {
        assert_eq!(parse_bin(b"VMF3"), None);
        assert_eq!(parse_bin(b"XXXX\x01\x00\x00\x00"), None);
        assert_eq!(parse_grid_bin(b"VMF3\x01\x00\x00\x00"), None);
        let rows = parse_site(SITE_SAMPLE.as_bytes()).expect("the site sample parses");
        let bin = write_bin(&rows).expect("finite site rows encode");
        assert_eq!(parse_bin(&bin[..bin.len() - 1]), None);
    }

    #[test]
    fn site_file_carries_the_measured_rows() {
        let Ok(bytes) = std::fs::read(SITE_PATH) else {
            eprintln!("site_file_carries_the_measured_rows: {SITE_PATH} absent — named skip");
            return;
        };
        assert_eq!(sha256_hex(&bytes), SITE_SHA);
        let rows = parse_site(&bytes).expect("the site file parses");
        assert_eq!(rows.len(), 1052);
        let aggo = rows
            .iter()
            .find(|r| r.station == "AGGO")
            .expect("AGGO stands");
        assert_eq!(aggo.mjd, 61042.0);
        assert_eq!(aggo.ah, 0.00126424);
        assert_eq!(aggo.aw, 0.00043626);
        assert_eq!(aggo.zhd_m, 2.2940);
        assert_eq!(aggo.zwd_m, 0.1199);
        assert_eq!(aggo.pressure_hpa, Some(1005.73));
        assert_eq!(aggo.temp_c, Some(22.79));
        assert_eq!(aggo.e_hpa, Some(16.62));
        let bin = write_bin(&rows).expect("finite site rows encode");
        assert_eq!(parse_bin(&bin), Some(rows));
    }

    #[test]
    fn grid_file_carries_the_measured_rows() {
        let Ok(bytes) = std::fs::read(GRID_PATH) else {
            eprintln!("grid_file_carries_the_measured_rows: {GRID_PATH} absent — named skip");
            return;
        };
        let cells = parse_grid(&bytes).expect("the grid file parses");
        assert_eq!(cells.len(), 64800);
        assert_eq!(
            cells[0],
            GridCell {
                lat_deg: 89.5,
                lon_deg: 0.5,
                ah: 0.00114741,
                aw: 0.00069930,
                zhd_m: 2.2862,
                zwd_m: 0.0158,
            }
        );
        let bin = write_grid_bin(&cells).expect("finite grid cells encode");
        assert_eq!(bin.len(), HEADER_BYTES + 64800 * GRID_RECORD_BYTES);
        assert_eq!(parse_grid_bin(&bin), Some(cells));
    }
}
