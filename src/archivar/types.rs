use super::*;

pub trait Radiator: Send + Sync {
    fn accept(&mut self, field: Arc<Buffer>);
}

pub const Φ: f64 = std::f64::consts::GOLDEN_RATIO;

#[derive(Clone)]
pub enum Motion {
    Surface {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
    },
    Barycenter {
        body_name: String,
        scale: f64,
    },
    Linear {
        p: [f64; 3],
        v: [f64; 3],
    },
    Kepler {
        rec: Arc<AsteroidRec>,
    },
    Spherical {
        rec: Arc<StarRec>,
    },
}

#[derive(Clone)]
pub enum SampleSource {
    Source(u32),
    Sensor,
    Ephemeris,
}

#[derive(Clone)]
pub struct Sample {
    pub source: SampleSource,
    pub epoch: f64,
    pub ttl: f64,
    pub extent: f64,
    pub tau: f64,
    pub kernel_id: f64,
    pub force_type: f64,
    pub absorption: f64,
    pub advection: f64,
    pub anchor_vmax: f64,
    pub anchor_amax: f64,
    pub anchor_p0: [f64; 3],
    pub motion: Motion,
    pub val: f64,
    pub name: String,
    pub z_flux: f64,
    pub freq: f64,
    pub bin_width: f64,
    pub color_index: f64,
    pub phase: Option<f64>,
}

#[derive(Clone, Debug)]
pub enum Position {
    Source,
    Surface {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
    },
    SurfaceFlow {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
        speed: f64,
        track: f64,
        vrate: Option<f64>,
    },
    StateVector {
        p: [f64; 3],
        v: [f64; 3],
        track: bool,
    },
    Barycenter {
        body_name: String,
        scale: f64,
    },
    Electrode {
        system: String,
        units: String,
        x: f64,
        y: f64,
        z: f64,
    },
}

#[derive(Clone, Debug)]
pub struct StationThread {
    pub body_name: String,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
}

impl StationThread {
    pub fn motion(&self) -> Motion {
        Motion::Surface {
            body_name: self.body_name.clone(),
            lat: self.lat,
            lon: self.lon,
            alt: self.alt,
        }
    }

    pub fn icrs_at(&self, tdb: f64, eph: &HashMap<String, BodyEphemeris>) -> Option<[f64; 3]> {
        self.motion().at(tdb, tdb, eph)
    }
}

#[derive(Clone)]
pub struct DeclaredBody {
    pub body_name: String,
    pub lat: f64,
    pub lon: f64,
    pub alt: Option<f64>,
    pub receiver_aperture: ReceiverAperture,
}

#[derive(Clone)]
pub struct Channel {
    pub name: String,
    pub value: f64,
    pub position: Position,
    pub epoch: f64,
    pub z: f64,
    pub freq: f64,
    pub bin_width: f64,
    pub station_code: Option<String>,
}

#[derive(Clone)]
pub enum Extract {
    Field(FieldConfig),
    First(FieldConfig, Option<(String, String)>),
    Last(FieldConfig, Option<(String, String)>),
    Count(FieldConfig),
    LastRow(FieldConfig),
    LastObj(String, String, String, String),
    LastLine(String),
    ObjLast(FieldConfig),
    GeojsonEvents {
        mag_key: String,
        min_mag: f64,
        outputs: Vec<String>,
        tau: f64,
        absorption: f64,
        advection: f64,
        mag_type_key: String,
    },
    QuakeMlEvents {
        outputs: Vec<String>,
        tau: f64,
        absorption: f64,
        advection: f64,
    },
    Path(FieldConfig),
    Deep(FieldConfig),
    Regex(FieldConfig),

