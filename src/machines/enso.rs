use super::*;

pub const ENSO_GRID: u32 = 21600;

pub const ENSO_RING_MAX: usize = 1024;

pub const ENSO_FETCH_TTL: u64 = 3600;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnsoStation {
    St51000,
    St51001,
    St41001,
    St41002,
    St41043,
    St41010,
    St41049,
    St42001,
    St42002,
    St42036,
    St42055,
    St44009,
    St44013,
    St45001,
    St45161,
    St45178,
    St45186,
    St45207,
    St46001,
    St46002,
    St46005,
    St46012,
    St46022,
    St46025,
    St46026,
    St46029,
    St46035,
    St46047,
    St46053,
    St46054,
    St46059,
    St46069,
    St46071,
    St46075,
    St46086,
    St51004,
    St15006,
}

impl EnsoStation {
    pub const ALL: [EnsoStation; 37] = [
        EnsoStation::St51000,
        EnsoStation::St51001,
        EnsoStation::St41001,
        EnsoStation::St41002,
        EnsoStation::St41043,
        EnsoStation::St41010,
        EnsoStation::St41049,
        EnsoStation::St42001,
        EnsoStation::St42002,
        EnsoStation::St42036,
        EnsoStation::St42055,
        EnsoStation::St44009,
        EnsoStation::St44013,
        EnsoStation::St45001,
        EnsoStation::St45161,
        EnsoStation::St45178,
        EnsoStation::St45186,
        EnsoStation::St45207,
        EnsoStation::St46001,
        EnsoStation::St46002,
        EnsoStation::St46005,
        EnsoStation::St46012,
        EnsoStation::St46022,
        EnsoStation::St46025,
        EnsoStation::St46026,
        EnsoStation::St46029,
        EnsoStation::St46035,
        EnsoStation::St46047,
        EnsoStation::St46053,
        EnsoStation::St46054,
        EnsoStation::St46059,
        EnsoStation::St46069,
        EnsoStation::St46071,
        EnsoStation::St46075,
        EnsoStation::St46086,
        EnsoStation::St51004,
        EnsoStation::St15006,
    ];

