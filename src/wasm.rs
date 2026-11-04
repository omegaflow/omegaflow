use std::collections::HashMap;
use std::sync::Arc;

use crate::archivar::{
    BodyEphemeris, MembraneCtx, Sample, SampleRecord, SpatialHash, build_spatial_hash,
    build_star_samples, parse_ephemeris_binary, query_hash,
};

#[repr(C)]
pub struct MembraneLookup {
    stars: Vec<Sample>,
    hash: Option<SpatialHash>,
    eph: HashMap<String, BodyEphemeris>,
    out: Vec<f64>,
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

    fn add_stars(&mut self, bytes: &[u8], catalog_epoch_yr: f64) {
        if bytes.is_empty() {
            return;
        }
        self.stars
            .extend(build_star_samples(bytes, Some(catalog_epoch_yr)));
        self.hash = None;
    }
}

fn inbound<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

fn layout(len: usize) -> std::alloc::Layout {
    std::alloc::Layout::from_size_align(len, 8).expect("wasm buffer layout")
}

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_alloc(len: usize) -> *mut u8 {
    if len == 0 {
        return std::ptr::NonNull::<u8>::dangling().as_ptr();
    }
    unsafe { std::alloc::alloc(layout(len)) }
}

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_dealloc(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    unsafe { std::alloc::dealloc(ptr, layout(len)) }
}

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_lookup_new(
    stars_ptr: *const u8,
    stars_len: usize,
    catalog_epoch_yr: f64,
) -> *mut MembraneLookup {
    let mut lookup = MembraneLookup {
        stars: Vec::new(),
        hash: None,
        eph: HashMap::new(),
        out: Vec::new(),
    };
    lookup.add_stars(inbound(stars_ptr, stars_len), catalog_epoch_yr);
    Box::into_raw(Box::new(lookup))
}

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_lookup_free(ptr: *mut MembraneLookup) {
    if ptr.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(ptr)) };
}

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_lookup_add_stars(
    ptr: *mut MembraneLookup,
    bytes_ptr: *const u8,
    bytes_len: usize,
    catalog_epoch_yr: f64,
) {
    if ptr.is_null() {
        return;
    }
    let lookup = unsafe { &mut *ptr };
    lookup.add_stars(inbound(bytes_ptr, bytes_len), catalog_epoch_yr);
}

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_lookup_load_ephemeris(
    ptr: *mut MembraneLookup,
    name_ptr: *const u8,
    name_len: usize,
    bytes_ptr: *const u8,
    bytes_len: usize,
) -> i32 {
    if ptr.is_null() {
        return 2;
    }
    let Ok(name) = std::str::from_utf8(inbound(name_ptr, name_len)) else {
        return 2;
    };
    let lookup = unsafe { &mut *ptr };
    match parse_ephemeris_binary(inbound(bytes_ptr, bytes_len)) {
        Some(eph) => {
            lookup.eph.insert(name.to_string(), eph);
            1
        }
        None => 2,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_lookup_query(
    ptr: *mut MembraneLookup,
    cx: f64,
    cy: f64,
    cz: f64,
    t2: f64,
    fx: f64,
    fy: f64,
    fz: f64,
) -> usize {
    if ptr.is_null() {
        return 0;
    }
    let lookup = unsafe { &mut *ptr };
    if lookup.hash.is_none() {
        let all: Vec<Arc<Sample>> = lookup.stars.iter().cloned().map(Arc::new).collect();
        lookup.hash = Some(build_spatial_hash(all, 1.0));
    }
    let Some(hash) = lookup.hash.as_ref() else {
        lookup.out.clear();
        return 0;
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
            eph: &lookup.eph,
        },
        &mut records,
    );
    lookup.out = MembraneLookup::flatten(&records);
    lookup.out.len()
}

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_lookup_result(ptr: *const MembraneLookup) -> *const f64 {
    if ptr.is_null() {
        return std::ptr::null();
    }
    let lookup = unsafe { &*ptr };
    lookup.out.as_ptr()
}
