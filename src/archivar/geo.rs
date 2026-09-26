pub const MAGIC_BGR: [u8; 4] = *b"BGR1";
pub const MAGIC_NRS: [u8; 4] = *b"NRS1";
pub const MAGIC_SDARN: [u8; 4] = *b"SDN1";
pub const MAGIC_SDRAW: [u8; 4] = *b"SDN2";
pub const MAGIC_ARGO: [u8; 4] = *b"ARG1";
pub const MAGIC_FDSN: [u8; 4] = *b"FDS1";
pub const MAGIC_GIC: [u8; 4] = *b"GIC1";
pub const MAGIC_IGETS: [u8; 4] = *b"IGT1";
pub const MAGIC_GBCO: [u8; 4] = *b"GBCO";
pub const MAGIC_SLB2: [u8; 4] = *b"SLB2";
pub const MAGIC_OCS: [u8; 4] = *b"OCS1";
pub const MAGIC_GMR: [u8; 4] = *b"GMR1";
pub const MAGIC_ISSLIS: [u8; 4] = *b"ISL1";
pub const MAGIC_LISOTD: [u8; 4] = *b"LOT1";
pub const MAGIC_TRMMLIS: [u8; 4] = *b"TRL1";
pub const MAGIC_GLML1B: [u8; 4] = *b"GLM1";
pub const MAGIC_GLML2: [u8; 4] = *b"GLM2";
pub const MAGIC_SMG: [u8; 4] = *b"SMG1";
pub const MAGIC_GHCN: [u8; 4] = *b"GHC1";
pub const MAGIC_GSOD: [u8; 4] = *b"GSD1";
pub const MAGIC_ISD: [u8; 4] = *b"ISD1";
pub const MAGIC_CDM: [u8; 4] = *b"CDM1";
pub const MAGIC_DCDB: [u8; 4] = *b"DCD1";
pub const MAGIC_KEO: [u8; 4] = *b"KEO1";
pub const MAGIC_WOD: [u8; 4] = *b"WOD1";
pub const MAGIC_HINET: [u8; 4] = *b"HNT1";
pub const MAGIC_COSMIC: [u8; 4] = *b"CSM1";
pub const MAGIC_ONC: [u8; 4] = *b"ONC1";
pub const MAGIC_NXR: [u8; 4] = *b"NXR1";
pub const MAGIC_USCRN: [u8; 4] = *b"USC1";
pub const MAGIC_GDP: [u8; 4] = *b"GDPT";
pub const MAGIC_CHAMP: [u8; 4] = *b"CHP1";
pub const MAGIC_LAS: [u8; 4] = *b"LAS1";
pub const MAGIC_KYOTO: [u8; 4] = *b"KYO1";
pub const MAGIC_HFR: [u8; 4] = *b"HFR1";
pub const MAGIC_OSM: [u8; 4] = *b"OSM1";
pub const MAGIC_TOAR: [u8; 4] = *b"TOA1";
pub const MAGIC_OGM: [u8; 4] = *b"OGM1";
pub const MAGIC_NOHR: [u8; 4] = *b"NOH1";
pub const MAGIC_DECAPS: [u8; 4] = crate::decaps::MAGIC;

pub const REC_BYTES: usize = 60;
pub const GBCO_REC_BYTES: usize = 24;
pub const SMG_REC_BYTES: usize = 64;

pub const COMP_BGR_AZIM: u32 = 1;
pub const COMP_BGR_VAPP: u32 = 2;
pub const COMP_BGR_RMS: u32 = 3;
pub const COMP_BGR_FREQ: u32 = 4;
pub const COMP_BGR_MAX: u32 = 4;

pub const COMP_NRS_PSD: u32 = 1;

pub const COMP_SDARN_V: u32 = 1;
pub const COMP_SDARN_POWER: u32 = 2;

pub const COMP_FDSN_BHZ: u32 = 1;
pub const COMP_FDSN_MAX: u32 = 1;

pub const COMP_GIC_A: u32 = 1;
pub const COMP_GIC_MAX: u32 = 1;

pub const COMP_IGETS_G: u32 = 1;
pub const COMP_IGETS_MAX: u32 = 1;

pub const COMP_ISSLIS_FLASH_RAD: u32 = 1;
pub const COMP_ISSLIS_MAX: u32 = 1;