    pub fn id(self) -> &'static str {
        match self {
            EnsoStation::St51000 => "51000",
            EnsoStation::St51001 => "51001",
            EnsoStation::St41001 => "41001",
            EnsoStation::St41002 => "41002",
            EnsoStation::St41043 => "41043",
            EnsoStation::St41010 => "41010",
            EnsoStation::St41049 => "41049",
            EnsoStation::St42001 => "42001",
            EnsoStation::St42002 => "42002",
            EnsoStation::St42036 => "42036",
            EnsoStation::St42055 => "42055",
            EnsoStation::St44009 => "44009",
            EnsoStation::St44013 => "44013",
            EnsoStation::St45001 => "45001",
            EnsoStation::St45161 => "45161",
            EnsoStation::St45178 => "45178",
            EnsoStation::St45186 => "45186",
            EnsoStation::St45207 => "45207",
            EnsoStation::St46001 => "46001",
            EnsoStation::St46002 => "46002",
            EnsoStation::St46005 => "46005",
            EnsoStation::St46012 => "46012",
            EnsoStation::St46022 => "46022",
            EnsoStation::St46025 => "46025",
            EnsoStation::St46026 => "46026",
            EnsoStation::St46029 => "46029",
            EnsoStation::St46035 => "46035",
            EnsoStation::St46047 => "46047",
            EnsoStation::St46053 => "46053",
            EnsoStation::St46054 => "46054",
            EnsoStation::St46059 => "46059",
            EnsoStation::St46069 => "46069",
            EnsoStation::St46071 => "46071",
            EnsoStation::St46075 => "46075",
            EnsoStation::St46086 => "46086",
            EnsoStation::St51004 => "51004",
            EnsoStation::St15006 => "15006",
        }
    }

    pub fn idx(self) -> usize {
        match self {
            EnsoStation::St51000 => 0,
            EnsoStation::St51001 => 1,
            EnsoStation::St41001 => 2,
            EnsoStation::St41002 => 3,
            EnsoStation::St41043 => 4,
            EnsoStation::St41010 => 5,
            EnsoStation::St41049 => 6,
            EnsoStation::St42001 => 7,
            EnsoStation::St42002 => 8,
            EnsoStation::St42036 => 9,
            EnsoStation::St42055 => 10,
            EnsoStation::St44009 => 11,
            EnsoStation::St44013 => 12,
            EnsoStation::St45001 => 13,
            EnsoStation::St45161 => 14,
            EnsoStation::St45178 => 15,
            EnsoStation::St45186 => 16,
            EnsoStation::St45207 => 17,
            EnsoStation::St46001 => 18,
            EnsoStation::St46002 => 19,
            EnsoStation::St46005 => 20,
            EnsoStation::St46012 => 21,
            EnsoStation::St46022 => 22,
            EnsoStation::St46025 => 23,
            EnsoStation::St46026 => 24,
            EnsoStation::St46029 => 25,
            EnsoStation::St46035 => 26,
            EnsoStation::St46047 => 27,
            EnsoStation::St46053 => 28,
            EnsoStation::St46054 => 29,
            EnsoStation::St46059 => 30,
            EnsoStation::St46069 => 31,
            EnsoStation::St46071 => 32,
            EnsoStation::St46075 => 33,
            EnsoStation::St46086 => 34,
            EnsoStation::St51004 => 35,
            EnsoStation::St15006 => 36,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnsoSeries {
    Wspd,
    Gst,
    Wvht,
    Dpd,
    Apd,
    Pres,
    Ptdy,
    Atmp,
    Wtmp,
    Dewp,
    Vis,
    Tide,
    WdirSin,
    WdirCos,
    MwdSin,
    MwdCos,
    Rain,
}

impl EnsoSeries {
    pub const ALL: [EnsoSeries; 17] = [
        EnsoSeries::Wspd,
        EnsoSeries::Gst,
        EnsoSeries::Wvht,
        EnsoSeries::Dpd,
        EnsoSeries::Apd,
        EnsoSeries::Pres,
        EnsoSeries::Ptdy,
        EnsoSeries::Atmp,
        EnsoSeries::Wtmp,
        EnsoSeries::Dewp,
        EnsoSeries::Vis,
        EnsoSeries::Tide,
        EnsoSeries::WdirSin,
        EnsoSeries::WdirCos,
        EnsoSeries::MwdSin,
        EnsoSeries::MwdCos,
        EnsoSeries::Rain,
    ];

    pub fn idx(self) -> usize {
        match self {
            EnsoSeries::Wspd => 0,
            EnsoSeries::Gst => 1,
            EnsoSeries::Wvht => 2,
            EnsoSeries::Dpd => 3,
            EnsoSeries::Apd => 4,
            EnsoSeries::Pres => 5,
            EnsoSeries::Ptdy => 6,
            EnsoSeries::Atmp => 7,
            EnsoSeries::Wtmp => 8,
            EnsoSeries::Dewp => 9,
            EnsoSeries::Vis => 10,
            EnsoSeries::Tide => 11,
            EnsoSeries::WdirSin => 12,
            EnsoSeries::WdirCos => 13,
            EnsoSeries::MwdSin => 14,
            EnsoSeries::MwdCos => 15,
            EnsoSeries::Rain => 16,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            EnsoSeries::Wspd => "wspd",
            EnsoSeries::Gst => "gst",
            EnsoSeries::Wvht => "wvht",
            EnsoSeries::Dpd => "dpd",
            EnsoSeries::Apd => "apd",
            EnsoSeries::Pres => "pres",
            EnsoSeries::Ptdy => "ptdy",
            EnsoSeries::Atmp => "atmp",
            EnsoSeries::Wtmp => "wtmp",
            EnsoSeries::Dewp => "dewp",
            EnsoSeries::Vis => "vis",
            EnsoSeries::Tide => "tide",
            EnsoSeries::WdirSin => "wdir_sin",
            EnsoSeries::WdirCos => "wdir_cos",
            EnsoSeries::MwdSin => "mwd_sin",
            EnsoSeries::MwdCos => "mwd_cos",
            EnsoSeries::Rain => "rain",
        }
    }

    pub fn column(self) -> &'static str {
        match self {
            EnsoSeries::Wspd => "WSPD",
            EnsoSeries::Gst => "GST",
            EnsoSeries::Wvht => "WVHT",
            EnsoSeries::Dpd => "DPD",
            EnsoSeries::Apd => "APD",
            EnsoSeries::Pres => "PRES",
            EnsoSeries::Ptdy => "PTDY",
            EnsoSeries::Atmp => "ATMP",
            EnsoSeries::Wtmp => "WTMP",
            EnsoSeries::Dewp => "DEWP",
            EnsoSeries::Vis => "VIS",
            EnsoSeries::Tide => "TIDE",
            EnsoSeries::WdirSin | EnsoSeries::WdirCos => "WDIR",
            EnsoSeries::MwdSin | EnsoSeries::MwdCos => "MWD",
            EnsoSeries::Rain => "RAIN",
        }
    }

    pub fn plausible(self, v: f64) -> bool {
        if !v.is_finite() {
            return false;
        }
        match self {
            EnsoSeries::Wspd
            | EnsoSeries::Gst
            | EnsoSeries::Wvht
            | EnsoSeries::Dpd
            | EnsoSeries::Apd
            | EnsoSeries::Vis
            | EnsoSeries::Rain => v > 0.0 && v < 99.0,
            EnsoSeries::Pres => v > 0.0 && v < 9999.0,
            EnsoSeries::Ptdy | EnsoSeries::Tide => v.abs() < 99.0,
            EnsoSeries::Atmp | EnsoSeries::Wtmp | EnsoSeries::Dewp => v > -100.0 && v < 900.0,
            EnsoSeries::WdirSin | EnsoSeries::WdirCos | EnsoSeries::MwdSin | EnsoSeries::MwdCos => {
                v >= 0.0 && v <= 360.0
            }
        }
    }

    pub fn transform(self, raw: f64) -> Option<f64> {
        if !self.plausible(raw) {
            return None;
        }
        match self {
            EnsoSeries::WdirSin | EnsoSeries::MwdSin => Some(raw.to_radians().sin()),
            EnsoSeries::WdirCos | EnsoSeries::MwdCos => Some(raw.to_radians().cos()),
            _ => Some(raw),
        }
    }
}

#[derive(Clone, Copy)]
pub struct EnsoCell {
    pub station: EnsoStation,
    pub series: EnsoSeries,
    pub bin: u64,
    pub value: f32,
}

pub fn enso_ndbc_parse(
    body: &str,
    lsk: &LeapSeconds,
) -> Option<Vec<(EnsoSeries, Vec<(f64, f64)>)>> {
    let header = body.lines().find(|l| l.starts_with('#'))?;
    let cols: Vec<&str> = header.split_whitespace().collect();
    let mut present: Vec<(EnsoSeries, usize)> = Vec::new();
    for s in EnsoSeries::ALL {
        if let Some(i) = cols.iter().position(|&c| c == s.column()) {
            if !present.iter().any(|&(cs, _)| cs == s) {
                present.push((s, i));
            }
        }
    }
    if present.is_empty() {
        return None;
    }
    let mut series: Vec<(EnsoSeries, Vec<(f64, f64)>)> =
        present.iter().map(|&(s, _)| (s, Vec::new())).collect();
    for line in body
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let p: Vec<&str> = line.split_whitespace().collect();
        let needed = present.iter().map(|&(_, i)| i).max().unwrap_or(0);
        if p.len() <= needed {
            continue;
        }
        let Ok(year) = p[0].parse::<i64>() else {
            continue;
        };
        let Ok(month) = p[1].parse::<i64>() else {
            continue;
        };
        let Ok(day) = p[2].parse::<i64>() else {
            continue;
        };
        let Ok(hour) = p[3].parse::<i64>() else {
            continue;
        };
        let Ok(minute) = p[4].parse::<i64>() else {
            continue;
        };
        let days = crate::lsk::days_from_civil(year, month, day)?;
        let unix = days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0;
        let Some(t) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        for (s, vec) in series.iter_mut() {
            let i = present.iter().find(|&&(cs, _)| cs == *s).map(|&(_, i)| i);
            let Some(i) = i else { continue };
            if let Ok(v) = p[i].parse::<f64>() {
                if let Some(tv) = s.transform(v) {
                    vec.push((t, tv));
                }
            }
        }
    }
    series.retain(|(_, vec)| !vec.is_empty());
    if series.is_empty() {
        None
    } else {
        Some(series)
    }
}

