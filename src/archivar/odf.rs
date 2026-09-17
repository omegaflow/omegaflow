pub const ORBIT_KEY: u32 = 1031;
pub const PK_FILE_LABEL: u32 = 101;
pub const PK_IDENTIFIER: u32 = 107;
pub const PK_ORBIT_HEADER: u32 = 109;
pub const PK_RAMP: u32 = 2030;
pub const PK_CLOCK: u32 = 2040;
pub const PK_SUMMARY: u32 = 105;

pub const ODF_TDB_OFFSET: f64 = -1577664000.0;

pub const COMP_OBSERVABLE: u32 = 1;
pub const TNF_COMP_UL_PHASE: u32 = 1;

pub const PODF_COL_TDB: usize = 0;
pub const PODF_COL_OBSERVABLE: usize = 1;

#[derive(Clone, Copy, Debug)]
pub struct OdOrbit {
    pub t_since_1950: f64,
    pub observable_hz: f64,
    pub dss_rx: i64,
    pub dss_tx: i64,
    pub data_type: i64,
    pub downlink_band: i64,
    pub uplink_band: i64,
    pub valid: bool,
    pub scid: i64,
    pub ref_hz: f64,
    pub compression_s: f64,
}

pub fn bits(words: &[u32; 9], first: usize, last: usize) -> i64 {
    let mut v: i64 = 0;
    for b in first..=last {
        let word = (b - 1) / 32;
        let wbit = (b - 1) % 32;
        let bit = (words[word] >> (31 - wbit)) & 1;
        v = (v << 1) | bit as i64;
    }
    v
}

pub fn twos(v: i64, width: usize) -> i64 {
    let sign = 1i64 << (width - 1);
    if v & sign != 0 {
        v - (1i64 << width)
    } else {
        v
    }
}

pub fn orbit_record(words: &[u32; 9]) -> Option<OdOrbit> {
    if words[0] < 0x400000 {
        return None;
    }
    let fmt = bits(words, 129, 131);
    let t_int = words[0] as i64;
    let observable = twos(words[2] as i64, 32) as f64 + twos(words[3] as i64, 32) as f64 / 1.0e9;
    let dss_rx = bits(words, 132, 138);
    let dss_tx = bits(words, 139, 145);
    let (t_frac, t_div, data_type, downlink_band, uplink_band, valid, scid, ref_hz, compression) =
        if fmt == 1 {
            (
                bits(words, 33, 64),
                1.0e9,
                bits(words, 150, 155),
                bits(words, 148, 149),
                bits(words, 187, 188),
                bits(words, 200, 200) == 0,
                bits(words, 161, 167),
                bits(words, 225, 256) as f64 * 10.0 + bits(words, 257, 264) as f64 * 0.1,
                bits(words, 201, 224) as f64 / 100.0,
            )
        } else {
            (
                bits(words, 33, 42),
                1.0e10,
                bits(words, 148, 153),
                bits(words, 154, 155),
                bits(words, 156, 157),
                bits(words, 160, 160) == 0,
                bits(words, 168, 177),
                bits(words, 179, 224) as f64 / 1000.0,
                bits(words, 245, 266) as f64 / 100.0,
            )
        };
    Some(OdOrbit {
        t_since_1950: t_int as f64 + t_frac as f64 / t_div,
        observable_hz: observable,
        dss_rx,
        dss_tx,
        data_type,
        downlink_band,
        uplink_band,
        valid,
        scid,
        ref_hz,
        compression_s: compression,
    })
}