pub const COMP_LISOTD_FLASH_RAD: u32 = 1;
pub const COMP_LISOTD_MAX: u32 = 1;

pub const COMP_TRMMLIS_FLASH_RAD: u32 = 1;
pub const COMP_TRMMLIS_MAX: u32 = 1;

pub const COMP_GLML1B_FLASH_ENERGY: u32 = 1;
pub const COMP_GLML1B_MAX: u32 = 1;

pub const COMP_GLML2_FLASH_ENERGY: u32 = 1;
pub const COMP_GLML2_MAX: u32 = 1;

pub const COMP_ARGO_DOXY: u32 = 1;
pub const COMP_ARGO_NITRATE: u32 = 2;
pub const COMP_ARGO_CHLA: u32 = 3;
pub const COMP_ARGO_BBP700: u32 = 4;
pub const COMP_ARGO_PH_TOTAL: u32 = 5;
pub const COMP_ARGO_MAX: u32 = 5;

pub const COMP_SMG_N_NEZ: u32 = 1;
pub const COMP_SMG_E_NEZ: u32 = 2;
pub const COMP_SMG_Z_NEZ: u32 = 3;
pub const COMP_SMG_N_GEO: u32 = 4;
pub const COMP_SMG_E_GEO: u32 = 5;
pub const COMP_SMG_Z_GEO: u32 = 6;
pub const COMP_SMG_MAX: u32 = 6;

pub const COMP_GHCN_TMAX: u32 = 1;
pub const COMP_GHCN_TMIN: u32 = 2;
pub const COMP_GHCN_PRCP: u32 = 3;
pub const COMP_GHCN_SNOW: u32 = 4;
pub const COMP_GHCN_SNWD: u32 = 5;
pub const COMP_GHCN_MAX: u32 = 5;

pub const COMP_GSOD_TEMP: u32 = 1;
pub const COMP_GSOD_DEWP: u32 = 2;
pub const COMP_GSOD_SLP: u32 = 3;
pub const COMP_GSOD_WDSP: u32 = 4;
pub const COMP_GSOD_GUST: u32 = 5;
pub const COMP_GSOD_TMAX: u32 = 6;
pub const COMP_GSOD_TMIN: u32 = 7;
pub const COMP_GSOD_PRCP: u32 = 8;
pub const COMP_GSOD_MAX: u32 = 8;

pub const COMP_ISD_TEMP: u32 = 1;
pub const COMP_ISD_DEWP: u32 = 2;
pub const COMP_ISD_SLP: u32 = 3;
pub const COMP_ISD_WDIR: u32 = 4;
pub const COMP_ISD_WSPD: u32 = 5;
pub const COMP_ISD_MAX: u32 = 5;

pub const COMP_CDM_MAX: u32 = crate::copernicus::VARIABLE_COUNT;

pub const COMP_DCDB_DEPTH: u32 = 1;
pub const COMP_DCDB_MAX: u32 = 1;

pub const COMP_KEO_TEMP: u32 = 1;
pub const COMP_KEO_PSAL: u32 = 2;
pub const COMP_KEO_UCUR: u32 = 3;
pub const COMP_KEO_VCUR: u32 = 4;
pub const COMP_KEO_MAX: u32 = 4;

pub const COMP_WOD_TEMP: u32 = 1;
pub const COMP_WOD_PSAL: u32 = 2;
pub const COMP_WOD_DOXY: u32 = 3;
pub const COMP_WOD_MAX: u32 = 3;

pub const COMP_HINET_U: u32 = 1;
pub const COMP_HINET_E: u32 = 2;
pub const COMP_HINET_N: u32 = 3;
pub const COMP_HINET_MAX: u32 = 3;

pub const COMP_COSMIC_REFRACT: u32 = 1;
pub const COMP_COSMIC_TEMP: u32 = 2;
pub const COMP_COSMIC_PRES: u32 = 3;
pub const COMP_COSMIC_MAX: u32 = 3;
pub const COMP_ONC_PSD: u32 = 1;
pub const COMP_ONC_MAX: u32 = 1;

pub const COMP_NXR_REF: u32 = 1;
pub const COMP_NXR_VEL: u32 = 2;
pub const COMP_NXR_SW: u32 = 3;
pub const COMP_NXR_MAX: u32 = 3;

pub const COMP_USCRN_TEMP: u32 = 1;
pub const COMP_USCRN_MAX: u32 = 1;