pub fn enso_send_bins(
    series: &[(f64, f64)],
    station: EnsoStation,
    kind: EnsoSeries,
    last_sent: &mut HashMap<(EnsoStation, EnsoSeries), u64>,
    tx: &mpsc::Sender<EnsoCell>,
) {
    let dt = ENSO_GRID as f64;
    let mut sorted: Vec<&(f64, f64)> = series
        .iter()
        .filter(|&&(t, _)| t.is_finite() && t > 0.0)
        .collect();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    let gate = last_sent.get(&(station, kind)).copied().unwrap_or(0);
    let mut cells: Vec<EnsoCell> = Vec::new();
    let mut cur_bin: i64 = i64::MIN;
    let mut sum: f64 = 0.0;
    let mut cnt: u32 = 0;
    for &&(t, v) in &sorted {
        let bin = (t / dt).floor() as i64;
        if bin != cur_bin {
            if cur_bin != i64::MIN && cnt > 0 {
                let b = cur_bin as u64;
                if b > gate {
                    cells.push(EnsoCell {
                        station,
                        series: kind,
                        bin: b,
                        value: (sum / cnt as f64) as f32,
                    });
                }
            }
            cur_bin = bin;
            sum = 0.0;
            cnt = 0;
        }
        sum += v;
        cnt += 1;
    }
    if cur_bin != i64::MIN && cnt > 0 {
        let b = cur_bin as u64;
        if b > gate {
            cells.push(EnsoCell {
                station,
                series: kind,
                bin: b,
                value: (sum / cnt as f64) as f32,
            });
        }
    }
    if gate == 0 && cells.len() > ENSO_RING_MAX {
        let drop = cells.len() - ENSO_RING_MAX;
        cells.drain(..drop);
    }
    for cell in &cells {
        let _ = tx.send(*cell);
    }
    if let Some(last) = cells.last() {
        last_sent.insert((station, kind), last.bin);
    }
}

