use std::collections::HashMap;
use std::sync::Arc;

use wasm_bindgen::prelude::*;

use crate::archivar::{
    BodyEphemeris, MembraneCtx, Sample, SampleRecord, SpatialHash, all_body_anchor_samples,
    build_spatial_hash, build_star_samples, parse_ephemeris_binary, query_hash,
};

#[wasm_bindgen]
pub struct MembraneLookup {
    stars: Vec<Sample>,
    hash: Option<SpatialHash>,
    eph: HashMap<String, BodyEphemeris>,
    bodies_sealed: usize,
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
        let mut lookup = MembraneLookup {
            stars: Vec::new(),
            hash: None,
            eph: HashMap::new(),
            bodies_sealed: usize::MAX,
        };
        lookup.add_stars(stars, catalog_epoch_yr);
        lookup
    }

    pub fn add_stars(&mut self, bytes: &[u8], catalog_epoch_yr: f64) {
        if bytes.is_empty() {
            return;
        }
        self.stars
            .extend(build_star_samples(bytes, Some(catalog_epoch_yr)));
        self.hash = None;
    }

    pub fn load_ephemeris(&mut self, name: &str, bytes: &[u8]) -> bool {
        match parse_ephemeris_binary(bytes) {
            Some(eph) => {
                self.eph.insert(name.to_string(), eph);
                true
            }
            None => false,
        }
    }

    pub fn query(
        &mut self,
        cx: f64,
        cy: f64,
        cz: f64,
        t2: f64,
        fx: f64,
        fy: f64,
        fz: f64,
    ) -> Vec<f64> {
        if self.hash.is_none() || self.bodies_sealed != self.eph.len() {
            let mut all: Vec<Arc<Sample>> = self.stars.iter().cloned().map(Arc::new).collect();
            for s in all_body_anchor_samples(&self.eph, t2) {
                all.push(Arc::new(s));
            }
            self.hash = Some(build_spatial_hash(all, 1.0));
            self.bodies_sealed = self.eph.len();
        }
        let Some(hash) = self.hash.as_ref() else {
            return Vec::new();
        };
        let floor = [1e-40f64; 9];
        let mut records: Vec<SampleRecord> = Vec::new();
        query_hash(
            hash,
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