pub const COMP_OSM_TEMP: u32 = 1;
pub const COMP_OSM_MAX: u32 = 1;

pub const COMP_CHAMP_DENS: u32 = 1;

pub const COMP_LAS_X: u32 = 1;
pub const COMP_LAS_Y: u32 = 2;
pub const COMP_LAS_Z: u32 = 3;
pub const COMP_LAS_INTENSITY: u32 = 4;
pub const COMP_LAS_CLASSIFICATION: u32 = 5;
pub const COMP_LAS_MAX: u32 = 5;

pub const COMP_KYOTO_PRESSURE: u32 = 1;
pub const COMP_KYOTO_MAX: u32 = 1;

pub const COMP_HFR_U: u32 = 1;
pub const COMP_HFR_V: u32 = 2;
pub const COMP_HFR_MAX: u32 = 2;

pub const COMP_TOAR_O3: u32 = 1;
pub const COMP_TOAR_MAX: u32 = 1;

pub const COMP_OGM_TEMP: u32 = 1;
pub const COMP_OGM_DEWP: u32 = 2;
pub const COMP_OGM_WSPD: u32 = 3;
pub const COMP_OGM_SLP: u32 = 4;
pub const COMP_OGM_MAX: u32 = 4;

pub const COMP_NOHR_SNOWFALL: u32 = 1;
pub const COMP_NOHR_MAX: u32 = 1;

pub struct GeoRec {
    pub t: f64,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub freq: f64,
    pub bin_width: f64,
    pub val: f64,
    pub comp: u32,
    pub station: u32,
}

pub struct GbcoRec {
    pub lat: f64,
    pub lon: f64,
    pub elev: f64,
}

pub fn magic_of(format: &str) -> Option<[u8; 4]> {
    match format {
        "bgr_infrasound" => Some(MAGIC_BGR),
        "noaa_nrs_psd" => Some(MAGIC_NRS),
        "superdarn_fitacf" => Some(MAGIC_SDARN),
        "superdarn_rawacf" => Some(MAGIC_SDRAW),
        "argo_bgc" => Some(MAGIC_ARGO),
        "fdsn_waveform" => Some(MAGIC_FDSN),
        "fmi_gic" => Some(MAGIC_GIC),
        "igets" => Some(MAGIC_IGETS),
        "hinet" => Some(MAGIC_HINET),
        "iss_lis" => Some(MAGIC_ISSLIS),
        "lis_otd" => Some(MAGIC_LISOTD),
        "trmm_lis" => Some(MAGIC_TRMMLIS),
        "glm_l1b" => Some(MAGIC_GLML1B),
        "glm_l2" => Some(MAGIC_GLML2),
        "supermag_1m" => Some(MAGIC_SMG),
        "noaa_ghcn_d" => Some(MAGIC_GHCN),
        "noaa_gsod" => Some(MAGIC_GSOD),
        "noaa_isd" => Some(MAGIC_ISD),
        "copernicus_cdm_obs" => Some(MAGIC_CDM),
        "noaa_dcdb_bathymetry" => Some(MAGIC_DCDB),
        "noaa_keo_papa" => Some(MAGIC_KEO),
        "noaa_wod" => Some(MAGIC_WOD),
        "cosmic_ro" => Some(MAGIC_COSMIC),
        "onc_hydrophone_psd" => Some(MAGIC_ONC),
        "nexrad_level2" => Some(MAGIC_NXR),
        "us_crn_hourly" => Some(MAGIC_USCRN),
        "gdp_drifter" => Some(MAGIC_GDP),
        "champ_plpt" => Some(MAGIC_CHAMP),
        "las" => Some(MAGIC_LAS),
        "kyoto_pressure" => Some(MAGIC_KYOTO),
        "hfrnet_rtv" => Some(MAGIC_HFR),
        "emodnet_hfr" => Some(MAGIC_HFR),
        "opensensemap_temperatur" => Some(MAGIC_OSM),
        "toar_surface_o3" => Some(MAGIC_TOAR),
        "ogimet_synop" => Some(MAGIC_OGM),
        "nohrsc_snowfall" => Some(MAGIC_NOHR),
        "decaps_dr2_stars" => Some(MAGIC_DECAPS),
        _ => None,
    }
}