pub fn enso_harvest(tx: mpsc::Sender<EnsoCell>, time: Arc<Mutex<Option<LeapSeconds>>>) {
    let mut last_sent: HashMap<(EnsoStation, EnsoSeries), u64> = HashMap::new();
    let mut said: HashSet<String> = HashSet::new();
    let mut backfilled: HashSet<EnsoStation> = HashSet::new();
    loop {
        let lsk = {
            let lock = match time.lock() {
                Ok(l) => l,
                Err(_) => break,
            };
            let Some(lsk) = lock.as_ref() else {
                drop(lock);
                thread::sleep(std::time::Duration::from_secs(ENSO_FETCH_TTL));
                continue;
            };
            lsk.clone()
        };
        for station in EnsoStation::ALL {
            if backfilled.insert(station) {
                enso_backfill(station, &lsk, &mut last_sent, &tx);
            }
            let url = format!(
                "https://www.ndbc.noaa.gov/data/realtime2/{}.txt",
                station.id()
            );
            if let Some(body) = fetch_raw(&url, None, &[], ENSO_FETCH_TTL) {
                match enso_ndbc_parse(&body, &lsk) {
                    Some(series) => {
                        for (kind, values) in &series {
                            enso_send_bins(values, station, *kind, &mut last_sent, &tx);
                        }
                    }
                    None => {
                        if said.insert(format!("{} column absent", station.id())) {
                            eprintln!("enso pair {} column absent", station.id());
                        }
                    }
                }
            }
        }
        thread::sleep(std::time::Duration::from_secs(ENSO_FETCH_TTL));
    }
}

pub fn enso_backfill(
    station: EnsoStation,
    lsk: &LeapSeconds,
    last_sent: &mut HashMap<(EnsoStation, EnsoSeries), u64>,
    tx: &mpsc::Sender<EnsoCell>,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let year = 1970 + (now / 31556952) as i64 - 1;
    if year < 1970 {
        return;
    }
    let url = format!(
        "https://www.ndbc.noaa.gov/view_text_file.php?filename={}h{}.txt.gz&dir=data/historical/stdmet/",
        station.id(),
        year
    );
    let cache = format!("/tmp/omegaflow_enso_cache/{}_{}.txt", station.id(), year);
    let body = match std::fs::read_to_string(&cache) {
        Ok(cached) => Some(cached),
        Err(_) => enso_cache_fetch(&url, &cache),
    };
    if let Some(body) = body {
        if let Some(series) = enso_ndbc_parse(&body, lsk) {
            for (kind, values) in &series {
                enso_send_bins(values, station, *kind, last_sent, tx);
            }
        }
    }
}

pub fn enso_cache_fetch(url: &str, cache: &str) -> Option<String> {
    let bytes = fetch_raw_bytes(url, 86400)?;
    let text = if bytes.starts_with(&[0x1f, 0x8b]) {
        crate::inflate::gunzip(&bytes).map(|b| String::from_utf8_lossy(&b).into_owned())?
    } else {
        String::from_utf8_lossy(&bytes).into_owned()
    };
    let _ = std::fs::create_dir_all("/tmp/omegaflow_enso_cache");
    let _ = std::fs::write(cache, &text);
    thread::sleep(std::time::Duration::from_secs(32));
    Some(text)
}

pub const ENSO_STATION_COUNT: usize = EnsoStation::ALL.len();

pub const ENSO_SERIES_COUNT: usize = EnsoSeries::ALL.len();

pub const ENSO_PAIR_COUNT: usize = ENSO_SERIES_COUNT * (ENSO_SERIES_COUNT - 1) / 2;

pub const ENSO_PROBE_MAX: usize = 512;

pub const ENSO_N_GATE: usize = 30;

pub const ENSO_SCALES: [f32; 3] = [1.0, 0.5, 2.0];

