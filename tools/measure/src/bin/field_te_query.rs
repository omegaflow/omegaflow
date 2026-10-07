use std::env;
use std::f64::consts::TAU;
use std::process::exit;

use omegaflow::archivar::witness::{WitnessKind, magic_identity, series_gate};
use omegaflow::archivar::{
    Extract, FieldConfig, SourceConfig, embedded_lsk, extract_series, fetch_raw_bytes_headers,
    geo_series_component_name, geo_series_parse_bin, live_markers, load_sources,
    series_component_name, series_rows,
};
use omegaflow::lsk::days_from_civil;
use omegaflow::mathematikerin::newell::newell_dphi_dt;
use omegaflow::mathematikerin::wy_max_t::{
    Member, ResampleMode, null_matrix, null_means_per_statistic, observed_family, phase_data,
    quantile, sigma_per_statistic, studentized_maxima,
};
use omegaflow::te::{
    BiasArm, LaggedCond, TE_NEFF_THRESHOLD, TeEstimator, TeNull, TeSurrogateParams,
    benjamini_hochberg_pass, binned_n_eff, conditional_embedded_te_phase,
    conditional_te_surrogates_n, kde_n_eff, surrogate_max_phase_n, surrogate_rank_p_value,
    surrogate_stats_phase_n, transfer_entropy_bias_adjusted, transfer_entropy_conditional_binned_n,
};

const MONTH_S: f64 = 2_592_000.0;
const CAL_MONTHS: usize = 12;
const CLIMATOLOGY_FLOOR: usize = 10;
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const TE_FLOOR: usize = 8;
const MAXT_ALPHA: f64 = 0.05;
const MAXT_N_CAP: usize = 2000;

const REC_FAM: f64 = 2.9610e-1;
const REC_D2T_WORD: &str = "silent";
const REC_D2T_LAG: usize = 8;
const REC_D2T_TE: f64 = 2.0700e-1;
const REC_T2D_WORD: &str = "silent";
const REC_T2D_LAG: usize = 9;
const REC_T2D_TE: f64 = 2.2693e-1;
const REC_CTE: f64 = 8.3587e-3;
const REC_CTE_THR: f64 = 2.6118e-2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Built,
    Pending,
    Probe,
}

