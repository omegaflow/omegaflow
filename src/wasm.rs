use std::collections::HashMap;
use std::sync::Arc;

use wasm_bindgen::prelude::*;

use crate::archivar::{
    BodyEphemeris, MembraneCtx, SampleRecord, SpatialHash, build_spatial_hash, build_star_samples,
    query_hash,
};

#[wasm_bindgen]
pub struct MembraneLookup {
    hash: SpatialHash,
    eph: HashMap<String, BodyEphemeris>,
}

impl MembraneLookup {
    fn flatten(records: &[SampleRecord]) -> Vec<f64> {
        let mut out = Vec::with_capacity(records.len() * 26);
        for r in records {
            out.extend_from_slice(&[
                r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8, r.9, r.10, r.11, r.12, r.13, r.14,
                r.15, r.16, r.17, r.18, r.19, r.20, r.21, r.22, r.23, r.24, r.25,
            ]);
        }
        out
    }
}

#[wasm_bindgen]
impl MembraneLookup {
    #[wasm_bindgen(constructor)]
    pub fn new(stars: &[u8], catalog_epoch_yr: f64) -> MembraneLookup {
        let samples = build_star_samples(stars, Some(catalog_epoch_yr));
        let hash = build_spatial_hash(samples.into_iter().map(Arc::new).collect(), 1.0);
        MembraneLookup {
            hash,
            eph: HashMap::new(),
        }
    }

    pub fn query(&self, cx: f64, cy: f64, cz: f64, t2: f64, fx: f64, fy: f64, fz: f64) -> Vec<f64> {
        let floor = [1e-40f64; 9];
        let mut records: Vec<SampleRecord> = Vec::new();
        query_hash(
            &self.hash,
            MembraneCtx {
                center: [cx, cy, cz],
                t2,
                pad: 1.0,
                delta_t_cache: 0.0,
                floor: &floor,
                softening: 1.0,
                forward: [fx, fy, fz],
                eph: &self.eph,
            },
            &mut records,
        );
        MembraneLookup::flatten(&records)
    }
}