pub const ENSO_SCALE_NAMES: [&str; 3] = ["h", "h/2", "2h"];

pub const ENSO_SHIFT_STEP_BINS: i64 = 4;

pub const ENSO_SHIFT_MAX_DAYS: i64 = 30;

pub const ENSO_SHIFT_COUNT: usize = (ENSO_SHIFT_MAX_DAYS * 2 + 1) as usize;

pub const ENSO_CELLS_PER_SCALE: usize = ENSO_SHIFT_COUNT * 2;

pub const ENSO_CELLS_PER_ROUND: usize = ENSO_CELLS_PER_SCALE * ENSO_SCALES.len();

pub fn enso_pair(pair_idx: usize) -> (EnsoSeries, EnsoSeries) {
    let mut k = 0usize;
    for i in 0..ENSO_SERIES_COUNT {
        for j in (i + 1)..ENSO_SERIES_COUNT {
            if k == pair_idx {
                return (EnsoSeries::ALL[i], EnsoSeries::ALL[j]);
            }
            k += 1;
        }
    }
    unreachable!()
}

pub fn enso_cell_desc(cell_idx: usize) -> (u8, u8, i64) {
    let scale_idx = (cell_idx / ENSO_CELLS_PER_SCALE) as u8;
    let rest = cell_idx % ENSO_CELLS_PER_SCALE;
    let dir = (rest / ENSO_SHIFT_COUNT) as u8;
    let shift_days = rest as i64 % ENSO_SHIFT_COUNT as i64 - ENSO_SHIFT_MAX_DAYS;
    (scale_idx, dir, shift_days)
}

pub fn enso_shift_pair(
    driver: &[(u64, f32)],
    target: &[(u64, f32)],
    shift_bins: i64,
) -> (Vec<f32>, Vec<f32>) {
    let mut ys = Vec::new();
    let mut xs = Vec::new();
    for &(tb, tv) in target {
        let Some(db) = tb.checked_sub_signed(shift_bins) else {
            continue;
        };
        let Ok(i) = driver.binary_search_by_key(&db, |&(b, _)| b) else {
            continue;
        };
        ys.push(driver[i].1);
        xs.push(tv);
    }
    if ys.len() > ENSO_RING_MAX {
        let drop = ys.len() - ENSO_RING_MAX;
        ys.drain(..drop);
        xs.drain(..drop);
    }
    (ys, xs)
}

pub struct EnsoCellVerdict {
    pub dir: u8,
    pub shift: i64,
    pub n: usize,
    pub te: f64,
    pub thr: f64,
}

pub struct EnsoAccum {
    pub m: usize,
    pub fam: f64,
    pub fam_any: bool,
    pub surr_over: usize,
    pub surr_total: usize,
    pub cells1: Vec<EnsoCellVerdict>,
    pub small: Vec<EnsoCellVerdict>,
}

impl EnsoAccum {
    pub fn fresh() -> EnsoAccum {
        EnsoAccum {
            m: 0,
            fam: 0.0,
            fam_any: false,
            surr_over: 0,
            surr_total: 0,
            cells1: Vec::new(),
            small: Vec::new(),
        }
    }
}

pub struct EnsoMatrixAccum {
    pub pairs: usize,
    pub arrows: usize,
    pub family: usize,
    pub h_bound: usize,
    pub silent: usize,
    pub absent: usize,
    pub expected: f64,
}

impl EnsoMatrixAccum {
    pub fn fresh() -> EnsoMatrixAccum {
        EnsoMatrixAccum {
            pairs: 0,
            arrows: 0,
            family: 0,
            h_bound: 0,
            silent: 0,
            absent: 0,
            expected: 0.0,
        }
    }
}

pub struct EnsoMachine {
    pub rx: mpsc::Receiver<EnsoCell>,
    pub rings: [[Vec<(u64, f32)>; ENSO_SERIES_COUNT]; ENSO_STATION_COUNT],
    pub rotor: usize,
    pub due: [bool; ENSO_STATION_COUNT],
    pub max_bin: [u64; ENSO_STATION_COUNT],
    pub last_round_bin: [u64; ENSO_STATION_COUNT],
    pub pair: [usize; ENSO_STATION_COUNT],
    pub cell: [usize; ENSO_STATION_COUNT],
    pub accum: [EnsoAccum; ENSO_STATION_COUNT],
    pub matrix: [EnsoMatrixAccum; ENSO_STATION_COUNT],
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub te_pipe: Option<wgpu::ComputePipeline>,
    pub te_bind: Option<wgpu::BindGroup>,
    pub te_series_buf: Option<wgpu::Buffer>,
    pub te_param_buf: Option<wgpu::Buffer>,
    pub te_out_buf: Option<wgpu::Buffer>,
    pub te_read_buf: Option<wgpu::Buffer>,
    pub te_map: Option<Arc<AtomicBool>>,
    pub pending: Option<(usize, usize, usize)>,
    pub pending_n: usize,
    pub named: String,
    pub rng: u64,
}