pub fn comp_max(format: &str) -> Option<u32> {
    match format {
        "bgr_infrasound" => Some(COMP_BGR_MAX),
        "noaa_nrs_psd" => Some(COMP_NRS_PSD),
        "superdarn_fitacf" => Some(COMP_SDARN_V),
        "superdarn_rawacf" => Some(COMP_SDARN_POWER),
        "argo_bgc" => Some(COMP_ARGO_MAX),
        "fdsn_waveform" => Some(COMP_FDSN_MAX),
        "fmi_gic" => Some(COMP_GIC_MAX),
        "igets" => Some(COMP_IGETS_MAX),
        "hinet" => Some(COMP_HINET_MAX),
        "iss_lis" => Some(COMP_ISSLIS_MAX),
        "lis_otd" => Some(COMP_LISOTD_MAX),
        "trmm_lis" => Some(COMP_TRMMLIS_MAX),
        "glm_l1b" => Some(COMP_GLML1B_MAX),
        "glm_l2" => Some(COMP_GLML2_MAX),
        "supermag_1m" => Some(COMP_SMG_MAX),
        "noaa_ghcn_d" => Some(COMP_GHCN_MAX),
        "noaa_gsod" => Some(COMP_GSOD_MAX),
        "noaa_isd" => Some(COMP_ISD_MAX),
        "copernicus_cdm_obs" => Some(COMP_CDM_MAX),
        "noaa_dcdb_bathymetry" => Some(COMP_DCDB_MAX),
        "noaa_keo_papa" => Some(COMP_KEO_MAX),
        "noaa_wod" => Some(COMP_WOD_MAX),
        "cosmic_ro" => Some(COMP_COSMIC_MAX),
        "onc_hydrophone_psd" => Some(COMP_ONC_MAX),
        "nexrad_level2" => Some(COMP_NXR_MAX),
        "us_crn_hourly" => Some(COMP_USCRN_MAX),
        "gdp_drifter" => Some(crate::gdp_drifter::COMP_SST),
        "champ_plpt" => Some(COMP_CHAMP_DENS),
        "las" => Some(COMP_LAS_MAX),
        "kyoto_pressure" => Some(COMP_KYOTO_MAX),
        "hfrnet_rtv" => Some(COMP_HFR_MAX),
        "emodnet_hfr" => Some(COMP_HFR_MAX),
        "opensensemap_temperatur" => Some(COMP_OSM_MAX),
        "toar_surface_o3" => Some(COMP_TOAR_MAX),
        "ogimet_synop" => Some(COMP_OGM_MAX),
        "nohrsc_snowfall" => Some(COMP_NOHR_MAX),
        "decaps_dr2_stars" => Some(crate::decaps::COMP_MAX),
        _ => None,
    }
}

pub fn pack_iaga(code: &str) -> Option<u32> {
    let b = code.as_bytes();
    if b.len() != 3
        || !b
            .iter()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    {
        return None;
    }
    Some((b[0] as u32) | (b[1] as u32) << 8 | (b[2] as u32) << 16)
}

pub fn iaga_of(station: u32) -> Option<String> {
    let c = |i: u32| ((station >> (8 * i)) & 0xff) as u8;
    let cs = [c(0), c(1), c(2)];
    if !cs
        .iter()
        .all(|x| x.is_ascii_uppercase() || x.is_ascii_digit())
    {
        return None;
    }
    String::from_utf8(cs.to_vec()).ok()
}

pub fn smg_record_bytes(r: &GeoRec) -> [u8; SMG_REC_BYTES] {
    let mut b = [0u8; SMG_REC_BYTES];
    b[0..8].copy_from_slice(&r.t.to_le_bytes());
    b[8..16].copy_from_slice(&r.lat.to_le_bytes());
    b[16..24].copy_from_slice(&r.lon.to_le_bytes());
    b[24..32].copy_from_slice(&r.alt.to_le_bytes());
    b[32..40].copy_from_slice(&r.freq.to_le_bytes());
    b[40..48].copy_from_slice(&r.bin_width.to_le_bytes());
    b[48..56].copy_from_slice(&r.val.to_le_bytes());
    b[56..60].copy_from_slice(&r.comp.to_le_bytes());
    b[60..64].copy_from_slice(&r.station.to_le_bytes());
    b
}