impl State {
    fn parse(token: &str) -> Option<State> {
        match token {
            "built" => Some(State::Built),
            "pending" => Some(State::Pending),
            "probe" => Some(State::Probe),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            State::Built => "built",
            State::Pending => "pending",
            State::Probe => "probe",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Seasonal {
    None,
    Climatology,
}

impl Seasonal {
    fn name(self) -> &'static str {
        match self {
            Seasonal::None => "none",
            Seasonal::Climatology => "climatology+standardize",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Register {
    Sources,
    Witnesses,
}

#[derive(Clone)]
struct WitnessRecord {
    kind: Option<WitnessKind>,
    kind_token: String,
    key: String,
    url: String,
    records: Vec<String>,
    force: Option<String>,
}

#[derive(Clone)]
struct Arm {
    name: String,
    state: State,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MatrixShape {
    Rect,
    Full,
    Upper,
}

impl MatrixShape {
    fn parse(token: &str) -> Option<MatrixShape> {
        match token {
            "rect" => Some(MatrixShape::Rect),
            "full" => Some(MatrixShape::Full),
            "upper" => Some(MatrixShape::Upper),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            MatrixShape::Rect => "rect",
            MatrixShape::Full => "full",
            MatrixShape::Upper => "upper",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MatrixCond {
    Rest,
    Uncond,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FdrMethod {
    Bh,
    By,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FdrScope {
    Matrix,
    Row,
    Col,
}

impl FdrScope {
    fn name(self) -> &'static str {
        match self {
            FdrScope::Matrix => "matrix",
            FdrScope::Row => "row",
            FdrScope::Col => "col",
        }
    }
}

#[derive(Clone)]
struct MatrixSpec {
    label: String,
    shape: MatrixShape,
    drivers: Vec<String>,
    targets: Vec<String>,
    channels: Vec<String>,
    cond: MatrixCond,
    fdr: (FdrMethod, f64, FdrScope),
    expect_cells: Option<usize>,
}

impl MatrixSpec {
    fn pool(&self) -> Vec<String> {
        if self.shape != MatrixShape::Rect {
            return self.channels.clone();
        }
        let mut out = self.drivers.clone();
        for t in &self.targets {
            if !out.iter().any(|n| n == t) {
                out.push(t.clone());
            }
        }
        out
    }

    fn cells(&self) -> Vec<(String, String)> {
        let mut out = Vec::new();
        match self.shape {
            MatrixShape::Rect => {
                for d in &self.drivers {
                    for t in &self.targets {
                        if d == t {
                            continue;
                        }
                        out.push((d.clone(), t.clone()));
                    }
                }
            }
            MatrixShape::Full => {
                for i in 0..self.channels.len() {
                    for j in 0..self.channels.len() {
                        if i == j {
                            continue;
                        }
                        out.push((self.channels[i].clone(), self.channels[j].clone()));
                    }
                }
            }
            MatrixShape::Upper => {
                for i in 0..self.channels.len() {
                    for j in (i + 1)..self.channels.len() {
                        out.push((self.channels[i].clone(), self.channels[j].clone()));
                    }
                }
            }
        }
        out
    }

    fn dropped_cells(&self) -> usize {
        if self.shape != MatrixShape::Rect {
            return 0;
        }
        self.drivers
            .iter()
            .filter(|d| self.targets.iter().any(|t| t == *d))
            .count()
    }

    fn declared_cells(&self) -> usize {
        self.cells().len()
    }

    fn cond_names(&self, driver: &str, target: &str) -> Vec<String> {
        match self.cond {
            MatrixCond::Uncond => Vec::new(),
            MatrixCond::Rest => self
                .pool()
                .into_iter()
                .filter(|n| n != driver && n != target)
                .collect(),
        }
    }
}

struct Descriptor {
    pair: Option<String>,
    driver: Option<Arm>,
    target: Option<Arm>,
    conds: Vec<Arm>,
    events: Vec<(String, String)>,
    gates: Vec<(String, String)>,
    seasonal: Seasonal,
    lags: Vec<usize>,
    surrogate: usize,
    register: Register,
    bin: Option<f64>,
    event_conditional: bool,
    count_quantiles: Option<usize>,
    matrix: Option<MatrixSpec>,
    derived: Vec<(String, Vec<String>)>,
}

fn default_lags() -> Vec<usize> {
    vec![0, 1, 3, 6, 12]
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

struct QueryAnchor {
    lat: Option<f64>,
    lon: Option<f64>,
    station: Option<String>,
}

impl QueryAnchor {
    fn empty() -> QueryAnchor {
        QueryAnchor {
            lat: None,
            lon: None,
            station: None,
        }
    }
}

fn parse_coord(args: &[String], flag: &str, min: f64, max: f64) -> Result<Option<f64>, String> {
    match arg_after(args, flag) {
        Some(t) => {
            let v: f64 = t
                .parse()
                .map_err(|_| format!("{flag} '{t}' carries no number"))?;
            if !v.is_finite() {
                return Err(format!("{flag} '{t}' is no finite coordinate"));
            }
            if !(min..=max).contains(&v) {
                return Err(format!("{flag} '{t}' lies outside [{min}, {max}]"));
            }
            Ok(Some(v))
        }
        None => Ok(None),
    }
}

fn one_coordinate(
    direct: Option<f64>,
    station: Option<f64>,
    direct_flag: &str,
    station_flag: &str,
) -> Result<Option<f64>, String> {
    match (direct, station) {
        (Some(a), Some(b)) if a != b => Err(format!(
            "{direct_flag} {a} and {station_flag} {b} name two coordinates — one anchor per query, never a silent preference"
        )),
        (Some(a), _) => Ok(Some(a)),
        (None, b) => Ok(b),
    }
}

fn anchor_from_args(args: &[String]) -> Result<QueryAnchor, String> {
    let lat = parse_coord(args, "--lat", -90.0, 90.0)?;
    let lon = parse_coord(args, "--lon", -180.0, 180.0)?;
    let station_lat = parse_coord(args, "--station-lat", -90.0, 90.0)?;
    let station_lon = parse_coord(args, "--station-lon", -180.0, 180.0)?;
    Ok(QueryAnchor {
        lat: one_coordinate(lat, station_lat, "--lat", "--station-lat")?,
        lon: one_coordinate(lon, station_lon, "--lon", "--station-lon")?,
        station: arg_after(args, "--station").map(|s| s.to_string()),
    })
}

fn resolve_query_slots(url: &str, anchor: &QueryAnchor) -> Result<String, String> {
    let mut resolved = url.to_string();
    for (slot, value) in [
        ("{lat}", anchor.lat.map(|v| format!("{v}"))),
        ("{lon}", anchor.lon.map(|v| format!("{v}"))),
        ("{station}", anchor.station.clone()),
    ] {
        if resolved.contains(slot) {
            match value {
                Some(v) => resolved = resolved.replace(slot, &v),
                None => {
                    return Err(format!(
                        "url carries the coordinate/station slot '{slot}' — no matching --lat/--lon/--station(/-lat/-lon) anchor was given; the slot stays unresolved, never a default"
                    ));
                }
            }
        }
    }
    Ok(resolved)
}

fn parse_lags(token: &str) -> Result<Vec<usize>, String> {
    let mut out = Vec::new();
    for part in token.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let lag: usize = part
            .parse()
            .map_err(|_| format!("lags token '{part}' carries no index"))?;
        out.push(lag);
    }
    if out.is_empty() {
        return Err("lags carries no index".into());
    }
    Ok(out)
}

fn parse_name_list(token: &str) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::new();
    for part in token.split(',') {
        let part = part.trim();
        if part.is_empty() {
            return Err("arm list carries an empty name".into());
        }
        if out.iter().any(|n| n == part) {
            return Err(format!(
                "arm name '{part}' appears twice — a duplicate arm is no family"
            ));
        }
        out.push(part.to_string());
    }
    if out.is_empty() {
        return Err("arm list carries no name".into());
    }
    Ok(out)
}

fn build_matrix_spec(
    head: Option<(String, MatrixShape)>,
    drivers: Option<Vec<String>>,
    targets: Option<Vec<String>>,
    channels: Option<Vec<String>>,
    cond: Option<MatrixCond>,
    fdr: Option<(FdrMethod, f64, FdrScope)>,
    expect: Option<usize>,
    conds: &[Arm],
    derived: &[(String, Vec<String>)],
) -> Result<Option<MatrixSpec>, String> {
    let (label, shape) = match head {
        Some(h) => h,
        None => {
            if let Some(c) = cond {
                let word = match c {
                    MatrixCond::Rest => "rest",
                    MatrixCond::Uncond => "none",
                };
                return Err(format!(
                    "cond {word} without matrix — the conditioning belongs to a matrix head"
                ));
            }
            if drivers.is_some() || targets.is_some() || channels.is_some() {
                return Err(
                    "matrix arm list without a matrix head — the arms belong to matrix <label> rect|full|upper"
                        .into(),
                );
            }
            if !derived.is_empty() {
                return Err(format!(
                    "from derived node '{}' without a matrix head — the derivation belongs to a declared matrix driver, target or channel",
                    derived[0].0
                ));
            }
            if fdr.is_some() {
                return Err("fdr without matrix — the correction belongs to a matrix head".into());
            }
            if expect.is_some() {
                return Err(
                    "expect cells without matrix — the lint belongs to a matrix head".into(),
                );
            }
            return Ok(None);
        }
    };
    if !conds.is_empty() {
        return Err(format!(
            "matrix refuses the fixed cond '{}' — the matrix carries cond rest|none only",
            conds[0].name
        ));
    }
    let cond =
        cond.ok_or("matrix carries no cond — cond rest|none is mandatory, never a silent default")?;
    let fdr = fdr.ok_or(
        "matrix carries no fdr — fdr bh|by <q> over matrix|row|col is mandatory, never a silent default",
    )?;
    let (drivers, targets, channels) = match shape {
        MatrixShape::Rect => {
            if channels.is_some() {
                return Err("matrix rect carries channels — rect uses drivers + targets".into());
            }
            let d = drivers.ok_or("matrix rect carries no drivers")?;
            let t = targets.ok_or("matrix rect carries no targets")?;
            (d, t, Vec::new())
        }
        MatrixShape::Full | MatrixShape::Upper => {
            if drivers.is_some() || targets.is_some() {
                return Err(
                    "matrix full|upper carries drivers/targets — full|upper uses channels".into(),
                );
            }
            let c = channels.ok_or("matrix full|upper carries no channels")?;
            (Vec::new(), Vec::new(), c)
        }
    };
    let spec = MatrixSpec {
        label,
        shape,
        drivers,
        targets,
        channels,
        cond,
        fdr,
        expect_cells: expect,
    };
    if let Some(n) = spec.expect_cells {
        let declared = spec.declared_cells();
        if n != declared {
            return Err(format!(
                "expect cells {n} differs from the declared {declared} cells"
            ));
        }
    }
    let pool = spec.pool();
    for (name, _) in derived {
        if !pool.iter().any(|p| p == name) {
            return Err(format!(
                "from derived node '{name}' is missing from the matrix pool — the derived node belongs in drivers, targets or channels"
            ));
        }
    }
    Ok(Some(spec))
}

fn parse_descriptor(text: &str) -> Result<Descriptor, String> {
    let mut pair = None;
    let mut driver: Option<Arm> = None;
    let mut target: Option<Arm> = None;
    let mut conds: Vec<Arm> = Vec::new();
    let mut events = Vec::new();
    let mut gates = Vec::new();
    let mut cadence = false;
    let mut seasonal = Seasonal::None;
    let mut lags: Option<Vec<usize>> = None;
    let mut surrogate: Option<usize> = None;
    let mut register = Register::Sources;
    let mut witness_primary = false;
    let mut bin: Option<f64> = None;
    let mut event_conditional = false;
    let mut witness_arm: Option<Arm> = None;
    let mut count_quantiles: Option<usize> = None;
    let mut matrix_head: Option<(String, MatrixShape)> = None;
    let mut matrix_drivers: Option<Vec<String>> = None;
    let mut matrix_targets: Option<Vec<String>> = None;
    let mut matrix_channels: Option<Vec<String>> = None;
    let mut matrix_cond: Option<MatrixCond> = None;
    let mut matrix_fdr: Option<(FdrMethod, f64, FdrScope)> = None;
    let mut matrix_expect: Option<usize> = None;
    let mut derived: Vec<(String, Vec<String>)> = Vec::new();

    for (lineno, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let Some(head) = parts.first().copied() else {
            continue;
        };
        let at = lineno + 1;
        match head {
            "pair" => {
                let name = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: pair carries no label"))?;
                pair = Some(name.to_string());
            }
            "driver" | "target" => {
                let name = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: {head} carries no field name"))?;
                let state = match parts.get(2).copied() {
                    Some(t) => State::parse(t).ok_or_else(|| {
                        format!("descriptor:{at}: {head} state '{t}' names no built|pending|probe")
                    })?,
                    None => State::Built,
                };
                if head == "driver" && witness_primary {
                    return Err(format!(
                        "descriptor:{at}: driver and witness name one primary arm — one per round"
                    ));
                }
                let arm = Arm {
                    name: name.to_string(),
                    state,
                };
                if head == "driver" {
                    driver = Some(arm);
                } else {
                    target = Some(arm);
                }
            }
            "cond" => {
                let token = *parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: cond carries no token"))?;
                if token == "rest" || token == "none" {
                    if matrix_cond.is_some() {
                        return Err(format!(
                            "descriptor:{at}: cond declared twice — one conditioning arm per matrix"
                        ));
                    }
                    matrix_cond = Some(if token == "rest" {
                        MatrixCond::Rest
                    } else {
                        MatrixCond::Uncond
                    });
                } else {
                    let state = match parts.get(2).copied() {
                        Some(t) => State::parse(t).ok_or_else(|| {
                            format!(
                                "descriptor:{at}: cond state '{t}' names no built|pending|probe"
                            )
                        })?,
                        None => State::Built,
                    };
                    conds.push(Arm {
                        name: token.to_string(),
                        state,
                    });
                }
            }
            "matrix" => {
                let label = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: matrix carries no label"))?;
                let shape_token = parts
                    .get(2)
                    .ok_or_else(|| format!("descriptor:{at}: matrix carries no rect|full|upper"))?;
                let shape = MatrixShape::parse(shape_token).ok_or_else(|| {
                    format!(
                        "descriptor:{at}: matrix shape '{shape_token}' names no rect|full|upper"
                    )
                })?;
                if matrix_head.is_some() {
                    return Err(format!("descriptor:{at}: matrix declared twice"));
                }
                matrix_head = Some((label.to_string(), shape));
            }
            "drivers" | "targets" | "channels" => {
                let token = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: {head} carries no list"))?;
                let list = parse_name_list(token)?;
                match head {
                    "drivers" => {
                        if matrix_drivers.is_some() {
                            return Err(format!("descriptor:{at}: drivers declared twice"));
                        }
                        matrix_drivers = Some(list);
                    }
                    "targets" => {
                        if matrix_targets.is_some() {
                            return Err(format!("descriptor:{at}: targets declared twice"));
                        }
                        matrix_targets = Some(list);
                    }
                    _ => {
                        if matrix_channels.is_some() {
                            return Err(format!("descriptor:{at}: channels declared twice"));
                        }
                        matrix_channels = Some(list);
                    }
                }
            }
            "from" => {
                let derived_name = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: from carries no derived name"))?;
                let token = parts.get(2).ok_or_else(|| {
                    format!("descriptor:{at}: from '{derived_name}' carries no carrier list")
                })?;
                if derived.iter().any(|(n, _)| n == derived_name) {
                    return Err(format!(
                        "descriptor:{at}: from '{derived_name}' declared twice — a duplicate derived node is no family"
                    ));
                }
                derived.push((derived_name.to_string(), parse_name_list(token)?));
            }
            "fdr" => {
                let method = match parts.get(1).copied() {
                    Some("bh") => FdrMethod::Bh,
                    Some("by") => FdrMethod::By,
                    Some(other) => {
                        return Err(format!(
                            "descriptor:{at}: fdr method '{other}' names no bh|by"
                        ));
                    }
                    None => {
                        return Err(format!("descriptor:{at}: fdr carries no bh|by method"));
                    }
                };
                let q_token = parts
                    .get(2)
                    .ok_or_else(|| format!("descriptor:{at}: fdr carries no q level"))?;
                let q: f64 = q_token
                    .parse()
                    .map_err(|_| format!("descriptor:{at}: fdr q '{q_token}' carries no number"))?;
                if !(q.is_finite() && q > 0.0 && q <= 1.0) {
                    return Err(format!(
                        "descriptor:{at}: fdr q '{q_token}' lies outside (0, 1]"
                    ));
                }
                if parts.get(3).copied() != Some("over") {
                    return Err(format!(
                        "descriptor:{at}: fdr carries no 'over matrix|row|col' scope"
                    ));
                }
                let scope = match parts.get(4).copied() {
                    Some("matrix") => FdrScope::Matrix,
                    Some("row") => FdrScope::Row,
                    Some("col") => FdrScope::Col,
                    Some(other) => {
                        return Err(format!(
                            "descriptor:{at}: fdr scope '{other}' names no matrix|row|col"
                        ));
                    }
                    None => {
                        return Err(format!("descriptor:{at}: fdr carries no scope"));
                    }
                };
                if matrix_fdr.is_some() {
                    return Err(format!("descriptor:{at}: fdr declared twice"));
                }
                matrix_fdr = Some((method, q, scope));
            }
            "expect" => {
                if parts.get(1).copied() != Some("cells") {
                    return Err(format!("descriptor:{at}: expect names no 'cells <n>'"));
                }
                let n_token = parts
                    .get(2)
                    .ok_or_else(|| format!("descriptor:{at}: expect cells carries no count"))?;
                let n: usize = n_token.parse().map_err(|_| {
                    format!("descriptor:{at}: expect cells '{n_token}' carries no count")
                })?;
                if matrix_expect.is_some() {
                    return Err(format!("descriptor:{at}: expect declared twice"));
                }
                matrix_expect = Some(n);
            }
            "witness" => {
                let name = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: witness carries no name"))?;
                let state = match parts.get(2).copied() {
                    Some(t) => State::parse(t).ok_or_else(|| {
                        format!("descriptor:{at}: witness state '{t}' names no built|pending|probe")
                    })?,
                    None => State::Built,
                };
                if event_conditional {
                    if witness_arm.is_some() {
                        return Err(format!(
                            "descriptor:{at}: event-conditional form carries one witness — a second is one too many"
                        ));
                    }
                    witness_arm = Some(Arm {
                        name: name.to_string(),
                        state,
                    });
                    register = Register::Witnesses;
                } else {
                    if driver.is_some() || witness_primary {
                        return Err(format!(
                            "descriptor:{at}: witness and driver name one primary arm — one per round"
                        ));
                    }
                    witness_primary = true;
                    register = Register::Witnesses;
                    driver = Some(Arm {
                        name: name.to_string(),
                        state,
                    });
                }
            }
            "register" => {
                let token = parts
                    .get(1)
                    .copied()
                    .ok_or_else(|| format!("descriptor:{at}: register carries no token"))?;
                register = match token {
                    "sources" => Register::Sources,
                    "witnesses" => Register::Witnesses,
                    other => {
                        return Err(format!(
                            "descriptor:{at}: register '{other}' names no sources|witnesses"
                        ));
                    }
                };
            }
            "event" | "gate" => {
                let name = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: {head} carries no ref"))?;
                let state = parts.get(2).copied().ok_or_else(|| {
                    format!("descriptor:{at}: {head} needs a pending|probe state")
                })?;
                if !matches!(state, "pending" | "probe") {
                    return Err(format!(
                        "descriptor:{at}: {head} state '{state}' is no pending|probe ref (never a silent number)"
                    ));
                }
                let entry = (name.to_string(), state.to_string());
                if head == "event" {
                    events.push(entry);
                } else {
                    gates.push(entry);
                }
            }
            "cadence" => {
                let token = parts
                    .get(1)
                    .copied()
                    .ok_or_else(|| format!("descriptor:{at}: cadence carries no value"))?;
                if token != "live" {
                    return Err(format!(
                        "descriptor:{at}: cadence '{token}' is no live measurement — the cadence is measured, never defaulted"
                    ));
                }
                cadence = true;
            }
            "seasonal" => {
                let token = parts
                    .get(1)
                    .copied()
                    .ok_or_else(|| format!("descriptor:{at}: seasonal carries no mode"))?;
                seasonal = match token {
                    "none" => Seasonal::None,
                    "climatology+standardize" => Seasonal::Climatology,
                    _ => {
                        return Err(format!(
                            "descriptor:{at}: seasonal '{token}' names no none|climatology+standardize"
                        ));
                    }
                };
            }
            "lags" => {
                let token = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: lags carries no list"))?;
                lags = Some(parse_lags(token)?);
            }
            "surrogate" => {
                let token = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: surrogate carries no count"))?;
                let n: usize = token.parse().map_err(|_| {
                    format!("descriptor:{at}: surrogate '{token}' carries no count")
                })?;
                surrogate = Some(n);
            }
            "bin" => {
                let token = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: bin carries no width"))?;
                let seconds: f64 = token.parse().map_err(|_| {
                    format!("descriptor:{at}: bin '{token}' carries no second count")
                })?;
                if !(seconds.is_finite() && seconds > 0.0) {
                    return Err(format!(
                        "descriptor:{at}: bin '{token}' is no positive finite width"
                    ));
                }
                bin = Some(seconds);
            }
            "count" => {
                let law = parts
                    .get(1)
                    .copied()
                    .ok_or_else(|| format!("descriptor:{at}: count carries no law"))?;
                if law != "quantile" {
                    return Err(format!("descriptor:{at}: count '{law}' names no quantile"));
                }
                let token = parts.get(2).ok_or_else(|| {
                    format!("descriptor:{at}: count quantile carries no bin count")
                })?;
                let q: usize = token.parse().map_err(|_| {
                    format!("descriptor:{at}: count quantile '{token}' carries no count")
                })?;
                if !q.is_power_of_two() || q < 2 {
                    return Err(format!(
                        "descriptor:{at}: count quantile {q} is no power of two >= 2"
                    ));
                }
                if !event_conditional {
                    return Err(format!(
                        "descriptor:{at}: count quantile belongs to form event-conditional"
                    ));
                }
                count_quantiles = Some(q);
            }
            "form" => {
                let token = parts
                    .get(1)
                    .copied()
                    .ok_or_else(|| format!("descriptor:{at}: form carries no name"))?;
                if token != "event-conditional" {
                    return Err(format!(
                        "descriptor:{at}: form '{token}' names no event-conditional"
                    ));
                }
                if witness_primary || driver.is_some() || target.is_some() {
                    return Err(format!(
                        "descriptor:{at}: form event-conditional must be declared before its witness/driver arms"
                    ));
                }
                event_conditional = true;
            }
            other => {
                return Err(format!("descriptor:{at}: unknown directive '{other}'"));
            }
        }
    }

    if !cadence {
        return Err(
            "cadence absent — the cadence is measured from the aligned grid, never defaulted"
                .into(),
        );
    }
    let matrix = build_matrix_spec(
        matrix_head,
        matrix_drivers,
        matrix_targets,
        matrix_channels,
        matrix_cond,
        matrix_fdr,
        matrix_expect,
        &conds,
        &derived,
    )?;
    if matrix.is_some() && event_conditional {
        return Err(
            "descriptor carries both a matrix and the event-conditional form — one round, one form"
                .into(),
        );
    }
    if matrix.is_some() {
        if lags.is_none() {
            return Err(
                "matrix carries no lags — lags is mandatory in the matrix, never a silent default"
                    .into(),
            );
        }
        if surrogate.is_none() {
            return Err(
                "matrix carries no surrogate — surrogate is mandatory in the matrix, never a silent default"
                    .into(),
            );
        }
    }
    let lags = match lags {
        Some(l) => l,
        None => default_lags(),
    };
    let surrogate = match surrogate {
        Some(s) => s,
        None => 100,
    };
    if event_conditional {
        let target = witness_arm
            .ok_or_else(|| "event-conditional form carries no witness arm".to_string())?;
        let driver =
            driver.ok_or_else(|| "event-conditional form carries no driver arm".to_string())?;
        return Ok(Descriptor {
            pair,
            driver: Some(driver),
            target: Some(target),
            conds,
            events,
            gates,
            seasonal,
            lags,
            surrogate,
            register: Register::Witnesses,
            bin,
            event_conditional: true,
            count_quantiles,
            matrix: None,
            derived: Vec::new(),
        });
    }
    if matrix.is_none() {
        if driver.is_none() {
            return Err("descriptor carries no driver arm".to_string());
        }
        if target.is_none() {
            return Err("descriptor carries no target arm".to_string());
        }
    }
    Ok(Descriptor {
        pair,
        driver,
        target,
        conds,
        events,
        gates,
        seasonal,
        lags,
        surrogate,
        register,
        bin,
        event_conditional: false,
        count_quantiles: None,
        matrix,
        derived,
    })
}

fn descriptor_from_args(args: &[String]) -> Result<Descriptor, String> {
    let driver = arg_after(args, "--driver").ok_or("--driver absent")?;
    let target = arg_after(args, "--target")
        .or(arg_after(args, "--field"))
        .ok_or("--target/--field absent")?;
    let conds: Vec<Arm> = {
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < args.len() {
            if args[i] == "--cond" {
                if let Some(n) = args.get(i + 1) {
                    out.push(Arm {
                        name: n.to_string(),
                        state: State::Built,
                    });
                    i += 1;
                }
            }
            i += 1;
        }
        out
    };
    let seasonal = match arg_after(args, "--seasonal") {
        Some(s) => s,
        None => "none",
    };
    let seasonal = match seasonal {
        "none" => Seasonal::None,
        "climatology+standardize" => Seasonal::Climatology,
        other => {
            return Err(format!(
                "--seasonal '{other}' names no none|climatology+standardize"
            ));
        }
    };
    let lags = match arg_after(args, "--lags") {
        Some(t) => parse_lags(t)?,
        None => default_lags(),
    };
    let surrogate = match arg_after(args, "--surrogate") {
        Some(t) => t
            .parse()
            .map_err(|_| format!("--surrogate '{t}' carries no count"))?,
        None => 100,
    };
    let register = match arg_after(args, "--register") {
        Some("sources") => Register::Sources,
        Some("witnesses") => Register::Witnesses,
        Some(other) => {
            return Err(format!("--register '{other}' names no sources|witnesses"));
        }
        None => Register::Sources,
    };
    let bin = match arg_after(args, "--bin") {
        Some(t) => {
            let seconds: f64 = t
                .parse()
                .map_err(|_| format!("--bin '{t}' carries no second count"))?;
            if !(seconds.is_finite() && seconds > 0.0) {
                return Err(format!("--bin '{t}' is no positive finite width"));
            }
            Some(seconds)
        }
        None => None,
    };
    Ok(Descriptor {
        pair: None,
        driver: Some(Arm {
            name: driver.to_string(),
            state: State::Built,
        }),
        target: Some(Arm {
            name: target.to_string(),
            state: State::Built,
        }),
        conds,
        events: Vec::new(),
        gates: Vec::new(),
        seasonal,
        lags,
        surrogate,
        register,
        bin,
        event_conditional: false,
        count_quantiles: None,
        matrix: None,
        derived: Vec::new(),
    })
}

fn field_matches(fc: &FieldConfig, station: Option<&str>, name: &str) -> bool {
    if fc.name == name || fc.key == name {
        return true;
    }
    match station {
        Some(code) => {
            let lower = code.to_ascii_lowercase();
            name == format!("{}_{}", fc.name, lower) || name == format!("{}_{}", fc.key, lower)
        }
        None => false,
    }
}

fn source_fields(s: &SourceConfig) -> Vec<FieldConfig> {
    let mut out = Vec::new();
    for e in &s.extracts {
        match e {
            Extract::Field(fc)
            | Extract::First(fc, _)
            | Extract::Last(fc, _)
            | Extract::Count(fc)
            | Extract::LastRow(fc)
            | Extract::ObjLast(fc)
            | Extract::Path(fc)
            | Extract::Deep(fc)
            | Extract::Regex(fc) => out.push(fc.clone()),
            Extract::Map { fields, .. }
            | Extract::CelestialMap { fields, .. }
            | Extract::ProfileMap { fields, .. }
            | Extract::EpnCore { fields, .. }
            | Extract::Rows { fields, .. }
            | Extract::Flatten { fields, .. }
            | Extract::CmrPolygon { fields, .. }
            | Extract::CelestialPolygon { fields, .. }
            | Extract::KeplerMap { fields, .. } => out.extend(fields.iter().cloned()),
            _ => {}
        }
    }
    out
}

fn field_sources(sources: &[SourceConfig], name: &str) -> Vec<(SourceConfig, FieldConfig)> {
    let mut out = Vec::new();
    for s in sources {
        for fc in source_fields(s) {
            if field_matches(&fc, s.station_code.as_deref(), name) {
                out.push((s.clone(), fc));
            }
        }
    }
    out
}

fn load_field_across_sources(
    sources: &[SourceConfig],
    name: &str,
    anchor: &QueryAnchor,
) -> Result<Option<Vec<(f64, f64)>>, String> {
    let candidates = field_sources(sources, name);
    if candidates.is_empty() {
        return Ok(None);
    }
    let mut reason = String::new();
    for (source, field) in candidates {
        match load_field(&source, &field, anchor) {
            Ok(series) => return Ok(Some(series)),
            Err(first) if reason.is_empty() => reason = first,
            Err(_) => {}
        }
    }
    Err(reason)
}

fn witness_kind_token(token: &str) -> Option<WitnessKind> {
    match token {
        "s2-direction" => Some(WitnessKind::S2Direction),
        "point-event" => Some(WitnessKind::PointEvent),
        "gestalt" => Some(WitnessKind::Gestalt),
        "presence" => Some(WitnessKind::Presence),
        "substance" => Some(WitnessKind::Substance),
        _ => None,
    }
}

fn load_witnesses() -> Vec<WitnessRecord> {
    match std::fs::read_to_string("phi/witnesses.φ") {
        Ok(content) => parse_witnesses(&content),
        Err(_) => Vec::new(),
    }
}

fn parse_witnesses(content: &str) -> Vec<WitnessRecord> {
    let mut out: Vec<WitnessRecord> = Vec::new();
    let mut block: Vec<&str> = Vec::new();
    for raw in content.lines().chain(std::iter::once("")) {
        let line = raw.trim();
        if line.is_empty() {
            if !block.is_empty() {
                if let Some(w) = witness_from_block(&block, &out) {
                    out.push(w);
                }
                block.clear();
            }
            continue;
        }
        block.push(line);
    }
    out
}

fn witness_from_block(lines: &[&str], out: &[WitnessRecord]) -> Option<WitnessRecord> {
    let mut kind: Option<WitnessKind> = None;
    let mut kind_token = String::new();
    let mut url: Option<String> = None;
    let mut records: Vec<String> = Vec::new();
    let mut force: Option<String> = None;
    for line in lines {
        if line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(head) = parts.next() else {
            continue;
        };
        match head {
            "witness" => {
                let Some(tok) = parts.next() else {
                    continue;
                };
                kind = witness_kind_token(tok);
                kind_token = tok.to_string();
            }
            "url" => {
                if let Some(v) = parts.next() {
                    url = Some(v.to_string());
                }
            }
            "record" => {
                for t in parts {
                    records.push(t.to_string());
                }
            }
            "force" => {
                force = parts.next().map(|s| s.to_string());
            }
            _ => {}
        }
    }
    let url = url?;
    if url.is_empty() || kind_token.is_empty() {
        return None;
    }
    let idx = out.iter().filter(|w| w.kind_token == kind_token).count();
    let key = format!("{kind_token}#{idx}");
    Some(WitnessRecord {
        kind,
        kind_token,
        key,
        url,
        records,
        force,
    })
}

fn find_witness<'a>(witnesses: &'a [WitnessRecord], name: &str) -> Option<&'a WitnessRecord> {
    if let Some(w) = witnesses.iter().find(|w| w.key == name) {
        return Some(w);
    }
    let by_record: Vec<&WitnessRecord> = witnesses
        .iter()
        .filter(|w| w.records.iter().any(|r| r == name))
        .collect();
    if by_record.len() == 1 {
        return Some(by_record[0]);
    }
    let by_url: Vec<&WitnessRecord> = witnesses.iter().filter(|w| w.url.contains(name)).collect();
    if by_url.len() == 1 {
        return Some(by_url[0]);
    }
    None
}

fn witness_series(w: &WitnessRecord) -> Result<Vec<(f64, f64)>, String> {
    let Some(bytes) = fetch_raw_bytes_headers(&w.url, &[]) else {
        return Err(format!("witness '{}' fetch void", w.key));
    };
    let text = String::from_utf8_lossy(&bytes);
    let stamps = event_unix_from_text(&text);
    if stamps.is_empty() {
        return Err(format!(
            "witness '{}' record carries no parsed event timestamp",
            w.key
        ));
    }
    let Some(lsk) = embedded_lsk() else {
        return Err("leap-second table absent — the witness event train stays unmeasured".into());
    };
    let mut out = Vec::new();
    for unix in stamps {
        if let Some(t) = lsk.unix_to_tdb(unix) {
            if t.is_finite() {
                out.push((t, 1.0));
            }
        }
    }
    if out.is_empty() {
        return Err(format!(
            "witness '{}' carries no timestamp inside the leap-second window",
            w.key
        ));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    Ok(out)
}

fn event_unix_from_text(text: &str) -> Vec<f64> {
    let mut out = Vec::new();
    for key in ["\"time_\"", "\"time\"", "\"at\""] {
        let mut from = 0usize;
        while let Some(rel) = text[from..].find(key) {
            let at = from + rel + key.len();
            from = at;
            let rest = text[at..].trim_start();
            let Some(body) = rest.strip_prefix(':') else {
                continue;
            };
            if let Some(v) = parse_json_time_scalar(body.trim_start()) {
                out.push(v);
            }
        }
    }
    out
}

fn parse_json_time_scalar(body: &str) -> Option<f64> {
    if let Some(inner) = body.strip_prefix('"') {
        let end = inner.find('"')?;
        return parse_epoch(&inner[..end]);
    }
    let end = body
        .find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace())
        .unwrap_or(body.len());
    let v: f64 = body[..end].parse().ok()?;
    if !v.is_finite() {
        return None;
    }
    Some(if v.abs() >= 1.0e11 { v / 1_000.0 } else { v })
}

fn offset_sign(time: &str) -> Option<usize> {
    if let Some(i) = time.rfind('+') {
        return Some(i);
    }
    match time.rfind('-') {
        Some(i) if i > 0 => Some(i),
        _ => None,
    }
}

fn parse_epoch(s: &str) -> Option<f64> {
    let s = s.trim();
    if let Ok(v) = s.parse::<f64>() {
        return if v.is_finite() { Some(v) } else { None };
    }
    let (date, time) = match s.split_once('T') {
        Some((d, t)) => (d, t),
        None => match s.split_once(' ') {
            Some((d, t)) => (d, t),
            None => (s, "0"),
        },
    };
    let (time, shift_s) = match offset_sign(time) {
        Some(i) => {
            let sign = if time.as_bytes()[i] == b'+' {
                1i64
            } else {
                -1i64
            };
            let off = &time[i + 1..];
            let mut op = off.split(':');
            let oh: i64 = op.next()?.parse().ok()?;
            let om: i64 = match op.next() {
                Some(v) => v.parse().ok()?,
                None => 0,
            };
            (&time[..i], sign * (oh * 3_600 + om * 60))
        }
        None => (time, 0),
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let clock = match time.split(|c| c == '.' || c == 'Z' || c == 'z').next() {
        Some(c) => c,
        None => "0",
    };
    let mut tp = clock.split(':');
    let hh: i64 = match tp.next() {
        Some(v) => v.parse().ok()?,
        None => 0,
    };
    let mm: i64 = match tp.next() {
        Some(v) => v.parse().ok()?,
        None => 0,
    };
    let ss: i64 = match tp.next() {
        Some(v) => v.parse().ok()?,
        None => 0,
    };
    let days = days_from_civil(y, m, d)?;
    Some(
        days as f64 * 86_400.0 + hh as f64 * 3_600.0 + mm as f64 * 60.0 + ss as f64
            - shift_s as f64,
    )
}

const DIRECTION_SURROGATES: usize = 1000;
const DIRECTION_ALPHA: f64 = 0.05;

const DIRECTION_RA_KEYS: [&str; 10] = [
    "ra",
    "ra_deg",
    "ra2000",
    "raj2000",
    "ra_mean",
    "ramean",
    "src_ra",
    "right_ascension",
    "ra_icrs",
    "centroid_ra",
];
const DIRECTION_DEC_KEYS: [&str; 10] = [
    "dec",
    "dec_deg",
    "dec2000",
    "dej2000",
    "dec_mean",
    "decmean",
    "src_dec",
    "declination",
    "dec_icrs",
    "centroid_dec",
];

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn uniform_unit(state: &mut u64) -> f64 {
    (splitmix64(state) >> 11) as f64 / (1u64 << 53) as f64
}

fn scalar_after(rest: &str) -> Option<f64> {
    let rest = rest.trim_start();
    let rest = rest
        .strip_prefix(':')
        .or_else(|| rest.strip_prefix('='))?
        .trim_start();
    let end = rest
        .find(|c: char| {
            !(c.is_ascii_digit() || c == '.' || c == '+' || c == '-' || c == 'e' || c == 'E')
        })
        .unwrap_or(rest.len());
    if end == 0 {
        return None;
    }
    let v: f64 = rest[..end].parse().ok()?;
    v.is_finite().then_some(v)
}

fn object_key_scalar(body: &str, keys: &[&str]) -> Option<f64> {
    for key in keys {
        let quoted = format!("\"{key}\"");
        if let Some(pos) = body.find(&quoted) {
            if let Some(v) = scalar_after(&body[pos + quoted.len()..]) {
                return Some(v);
            }
        }
    }
    let lower = body.to_ascii_lowercase();
    for key in keys {
        let pat = format!("{key}=");
        if let Some(pos) = lower.find(&pat) {
            if let Some(v) = scalar_after(&body[pos + key.len()..]) {
                return Some(v);
            }
        }
    }
    None
}

fn parse_object_pairs(text: &str) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    collect_objects(text, &mut out);
    out
}

fn strip_nested_braces(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut depth = 0i32;
    for c in body.chars() {
        match c {
            '{' => depth += 1,
            '}' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {
                if depth == 0 {
                    out.push(c);
                } else {
                    out.push(' ');
                }
            }
        }
    }
    out
}

fn collect_objects(text: &str, out: &mut Vec<(f64, f64)>) {
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'{' {
            i += 1;
            continue;
        }
        let start = i + 1;
        let mut depth = 1usize;
        let mut j = start;
        while j < bytes.len() && depth > 0 {
            match bytes[j] {
                b'{' => depth += 1,
                b'}' => depth -= 1,
                _ => {}
            }
            j += 1;
        }
        if depth != 0 {
            break;
        }
        let body = &text[start..j - 1];
        let shallow = strip_nested_braces(body);
        if let (Some(ra), Some(dec)) = (
            object_key_scalar(&shallow, &DIRECTION_RA_KEYS),
            object_key_scalar(&shallow, &DIRECTION_DEC_KEYS),
        ) {
            out.push((ra, dec));
        }
        if body.contains('{') {
            collect_objects(body, out);
        }
        i = j;
    }
}

fn parse_column_pairs(text: &str) -> Vec<(f64, f64)> {
    let mut lines = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'));
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let cols: Vec<String> = header
        .split(',')
        .map(|c| c.trim().trim_matches('"').to_ascii_lowercase())
        .collect();
    let ra_i = cols
        .iter()
        .position(|c| DIRECTION_RA_KEYS.contains(&c.as_str()));
    let dec_i = cols
        .iter()
        .position(|c| DIRECTION_DEC_KEYS.contains(&c.as_str()));
    let (Some(ra_i), Some(dec_i)) = (ra_i, dec_i) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in lines {
        let cells: Vec<&str> = line.split(',').collect();
        let cell = |i: usize| -> Option<f64> {
            cells
                .get(i)?
                .trim()
                .trim_matches('"')
                .parse::<f64>()
                .ok()
                .filter(|v| v.is_finite())
        };
        if let (Some(ra), Some(dec)) = (cell(ra_i), cell(dec_i)) {
            out.push((ra, dec));
        }
    }
    out
}

fn witness_directions(w: &WitnessRecord) -> Result<Vec<(f64, f64)>, String> {
    let Some(bytes) = fetch_raw_bytes_headers(&w.url, &[]) else {
        return Err(format!("witness '{}' fetch void", w.key));
    };
    let text = String::from_utf8_lossy(&bytes);
    let mut pairs = parse_object_pairs(&text);
    if pairs.is_empty() {
        pairs = parse_column_pairs(&text);
    }
    if pairs.is_empty() {
        return Err(format!(
            "witness '{}' carries no parseable ra/dec pair ({} bytes fetched)",
            w.key,
            bytes.len()
        ));
    }
    Ok(pairs)
}

fn rayleigh_z(angles: &[f64]) -> Option<f64> {
    let n = angles.len();
    if n == 0 {
        return None;
    }
    let mut c = 0.0f64;
    let mut s = 0.0f64;
    for &a in angles {
        if !a.is_finite() {
            return None;
        }
        c += a.cos();
        s += a.sin();
    }
    let r = ((c / n as f64).powi(2) + (s / n as f64).powi(2)).sqrt();
    Some(n as f64 * r * r)
}

fn rayleigh_p(z: f64, n: usize) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let nf = n as f64;
    let corr = 1.0 + (2.0 * z - z * z) / (4.0 * nf)
        - (24.0 * z - 132.0 * z * z + 76.0 * z * z * z - 9.0 * z * z * z * z) / (288.0 * nf * nf);
    ((-z).exp() * corr).clamp(0.0, 1.0)
}

fn kuiper_v(values: &[f64]) -> Option<f64> {
    let n = values.len();
    if n == 0 {
        return None;
    }
    let mut v: Vec<f64> = Vec::with_capacity(n);
    for &x in values {
        if !x.is_finite() {
            return None;
        }
        v.push(x);
    }
    v.sort_by(|a, b| a.total_cmp(b));
    let nf = n as f64;
    let mut d_plus = f64::NEG_INFINITY;
    let mut d_minus = f64::NEG_INFINITY;
    for (i, &x) in v.iter().enumerate() {
        let i_f = i as f64;
        d_plus = d_plus.max((i_f + 1.0) / nf - x);
        d_minus = d_minus.max(x - i_f / nf);
    }
    Some(d_plus + d_minus)
}

fn kuiper_p(v: f64, n: usize) -> f64 {
    if n == 0 || v <= 0.0 {
        return 1.0;
    }
    let nf = n as f64;
    let lambda = (nf.sqrt() + 0.155 + 0.24 / nf.sqrt()) * v;
    let mut sum = 0.0f64;
    for k in 1..=100 {
        let kf = k as f64;
        let term = (-2.0 * kf * kf * lambda * lambda).exp();
        sum += if k % 2 == 1 { term } else { -term };
    }
    (2.0 * sum).clamp(0.0, 1.0)
}

fn von_mises_fit(angles: &[f64]) -> Option<(f64, f64)> {
    let n = angles.len();
    if n == 0 {
        return None;
    }
    let mut c = 0.0f64;
    let mut s = 0.0f64;
    for &a in angles {
        if !a.is_finite() {
            return None;
        }
        c += a.cos();
        s += a.sin();
    }
    let r = ((c / n as f64).powi(2) + (s / n as f64).powi(2)).sqrt();
    let mu = s.atan2(c).rem_euclid(TAU);
    let kappa = if r < 0.53 {
        2.0 * r + r.powi(3) + 5.0 * r.powi(5) / 6.0
    } else if r < 0.85 {
        -0.4 + 1.39 * r + 0.43 / (1.0 - r)
    } else if r < 1.0 {
        1.0 / (r.powi(3) - 4.0 * r * r + 3.0 * r)
    } else {
        f64::INFINITY
    };
    Some((mu, kappa))
}

fn direction_angles(pairs: &[(f64, f64)]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let ra_deg: Vec<f64> = pairs.iter().map(|(ra, _)| ra.rem_euclid(360.0)).collect();
    let angles: Vec<f64> = ra_deg
        .iter()
        .map(|d| d.to_radians().rem_euclid(TAU))
        .collect();
    let ra_frac: Vec<f64> = ra_deg.iter().map(|d| d / 360.0).collect();
    let dec_frac: Vec<f64> = pairs
        .iter()
        .map(|(_, dec)| ((dec + 90.0).rem_euclid(180.0)) / 180.0)
        .collect();
    (angles, ra_frac, dec_frac)
}

fn run_direction_query(name: &str, witnesses: &[WitnessRecord]) -> i32 {
    println!("=== direction query — circular form over phi/witnesses.φ ===");
    let Some(w) = find_witness(witnesses, name) else {
        println!("DIRECTION: pending — '{name}' stands in no witness block");
        return 0;
    };
    if w.kind != Some(WitnessKind::S2Direction) {
        println!(
            "DIRECTION: refuse — witness {} kind {} is no s2-direction arm",
            w.key, w.kind_token
        );
        return 0;
    }
    println!(
        "witness {} | kind {} | record {} | force {} | {}",
        w.key,
        w.kind_token,
        w.records.join("+"),
        w.force.as_deref().unwrap_or("absent"),
        w.url
    );
    let pairs = match witness_directions(w) {
        Ok(p) => p,
        Err(reason) => {
            println!("DIRECTION: pending — {reason}; the arm stays pending, never a silent 0.0");
            return 0;
        }
    };
    if pairs.len() < 3 {
        println!(
            "DIRECTION: pending — witness {} carries n = {} direction(s) < 3; a circular test needs a sample",
            w.key,
            pairs.len()
        );
        return 0;
    }
    let (angles, ra_frac, dec_frac) = direction_angles(&pairs);
    let z = rayleigh_z(&angles);
    let v_ra = kuiper_v(&ra_frac);
    let v_dec = kuiper_v(&dec_frac);
    let fit = von_mises_fit(&angles);
    println!("directions loaded: n = {}", pairs.len());

    let mut state = SURROGATE_SEED;
    let mut zs = Vec::with_capacity(DIRECTION_SURROGATES);
    let mut vs = Vec::with_capacity(DIRECTION_SURROGATES);
    for _ in 0..DIRECTION_SURROGATES {
        let mut u_ra = Vec::with_capacity(pairs.len());
        let mut u_dec = Vec::with_capacity(pairs.len());
        for _ in 0..pairs.len() {
            u_ra.push(uniform_unit(&mut state));
            u_dec.push(uniform_unit(&mut state));
        }
        let sim_angles: Vec<f64> = u_ra.iter().map(|u| u * TAU).collect();
        if let Some(zz) = rayleigh_z(&sim_angles) {
            zs.push(zz);
        }
        if let Some(vv) = kuiper_v(&u_ra) {
            vs.push(vv);
        }
    }
    zs.sort_by(|a, b| a.total_cmp(b));
    vs.sort_by(|a, b| a.total_cmp(b));
    let thr_z = quantile(&zs, DIRECTION_ALPHA);
    let thr_v = quantile(&vs, DIRECTION_ALPHA);
    println!(
        "null: uniform direction null (Rayleigh z on RA, Kuiper V on RA/Dec) | B = {DIRECTION_SURROGATES}"
    );
    if let Some(z) = z {
        let word = match thr_z {
            Some(t) if z > t => "non-uniform",
            Some(_) => "uniform-consistent",
            None => "threshold pending",
        };
        println!(
            "Rayleigh (RA): z = {z:.4e} | p = {:.4e} | surrogate threshold = {} | {word}",
            rayleigh_p(z, pairs.len()),
            fmt_opt(thr_z)
        );
    } else {
        println!("Rayleigh (RA): absent — the sample carries a non-finite angle");
    }
    if let Some(v) = v_ra {
        let word = match thr_v {
            Some(t) if v > t => "non-uniform",
            Some(_) => "uniform-consistent",
            None => "threshold pending",
        };
        println!(
            "Kuiper (RA): V = {v:.4e} | p = {:.4e} | surrogate threshold = {} | {word}",
            kuiper_p(v, pairs.len()),
            fmt_opt(thr_v)
        );
    }
    if let Some(v) = v_dec {
        println!(
            "Kuiper (Dec): V = {v:.4e} | p = {:.4e}",
            kuiper_p(v, pairs.len())
        );
    }
    match fit {
        Some((mu, kappa)) => println!("von Mises fit: mu = {mu:.4e} rad | kappa = {kappa:.4e}"),
        None => println!("von Mises fit: absent — the sample carries a non-finite angle"),
    }
    println!(
        "null discipline: the threshold is the measured (1 - alpha = {:.2}) surrogate quantile over {DIRECTION_SURROGATES} uniform draws; the arm is carried, never a silent 0.0.",
        1.0 - DIRECTION_ALPHA
    );
    0
}

fn witness_magic(records: &[String]) -> Option<[u8; 4]> {
    for r in records {
        let upper = r.trim().to_ascii_uppercase();
        if upper.len() == 4 {
            let b = upper.as_bytes();
            return Some([b[0], b[1], b[2], b[3]]);
        }
    }
    None
}

fn spectral_epoch_names(arg: &str) -> Vec<&str> {
    arg.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect()
}

fn parse_axis_value_pairs(text: &str) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut nums = line
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter_map(|t| t.trim().parse::<f64>().ok())
            .filter(|v| v.is_finite());
        if let (Some(axis), Some(val)) = (nums.next(), nums.next()) {
            out.push((axis, val));
        }
    }
    out
}

fn spectral_epoch_series(w: &WitnessRecord) -> Result<Vec<(f64, f64)>, String> {
    let url = resolve_time_markers(&w.url);
    guard_url_template_resolved(&url)?;
    let Some(bytes) = fetch_raw_bytes_headers(&url, &[]) else {
        return Err(format!("witness '{}' fetch void ({url})", w.key));
    };
    let text = String::from_utf8_lossy(&bytes);
    if w.records.iter().any(|r| r == "rixs")
        && let Some(spec) = omegaflow::rixs::parse_sw_spin(&text)
    {
        return Ok(spec.eloss_ev.into_iter().zip(spec.weight).collect());
    }
    let pairs = parse_axis_value_pairs(&text);
    if pairs.is_empty() {
        return Err(format!(
            "witness '{}' carries no parseable axis/value series ({} bytes fetched)",
            w.key,
            bytes.len()
        ));
    }
    Ok(pairs)
}

fn axis_tolerance(axis: &[(f64, f64)]) -> f64 {
    let mut xs: Vec<f64> = axis.iter().map(|&(x, _)| x).collect();
    xs.sort_by(|a, b| a.total_cmp(b));
    let mut min_gap = f64::INFINITY;
    for pair in xs.windows(2) {
        let gap = pair[1] - pair[0];
        if gap > 0.0 && gap < min_gap {
            min_gap = gap;
        }
    }
    if min_gap.is_finite() {
        min_gap * 0.5
    } else {
        1.0e-6
    }
}

fn align_spectral_epochs(
    epoch_a: &[(f64, f64)],
    epoch_b: &[(f64, f64)],
    tol: f64,
) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    for &(ax, av) in epoch_a {
        let mut best: Option<(f64, f64)> = None;
        for &(bx, bv) in epoch_b {
            let d = (bx - ax).abs();
            if d <= tol && best.is_none_or(|(bd, _)| d < bd) {
                best = Some((d, bv));
            }
        }
        if let Some((_, bv)) = best {
            out.push((ax, av, bv));
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

struct SpectralEpochComparison {
    n: usize,
    correlation: Option<f64>,
    mean_abs_delta: f64,
    max_abs_delta: f64,
    sign_agreement: f64,
}

fn compare_spectral_epochs(aligned: &[(f64, f64, f64)]) -> Option<SpectralEpochComparison> {
    if aligned.len() < 2 {
        return None;
    }
    let n = aligned.len() as f64;
    let mean_a = aligned.iter().map(|&(_, a, _)| a).sum::<f64>() / n;
    let mean_b = aligned.iter().map(|&(_, _, b)| b).sum::<f64>() / n;
    let mean_abs_delta = aligned.iter().map(|&(_, a, b)| (a - b).abs()).sum::<f64>() / n;
    let max_abs_delta = aligned
        .iter()
        .map(|&(_, a, b)| (a - b).abs())
        .fold(0.0_f64, f64::max);
    let agree = aligned
        .iter()
        .filter(|&&(_, a, b)| (a - mean_a).signum() == (b - mean_b).signum())
        .count();
    let sign_agreement = agree as f64 / n;
    let mut cov = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;
    for &(_, a, b) in aligned {
        cov += (a - mean_a) * (b - mean_b);
        var_a += (a - mean_a).powi(2);
        var_b += (b - mean_b).powi(2);
    }
    let denom = (var_a * var_b).sqrt();
    let correlation = if denom > 0.0 { Some(cov / denom) } else { None };
    Some(SpectralEpochComparison {
        n: aligned.len(),
        correlation,
        mean_abs_delta,
        max_abs_delta,
        sign_agreement,
    })
}

fn run_spectral_query(name_arg: &str, witnesses: &[WitnessRecord]) -> i32 {
    println!("=== spectral / substance / gestalt form over phi/witnesses.φ ===");
    let names = spectral_epoch_names(name_arg);
    let Some(&first) = names.first() else {
        println!("SPECTRAL: pending — no witness named");
        return 0;
    };
    let Some(w0) = find_witness(witnesses, first) else {
        println!("SPECTRAL: pending — '{first}' stands in no witness block");
        return 0;
    };
    let nature = match w0.kind {
        Some(WitnessKind::Substance) => {
            "a single material-probe spectrum (one probe, no repeat epoch)"
        }
        Some(WitnessKind::Gestalt) => {
            "a body-surface raster (tau = geological stability, no stamped axis)"
        }
        Some(WitnessKind::S2Direction) => {
            "a direction catalogue (a circular arm — use --direction)"
        }
        Some(WitnessKind::Presence) => "a presence field",
        Some(WitnessKind::PointEvent) => "an event train (use the descriptor path)",
        None => "an unregistered witness kind",
    };
    println!(
        "witness {} | kind {} | record {} | force {} | {}",
        w0.key,
        w0.kind_token,
        w0.records.join("+"),
        w0.force.as_deref().unwrap_or("absent"),
        w0.url
    );
    let magic = witness_magic(&w0.records);
    let token = magic.map(|m| String::from_utf8_lossy(&m).into_owned());
    let identity = magic.and_then(magic_identity);
    println!(
        "magic_identity({}) = {:?} | series_gate = {:?}",
        token.as_deref().unwrap_or("absent"),
        identity,
        series_gate(magic, &[])
    );
    if names.len() < 2 {
        println!(
            "SPECTRAL: refuse — witness {} carries {nature}; the series gate refuses a witness record as an oscillator series (src/archivar/witness.rs:80), and the pre-registered spectral arm compares >= 2 aligned epochs of one probe. Missing data side: a repeated epoch axis (>= 2 epochs). No value is fabricated; the arm stays an honest refusal.",
            w0.key
        );
        return 0;
    }
    let mut epochs: Vec<&WitnessRecord> = Vec::with_capacity(names.len());
    for n in &names {
        match find_witness(witnesses, n) {
            Some(w) => epochs.push(w),
            None => {
                println!("SPECTRAL: pending — epoch '{n}' stands in no witness block");
                return 0;
            }
        }
    }
    for w in &epochs[1..] {
        if w.kind != w0.kind || w.records != w0.records {
            println!(
                "SPECTRAL: refuse — epoch {} (kind {}, record {}) is no repeated epoch of one probe (kind {}, record {})",
                w.key,
                w.kind_token,
                w.records.join("+"),
                w0.kind_token,
                w0.records.join("+")
            );
            return 0;
        }
    }
    let mut loaded: Vec<(String, Vec<(f64, f64)>)> = Vec::with_capacity(epochs.len());
    for w in &epochs {
        match spectral_epoch_series(w) {
            Ok(series) => loaded.push((w.key.clone(), series)),
            Err(reason) => {
                println!("SPECTRAL: pending — {reason}; the arm stays pending, never a silent 0.0");
                return 0;
            }
        }
    }
    println!(
        "epochs loaded: {} block(s) of one probe | reference {} carries {} point(s)",
        loaded.len(),
        loaded[0].0,
        loaded[0].1.len()
    );
    let tol = axis_tolerance(&loaded[0].1);
    let mut any = false;
    for (key, series) in &loaded[1..] {
        println!(
            "epoch {} carries {} point(s) | reference {} carries {} point(s)",
            key,
            series.len(),
            loaded[0].0,
            loaded[0].1.len()
        );
        let aligned = align_spectral_epochs(&loaded[0].1, series, tol);
        match compare_spectral_epochs(&aligned) {
            Some(cmp) => {
                any = true;
                println!(
                    "epoch pair {} vs {}: aligned n = {}",
                    loaded[0].0, key, cmp.n
                );
                println!(
                    "magnitude: mean |Δ| = {:.6e} | max |Δ| = {:.6e} | sign agreement = {:.4}",
                    cmp.mean_abs_delta, cmp.max_abs_delta, cmp.sign_agreement
                );
                match cmp.correlation {
                    Some(r) => println!("correlation over the aligned axis: r = {r:.6}"),
                    None => println!(
                        "correlation over the aligned axis: absent — one epoch carries no variance over the aligned axis"
                    ),
                }
                println!(
                    "the two epochs are carried side by side (never averaged): reference {} point(s), epoch {} point(s), shared axis {} point(s)",
                    loaded[0].1.len(),
                    series.len(),
                    cmp.n
                );
            }
            None => println!(
                "epoch pair {} vs {}: pending — {} aligned point(s) (< 2); the epochs carry no shared axis within tolerance {:.3e}",
                loaded[0].0,
                key,
                aligned.len(),
                tol
            ),
        }
    }
    if !any {
        println!(
            "SPECTRAL: pending — no epoch pair aligns to >= 2 points; the arm stays pending, never a silent 0.0"
        );
    }
    0
}

fn load_text_rows(src: &SourceConfig, fc: &FieldConfig, bytes: &[u8]) -> Option<Vec<(f64, f64)>> {
    let lsk = embedded_lsk()?;
    let (epoch_col, field_key) = src.extracts.iter().find_map(|e| match e {
        Extract::Rows {
            fields, epoch_cols, ..
        } if fields.iter().any(|f| field_matches(f, None, &fc.name)) => {
            let epoch = match epoch_cols
                .first()
                .and_then(|c| c.trim().parse::<usize>().ok())
            {
                Some(v) => v,
                None => 0,
            };
            let key = fc.key.trim().parse::<usize>().ok()?;
            Some((epoch, key))
        }
        _ => None,
    })?;
    let text = String::from_utf8_lossy(bytes);
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(|c| c.trim()).collect();
        let Some(epoch_s) = cols.get(epoch_col) else {
            continue;
        };
        let Some(unix) = parse_epoch(epoch_s) else {
            continue;
        };
        let Some(t) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let Some(raw) = cols.get(field_key).and_then(|c| c.parse::<f64>().ok()) else {
            continue;
        };
        if raw.is_finite() {
            out.push((t, raw));
        }
    }
    if out.is_empty() {
        None
    } else {
        out.sort_by(|a, b| a.0.total_cmp(&b.0));
        Some(out)
    }
}

fn text_source_for_field(src: &SourceConfig, name: &str) -> Option<SourceConfig> {
    let mut kept = Vec::new();
    for e in &src.extracts {
        match e {
            Extract::Field(fc) if field_matches(fc, None, name) => kept.push(e.clone()),
            Extract::First(fc, _)
            | Extract::Last(fc, _)
            | Extract::Path(fc)
            | Extract::Deep(fc)
            | Extract::Regex(fc)
                if field_matches(fc, None, name) =>
            {
                kept.push(e.clone());
            }
            Extract::Hapi(pairs) if pairs.iter().any(|(_, n)| n == name) => kept.push(e.clone()),
            _ => {}
        }
    }
    if kept.is_empty() {
        None
    } else {
        let mut filtered = src.clone();
        filtered.extracts = kept;
        Some(filtered)
    }
}

fn first_url_template_slot(url: &str) -> Option<&str> {
    let open = url.find('{')?;
    let after = &url[open + 1..];
    let close = after.find('}')?;
    Some(&after[..close])
}

fn resolve_time_markers(url: &str) -> String {
    let mut resolved = url.to_string();
    for (marker, value) in live_markers() {
        resolved = resolved.replace(&marker, &value);
    }
    resolved
}

fn guard_url_template_resolved(url: &str) -> Result<(), String> {
    match first_url_template_slot(url) {
        Some(slot) => Err(format!(
            "url carries the unresolved template slot '{{{slot}}}' — the epoch markers ({{now}}/{{week_ago}}/{{hour_ago}}) are filled from the running clock and the lat/lon/station slots from the query anchor, but this slot needs a query anchor that load_field does not hold; resolve it before the fetch, or the source stays unmeasured"
        )),
        None => Ok(()),
    }
}

fn load_field(
    src: &SourceConfig,
    fc: &FieldConfig,
    anchor: &QueryAnchor,
) -> Result<Vec<(f64, f64)>, String> {
    let url = resolve_time_markers(&src.url);
    let url = resolve_query_slots(&url, anchor)?;
    guard_url_template_resolved(&url)?;
    let Some(bytes) = fetch_raw_bytes_headers(&url, &src.headers) else {
        return Err(format!("{url} fetch void"));
    };
    if let Some(rows) = series_rows(&src.format, &bytes) {
        let mut out = Vec::new();
        for r in &rows {
            if series_component_name(&src.format, r.comp) == Some(fc.name.as_str())
                && r.value.is_finite()
            {
                out.push((r.t, r.value));
            }
        }
        if out.is_empty() {
            return Err(format!(
                "format {} carries no component '{}'",
                src.format, fc.name
            ));
        }
        out.sort_by(|a, b| a.0.total_cmp(&b.0));
        return Ok(out);
    }
    if let Some(recs) = geo_series_parse_bin(&src.format, &bytes) {
        let mut out = Vec::new();
        for r in &recs {
            if geo_series_component_name(&src.format, r.comp) == Some(fc.name.as_str())
                && r.val.is_finite()
            {
                out.push((r.t, r.val));
            }
        }
        if out.is_empty() {
            return Err(format!(
                "format {} carries no component '{}'",
                src.format, fc.name
            ));
        }
        out.sort_by(|a, b| a.0.total_cmp(&b.0));
        return Ok(out);
    }
    if src.format == "text" {
        if let Some(series) = load_text_rows(src, fc, &bytes) {
            return Ok(series);
        }
    }
    if let Some(filtered) = text_source_for_field(src, &fc.name) {
        let Some(lsk) = embedded_lsk() else {
            return Err(format!(
                "leap-second table absent — format {} text arm stays unmeasured",
                src.format
            ));
        };
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let mut series = extract_series(&filtered, &text, &lsk);
        if !series.is_empty() {
            series.sort_by(|a, b| a.0.total_cmp(&b.0));
            return Ok(series);
        }
        return Err(format!(
            "format {} text arm carries no series for '{}'",
            src.format, fc.name
        ));
    }
    Err(format!(
        "format {} carries no field selector arm for '{}'",
        src.format, fc.name
    ))
}

enum ArmLoad {
    Ready {
        source: SourceConfig,
        field: FieldConfig,
        series: Vec<(f64, f64)>,
    },
    WitnessReady {
        detail: String,
        series: Vec<(f64, f64)>,
    },
    Pending(String),
}

fn try_source(sources: &[SourceConfig], arm: &Arm, anchor: &QueryAnchor) -> Option<ArmLoad> {
    let candidates = field_sources(sources, &arm.name);
    if candidates.is_empty() {
        return None;
    }
    let mut reason: Option<String> = None;
    for (source, field) in candidates {
        match load_field(&source, &field, anchor) {
            Ok(series) => {
                return Some(ArmLoad::Ready {
                    source,
                    field,
                    series,
                });
            }
            Err(first) if reason.is_none() => reason = Some(first),
            Err(_) => {}
        }
    }
    reason.map(ArmLoad::Pending)
}

fn try_witness(witnesses: &[WitnessRecord], arm: &Arm) -> Option<ArmLoad> {
    find_witness(witnesses, &arm.name).map(|w| load_witness_arm(w, arm.state))
}

fn load_witness_arm(w: &WitnessRecord, state: State) -> ArmLoad {
    if state == State::Pending {
        return ArmLoad::Pending("descriptor state pending (register duty)".into());
    }
    if w.kind != Some(WitnessKind::PointEvent) {
        return ArmLoad::Pending(format!(
            "witness '{}' kind {} is no event channel (τ=0 catalogue/spectrum/gestalt) — never poured into a series, never 0.0",
            w.key, w.kind_token
        ));
    }
    match witness_series(w) {
        Ok(series) => ArmLoad::WitnessReady {
            detail: format!(
                "record {} force {} | {}",
                w.records.join("+"),
                w.force.as_deref().unwrap_or("absent"),
                w.url
            ),
            series,
        },
        Err(reason) => ArmLoad::Pending(reason),
    }
}

fn load_arm(
    sources: &[SourceConfig],
    witnesses: &[WitnessRecord],
    arm: &Arm,
    register: Register,
    anchor: &QueryAnchor,
) -> ArmLoad {
    if arm.state == State::Pending {
        return ArmLoad::Pending("descriptor state pending (register duty)".into());
    }
    let order: [u8; 2] = if register == Register::Witnesses {
        [1, 0]
    } else {
        [0, 1]
    };
    for which in order {
        let attempt = if which == 0 {
            try_source(sources, arm, anchor)
        } else {
            try_witness(witnesses, arm)
        };
        if let Some(load) = attempt {
            return load;
        }
    }
    ArmLoad::Pending(format!("'{}' stands in no register block", arm.name))
}

fn print_arm(role: &str, arm: &Arm, load: &ArmLoad) {
    match load {
        ArmLoad::Ready {
            source,
            field,
            series,
        } => println!(
            "ARM {role:<6} {} {} | format {} | force {} unit {} | n = {} | {}",
            arm.name,
            arm.state.name(),
            source.format,
            field.force,
            field.unit,
            series.len(),
            source.url
        ),
        ArmLoad::WitnessReady { detail, series } => {
            println!(
                "ARM {role:<6} {} {} | witness | n = {} | {detail}",
                arm.name,
                arm.state.name(),
                series.len()
            );
        }
        ArmLoad::Pending(reason) => {
            println!(
                "PENDING {role:<6} {} {} — {reason}",
                arm.name,
                arm.state.name()
            );
        }
    }
}

fn print_count_arm(role: &str, cells: &[Option<f64>]) {
    let carried = cells.iter().filter(|c| c.is_some()).count();
    let pending = cells.len() - carried;
    println!(
        "count arm {role}: {carried} bins carry events | {pending} bins pending (no event inside the fetched train — never 0.0 as a physical value)"
    );
}

fn median_dt(series: &[(f64, f64)]) -> Option<f64> {
    if series.len() < 2 {
        return None;
    }
    let mut d: Vec<f64> = series
        .windows(2)
        .map(|w| w[1].0 - w[0].0)
        .filter(|x| x.is_finite() && *x > 0.0)
        .collect();
    if d.is_empty() {
        return None;
    }
    d.sort_by(|a, b| a.total_cmp(b));
    Some(d[d.len() / 2])
}

fn bin_to_grid(series: &[(f64, f64)], grid: &[f64], last_step: f64) -> Vec<Option<f64>> {
    let mut sums = vec![0.0f64; grid.len()];
    let mut counts = vec![0u32; grid.len()];
    let mut mi = 0usize;
    for &(t, v) in series {
        if !v.is_finite() {
            continue;
        }
        while mi + 1 < grid.len() && t >= grid[mi + 1] {
            mi += 1;
        }
        if t < grid[mi] {
            continue;
        }
        let hi = match grid.get(mi + 1) {
            Some(&h) => h,
            None => grid[mi] + last_step,
        };
        if t >= hi {
            continue;
        }
        sums[mi] += v;
        counts[mi] += 1;
    }
    (0..grid.len())
        .map(|i| {
            if counts[i] > 0 {
                Some(sums[i] / counts[i] as f64)
            } else {
                None
            }
        })
        .collect()
}

fn bin_count_to_grid(series: &[(f64, f64)], grid: &[f64], last_step: f64) -> Vec<Option<f64>> {
    let mut counts = vec![0u32; grid.len()];
    let mut mi = 0usize;
    for &(t, _) in series {
        while mi + 1 < grid.len() && t >= grid[mi + 1] {
            mi += 1;
        }
        if t < grid[mi] {
            continue;
        }
        let hi = match grid.get(mi + 1) {
            Some(&h) => h,
            None => grid[mi] + last_step,
        };
        if t >= hi {
            continue;
        }
        counts[mi] += 1;
    }
    counts
        .iter()
        .map(|&c| if c > 0 { Some(c as f64) } else { None })
        .collect()
}

fn deseasonalize(series: &[Option<f64>]) -> Vec<Option<f64>> {
    let mut sums = [0.0f64; CAL_MONTHS];
    let mut sumsq = [0.0f64; CAL_MONTHS];
    let mut counts = [0u32; CAL_MONTHS];
    for (i, v) in series.iter().enumerate() {
        if let Some(x) = v {
            if x.is_finite() {
                sums[i % CAL_MONTHS] += x;
                sumsq[i % CAL_MONTHS] += x * x;
                counts[i % CAL_MONTHS] += 1;
            }
        }
    }
    let mut means = [0.0f64; CAL_MONTHS];
    let mut sds = [0.0f64; CAL_MONTHS];
    for (m, mean) in means.iter_mut().enumerate() {
        if counts[m] >= CLIMATOLOGY_FLOOR as u32 {
            let n = counts[m] as f64;
            *mean = sums[m] / n;
            let var = (sumsq[m] / n - *mean * *mean).max(0.0);
            sds[m] = var.sqrt();
        } else if counts[m] > 0 {
            println!(
                "deseasonalize: calendar month {:02} carries n = {} < floor {CLIMATOLOGY_FLOOR} — its values stay unchanged (climatology not removed)",
                m + 1,
                counts[m]
            );
        }
    }
    series
        .iter()
        .enumerate()
        .map(|(i, v)| match v {
            Some(x) if counts[i % CAL_MONTHS] >= CLIMATOLOGY_FLOOR as u32 => {
                let m = i % CAL_MONTHS;
                let centered = *x - means[m];
                if sds[m].is_finite() && sds[m] > 0.0 {
                    Some(centered / sds[m])
                } else {
                    Some(centered)
                }
            }
            other => *other,
        })
        .collect()
}

struct Aligned {
    driver: Vec<Option<f64>>,
    target: Vec<Option<f64>>,
    cond: Option<Vec<Option<f64>>>,
    grid: Vec<f64>,
    cadence_s: f64,
}

fn align(
    driver: &[(f64, f64)],
    target: &[(f64, f64)],
    cond: Option<&[(f64, f64)]>,
    seasonal: Seasonal,
    driver_count: bool,
    target_count: bool,
    cond_count: bool,
    bin_seconds: Option<f64>,
) -> Result<Aligned, String> {
    let monthly = matches!(seasonal, Seasonal::Climatology);
    let (grid, grid_dt) = match bin_seconds {
        Some(step) => {
            if !(step.is_finite() && step > 0.0) {
                return Err(format!("bin {step} is no positive finite width"));
            }
            let d0 = driver
                .first()
                .map(|p| p.0)
                .ok_or("driver carries no stamped sample")?;
            let d1 = driver
                .last()
                .map(|p| p.0)
                .ok_or("driver carries no stamped sample")?;
            let t0 = target
                .first()
                .map(|p| p.0)
                .ok_or("target carries no stamped sample")?;
            let t1 = target
                .last()
                .map(|p| p.0)
                .ok_or("target carries no stamped sample")?;
            let mut start = d0.max(t0);
            let mut end = d1.min(t1);
            if let Some(c) = cond {
                if let (Some(c0), Some(c1)) = (c.first().map(|p| p.0), c.last().map(|p| p.0)) {
                    start = start.max(c0);
                    end = end.min(c1);
                }
            }
            if !(end > start) {
                return Err("bin window carries no overlap between the arms".into());
            }
            let n = ((end - start) / step).ceil() as usize;
            if n == 0 {
                return Err("bin window carries no cell".into());
            }
            let grid: Vec<f64> = (0..n).map(|i| start + i as f64 * step).collect();
            (grid, step)
        }
        None => {
            let d_dt =
                median_dt(driver).ok_or("driver cadence underdetermined (< 2 stamped samples)")?;
            let t_dt =
                median_dt(target).ok_or("target cadence underdetermined (< 2 stamped samples)")?;
            let grid_from_target = if target_count && !driver_count {
                false
            } else if driver_count && !target_count {
                true
            } else {
                t_dt >= d_dt
            };
            let grid_dt = if grid_from_target { t_dt } else { d_dt };
            let grid: Vec<f64> = if grid_from_target {
                target.iter().map(|p| p.0).collect()
            } else {
                driver.iter().map(|p| p.0).collect()
            };
            (grid, grid_dt)
        }
    };
    if monthly && !(25.0 * 86_400.0..=32.0 * 86_400.0).contains(&grid_dt) {
        return Err(format!(
            "climatology+standardize needs a monthly grid arm (slowest cadence {grid_dt:.0} s); the deseasonalization lives on grid index mod {CAL_MONTHS}"
        ));
    }
    let last_step = if monthly { MONTH_S } else { grid_dt };
    let mut d_cells = if driver_count {
        bin_count_to_grid(driver, &grid, last_step)
    } else {
        bin_to_grid(driver, &grid, last_step)
    };
    let mut t_cells = if target_count {
        bin_count_to_grid(target, &grid, last_step)
    } else {
        bin_to_grid(target, &grid, last_step)
    };
    let mut c_cells = cond.map(|c| {
        if cond_count {
            bin_count_to_grid(c, &grid, last_step)
        } else {
            bin_to_grid(c, &grid, last_step)
        }
    });
    if monthly {
        d_cells = deseasonalize(&d_cells);
        t_cells = deseasonalize(&t_cells);
        c_cells = c_cells.map(|c| deseasonalize(&c));
    }
    Ok(Aligned {
        driver: d_cells,
        target: t_cells,
        cond: c_cells,
        grid,
        cadence_s: grid_dt,
    })
}

fn pair2(a: &[Option<f64>], b: &[Option<f64>]) -> (Vec<f32>, Vec<f32>) {
    let mut av = Vec::new();
    let mut bv = Vec::new();
    for (x, y) in a.iter().zip(b.iter()) {
        if let (Some(x), Some(y)) = (x, y) {
            if x.is_finite() && y.is_finite() {
                av.push(*x as f32);
                bv.push(*y as f32);
            }
        }
    }
    (av, bv)
}

fn pair3(
    a: &[Option<f64>],
    b: &[Option<f64>],
    c: &[Option<f64>],
) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut av = Vec::new();
    let mut bv = Vec::new();
    let mut cv = Vec::new();
    for ((x, y), z) in a.iter().zip(b.iter()).zip(c.iter()) {
        if let (Some(x), Some(y), Some(z)) = (x, y, z) {
            if x.is_finite() && y.is_finite() && z.is_finite() {
                av.push(*x as f32);
                bv.push(*y as f32);
                cv.push(*z as f32);
            }
        }
    }
    (av, bv, cv)
}

struct LagRow {
    lag: usize,
    te_d2t: Option<f64>,
    thr_d2t: Option<f64>,
    te_t2d: Option<f64>,
    thr_t2d: Option<f64>,
    bias_d2t: Option<f64>,
    bias_state_d2t: &'static str,
    bias_t2d: Option<f64>,
    bias_state_t2d: &'static str,
}

fn bias_column(
    te: Option<f64>,
    n: usize,
    n_eff: Option<f64>,
    arm: BiasArm,
) -> (Option<f64>, &'static str) {
    let Some(te) = te else {
        return (None, "absent");
    };
    let Some(floor) = arm.neff_floor() else {
        return (None, "floor_unmeasured");
    };
    let ne = n_eff.unwrap_or(f64::NAN);
    if !(ne.is_finite() && ne >= floor) {
        return (None, "unadjusted_below_floor");
    }
    match arm.table_lookup(n) {
        Some(mk) if te.is_finite() && mk.is_finite() => {
            (Some(transfer_entropy_bias_adjusted(te, mk)), "adjusted")
        }
        _ if arm.table_len() == 0 => (None, "table_pending"),
        _ => (None, "off_table"),
    }
}

struct Direction {
    word: String,
    best_lag: Option<usize>,
    best_te: Option<f64>,
}

struct MaxT {
    replicas: usize,
    finite: usize,
    quantile: f64,
    observed_max: Option<f64>,
    clears: bool,
    mde: Vec<Option<f64>>,
    n_eff: Vec<Option<f64>>,
}

struct QueryResult {
    n_paired: usize,
    fam: Option<f64>,
    fam_core: Option<f64>,
    rows: Vec<LagRow>,
    d2t: Direction,
    t2d: Direction,
    cte: Option<(f64, f64)>,
    maxt: Option<MaxT>,
}

fn direction_of(rows: &[LagRow], d2t: bool, fam: Option<f64>) -> Direction {
    let mut best: Option<(usize, f64, Option<f64>)> = None;
    for r in rows {
        let (te, thr) = if d2t {
            (r.te_d2t, r.thr_d2t)
        } else {
            (r.te_t2d, r.thr_t2d)
        };
        let Some(te) = te else {
            continue;
        };
        if best.map_or(true, |(_, b, _)| te > b) {
            best = Some((r.lag, te, thr));
        }
    }
    match best {
        None => Direction {
            word: "absent".into(),
            best_lag: None,
            best_te: None,
        },
        Some((lag, te, thr)) => {
            let word = match fam {
                Some(f) if te > f => "arrow",
                _ => match thr {
                    Some(t) if te > t => "family bound",
                    _ => "silent",
                },
            };
            Direction {
                word: word.into(),
                best_lag: Some(lag),
                best_te: Some(te),
            }
        }
    }
}

fn compute_max_t(
    members: &[Member],
    observed: &[Option<f64>],
    times_s: &[f64],
    block: usize,
    surrogates: usize,
) -> Option<MaxT> {
    let n = times_s.len();
    if n == 0 || n > MAXT_N_CAP || surrogates < 2 {
        return None;
    }
    let phase = phase_data(times_s);
    let threads = std::thread::available_parallelism().map_or(1, |p| p.get());
    let nulls = null_matrix(
        members,
        0..surrogates,
        block,
        ResampleMode::Driver,
        SURROGATE_SEED,
        threads,
        &phase,
    );
    let sigma = sigma_per_statistic(&nulls);
    let means = null_means_per_statistic(&nulls);
    let stud = studentized_maxima(&nulls, &means, &sigma);
    let mut finite: Vec<f64> = stud.iter().copied().filter(|v| v.is_finite()).collect();
    finite.sort_by(|a, b| a.total_cmp(b));
    let quantile_v = quantile(&finite, MAXT_ALPHA)?;
    let obs_stud: Vec<Option<f64>> = observed
        .iter()
        .enumerate()
        .map(|(mi, o)| {
            let mu = means.get(mi).copied().flatten()?;
            let s = sigma.get(mi).copied().flatten()?;
            Some(((*o)? - mu) / s)
        })
        .collect();
    let raw_max = obs_stud
        .iter()
        .flatten()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let observed_max = raw_max.is_finite().then_some(raw_max);
    let mde: Vec<Option<f64>> = sigma
        .iter()
        .map(|s| s.map(|sig| quantile_v * sig))
        .collect();
    let n_eff: Vec<Option<f64>> = members
        .iter()
        .map(|m| kde_n_eff(&m.target, &m.driver, m.lag))
        .collect();
    Some(MaxT {
        replicas: surrogates,
        finite: finite.len(),
        quantile: quantile_v,
        observed_max,
        clears: observed_max.is_some_and(|v| v > quantile_v),
        mde,
        n_eff,
    })
}

fn analyze(
    desc: &Descriptor,
    target_s: &[f32],
    driver_s: &[f32],
    times_s: &[f64],
    block: usize,
    cond_triple: Option<(&[f32], &[f32], &[f32])>,
    fam_override: Option<f64>,
) -> QueryResult {
    let mut members: Vec<Member> = Vec::new();
    let mut meta: Vec<(usize, bool)> = Vec::new();
    for &lag in &desc.lags {
        members.push(Member::new(
            "driver->target",
            0,
            lag,
            target_s.to_vec(),
            driver_s.to_vec(),
        ));
        meta.push((lag, true));
        members.push(Member::new(
            "target->driver",
            1,
            lag,
            driver_s.to_vec(),
            target_s.to_vec(),
        ));
        meta.push((lag, false));
    }
    let observed = observed_family(&members);
    let maxt = compute_max_t(&members, &observed, times_s, block, desc.surrogate);

    let mut rows: Vec<LagRow> = Vec::with_capacity(desc.lags.len());
    for &lag in &desc.lags {
        let thr_d2t =
            surrogate_stats_phase_n(target_s, driver_s, lag, SURROGATE_SEED, desc.surrogate)
                .map(|(_, _, t)| t);
        let thr_t2d =
            surrogate_stats_phase_n(driver_s, target_s, lag, SURROGATE_SEED, desc.surrogate)
                .map(|(_, _, t)| t);
        rows.push(LagRow {
            lag,
            te_d2t: None,
            thr_d2t,
            te_t2d: None,
            thr_t2d,
            bias_d2t: None,
            bias_state_d2t: "absent",
            bias_t2d: None,
            bias_state_t2d: "absent",
        });
    }
    for (mi, (lag, d2t)) in meta.iter().enumerate() {
        let Some(te) = observed.get(mi).copied().flatten() else {
            continue;
        };
        if let Some(row) = rows.iter_mut().find(|r| r.lag == *lag) {
            if *d2t {
                row.te_d2t = Some(te);
                let (adj, state) = bias_column(
                    Some(te),
                    target_s.len(),
                    kde_n_eff(target_s, driver_s, *lag),
                    BiasArm::ScalarKde,
                );
                row.bias_d2t = adj;
                row.bias_state_d2t = state;
            } else {
                row.te_t2d = Some(te);
                let (adj, state) = bias_column(
                    Some(te),
                    driver_s.len(),
                    kde_n_eff(driver_s, target_s, *lag),
                    BiasArm::ScalarKde,
                );
                row.bias_t2d = adj;
                row.bias_state_t2d = state;
            }
        }
    }

    let mut fam_core: Option<f64> = None;
    for &lag in &desc.lags {
        for (x, y) in [(target_s, driver_s), (driver_s, target_s)] {
            if let Some(m) = surrogate_max_phase_n(x, y, lag, SURROGATE_SEED, desc.surrogate) {
                fam_core = Some(match fam_core {
                    Some(f) => f.max(m),
                    None => m,
                });
            }
        }
    }
    let fam = fam_override.or(fam_core);

    let d2t = direction_of(&rows, true, fam);
    let t2d = direction_of(&rows, false, fam);
    let cte = cond_triple.and_then(|(t, d, c)| {
        conditional_embedded_te_phase(t, d, c, 3, SURROGATE_SEED).map(|v| (v.te, v.threshold))
    });

    QueryResult {
        n_paired: target_s.len(),
        fam,
        fam_core,
        rows,
        d2t,
        t2d,
        cte,
        maxt,
    }
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4e}"),
        None => "absent".to_string(),
    }
}

fn print_result(result: &QueryResult, desc: &Descriptor) {
    println!();
    println!(
        "=== TE machine (omegaflow::te, phase-randomized null, {} surrogates) | paired n = {} ===",
        desc.surrogate, result.n_paired
    );
    println!(
        "{:>4} | {:>14} | {:>14} | {:>14} | {:>14}",
        "lag", "TE(d->t)", "thr(d->t)", "TE(t->d)", "thr(t->d)"
    );
    for r in &result.rows {
        println!(
            "{:>4} | {:>14} | {:>14} | {:>14} | {:>14}",
            r.lag,
            fmt_opt(r.te_d2t),
            fmt_opt(r.thr_d2t),
            fmt_opt(r.te_t2d),
            fmt_opt(r.thr_t2d)
        );
    }
    println!();
    println!(
        "family bound fam = {} (round max over the measured directed pairs x lags, phase surrogates)",
        fmt_opt(result.fam)
    );
    if result.fam != result.fam_core {
        println!(
            "core family bound over the declared arms = {}",
            fmt_opt(result.fam_core)
        );
    }
    let mut bias_printed = false;
    for r in &result.rows {
        for (dir, te, adj, state) in [
            ("driver->target", r.te_d2t, r.bias_d2t, r.bias_state_d2t),
            ("target->driver", r.te_t2d, r.bias_t2d, r.bias_state_t2d),
        ] {
            if state == "absent" {
                continue;
            }
            if !bias_printed {
                println!();
                println!(
                    "bias (report site only; raw TE untouched; gate n_eff >= {}):",
                    fmt_opt(TE_NEFF_THRESHOLD)
                );
                bias_printed = true;
            }
            println!(
                "bias | lag {} | {} | TE_raw {} | adjusted {}",
                r.lag,
                dir,
                fmt_opt(te),
                match adj {
                    Some(v) => format!("{v:.4e}"),
                    None => state.to_string(),
                }
            );
        }
    }
    match &result.maxt {
        Some(mt) => {
            println!(
                "studentized max-T null (bucket resample, B = {}): {} finite | (1 - alpha) quantile = {}",
                mt.replicas,
                mt.finite,
                fmt_opt(Some(mt.quantile))
            );
            println!(
                "observed studentized family maximum = {} | clears quantile: {}",
                fmt_opt(mt.observed_max),
                mt.clears
            );
            for (i, &lag) in desc.lags.iter().enumerate() {
                for (dir, mi) in [("driver->target", 2 * i), ("target->driver", 2 * i + 1)] {
                    println!(
                        "max-T {dir} | lag {lag} | n_eff {} | MDE {}",
                        fmt_opt(mt.n_eff.get(mi).copied().flatten()),
                        fmt_opt(mt.mde.get(mi).copied().flatten())
                    );
                }
            }
        }
        None => println!(
            "studentized max-T null: pending — no studentized bucket max-T null was measured (paired n = {} vs cap {MAXT_N_CAP}, B = {}); the phase-surrogate family bound stands",
            result.n_paired, desc.surrogate
        ),
    }
    println!(
        "driver -> target: {} | best lag {} | TE {}",
        result.d2t.word,
        match result.d2t.best_lag {
            Some(l) => l.to_string(),
            None => "absent".to_string(),
        },
        fmt_opt(result.d2t.best_te)
    );
    println!(
        "target -> driver: {} | best lag {} | TE {}",
        result.t2d.word,
        match result.t2d.best_lag {
            Some(l) => l.to_string(),
            None => "absent".to_string(),
        },
        fmt_opt(result.t2d.best_te)
    );
    match result.cte {
        Some((te, thr)) => {
            let word = if te > thr {
                "conditioned arrow"
            } else {
                "conditioned silent"
            };
            println!(
                "cTE(target -> driver | cond) = {te:.4e} | threshold {thr:.4e} | {word} | unadjusted_conditional"
            );
        }
        None => {
            if !desc.conds.is_empty() {
                println!("cTE pending — the conditioning arm carries no aligned series");
            }
        }
    }
}

fn execute(
    desc: &Descriptor,
    sources: &[SourceConfig],
    witnesses: &[WitnessRecord],
    fam_override: Option<f64>,
    anchor: &QueryAnchor,
) -> Option<QueryResult> {
    println!("=== field_te_query — fields by name over the Archivar path, the one TE machine ===");
    if let Some(p) = &desc.pair {
        println!("round: {p}");
    }
    println!(
        "alignment: seasonal {} | lags {:?} | surrogate {} | cadence measured live",
        desc.seasonal.name(),
        desc.lags,
        desc.surrogate
    );

    let cond_arm = if desc.conds.len() == 1 {
        desc.conds.first()
    } else {
        None
    };
    if desc.conds.len() > 1 {
        println!(
            "cTE pending — {} confounders named; the aligned triple carries one arm (the multi-confounder path stays a named pending)",
            desc.conds.len()
        );
    }
    let Some(driver_arm) = desc.driver.as_ref() else {
        println!(
            "driver arm absent — a pair query needs a driver; the matrix form is dispatched separately"
        );
        return None;
    };
    let Some(target_arm) = desc.target.as_ref() else {
        println!(
            "target arm absent — a pair query needs a target; the matrix form is dispatched separately"
        );
        return None;
    };
    let driver_load = load_arm(sources, witnesses, driver_arm, desc.register, anchor);
    let target_load = load_arm(sources, witnesses, target_arm, desc.register, anchor);
    let cond_load = cond_arm.map(|a| load_arm(sources, witnesses, a, desc.register, anchor));
    print_arm("driver", driver_arm, &driver_load);
    print_arm("target", target_arm, &target_load);
    if let (Some(arm), Some(load)) = (cond_arm, &cond_load) {
        print_arm("cond", arm, load);
    }
    if !desc.events.is_empty() || !desc.gates.is_empty() {
        println!();
        println!("pending register (refs, never a silent number):");
        for (r, s) in &desc.events {
            println!("  event {r} {s}");
        }
        for (r, s) in &desc.gates {
            println!("  gate {r} {s}");
        }
    }

    let driver_series = match &driver_load {
        ArmLoad::Ready { series, .. } | ArmLoad::WitnessReady { series, .. } => series,
        ArmLoad::Pending(reason) => {
            println!();
            println!("driver arm pending ({reason}) — the pair stays unmeasured");
            return None;
        }
    };
    let target_series = match &target_load {
        ArmLoad::Ready { series, .. } | ArmLoad::WitnessReady { series, .. } => series,
        ArmLoad::Pending(reason) => {
            println!();
            println!("target arm pending ({reason}) — the pair stays unmeasured");
            return None;
        }
    };
    let cond_series = match &cond_load {
        Some(ArmLoad::Ready { series, .. }) | Some(ArmLoad::WitnessReady { series, .. }) => {
            Some(series.as_slice())
        }
        _ => None,
    };

    let driver_count = match &driver_load {
        ArmLoad::WitnessReady { .. } => true,
        _ => false,
    };
    let target_count = match &target_load {
        ArmLoad::WitnessReady { .. } => true,
        _ => false,
    };
    let cond_count = match &cond_load {
        Some(ArmLoad::WitnessReady { .. }) => true,
        _ => false,
    };
    let aligned = match align(
        driver_series,
        target_series,
        cond_series,
        desc.seasonal,
        driver_count,
        target_count,
        cond_count,
        desc.bin,
    ) {
        Ok(a) => a,
        Err(reason) => {
            println!();
            println!("alignment absent: {reason}");
            return None;
        }
    };
    println!();
    if desc.bin.is_some() {
        println!(
            "aligned grid: cells {} | cadence {} s (declared bin)",
            aligned.grid.len(),
            fmt_opt(Some(aligned.cadence_s))
        );
    } else {
        println!(
            "aligned grid: cells {} | cadence {} s (measured from the slowest arm)",
            aligned.grid.len(),
            fmt_opt(Some(aligned.cadence_s))
        );
    }
    if driver_count {
        print_count_arm("driver", &aligned.driver);
    }
    if target_count {
        print_count_arm("target", &aligned.target);
    }
    if cond_count {
        if let Some(c) = &aligned.cond {
            print_count_arm("cond", c);
        }
    }
    let (target_s, driver_s) = pair2(&aligned.target, &aligned.driver);
    let cond_triple = aligned
        .cond
        .as_ref()
        .map(|c| pair3(&aligned.target, &aligned.driver, c));
    if target_s.len() < TE_FLOOR {
        println!(
            "paired samples n = {} < {TE_FLOOR} — the TE stays unmeasured",
            target_s.len()
        );
        return None;
    }
    let times_s: Vec<f64> = aligned
        .grid
        .iter()
        .zip(aligned.target.iter())
        .zip(aligned.driver.iter())
        .filter_map(|((&t, a), b)| match (a, b) {
            (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some(t),
            _ => None,
        })
        .collect();
    let block = if aligned.cadence_s <= 3600.0 { 24 } else { 1 };
    let cond_in = cond_triple
        .as_ref()
        .map(|(t, d, c)| (t.as_slice(), d.as_slice(), c.as_slice()));
    let result = analyze(
        desc,
        &target_s,
        &driver_s,
        &times_s,
        block,
        cond_in,
        fam_override,
    );
    print_result(&result, desc);
    Some(result)
}

fn is_arrow(word: &str) -> bool {
    word == "arrow"
}

fn run_parity(sources: &[SourceConfig], witnesses: &[WitnessRecord]) -> i32 {
    let desc = Descriptor {
        pair: Some("enso-bz-sst (Blatt I)".into()),
        driver: Some(Arm {
            name: "omni_hro_imf_bz_gsm_nt".into(),
            state: State::Built,
        }),
        target: Some(Arm {
            name: "ersstv5_nino34_ssta".into(),
            state: State::Built,
        }),
        conds: vec![Arm {
            name: "tao_wnd_zonal_m_s".into(),
            state: State::Built,
        }],
        events: Vec::new(),
        gates: Vec::new(),
        seasonal: Seasonal::Climatology,
        lags: (0..=12).collect(),
        surrogate: 100,
        register: Register::Sources,
        bin: None,
        event_conditional: false,
        count_quantiles: None,
        matrix: None,
        derived: Vec::new(),
    };
    let Some(result) = execute(
        &desc,
        sources,
        witnesses,
        Some(REC_FAM),
        &QueryAnchor::empty(),
    ) else {
        println!();
        println!("PARITY: UNMEASURED (an arm or the alignment stays absent)");
        return 0;
    };

    println!();
    println!("=== parity bridge — core vs the recorded ENSO Blatt I verdict ===");
    println!(
        "recorded: fam {REC_FAM:.4e} | Bz->SST {REC_D2T_WORD} best lag {REC_D2T_LAG} TE {REC_D2T_TE:.4e} | SST->Bz {REC_T2D_WORD} best lag {REC_T2D_LAG} TE {REC_T2D_TE:.4e} | cTE {REC_CTE:.4e} < {REC_CTE_THR:.4e}"
    );
    println!(
        "  (docs/blatt/sonne-erde-blatt.md:24; docs/handover/archiv/handover-2026-09-30-river-folge74.md:91-92)"
    );
    let cte_word = match result.cte {
        Some((te, thr)) => {
            if te > thr {
                "conditioned arrow"
            } else {
                "conditioned silent"
            }
        }
        None => "absent",
    };
    println!(
        "core:     fam {} | Bz->SST {} best lag {} TE {} | SST->Bz {} best lag {} TE {} | cTE {}",
        fmt_opt(result.fam),
        result.d2t.word,
        match result.d2t.best_lag {
            Some(l) => l.to_string(),
            None => "absent".to_string(),
        },
        fmt_opt(result.d2t.best_te),
        result.t2d.word,
        match result.t2d.best_lag {
            Some(l) => l.to_string(),
            None => "absent".to_string(),
        },
        fmt_opt(result.t2d.best_te),
        cte_word
    );
    let d2t_ok = is_arrow(&result.d2t.word) == is_arrow(REC_D2T_WORD);
    let t2d_ok = is_arrow(&result.t2d.word) == is_arrow(REC_T2D_WORD);
    let cte_ok = match cte_word {
        "conditioned silent" => true,
        "conditioned arrow" => false,
        _ => false,
    };
    println!(
        "Bz->SST: {} (core {} vs recorded {})",
        if d2t_ok { "GLEICH" } else { "ABWEICHEND" },
        result.d2t.word,
        REC_D2T_WORD
    );
    println!(
        "SST->Bz: {} (core {} vs recorded {})",
        if t2d_ok { "GLEICH" } else { "ABWEICHEND" },
        result.t2d.word,
        REC_T2D_WORD
    );
    println!(
        "cTE:     {} (core {} vs recorded conditioned silent)",
        if cte_ok { "GLEICH" } else { "ABWEICHEND" },
        cte_word
    );
    if result.d2t.best_lag == Some(REC_D2T_LAG) {
        if let Some(te) = result.d2t.best_te {
            println!(
                "Bz->SST lag/TE drift: recorded lag {REC_D2T_LAG} TE {REC_D2T_TE:.4e} vs core lag {REC_D2T_LAG} TE {te:.4e}"
            );
        }
    }
    if result.t2d.best_lag == Some(REC_T2D_LAG) {
        if let Some(te) = result.t2d.best_te {
            println!(
                "SST->Bz lag/TE drift: recorded lag {REC_T2D_LAG} TE {REC_T2D_TE:.4e} vs core lag {REC_T2D_LAG} TE {te:.4e}"
            );
        }
    }
    if d2t_ok && t2d_ok && cte_ok {
        println!("PARITY: GLEICH — the core reproduces the recorded ENSO Blatt I verdict");
        0
    } else {
        println!("PARITY: ABWEICHEND — a probe stays held until the divergence is named");
        1
    }
}

const EVENT_WINDOW_S: f64 = 7.0 * 86_400.0;
const EVENT_BIN_S: f64 = 3_600.0;
const EVENT_FLOOR: usize = 2;
const EVENT_SURROGATES: usize = 100;
const EVENT_GUARD_S: f64 = 6.0 * 3_600.0;

struct EventAverage {
    lag_s: Vec<f64>,
    mean: Vec<Option<f64>>,
    event_count: usize,
}

fn sample_nearest(series: &[(f64, f64)], t: f64) -> Option<f64> {
    if series.is_empty() {
        return None;
    }
    let mut lo = 0usize;
    let mut hi = series.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if series[mid].0 < t {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let before = lo.checked_sub(1).and_then(|i| series.get(i));
    let after = series.get(lo);
    let pick = match (before, after) {
        (Some(b), Some(a)) => {
            if (b.0 - t).abs() <= (a.0 - t).abs() {
                Some(*b)
            } else {
                Some(*a)
            }
        }
        (Some(b), None) => Some(*b),
        (None, Some(a)) => Some(*a),
        (None, None) => None,
    };
    pick.filter(|(_, v)| v.is_finite()).map(|(_, v)| v)
}

fn event_triggered_average(
    driver: &[(f64, f64)],
    events: &[(f64, f64)],
    window_s: f64,
    bin_s: f64,
) -> EventAverage {
    let n_bins = (window_s / bin_s).floor().max(0.0) as usize + 1;
    let lag_s: Vec<f64> = (0..n_bins).map(|k| k as f64 * bin_s).collect();
    let mut sums = vec![0.0f64; n_bins];
    let mut counts = vec![0usize; n_bins];
    let mut event_count = 0usize;
    let (Some(&(d0, _)), Some(&(d1, _))) = (driver.first(), driver.last()) else {
        return EventAverage {
            lag_s,
            mean: vec![None; n_bins],
            event_count: 0,
        };
    };
    for &(te, _) in events {
        if te < d0 + window_s || te > d1 - window_s {
            continue;
        }
        event_count += 1;
        for (k, lag) in lag_s.iter().enumerate() {
            if let Some(v) = sample_nearest(driver, te + lag) {
                sums[k] += v;
                counts[k] += 1;
            }
        }
    }
    let mean = (0..n_bins)
        .map(|k| {
            if counts[k] > 0 {
                Some(sums[k] / counts[k] as f64)
            } else {
                None
            }
        })
        .collect();
    EventAverage {
        lag_s,
        mean,
        event_count,
    }
}

fn eta_peak(avg: &EventAverage) -> Option<f64> {
    let mut best: Option<f64> = None;
    for v in avg.mean.iter().flatten() {
        let a = v.abs();
        best = Some(best.map_or(a, |b| b.max(a)));
    }
    best
}

struct EventNull {
    surrogates: usize,
    mean: f64,
    sd: f64,
    threshold: f64,
}

fn wrap_time(t: f64, d0: f64, span: f64) -> f64 {
    d0 + (t - d0).rem_euclid(span)
}

fn omori_preserving_shift_null(
    driver: &[(f64, f64)],
    events: &[(f64, f64)],
    window_s: f64,
    bin_s: f64,
    surrogates: usize,
    guard_s: f64,
    seed: u64,
) -> Option<EventNull> {
    if events.len() < EVENT_FLOOR || surrogates < 2 || driver.len() < 2 {
        return None;
    }
    let (d0, d1) = (driver.first()?.0, driver.last()?.0);
    let span = d1 - d0;
    if !(span > 2.0 * guard_s) {
        return None;
    }
    let mut state = seed;
    let mut peaks = Vec::with_capacity(surrogates);
    for _ in 0..surrogates {
        let magnitude = guard_s + uniform_unit(&mut state) * (span - 2.0 * guard_s);
        let offset = if uniform_unit(&mut state) < 0.5 {
            magnitude
        } else {
            magnitude - span
        };
        let mut shifted: Vec<(f64, f64)> = driver
            .iter()
            .map(|&(t, v)| (wrap_time(t + offset, d0, span), v))
            .collect();
        shifted.sort_by(|a, b| a.0.total_cmp(&b.0));
        let avg = event_triggered_average(&shifted, events, window_s, bin_s);
        if let Some(p) = eta_peak(&avg) {
            peaks.push(p);
        }
    }
    if peaks.len() < 2 {
        return None;
    }
    let mean = peaks.iter().sum::<f64>() / peaks.len() as f64;
    let var = peaks.iter().map(|p| (p - mean) * (p - mean)).sum::<f64>() / (peaks.len() - 1) as f64;
    let sd = var.sqrt();
    Some(EventNull {
        surrogates: peaks.len(),
        mean,
        sd,
        threshold: mean + 2.0 * sd,
    })
}

const COUNT_PANEL_FLOOR: usize = 20;

fn largest_pow2_bin(n: usize) -> usize {
    let mut q = 1usize;
    while q * 2 <= n && n / (q * 2) >= COUNT_PANEL_FLOOR {
        q *= 2;
    }
    q
}

fn quantile_sorted(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }
    let pos = p.clamp(0.0, 1.0) * (sorted.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    if lo == hi {
        return sorted[lo];
    }
    let frac = pos - lo as f64;
    sorted[lo] * (1.0 - frac) + sorted[hi] * frac
}

fn driver_decorrelation_s(driver: &[(f64, f64)]) -> Option<f64> {
    if driver.len() < 8 {
        return None;
    }
    let mut dts: Vec<f64> = driver
        .windows(2)
        .map(|w| w[1].0 - w[0].0)
        .filter(|d| d.is_finite() && *d > 0.0)
        .collect();
    if dts.len() < 4 {
        return None;
    }
    dts.sort_by(|a, b| a.total_cmp(b));
    let dt = dts[dts.len() / 2];
    if !(dt.is_finite() && dt > 0.0) {
        return None;
    }
    let t0 = driver.first()?.0;
    let t1 = driver.last()?.0;
    let n = ((t1 - t0) / dt).floor() as usize + 1;
    if n < 8 {
        return None;
    }
    let grid: Vec<f64> = (0..n)
        .map(|k| sample_nearest(driver, t0 + k as f64 * dt).unwrap_or(f64::NAN))
        .collect();
    let finite: Vec<f64> = grid.iter().copied().filter(|v| v.is_finite()).collect();
    if finite.len() * 2 < n {
        return None;
    }
    let mean = finite.iter().sum::<f64>() / finite.len() as f64;
    let var = finite.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / finite.len() as f64;
    if !(var > 0.0) {
        return None;
    }
    let threshold = 1.0 / std::f64::consts::E;
    let max_lag = (n / 4).max(2).min(n.saturating_sub(1));
    for lag in 1..=max_lag {
        let mut num = 0.0f64;
        let mut cnt = 0usize;
        for i in 0..(n - lag) {
            let a = grid[i];
            let b = grid[i + lag];
            if a.is_finite() && b.is_finite() {
                num += (a - mean) * (b - mean);
                cnt += 1;
            }
        }
        if cnt == 0 {
            continue;
        }
        let acf = (num / cnt as f64) / var;
        if acf <= threshold {
            return Some(lag as f64 * dt);
        }
    }
    None
}

fn print_count_panel_raw(observed: &[usize], expected: &[f64], q: usize, n: usize) {
    println!(
        "count panel: n = {n} event(s) | q = {q} driver-quantile bins | exposure-weighted expectation"
    );
    for b in 0..q {
        println!(
            "  bin {b}: observed {} | expected {:.2}",
            observed[b], expected[b]
        );
    }
}

fn print_count_panel(
    observed: &[usize],
    expected: &[f64],
    null_mean: &[f64],
    p_bin: &[f64],
    adj: &[bool],
    q: usize,
    n: usize,
) {
    println!(
        "count panel: n = {n} event(s) | q = {q} driver-quantile bins | exposure-weighted expectation"
    );
    for b in 0..q {
        let word = if adj[b] {
            "above the null (BH)"
        } else {
            "consistent with the null"
        };
        println!(
            "  bin {b}: observed {} | expected {:.2} | null mean {:.2} | rank p {:.4} | {word}",
            observed[b], expected[b], null_mean[b], p_bin[b]
        );
    }
}

fn run_count_panel(driver: &[(f64, f64)], events: &[(f64, f64)], q: usize, surrogates: usize) {
    println!("count panel form: quantile bins = {q} | surrogates = {surrogates}");
    let (Some(&(d0, _)), Some(&(d1, _))) = (driver.first(), driver.last()) else {
        println!("count panel pending — the driver carries no span");
        return;
    };
    let span = d1 - d0;
    if !(span > 0.0) {
        println!("count panel pending — the driver span is not positive");
        return;
    }
    let inside: Vec<(f64, f64)> = events
        .iter()
        .copied()
        .filter(|(t, _)| *t >= d0 && *t <= d1)
        .collect();
    let n = inside.len();
    if n < COUNT_PANEL_FLOOR {
        println!(
            "count panel block: n = {n} event(s) inside the driver span < floor {COUNT_PANEL_FLOOR}; a count panel needs >= {COUNT_PANEL_FLOOR} events. Missing data side: a longer event train."
        );
        return;
    }
    if !q.is_power_of_two() || q < 2 {
        println!(
            "count panel block: quantile count {q} is no power of two >= 2 — the bin law stays unnamed"
        );
        return;
    }
    if n / q < COUNT_PANEL_FLOOR {
        println!(
            "count panel block: n = {n} event(s) inside the driver span; floor(n/q) = {} < floor {COUNT_PANEL_FLOOR} for q = {q}; the largest power of two with n/q >= {COUNT_PANEL_FLOOR} is {} — the panel is too thin",
            n / q,
            largest_pow2_bin(n)
        );
        return;
    }
    let mut values: Vec<f64> = driver
        .iter()
        .map(|(_, v)| *v)
        .filter(|v| v.is_finite())
        .collect();
    if values.len() < q {
        println!("count panel pending — the driver carries fewer than {q} finite values");
        return;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let mut edges: Vec<f64> = Vec::with_capacity(q + 1);
    for k in 0..=q {
        edges.push(quantile_sorted(&values, k as f64 / q as f64));
    }
    let bin_of = |v: f64| -> Option<usize> {
        if !v.is_finite() {
            return None;
        }
        let mut b = 0usize;
        while b + 1 < q && v > edges[b + 1] {
            b += 1;
        }
        Some(b)
    };
    let mut dts: Vec<f64> = driver
        .windows(2)
        .map(|w| w[1].0 - w[0].0)
        .filter(|d| d.is_finite() && *d > 0.0)
        .collect();
    if dts.len() < 4 {
        println!("count panel pending — the driver carries fewer than four valid gaps");
        return;
    }
    dts.sort_by(|a, b| a.total_cmp(b));
    let dt = dts[dts.len() / 2];
    let mut occupancy = vec![0usize; q];
    let mut n_valid = 0usize;
    for (_, v) in driver.iter() {
        if v.is_finite() {
            n_valid += 1;
            if let Some(b) = bin_of(*v) {
                occupancy[b] += 1;
            }
        }
    }
    if n_valid < q {
        println!("count panel pending — the driver carries fewer than {q} valid samples");
        return;
    }
    let t_exposure = n_valid as f64 * dt;
    let lambda0 = n as f64 / t_exposure;
    let expected: Vec<f64> = occupancy
        .iter()
        .map(|c| lambda0 * (*c as f64) * dt)
        .collect();
    if expected.iter().any(|e| !(*e > 0.0)) {
        println!(
            "count panel block: at least one quantile bin carries zero exposure — the quantile edges collapse; q = {q} is too fine for the driver's distinct values"
        );
        return;
    }
    let mut observed = vec![0usize; q];
    for &(t, _) in &inside {
        if let Some(v) = sample_nearest(driver, t) {
            if let Some(b) = bin_of(v) {
                observed[b] += 1;
            }
        }
    }
    let chi2 = |counts: &[usize]| -> f64 {
        counts
            .iter()
            .zip(expected.iter())
            .map(|(c, e)| {
                let d = *c as f64 - e;
                d * d / e
            })
            .sum()
    };
    let obs_chi2 = chi2(&observed);
    let guard_s = match driver_decorrelation_s(driver) {
        Some(g) if g > 0.0 => g,
        _ => {
            println!(
                "count panel null pending — the driver decorrelation time is absent (the autocorrelation carries no 1/e crossing); no silent guard"
            );
            print_count_panel_raw(&observed, &expected, q, n);
            return;
        }
    };
    if !(span > 2.0 * guard_s) {
        println!(
            "count panel null pending — the driver span is too short for the measured guard band {guard_s:.1} s"
        );
        print_count_panel_raw(&observed, &expected, q, n);
        return;
    }
    let resolution_needed = (q as f64 / MAXT_ALPHA).ceil() as usize;
    if surrogates + 1 < resolution_needed {
        println!(
            "count panel resolution: B + 1 = {} < q/alpha = {resolution_needed} (q = {q}, alpha = {MAXT_ALPHA}) — the surrogate ensemble cannot resolve a BH-corrected per-bin p; the empirical p stays a lower bound (Phipson & Smyth 2010)",
            surrogates + 1
        );
    }
    let mut state = SURROGATE_SEED;
    let mut null_counts: Vec<Vec<usize>> = vec![Vec::new(); q];
    let mut null_chi2: Vec<f64> = Vec::with_capacity(surrogates);
    for _ in 0..surrogates {
        let magnitude = guard_s + uniform_unit(&mut state) * (span - 2.0 * guard_s);
        let offset = if uniform_unit(&mut state) < 0.5 {
            magnitude
        } else {
            magnitude - span
        };
        let mut shifted: Vec<(f64, f64)> = driver
            .iter()
            .map(|&(t, v)| (wrap_time(t + offset, d0, span), v))
            .collect();
        shifted.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut counts = vec![0usize; q];
        for &(t, _) in &inside {
            if let Some(v) = sample_nearest(&shifted, t) {
                if let Some(b) = bin_of(v) {
                    counts[b] += 1;
                }
            }
        }
        for (b, c) in counts.iter().enumerate() {
            null_counts[b].push(*c);
        }
        null_chi2.push(chi2(&counts));
    }
    if null_chi2.len() < 2 {
        println!("count panel null pending — fewer than two surrogate draws landed");
        print_count_panel_raw(&observed, &expected, q, n);
        return;
    }
    let b_total = null_chi2.len() as f64;
    let ge = null_chi2.iter().filter(|x| **x >= obs_chi2).count();
    let p_global = (1.0 + ge as f64) / (b_total + 1.0);
    let mut null_mean = vec![0.0f64; q];
    let mut p_bin = vec![0.0f64; q];
    for b in 0..q {
        let c = &null_counts[b];
        let mean = c.iter().map(|x| *x as f64).sum::<f64>() / c.len() as f64;
        null_mean[b] = mean;
        let ge_b = c.iter().filter(|x| **x >= observed[b]).count();
        p_bin[b] = (1.0 + ge_b as f64) / (c.len() as f64 + 1.0);
    }
    let adj = benjamini_hochberg_pass(&p_bin, MAXT_ALPHA);
    print_count_panel(&observed, &expected, &null_mean, &p_bin, &adj, q, n);
    println!(
        "count panel global: chi2 = {obs_chi2:.2} | B = {} | empirical p = {p_global:.4} (rank p, (1+#{{>=obs}})/(B+1)) | mean+2sd is screening only",
        null_chi2.len()
    );
    let pass = adj.iter().filter(|a| **a).count();
    let word = if p_global <= MAXT_ALPHA || pass > 0 {
        "above the null"
    } else {
        "consistent with the null"
    };
    println!(
        "count panel verdict: {word} — global empirical p {p_global:.4}; {pass} of {q} quantile bins survive the Benjamini-Hochberg step at alpha {MAXT_ALPHA}; exposure-weighted expectation {:.2} event(s)/panel. The event times stay fixed, only the driver alignment is broken, so the event clustering is part of the null.",
        expected.iter().sum::<f64>()
    );
}

fn run_event_conditional(
    driver: &[(f64, f64)],
    events: &[(f64, f64)],
    bin_s: f64,
    surrogates: usize,
) {
    println!(
        "event-conditional form: window {} s | bin {} s | guard {} s",
        EVENT_WINDOW_S, bin_s, EVENT_GUARD_S
    );
    let avg = event_triggered_average(driver, events, EVENT_WINDOW_S, bin_s);
    println!(
        "event-triggered average: {} event(s) inside the driver span | {} lag cells | observed ETA peak |mean| = {}",
        avg.event_count,
        avg.lag_s.len(),
        fmt_opt(eta_peak(&avg))
    );
    if avg.event_count < EVENT_FLOOR {
        println!(
            "event-conditional block: n = {} event(s) < floor {EVENT_FLOOR}; a single event carries no distribution. Missing data side: >= {EVENT_FLOOR} events (a declustered mainshock set).",
            avg.event_count
        );
        return;
    }
    match omori_preserving_shift_null(
        driver,
        events,
        EVENT_WINDOW_S,
        bin_s,
        surrogates,
        EVENT_GUARD_S,
        SURROGATE_SEED,
    ) {
        Some(null) => {
            let word = match eta_peak(&avg) {
                Some(p) if p > null.threshold => "above the null",
                Some(_) => "consistent with the null",
                None => "absent",
            };
            println!(
                "Omori-preserving circular driver shift null: B = {} | mean = {:.4e} | sd = {:.4e} | threshold (mean + 2 sd) = {:.4e}",
                null.surrogates, null.mean, null.sd, null.threshold
            );
            println!(
                "event-conditional verdict: {word} — the event train's clustering stays fixed, only the driver alignment is broken; the null is Omori-preserving by construction."
            );
        }
        None => println!(
            "event-conditional null pending — the driver span is too short for the guard band or the train carries < {EVENT_FLOOR} events"
        ),
    }
}

fn run_parity_witness(
    name: &str,
    witnesses: &[WitnessRecord],
    sources: &[SourceConfig],
    driver_name: Option<&str>,
    anchor: &QueryAnchor,
) -> i32 {
    println!("=== parity bridge — witness register over phi/witnesses.φ ===");
    let Some(w) = find_witness(witnesses, name) else {
        println!("PARITY-WITNESS: pending — '{name}' stands in no witness block");
        return 0;
    };
    println!(
        "witness {} | kind {} | record {} | force {} | {}",
        w.key,
        w.kind_token,
        w.records.join("+"),
        w.force.as_deref().unwrap_or("absent"),
        w.url
    );
    let arm = Arm {
        name: w.key.clone(),
        state: State::Built,
    };
    let load = load_witness_arm(w, State::Built);
    print_arm("witness", &arm, &load);
    match &load {
        ArmLoad::WitnessReady { series, .. } => {
            println!(
                "event train: {} stamped events, t in TDB from the loaded record",
                series.len()
            );
            if w.kind == Some(WitnessKind::PointEvent) {
                match driver_name {
                    Some(dn) => match load_field_across_sources(sources, dn, anchor) {
                        Ok(Some(driver)) => {
                            run_event_conditional(&driver, series, EVENT_BIN_S, EVENT_SURROGATES)
                        }
                        Ok(None) => println!(
                            "event-conditional query pending — driver '{dn}' stands in no source block"
                        ),
                        Err(reason) => {
                            println!("event-conditional query pending — driver '{dn}' {reason}")
                        }
                    },
                    None => println!(
                        "event-conditional block: the point-event train carries n = {} event(s); the event-triggered average (event_triggered_average) and the Omori-preserving circular driver shift null (omori_preserving_shift_null) are built, but no aligned driver series was named (--driver <field>). Missing data side: a driver series; then the form needs >= {EVENT_FLOOR} events.",
                        series.len()
                    ),
                }
            }
        }
        ArmLoad::Pending(reason) => println!("witness arm stays pending — {reason}"),
        ArmLoad::Ready { .. } => {}
    }
    println!(
        "PARITY-WITNESS: pending — no recorded witness TE verdict in docs/ to compare against (measured 2026-10-03 via sgrep over docs/; the point-event register verdict docs/handover/archiv/handover-2026-10-01-mountain-folge218.md:69 names the record build, no TE parity). The arm is measured, the parity stays named pending, never invented."
    );
    0
}

fn run_descriptor_event_conditional(
    desc: &Descriptor,
    witnesses: &[WitnessRecord],
    sources: &[SourceConfig],
    anchor: &QueryAnchor,
) -> i32 {
    println!("=== event-conditional form — descriptor over witness x source ===");
    let Some(target_arm) = desc.target.as_ref() else {
        println!("event-conditional form pending — the descriptor carries no witness/target arm");
        return 0;
    };
    let witness_name = target_arm.name.as_str();
    let Some(w) = find_witness(witnesses, witness_name) else {
        println!("event-conditional form pending — '{witness_name}' stands in no witness block");
        return 0;
    };
    println!(
        "witness {} | kind {} | record {} | {}",
        w.key,
        w.kind_token,
        w.records.join("+"),
        w.url
    );
    let load = load_witness_arm(w, State::Built);
    match &load {
        ArmLoad::WitnessReady { series, .. } => {
            let Some(driver_arm) = desc.driver.as_ref() else {
                println!("event-conditional form pending — the descriptor carries no driver arm");
                return 0;
            };
            let driver_name = driver_arm.name.as_str();
            let bin_s = desc.bin.unwrap_or(EVENT_BIN_S);
            match load_field_across_sources(sources, driver_name, anchor) {
                Ok(Some(driver)) => {
                    match desc.count_quantiles {
                        Some(q) => run_count_panel(&driver, series, q, desc.surrogate),
                        None => run_event_conditional(&driver, series, bin_s, desc.surrogate),
                    }
                    0
                }
                Ok(None) => {
                    println!(
                        "event-conditional query pending — driver '{driver_name}' stands in no source block"
                    );
                    0
                }
                Err(reason) => {
                    println!("event-conditional query pending — driver '{driver_name}' {reason}");
                    0
                }
            }
        }
        ArmLoad::Pending(reason) => {
            println!("event-conditional witness arm stays pending — {reason}");
            0
        }
        ArmLoad::Ready { .. } => 0,
    }
}

const MATRIX_BINS: usize = 4;

struct MatrixCellOutcome {
    id: String,
    cond_n: usize,
    n: usize,
    n_eff: Option<f64>,
    te: Option<f64>,
    p: f64,
    floor: bool,
    pass: bool,
    res_pair: Option<(f64, f64)>,
    alignment_absent: bool,
}

fn resolution_representable(grid_dt: f64, tau_d: f64, tau_t: f64) -> bool {
    if !(grid_dt.is_finite() && grid_dt > 0.0 && tau_d.is_finite() && tau_t.is_finite()) {
        return false;
    }
    grid_dt + 1.0 >= tau_d.max(tau_t)
}

fn benjamini_yekutieli_pass(p_values: &[f64], level: f64) -> Vec<bool> {
    if p_values.is_empty() || !(level > 0.0 && level <= 1.0) {
        return vec![false; p_values.len()];
    }
    let m = p_values.len() as f64;
    let harmonic: f64 = (1..=p_values.len()).map(|k| 1.0 / k as f64).sum();
    let denom = m * harmonic;
    let mut sorted: Vec<f64> = p_values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let mut cutoff = 0.0f64;
    let mut found = false;
    for (k, &p) in sorted.iter().enumerate() {
        let rank = (k + 1) as f64;
        if p <= rank / denom * level {
            cutoff = p;
            found = true;
        }
    }
    if !found {
        return vec![false; p_values.len()];
    }
    p_values.iter().map(|&p| p <= cutoff).collect()
}

fn align_many(
    arms: &[&[(f64, f64)]],
    seasonal: Seasonal,
    bin_seconds: Option<f64>,
) -> Result<(Vec<Vec<Option<f64>>>, Vec<f64>, f64), String> {
    if arms.is_empty() {
        return Err("matrix carries no arm series".into());
    }
    let monthly = matches!(seasonal, Seasonal::Climatology);
    let (grid, grid_dt) = match bin_seconds {
        Some(step) => {
            if !(step.is_finite() && step > 0.0) {
                return Err(format!("bin {step} is no positive finite width"));
            }
            let mut start = f64::NEG_INFINITY;
            let mut end = f64::INFINITY;
            for a in arms {
                let a0 = a
                    .first()
                    .map(|p| p.0)
                    .ok_or("an arm carries no stamped sample")?;
                let a1 = a
                    .last()
                    .map(|p| p.0)
                    .ok_or("an arm carries no stamped sample")?;
                start = start.max(a0);
                end = end.min(a1);
            }
            if !(end > start) {
                return Err("bin window carries no overlap between the arms".into());
            }
            let n = ((end - start) / step).ceil() as usize;
            if n == 0 {
                return Err("bin window carries no cell".into());
            }
            let grid: Vec<f64> = (0..n).map(|i| start + i as f64 * step).collect();
            (grid, step)
        }
        None => {
            let mut best_idx = 0usize;
            let mut best_dt = f64::NEG_INFINITY;
            for (i, a) in arms.iter().enumerate() {
                if let Some(dt) = median_dt(a) {
                    if dt > best_dt {
                        best_dt = dt;
                        best_idx = i;
                    }
                }
            }
            if !best_dt.is_finite() {
                return Err("no arm cadence was measurable".into());
            }
            let grid: Vec<f64> = arms[best_idx].iter().map(|p| p.0).collect();
            (grid, best_dt)
        }
    };
    if monthly && !(25.0 * 86_400.0..=32.0 * 86_400.0).contains(&grid_dt) {
        return Err(format!(
            "climatology+standardize needs a monthly grid arm (slowest cadence {grid_dt:.0} s); the deseasonalization lives on grid index mod {CAL_MONTHS}"
        ));
    }
    let last_step = if monthly { MONTH_S } else { grid_dt };
    let mut cells: Vec<Vec<Option<f64>>> = arms
        .iter()
        .map(|a| bin_to_grid(a, &grid, last_step))
        .collect();
    if monthly {
        for c in cells.iter_mut() {
            *c = deseasonalize(c);
        }
    }
    Ok((cells, grid, grid_dt))
}

fn joint_columns(cols: &[&[Option<f64>]]) -> Vec<Vec<f32>> {
    let mut out: Vec<Vec<f32>> = (0..cols.len()).map(|_| Vec::new()).collect();
    if cols.is_empty() {
        return out;
    }
    for i in 0..cols[0].len() {
        let mut row_ok = true;
        for c in cols {
            match c.get(i) {
                Some(Some(v)) if v.is_finite() => {}
                _ => {
                    row_ok = false;
                    break;
                }
            }
        }
        if !row_ok {
            continue;
        }
        for (ci, c) in cols.iter().enumerate() {
            if let Some(Some(v)) = c.get(i) {
                out[ci].push(*v as f32);
            }
        }
    }
    out
}

struct CellTe {
    te: f64,
    n_eff: Option<f64>,
    surrogates: Vec<f64>,
}

fn cell_te_and_surrogates(
    target: &[f32],
    driver: &[f32],
    conds: &[LaggedCond],
    lags: &[usize],
    bins: usize,
    n_surr: usize,
    seed: u64,
) -> Option<CellTe> {
    let mut obs_best: Option<(f64, usize)> = None;
    let mut surr_max: Option<Vec<f64>> = None;
    for &lag in lags {
        let te = transfer_entropy_conditional_binned_n(target, driver, conds, lag, bins)?;
        let surr = conditional_te_surrogates_n(
            target,
            driver,
            conds,
            TeSurrogateParams {
                lag,
                max_lag: lag,
                bins,
                seed,
                n_surr,
                null: TeNull::Phase,
                block: 0,
                est: TeEstimator::Binned,
                k: 4,
            },
        )?;
        obs_best = Some(match obs_best {
            Some((best, best_lag)) if best >= te => (best, best_lag),
            _ => (te, lag),
        });
        surr_max = Some(match surr_max {
            Some(prev) => prev.into_iter().zip(surr).map(|(a, b)| a.max(b)).collect(),
            None => surr,
        });
    }
    let (te, lag) = obs_best?;
    Some(CellTe {
        te,
        n_eff: binned_n_eff(target, driver, lag, bins),
        surrogates: surr_max?,
    })
}

fn load_matrix_arm(
    name: &str,
    sources: &[SourceConfig],
    witnesses: &[WitnessRecord],
    anchor: &QueryAnchor,
    derived: &[(String, Vec<String>)],
) -> Result<Vec<(f64, f64)>, String> {
    if let Some((_, carriers)) = derived.iter().find(|(n, _)| n == name) {
        return derive_matrix_arm(name, carriers, sources, witnesses, anchor);
    }
    load_raw_matrix_arm(name, sources, witnesses, anchor)
}

fn load_raw_matrix_arm(
    name: &str,
    sources: &[SourceConfig],
    witnesses: &[WitnessRecord],
    anchor: &QueryAnchor,
) -> Result<Vec<(f64, f64)>, String> {
    match load_field_across_sources(sources, name, anchor) {
        Ok(Some(series)) => return Ok(series),
        Ok(None) => {}
        Err(reason) => return Err(reason),
    }
    if let Some(w) = find_witness(witnesses, name) {
        match load_witness_arm(w, State::Built) {
            ArmLoad::WitnessReady { series, .. } => return Ok(series),
            ArmLoad::Ready { series, .. } => return Ok(series),
            ArmLoad::Pending(reason) => return Err(reason),
        }
    }
    Err(format!(
        "'{name}' stands in no source block and no witness block"
    ))
}

fn derive_matrix_arm(
    derived_name: &str,
    carriers: &[String],
    sources: &[SourceConfig],
    witnesses: &[WitnessRecord],
    anchor: &QueryAnchor,
) -> Result<Vec<(f64, f64)>, String> {
    if derived_name != "newell_dphi_dt" && derived_name != "newell_dphi_dt_omni" {
        return Err(format!(
            "derived node '{derived_name}' names no built derivation"
        ));
    }
    if carriers.len() != 3 {
        return Err(format!(
            "derived node '{derived_name}' carries {} carrier fields — newell_dphi_dt carries exactly three (by, bz, speed)",
            carriers.len()
        ));
    }
    let mut series: Vec<Vec<(f64, f64)>> = Vec::with_capacity(carriers.len());
    for name in carriers {
        series.push(load_raw_matrix_arm(name, sources, witnesses, anchor)?);
    }
    let refs: Vec<&[(f64, f64)]> = series.iter().map(|s| s.as_slice()).collect();
    let (cells, grid, _) = align_many(&refs, Seasonal::None, None)?;
    let values = newell_dphi_dt(&cells[0], &cells[1], &cells[2]);
    Ok(grid
        .iter()
        .zip(values.iter())
        .filter_map(|(t, v)| v.map(|val| (*t, val)))
        .collect())
}

fn matrix_cell_bias(
    te: f64,
    n: usize,
    cond_n: usize,
    n_eff: Option<f64>,
) -> (Option<f64>, &'static str) {
    if cond_n > 0 {
        return (None, "unadjusted_conditional");
    }
    bias_column(Some(te), n, n_eff, BiasArm::BinnedHistogram)
}

fn run_pair_matrix(
    desc: &Descriptor,
    sources: &[SourceConfig],
    witnesses: &[WitnessRecord],
    anchor: &QueryAnchor,
) -> i32 {
    let Some(spec) = &desc.matrix else {
        println!("matrix form: the descriptor carries no matrix spec");
        return 0;
    };
    println!(
        "=== pair matrix — {} [{}] ===",
        spec.label,
        spec.shape.name()
    );
    let cells = spec.cells();
    let pool = spec.pool();
    let dropped = spec.dropped_cells();
    let cond_word = match spec.cond {
        MatrixCond::Rest => "rest",
        MatrixCond::Uncond => "none",
    };
    let fdr_word = match spec.fdr.0 {
        FdrMethod::Bh => "bh",
        FdrMethod::By => "by",
    };
    println!(
        "cells {} | dropped (driver==target) {} | pool {} | cond {} | fdr {} q {} over {} | bins {} | surrogate {} | lags {:?}",
        cells.len(),
        dropped,
        pool.len(),
        cond_word,
        fdr_word,
        spec.fdr.1,
        spec.fdr.2.name(),
        MATRIX_BINS,
        desc.surrogate,
        desc.lags
    );
    if let Some(n) = spec.expect_cells {
        println!(
            "expect cells {n} matched the declared {} cells",
            spec.declared_cells()
        );
    }
    if cells.is_empty() {
        println!("matrix pending — the declared arms carry no cell");
        return 0;
    }

    let loaded: Vec<Option<Vec<(f64, f64)>>> = pool
        .iter()
        .map(
            |name| match load_matrix_arm(name, sources, witnesses, anchor, &desc.derived) {
                Ok(series) => Some(series),
                Err(reason) => {
                    println!("arm '{name}' stays pending — {reason}");
                    None
                }
            },
        )
        .collect();
    let pos: Vec<bool> = loaded.iter().map(|l| l.is_some()).collect();
    let native: Vec<Option<f64>> = pool
        .iter()
        .enumerate()
        .map(|(i, _)| match loaded.get(i).and_then(|l| l.as_ref()) {
            Some(s) => median_dt(s),
            None => None,
        })
        .collect();
    println!(
        "arms measured {} of {} | per-cell alignment, every cell carries its own (tau_d x tau_t)",
        loaded.iter().filter(|l| l.is_some()).count(),
        pool.len()
    );
    println!();

    let idx_of = |name: &str| pool.iter().position(|p| p == name);
    let mut outcomes: Vec<MatrixCellOutcome> = Vec::with_capacity(cells.len());
    for (d, t) in &cells {
        let id = format!("{d}->{t}");
        let Some(di) = idx_of(d) else {
            outcomes.push(MatrixCellOutcome {
                id,
                cond_n: 0,
                n: 0,
                n_eff: None,
                te: None,
                p: 1.0,
                floor: true,
                res_pair: None,
                alignment_absent: false,
                pass: false,
            });
            continue;
        };
        let Some(ti) = idx_of(t) else {
            outcomes.push(MatrixCellOutcome {
                id,
                cond_n: 0,
                n: 0,
                n_eff: None,
                te: None,
                p: 1.0,
                floor: true,
                res_pair: None,
                alignment_absent: false,
                pass: false,
            });
            continue;
        };
        let cond_idx: Vec<usize> = spec
            .cond_names(d, t)
            .iter()
            .filter_map(|n| idx_of(n))
            .collect();
        let cond_n = cond_idx.len();
        let required: Vec<usize> = std::iter::once(di)
            .chain(std::iter::once(ti))
            .chain(cond_idx.iter().copied())
            .collect();
        if required.iter().any(|&k| !pos[k]) {
            outcomes.push(MatrixCellOutcome {
                id,
                cond_n,
                n: 0,
                n_eff: None,
                te: None,
                p: 1.0,
                floor: true,
                res_pair: None,
                alignment_absent: false,
                pass: false,
            });
            continue;
        }
        let req_arms: Vec<&[(f64, f64)]> = required
            .iter()
            .map(|&k| {
                loaded[k]
                    .as_ref()
                    .expect("the required arm was measured")
                    .as_slice()
            })
            .collect();
        let (cell_columns, _, grid_dt) = match align_many(&req_arms, desc.seasonal, desc.bin) {
            Ok(a) => a,
            Err(_reason) => {
                outcomes.push(MatrixCellOutcome {
                    id,
                    cond_n,
                    n: 0,
                    n_eff: None,
                    te: None,
                    p: 1.0,
                    floor: true,
                    res_pair: None,
                    alignment_absent: true,
                    pass: false,
                });
                continue;
            }
        };
        let tau_d = native.get(di).copied().flatten();
        let tau_t = native.get(ti).copied().flatten();
        if let (Some(td), Some(tt)) = (tau_d, tau_t) {
            if !resolution_representable(grid_dt, td, tt) {
                outcomes.push(MatrixCellOutcome {
                    id,
                    cond_n,
                    n: 0,
                    n_eff: None,
                    te: None,
                    p: 1.0,
                    floor: true,
                    res_pair: Some((td, tt)),
                    alignment_absent: false,
                    pass: false,
                });
                continue;
            }
        }
        let cols: Vec<&[Option<f64>]> = cell_columns.iter().map(|c| c.as_slice()).collect();
        let joint = joint_columns(&cols);
        let Some(n) = joint.first().map(|c| c.len()) else {
            outcomes.push(MatrixCellOutcome {
                id,
                cond_n,
                n: 0,
                n_eff: None,
                te: None,
                p: 1.0,
                floor: true,
                res_pair: None,
                alignment_absent: false,
                pass: false,
            });
            continue;
        };
        if n < TE_FLOOR {
            outcomes.push(MatrixCellOutcome {
                id,
                cond_n,
                n,
                n_eff: None,
                te: None,
                p: 1.0,
                floor: true,
                res_pair: None,
                alignment_absent: false,
                pass: false,
            });
            continue;
        }
        let driver = &joint[0];
        let target = &joint[1];
        let conds: Vec<LaggedCond> = joint[2..]
            .iter()
            .map(|s| LaggedCond {
                series: s.as_slice(),
                lag: 0,
            })
            .collect();
        match cell_te_and_surrogates(
            target,
            driver,
            &conds,
            &desc.lags,
            MATRIX_BINS,
            desc.surrogate,
            SURROGATE_SEED,
        ) {
            Some(cell) => {
                let p = surrogate_rank_p_value(cell.te, &cell.surrogates).unwrap_or(1.0);
                outcomes.push(MatrixCellOutcome {
                    id,
                    cond_n,
                    n,
                    n_eff: cell.n_eff,
                    te: Some(cell.te),
                    p,
                    floor: false,
                    res_pair: None,
                    alignment_absent: false,
                    pass: false,
                });
            }
            None => {
                outcomes.push(MatrixCellOutcome {
                    id,
                    cond_n,
                    n,
                    n_eff: None,
                    te: None,
                    p: 1.0,
                    floor: true,
                    res_pair: None,
                    alignment_absent: false,
                    pass: false,
                });
            }
        }
    }

    let mut group_keys: Vec<String> = Vec::new();
    let mut group_members: Vec<Vec<usize>> = Vec::new();
    for (i, (d, t)) in cells.iter().enumerate() {
        let key = match spec.fdr.2 {
            FdrScope::Row => d.clone(),
            FdrScope::Col => t.clone(),
            FdrScope::Matrix => String::new(),
        };
        if spec.fdr.2 == FdrScope::Matrix {
            if group_members.is_empty() {
                group_keys.push(String::new());
                group_members.push(Vec::new());
            }
            group_members[0].push(i);
            continue;
        }
        match group_keys.iter().position(|k| k == &key) {
            Some(g) => group_members[g].push(i),
            None => {
                group_keys.push(key);
                group_members.push(vec![i]);
            }
        }
    }
    for members in &group_members {
        let pvals: Vec<f64> = members.iter().map(|&i| outcomes[i].p).collect();
        let passes = match spec.fdr.0 {
            FdrMethod::Bh => benjamini_hochberg_pass(&pvals, spec.fdr.1),
            FdrMethod::By => benjamini_yekutieli_pass(&pvals, spec.fdr.1),
        };
        for (mi, &i) in members.iter().enumerate() {
            if let (Some(pass), Some(outcome)) = (passes.get(mi), outcomes.get_mut(i)) {
                outcome.pass = *pass;
            }
        }
    }

    println!(
        "{:<28} | {:>4} | {:>5} | {:>13} | {:>9} | {:>12} | {:>12} | {:<22} | {:>8} | {}",
        "cell",
        "cond",
        "n",
        "res(tau_d x tau_t)",
        "n_eff",
        "TE",
        "TE_bias",
        "bias_state",
        "p",
        "verdict"
    );
    for o in &outcomes {
        let te = match o.te {
            Some(v) => format!("{v:.4e}"),
            None => "absent".to_string(),
        };
        let (bias, bias_state) = match o.te {
            Some(v) if !o.floor => matrix_cell_bias(v, o.n, o.cond_n, o.n_eff),
            _ => (None, "absent"),
        };
        let bias_word = match bias {
            Some(v) => format!("{v:.4e}"),
            None => "absent".to_string(),
        };
        let word = if o.alignment_absent {
            "alignment pending"
        } else if o.res_pair.is_some() {
            "resolution pending"
        } else if o.floor {
            "floor (p = 1)"
        } else if o.pass {
            "arrow (fdr pass)"
        } else {
            "silent"
        };
        let res = match o.res_pair {
            Some((a, b)) => format!("{a:.0}x{b:.0}"),
            None => "-".to_string(),
        };
        println!(
            "{:<28} | {:>4} | {:>5} | {:>13} | {:>9} | {:>12} | {:>12} | {:<22} | {:>8} | {}",
            o.id,
            o.cond_n,
            o.n,
            res,
            fmt_opt(o.n_eff),
            te,
            bias_word,
            bias_state,
            format!("{:.4}", o.p),
            word
        );
    }
    let passed = outcomes.iter().filter(|o| o.pass).count();
    println!(
        "fdr {} q {} over {}: {} of {} cells pass",
        fdr_word,
        spec.fdr.1,
        spec.fdr.2.name(),
        passed,
        outcomes.len()
    );
    0
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    println!(
        "grammar: pair <label> | form event-conditional | count quantile <q> | driver|target <field> [built|pending|probe] | cond <field> [built|pending|probe] (repeatable: a confounder list) | witness <name> [built|pending|probe] | register sources|witnesses | event|gate <ref> pending|probe | cadence live | seasonal none|climatology+standardize | lags <list> | surrogate <n> | bin <seconds> | anchor --lat <deg> --lon <deg> | --station <name> [--station-lat <deg> --station-lon <deg>] | modes --direction <witness> | --spectral <witness>[,<witness>...] | --parity-witness <witness> [--driver <field>] | from <derived> <carrier,carrier,...> | matrix <label> rect|full|upper | drivers|targets|channels <a,b,...> | cond rest|none | fdr bh|by <q> over matrix|row|col | expect cells <n>"
    );
    let sources = load_sources();
    let witnesses = load_witnesses();
    let anchor = match anchor_from_args(&args) {
        Ok(a) => a,
        Err(reason) => {
            eprintln!("{reason}");
            exit(2);
        }
    };
    if let Some(name) = arg_after(&args, "--direction") {
        exit(run_direction_query(name, &witnesses));
    }
    if let Some(name) = arg_after(&args, "--spectral") {
        exit(run_spectral_query(name, &witnesses));
    }
    if let Some(name) = arg_after(&args, "--parity-witness") {
        exit(run_parity_witness(
            name,
            &witnesses,
            &sources,
            arg_after(&args, "--driver"),
            &anchor,
        ));
    }
    if sources.is_empty() {
        eprintln!("phi/sources.φ carries no block — the register stays unread");
        exit(2);
    }

    if args.iter().any(|a| a == "--parity") {
        exit(run_parity(&sources, &witnesses));
    }

    let desc = if let Some(path) = arg_after(&args, "--descriptor") {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("--descriptor {path}: {e}");
                exit(2);
            }
        };
        match parse_descriptor(&text) {
            Ok(d) => d,
            Err(reason) => {
                eprintln!("{reason}");
                exit(2);
            }
        }
    } else {
        match descriptor_from_args(&args) {
            Ok(d) => d,
            Err(reason) => {
                eprintln!("{reason}");
                exit(2);
            }
        }
    };

    if desc.matrix.is_some() {
        exit(run_pair_matrix(&desc, &sources, &witnesses, &anchor));
    }

    if desc.event_conditional {
        exit(run_descriptor_event_conditional(
            &desc, &witnesses, &sources, &anchor,
        ));
    }

    match execute(&desc, &sources, &witnesses, None, &anchor) {
        Some(_) => {
            println!("Silent lines are findings. Exit 0.");
        }
        None => {
            println!("The query stays unmeasured; pending arms are named above. Exit 0.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field_cfg(name: &str) -> FieldConfig {
        FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            kernel: 0,
            force: 0,
            tau: 0.0,
            absorption: 0.0,
            advection: 0.0,
            unit: String::new(),
            freq: 0.0,
            bin_width: 0.0,
            fold: None,
            aperture: omegaflow::archivar::Aperture::None,
        }
    }

    fn source_cfg(field: FieldConfig, station: Option<&str>) -> SourceConfig {
        SourceConfig {
            ttl: 3600,
            url: "https://example.com/x".into(),
            origin: None,
            terms: None,
            frame: omegaflow::archivar::Frame::Manifest,
            format: "text".into(),
            extracts: vec![Extract::Field(field)],
            headers: vec![],
            post_body: None,
            target: None,
            catalog: None,
            range: None,
            max_freq: None,
            min_freq: None,
            body: None,
            stations_url: None,
            stations_path: String::new(),
            stations_lat: String::new(),
            stations_lon: String::new(),
            stations_id: String::new(),
            hapi_fill: std::collections::HashMap::new(),
            flux_from_mag: None,
            abs_mag_from: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            stations_flatten: String::new(),
            stations_filter: None,
            fanout_delay: 0,
            sha256: None,
            window: None,
            live_only: false,
            station_code: station.map(|c| c.to_string()),
            cgm_lat: None,
            cgm_source: None,
        }
    }

    #[test]
    fn field_sources_resolves_station_qualified_channel_to_its_block() {
        let aae = source_cfg(field_cfg("intermagnet_xyz_x_nt"), Some("AAE"));
        let abg = source_cfg(field_cfg("intermagnet_xyz_x_nt"), Some("ABG"));

        let qualified = field_sources(std::slice::from_ref(&aae), "intermagnet_xyz_x_nt_aae");
        assert_eq!(
            qualified.len(),
            1,
            "the station channel names exactly its own block"
        );
        assert_eq!(
            qualified[0].0.station_code.as_deref(),
            Some("AAE"),
            "the resolved block carries the station of the request"
        );

        let bare = field_sources(std::slice::from_ref(&aae), "intermagnet_xyz_x_nt");
        assert_eq!(
            bare.len(),
            1,
            "the bare field name still finds the same block"
        );
        assert_eq!(bare[0].0.station_code.as_deref(), Some("AAE"));

        let both = [aae, abg];
        let abg_only = field_sources(&both, "intermagnet_xyz_x_nt_abg");
        assert_eq!(
            abg_only.len(),
            1,
            "the ABG channel addresses only the ABG block"
        );
        assert_eq!(abg_only[0].0.station_code.as_deref(), Some("ABG"));
        assert!(
            field_sources(&both, "intermagnet_xyz_x_aae").is_empty(),
            "an unqualified/unknown channel resolves nothing, never the first hit"
        );
    }

    #[test]
    fn matrix_resolution_gate_flags_the_coarser_pair_never_bins_silently() {
        assert!(resolution_representable(86_400.0, 3_600.0, 3_600.0));
        assert!(resolution_representable(86_400.0, 86_400.0, 3_600.0));
        assert!(!resolution_representable(86_400.0, 2_592_000.0, 6.0));
        assert!(!resolution_representable(86_400.0, 6.0, 2_592_000.0));
        assert!(!resolution_representable(86_400.0, f64::NAN, 6.0));
        assert!(!resolution_representable(0.0, 6.0, 6.0));
    }

    #[test]
    fn matrix_cell_alignment_is_per_pair_not_global() {
        let coarse = vec![(0.0, 1.0), (2_592_000.0, 2.0), (5_184_000.0, 3.0)];
        let fast = vec![(0.0, 1.0), (86_400.0, 2.0), (172_800.0, 3.0)];
        let far = vec![
            (1.0e9, 1.0),
            (1.0e9 + 86_400.0, 2.0),
            (1.0e9 + 172_800.0, 3.0),
        ];
        let all = vec![coarse.as_slice(), fast.as_slice(), far.as_slice()];
        assert!(
            align_many(&all, Seasonal::None, Some(86_400.0)).is_err(),
            "a global alignment over all three arms carries no common window"
        );
        let pair = vec![coarse.as_slice(), fast.as_slice()];
        assert!(
            align_many(&pair, Seasonal::None, Some(86_400.0)).is_ok(),
            "the overlapping pair aligns on its own, per cell, never gated by a far arm"
        );
    }

    #[test]
    fn bias_column_gates_on_n_eff_and_exact_n() {
        let te = 5.0e-1;
        let (adj, state) = bias_column(Some(te), 800, Some(1.8485e1), BiasArm::ScalarKde);
        assert_eq!(state, "adjusted");
        let adj = adj.expect("a measured n at the floor carries the measured bias");
        assert!(
            (adj - (te + 8.866e-2)).abs() < 1e-12,
            "the negative small-sample bias is added back: adjusted = te - m_k, m_k = -8.866e-2"
        );

        let (adj, state) = bias_column(Some(te), 800, Some(1.0), BiasArm::ScalarKde);
        assert_eq!(state, "unadjusted_below_floor");
        assert!(
            adj.is_none(),
            "below the n_eff floor nothing is manufactured"
        );

        let (adj, state) = bias_column(Some(te), 8547, Some(44.5), BiasArm::ScalarKde);
        assert_eq!(state, "off_table");
        assert!(
            adj.is_none(),
            "an unmeasured n is off-table, never interpolated"
        );

        let (adj, state) = bias_column(None, 800, Some(44.5), BiasArm::ScalarKde);
        assert_eq!(state, "absent");
        assert!(adj.is_none());

        let (adj, state) = bias_column(Some(te), 800, Some(44.5), BiasArm::BinnedHistogram);
        assert_eq!(
            state, "adjusted",
            "the binned arm now carries its own measured table"
        );
        let adj = adj.expect("the binned table entry applies at the operating size");
        assert!(
            (adj - (te - 2.715e-3)).abs() < 1e-12,
            "m_k is the measured binned entry, never the scalar socket"
        );

        let (adj, state) = bias_column(Some(te), 5000, Some(44.5), BiasArm::BinnedHistogram);
        assert_eq!(
            state, "off_table",
            "an unmeasured n is off-table, never interpolated"
        );
        assert!(adj.is_none());

        let (adj, state) = bias_column(Some(te), 800, Some(1.9e1), BiasArm::BinnedHistogram);
        assert_eq!(
            state, "unadjusted_below_floor",
            "the histogram arm reads its own floor, never the KDE socket"
        );
        assert!(adj.is_none());
        let (adj, state) = bias_column(Some(te), 800, Some(1.9e1), BiasArm::ScalarKde);
        assert_eq!(
            state, "adjusted",
            "the same n_eff clears the KDE floor — the two arms are not interchangeable"
        );
        assert!(adj.is_some());
    }

    #[test]
    fn bias_table_only_on_unconditional_cells() {
        let te = 5.0e-1;
        let (adj, state) = matrix_cell_bias(te, 800, 0, Some(44.5));
        assert_eq!(
            state, "adjusted",
            "the unconditional cell reaches its own measured binned table"
        );
        let adj = adj.expect("the binned entry applies above the floor");
        assert!(
            (adj - (te - 2.715e-3)).abs() < 1e-12,
            "no scalar socket, the binned measured entry applies"
        );

        let (adj, state) = matrix_cell_bias(te, 800, 2, Some(44.5));
        assert_eq!(
            state, "unadjusted_conditional",
            "a conditional cell is another estimator and never reaches the unconditional table"
        );
        assert!(adj.is_none());

        let (adj, state) = matrix_cell_bias(te, 800, 0, Some(1.0));
        assert_eq!(state, "unadjusted_below_floor");
        assert!(
            adj.is_none(),
            "below the n_eff floor nothing is manufactured"
        );

        let (adj, state) = matrix_cell_bias(te, 8546, 0, Some(44.5));
        assert_eq!(
            state, "adjusted",
            "8546 is a measured binned size since the binned probe landed"
        );
        assert!(adj.is_some());

        let (adj, state) = matrix_cell_bias(te, 5000, 0, Some(44.5));
        assert_eq!(
            state, "off_table",
            "an unmeasured n is off-table, never interpolated"
        );
        assert!(adj.is_none());
    }

    #[test]
    fn matrix_cell_measures_the_arrow_of_its_target_driver_order() {
        let n = 400usize;
        let mut state = SURROGATE_SEED;
        let mut gauss = || loop {
            let u1 = uniform_unit(&mut state) * 2.0 - 1.0;
            let u2 = uniform_unit(&mut state) * 2.0 - 1.0;
            let s = u1 * u1 + u2 * u2;
            if s > 0.0 && s < 1.0 {
                break (u1 * (-2.0 * s.ln() / s).sqrt()) as f32;
            }
        };
        let mut driver = vec![0f32; n];
        for t in 1..n {
            driver[t] = 0.7 * driver[t - 1] + 0.1 * gauss();
        }
        let mut target = vec![0f32; n];
        for t in 1..n {
            target[t] = 0.5 * target[t - 1] + 0.6 * driver[t - 1] + 0.1 * gauss();
        }
        let fwd =
            cell_te_and_surrogates(&target, &driver, &[], &[1], MATRIX_BINS, 20, SURROGATE_SEED)
                .expect("the coupled forward cell is measurable");
        let rev =
            cell_te_and_surrogates(&driver, &target, &[], &[1], MATRIX_BINS, 20, SURROGATE_SEED)
                .expect("the reverse cell is measurable");
        assert!(
            fwd.te > rev.te,
            "the cell measures the arrow of its target/driver order, got forward {} reverse {}",
            fwd.te,
            rev.te
        );
        assert!(
            fwd.n_eff.is_some(),
            "the winning window carries its KDE n_eff"
        );
    }

    #[test]
    fn maxt_wiring_carries_a_finite_quantile_on_a_synthetic_month() {
        let n = 480usize;
        let epoch = 1_000_000_000.0f64;
        let times: Vec<f64> = (0..n).map(|i| epoch + i as f64 * MONTH_S).collect();
        let driver: Vec<f32> = (0..n).map(|i| (i as f32 * 0.31).sin()).collect();
        let target: Vec<f32> = (0..n)
            .map(|i| 0.5 * (i as f32 * 0.31).sin() + (i as f32 * 0.7).cos())
            .collect();
        let members = vec![
            Member::new("d2t", 0, 1, target.clone(), driver.clone()),
            Member::new("t2d", 1, 1, driver.clone(), target.clone()),
        ];
        let observed = observed_family(&members);
        let maxt = compute_max_t(&members, &observed, &times, 1, 20)
            .expect("the synthetic family carries a max-T null");
        assert!(
            maxt.quantile.is_finite(),
            "the studentized max-T quantile must be finite"
        );
        assert_eq!(maxt.n_eff.len(), 2, "n_eff is carried per member");
        assert!(
            maxt.mde.iter().all(|m| m.is_some()),
            "the MDE row carries a value per statistic with a finite sigma"
        );
    }

    #[test]
    fn direction_statistics_measure_a_clustered_sample() {
        let clustered: Vec<f64> = (0..10).map(|i| 0.05 + i as f64 * 1.0e-4).collect();
        let z = rayleigh_z(&clustered).expect("a finite clustered sample carries a Rayleigh z");
        assert!(z > 9.0, "a concentrated sample carries a large Rayleigh z");
        assert!(
            rayleigh_p(z, clustered.len()).is_finite(),
            "the Rayleigh p-value is finite"
        );
        let uniform: Vec<f64> = (0..8).map(|i| i as f64 * TAU / 8.0).collect();
        let z_uniform = rayleigh_z(&uniform).expect("the uniform sample carries a z");
        assert!(
            z_uniform < 1.0e-6,
            "an evenly spaced circle carries a vanishing Rayleigh z"
        );
        let v = kuiper_v(&[0.5]).expect("a single cell carries a Kuiper V");
        assert!(
            (v - 1.0).abs() < 1.0e-12,
            "one cell at the midpoint gives V = 1"
        );
        assert!(kuiper_p(v, 1).is_finite(), "the Kuiper p-value is finite");
        let (mu, kappa) =
            von_mises_fit(&clustered).expect("the clustered sample carries a von Mises fit");
        assert!(
            mu.is_finite() && kappa > 0.0,
            "the fit carries mu and a positive kappa"
        );
    }

    #[test]
    fn direction_parsers_read_json_and_columns() {
        let json = r#"[{"ra":10.5,"dec":-20.0},{"ra":30,"dec":41}]"#;
        let pairs = parse_object_pairs(json);
        assert_eq!(pairs.len(), 2, "two JSON objects carry two direction pairs");
        assert!((pairs[0].0 - 10.5).abs() < 1.0e-12);
        assert!((pairs[0].1 + 20.0).abs() < 1.0e-12);
        let csv = "ra,dec\n10,20\n30,40\n";
        let cols = parse_column_pairs(csv);
        assert_eq!(cols.len(), 2, "the named columns carry two direction pairs");
        assert!((cols[1].1 - 40.0).abs() < 1.0e-12);
    }

    #[test]
    fn event_triggered_average_counts_and_fills_the_window() {
        let driver: Vec<(f64, f64)> = (0..240)
            .map(|i| (i as f64 * 3_600.0, (i as f64 * 0.1).sin()))
            .collect();
        let events = [(100.0 * 3_600.0, 1.0)];
        let avg = event_triggered_average(&driver, &events, 86_400.0, 3_600.0);
        assert_eq!(
            avg.lag_s.len(),
            25,
            "a one-day window at one-hour bins has 25 cells"
        );
        assert_eq!(avg.event_count, 1, "the covered event is counted");
        assert!(
            avg.mean.iter().all(|m| m.is_some()),
            "a fully covered window fills every cell"
        );
    }

    #[test]
    fn omori_shift_null_stays_absent_below_the_event_floor() {
        let driver: Vec<(f64, f64)> = (0..240).map(|i| (i as f64 * 3_600.0, i as f64)).collect();
        let single = [(100.0 * 3_600.0, 1.0)];
        assert!(
            omori_preserving_shift_null(&driver, &single, 86_400.0, 3_600.0, 8, 3_600.0, 7,)
                .is_none(),
            "one event carries no shift-null distribution"
        );
        let paired = [(100.0 * 3_600.0, 1.0), (150.0 * 3_600.0, 1.0)];
        let null = omori_preserving_shift_null(&driver, &paired, 86_400.0, 3_600.0, 8, 3_600.0, 7)
            .expect("two covered events carry a shift-null distribution");
        assert!(
            null.threshold.is_finite() && null.surrogates >= 2,
            "the Omori-preserving null carries a measured threshold"
        );
    }

    #[test]
    fn url_template_guard_refuses_an_unresolved_slot() {
        let err =
            guard_url_template_resolved("https://api.open-meteo.com/v1/forecast?latitude={lat}")
                .expect_err("a template url stays unresolved");
        assert!(err.contains("{lat}"), "the refusal names the slot: {err}");
        assert!(
            err.contains("unresolved template slot"),
            "the refusal names the cause: {err}"
        );
    }

    #[test]
    fn url_template_guard_passes_a_resolved_url() {
        assert!(
            guard_url_template_resolved("https://example.org/data?start=2026-01-01").is_ok(),
            "a url without a template slot passes the guard"
        );
    }

    #[test]
    fn first_url_template_slot_names_the_first_slot() {
        assert_eq!(
            first_url_template_slot("https://x.example/a?start={week_ago}&stop={now}"),
            Some("week_ago"),
            "the first slot is named, not the last"
        );
        assert_eq!(
            first_url_template_slot("https://x.example/data"),
            None,
            "a url without a slot carries none"
        );
    }

    #[test]
    fn time_markers_resolve_a_temporal_only_template() {
        let resolved = resolve_time_markers(
            "https://cdaweb.gsfc.nasa.gov/hapi/data?id=OMNI2_H0_MRG1HR&time.min={week_ago}T00:00:00Z&time.max={now}Z&format=json",
        );
        assert!(
            !resolved.contains('{'),
            "every clock slot is filled, none left: {resolved}"
        );
        assert!(
            resolved.contains("time.min=20") && resolved.contains("time.max=20"),
            "the epoch markers carry a date: {resolved}"
        );
        assert!(
            guard_url_template_resolved(&resolved).is_ok(),
            "a resolved temporal url passes the guard"
        );
    }

    #[test]
    fn time_markers_leave_a_coordinate_slot_for_the_guard() {
        let resolved = resolve_time_markers(
            "https://earthquake.usgs.gov/fdsnws/event/1/query?starttime={hour_ago}&latitude={lat}&longitude={lon}",
        );
        assert!(
            !resolved.contains("{hour_ago}"),
            "the clock slot is filled: {resolved}"
        );
        assert!(
            resolved.contains("{lat}") && resolved.contains("{lon}"),
            "the coordinate slots survive — no anchor in this query: {resolved}"
        );
        let err = guard_url_template_resolved(&resolved)
            .expect_err("a coordinate template stays unresolved");
        assert!(err.contains("{lat}"), "the refusal names the slot: {err}");
    }

    #[test]
    fn coordinate_slots_fill_from_a_latitude_longitude_anchor() {
        let anchor = QueryAnchor {
            lat: Some(48.5),
            lon: Some(9.25),
            station: None,
        };
        let resolved = resolve_query_slots(
            "https://earthquake.usgs.gov/fdsnws/event/1/query?latitude={lat}&longitude={lon}",
            &anchor,
        )
        .expect("a set lat/lon anchor fills both coordinate slots");
        assert_eq!(
            resolved,
            "https://earthquake.usgs.gov/fdsnws/event/1/query?latitude=48.5&longitude=9.25",
            "the values land in the slots"
        );
        assert!(
            guard_url_template_resolved(&resolved).is_ok(),
            "the resolved url passes the template guard"
        );
    }

    #[test]
    fn missing_coordinate_anchor_refuses_the_slot_by_name() {
        let anchor = QueryAnchor::empty();
        let err = resolve_query_slots(
            "https://earthquake.usgs.gov/fdsnws/event/1/query?latitude={lat}&longitude={lon}",
            &anchor,
        )
        .expect_err("a missing anchor stays unresolved");
        assert!(
            err.contains("{lat}"),
            "the refusal names the missing slot: {err}"
        );
        assert!(
            err.contains("never a default"),
            "the refusal carries no fabricated value: {err}"
        );
    }

    #[test]
    fn station_name_and_coordinates_fill_their_own_slots() {
        let anchor = QueryAnchor {
            lat: Some(-33.87),
            lon: Some(151.21),
            station: Some("SYDNEY".to_string()),
        };
        let resolved = resolve_query_slots(
            "https://x.example/data?station={station}&latitude={lat}&longitude={lon}",
            &anchor,
        )
        .expect("a station bundle fills the station and coordinate slots");
        assert_eq!(
            resolved, "https://x.example/data?station=SYDNEY&latitude=-33.87&longitude=151.21",
            "the station name and its coordinates land in their slots"
        );
    }

    #[test]
    fn anchor_from_args_reads_direct_and_station_coordinates() {
        let args: Vec<String> = [
            "--lat",
            "10.5",
            "--station-lon",
            "-20.25",
            "--station",
            "P1",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let anchor = anchor_from_args(&args).expect("direct and station coordinates parse");
        assert_eq!(anchor.lat, Some(10.5), "the direct latitude is carried");
        assert_eq!(anchor.lon, Some(-20.25), "the station longitude is carried");
        assert_eq!(anchor.station.as_deref(), Some("P1"));
    }

    #[test]
    fn anchor_from_args_refuses_two_differing_coordinates() {
        let args: Vec<String> = ["--lat", "10.5", "--station-lat", "11.0"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let err = anchor_from_args(&args)
            .err()
            .expect("two coordinates name a riss");
        assert!(
            err.contains("--lat") && err.contains("--station-lat"),
            "the refusal names both coordinates: {err}"
        );
    }

    #[test]
    fn anchor_from_args_refuses_an_out_of_range_coordinate() {
        let args: Vec<String> = ["--lat", "120.0"].iter().map(|s| s.to_string()).collect();
        let err = anchor_from_args(&args)
            .err()
            .expect("a latitude above 90 is no coordinate");
        assert!(
            err.contains("outside") && err.contains("120"),
            "the refusal names the range: {err}"
        );
    }

    #[test]
    fn conditional_descriptor_carries_a_confounder_list() {
        let text = "\
pair corona-conditional
register sources
driver aia_304_dn
target aia_131_dn
cond goes_xrsb_flux
cond aia_171_dn probe
cadence live
bin 24
seasonal none
";
        let desc =
            parse_descriptor(text).expect("a conditional descriptor with two confounders parses");
        assert_eq!(desc.conds.len(), 2, "both confounders are carried");
        assert_eq!(desc.conds[0].name, "goes_xrsb_flux");
        assert_eq!(desc.conds[1].state, State::Probe);
    }

    #[test]
    fn conditional_descriptor_refuses_an_unknown_confounder_state() {
        let text = "\
driver aia_304_dn
target aia_131_dn
cond aia_171_dn maybe
cadence live
";
        let err = parse_descriptor(text)
            .err()
            .expect("an unknown confounder state is refused");
        assert!(
            err.contains("aia_171_dn") || err.contains("maybe"),
            "the refusal names the confounder: {err}"
        );
    }

    #[test]
    fn event_conditional_descriptor_carries_witness_and_driver() {
        let text = "\
pair erbq-event
form event-conditional
witness point-event#1
driver omni_hro_imf_bz_gsm_nt
cadence live
bin 3600
surrogate 100
";
        let desc = parse_descriptor(text).expect("an event-conditional descriptor parses");
        assert!(desc.event_conditional, "the form flag is carried");
        assert_eq!(
            desc.target.as_ref().map(|a| a.name.as_str()),
            Some("point-event#1"),
            "the witness is the target"
        );
        assert_eq!(
            desc.driver.as_ref().map(|a| a.name.as_str()),
            Some("omni_hro_imf_bz_gsm_nt"),
            "the field is the driver"
        );
        assert_eq!(desc.register, Register::Witnesses);
        assert_eq!(desc.bin, Some(3600.0));
        assert_eq!(desc.surrogate, 100);
    }

    #[test]
    fn event_conditional_descriptor_refuses_a_missing_driver() {
        let text = "\
form event-conditional
witness point-event#1
cadence live
";
        let err = parse_descriptor(text)
            .err()
            .expect("a missing driver arm is refused");
        assert!(
            err.contains("event-conditional") && err.contains("driver"),
            "the refusal names the missing arm: {err}"
        );
    }

    #[test]
    fn event_conditional_descriptor_refuses_a_missing_witness() {
        let text = "\
form event-conditional
driver omni_hro_imf_bz_gsm_nt
cadence live
";
        let err = parse_descriptor(text)
            .err()
            .expect("a missing witness arm is refused");
        assert!(
            err.contains("event-conditional") && err.contains("witness"),
            "the refusal names the missing arm: {err}"
        );
    }

    #[test]
    fn event_conditional_form_must_precede_its_arms() {
        let text = "\
witness point-event#1
form event-conditional
driver omni_hro_imf_bz_gsm_nt
cadence live
";
        let err = parse_descriptor(text)
            .err()
            .expect("a late form line is refused");
        assert!(
            err.contains("event-conditional") && err.contains("before"),
            "the refusal names the ordering: {err}"
        );
    }

    #[test]
    fn unknown_descriptor_form_is_refused() {
        let text = "\
form something-else
driver a
target b
cadence live
";
        let err = parse_descriptor(text)
            .err()
            .expect("an unknown form is refused");
        assert!(
            err.contains("something-else"),
            "the refusal names the form: {err}"
        );
    }

    #[test]
    fn count_quantile_panel_parses_in_event_conditional_form() {
        let text = "\
form event-conditional
witness point-event#1
driver omni_hro_imf_bz_gsm_nt
count quantile 8
cadence live
surrogate 100
";
        let desc = parse_descriptor(text).expect("a count panel parses");
        assert_eq!(desc.count_quantiles, Some(8));
        assert!(desc.event_conditional);
    }

    #[test]
    fn count_quantile_refuses_a_non_power_of_two() {
        let text = "\
form event-conditional
witness point-event#1
driver omni_hro_imf_bz_gsm_nt
count quantile 7
cadence live
";
        let err = parse_descriptor(text)
            .err()
            .expect("a non-power-of-two bin count is refused");
        assert!(
            err.contains("power of two"),
            "the refusal names the bin law: {err}"
        );
    }

    #[test]
    fn count_quantile_without_the_form_is_refused() {
        let text = "\
witness point-event#1
count quantile 8
cadence live
";
        let err = parse_descriptor(text)
            .err()
            .expect("a count panel without the form is refused");
        assert!(
            err.contains("event-conditional"),
            "the refusal names the form: {err}"
        );
    }

    #[test]
    fn largest_pow2_bin_follows_the_floor() {
        assert_eq!(largest_pow2_bin(198), 8, "198/8 = 24 >= 20, 198/16 < 20");
        assert_eq!(largest_pow2_bin(100), 4, "100/4 = 25 >= 20, 100/8 < 20");
        assert_eq!(largest_pow2_bin(20), 1, "20/2 = 10 < 20");
    }

    #[test]
    fn quantile_sorted_interpolates() {
        let v = [0.0, 1.0, 2.0, 3.0];
        assert!((quantile_sorted(&v, 0.0) - 0.0).abs() < 1e-12);
        assert!((quantile_sorted(&v, 1.0) - 3.0).abs() < 1e-12);
        assert!((quantile_sorted(&v, 0.5) - 1.5).abs() < 1e-12);
    }

    #[test]
    fn driver_decorrelation_reads_a_decaying_series() {
        let driver: Vec<(f64, f64)> = (0..64).map(|k| (k as f64, 0.9f64.powi(k))).collect();
        let guard = driver_decorrelation_s(&driver).expect("an AR(1)-like series crosses 1/e");
        assert!(guard > 0.0, "the guard is a positive time");
    }

    #[test]
    fn spectral_epoch_names_split_a_paired_call() {
        assert_eq!(
            spectral_epoch_names("substance#0,substance#1"),
            vec!["substance#0", "substance#1"],
            "a comma-separated pair carries two epochs"
        );
        assert_eq!(
            spectral_epoch_names("rixs"),
            vec!["rixs"],
            "a single name stays a single epoch"
        );
        assert!(
            spectral_epoch_names(" , ").is_empty(),
            "blank entries carry no epoch"
        );
    }

    #[test]
    fn spectral_axis_value_parser_skips_a_header_and_junk() {
        let text =
            "Eloss weight err\n-0.8951 19.6285 1.5233\n-0.2432 93.5101 3.0285\nnot a number here\n";
        let pairs = parse_axis_value_pairs(text);
        assert_eq!(pairs.len(), 2, "only the two numeric rows carry a pair");
        assert!((pairs[0].0 + 0.8951).abs() < 1.0e-12);
        assert!((pairs[1].1 - 93.5101).abs() < 1.0e-12);
    }

    #[test]
    fn spectral_epochs_align_on_the_shared_axis() {
        let epoch_a = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)];
        let epoch_b = vec![(0.0, 10.0), (1.0, 20.0), (2.0, 30.0), (3.0, 40.0)];
        let aligned = align_spectral_epochs(&epoch_a, &epoch_b, 0.5);
        assert_eq!(aligned.len(), 3, "the shared axis carries three points");
        assert_eq!(
            aligned[1],
            (1.0, 2.0, 20.0),
            "point one of the axis carries both epochs side by side"
        );
        let none = align_spectral_epochs(&epoch_a, &[(10.0, 1.0)], 0.5);
        assert!(none.is_empty(), "a disjoint axis carries no aligned point");
    }

    #[test]
    fn spectral_epoch_comparison_never_averages() {
        let aligned = vec![(0.0, 1.0, 3.0), (1.0, 2.0, 2.0), (2.0, 3.0, 1.0)];
        let cmp = compare_spectral_epochs(&aligned).expect("three aligned points carry a verdict");
        assert_eq!(cmp.n, 3);
        assert!(
            (cmp.mean_abs_delta - 4.0 / 3.0).abs() < 1.0e-12,
            "the magnitude is the mean of the per-axis deltas, not a mean over epochs"
        );
        assert!(
            (cmp.max_abs_delta - 2.0).abs() < 1.0e-12,
            "the largest per-axis delta is carried"
        );
        assert!(
            (cmp.sign_agreement - 1.0 / 3.0).abs() < 1.0e-12,
            "only the midpoint keeps the sign"
        );
        let r = cmp.correlation.expect("both epochs carry variance");
        assert!(
            (r + 1.0).abs() < 1.0e-12,
            "the anti-correlated epoch pair carries r = -1: {r}"
        );
        assert!(
            compare_spectral_epochs(&aligned[..1]).is_none(),
            "one aligned point carries no comparison"
        );
    }

    fn matrix_rect_text() -> &'static str {
        "matrix m rect\n\
         drivers a,b,c\n\
         targets b,c,d\n\
         cond rest\n\
         fdr bh 0.05 over matrix\n\
         lags 1,2\n\
         surrogate 20\n\
         cadence live\n"
    }

    #[test]
    fn matrix_rect_parses_with_drivers_and_targets() {
        let desc = parse_descriptor(matrix_rect_text()).expect("a rect matrix parses");
        let spec = desc.matrix.expect("the matrix spec is carried");
        assert_eq!(spec.shape, MatrixShape::Rect);
        assert_eq!(
            spec.declared_cells(),
            7,
            "targets x drivers drops the d==t cells"
        );
        assert_eq!(
            spec.dropped_cells(),
            2,
            "the two shared names are dropped and counted"
        );
        assert_eq!(
            spec.pool().len(),
            4,
            "the rect pool is drivers union targets"
        );
        assert_eq!(spec.fdr.0, FdrMethod::Bh);
        assert_eq!(spec.fdr.1, 0.05);
        assert_eq!(spec.fdr.2, FdrScope::Matrix);
    }

    #[test]
    fn matrix_full_parses_with_channels() {
        let text = "matrix m full\n\
                    channels a,b,c\n\
                    cond none\n\
                    fdr by 0.1 over col\n\
                    lags 1\n\
                    surrogate 20\n\
                    cadence live\n";
        let desc = parse_descriptor(text).expect("a full matrix parses");
        let spec = desc.matrix.expect("the matrix spec is carried");
        assert_eq!(spec.shape, MatrixShape::Full);
        assert_eq!(spec.declared_cells(), 6, "full ordered N(N-1) cells");
        assert_eq!(spec.fdr.0, FdrMethod::By);
        assert_eq!(spec.fdr.2, FdrScope::Col);
    }

    #[test]
    fn matrix_upper_parses_with_channels() {
        let text = "matrix m upper\n\
                    channels a,b,c,d\n\
                    cond rest\n\
                    fdr bh 0.05 over row\n\
                    lags 3\n\
                    surrogate 30\n\
                    cadence live\n";
        let desc = parse_descriptor(text).expect("an upper matrix parses");
        let spec = desc.matrix.expect("the matrix spec is carried");
        assert_eq!(spec.shape, MatrixShape::Upper);
        assert_eq!(spec.declared_cells(), 6, "upper i<j carries N(N-1)/2 cells");
        assert_eq!(spec.fdr.2, FdrScope::Row);
    }

    #[test]
    fn matrix_cond_rest_removes_driver_and_target_from_the_pool() {
        let desc = parse_descriptor(matrix_rect_text()).expect("a rect matrix parses");
        let spec = desc.matrix.expect("the matrix spec is carried");
        let mut cond = spec.cond_names("a", "b");
        cond.sort();
        assert_eq!(
            cond,
            vec!["c".to_string(), "d".to_string()],
            "cond rest is the pool minus the driver and the target"
        );
        assert!(
            spec.cond_names("b", "c")
                .iter()
                .all(|n| n == "a" || n == "d"),
            "the target and the driver are both excluded"
        );
    }

    #[test]
    fn matrix_refuses_a_fixed_cond_arm() {
        let text = "matrix m rect\n\
                    drivers a,b\n\
                    targets c,d\n\
                    cond xrsb\n\
                    fdr bh 0.05 over matrix\n\
                    lags 1\n\
                    surrogate 20\n\
                    cadence live\n";
        let err = parse_descriptor(text)
            .err()
            .expect("a fixed cond is refused in the matrix");
        assert!(
            err.contains("xrsb") && err.contains("matrix"),
            "the refusal names the fixed confounder: {err}"
        );
    }

    #[test]
    fn matrix_requires_an_fdr_arm() {
        let text = "matrix m rect\n\
                    drivers a,b\n\
                    targets c,d\n\
                    cond rest\n\
                    lags 1\n\
                    surrogate 20\n\
                    cadence live\n";
        let err = parse_descriptor(text)
            .err()
            .expect("a matrix without fdr is refused");
        assert!(
            err.contains("fdr"),
            "the refusal names the missing fdr: {err}"
        );
    }

    #[test]
    fn matrix_expect_cells_mismatch_names_both_counts() {
        let text = "matrix m rect\n\
                    drivers a,b,c\n\
                    targets b,c,d\n\
                    cond rest\n\
                    fdr bh 0.05 over matrix\n\
                    expect cells 5\n\
                    lags 1\n\
                    surrogate 20\n\
                    cadence live\n";
        let err = parse_descriptor(text)
            .err()
            .expect("a mismatching expect cells is refused");
        assert!(
            err.contains('5') && err.contains('7'),
            "the refusal names both counts: {err}"
        );
    }

    #[test]
    fn matrix_requires_lags_and_surrogate() {
        let no_lags = "matrix m rect\n\
                       drivers a,b\n\
                       targets c,d\n\
                       cond rest\n\
                       fdr bh 0.05 over matrix\n\
                       surrogate 20\n\
                       cadence live\n";
        let err = parse_descriptor(no_lags)
            .err()
            .expect("a matrix without lags is refused");
        assert!(
            err.contains("lags"),
            "the refusal names the missing lags: {err}"
        );

        let no_surr = "matrix m rect\n\
                       drivers a,b\n\
                       targets c,d\n\
                       cond rest\n\
                       fdr bh 0.05 over matrix\n\
                       lags 1\n\
                       cadence live\n";
        let err = parse_descriptor(no_surr)
            .err()
            .expect("a matrix without surrogate is refused");
        assert!(
            err.contains("surrogate"),
            "the refusal names the missing surrogate: {err}"
        );
    }

    #[test]
    fn matrix_refuses_a_duplicate_arm() {
        let text = "matrix m full\n\
                    channels a,a,b\n\
                    cond none\n\
                    fdr bh 0.05 over matrix\n\
                    lags 1\n\
                    surrogate 20\n\
                    cadence live\n";
        let err = parse_descriptor(text)
            .err()
            .expect("a duplicate arm is refused");
        assert!(
            err.contains('a') && err.contains("twice"),
            "the refusal names the duplicate: {err}"
        );
    }

    #[test]
    fn cond_rest_without_matrix_is_refused() {
        let text = "driver a\n\
                    target b\n\
                    cond rest\n\
                    cadence live\n";
        let err = parse_descriptor(text)
            .err()
            .expect("cond rest without a matrix is refused");
        assert!(
            err.contains("rest") || err.contains("matrix"),
            "the refusal names the orphan conditioning: {err}"
        );
    }

    #[test]
    fn matrix_rect_refuses_channels() {
        let text = "matrix m rect\n\
                    drivers a,b\n\
                    targets c,d\n\
                    channels e,f\n\
                    cond rest\n\
                    fdr bh 0.05 over matrix\n\
                    lags 1\n\
                    surrogate 20\n\
                    cadence live\n";
        let err = parse_descriptor(text)
            .err()
            .expect("rect with channels is refused");
        assert!(err.contains("channels"), "the refusal names the arm: {err}");
    }

    #[test]
    fn benjamini_yekutieli_is_stricter_than_benjamini_hochberg() {
        let p = vec![0.001, 0.01, 0.02, 0.04, 0.5];
        let bh = benjamini_hochberg_pass(&p, 0.05);
        let by = benjamini_yekutieli_pass(&p, 0.05);
        assert!(
            by.iter().filter(|x| **x).count() <= bh.iter().filter(|x| **x).count(),
            "the Yekutieli correction is never weaker than BH"
        );
    }

    #[test]
    fn matrix_from_derived_node_is_carried() {
        let text = "matrix m rect\n\
                    drivers a,newell_dphi_dt\n\
                    targets b\n\
                    from newell_dphi_dt magnetosphere_imf_by_nt,magnetosphere_imf_bz_nt,solar_wind_speed_km_s\n\
                    cond none\n\
                    fdr bh 0.05 over matrix\n\
                    lags 1\n\
                    surrogate 20\n\
                    cadence live\n";
        let desc = parse_descriptor(text).expect("a descriptor with a from line parses");
        assert_eq!(
            desc.derived,
            vec![(
                "newell_dphi_dt".to_string(),
                vec![
                    "magnetosphere_imf_by_nt".to_string(),
                    "magnetosphere_imf_bz_nt".to_string(),
                    "solar_wind_speed_km_s".to_string(),
                ],
            )]
        );
    }

    #[test]
    fn matrix_from_derived_node_absent_from_pool_is_refused() {
        let text = "matrix m rect\n\
                    drivers a,b\n\
                    targets c,d\n\
                    from newell_dphi_dt x,y,z\n\
                    cond none\n\
                    fdr bh 0.05 over matrix\n\
                    lags 1\n\
                    surrogate 20\n\
                    cadence live\n";
        let err = parse_descriptor(text)
            .err()
            .expect("a derived node absent from the pool is refused");
        assert!(
            err.contains("newell_dphi_dt") && err.contains("pool"),
            "the refusal names the missing derived node: {err}"
        );
    }

    #[test]
    fn from_without_matrix_head_is_refused() {
        let text = "driver a\n\
                    target b\n\
                    from newell_dphi_dt x,y,z\n\
                    cadence live\n";
        let err = parse_descriptor(text)
            .err()
            .expect("from without a matrix head is refused");
        assert!(
            err.contains("matrix"),
            "the refusal names the missing matrix head: {err}"
        );
    }
}