impl EnsoMachine {
    pub fn new(rx: mpsc::Receiver<EnsoCell>) -> Self {
        EnsoMachine {
            rx,
            rings: std::array::from_fn(|_| std::array::from_fn(|_| Vec::new())),
            rotor: 0,
            due: [false; ENSO_STATION_COUNT],
            max_bin: [0; ENSO_STATION_COUNT],
            last_round_bin: [0; ENSO_STATION_COUNT],
            pair: [0; ENSO_STATION_COUNT],
            cell: [0; ENSO_STATION_COUNT],
            accum: std::array::from_fn(|_| EnsoAccum::fresh()),
            matrix: std::array::from_fn(|_| EnsoMatrixAccum::fresh()),
            device: None,
            queue: None,
            te_pipe: None,
            te_bind: None,
            te_series_buf: None,
            te_param_buf: None,
            te_out_buf: None,
            te_read_buf: None,
            te_map: None,
            pending: None,
            pending_n: 0,
            named: String::new(),
            rng: 0,
        }
    }

    pub fn bind_gpu(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        te_pipe: wgpu::ComputePipeline,
        te_layout: &wgpu::BindGroupLayout,
    ) {
        let series_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: TE_SERIES_BYTES,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let param_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let out_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: te_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: series_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: param_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: out_buf.as_entire_binding(),
                },
            ],
        });
        self.device = Some(device.clone());
        self.queue = Some(queue.clone());
        self.te_pipe = Some(te_pipe);
        self.te_bind = Some(bind);
        self.te_series_buf = Some(series_buf);
        self.te_param_buf = Some(param_buf);
        self.te_out_buf = Some(out_buf);
        self.te_read_buf = Some(read_buf);
    }

    pub fn say(&mut self, line: String) {
        if self.named != line {
            eprintln!("{}", line);
            self.named = line;
        }
    }

    pub fn cell_label(station: EnsoStation, pair_idx: usize, cell_idx: usize) -> String {
        let (scale_idx, dir, shift_days) = enso_cell_desc(cell_idx);
        let (a, b) = enso_pair(pair_idx);
        let (driver, target) = if dir == 0 { (a, b) } else { (b, a) };
        format!(
            "enso te {} {}→{} {} {}d",
            station.id(),
            driver.name(),
            target.name(),
            ENSO_SCALE_NAMES[scale_idx as usize],
            shift_days
        )
    }

    pub fn fold(&mut self, station_idx: usize, pair_idx: usize, cell_idx: usize, n: usize) {
        let Some(read_buf) = self.te_read_buf.as_ref() else {
            return;
        };
        let verdict = te_read_verdict(read_buf);
        let station = EnsoStation::ALL[station_idx];
        let label = Self::cell_label(station, pair_idx, cell_idx);
        let (scale_idx, dir, shift_days) = enso_cell_desc(cell_idx);
        let Some(v) = crate::te::topological_verdict_from_gpu(&verdict) else {
            self.say(format!(
                "{} n {} state {}",
                label,
                n,
                te_absence_word(&verdict)
            ));
            return;
        };
        let excess = v.te - v.threshold;
        let acc = &mut self.accum[station_idx];
        if scale_idx == 0 {
            acc.m += 1;
            for s in 2..12 {
                if verdict[s * 6 + 4] == 1.0 {
                    let st = verdict[s * 6 + 1] as f64;
                    if !acc.fam_any || st > acc.fam {
                        acc.fam = st;
                        acc.fam_any = true;
                    }
                    if st > v.threshold {
                        acc.surr_over += 1;
                    }
                    acc.surr_total += 1;
                }
            }
            acc.cells1.push(EnsoCellVerdict {
                dir,
                shift: shift_days,
                n,
                te: v.te,
                thr: v.threshold,
            });
        } else {
            acc.small.push(EnsoCellVerdict {
                dir,
                shift: shift_days,
                n,
                te: v.te,
                thr: v.threshold,
            });
        }
        if excess > 0.0 {
            self.say(format!(
                "{} n {} te {:.3} thr {:.3} state arrow",
                label, n, v.te, v.threshold
            ));
        }
    }

    pub fn collect(&mut self) {
        let Some(prev) = self.te_map.take() else {
            return;
        };
        if !prev.load(Ordering::SeqCst) {
            self.te_map = Some(prev);
            return;
        }
        if let Some((si, pair, cell)) = self.pending.take() {
            let n = self.pending_n;
            self.fold(si, pair, cell, n);
        }
    }

    pub fn probe(&mut self, station_idx: usize, pair_idx: usize, cell_idx: usize) {
        let station = EnsoStation::ALL[station_idx];
        let label = Self::cell_label(station, pair_idx, cell_idx);
        let (scale_idx, dir, shift_days) = enso_cell_desc(cell_idx);
        let shift_bins = shift_days * ENSO_SHIFT_STEP_BINS;
        let h_scale = ENSO_SCALES[scale_idx as usize];
        let (a, b) = enso_pair(pair_idx);
        let (driver, target) = if dir == 0 { (a, b) } else { (b, a) };
        let driver_ring = &self.rings[station_idx][driver.idx()];
        let target_ring = &self.rings[station_idx][target.idx()];
        let (mut ys, mut xs) = enso_shift_pair(driver_ring, target_ring, shift_bins);
        if xs.len() > ENSO_PROBE_MAX {
            let drop = xs.len() - ENSO_PROBE_MAX;
            ys.drain(..drop);
            xs.drain(..drop);
        }
        if xs.len() < ENSO_N_GATE {
            self.say(format!("{} n {} state no statement", label, xs.len()));
            return;
        }
        let Some(device) = self.device.clone() else {
            return;
        };
        let Some(queue) = self.queue.clone() else {
            return;
        };
        let Some(pipe) = self.te_pipe.as_ref() else {
            return;
        };
        let Some(bind) = self.te_bind.as_ref() else {
            return;
        };
        let Some(series_buf) = self.te_series_buf.as_ref() else {
            return;
        };
        let Some(param_buf) = self.te_param_buf.as_ref() else {
            return;
        };
        let Some(out_buf) = self.te_out_buf.as_ref() else {
            return;
        };
        let Some(read_buf) = self.te_read_buf.as_ref() else {
            return;
        };
        let m = xs.len();
        let mut data = vec![0f32; 12 * TE_SERIES_STRIDE];
        data[0..m].copy_from_slice(&xs);
        data[TE_SERIES_STRIDE..TE_SERIES_STRIDE + m].copy_from_slice(&ys);
        let mut rng = self.rng.wrapping_add(0x9e3779b97f4a7c15);
        for s in 0..10 {
            let surr = crate::te::phase_randomized_surrogate(&ys, &mut rng);
            let off = (2 + s) * TE_SERIES_STRIDE;
            data[off..off + m].copy_from_slice(&surr);
        }
        queue.write_buffer(series_buf, 0, &le_bytes_f32(&data));
        let max_lag = (m as f64 / Φ) as u32;
        let param = [m as u32, max_lag, h_scale.to_bits(), 0];
        let mut pb = [0u8; 16];
        for (i, x) in param.iter().enumerate() {
            pb[i * 4..i * 4 + 4].copy_from_slice(&x.to_le_bytes());
        }
        queue.write_buffer(param_buf, 0, &pb);
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(pipe);
            pass.set_bind_group(0, bind, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        enc.copy_buffer_to_buffer(out_buf, 0, read_buf, 0, 288);
        queue.submit(std::iter::once(enc.finish()));
        let mapped = Arc::new(AtomicBool::new(false));
        let m2 = mapped.clone();
        let slice = read_buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, move |r| {
            m2.store(r.is_ok(), Ordering::SeqCst);
        });
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(250);
        while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
            device.poll(wgpu::Maintain::Poll);
        }
        if !mapped.load(Ordering::SeqCst) {
            self.te_map = Some(mapped);
            self.pending = Some((station_idx, pair_idx, cell_idx));
            self.pending_n = m;
            self.say(format!("{} state readback pending", label));
            return;
        }
        self.fold(station_idx, pair_idx, cell_idx, m);
    }

    pub fn sheet(&mut self, station: EnsoStation, pair_idx: usize) {
        let si = station.idx();
        let (a, b) = enso_pair(pair_idx);
        let (state, sheet_parts, p_hat, m_count) = {
            let acc = &self.accum[si];
            let mut best: Option<(u8, i64, usize, f64, f64, f64)> = None;
            for c in &acc.cells1 {
                let e = c.te - c.thr;
                if best.map_or(true, |(_, _, _, be, _, _)| e > be) {
                    best = Some((c.dir, c.shift, c.n, e, c.te, c.thr));
                }
            }
            let p_hat = if acc.surr_total > 0 {
                acc.surr_over as f64 / acc.surr_total as f64
            } else {
                0.0
            };
            if let Some((dir, d_star, n_star, excess, te, thr)) = best {
                let robust = 1 + acc
                    .small
                    .iter()
                    .filter(|c| c.dir == dir && c.shift == d_star && c.te > c.thr)
                    .count();
                let (driver, target) = if dir == 0 { (a, b) } else { (b, a) };
                let state = if excess <= 0.0 {
                    "silent"
                } else if acc.fam_any && te <= acc.fam {
                    "family bound"
                } else if robust < 3 {
                    "h bound"
                } else {
                    "arrow"
                };
                let parts = format!(
                    "enso sheet {} {}→{} lag {}d n {} te {:.3} thr {:.3} fam {:.3} p {:.3} M {} h {}/3 state {}",
                    station.id(),
                    driver.name(),
                    target.name(),
                    d_star,
                    n_star,
                    te,
                    thr,
                    acc.fam,
                    p_hat,
                    acc.m,
                    robust,
                    state
                );
                (state, parts, p_hat, acc.m)
            } else {
                let parts = format!(
                    "enso sheet {} {}→{} state no statement",
                    station.id(),
                    a.name(),
                    b.name()
                );
                ("no statement", parts, p_hat, acc.m)
            }
        };
        self.say(sheet_parts);
        let m = &mut self.matrix[si];
        m.pairs += 1;
        m.expected += p_hat * m_count as f64;
        match state {
            "arrow" => m.arrows += 1,
            "family bound" => m.family += 1,
            "h bound" => m.h_bound += 1,
            "silent" => m.silent += 1,
            _ => m.absent += 1,
        }
    }

    pub fn matrix(&mut self, station: EnsoStation) {
        let m = &self.matrix[station.idx()];
        self.say(format!(
            "enso matrix {} pairs {} arrows {} family {} hbound {} silent {} absent {} expected {:.2} state measured",
            station.id(),
            m.pairs,
            m.arrows,
            m.family,
            m.h_bound,
            m.silent,
            m.absent,
            m.expected
        ));
    }

    pub fn tick(&mut self, ring_gen: u64) {
        self.rng = ring_gen;
        let mut grew = [false; ENSO_STATION_COUNT];
        let mut first = [false; ENSO_STATION_COUNT];
        while let Ok(cell) = self.rx.try_recv() {
            let si = cell.station.idx();
            let ring = &mut self.rings[si][cell.series.idx()];
            if ring.is_empty() {
                first[si] = true;
            }
            match ring.binary_search_by_key(&cell.bin, |&(b, _)| b) {
                Ok(i) => ring[i] = (cell.bin, cell.value),
                Err(i) => ring.insert(i, (cell.bin, cell.value)),
            }
            while ring.len() > ENSO_RING_MAX {
                ring.remove(0);
            }
            if cell.bin > self.max_bin[si] {
                self.max_bin[si] = cell.bin;
                grew[si] = true;
            }
        }
        for si in 0..ENSO_STATION_COUNT {
            if first[si] && !self.due[si] {
                let counts: Vec<String> = EnsoSeries::ALL
                    .iter()
                    .map(|s| format!("{} {}", s.name(), self.rings[si][s.idx()].len()))
                    .collect();
                self.say(format!(
                    "enso ring {} {}",
                    EnsoStation::ALL[si].id(),
                    counts.join(" ")
                ));
            }
            if grew[si] && self.max_bin[si] > self.last_round_bin[si] {
                self.due[si] = true;
            }
        }
        if self.te_map.is_some() {
            self.collect();
        }
        if self.te_map.is_none() {
            for offset in 0..ENSO_STATION_COUNT {
                let si = (self.rotor + offset) % ENSO_STATION_COUNT;
                if !self.due[si] {
                    continue;
                }
                if self.cell[si] < ENSO_CELLS_PER_ROUND {
                    let cell = self.cell[si];
                    self.cell[si] += 1;
                    self.probe(si, self.pair[si], cell);
                } else {
                    self.cell[si] = 0;
                    let pair = self.pair[si];
                    self.pair[si] += 1;
                    self.sheet(EnsoStation::ALL[si], pair);
                    self.accum[si] = EnsoAccum::fresh();
                    if self.pair[si] >= ENSO_PAIR_COUNT {
                        self.pair[si] = 0;
                        self.last_round_bin[si] = self.max_bin[si];
                        self.due[si] = false;
                        self.rotor = (si + 1) % ENSO_STATION_COUNT;
                        self.matrix(EnsoStation::ALL[si]);
                        self.matrix[si] = EnsoMatrixAccum::fresh();
                    }
                }
                break;
            }
        }
    }
}