pub fn smg_record_at(bytes: &[u8], idx: usize) -> Option<GeoRec> {
    let start = idx.checked_mul(SMG_REC_BYTES)?;
    let s = bytes.get(start..start + SMG_REC_BYTES)?;
    let f64_of = |r: std::ops::Range<usize>| {
        s.get(r)
            .and_then(|x| x.try_into().ok())
            .map(f64::from_le_bytes)
    };
    Some(GeoRec {
        t: f64_of(0..8)?,
        lat: f64_of(8..16)?,
        lon: f64_of(16..24)?,
        alt: f64_of(24..32)?,
        freq: f64_of(32..40)?,
        bin_width: f64_of(40..48)?,
        val: f64_of(48..56)?,
        comp: u32::from_le_bytes(s.get(56..60)?.try_into().ok()?),
        station: u32::from_le_bytes(s.get(60..64)?.try_into().ok()?),
    })
}

pub fn write_bin(magic: [u8; 4], records: &[GeoRec]) -> Vec<u8> {
    let smg = magic == MAGIC_SMG;
    let rec = if smg { SMG_REC_BYTES } else { REC_BYTES };
    let mut buf = Vec::with_capacity(8 + records.len() * rec);
    buf.extend_from_slice(&magic);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        if smg {
            buf.extend_from_slice(&smg_record_bytes(r));
        } else {
            buf.extend_from_slice(&r.t.to_le_bytes());
            buf.extend_from_slice(&r.lat.to_le_bytes());
            buf.extend_from_slice(&r.lon.to_le_bytes());
            buf.extend_from_slice(&r.alt.to_le_bytes());
            buf.extend_from_slice(&r.freq.to_le_bytes());
            buf.extend_from_slice(&r.bin_width.to_le_bytes());
            buf.extend_from_slice(&r.val.to_le_bytes());
            buf.extend_from_slice(&r.comp.to_le_bytes());
        }
    }
    buf
}

pub fn parse_bin(magic: [u8; 4], bytes: &[u8]) -> Option<Vec<GeoRec>> {
    if bytes.len() < 8 || bytes[0..4] != magic {
        return None;
    }
    if magic == MAGIC_DECAPS {
        return None;
    }
    if magic == MAGIC_GDP {
        return crate::gdp_drifter::parse_bin(bytes).map(|r| crate::gdp_drifter::to_geo(&r));
    }
    let smg = magic == MAGIC_SMG;
    let rec = if smg { SMG_REC_BYTES } else { REC_BYTES };
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / rec {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let f64_of = |off: usize| {
            bytes
                .get(off..off + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let t = f64_of(off)?;
        off += 8;
        let lat = f64_of(off)?;
        off += 8;
        let lon = f64_of(off)?;
        off += 8;
        let alt = f64_of(off)?;
        off += 8;
        let freq = f64_of(off)?;
        off += 8;
        let bin_width = f64_of(off)?;
        off += 8;
        let val = f64_of(off)?;
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        let station = if smg {
            let s = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
            off += 4;
            s
        } else {
            0
        };
        out.push(GeoRec {
            t,
            lat,
            lon,
            alt,
            freq,
            bin_width,
            val,
            comp,
            station,
        });
    }
    Some(out)
}

pub fn verify_bin(magic: [u8; 4], bytes: &[u8]) -> Option<usize> {
    if bytes.len() < 8 || bytes[0..4] != magic {
        return None;
    }
    let smg = magic == MAGIC_SMG;
    let rec = if smg { SMG_REC_BYTES } else { REC_BYTES };
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / rec {
        return None;
    }
    let mut prev_t: Option<f64> = None;
    let mut off = 8usize;
    for _ in 0..n {
        let f64_of = |off: usize| {
            bytes
                .get(off..off + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let t = f64_of(off)?;
        off += 8;
        off += 8;
        off += 8;
        off += 8;
        off += 8;
        off += 8;
        let val = f64_of(off)?;
        off += 8;
        off += 4;
        if smg {
            off += 4;
        }
        if !t.is_finite() || !val.is_finite() {
            return None;
        }
        if let Some(p) = prev_t
            && t < p
        {
            return None;
        }
        prev_t = Some(t);
    }
    Some(n)
}

pub fn write_gbco(records: &[GbcoRec]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * GBCO_REC_BYTES);
    buf.extend_from_slice(&MAGIC_GBCO);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.lat.to_le_bytes());
        buf.extend_from_slice(&r.lon.to_le_bytes());
        buf.extend_from_slice(&r.elev.to_le_bytes());
    }
    buf
}

pub fn parse_gbco(bytes: &[u8]) -> Option<Vec<GbcoRec>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC_GBCO {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * GBCO_REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let f64_of = |off: usize| {
            bytes
                .get(off..off + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let lat = f64_of(off)?;
        off += 8;
        let lon = f64_of(off)?;
        off += 8;
        let elev = f64_of(off)?;
        off += 8;
        if !lat.is_finite() || !lon.is_finite() || !elev.is_finite() {
            return None;
        }
        out.push(GbcoRec { lat, lon, elev });
    }
    Some(out)
}

pub fn parse_slab2(bytes: &[u8]) -> Option<Vec<GbcoRec>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC_SLB2 {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * GBCO_REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let f64_of = |off: usize| {
            bytes
                .get(off..off + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let lat = f64_of(off)?;
        off += 8;
        let lon = f64_of(off)?;
        off += 8;
        let elev = f64_of(off)?;
        off += 8;
        if !lat.is_finite() || !lon.is_finite() || !elev.is_finite() {
            return None;
        }
        out.push(GbcoRec { lat, lon, elev });
    }
    Some(out)
}

pub fn write_gmr(records: &[GbcoRec]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * GBCO_REC_BYTES);
    buf.extend_from_slice(&MAGIC_GMR);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.lat.to_le_bytes());
        buf.extend_from_slice(&r.lon.to_le_bytes());
        buf.extend_from_slice(&r.elev.to_le_bytes());
    }
    buf
}