pub fn write_podf_bin(records: &[[f64; 9]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 72);
    out.extend_from_slice(b"PODF");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub const PODF_SHARD_BUDGET: usize = 1 << 30;
pub const PODF_SHARD_LIMIT: usize = 1 << 31;

pub fn podf_shard_ranges(count: usize, budget: usize) -> Vec<(usize, usize)> {
    if count == 0 {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    let mut lo = 0usize;
    while lo < count {
        let mut hi = lo + 1;
        while hi < count && 8 + (hi + 1 - lo) * 72 <= budget {
            hi += 1;
        }
        ranges.push((lo, hi));
        lo = hi;
    }
    ranges
}

pub fn podf_shard_name(prefix: &str, t_lo: f64, t_hi: f64) -> String {
    format!("{prefix}_t{}_{}.bin", t_lo as i64, t_hi as i64)
}

pub fn podf_shard_name_ord(prefix: &str, t_lo: f64, t_hi: f64, ord: usize) -> String {
    format!("{prefix}_t{}_{}_{}.bin", t_lo as i64, t_hi as i64, ord)
}

pub fn parse_podf_shard_name(filename: &str) -> Option<(&str, f64, f64)> {
    let stem = filename.strip_suffix(".bin")?;
    let (name, range) = stem.rsplit_once("_t")?;
    let mut it = range.split('_');
    let lo: f64 = it.next()?.parse().ok()?;
    let hi: f64 = it.next()?.parse().ok()?;
    Some((name, lo, hi))
}

pub fn parse_podf_bin(data: &[u8]) -> Option<Vec<[f64; 9]>> {
    if data.len() < 8 || &data[0..4] != b"PODF" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 72 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 72;
        let mut r = [0.0f64; 9];
        for k in 0..9 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_podf_bin(bytes)?;
    let mut out = Vec::with_capacity(rows.len());
    for r in &rows {
        let t = r[PODF_COL_TDB];
        let v = r[PODF_COL_OBSERVABLE];
        if !t.is_finite() || !v.is_finite() {
            continue;
        }
        out.push((t, v, COMP_OBSERVABLE));
    }
    Some(out)
}

pub fn write_p11r_bin(records: &[[f64; 9]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 72);
    out.extend_from_slice(b"P11R");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_p11r_bin(data: &[u8]) -> Option<Vec<[f64; 9]>> {
    if data.len() < 8 || &data[0..4] != b"P11R" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 72 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 72;
        let mut r = [0.0f64; 9];
        for k in 0..9 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

pub fn parse_odf(bytes: &[u8]) -> Option<Vec<OdOrbit>> {
    if !bytes.len().is_multiple_of(36) {
        return None;
    }
    let mut out = Vec::new();
    let mut in_orbit = false;
    let n = bytes.len() / 36;
    for i in 0..n {
        let mut words = [0u32; 9];
        for k in 0..9 {
            words[k] =
                u32::from_be_bytes(bytes[i * 36 + k * 4..i * 36 + k * 4 + 4].try_into().ok()?);
        }
        match words[0] {
            PK_FILE_LABEL | PK_IDENTIFIER | PK_ORBIT_HEADER | PK_RAMP | PK_CLOCK | PK_SUMMARY => {
                in_orbit = words[0] == PK_ORBIT_HEADER;
            }
            _ => {
                if in_orbit && let Some(r) = orbit_record(&words) {
                    out.push(r);
                }
            }
        }
    }
    Some(out)
}

pub const TNF_LABEL_LEN: usize = 20;

pub const TNF_FORMAT_UL_CARRIER_PHASE: u8 = 0;
pub const TNF_FORMAT_DL_CARRIER_PHASE: u8 = 1;
pub const TNF_FORMAT_UL_SEQ_RANGING_PHASE: u8 = 2;
pub const TNF_FORMAT_DL_SEQ_RANGING_PHASE: u8 = 3;
pub const TNF_FORMAT_UL_PN_RANGING_PHASE: u8 = 4;
pub const TNF_FORMAT_DL_PN_RANGING_PHASE: u8 = 5;
pub const TNF_FORMAT_DOPPLER_COUNT: u8 = 6;
pub const TNF_FORMAT_SEQUENTIAL_RANGE: u8 = 7;
pub const TNF_FORMAT_ANGLE: u8 = 8;
pub const TNF_FORMAT_RAMP: u8 = 9;
pub const TNF_FORMAT_VLBI: u8 = 10;
pub const TNF_FORMAT_DRVID: u8 = 11;
pub const TNF_FORMAT_SMOOTHED_NOISE: u8 = 12;
pub const TNF_FORMAT_ALLAN_DEVIATION: u8 = 13;
pub const TNF_FORMAT_PN_RANGE: u8 = 14;
pub const TNF_FORMAT_TONE_RANGE: u8 = 15;
pub const TNF_FORMAT_CARRIER_OBSERVABLE: u8 = 16;
pub const TNF_FORMAT_TOTAL_PHASE_OBSERVABLE: u8 = 17;

#[derive(Clone, Copy, Debug)]
pub struct TnfSfdu {
    pub offset: usize,
    pub total_len: usize,
    pub data_description_id: [u8; 4],
    pub sfdu_length: u64,
    pub aggr_chdo_type: u16,
    pub aggr_chdo_length: u16,
    pub primary_chdo_type: u16,
    pub primary_chdo_length: u16,
    pub mjr_data_class: u8,
    pub mnr_data_class: u8,
    pub mission_id: u8,
    pub format_code: u8,
    pub secondary_chdo_type: u16,
    pub secondary_chdo_length: u16,
    pub scft_id: u8,
    pub rec_seq_num: u32,
    pub year: u16,
    pub doy: u16,
    pub sec: f64,
}

fn be16(b: &[u8]) -> u16 {
    u16::from_be_bytes([b[0], b[1]])
}

fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

fn be64(b: &[u8]) -> u64 {
    u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

fn read_tnf_sfdu(bytes: &[u8], offset: usize) -> Option<TnfSfdu> {
    if bytes.len() < 60 || &bytes[0..4] != b"NJPL" {
        return None;
    }
    let sfdu_length = be64(&bytes[12..20]);
    let total_len = TNF_LABEL_LEN + sfdu_length as usize;
    if bytes.len() < total_len {
        return None;
    }
    let mut data_description_id = [0u8; 4];
    data_description_id.copy_from_slice(&bytes[8..12]);
    Some(TnfSfdu {
        offset,
        total_len,
        data_description_id,
        sfdu_length,
        aggr_chdo_type: be16(&bytes[20..22]),
        aggr_chdo_length: be16(&bytes[22..24]),
        primary_chdo_type: be16(&bytes[24..26]),
        primary_chdo_length: be16(&bytes[26..28]),
        mjr_data_class: bytes[28],
        mnr_data_class: bytes[29],
        mission_id: bytes[30],
        format_code: bytes[31],
        secondary_chdo_type: be16(&bytes[32..34]),
        secondary_chdo_length: be16(&bytes[34..36]),
        scft_id: bytes[39],
        rec_seq_num: be32(&bytes[44..48]),
        year: be16(&bytes[48..50]),
        doy: be16(&bytes[50..52]),
        sec: f64::from_be_bytes(bytes[52..60].try_into().ok()?),
    })
}

pub fn scan_tnf_sfdus(bytes: &[u8]) -> Option<Vec<TnfSfdu>> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off < bytes.len() {
        let frame = read_tnf_sfdu(&bytes[off..], off)?;
        off += frame.total_len;
        out.push(frame);
    }
    Some(out)
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt0 {
    pub orig_id: u8,
    pub last_modifier_id: u8,
    pub upl_rec_seq_num: u32,
    pub rct_day: u16,
    pub rct_msec: u32,
    pub ul_dss_id: u8,
    pub ul_band: u8,
    pub ul_assembly_num: u8,
    pub transmit_num: u8,
    pub transmit_stat: u8,
    pub transmit_mode: u8,
    pub cmd_modul_stat: u8,
    pub rng_modul_stat: u8,
    pub fts_vld_flag: u8,
    pub transmit_time_tag_delay: f64,
    pub ul_zheight_corr: f32,
    pub mod_day: u16,
    pub mod_msec: u32,
    pub version_num: u8,
    pub sub_version_num: u8,
    pub sub_sub_version_num: u8,
    pub data_chdo_type: u16,
    pub data_chdo_length: u16,
    pub ul_hi_phs_cycles: u32,
    pub ul_lo_phs_cycles: u32,
    pub ul_frac_phs_cycles: u32,
    pub ramp_freq: f64,
    pub ramp_rate: f64,
    pub transmit_switch_stat: u8,
    pub ramp_type: u8,
    pub transmit_op_pwr: f32,
    pub sup_data_id: [u8; 8],
    pub sup_data_rev: [u8; 8],
    pub prdx_time_offset: f64,
    pub prdx_freq_offset: f64,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
    pub fabricated_sfdu_flag: u8,
}

pub fn tnf_dt0(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt0> {
    if frame.format_code != TNF_FORMAT_UL_CARRIER_PHASE || bytes.len() < 182 {
        return None;
    }
    let mut sup_data_id = [0u8; 8];
    sup_data_id.copy_from_slice(&bytes[140..148]);
    let mut sup_data_rev = [0u8; 8];
    sup_data_rev.copy_from_slice(&bytes[148..156]);
    Some(TnfDt0 {
        orig_id: bytes[36],
        last_modifier_id: bytes[37],
        upl_rec_seq_num: be32(&bytes[40..44]),
        rct_day: be16(&bytes[60..62]),
        rct_msec: be32(&bytes[62..66]),
        ul_dss_id: bytes[66],
        ul_band: bytes[67],
        ul_assembly_num: bytes[68],
        transmit_num: bytes[69],
        transmit_stat: bytes[70],
        transmit_mode: bytes[71],
        cmd_modul_stat: bytes[72],
        rng_modul_stat: bytes[73],
        fts_vld_flag: bytes[74],
        transmit_time_tag_delay: f64::from_be_bytes(bytes[76..84].try_into().ok()?),
        ul_zheight_corr: f32::from_be_bytes(bytes[84..88].try_into().ok()?),
        mod_day: be16(&bytes[88..90]),
        mod_msec: be32(&bytes[90..94]),
        version_num: bytes[94],
        sub_version_num: bytes[95],
        sub_sub_version_num: bytes[96],
        data_chdo_type: be16(&bytes[102..104]),
        data_chdo_length: be16(&bytes[104..106]),
        ul_hi_phs_cycles: be32(&bytes[106..110]),
        ul_lo_phs_cycles: be32(&bytes[110..114]),
        ul_frac_phs_cycles: be32(&bytes[114..118]),
        ramp_freq: f64::from_be_bytes(bytes[118..126].try_into().ok()?),
        ramp_rate: f64::from_be_bytes(bytes[126..134].try_into().ok()?),
        transmit_switch_stat: bytes[134],
        ramp_type: bytes[135],
        transmit_op_pwr: f32::from_be_bytes(bytes[136..140].try_into().ok()?),
        sup_data_id,
        sup_data_rev,
        prdx_time_offset: f64::from_be_bytes(bytes[156..164].try_into().ok()?),
        prdx_freq_offset: f64::from_be_bytes(bytes[164..172].try_into().ok()?),
        time_tag_corr_flag: bytes[172],
        type_time_corr_flag: bytes[173],
        fabricated_sfdu_flag: bytes[174],
    })
}

pub fn tnf_phase_cycles(hi: u32, lo: u32, frac: u32) -> f64 {
    hi as f64 * 4_294_967_296.0 + lo as f64 + frac as f64 / 4_294_967_296.0
}

pub fn tnf_ul_phase_cycles(d: &TnfDt0) -> f64 {
    tnf_phase_cycles(d.ul_hi_phs_cycles, d.ul_lo_phs_cycles, d.ul_frac_phs_cycles)
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt1 {
    pub dl_dss_id: u8,
    pub dl_band: u8,
    pub carr_loop_bw: f32,
    pub pcn0: f32,
    pub pcn0_resid: f32,
    pub pdn0: f32,
    pub pdn0_resid: f32,
    pub system_noise_temp: f32,
    pub phases: [[u32; 3]; 10],
    pub phase_avg: [u32; 3],
    pub dl_freq: f64,
    pub dop_resid: f32,
    pub dop_noise: f32,
    pub slipped_cycles: i32,
    pub carr_loop_type: u8,
    pub snt_flag: u8,
    pub carr_resid_wt: f32,
    pub sup_data_id: [u8; 8],
    pub sup_data_rev: [u8; 8],
    pub prdx_time_offset: f64,
    pub prdx_freq_offset: f64,
    pub carr_resid_tol_flag: u8,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
    pub dop_mode_corr_flag: u8,
    pub ul_stn_corr_flag: u8,
}

pub fn tnf_dt1(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt1> {
    if frame.format_code != TNF_FORMAT_DL_CARRIER_PHASE || bytes.len() < 378 {
        return None;
    }
    let mut phases = [[0u32; 3]; 10];
    for i in 0..10 {
        let base = 174 + i * 12;
        phases[i] = [
            be32(&bytes[base..base + 4]),
            be32(&bytes[base + 4..base + 8]),
            be32(&bytes[base + 8..base + 12]),
        ];
    }
    let mut sup_data_id = [0u8; 8];
    sup_data_id.copy_from_slice(&bytes[332..340]);
    let mut sup_data_rev = [0u8; 8];
    sup_data_rev.copy_from_slice(&bytes[340..348]);
    Some(TnfDt1 {
        dl_dss_id: bytes[66],
        dl_band: bytes[67],
        carr_loop_bw: f32::from_be_bytes(bytes[150..154].try_into().ok()?),
        pcn0: f32::from_be_bytes(bytes[154..158].try_into().ok()?),
        pcn0_resid: f32::from_be_bytes(bytes[158..162].try_into().ok()?),
        pdn0: f32::from_be_bytes(bytes[162..166].try_into().ok()?),
        pdn0_resid: f32::from_be_bytes(bytes[166..170].try_into().ok()?),
        system_noise_temp: f32::from_be_bytes(bytes[170..174].try_into().ok()?),
        phases,
        phase_avg: [
            be32(&bytes[294..298]),
            be32(&bytes[298..302]),
            be32(&bytes[302..306]),
        ],
        dl_freq: f64::from_be_bytes(bytes[306..314].try_into().ok()?),
        dop_resid: f32::from_be_bytes(bytes[314..318].try_into().ok()?),
        dop_noise: f32::from_be_bytes(bytes[318..322].try_into().ok()?),
        slipped_cycles: i32::from_be_bytes(bytes[322..326].try_into().ok()?),
        carr_loop_type: bytes[326],
        snt_flag: bytes[327],
        carr_resid_wt: f32::from_be_bytes(bytes[328..332].try_into().ok()?),
        sup_data_id,
        sup_data_rev,
        prdx_time_offset: f64::from_be_bytes(bytes[348..356].try_into().ok()?),
        prdx_freq_offset: f64::from_be_bytes(bytes[356..364].try_into().ok()?),
        carr_resid_tol_flag: bytes[364],
        time_tag_corr_flag: bytes[365],
        type_time_corr_flag: bytes[366],
        dop_mode_corr_flag: bytes[367],
        ul_stn_corr_flag: bytes[368],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt2 {
    pub ul_dss_id: u8,
    pub ul_band: u8,
    pub stn_cal: f64,
    pub ul_stn_cal: f64,
    pub ul_cal_freq: f64,
    pub cal_std_dev: f32,
    pub cal_pts: u16,
    pub ul_rng_phs: f64,
    pub transmit_switch_stat: u8,
    pub invert: u8,
    pub transmit_op_pwr: f32,
    pub template_id: [u8; 8],
    pub t1: u16,
    pub t2: u16,
    pub t3: u16,
    pub first_comp_num: u8,
    pub last_comp_num: u8,
    pub chop_comp_num: u8,
    pub num_drvid: u8,
    pub tx_inphs_year: u16,
    pub tx_inphs_doy: u16,
    pub tx_inphs_sec: f64,
    pub carr_sup_rng_modul: f32,
    pub rng_modul_amp: u16,
    pub exc_scalar_num: u32,
    pub exc_scalar_den: u32,
    pub rng_cycle_time: f64,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
    pub clock_waveform: u8,
    pub chop_start_num: u8,
    pub rng_meas_type: u8,
    pub fabricated_sfdu_flag: u8,
}

pub fn tnf_dt2(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt2> {
    if frame.format_code != TNF_FORMAT_UL_SEQ_RANGING_PHASE || bytes.len() < 214 {
        return None;
    }
    let mut template_id = [0u8; 8];
    template_id.copy_from_slice(&bytes[150..158]);
    Some(TnfDt2 {
        ul_dss_id: bytes[66],
        ul_band: bytes[67],
        stn_cal: f64::from_be_bytes(bytes[106..114].try_into().ok()?),
        ul_stn_cal: f64::from_be_bytes(bytes[114..122].try_into().ok()?),
        ul_cal_freq: f64::from_be_bytes(bytes[122..130].try_into().ok()?),
        cal_std_dev: f32::from_be_bytes(bytes[130..134].try_into().ok()?),
        cal_pts: be16(&bytes[134..136]),
        ul_rng_phs: f64::from_be_bytes(bytes[136..144].try_into().ok()?),
        transmit_switch_stat: bytes[144],
        invert: bytes[145],
        transmit_op_pwr: f32::from_be_bytes(bytes[146..150].try_into().ok()?),
        template_id,
        t1: be16(&bytes[158..160]),
        t2: be16(&bytes[160..162]),
        t3: be16(&bytes[162..164]),
        first_comp_num: bytes[164],
        last_comp_num: bytes[165],
        chop_comp_num: bytes[166],
        num_drvid: bytes[167],
        tx_inphs_year: be16(&bytes[168..170]),
        tx_inphs_doy: be16(&bytes[170..172]),
        tx_inphs_sec: f64::from_be_bytes(bytes[172..180].try_into().ok()?),
        carr_sup_rng_modul: f32::from_be_bytes(bytes[180..184].try_into().ok()?),
        rng_modul_amp: be16(&bytes[184..186]),
        exc_scalar_num: be32(&bytes[186..190]),
        exc_scalar_den: be32(&bytes[190..194]),
        rng_cycle_time: f64::from_be_bytes(bytes[194..202].try_into().ok()?),
        time_tag_corr_flag: bytes[202],
        type_time_corr_flag: bytes[203],
        clock_waveform: bytes[204],
        chop_start_num: bytes[205],
        rng_meas_type: bytes[206],
        fabricated_sfdu_flag: bytes[207],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt3 {
    pub dl_dss_id: u8,
    pub dl_band: u8,
    pub stn_cal: f64,
    pub dl_stn_cal: f64,
    pub dl_cal_freq: f64,
    pub cal_std_dev: f32,
    pub cal_pts: u16,
    pub dl_rng_phs: f64,
    pub figure_merit: f32,
    pub rng_resid: f64,
    pub drvid: f64,
    pub rtlt: f32,
    pub pcn0: f32,
    pub pcn0_resid: f32,
    pub pdn0: f32,
    pub pdn0_resid: f32,
    pub prn0: f32,
    pub prn0_resid: f32,
    pub system_noise_temp: f32,
    pub carr_loop_type: u8,
    pub snt_flag: u8,
    pub carr_resid_wt: f32,
    pub template_id: [u8; 8],
    pub invert: u8,
    pub correl_type: u8,
    pub t1: u16,
    pub t2: u16,
    pub t3: u16,
    pub first_comp_num: u8,
    pub last_comp_num: u8,
    pub chop_comp_num: u8,
    pub num_drvid: u8,
    pub rcv_inphs_year: u16,
    pub rcv_inphs_doy: u16,
    pub rcv_inphs_sec: f64,
    pub exc_scalar_num: u32,
    pub exc_scalar_den: u32,
    pub rng_cycle_time: f64,
    pub inphs_correl: f32,
    pub quad_phs_correl: f32,
    pub metrics_vld_flag: u8,
    pub correl_vld_flag: u8,
    pub rng_resid_tol_flag: u8,
    pub drvid_tol_flag: u8,
    pub prn0_resid_tol_flag: u8,
    pub rng_sigma_tol_flag: u8,
    pub rng_vld_flag: u8,
    pub rng_config_flag: u8,
    pub rng_hw_flag: u8,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
    pub dop_mode_corr_flag: u8,
    pub ul_stn_corr_flag: u8,
    pub chop_start_num: u8,
    pub rng_meas_type: u8,
    pub stn_cal_corr_flag: u8,
}

pub fn tnf_dt3(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt3> {
    if frame.format_code != TNF_FORMAT_DL_SEQ_RANGING_PHASE || bytes.len() < 324 {
        return None;
    }
    let mut template_id = [0u8; 8];
    template_id.copy_from_slice(&bytes[246..254]);
    Some(TnfDt3 {
        dl_dss_id: bytes[66],
        dl_band: bytes[67],
        stn_cal: f64::from_be_bytes(bytes[150..158].try_into().ok()?),
        dl_stn_cal: f64::from_be_bytes(bytes[158..166].try_into().ok()?),
        dl_cal_freq: f64::from_be_bytes(bytes[166..174].try_into().ok()?),
        cal_std_dev: f32::from_be_bytes(bytes[174..178].try_into().ok()?),
        cal_pts: be16(&bytes[178..180]),
        dl_rng_phs: f64::from_be_bytes(bytes[180..188].try_into().ok()?),
        figure_merit: f32::from_be_bytes(bytes[188..192].try_into().ok()?),
        rng_resid: f64::from_be_bytes(bytes[192..200].try_into().ok()?),
        drvid: f64::from_be_bytes(bytes[200..208].try_into().ok()?),
        rtlt: f32::from_be_bytes(bytes[208..212].try_into().ok()?),
        pcn0: f32::from_be_bytes(bytes[212..216].try_into().ok()?),
        pcn0_resid: f32::from_be_bytes(bytes[216..220].try_into().ok()?),
        pdn0: f32::from_be_bytes(bytes[220..224].try_into().ok()?),
        pdn0_resid: f32::from_be_bytes(bytes[224..228].try_into().ok()?),
        prn0: f32::from_be_bytes(bytes[228..232].try_into().ok()?),
        prn0_resid: f32::from_be_bytes(bytes[232..236].try_into().ok()?),
        system_noise_temp: f32::from_be_bytes(bytes[236..240].try_into().ok()?),
        carr_loop_type: bytes[240],
        snt_flag: bytes[241],
        carr_resid_wt: f32::from_be_bytes(bytes[242..246].try_into().ok()?),
        template_id,
        invert: bytes[254],
        correl_type: bytes[255],
        t1: be16(&bytes[256..258]),
        t2: be16(&bytes[258..260]),
        t3: be16(&bytes[260..262]),
        first_comp_num: bytes[262],
        last_comp_num: bytes[263],
        chop_comp_num: bytes[264],
        num_drvid: bytes[265],
        rcv_inphs_year: be16(&bytes[266..268]),
        rcv_inphs_doy: be16(&bytes[268..270]),
        rcv_inphs_sec: f64::from_be_bytes(bytes[270..278].try_into().ok()?),
        exc_scalar_num: be32(&bytes[278..282]),
        exc_scalar_den: be32(&bytes[282..286]),
        rng_cycle_time: f64::from_be_bytes(bytes[286..294].try_into().ok()?),
        inphs_correl: f32::from_be_bytes(bytes[294..298].try_into().ok()?),
        quad_phs_correl: f32::from_be_bytes(bytes[298..302].try_into().ok()?),
        metrics_vld_flag: bytes[302],
        correl_vld_flag: bytes[303],
        rng_resid_tol_flag: bytes[304],
        drvid_tol_flag: bytes[305],
        prn0_resid_tol_flag: bytes[306],
        rng_sigma_tol_flag: bytes[307],
        rng_vld_flag: bytes[308],
        rng_config_flag: bytes[309],
        rng_hw_flag: bytes[310],
        time_tag_corr_flag: bytes[311],
        type_time_corr_flag: bytes[312],
        dop_mode_corr_flag: bytes[313],
        ul_stn_corr_flag: bytes[314],
        chop_start_num: bytes[315],
        rng_meas_type: bytes[316],
        stn_cal_corr_flag: bytes[317],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt4 {
    pub ul_dss_id: u8,
    pub ul_band: u8,
    pub stn_cal: f64,
    pub ul_stn_cal: f64,
    pub ul_cal_freq: f64,
    pub cal_std_dev: f32,
    pub cal_pts: u16,
    pub ul_rng_phs: f64,
    pub state_subcodes: [u8; 6],
    pub pn_clk_phs: f64,
    pub transmit_switch_stat: u8,
    pub invert: u8,
    pub transmit_op_pwr: f32,
    pub template_id: [u8; 22],
    pub clk_divider: u8,
    pub len_subcodes: [u8; 6],
    pub op_subcodes: [u8; 5],
    pub def_subcodes: [u64; 6],
    pub pn_code_length: u32,
    pub tx_inphs_year: u16,
    pub tx_inphs_doy: u16,
    pub tx_inphs_sec: f64,
    pub carr_sup_rng_modul: f32,
    pub rng_modul_amp: u16,
    pub exc_scalar_num: u32,
    pub exc_scalar_den: u32,
    pub rng_cycle_time: f64,
    pub clock_waveform: u8,
    pub rng_meas_type: u8,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
    pub fabricated_sfdu_flag: u8,
    pub op_subcode6: u8,
    pub ccsds_k: u8,
    pub ccsds_l: u8,
    pub ul_rng_modulo: u32,
}

pub fn tnf_dt4(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt4> {
    if frame.format_code != TNF_FORMAT_UL_PN_RANGING_PHASE || bytes.len() < 296 {
        return None;
    }
    let mut template_id = [0u8; 22];
    template_id.copy_from_slice(&bytes[164..186]);
    let mut def_subcodes = [0u64; 6];
    for i in 0..6 {
        let base = 198 + i * 8;
        def_subcodes[i] = be64(&bytes[base..base + 8]);
    }
    Some(TnfDt4 {
        ul_dss_id: bytes[66],
        ul_band: bytes[67],
        stn_cal: f64::from_be_bytes(bytes[106..114].try_into().ok()?),
        ul_stn_cal: f64::from_be_bytes(bytes[114..122].try_into().ok()?),
        ul_cal_freq: f64::from_be_bytes(bytes[122..130].try_into().ok()?),
        cal_std_dev: f32::from_be_bytes(bytes[130..134].try_into().ok()?),
        cal_pts: be16(&bytes[134..136]),
        ul_rng_phs: f64::from_be_bytes(bytes[136..144].try_into().ok()?),
        state_subcodes: bytes[144..150].try_into().ok()?,
        pn_clk_phs: f64::from_be_bytes(bytes[150..158].try_into().ok()?),
        transmit_switch_stat: bytes[158],
        invert: bytes[159],
        transmit_op_pwr: f32::from_be_bytes(bytes[160..164].try_into().ok()?),
        template_id,
        clk_divider: bytes[186],
        len_subcodes: bytes[187..193].try_into().ok()?,
        op_subcodes: bytes[193..198].try_into().ok()?,
        def_subcodes,
        pn_code_length: be32(&bytes[246..250]),
        tx_inphs_year: be16(&bytes[250..252]),
        tx_inphs_doy: be16(&bytes[252..254]),
        tx_inphs_sec: f64::from_be_bytes(bytes[254..262].try_into().ok()?),
        carr_sup_rng_modul: f32::from_be_bytes(bytes[262..266].try_into().ok()?),
        rng_modul_amp: be16(&bytes[266..268]),
        exc_scalar_num: be32(&bytes[268..272]),
        exc_scalar_den: be32(&bytes[272..276]),
        rng_cycle_time: f64::from_be_bytes(bytes[276..284].try_into().ok()?),
        clock_waveform: bytes[284],
        rng_meas_type: bytes[285],
        time_tag_corr_flag: bytes[286],
        type_time_corr_flag: bytes[287],
        fabricated_sfdu_flag: bytes[288],
        op_subcode6: bytes[289],
        ccsds_k: bytes[290],
        ccsds_l: bytes[291],
        ul_rng_modulo: be32(&bytes[292..296]),
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt5 {
    pub dl_dss_id: u8,
    pub dl_band: u8,
    pub stn_cal: f64,
    pub dl_stn_cal: f64,
    pub dl_cal_freq: f64,
    pub cal_std_dev: f32,
    pub cal_pts: u16,
    pub dl_rng_phs: f64,
    pub figure_merit: f32,
    pub rng_resid: f64,
    pub drvid: f64,
    pub rtlt: f32,
    pub pcn0: f32,
    pub pcn0_resid: f32,
    pub pdn0: f32,
    pub pdn0_resid: f32,
    pub prn0: f32,
    pub prn0_resid: f32,
    pub system_noise_temp: f32,
    pub state_subcodes: [u8; 6],
    pub pn_clk_phs: f64,
    pub carr_loop_type: u8,
    pub snt_flag: u8,
    pub carr_resid_wt: f32,
    pub template_id: [u8; 20],
    pub invert: u8,
    pub correl_type: u8,
    pub int_time: u32,
    pub clk_divider: u8,
    pub len_subcodes: [u8; 6],
    pub op_subcodes: [u8; 5],
    pub def_subcodes: [u64; 6],
    pub pn_code_length: u32,
    pub rcv_inphs_year: u16,
    pub rcv_inphs_doy: u16,
    pub rcv_inphs_sec: f64,
    pub exc_scalar_num: u32,
    pub exc_scalar_den: u32,
    pub rng_cycle_time: f64,
    pub inphs_correl: f32,
    pub quad_phs_correl: f32,
    pub metrics_vld_flag: u8,
    pub correl_vld_flag: u8,
    pub rng_resid_tol_flag: u8,
    pub drvid_tol_flag: u8,
    pub prn0_resid_tol_flag: u8,
    pub rng_sigma_tol_flag: u8,
    pub rng_vld_flag: u8,
    pub rng_config_flag: u8,
    pub rng_hw_flag: u8,
    pub rng_meas_type: u8,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
    pub dop_mode_corr_flag: u8,
    pub ul_stn_corr_flag: u8,
    pub stn_cal_corr_flag: u8,
    pub op_subcode6: u8,
    pub ccsds_k: u8,
    pub ccsds_l: u8,
    pub dl_rng_modulo: u32,
}

pub fn tnf_dt5(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt5> {
    if frame.format_code != TNF_FORMAT_DL_PN_RANGING_PHASE || bytes.len() < 408 {
        return None;
    }
    let mut template_id = [0u8; 20];
    template_id.copy_from_slice(&bytes[260..280]);
    let mut def_subcodes = [0u64; 6];
    for i in 0..6 {
        let base = 298 + i * 8;
        def_subcodes[i] = be64(&bytes[base..base + 8]);
    }
    Some(TnfDt5 {
        dl_dss_id: bytes[66],
        dl_band: bytes[67],
        stn_cal: f64::from_be_bytes(bytes[150..158].try_into().ok()?),
        dl_stn_cal: f64::from_be_bytes(bytes[158..166].try_into().ok()?),
        dl_cal_freq: f64::from_be_bytes(bytes[166..174].try_into().ok()?),
        cal_std_dev: f32::from_be_bytes(bytes[174..178].try_into().ok()?),
        cal_pts: be16(&bytes[178..180]),
        dl_rng_phs: f64::from_be_bytes(bytes[180..188].try_into().ok()?),
        figure_merit: f32::from_be_bytes(bytes[188..192].try_into().ok()?),
        rng_resid: f64::from_be_bytes(bytes[192..200].try_into().ok()?),
        drvid: f64::from_be_bytes(bytes[200..208].try_into().ok()?),
        rtlt: f32::from_be_bytes(bytes[208..212].try_into().ok()?),
        pcn0: f32::from_be_bytes(bytes[212..216].try_into().ok()?),
        pcn0_resid: f32::from_be_bytes(bytes[216..220].try_into().ok()?),
        pdn0: f32::from_be_bytes(bytes[220..224].try_into().ok()?),
        pdn0_resid: f32::from_be_bytes(bytes[224..228].try_into().ok()?),
        prn0: f32::from_be_bytes(bytes[228..232].try_into().ok()?),
        prn0_resid: f32::from_be_bytes(bytes[232..236].try_into().ok()?),
        system_noise_temp: f32::from_be_bytes(bytes[236..240].try_into().ok()?),
        state_subcodes: bytes[240..246].try_into().ok()?,
        pn_clk_phs: f64::from_be_bytes(bytes[246..254].try_into().ok()?),
        carr_loop_type: bytes[254],
        snt_flag: bytes[255],
        carr_resid_wt: f32::from_be_bytes(bytes[256..260].try_into().ok()?),
        template_id,
        invert: bytes[280],
        correl_type: bytes[281],
        int_time: be32(&bytes[282..286]),
        clk_divider: bytes[286],
        len_subcodes: bytes[287..293].try_into().ok()?,
        op_subcodes: bytes[293..298].try_into().ok()?,
        def_subcodes,
        pn_code_length: be32(&bytes[346..350]),
        rcv_inphs_year: be16(&bytes[350..352]),
        rcv_inphs_doy: be16(&bytes[352..354]),
        rcv_inphs_sec: f64::from_be_bytes(bytes[354..362].try_into().ok()?),
        exc_scalar_num: be32(&bytes[362..366]),
        exc_scalar_den: be32(&bytes[366..370]),
        rng_cycle_time: f64::from_be_bytes(bytes[370..378].try_into().ok()?),
        inphs_correl: f32::from_be_bytes(bytes[378..382].try_into().ok()?),
        quad_phs_correl: f32::from_be_bytes(bytes[382..386].try_into().ok()?),
        metrics_vld_flag: bytes[386],
        correl_vld_flag: bytes[387],
        rng_resid_tol_flag: bytes[388],
        drvid_tol_flag: bytes[389],
        prn0_resid_tol_flag: bytes[390],
        rng_sigma_tol_flag: bytes[391],
        rng_vld_flag: bytes[392],
        rng_config_flag: bytes[393],
        rng_hw_flag: bytes[394],
        rng_meas_type: bytes[395],
        time_tag_corr_flag: bytes[396],
        type_time_corr_flag: bytes[397],
        dop_mode_corr_flag: bytes[398],
        ul_stn_corr_flag: bytes[399],
        stn_cal_corr_flag: bytes[400],
        op_subcode6: bytes[401],
        ccsds_k: bytes[402],
        ccsds_l: bytes[403],
        dl_rng_modulo: be32(&bytes[404..408]),
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt6 {
    pub dl_dss_id: u8,
    pub ul_band: u8,
    pub ref_rcv_type: u8,
    pub sampl_interval: f32,
    pub rcv_sig_lvl: f32,
    pub ul_freq: f64,
    pub dop_cnt_bias_freq: f64,
    pub dop_cnt: f64,
    pub dop_pseudo_resid: f64,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
    pub dop_mode_corr_flag: u8,
    pub ul_stn_corr_flag: u8,
    pub dl_band_corr_flag: u8,
    pub dop_vld_flag: u8,
}

pub fn tnf_dt6(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt6> {
    if frame.format_code != TNF_FORMAT_DOPPLER_COUNT || bytes.len() < 220 {
        return None;
    }
    Some(TnfDt6 {
        dl_dss_id: bytes[82],
        ul_band: bytes[63],
        ref_rcv_type: bytes[164],
        sampl_interval: f32::from_be_bytes(bytes[166..170].try_into().ok()?),
        rcv_sig_lvl: f32::from_be_bytes(bytes[170..174].try_into().ok()?),
        ul_freq: f64::from_be_bytes(bytes[174..182].try_into().ok()?),
        dop_cnt_bias_freq: f64::from_be_bytes(bytes[182..190].try_into().ok()?),
        dop_cnt: f64::from_be_bytes(bytes[190..198].try_into().ok()?),
        dop_pseudo_resid: f64::from_be_bytes(bytes[198..206].try_into().ok()?),
        time_tag_corr_flag: bytes[206],
        type_time_corr_flag: bytes[207],
        dop_mode_corr_flag: bytes[208],
        ul_stn_corr_flag: bytes[209],
        dl_band_corr_flag: bytes[210],
        dop_vld_flag: bytes[211],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt7 {
    pub dl_dss_id: u8,
    pub ul_band: u8,
    pub ul_stn_cal: f64,
    pub dl_stn_cal: f64,
    pub meas_rng: f64,
    pub rng_obs: f64,
    pub rng_obs_dl: f64,
    pub clock_waveform: u8,
    pub chop_start_num: u8,
    pub figure_merit: f32,
    pub drvid: f64,
    pub rtlt: f32,
    pub prn0: f32,
    pub transmit_pwr: f32,
    pub invert: u8,
    pub correl_type: u8,
    pub t1: u16,
    pub t2: u16,
    pub t3: u16,
    pub first_comp_num: u8,
    pub last_comp_num: u8,
    pub chop_comp_num: u8,
    pub num_drvid: u8,
    pub tx_inphs_time: f32,
    pub rcv_inphs_time: f32,
    pub carr_sup_rng_modul: f32,
    pub exc_scalar_num: u32,
    pub exc_scalar_den: u32,
    pub rng_cycle_time: f64,
    pub rng_modulo: u32,
    pub inphs_correl: f32,
    pub quad_phs_correl: f32,
    pub ul_freq: f64,
    pub rng_type: u8,
    pub fabricated_ul_flag: u8,
    pub rng_noise: f32,
    pub rng_prefit_resid: f64,
    pub rng_dl_prefit_resid: f64,
    pub rng_prefit_resid_vld_flag: u8,
    pub rng_dl_prefit_resid_vld_flag: u8,
    pub rng_resid_tol_value: f32,
    pub drvid_tol_value: f32,
    pub prn0_resid_tol_value: f32,
    pub rng_sigma_tol_value: f32,
    pub fom_tol_value: f32,
    pub rng_resid_tol_flag: u8,
    pub drvid_tol_flag: u8,
    pub prn0_resid_tol_flag: u8,
    pub rng_sigma_tol_flag: u8,
    pub rng_vld_flag: u8,
    pub rng_config_flag: u8,
    pub stn_cal_corr_flag: u8,
    pub rng_chan_num: u8,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
}

pub fn tnf_dt7(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt7> {
    if frame.format_code != TNF_FORMAT_SEQUENTIAL_RANGE || bytes.len() < 350 {
        return None;
    }
    Some(TnfDt7 {
        dl_dss_id: bytes[82],
        ul_band: bytes[63],
        ul_stn_cal: f64::from_be_bytes(bytes[164..172].try_into().ok()?),
        dl_stn_cal: f64::from_be_bytes(bytes[172..180].try_into().ok()?),
        meas_rng: f64::from_be_bytes(bytes[180..188].try_into().ok()?),
        rng_obs: f64::from_be_bytes(bytes[188..196].try_into().ok()?),
        rng_obs_dl: f64::from_be_bytes(bytes[196..204].try_into().ok()?),
        clock_waveform: bytes[204],
        chop_start_num: bytes[205],
        figure_merit: f32::from_be_bytes(bytes[206..210].try_into().ok()?),
        drvid: f64::from_be_bytes(bytes[210..218].try_into().ok()?),
        rtlt: f32::from_be_bytes(bytes[218..222].try_into().ok()?),
        prn0: f32::from_be_bytes(bytes[222..226].try_into().ok()?),
        transmit_pwr: f32::from_be_bytes(bytes[226..230].try_into().ok()?),
        invert: bytes[230],
        correl_type: bytes[231],
        t1: be16(&bytes[232..234]),
        t2: be16(&bytes[234..236]),
        t3: be16(&bytes[236..238]),
        first_comp_num: bytes[238],
        last_comp_num: bytes[239],
        chop_comp_num: bytes[240],
        num_drvid: bytes[241],
        tx_inphs_time: f32::from_be_bytes(bytes[242..246].try_into().ok()?),
        rcv_inphs_time: f32::from_be_bytes(bytes[246..250].try_into().ok()?),
        carr_sup_rng_modul: f32::from_be_bytes(bytes[250..254].try_into().ok()?),
        exc_scalar_num: be32(&bytes[254..258]),
        exc_scalar_den: be32(&bytes[258..262]),
        rng_cycle_time: f64::from_be_bytes(bytes[262..270].try_into().ok()?),
        rng_modulo: be32(&bytes[270..274]),
        inphs_correl: f32::from_be_bytes(bytes[274..278].try_into().ok()?),
        quad_phs_correl: f32::from_be_bytes(bytes[278..282].try_into().ok()?),
        ul_freq: f64::from_be_bytes(bytes[282..290].try_into().ok()?),
        rng_type: bytes[290],
        fabricated_ul_flag: bytes[291],
        rng_noise: f32::from_be_bytes(bytes[292..296].try_into().ok()?),
        rng_prefit_resid: f64::from_be_bytes(bytes[296..304].try_into().ok()?),
        rng_dl_prefit_resid: f64::from_be_bytes(bytes[304..312].try_into().ok()?),
        rng_prefit_resid_vld_flag: bytes[312],
        rng_dl_prefit_resid_vld_flag: bytes[313],
        rng_resid_tol_value: f32::from_be_bytes(bytes[314..318].try_into().ok()?),
        drvid_tol_value: f32::from_be_bytes(bytes[318..322].try_into().ok()?),
        prn0_resid_tol_value: f32::from_be_bytes(bytes[322..326].try_into().ok()?),
        rng_sigma_tol_value: f32::from_be_bytes(bytes[326..330].try_into().ok()?),
        fom_tol_value: f32::from_be_bytes(bytes[330..334].try_into().ok()?),
        rng_resid_tol_flag: bytes[334],
        drvid_tol_flag: bytes[335],
        prn0_resid_tol_flag: bytes[336],
        rng_sigma_tol_flag: bytes[337],
        rng_vld_flag: bytes[338],
        rng_config_flag: bytes[339],
        stn_cal_corr_flag: bytes[340],
        rng_chan_num: bytes[341],
        time_tag_corr_flag: bytes[342],
        type_time_corr_flag: bytes[343],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt8 {
    pub dl_dss_id: u8,
    pub ul_band: u8,
    pub source_type: u8,
    pub ang_type: u8,
    pub ang_vld_flag: u8,
    pub ang_mode: u8,
    pub conscan_mode: u8,
    pub acq_aid_mode: u8,
    pub ang1: f32,
    pub ang2: f32,
    pub ang1_pseudo_resid: f32,
    pub ang2_pseudo_resid: f32,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
}

pub fn tnf_dt8(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt8> {
    if frame.format_code != TNF_FORMAT_ANGLE || bytes.len() < 198 {
        return None;
    }
    Some(TnfDt8 {
        dl_dss_id: bytes[82],
        ul_band: bytes[63],
        source_type: bytes[164],
        ang_type: bytes[165],
        ang_vld_flag: bytes[166],
        ang_mode: bytes[167],
        conscan_mode: bytes[168],
        acq_aid_mode: bytes[169],
        ang1: f32::from_be_bytes(bytes[170..174].try_into().ok()?),
        ang2: f32::from_be_bytes(bytes[174..178].try_into().ok()?),
        ang1_pseudo_resid: f32::from_be_bytes(bytes[178..182].try_into().ok()?),
        ang2_pseudo_resid: f32::from_be_bytes(bytes[182..186].try_into().ok()?),
        time_tag_corr_flag: bytes[186],
        type_time_corr_flag: bytes[187],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt9 {
    pub ul_dss_id: u8,
    pub ul_band: u8,
    pub ul_hi_phs_cycles: u32,
    pub ul_lo_phs_cycles: u32,
    pub ul_frac_phs_cycles: u32,
    pub ramp_freq: f64,
    pub ramp_rate: f64,
    pub ramp_type: u8,
    pub fabricated_sfdu_flag: u8,
}

pub fn tnf_dt9(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt9> {
    if frame.format_code != TNF_FORMAT_RAMP || bytes.len() < 144 {
        return None;
    }
    Some(TnfDt9 {
        ul_dss_id: bytes[66],
        ul_band: bytes[67],
        ul_hi_phs_cycles: be32(&bytes[106..110]),
        ul_lo_phs_cycles: be32(&bytes[110..114]),
        ul_frac_phs_cycles: be32(&bytes[114..118]),
        ramp_freq: f64::from_be_bytes(bytes[118..126].try_into().ok()?),
        ramp_rate: f64::from_be_bytes(bytes[126..134].try_into().ok()?),
        ramp_type: bytes[134],
        fabricated_sfdu_flag: bytes[135],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt10 {
    pub ul_dss_id: u8,
    pub dl_dss_id: u8,
    pub dl_dss_id_2: u8,
    pub dl_band: u8,
    pub clk_off_epoch_year: u16,
    pub clk_off_epoch_doy: u16,
    pub clk_off_epoch_sec: f64,
    pub clk_off_1: f32,
    pub clk_off_2: f32,
    pub phs_cal_flag: u8,
    pub chan_sampl_flag: u8,
    pub quasar_id: [u8; 12],
    pub quasar_id_num: u16,
    pub data_qual_flag: u8,
    pub freq_chan_num: u8,
    pub mode_id: u8,
    pub modulo_flag: u8,
    pub ref_freq: f64,
    pub modulus: f64,
    pub dod_cnt_time: f32,
    pub dod_obs: f64,
    pub dor_obs: f64,
}

pub fn tnf_dt10(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt10> {
    if frame.format_code != TNF_FORMAT_VLBI || bytes.len() < 224 {
        return None;
    }
    let mut quasar_id = [0u8; 12];
    quasar_id.copy_from_slice(&bytes[150..162]);
    Some(TnfDt10 {
        ul_dss_id: bytes[62],
        dl_dss_id: bytes[63],
        dl_dss_id_2: bytes[64],
        dl_band: bytes[65],
        clk_off_epoch_year: be16(&bytes[128..130]),
        clk_off_epoch_doy: be16(&bytes[130..132]),
        clk_off_epoch_sec: f64::from_be_bytes(bytes[132..140].try_into().ok()?),
        clk_off_1: f32::from_be_bytes(bytes[140..144].try_into().ok()?),
        clk_off_2: f32::from_be_bytes(bytes[144..148].try_into().ok()?),
        phs_cal_flag: bytes[148],
        chan_sampl_flag: bytes[149],
        quasar_id,
        quasar_id_num: be16(&bytes[162..164]),
        data_qual_flag: bytes[164],
        freq_chan_num: bytes[165],
        mode_id: bytes[166],
        modulo_flag: bytes[167],
        ref_freq: f64::from_be_bytes(bytes[168..176].try_into().ok()?),
        modulus: f64::from_be_bytes(bytes[176..184].try_into().ok()?),
        dod_cnt_time: f32::from_be_bytes(bytes[184..188].try_into().ok()?),
        dod_obs: f64::from_be_bytes(bytes[188..196].try_into().ok()?),
        dor_obs: f64::from_be_bytes(bytes[196..204].try_into().ok()?),
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt11 {
    pub dl_dss_id: u8,
    pub ul_band: u8,
    pub drvid_type: u8,
    pub drvid_pts: u8,
    pub drvid: f64,
    pub prn0: f32,
    pub drvid_noise: f32,
    pub drvid_tol_value: f32,
    pub prn0_resid_tol_value: f32,
    pub drvid_tol_flag: u8,
    pub prn0_resid_tol_flag: u8,
    pub drvid_noise_pts: u8,
}

pub fn tnf_dt11(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt11> {
    if frame.format_code != TNF_FORMAT_DRVID || bytes.len() < 202 {
        return None;
    }
    Some(TnfDt11 {
        dl_dss_id: bytes[82],
        ul_band: bytes[63],
        drvid_type: bytes[164],
        drvid_pts: bytes[165],
        drvid: f64::from_be_bytes(bytes[166..174].try_into().ok()?),
        prn0: f32::from_be_bytes(bytes[174..178].try_into().ok()?),
        drvid_noise: f32::from_be_bytes(bytes[178..182].try_into().ok()?),
        drvid_tol_value: f32::from_be_bytes(bytes[182..186].try_into().ok()?),
        prn0_resid_tol_value: f32::from_be_bytes(bytes[186..190].try_into().ok()?),
        drvid_tol_flag: bytes[191],
        prn0_resid_tol_flag: bytes[192],
        drvid_noise_pts: bytes[193],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt12 {
    pub dl_dss_id: u8,
    pub dl_band: u8,
    pub noise_01sec: f32,
    pub noise_1sec: f32,
    pub noise_10sec: f32,
    pub noise_100sec: f32,
    pub noise_200sec: f32,
    pub noise_600sec: f32,
    pub int_time: u32,
    pub percent_data_used: f32,
    pub new_01sec: u8,
    pub new_1sec: u8,
    pub new_10sec: u8,
    pub new_100sec: u8,
    pub new_200sec: u8,
    pub new_600sec: u8,
}

pub fn tnf_dt12(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt12> {
    if frame.format_code != TNF_FORMAT_SMOOTHED_NOISE || bytes.len() < 184 {
        return None;
    }
    Some(TnfDt12 {
        dl_dss_id: bytes[62],
        dl_band: bytes[63],
        noise_01sec: f32::from_be_bytes(bytes[138..142].try_into().ok()?),
        noise_1sec: f32::from_be_bytes(bytes[142..146].try_into().ok()?),
        noise_10sec: f32::from_be_bytes(bytes[146..150].try_into().ok()?),
        noise_100sec: f32::from_be_bytes(bytes[150..154].try_into().ok()?),
        noise_200sec: f32::from_be_bytes(bytes[154..158].try_into().ok()?),
        noise_600sec: f32::from_be_bytes(bytes[158..162].try_into().ok()?),
        int_time: be32(&bytes[162..166]),
        percent_data_used: f32::from_be_bytes(bytes[166..170].try_into().ok()?),
        new_01sec: bytes[170],
        new_1sec: bytes[171],
        new_10sec: bytes[172],
        new_100sec: bytes[173],
        new_200sec: bytes[174],
        new_600sec: bytes[175],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt13 {
    pub dl_dss_id: u8,
    pub dl_band: u8,
    pub allan_01sec: f32,
    pub allan_1sec: f32,
    pub allan_10sec: f32,
    pub allan_100sec: f32,
    pub allan_1000sec: f32,
    pub int_time: u32,
    pub percent_data_used: f32,
    pub rpt_cause: u8,
    pub new_01sec: u8,
    pub new_1sec: u8,
    pub new_10sec: u8,
    pub new_100sec: u8,
    pub new_1000sec: u8,
}

pub fn tnf_dt13(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt13> {
    if frame.format_code != TNF_FORMAT_ALLAN_DEVIATION || bytes.len() < 180 {
        return None;
    }
    Some(TnfDt13 {
        dl_dss_id: bytes[62],
        dl_band: bytes[63],
        allan_01sec: f32::from_be_bytes(bytes[138..142].try_into().ok()?),
        allan_1sec: f32::from_be_bytes(bytes[142..146].try_into().ok()?),
        allan_10sec: f32::from_be_bytes(bytes[146..150].try_into().ok()?),
        allan_100sec: f32::from_be_bytes(bytes[150..154].try_into().ok()?),
        allan_1000sec: f32::from_be_bytes(bytes[154..158].try_into().ok()?),
        int_time: be32(&bytes[158..162]),
        percent_data_used: f32::from_be_bytes(bytes[162..166].try_into().ok()?),
        rpt_cause: bytes[166],
        new_01sec: bytes[167],
        new_1sec: bytes[168],
        new_10sec: bytes[169],
        new_100sec: bytes[170],
        new_1000sec: bytes[171],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt14 {
    pub dl_dss_id: u8,
    pub ul_band: u8,
    pub ul_stn_cal: f64,
    pub dl_stn_cal: f64,
    pub meas_rng: f64,
    pub rng_obs_dl: f64,
    pub figure_merit: f32,
    pub drvid: f64,
    pub rtlt: f32,
    pub prn0: f32,
    pub transmit_pwr: f32,
    pub invert: u8,
    pub correl_type: u8,
    pub clk_divider: u8,
    pub len_subcodes: [u8; 6],
    pub op_subcodes: [u8; 5],
    pub def_subcodes: [u64; 6],
    pub pn_code_length: u32,
    pub tx_inphs_time: f32,
    pub rcv_inphs_time: f32,
    pub carr_sup_rng_modul: f32,
    pub exc_scalar_num: u32,
    pub exc_scalar_den: u32,
    pub rng_cycle_time: f64,
    pub rng_modulo: u32,
    pub rng_type: u8,
    pub fabricated_ul_flag: u8,
    pub rng_noise: f32,
    pub rng_obs: f64,
    pub rng_dl_prefit_resid_vld_flag: u8,
    pub clock_waveform: u8,
    pub rng_resid_tol_value: f32,
    pub drvid_tol_value: f32,
    pub prn0_resid_tol_value: f32,
    pub rng_sigma_tol_value: f32,
    pub fom_tol_value: f32,
    pub rng_resid_tol_flag: u8,
    pub drvid_tol_flag: u8,
    pub prn0_resid_tol_flag: u8,
    pub rng_sigma_tol_flag: u8,
    pub rng_vld_flag: u8,
    pub rng_config_flag: u8,
    pub stn_cal_corr_flag: u8,
    pub op_subcode6: u8,
    pub ccsds_k: u8,
    pub ccsds_l: u8,
    pub rng_dl_prefit_resid: f64,
}

pub fn tnf_dt14(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt14> {
    if frame.format_code != TNF_FORMAT_PN_RANGE || bytes.len() < 372 {
        return None;
    }
    let mut def_subcodes = [0u64; 6];
    for i in 0..6 {
        let base = 234 + i * 8;
        def_subcodes[i] = be64(&bytes[base..base + 8]);
    }
    Some(TnfDt14 {
        dl_dss_id: bytes[82],
        ul_band: bytes[63],
        ul_stn_cal: f64::from_be_bytes(bytes[164..172].try_into().ok()?),
        dl_stn_cal: f64::from_be_bytes(bytes[172..180].try_into().ok()?),
        meas_rng: f64::from_be_bytes(bytes[180..188].try_into().ok()?),
        rng_obs_dl: f64::from_be_bytes(bytes[188..196].try_into().ok()?),
        figure_merit: f32::from_be_bytes(bytes[196..200].try_into().ok()?),
        drvid: f64::from_be_bytes(bytes[200..208].try_into().ok()?),
        rtlt: f32::from_be_bytes(bytes[208..212].try_into().ok()?),
        prn0: f32::from_be_bytes(bytes[212..216].try_into().ok()?),
        transmit_pwr: f32::from_be_bytes(bytes[216..220].try_into().ok()?),
        invert: bytes[220],
        correl_type: bytes[221],
        clk_divider: bytes[222],
        len_subcodes: bytes[223..229].try_into().ok()?,
        op_subcodes: bytes[229..234].try_into().ok()?,
        def_subcodes,
        pn_code_length: be32(&bytes[282..286]),
        tx_inphs_time: f32::from_be_bytes(bytes[286..290].try_into().ok()?),
        rcv_inphs_time: f32::from_be_bytes(bytes[290..294].try_into().ok()?),
        carr_sup_rng_modul: f32::from_be_bytes(bytes[294..298].try_into().ok()?),
        exc_scalar_num: be32(&bytes[298..302]),
        exc_scalar_den: be32(&bytes[302..306]),
        rng_cycle_time: f64::from_be_bytes(bytes[306..314].try_into().ok()?),
        rng_modulo: be32(&bytes[314..318]),
        rng_type: bytes[318],
        fabricated_ul_flag: bytes[319],
        rng_noise: f32::from_be_bytes(bytes[320..324].try_into().ok()?),
        rng_obs: f64::from_be_bytes(bytes[324..332].try_into().ok()?),
        rng_dl_prefit_resid_vld_flag: bytes[332],
        clock_waveform: bytes[333],
        rng_resid_tol_value: f32::from_be_bytes(bytes[334..338].try_into().ok()?),
        drvid_tol_value: f32::from_be_bytes(bytes[338..342].try_into().ok()?),
        prn0_resid_tol_value: f32::from_be_bytes(bytes[342..346].try_into().ok()?),
        rng_sigma_tol_value: f32::from_be_bytes(bytes[346..350].try_into().ok()?),
        fom_tol_value: f32::from_be_bytes(bytes[350..354].try_into().ok()?),
        rng_resid_tol_flag: bytes[354],
        drvid_tol_flag: bytes[355],
        prn0_resid_tol_flag: bytes[356],
        rng_sigma_tol_flag: bytes[357],
        rng_vld_flag: bytes[358],
        rng_config_flag: bytes[359],
        stn_cal_corr_flag: bytes[360],
        op_subcode6: bytes[361],
        ccsds_k: bytes[362],
        ccsds_l: bytes[363],
        rng_dl_prefit_resid: f64::from_be_bytes(bytes[364..372].try_into().ok()?),
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt15 {
    pub dl_dss_id: u8,
    pub ul_band: u8,
    pub source_type: u8,
    pub mjr_tone_freq: u8,
    pub mnr_tone_freq: u8,
    pub rng_prefit_resid_vld_flag: u8,
    pub meas_rng: f64,
    pub rng_obs: f64,
    pub stn_cal: f64,
    pub carr_pwr: f32,
    pub rng_prefit_resid: f64,
    pub ul_freq: f64,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
}

pub fn tnf_dt15(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt15> {
    if frame.format_code != TNF_FORMAT_TONE_RANGE || bytes.len() < 214 {
        return None;
    }
    Some(TnfDt15 {
        dl_dss_id: bytes[82],
        ul_band: bytes[63],
        source_type: bytes[164],
        mjr_tone_freq: bytes[165],
        mnr_tone_freq: bytes[166],
        rng_prefit_resid_vld_flag: bytes[167],
        meas_rng: f64::from_be_bytes(bytes[168..176].try_into().ok()?),
        rng_obs: f64::from_be_bytes(bytes[176..184].try_into().ok()?),
        stn_cal: f64::from_be_bytes(bytes[184..192].try_into().ok()?),
        carr_pwr: f32::from_be_bytes(bytes[192..196].try_into().ok()?),
        rng_prefit_resid: f64::from_be_bytes(bytes[196..204].try_into().ok()?),
        ul_freq: f64::from_be_bytes(bytes[204..212].try_into().ok()?),
        time_tag_corr_flag: bytes[212],
        type_time_corr_flag: bytes[213],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt16 {
    pub dl_dss_id: u8,
    pub ul_band: u8,
    pub ref_rcv_type: u8,
    pub fabricated_ul_flag: u8,
    pub carr_prefit_resid_tol_value: f32,
    pub dop_noise: f32,
    pub delta_ff: f64,
    pub rcv_sig_lvl: f32,
    pub num_obs: u16,
    pub obs_cnt_time: f32,
    pub rcv_carr_obs: f64,
    pub carr_prefit_resid: f32,
    pub carr_prefit_resid_vld_flag: u8,
    pub carr_prefit_resid_tol_flag: u8,
    pub carr_resid_wt: f32,
}

pub fn tnf_dt16(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt16> {
    if frame.format_code != TNF_FORMAT_CARRIER_OBSERVABLE || bytes.len() < 220 {
        return None;
    }
    Some(TnfDt16 {
        dl_dss_id: bytes[82],
        ul_band: bytes[63],
        ref_rcv_type: bytes[164],
        fabricated_ul_flag: bytes[165],
        carr_prefit_resid_tol_value: f32::from_be_bytes(bytes[166..170].try_into().ok()?),
        dop_noise: f32::from_be_bytes(bytes[172..176].try_into().ok()?),
        delta_ff: f64::from_be_bytes(bytes[176..184].try_into().ok()?),
        rcv_sig_lvl: f32::from_be_bytes(bytes[184..188].try_into().ok()?),
        num_obs: be16(&bytes[188..190]),
        obs_cnt_time: f32::from_be_bytes(bytes[190..194].try_into().ok()?),
        rcv_carr_obs: f64::from_be_bytes(bytes[194..202].try_into().ok()?),
        carr_prefit_resid: f32::from_be_bytes(bytes[202..206].try_into().ok()?),
        carr_prefit_resid_vld_flag: bytes[206],
        carr_prefit_resid_tol_flag: bytes[207],
        carr_resid_wt: f32::from_be_bytes(bytes[208..212].try_into().ok()?),
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt17 {
    pub dl_dss_id: u8,
    pub ul_band: u8,
    pub ref_rcv_type: u8,
    pub fabricated_ul_flag: u8,
    pub total_cnt_phs_prefit_resid_tol_value: f32,
    pub dop_noise: f32,
    pub delta_ff: f64,
    pub rcv_sig_lvl: f32,
    pub num_obs: u16,
    pub obs_cnt_time: f32,
    pub st_year: u16,
    pub st_doy: u16,
    pub st_sec: f64,
    pub obs_hi: u32,
    pub obs_lo: u32,
    pub obs_frac: u32,
    pub prefit_resid: f32,
    pub prefit_resid_vld_flag: u8,
    pub prefit_resid_tol_flag: u8,
    pub carr_resid_wt: f32,
}

pub fn tnf_dt17(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt17> {
    if frame.format_code != TNF_FORMAT_TOTAL_PHASE_OBSERVABLE || bytes.len() < 236 {
        return None;
    }
    Some(TnfDt17 {
        dl_dss_id: bytes[82],
        ul_band: bytes[63],
        ref_rcv_type: bytes[164],
        fabricated_ul_flag: bytes[165],
        total_cnt_phs_prefit_resid_tol_value: f32::from_be_bytes(bytes[166..170].try_into().ok()?),
        dop_noise: f32::from_be_bytes(bytes[172..176].try_into().ok()?),
        delta_ff: f64::from_be_bytes(bytes[176..184].try_into().ok()?),
        rcv_sig_lvl: f32::from_be_bytes(bytes[184..188].try_into().ok()?),
        num_obs: be16(&bytes[188..190]),
        obs_cnt_time: f32::from_be_bytes(bytes[190..194].try_into().ok()?),
        st_year: be16(&bytes[194..196]),
        st_doy: be16(&bytes[196..198]),
        st_sec: f64::from_be_bytes(bytes[198..206].try_into().ok()?),
        obs_hi: be32(&bytes[206..210]),
        obs_lo: be32(&bytes[210..214]),
        obs_frac: be32(&bytes[214..218]),
        prefit_resid: f32::from_be_bytes(bytes[218..222].try_into().ok()?),
        prefit_resid_vld_flag: bytes[222],
        prefit_resid_tol_flag: bytes[223],
        carr_resid_wt: f32::from_be_bytes(bytes[224..228].try_into().ok()?),
    })
}

pub const TNF_ROW_TDB: usize = 0;
pub const TNF_ROW_OBSERVABLE: usize = 1;
pub const TNF_ROW_SUPPORT: usize = 2;
pub const TNF_ROW_DSS: usize = 3;
pub const TNF_ROW_SCFT: usize = 4;
pub const TNF_ROW_FORMAT: usize = 5;
pub const TNF_ROW_BAND: usize = 6;
pub const TNF_ROW_SEQ: usize = 7;
pub const TNF_ROW_TERTIARY: usize = 8;

fn tnf_row(frame: &TnfSfdu, bytes: &[u8], tdb: f64) -> Option<[f64; 9]> {
    let fcode = frame.format_code as f64;
    let scft = frame.scft_id as f64;
    let seq = frame.rec_seq_num as f64;
    match frame.format_code {
        TNF_FORMAT_UL_CARRIER_PHASE => {
            let d = tnf_dt0(frame, bytes)?;
            Some([
                tdb,
                tnf_ul_phase_cycles(&d),
                d.ramp_freq,
                d.ul_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.prdx_time_offset,
            ])
        }
        TNF_FORMAT_DL_CARRIER_PHASE => {
            let d = tnf_dt1(frame, bytes)?;
            Some([
                tdb,
                tnf_phase_cycles(d.phase_avg[0], d.phase_avg[1], d.phase_avg[2]),
                d.dl_freq,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.dl_band as f64,
                seq,
                d.prdx_time_offset,
            ])
        }
        TNF_FORMAT_UL_SEQ_RANGING_PHASE => {
            let d = tnf_dt2(frame, bytes)?;
            Some([
                tdb,
                d.ul_rng_phs,
                d.ul_cal_freq,
                d.ul_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.cal_std_dev as f64,
            ])
        }
        TNF_FORMAT_DL_SEQ_RANGING_PHASE => {
            let d = tnf_dt3(frame, bytes)?;
            Some([
                tdb,
                d.dl_rng_phs,
                d.dl_cal_freq,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.dl_band as f64,
                seq,
                d.rng_resid,
            ])
        }
        TNF_FORMAT_UL_PN_RANGING_PHASE => {
            let d = tnf_dt4(frame, bytes)?;
            Some([
                tdb,
                d.ul_rng_phs,
                d.pn_clk_phs,
                d.ul_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.transmit_op_pwr as f64,
            ])
        }
        TNF_FORMAT_DL_PN_RANGING_PHASE => {
            let d = tnf_dt5(frame, bytes)?;
            Some([
                tdb,
                d.dl_rng_phs,
                d.pn_clk_phs,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.dl_band as f64,
                seq,
                d.rng_resid,
            ])
        }
        TNF_FORMAT_DOPPLER_COUNT => {
            let d = tnf_dt6(frame, bytes)?;
            Some([
                tdb,
                d.dop_cnt,
                d.ul_freq,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.dop_pseudo_resid,
            ])
        }
        TNF_FORMAT_SEQUENTIAL_RANGE => {
            let d = tnf_dt7(frame, bytes)?;
            Some([
                tdb,
                d.meas_rng,
                d.drvid,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.rng_prefit_resid,
            ])
        }
        TNF_FORMAT_ANGLE => {
            let d = tnf_dt8(frame, bytes)?;
            Some([
                tdb,
                d.ang1 as f64,
                d.ang2 as f64,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.ang1_pseudo_resid as f64,
            ])
        }
        TNF_FORMAT_RAMP => {
            let d = tnf_dt9(frame, bytes)?;
            Some([
                tdb,
                tnf_phase_cycles(d.ul_hi_phs_cycles, d.ul_lo_phs_cycles, d.ul_frac_phs_cycles),
                d.ramp_freq,
                d.ul_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.ramp_rate,
            ])
        }
        TNF_FORMAT_VLBI => {
            let d = tnf_dt10(frame, bytes)?;
            Some([
                tdb,
                d.dod_obs,
                d.dor_obs,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.dl_band as f64,
                seq,
                d.ref_freq,
            ])
        }
        TNF_FORMAT_DRVID => {
            let d = tnf_dt11(frame, bytes)?;
            Some([
                tdb,
                d.drvid,
                d.prn0 as f64,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.drvid_noise as f64,
            ])
        }
        TNF_FORMAT_SMOOTHED_NOISE => {
            let d = tnf_dt12(frame, bytes)?;
            Some([
                tdb,
                d.noise_01sec as f64,
                d.noise_1sec as f64,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.dl_band as f64,
                seq,
                d.int_time as f64,
            ])
        }
        TNF_FORMAT_ALLAN_DEVIATION => {
            let d = tnf_dt13(frame, bytes)?;
            Some([
                tdb,
                d.allan_01sec as f64,
                d.allan_1sec as f64,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.dl_band as f64,
                seq,
                d.int_time as f64,
            ])
        }
        TNF_FORMAT_PN_RANGE => {
            let d = tnf_dt14(frame, bytes)?;
            Some([
                tdb,
                d.meas_rng,
                d.drvid,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.rng_dl_prefit_resid,
            ])
        }
        TNF_FORMAT_TONE_RANGE => {
            let d = tnf_dt15(frame, bytes)?;
            Some([
                tdb,
                d.meas_rng,
                d.rng_obs,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.rng_prefit_resid,
            ])
        }
        TNF_FORMAT_CARRIER_OBSERVABLE => {
            let d = tnf_dt16(frame, bytes)?;
            Some([
                tdb,
                d.rcv_carr_obs,
                d.delta_ff,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.obs_cnt_time as f64,
            ])
        }
        TNF_FORMAT_TOTAL_PHASE_OBSERVABLE => {
            let d = tnf_dt17(frame, bytes)?;
            if d.num_obs < 1 {
                return None;
            }
            Some([
                tdb,
                tnf_phase_cycles(d.obs_hi, d.obs_lo, d.obs_frac),
                d.delta_ff,
                d.dl_dss_id as f64,
                scft,
                fcode,
                d.ul_band as f64,
                seq,
                d.obs_cnt_time as f64,
            ])
        }
        _ => None,
    }
}

pub fn tnf_rows(bytes: &[u8], lsk: &crate::archivar::lsk::LeapSeconds) -> Option<Vec<[f64; 9]>> {
    let frames = scan_tnf_sfdus(bytes)?;
    let mut out = Vec::with_capacity(frames.len());
    for f in &frames {
        let Some(day0) = crate::archivar::lsk::days_from_civil(f.year as i64, 1, 1) else {
            continue;
        };
        let unix = day0 as f64 * 86400.0 + (f.doy as f64 - 1.0) * 86400.0 + f.sec;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let sfdu = &bytes[f.offset..f.offset + f.total_len];
        let Some(row) = tnf_row(f, sfdu, tdb) else {
            continue;
        };
        out.push(row);
    }
    Some(out)
}

pub fn tnf_parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_podf_bin(bytes)?;
    let mut out = Vec::with_capacity(rows.len());
    for r in &rows {
        if r[TNF_ROW_FORMAT] != 0.0 {
            continue;
        }
        let t = r[PODF_COL_TDB];
        let v = r[PODF_COL_OBSERVABLE];
        if !t.is_finite() || !v.is_finite() {
            continue;
        }
        out.push((t, v, TNF_COMP_UL_PHASE));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_words() -> [u32; 9] {
        [
            0x6C028048, 0x00000000, 0xFFFA28EE, 0xD86F2B24, 0x4FC005C4, 0x02764217, 0x77808DE8,
            0x00000005, 0xDC000000,
        ]
    }

    fn galileo_delta_dor_words() -> [u32; 9] {
        [
            0x578AF474, 0x00000000, 0x000010CD, 0x10877FE6, 0x23802830, 0x9A000FC0, 0x00015F90,
            0x0DADE373, 0x1300000B,
        ]
    }

    #[test]
    fn golden_record_unpacks() {
        let r = orbit_record(&example_words()).unwrap();
        assert!((r.t_since_1950 - 1812103240.0).abs() < 1.0e-9);
        assert!((r.observable_hz - (-382738.663803100)).abs() < 1.0e-6);
        assert_eq!(r.dss_rx, 63);
        assert_eq!(r.dss_tx, 0);
        assert_eq!(r.data_type, 11);
        assert_eq!(r.downlink_band, 2);
        assert!(r.valid);
        assert_eq!(r.scid, 236);
        assert!((r.ref_hz - 2299812417.0).abs() < 1.0e-6);
        assert!((r.compression_s - 60.0).abs() < 1.0e-9);
    }

    #[test]
    fn golden_file_parses() {
        let bytes = std::fs::read("src/archivar/kernels/odf07155.dat").unwrap();
        let recs = parse_odf(&bytes).unwrap();
        assert!(!recs.is_empty());
        let first = recs[0];
        assert!((first.t_since_1950 - 1812103240.0).abs() < 1.0e-9);
        assert_eq!(first.data_type, 11);
    }

    #[test]
    fn real_galileo_delta_dor_record_decodes() {
        let r = orbit_record(&galileo_delta_dor_words()).unwrap();
        assert!((r.t_since_1950 - 1468724340.0).abs() < 1.0e-9);
        assert!((r.observable_hz - 4301.277315558).abs() < 1.0e-6);
        assert_eq!(r.dss_rx, 14);
        assert_eq!(r.dss_tx, 0);
        assert_eq!(r.data_type, 1);
        assert_eq!(r.downlink_band, 1);
        assert_eq!(r.uplink_band, 0);
        assert!(r.valid);
        assert_eq!(r.scid, 77);
        assert!((r.ref_hz - 2294997631.9).abs() < 1.0e-6);
        assert!((r.compression_s - 900.0).abs() < 1.0e-9);
    }

    fn set_bits(words: &mut [u32; 9], first: usize, last: usize, value: i64) {
        let width = last - first + 1;
        for b in 0..width {
            let bitpos = first + b;
            let word = (bitpos - 1) / 32;
            let wbit = (bitpos - 1) % 32;
            let mask = 1u32 << (31 - wbit);
            if (value >> (width - 1 - b)) & 1 == 1 {
                words[word] |= mask;
            } else {
                words[word] &= !mask;
            }
        }
    }

    #[test]
    fn format_1_record_decodes_from_the_1988_sis_layout() {
        let mut w = [0u32; 9];
        w[0] = 0x6C028048;
        set_bits(&mut w, 129, 131, 1);
        set_bits(&mut w, 150, 155, 12);
        set_bits(&mut w, 148, 149, 1);
        set_bits(&mut w, 187, 188, 1);
        set_bits(&mut w, 200, 200, 0);
        set_bits(&mut w, 161, 167, 77);
        set_bits(&mut w, 225, 256, 229_981_241);
        set_bits(&mut w, 257, 264, 7);
        set_bits(&mut w, 201, 224, 6000);
        let r = orbit_record(&w).unwrap();
        assert_eq!(r.data_type, 12);
        assert_eq!(r.downlink_band, 1);
        assert_eq!(r.uplink_band, 1);
        assert!(r.valid);
        assert_eq!(r.scid, 77);
        assert!((r.ref_hz - 2299812410.7).abs() < 1.0e-6);
        assert!((r.compression_s - 60.0).abs() < 1.0e-9);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_odf(b"X").is_none());
    }

    #[test]
    fn podf_roundtrip() {
        let recs = vec![[1.0, 2.0, 3.0, 14.0, 43.0, 12.0, 1.0, 24.0, 60.0]];
        let bytes = write_podf_bin(&recs);
        let parsed = parse_podf_bin(&bytes).unwrap();
        assert_eq!(parsed, recs);
        assert!(parse_podf_bin(b"X").is_none());
    }

    #[test]
    fn shard_ranges_cover_and_respect_budget() {
        let ranges = podf_shard_ranges(5, 160);
        assert_eq!(ranges, vec![(0, 2), (2, 4), (4, 5)]);
        for &(lo, hi) in &ranges {
            assert!(lo < hi);
            assert!(8 + (hi - lo) * 72 <= 160);
        }
        let mut covered: Vec<usize> = Vec::new();
        for &(lo, hi) in &ranges {
            covered.extend(lo..hi);
        }
        assert_eq!(covered, (0..5).collect::<Vec<usize>>());
    }

    #[test]
    fn shard_ranges_empty_for_zero_records() {
        assert!(podf_shard_ranges(0, 1 << 30).is_empty());
    }

    #[test]
    fn shard_ranges_single_oversize_record_is_one_shard() {
        assert_eq!(podf_shard_ranges(1, 10), vec![(0, 1)]);
    }

    #[test]
    fn shard_names_unique_for_multi_shard_partition() {
        let names = [
            podf_shard_name("mro_odf", 700_000_000.0, 800_000_000.0),
            podf_shard_name("mro_odf", 800_000_000.0, 900_000_000.0),
            podf_shard_name("mro_odf", 900_000_000.0, 1_000_000_000.0),
        ];
        let mut set = std::collections::BTreeSet::new();
        for n in names {
            assert!(set.insert(n), "duplicate shard name");
        }
    }

    #[test]
    fn shard_name_ordinal_disambiguates_equal_tdb_seconds() {
        assert_eq!(
            podf_shard_name("mro_odf", 123_456_789.2, 123_456_789.2),
            podf_shard_name("mro_odf", 123_456_789.8, 123_456_789.8)
        );
        assert_ne!(
            podf_shard_name_ord("mro_odf", 123_456_789.2, 123_456_789.2, 0),
            podf_shard_name_ord("mro_odf", 123_456_789.8, 123_456_789.8, 1)
        );
    }

    #[test]
    fn shard_name_roundtrips_through_the_parser() {
        let name = podf_shard_name("mars_express_odf", 700_000_000.0, 800_000_000.0);
        assert_eq!(
            parse_podf_shard_name(&name),
            Some(("mars_express_odf", 700_000_000.0, 800_000_000.0))
        );
        let ord = podf_shard_name_ord("mro_odf", 123_456_789.2, 123_456_789.8, 1);
        assert_eq!(
            parse_podf_shard_name(&ord),
            Some(("mro_odf", 123_456_789.0, 123_456_789.0))
        );
        assert_eq!(parse_podf_shard_name("mro_odf.bin"), None);
        assert_eq!(parse_podf_shard_name("mro_odf_t700.bin"), None);
    }

    #[test]
    fn shard_roundtrip_reconstructs_record_order() {
        let mut merged: Vec<[f64; 9]> = Vec::new();
        for i in 0..10 {
            let t = i as f64 * 100.0;
            merged.push([t, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        }
        let ranges = podf_shard_ranges(merged.len(), 500);
        assert!(ranges.len() > 1);
        let mut reconstructed: Vec<[f64; 9]> = Vec::new();
        for &(lo, hi) in &ranges {
            let bin = write_podf_bin(&merged[lo..hi]);
            let parsed = parse_podf_bin(&bin).unwrap();
            reconstructed.extend_from_slice(&parsed);
        }
        assert_eq!(reconstructed, merged);
    }

    #[test]
    fn p11r_roundtrip() {
        let recs = vec![[-8.0e8, 12.5, 2.3e9, 2.3e9, -1.7e4, 43.0, 43.0, 12.0, 60.0]];
        let bytes = write_p11r_bin(&recs);
        let parsed = parse_p11r_bin(&bytes).unwrap();
        assert_eq!(parsed, recs);
        assert!(parse_p11r_bin(b"X").is_none());
    }

    const NHREX_TNF_HEAD_2: &str = "4e4a504c324930304331323300000000000000a20001004e00020004060e18000084004230310062000000000000000007dc001540e77da000000000506304b6727a1a0201010000000001003f142f61ed5ae1ce344b7abd0000000000002200020000000000000a004c0042cd063881085a59b3cfd941faa3a0f766b3b600000000000000000000388c38480000000000000000000000000000000000000000000000000000000000000000000000000000000000004e4a504c324930304331323300000000000000a20001004e00020004060e18000084004230310062000000010000000107dc001540e77dc000000000506304b6727a1a0201010000000001003f142f61ed5ae1ce344b7abd0000000000002200020000000000000a004c0042cd07e2bb17d0c4ef343341faa3a0f766b3b600000000000000000003390b52d6000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    fn unhex(s: &str) -> Vec<u8> {
        let b = s.as_bytes();
        let mut out = Vec::with_capacity(b.len() / 2);
        let mut i = 0;
        while i + 1 < b.len() {
            let hi = (b[i] as char).to_digit(16).unwrap() as u8;
            let lo = (b[i + 1] as char).to_digit(16).unwrap() as u8;
            out.push((hi << 4) | lo);
            i += 2;
        }
        out
    }

    #[test]
    fn nhrex_tnf_sfdu_framing_decodes() {
        let bytes = unhex(NHREX_TNF_HEAD_2);
        let frames = scan_tnf_sfdus(&bytes).unwrap();
        assert_eq!(frames.len(), 2);

        let f = &frames[0];
        assert_eq!(f.offset, 0);
        assert_eq!(f.total_len, 182);
        assert_eq!(&f.data_description_id, b"C123");
        assert_eq!(f.sfdu_length, 162);
        assert_eq!(f.aggr_chdo_type, 1);
        assert_eq!(f.aggr_chdo_length, 78);
        assert_eq!(f.primary_chdo_type, 2);
        assert_eq!(f.primary_chdo_length, 4);
        assert_eq!(f.mjr_data_class, 6);
        assert_eq!(f.mnr_data_class, 14);
        assert_eq!(f.mission_id, 24);
        assert_eq!(f.format_code, 0);
        assert_eq!(f.secondary_chdo_type, 132);
        assert_eq!(f.secondary_chdo_length, 66);
        assert_eq!(f.scft_id, 98);
        assert_eq!(f.rec_seq_num, 0);
        assert_eq!(f.year, 2012);
        assert_eq!(f.doy, 21);
        assert_eq!(f.sec, 48109.0);

        let g = &frames[1];
        assert_eq!(g.offset, 182);
        assert_eq!(g.total_len, 182);
        assert_eq!(&g.data_description_id, b"C123");
        assert_eq!(g.format_code, 0);
        assert_eq!(g.scft_id, 98);
        assert_eq!(g.rec_seq_num, 1);
        assert_eq!(g.year, 2012);
        assert_eq!(g.doy, 21);
        assert_eq!(g.sec, 48110.0);
    }

    #[test]
    fn nhrex_tnf_dt0_uplink_carrier_phase_decodes() {
        let bytes = unhex(NHREX_TNF_HEAD_2);
        let frames = scan_tnf_sfdus(&bytes).unwrap();
        let f = &frames[0];
        let d = tnf_dt0(f, &bytes[f.offset..]).unwrap();

        assert_eq!(d.orig_id, 0x30);
        assert_eq!(d.last_modifier_id, 0x31);
        assert_eq!(d.upl_rec_seq_num, 0);
        assert_eq!(d.rct_day, 20579);
        assert_eq!(d.rct_msec, 79065722);
        assert_eq!(d.ul_dss_id, 26);
        assert_eq!(d.ul_band, 2);
        assert_eq!(d.ul_assembly_num, 1);
        assert_eq!(d.transmit_num, 1);
        assert_eq!(d.transmit_stat, 0);
        assert_eq!(d.transmit_mode, 0);
        assert_eq!(d.cmd_modul_stat, 0);
        assert_eq!(d.rng_modul_stat, 0);
        assert_eq!(d.fts_vld_flag, 1);
        assert_eq!(d.transmit_time_tag_delay.to_bits(), 0x3f142f61ed5ae1ce);
        assert_eq!(d.ul_zheight_corr.to_bits(), 0x344b7abd);
        assert_eq!(d.mod_day, 0);
        assert_eq!(d.mod_msec, 0);
        assert_eq!(d.version_num, 34);
        assert_eq!(d.sub_version_num, 0);
        assert_eq!(d.sub_sub_version_num, 2);
        assert_eq!(d.data_chdo_type, 10);
        assert_eq!(d.data_chdo_length, 76);
        assert_eq!(d.ul_hi_phs_cycles, 0x0042cd06);
        assert_eq!(d.ul_lo_phs_cycles, 0x3881085a);
        assert_eq!(d.ul_frac_phs_cycles, 0x59b3cfd9);
        assert_eq!(d.ramp_freq.to_bits(), 0x41faa3a0f766b3b6);
        assert_eq!(d.ramp_rate, 0.0);
        assert_eq!(d.transmit_switch_stat, 0);
        assert_eq!(d.ramp_type, 0);
        assert_eq!(d.transmit_op_pwr.to_bits(), 0x388c3848);
        assert_eq!(d.sup_data_id, [0u8; 8]);
        assert_eq!(d.sup_data_rev, [0u8; 8]);
        assert_eq!(d.prdx_time_offset, 0.0);
        assert_eq!(d.prdx_freq_offset, 0.0);
        assert_eq!(d.time_tag_corr_flag, 0);
        assert_eq!(d.type_time_corr_flag, 0);
        assert_eq!(d.fabricated_sfdu_flag, 0);
    }

    #[test]
    fn tnf_rows_resolves_dt0_to_tdb_phase_rows() {
        let lsk = crate::archivar::lsk::parse(
            "DELTET/DELTA_T_A = 32.184\n\
             DELTET/DELTA_AT = ( 10, @1972-JAN-1, 37, @2017-JAN-1 )",
        )
        .unwrap();
        let bytes = unhex(NHREX_TNF_HEAD_2);
        let rows = tnf_rows(&bytes, &lsk).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows[0][0] < rows[1][0], "tdb rows follow the SFDU order");
        let phase = 0x0042cd06u32 as f64 * 4_294_967_296.0
            + 0x3881085au32 as f64
            + 0x59b3cfd9u32 as f64 / 4_294_967_296.0;
        assert_eq!(rows[0][1], phase);
        assert_eq!(rows[0][3], 26.0, "ul_dss_id carries the DT0 field");
        assert_eq!(rows[0][4], 98.0, "scft_id carries the SFDU field");
        assert_eq!(rows[0][5], 0.0, "format_code 0 marks DT0");
    }

    fn put_u16(b: &mut [u8], off: usize, v: u16) {
        b[off..off + 2].copy_from_slice(&v.to_be_bytes());
    }

    fn put_u32(b: &mut [u8], off: usize, v: u32) {
        b[off..off + 4].copy_from_slice(&v.to_be_bytes());
    }

    fn put_f32(b: &mut [u8], off: usize, v: f32) {
        b[off..off + 4].copy_from_slice(&v.to_be_bytes());
    }

    fn put_f64(b: &mut [u8], off: usize, v: f64) {
        b[off..off + 8].copy_from_slice(&v.to_be_bytes());
    }

    fn sfdu_label(b: &mut [u8], format_code: u8, year: u16, doy: u16, sec: f64) {
        let sfdu_length = (b.len() - TNF_LABEL_LEN) as u64;
        b[0..4].copy_from_slice(b"NJPL");
        b[8..12].copy_from_slice(b"C125");
        b[12..20].copy_from_slice(&sfdu_length.to_be_bytes());
        b[31] = format_code;
        put_u16(b, 48, year);
        put_u16(b, 50, doy);
        put_f64(b, 52, sec);
    }

    fn test_lsk() -> crate::archivar::lsk::LeapSeconds {
        crate::archivar::lsk::parse(
            "DELTET/DELTA_T_A = 32.184\n\
             DELTET/DELTA_AT = ( 10, @1972-JAN-1, 37, @2017-JAN-1 )",
        )
        .unwrap()
    }

    #[test]
    fn tnf_dt6_doppler_count_decodes_and_rows() {
        let mut bytes = vec![0u8; 220];
        sfdu_label(&mut bytes, 6, 2015, 100, 12345.0);
        bytes[82] = 43;
        bytes[63] = 2;
        bytes[164] = 2;
        put_f32(&mut bytes, 166, 0.1);
        put_f64(&mut bytes, 174, 7.169e9);
        put_f64(&mut bytes, 182, -12.5);
        put_f64(&mut bytes, 190, 1234567.89);
        put_f64(&mut bytes, 198, 0.004);
        bytes[211] = 1;
        let frames = scan_tnf_sfdus(&bytes).unwrap();
        let d = tnf_dt6(&frames[0], &bytes).unwrap();
        assert_eq!(d.dl_dss_id, 43);
        assert_eq!(d.ul_band, 2);
        assert_eq!(d.ref_rcv_type, 2);
        assert_eq!(d.sampl_interval, 0.1);
        assert_eq!(d.ul_freq, 7.169e9);
        assert_eq!(d.dop_cnt_bias_freq, -12.5);
        assert_eq!(d.dop_cnt, 1234567.89);
        assert_eq!(d.dop_pseudo_resid, 0.004);
        assert_eq!(d.dop_vld_flag, 1);
        let rows = tnf_rows(&bytes, &test_lsk()).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][TNF_ROW_FORMAT], 6.0);
        assert_eq!(rows[0][TNF_ROW_OBSERVABLE], 1234567.89);
        assert_eq!(rows[0][TNF_ROW_DSS], 43.0);
        assert_eq!(rows[0][TNF_ROW_BAND], 2.0);
    }

    #[test]
    fn tnf_dt17_total_count_phase_decodes_and_rows() {
        let mut bytes = vec![0u8; 236];
        sfdu_label(&mut bytes, 17, 2015, 101, 12346.0);
        bytes[82] = 63;
        bytes[63] = 1;
        bytes[164] = 2;
        put_u16(&mut bytes, 188, 1);
        put_f32(&mut bytes, 190, 0.1);
        put_u16(&mut bytes, 194, 2015);
        put_u16(&mut bytes, 196, 101);
        put_f64(&mut bytes, 198, 12345.5);
        put_u32(&mut bytes, 206, 0x0042cd06);
        put_u32(&mut bytes, 210, 0x3881085a);
        put_u32(&mut bytes, 214, 0x59b3cfd9);
        put_f32(&mut bytes, 218, 1.25);
        bytes[222] = 1;
        bytes[223] = 1;
        let frames = scan_tnf_sfdus(&bytes).unwrap();
        let d = tnf_dt17(&frames[0], &bytes).unwrap();
        assert_eq!(d.num_obs, 1);
        assert_eq!(d.st_year, 2015);
        assert_eq!(d.st_doy, 101);
        assert_eq!(d.obs_hi, 0x0042cd06);
        assert_eq!(d.obs_lo, 0x3881085a);
        assert_eq!(d.obs_frac, 0x59b3cfd9);
        assert_eq!(d.prefit_resid, 1.25);
        assert_eq!(d.prefit_resid_vld_flag, 1);
        assert_eq!(d.prefit_resid_tol_flag, 1);
        let rows = tnf_rows(&bytes, &test_lsk()).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][TNF_ROW_FORMAT], 17.0);
        let phase = tnf_phase_cycles(0x0042cd06, 0x3881085a, 0x59b3cfd9);
        assert_eq!(rows[0][TNF_ROW_OBSERVABLE], phase);
    }

    #[test]
    fn tnf_dt6_truncated_sfdu_is_a_measured_skip() {
        let mut bytes = vec![0u8; 220];
        sfdu_label(&mut bytes, 6, 2015, 100, 12345.0);
        let frames = scan_tnf_sfdus(&bytes).unwrap();
        assert!(tnf_dt6(&frames[0], &bytes[..219]).is_none());
    }

    #[test]
    fn tnf_dt1_downlink_carrier_phase_decodes_phase_arrays() {
        let mut bytes = vec![0u8; 378];
        sfdu_label(&mut bytes, 1, 2015, 100, 12345.0);
        bytes[66] = 43;
        bytes[67] = 2;
        put_f32(&mut bytes, 150, 0.25);
        put_u32(&mut bytes, 174, 1);
        put_u32(&mut bytes, 178, 2);
        put_u32(&mut bytes, 182, 3);
        put_u32(&mut bytes, 294, 11);
        put_u32(&mut bytes, 298, 22);
        put_u32(&mut bytes, 302, 33);
        put_f64(&mut bytes, 306, 8.4e9);
        put_f64(&mut bytes, 348, 0.05);
        put_f64(&mut bytes, 356, -1.0e-3);
        bytes[326] = 2;
        bytes[327] = 1;
        let frames = scan_tnf_sfdus(&bytes).unwrap();
        let d = tnf_dt1(&frames[0], &bytes).unwrap();
        assert_eq!(d.phases[0], [1, 2, 3]);
        assert_eq!(d.phase_avg, [11, 22, 33]);
        assert_eq!(d.dl_freq, 8.4e9);
        assert_eq!(d.carr_loop_type, 2);
        assert_eq!(d.snt_flag, 1);
        assert_eq!(d.prdx_time_offset, 0.05);
        assert_eq!(d.prdx_freq_offset, -1.0e-3);
        let rows = tnf_rows(&bytes, &test_lsk()).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][TNF_ROW_FORMAT], 1.0);
        assert_eq!(rows[0][TNF_ROW_OBSERVABLE], tnf_phase_cycles(11, 22, 33));
        assert_eq!(rows[0][TNF_ROW_DSS], 43.0);
        assert_eq!(rows[0][TNF_ROW_BAND], 2.0);
    }

    #[test]
    fn tnf_dt12_smoothed_noise_decodes_from_chdo136() {
        let mut bytes = vec![0u8; 184];
        sfdu_label(&mut bytes, 12, 2015, 100, 12345.0);
        bytes[62] = 26;
        bytes[63] = 2;
        put_f32(&mut bytes, 138, 0.011);
        put_f32(&mut bytes, 142, 0.012);
        put_u32(&mut bytes, 162, 60);
        put_f32(&mut bytes, 166, 98.5);
        bytes[170] = 1;
        let frames = scan_tnf_sfdus(&bytes).unwrap();
        let d = tnf_dt12(&frames[0], &bytes).unwrap();
        assert_eq!(d.dl_dss_id, 26);
        assert_eq!(d.dl_band, 2);
        assert_eq!(d.noise_01sec, 0.011);
        assert_eq!(d.noise_1sec, 0.012);
        assert_eq!(d.int_time, 60);
        assert_eq!(d.percent_data_used, 98.5);
        assert_eq!(d.new_01sec, 1);
    }
}