    Map {
        arr_path: String,
        lat_key: String,
        lon_key: String,
        alt_key: String,
        epoch_key: String,
        val_key: String,
        alt_scale: f64,
        vel_key: String,
        vel_scale: f64,
        trk_key: String,
        vr_key: String,
        fields: Vec<FieldConfig>,
        lat_sign: Option<String>,
        lon_sign: Option<String>,
        epoch_scale: f64,
        tau_key: String,
        mag_type_key: String,
    },
    CelestialMap {
        arr_path: String,
        ra_key: String,
        dec_key: String,
        dist_key: String,
        dist_scale: Option<f64>,
        plx_key: String,
        z_key: String,
        pmra_key: String,
        pmdec_key: String,
        rv_key: String,
        rv_scale: Option<f64>,
        epoch_key: String,
        epoch_mjd: bool,
        fields: Vec<FieldConfig>,
        tau_key: String,
    },
    ProfileMap {
        arr_path: String,
        lat_key: String,
        lon_key: String,
        epoch_key: String,
        pressure_var: String,
        pressure_scale: f64,
        fields: Vec<FieldConfig>,
    },
    EpnCore {
        arr_path: String,
        body_key: String,
        lon_min_key: String,
        lon_max_key: String,
        lat_min_key: String,
        lat_max_key: String,
        alt_min_key: String,
        alt_max_key: String,
        s_region_key: String,
        epoch_key: String,
        epoch_mjd: bool,
        val_key: String,
        fields: Vec<FieldConfig>,
    },
    Volume {
        value_key: String,
        lat_key: String,
        lon_key: String,
        depth_key: String,
        depth_scale: f64,
        name: String,
        frame_body: Option<String>,
    },
    Rows {
        last_line: bool,
        lat_key: String,
        lon_key: String,
        fields: Vec<FieldConfig>,
        tau_key: String,
        epoch_cols: Vec<String>,
        gates: Vec<(String, f64, f64)>,
        bin_s: u64,
        name_prefix: String,
    },
    Flatten {
        arr_path: String,
        geom_path: String,
        epoch_key: String,
        fields: Vec<FieldConfig>,
    },
    CmrPolygon {
        arr_path: String,
        fields: Vec<FieldConfig>,
        epoch_key: String,
        alt_key: String,
        val_key: String,
    },
    CelestialPolygon {
        arr_path: String,
        radius: f64,
        fields: Vec<FieldConfig>,
        epoch_key: String,
        val_key: String,
    },
    KeplerMap {
        arr_path: String,
        a_key: String,
        e_key: String,
        i_key: String,
        om_key: String,
        w_key: String,
        ma_key: String,
        epoch_key: String,
        q_key: String,
        tp_key: String,
        fields: Vec<FieldConfig>,
    },
    Hapi(Vec<(String, String)>),
    Alerce(String),
    XmlCount(String, String),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Aperture {
    None,
    Flux,
}

impl Aperture {
    pub fn of_class(class: &str) -> Option<Self> {
        match class {
            "none" => Some(Aperture::None),
            "flux" => Some(Aperture::Flux),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ReceiverAperture {
    slots: [Option<f64>; 18],
}

impl ReceiverAperture {
    pub fn new() -> Self {
        Self { slots: [None; 18] }
    }

    fn index(force: u8, class: Aperture) -> Option<usize> {
        if (force as usize) >= 9 {
            return None;
        }
        let c = match class {
            Aperture::None => 0usize,
            Aperture::Flux => 1usize,
        };
        Some(force as usize * 2 + c)
    }

    pub fn get(&self, force: u8, class: Aperture) -> Option<f64> {
        Self::index(force, class).and_then(|i| self.slots[i])
    }

    pub fn set(&mut self, force: u8, class: Aperture, length: f64) {
        if let Some(i) = Self::index(force, class) {
            self.slots[i] = Some(length);
        }
    }

    pub fn resolve(&self, force: u8, class: Aperture) -> Option<f64> {
        self.get(force, class)
    }

    pub fn parse_declaration(spec: &str) -> Self {
        let mut aperture = Self::new();
        for entry in spec.split(',') {
            let mut parts = entry.split(':');
            let Some(force_name) = parts.next() else {
                continue;
            };
            let Some(class_name) = parts.next() else {
                continue;
            };
            let Some(length_spec) = parts.next() else {
                continue;
            };
            let Some(force) = force_id_of(force_name) else {
                continue;
            };
            let Some(class) = Aperture::of_class(class_name) else {
                continue;
            };
            let Ok(length) = length_spec.parse::<f64>() else {
                continue;
            };
            if length.is_finite() && length > 0.0 {
                aperture.set(force, class, length);
            }
        }
        aperture
    }
}

impl Default for ReceiverAperture {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct FieldConfig {
    pub key: String,
    pub name: String,
    pub band_id: Option<String>,
    pub kernel: u8,
    pub force: u8,
    pub tau: f64,
    pub absorption: f64,
    pub advection: f64,
    pub unit: String,
    pub freq: f64,
    pub bin_width: f64,
    pub fold: Option<(u8, String)>,
    pub aperture: Aperture,
}

pub const SLOT_ABSENT: f64 = -1.0;

pub const PRESENCE_FLAG_PHASE: f64 = 1.0;
pub const PRESENCE_FLAG_ABSORPTION: f64 = 2.0;
pub const PRESENCE_FLAG_ADVECTION: f64 = 4.0;
pub const PRESENCE_FLAG_FLUX: f64 = 8.0;
pub const PRESENCE_FLAG_QUANTITY: f64 = 16.0;
pub const FORCE_TYPE_QUANTITY: u8 = 255;

impl FieldConfig {
    pub fn absorption_measured(&self) -> Option<f64> {
        if self.absorption == SLOT_ABSENT {
            None
        } else {
            Some(self.absorption)
        }
    }

    pub fn advection_measured(&self) -> Option<f64> {
        if self.advection == SLOT_ABSENT {
            None
        } else {
            Some(self.advection)
        }
    }
}

pub fn slot_measured(value: f64) -> bool {
    value.is_finite() && value != SLOT_ABSENT
}

pub fn slot_or_pad(value: f64) -> f64 {
    if slot_measured(value) { value } else { 0.0 }
}

pub fn presence_flags(phase: Option<f64>, absorption: f64, advection: f64, z_flux: f64) -> f64 {
    let mut flags = 0.0;
    if phase.is_some() {
        flags += PRESENCE_FLAG_PHASE;
    }
    if slot_measured(absorption) {
        flags += PRESENCE_FLAG_ABSORPTION;
    }
    if slot_measured(advection) {
        flags += PRESENCE_FLAG_ADVECTION;
    }
    if slot_measured(z_flux) {
        flags += PRESENCE_FLAG_FLUX;
    }
    flags
}

pub struct BrowserSensor {
    pub key: String,
    pub force: u8,
    pub kernel: u8,
    pub ttl: f64,
    pub tau: Option<f64>,
    pub unit: String,
}

#[derive(Clone)]
pub enum Frame {
    Surface {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
    },
    Barycenter {
        body_name: String,
        scale: f64,
    },
    Manifest,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeAxis {
    pub start_m: f64,
    pub step_m: f64,
}

#[derive(Clone)]
pub struct SourceConfig {
    pub ttl: u64,
    pub url: String,
    pub origin: Option<String>,
    pub terms: Option<String>,
    pub rights_identifier: Option<String>,
    pub rights_scheme: Option<String>,
    pub rights_uri: Option<String>,
    pub frame: Frame,
    pub format: String,
    pub extracts: Vec<Extract>,
    pub headers: Vec<(String, String)>,
    pub post_body: Option<String>,
    pub target: Option<String>,
    pub catalog: Option<String>,
    pub range: Option<RangeAxis>,
    pub max_freq: Option<f64>,
    pub min_freq: Option<f64>,
    pub body: Option<String>,
    pub stations_url: Option<String>,
    pub stations_path: String,
    pub stations_lat: String,
    pub stations_lon: String,
    pub stations_id: String,
    pub hapi_fill: HashMap<String, f64>,
    pub flux_from_mag: Option<String>,
    pub abs_mag_from: Option<String>,
    pub catalog_epoch: Option<f64>,
    pub repeat_ra_bins: u32,
    pub fanout_cap: u32,
    pub stations_flatten: String,
    pub stations_filter: Option<(String, String)>,
    pub fanout_delay: u64,
    pub sha256: Option<String>,
    pub window: Option<(f64, f64)>,
    pub live_only: bool,
    pub station_code: Option<String>,
    pub cgm_lat: Option<f64>,
    pub cgm_source: Option<String>,
    pub geomag_lat: Option<f64>,
}

pub const J2000_EPOCH: f64 = 2451545.0;

pub const PARSEC_M: f64 = 3.085677581e16;

pub const NO_CADENCE: u64 = 1 << 25;

pub const C_LIGHT: f64 = 299792458.0;

pub const HUBBLE_H0: f64 = 70000.0 / (PARSEC_M * 1.0e6);

pub const MAS_YR_TO_RAD_S: f64 = 4.84813681109536e-9 / 31557600.0;

pub const GAUSS_K: f64 = 0.01720209895;

pub type SampleRecord = (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
);

pub fn frame_body_name(frame: &Frame) -> String {
    match frame {
        Frame::Surface { body_name, .. } | Frame::Barycenter { body_name, .. } => body_name.clone(),
        Frame::Manifest => String::new(),
    }
}