pub fn parse_gmr(bytes: &[u8]) -> Option<Vec<GbcoRec>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC_GMR {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * GBCO_REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let f64_of = |off: usize| {
            bytes
                .get(off..off + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let lat = f64_of(off)?;
        off += 8;
        let lon = f64_of(off)?;
        off += 8;
        let elev = f64_of(off)?;
        off += 8;
        if !lat.is_finite() || !lon.is_finite() || !elev.is_finite() {
            return None;
        }
        out.push(GbcoRec { lat, lon, elev });
    }
    Some(out)
}

pub fn write_ocs(records: &[GbcoRec]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * GBCO_REC_BYTES);
    buf.extend_from_slice(&MAGIC_OCS);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.lat.to_le_bytes());
        buf.extend_from_slice(&r.lon.to_le_bytes());
        buf.extend_from_slice(&r.elev.to_le_bytes());
    }
    buf
}

pub fn parse_ocs(bytes: &[u8]) -> Option<Vec<GbcoRec>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC_OCS {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * GBCO_REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let f64_of = |off: usize| {
            bytes
                .get(off..off + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let lat = f64_of(off)?;
        off += 8;
        let lon = f64_of(off)?;
        off += 8;
        let elev = f64_of(off)?;
        off += 8;
        if !lat.is_finite() || !lon.is_finite() || !elev.is_finite() {
            return None;
        }
        out.push(GbcoRec { lat, lon, elev });
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            GeoRec {
                t: 123456789.0,
                lat: -67.6,
                lon: 62.87,
                alt: 60.0,
                freq: crate::spectral::SPECTRAL_NO_BAND,
                bin_width: crate::spectral::SPECTRAL_NO_BAND,
                val: 30.45786,
                comp: COMP_BGR_AZIM,
                station: 0,
            },
            GeoRec {
                t: 123456849.0,
                lat: -67.6,
                lon: 62.87,
                alt: 60.0,
                freq: crate::spectral::SPECTRAL_NO_BAND,
                bin_width: crate::spectral::SPECTRAL_NO_BAND,
                val: 355.75248,
                comp: COMP_BGR_VAPP,
                station: 0,
            },
        ];
        let bytes = write_bin(MAGIC_BGR, &records);
        let parsed = parse_bin(MAGIC_BGR, &bytes).unwrap();
        assert_eq!(parsed.len(), records.len());
        for (a, b) in parsed.iter().zip(records.iter()) {
            assert_eq!(a.t, b.t);
            assert_eq!(a.lat, b.lat);
            assert_eq!(a.lon, b.lon);
            assert_eq!(a.alt, b.alt);
            assert_eq!(a.freq, b.freq);
            assert_eq!(a.val, b.val);
            assert_eq!(a.comp, b.comp);
        }
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(MAGIC_BGR, b"X").is_none());
        assert!(parse_bin(MAGIC_BGR, b"BGR1abc").is_none());
        assert!(parse_bin(MAGIC_BGR, b"NRS1").is_none());
    }

    #[test]
    fn supermag_station_roundtrip_and_iaga() {
        let rec = GeoRec {
            t: 753_440_003.0,
            lat: 69.66,
            lon: 18.94,
            alt: 0.0,
            freq: crate::spectral::SPECTRAL_NO_BAND,
            bin_width: 60.0,
            val: -177.9,
            comp: COMP_SMG_N_NEZ,
            station: pack_iaga("TRO").unwrap(),
        };
        let bytes = write_bin(MAGIC_SMG, &[rec]);
        assert_eq!(bytes.len(), 8 + SMG_REC_BYTES);
        let parsed = parse_bin(MAGIC_SMG, &bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(iaga_of(parsed[0].station).as_deref(), Some("TRO"));
        assert_eq!(parsed[0].val, -177.9);
        assert_eq!(pack_iaga("tro"), None);
        assert_eq!(pack_iaga("TROO"), None);
        assert_eq!(iaga_of(0), None);
    }

    #[test]
    fn pack_iaga_digit_code_roundtrip() {
        for code in ["T03", "A01", "1AB", "123", "B27", "PG5"] {
            let packed = pack_iaga(code).expect("digit code packs");
            assert_eq!(iaga_of(packed).as_deref(), Some(code));
        }
        assert_eq!(pack_iaga("t03"), None);
        assert_eq!(pack_iaga("T0"), None);
        assert_eq!(pack_iaga("T033"), None);
        assert_eq!(pack_iaga("T-3"), None);
        assert_ne!(pack_iaga("T03"), pack_iaga("TRO"));
        assert_eq!(pack_iaga("TRO"), Some(0x4f5254));
    }

    #[test]
    fn gbco_roundtrip() {
        let records = vec![
            GbcoRec {
                lat: -67.6,
                lon: 62.87,
                elev: 30.0,
            },
            GbcoRec {
                lat: 72.49,
                lon: -156.6,
                elev: -833.0,
            },
        ];
        let bytes = write_gbco(&records);
        let parsed = parse_gbco(&bytes).unwrap();
        assert_eq!(parsed.len(), records.len());
        for (a, b) in parsed.iter().zip(records.iter()) {
            assert_eq!(a.lat, b.lat);
            assert_eq!(a.lon, b.lon);
            assert_eq!(a.elev, b.elev);
        }
    }

    #[test]
    fn gbco_rejects_foreign_bytes() {
        assert!(parse_gbco(b"X").is_none());
        assert!(parse_gbco(b"GBCOabc").is_none());
        assert!(parse_gbco(b"BGR1").is_none());
    }

    #[test]
    fn ocs_roundtrip_and_rejections() {
        let records = vec![GbcoRec {
            lat: 39.2059,
            lon: -76.5178,
            elev: -10.363,
        }];
        let bytes = write_ocs(&records);
        let parsed = parse_ocs(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].lat, 39.2059);
        assert_eq!(parsed[0].lon, -76.5178);
        assert_eq!(parsed[0].elev, -10.363);
        assert!(parse_ocs(b"X").is_none());
        assert!(parse_ocs(b"OCS1abc").is_none());
        assert!(parse_ocs(&write_gbco(&records)).is_none());
    }

    #[test]
    fn gmr_roundtrip_and_rejections() {
        let records = vec![
            GbcoRec {
                lat: 40.5,
                lon: -9.5,
                elev: -294.0,
            },
            GbcoRec {
                lat: 41.0,
                lon: -10.0,
                elev: -3657.987,
            },
        ];
        let bytes = write_gmr(&records);
        let parsed = parse_gmr(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].lat, 40.5);
        assert_eq!(parsed[0].lon, -9.5);
        assert_eq!(parsed[0].elev, -294.0);
        assert_eq!(parsed[1].elev, -3657.987);
        assert!(parse_gmr(b"X").is_none());
        assert!(parse_gmr(b"GMR1abc").is_none());
        assert!(parse_gmr(&write_ocs(&records)).is_none());
        let nan = write_gmr(&[GbcoRec {
            lat: 40.5,
            lon: -9.5,
            elev: f64::NAN,
        }]);
        assert!(parse_gmr(&nan).is_none());
    }

    #[test]
    fn slab2_roundtrip_and_rejections() {
        fn enc(recs: &[(f64, f64, f64)]) -> Vec<u8> {
            let mut buf = Vec::with_capacity(8 + recs.len() * GBCO_REC_BYTES);
            buf.extend_from_slice(&MAGIC_SLB2);
            buf.extend_from_slice(&(recs.len() as u32).to_le_bytes());
            for (lat, lon, elev) in recs {
                buf.extend_from_slice(&lat.to_le_bytes());
                buf.extend_from_slice(&lon.to_le_bytes());
                buf.extend_from_slice(&elev.to_le_bytes());
            }
            buf
        }
        let bytes = enc(&[(68.0, 161.05, -10500.0), (50.0, 10.0, -30000.0)]);
        let parsed = parse_slab2(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].lat, 68.0);
        assert_eq!(parsed[0].lon, 161.05);
        assert_eq!(parsed[0].elev, -10500.0);
        assert_eq!(parsed[1].elev, -30000.0);

        assert!(parse_slab2(b"X").is_none());
        assert!(parse_slab2(b"SLB2abc").is_none());
        let gbco = write_gbco(&[GbcoRec {
            lat: 1.0,
            lon: 2.0,
            elev: 3.0,
        }]);
        assert!(parse_slab2(&gbco).is_none());
        let short = bytes[..bytes.len() - 1].to_vec();
        assert!(parse_slab2(&short).is_none());
        let nan = enc(&[(68.0, 161.05, f64::NAN)]);
        assert!(parse_slab2(&nan).is_none());
    }

    #[test]
    fn rejects_oversized_declared_count_and_non_finite_elevation() {
        let records = vec![GbcoRec {
            lat: 72.49,
            lon: -156.6,
            elev: -833.0,
        }];
        let bytes = write_gbco(&records);
        assert!(parse_gbco(&bytes).is_some());
        let mut short = bytes[..bytes.len() - 1].to_vec();
        let n = u32::from_le_bytes(short[4..8].try_into().unwrap());
        short[4..8].copy_from_slice(&(n + 1).to_le_bytes());
        assert!(parse_gbco(&short).is_none());
        let nan = write_gbco(&[GbcoRec {
            lat: 72.49,
            lon: -156.6,
            elev: f64::NAN,
        }]);
        assert!(parse_gbco(&nan).is_none());
    }

    #[test]
    fn verify_bin_streams_count_and_rejects_disorder() {
        let records = vec![
            GeoRec {
                t: 1.0,
                lat: 1.0,
                lon: 2.0,
                alt: 3.0,
                freq: crate::spectral::SPECTRAL_NO_BAND,
                bin_width: 60.0,
                val: 9.81,
                comp: COMP_IGETS_G,
                station: 0,
            },
            GeoRec {
                t: 2.0,
                lat: 1.0,
                lon: 2.0,
                alt: 3.0,
                freq: crate::spectral::SPECTRAL_NO_BAND,
                bin_width: 60.0,
                val: 9.82,
                comp: COMP_IGETS_G,
                station: 0,
            },
        ];
        let bytes = write_bin(MAGIC_IGETS, &records);
        assert_eq!(verify_bin(MAGIC_IGETS, &bytes), Some(2));
        assert!(verify_bin(MAGIC_IGETS, b"IGT1").is_none());
        assert!(verify_bin(MAGIC_BGR, &bytes).is_none());
        let mut disorder = records;
        disorder[1].t = 0.5;
        let bytes = write_bin(MAGIC_IGETS, &disorder);
        assert!(verify_bin(MAGIC_IGETS, &bytes).is_none());
    }

    #[test]
    fn verify_bin_rejects_non_finite_time_and_value() {
        let mut records = vec![GeoRec {
            t: 1.0,
            lat: 1.0,
            lon: 2.0,
            alt: 3.0,
            freq: crate::spectral::SPECTRAL_NO_BAND,
            bin_width: 60.0,
            val: 9.81,
            comp: COMP_IGETS_G,
            station: 0,
        }];
        records[0].t = f64::NAN;
        assert!(verify_bin(MAGIC_IGETS, &write_bin(MAGIC_IGETS, &records)).is_none());
        records[0].t = 1.0;
        records[0].val = f64::INFINITY;
        assert!(verify_bin(MAGIC_IGETS, &write_bin(MAGIC_IGETS, &records)).is_none());
    }
}
