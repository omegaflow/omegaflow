use super::*;

pub trait Radiator: Send + Sync {
    fn accept(&mut self, field: Arc<Buffer>);
}

pub const Φ: f64 = 1.618033988749895;

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
    pub z: f64,
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
}

#[derive(Clone)]
pub struct DeclaredBody {
    pub body_name: String,
    pub lat: f64,
    pub lon: f64,
    pub alt: Option<f64>,
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
        dist_scale: f64,
        plx_key: String,
        z_key: String,
        pmra_key: String,
        pmdec_key: String,
        rv_key: String,
        rv_scale: f64,
        epoch_key: String,
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
    Rows {
        last_line: bool,
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

#[derive(Clone)]
pub struct FieldConfig {
    pub key: String,
    pub name: String,
    pub kernel: u8,
    pub force: u8,
    pub tau: f64,
    pub absorption: f64,
    pub advection: f64,
    pub unit: String,
    pub fold: Option<(u8, String)>,
}

pub struct BrowserSensor {
    pub key: String,
    pub force: u8,
    pub kernel: u8,
    pub ttl: f64,
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

#[derive(Clone)]
pub struct SourceConfig {
    pub ttl: u64,
    pub url: String,
    pub frame: Frame,
    pub format: String,
    pub extracts: Vec<Extract>,
    pub headers: Vec<(String, String)>,
    pub post_body: Option<String>,
    pub target: Option<String>,
    pub catalog: Option<String>,
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
}

pub const J2000_EPOCH: f64 = 2451545.0;

pub const PARSEC_M: f64 = 3.085677581e16;

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
