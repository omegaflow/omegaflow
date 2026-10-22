use std::collections::BTreeMap;

use super::*;
use crate::mathematikerin::channel::ChannelDescriptor;
use crate::mathematikerin::manifest::{
    Manifest, ManifestModel, ManifestSelection, ModelRefusal,
};

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
    pub channel_ref: Option<f64>,
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

    fn index(channel_ref: u8, class: Aperture) -> Option<usize> {
        if (channel_ref as usize) >= 9 {
            return None;
        }
        let c = match class {
            Aperture::None => 0usize,
            Aperture::Flux => 1usize,
        };
        Some(channel_ref as usize * 2 + c)
    }

    pub fn get(&self, channel_ref: u8, class: Aperture) -> Option<f64> {
        Self::index(channel_ref, class).and_then(|i| self.slots[i])
    }

    pub fn set(&mut self, channel_ref: u8, class: Aperture, length: f64) {
        if let Some(i) = Self::index(channel_ref, class) {
            self.slots[i] = Some(length);
        }
    }

    pub fn resolve(&self, channel_ref: u8, class: Aperture) -> Option<f64> {
        self.get(channel_ref, class)
    }

    pub fn for_channel(&self, channel_ref: Option<u8>, class: Aperture) -> Option<f64> {
        channel_ref.and_then(|channel_ref| self.get(channel_ref, class))
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
            let Some(channel_ref) = force_id_of(force_name) else {
                continue;
            };
            let Some(class) = Aperture::of_class(class_name) else {
                continue;
            };
            let Ok(length) = length_spec.parse::<f64>() else {
                continue;
            };
            if length.is_finite() && length > 0.0 {
                aperture.set(channel_ref, class, length);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TauSource {
    Measured,
    Pending,
    Model,
}

impl TauSource {
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "measured" => Some(Self::Measured),
            "pending" => Some(Self::Pending),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Measured => "measured",
            Self::Pending => "pending",
            Self::Model => "model",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldModel {
    pub kernel: u8,
    pub tau: f64,
    pub tau_src: TauSource,
    pub absorption: f64,
    pub advection: f64,
}

#[derive(Clone)]
pub struct FieldConfig {
    pub key: String,
    pub name: String,
    pub band_id: Option<String>,
    pub model: Option<FieldModel>,
    pub channel_ref: Option<u8>,
    pub unit: String,
    pub freq: f64,
    pub bin_width: f64,
    pub fold: Option<(u8, String)>,
    pub aperture: Aperture,
}

impl std::ops::Deref for FieldConfig {
    type Target = FieldModel;

    fn deref(&self) -> &Self::Target {
        self.model
            .as_ref()
            .expect("FieldConfig carries no model; callers must refuse absent model")
    }
}

impl std::ops::DerefMut for FieldConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.model
            .as_mut()
            .expect("FieldConfig carries no model; callers must refuse absent model")
    }
}

impl FieldConfig {
    pub fn from_evidence(
        block: &SourceBlock,
        field: &SourceField,
        selection: &ManifestSelection,
        channel_ref: Option<u8>,
    ) -> Result<Self, ModelRefusal> {
        let model = field.selected_model(block, selection)?;
        Ok(Self::from_model(field, &model, channel_ref))
    }

    pub fn from_model(
        field: &SourceField,
        model: &ManifestModel,
        channel_ref: Option<u8>,
    ) -> Self {
        Self {
            key: field.key.clone(),
            name: field.name.clone(),
            band_id: None,
            model: Some(FieldModel {
                kernel: model.kernel(),
                tau: model.tau(),
                tau_src: TauSource::Model,
                absorption: model.absorption(),
                advection: model.advection(),
            }),
            channel_ref,
            unit: field.unit.clone(),
            freq: crate::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: Aperture::None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediumValue {
    Absent,
    Pending,
    Measured(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct MediumLine {
    pub absorption: MediumValue,
    pub absorption_unit: Option<String>,
    pub advection: MediumValue,
    pub advection_unit: Option<String>,
}

pub const SLOT_ABSENT: f64 = -1.0;

pub const PRESENCE_FLAG_PHASE: f64 = 1.0;
pub const PRESENCE_FLAG_ABSORPTION: f64 = 2.0;
pub const PRESENCE_FLAG_ADVECTION: f64 = 4.0;
pub const PRESENCE_FLAG_FLUX: f64 = 8.0;
pub const PRESENCE_FLAG_QUANTITY: f64 = 16.0;

pub fn wire_channel_ref(channel_ref: Option<f64>) -> f64 {
    let Some(channel_ref) = channel_ref else {
        return 0.0;
    };
    channel_ref
}

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
    pub channel_ref: u8,
    pub kernel: u8,
    pub ttl: f64,
    pub tau: Option<f64>,
    pub unit: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnchorKind {
    At,
    On,
    Global,
    Coverage,
    Orbit,
    Query,
    Track,
    Sky,
    Absent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Anchor {
    pub kind: AnchorKind,
    pub body: Option<String>,
    pub attrs: BTreeMap<String, String>,
}

impl Anchor {
    pub fn frame(&self) -> Frame {
        match self.kind {
            AnchorKind::At => self
                .body
                .as_ref()
                .map(|body| Frame::Barycenter {
                    body_name: body.clone(),
                    scale: 1.0,
                })
                .unwrap_or(Frame::Manifest),
            AnchorKind::Query | AnchorKind::Track | AnchorKind::Coverage | AnchorKind::Global => {
                self.body
                    .as_ref()
                    .map(|body| Frame::Body {
                        body_name: body.clone(),
                    })
                    .unwrap_or(Frame::Manifest)
            }
            AnchorKind::On => {
                let Some(body) = self.body.as_ref() else {
                    return Frame::Manifest;
                };
                let Some(lat) = self.attrs.get("lat").and_then(|v| v.parse::<f64>().ok()) else {
                    return Frame::Manifest;
                };
                let Some(lon) = self.attrs.get("lon").and_then(|v| v.parse::<f64>().ok()) else {
                    return Frame::Manifest;
                };
                let alt = match self.attrs.get("alt").map(String::as_str) {
                    Some("absent" | "pending") | None => None,
                    Some(v) => v.parse::<f64>().ok(),
                };
                Frame::Surface {
                    body_name: body.clone(),
                    lat,
                    lon,
                    alt,
                }
            }
            _ => Frame::Manifest,
        }
    }
}

#[derive(Clone)]
pub enum Frame {
    Surface {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: Option<f64>,
    },
    Barycenter {
        body_name: String,
        scale: f64,
    },
    Body {
        body_name: String,
    },
    Manifest,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeAxis {
    pub start_m: f64,
    pub step_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryCenter {
    Receiver,
    Anchor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelKind {
    Static,
    TimeVariable,
}

pub fn source_model_kind(format: &str) -> Option<ModelKind> {
    match format {
        "gfc" => Some(ModelKind::Static),
        "gfct" => Some(ModelKind::TimeVariable),
        _ => None,
    }
}

pub fn parse_model_kind(tok: &str) -> Option<ModelKind> {
    match tok {
        "static" => Some(ModelKind::Static),
        "time_variable" => Some(ModelKind::TimeVariable),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModelHead {
    pub gm_m3_s2: f64,
    pub radius_m: f64,
    pub norm: crate::archivar::gfc::Norm,
    pub tide_system: Option<String>,
    pub max_degree: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceField {
    pub key: String,
    pub name: String,
    pub unit: String,
    pub source_column: Option<usize>,
    pub source_key: Option<String>,
    pub alt_offset: f64,
    pub alt_min_offset: Option<f64>,
    pub alt_max_offset: Option<f64>,
}

impl SourceField {
    pub fn source_ref(&self, block: &SourceBlock) -> String {
        format!("{}#{}", block.url, self.key)
    }

    pub fn selected_model(
        &self,
        block: &SourceBlock,
        selection: &ManifestSelection,
    ) -> Result<ManifestModel, ModelRefusal> {
        selection.resolve_for(&self.source_ref(block))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SourcePosition {
    Fixed { lat: f64, lon: f64 },
    Moving { lat_key: String, lon_key: String },
    At {
        body: String,
        frame: String,
        distance_au: Option<f64>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceBlock {
    pub url: String,
    pub terms: String,
    pub origin: Option<String>,
    pub ttl: u64,
    pub position: SourcePosition,
    pub alt_ref: Option<f64>,
    pub alt_key: Option<String>,
    pub datum: String,
    pub time: Option<String>,
    pub time_key: Option<String>,
    pub time_parts: Option<Vec<String>>,
    pub time_scale: String,
    pub fields: Vec<SourceField>,
}

pub struct RuntimeSource {
    pub block: SourceBlock,
    pub fields: Vec<FieldConfig>,
}

impl SourceBlock {
    pub fn field(&self, key: &str) -> Option<&SourceField> {
        self.fields.iter().find(|field| field.key == key)
    }

    pub fn runtime_fields<F>(
        &self,
        manifest: &Manifest,
        channel_for: F,
    ) -> Result<Vec<FieldConfig>, ModelRefusal>
    where
        F: Fn(&SourceField) -> Option<u8>,
    {
        self.fields
            .iter()
            .map(|field| {
                let model = manifest.resolve(&field.source_ref(self))?;
                Ok(FieldConfig::from_model(field, &model, channel_for(field)))
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct SourceConfig {
    pub ttl: u64,
    pub url: String,
    pub origin: Option<String>,
    pub compiler: Option<String>,
    pub terms: Option<String>,
    pub medium: Option<MediumLine>,
    pub band: Option<String>,
    pub footprint: Option<String>,
    pub witness: Option<String>,
    pub rights_identifier: Option<String>,
    pub rights_scheme: Option<String>,
    pub rights_uri: Option<String>,
    pub anchor: Option<Anchor>,
    pub format: String,
    pub model_kind: Option<ModelKind>,
    pub head: Option<ModelHead>,
    pub extracts: Vec<Extract>,
    pub channels: Vec<ChannelDescriptor>,
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
    pub fanout_center: Option<QueryCenter>,
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
    pub weberin_role: Option<WeberinRole>,
}

impl SourceConfig {
    pub fn frame(&self) -> Frame {
        self.anchor
            .as_ref()
            .map(Anchor::frame)
            .unwrap_or(Frame::Manifest)
    }
}

pub const J2000_EPOCH: f64 = 2451545.0;

pub const J2000_UNIX_OFFSET: f64 = 946_728_000.0;

pub const SECONDS_PER_DAY: f64 = 86400.0;

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
        Frame::Surface { body_name, .. }
        | Frame::Barycenter { body_name, .. }
        | Frame::Body { body_name } => body_name.clone(),
        Frame::Manifest => String::new(),
    }
}

#[cfg(test)]
mod model_kind_tests {
    use super::*;

    #[test]
    fn gfc_is_the_static_icgem_format() {
        assert_eq!(source_model_kind("gfc"), Some(ModelKind::Static));
    }

    #[test]
    fn gfct_is_the_time_variable_icgem_format() {
        assert_eq!(source_model_kind("gfct"), Some(ModelKind::TimeVariable));
    }

    #[test]
    fn an_unknown_format_carries_no_guessed_kind() {
        assert_eq!(source_model_kind("ephemeris_binary"), None);
    }
}

#[cfg(test)]
mod source_block_model_tests {
    use super::*;
    use crate::mathematikerin::manifest::{
        ManifestIntent, ManifestSelection, ModelGap, ModelRefusal,
    };

    fn block() -> SourceBlock {
        SourceBlock {
            url: "https://example.org/data.csv".to_string(),
            terms: "CC-BY-4.0 https://example.org/terms".to_string(),
            origin: None,
            ttl: 60,
            position: SourcePosition::Fixed { lat: 1.0, lon: 2.0 },
            alt_ref: Some(0.0),
            alt_key: None,
            datum: "msl".to_string(),
            time: Some("2026-10-22T00:00:00Z".to_string()),
            time_key: None,
            time_parts: None,
            time_scale: "UTC".to_string(),
            fields: vec![
                SourceField {
                    key: "flux".to_string(),
                    name: "flux".to_string(),
                    unit: "W/m^2".to_string(),
                    source_column: Some(1),
                    source_key: None,
                    alt_offset: 2.0,
                    alt_min_offset: None,
                    alt_max_offset: None,
                },
                SourceField {
                    key: "temp".to_string(),
                    name: "temperature".to_string(),
                    unit: "K".to_string(),
                    source_column: Some(2),
                    source_key: None,
                    alt_offset: 2.0,
                    alt_min_offset: None,
                    alt_max_offset: None,
                },
            ],
        }
    }

    #[test]
    fn field_pointer_joins_url_and_key() {
        let block = block();
        let field = block.field("flux").expect("flux is declared");
        assert_eq!(
            field.source_ref(&block),
            "https://example.org/data.csv#flux"
        );
        assert!(block.field("absent").is_none());
    }

    #[test]
    fn a_field_without_a_selection_reports_gaps_not_zeroes() {
        let block = block();
        let field = block.field("flux").expect("flux is declared");
        let bare =
            ManifestSelection::new("https://example.org/data.csv#flux", ManifestIntent::Science)
                .expect("select");
        match field.selected_model(&block, &bare) {
            Err(ModelRefusal::Gaps(gaps)) => assert_eq!(
                gaps,
                vec![
                    ModelGap::Kernel,
                    ModelGap::Tau,
                    ModelGap::Absorption,
                    ModelGap::Advection,
                ]
            ),
            other => panic!("absence must be a gap, got {other:?}"),
        }
    }

    #[test]
    fn another_fields_selection_does_not_leak_onto_this_field() {
        let block = block();
        let field = block.field("flux").expect("flux is declared");
        let other =
            ManifestSelection::new("https://example.org/data.csv#temp", ManifestIntent::Science)
                .expect("select")
                .with_kernel("inverse-square")
                .expect("kernel")
                .with_model_tau(
                    60.0,
                    crate::mathematikerin::manifest::ModelTauSource::Chosen,
                )
                .expect("tau")
                .with_model_absorption(0.0)
                .expect("absorption")
                .with_model_advection(0.0)
                .expect("advection");
        assert!(matches!(
            field.selected_model(&block, &other),
            Err(ModelRefusal::ForeignSource { .. })
        ));
    }

    #[test]
    fn a_complete_selection_supplies_the_model() {
        let block = block();
        let field = block.field("flux").expect("flux is declared");
        let chosen =
            ManifestSelection::new("https://example.org/data.csv#flux", ManifestIntent::Science)
                .expect("select")
                .with_kernel("inverse-square")
                .expect("kernel")
                .with_model_tau(
                    60.0,
                    crate::mathematikerin::manifest::ModelTauSource::Chosen,
                )
                .expect("tau")
                .with_model_absorption(0.25)
                .expect("absorption")
                .with_model_advection(400.0)
                .expect("advection");
        let model = field.selected_model(&block, &chosen).expect("model");
        assert_eq!(model.kernel(), 0);
        assert_eq!(model.tau(), 60.0);
        assert_eq!(model.absorption(), 0.25);
        assert_eq!(model.advection(), 400.0);
        assert_eq!(field.alt_offset, 2.0, "evidence stays evidence");
    }

    #[test]
    fn from_evidence_builds_a_runtime_field_only_from_a_complete_selection() {
        let block = block();
        let field = block.field("flux").expect("flux is declared");
        let chosen =
            ManifestSelection::new("https://example.org/data.csv#flux", ManifestIntent::Science)
                .expect("select")
                .with_kernel("inverse-square")
                .expect("kernel")
                .with_model_tau(
                    60.0,
                    crate::mathematikerin::manifest::ModelTauSource::Chosen,
                )
                .expect("tau")
                .with_model_absorption(0.25)
                .expect("absorption")
                .with_model_advection(400.0)
                .expect("advection");
        let field_config =
            FieldConfig::from_evidence(&block, field, &chosen, Some(7)).expect("runtime field");
        let model = field_config.model.expect("model");
        assert_eq!(model.kernel, 0);
        assert_eq!(model.tau, 60.0);
        assert_eq!(model.tau_src, TauSource::Model);
        assert_eq!(model.absorption, 0.25);
        assert_eq!(model.advection, 400.0);
        assert_eq!(field_config.channel_ref, Some(7));
        assert_eq!(field_config.key, "flux");
    }

    #[test]
    fn a_block_builds_runtime_fields_only_through_the_manifest() {
        let block = block();
        let mut manifest = Manifest::new();
        for key in ["flux", "temp"] {
            manifest
                .add(
                    ManifestSelection::new(
                        format!("https://example.org/data.csv#{key}"),
                        ManifestIntent::Science,
                    )
                    .expect("select")
                    .with_kernel("inverse-square")
                    .expect("kernel")
                    .with_model_tau(60.0, crate::mathematikerin::manifest::ModelTauSource::Chosen)
                    .expect("tau")
                    .with_model_absorption(0.0)
                    .expect("absorption")
                    .with_model_advection(0.0)
                    .expect("advection"),
                )
                .expect("add");
        }
        let fields = block
            .runtime_fields(&manifest, |_| Some(1))
            .expect("all fields resolve");
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].model.as_ref().expect("model").tau, 60.0);

        let incomplete = Manifest::new();
        assert!(matches!(
            block.runtime_fields(&incomplete, |_| None),
            Err(ModelRefusal::Unselected { .. })
        ));
    }
}
