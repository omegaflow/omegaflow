use crate::noaa_coops::haversine_km;
use omegaflow::archivar::inflate::gunzip_stream;
use omegaflow::archivar::netcdf::{NetcdfFile, NetcdfType, NetcdfVar};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const G: f64 = 9.80665;
const EARTH_RADIUS_M: f64 = 6_371_000.0;

#[derive(Clone, Debug)]
pub struct DepthGrid {
    pub depths: Vec<f32>,
    pub nlon: usize,
    pub nlat: usize,
    pub lon0: f64,
    pub lat0: f64,
    pub dlon: f64,
    pub dlat: f64,
}

#[derive(Clone, Debug)]
pub enum EikonalNote {
    Gunzip,
    Parse,
    AbsentZ,
    TypeZ,
    AbsentScale,
    AbsentOffset,
    AbsentCoord,
    Shape,
    RawZ,
}

impl DepthGrid {
    pub fn node_lat_lon(&self, idx: usize) -> Option<(f64, f64)> {
        if idx >= self.depths.len() || self.nlon == 0 {
            return None;
        }
        let row = idx / self.nlon;
        let col = idx % self.nlon;
        Some((
            self.lat0 + row as f64 * self.dlat,
            self.lon0 + col as f64 * self.dlon,
        ))
    }

    pub fn is_boundary(&self, idx: usize) -> bool {
        if idx >= self.depths.len() || self.nlon == 0 {
            return false;
        }
        let row = idx / self.nlon;
        let col = idx % self.nlon;
        row == 0 || col == 0 || row + 1 == self.nlat || col + 1 == self.nlon
    }
}

fn wrap_lon(lon: f64, lon0: f64) -> f64 {
    let mut x = lon;
    while x < lon0 {
        x += 360.0;
    }
    while x >= lon0 + 360.0 {
        x -= 360.0;
    }
    x
}

pub fn node_at(grid: &DepthGrid, lat: f64, lon: f64) -> Option<usize> {
    if grid.nlon == 0 || grid.nlat == 0 || !lat.is_finite() || !lon.is_finite() {
        return None;
    }
    let fi = ((lat - grid.lat0) / grid.dlat).round();
    if !fi.is_finite() {
        return None;
    }
    let i = fi as isize;
    if i < 0 || i as usize >= grid.nlat {
        return None;
    }
    let x = wrap_lon(lon, grid.lon0);
    let fj = ((x - grid.lon0) / grid.dlon).round();
    if !fj.is_finite() {
        return None;
    }
    let j = fj as isize;
    if j < 0 || j as usize >= grid.nlon {
        return None;
    }
    Some(i as usize * grid.nlon + j as usize)
}

pub fn unwrap_window(
    global: &DepthGrid,
    lon_min: f64,
    lon_max: f64,
    lat_min: f64,
    lat_max: f64,
) -> Option<DepthGrid> {
    if global.nlon == 0 || global.nlat == 0 {
        return None;
    }
    if !(lon_max > lon_min) || !(lat_max > lat_min) || (lon_max - lon_min) >= 360.0 {
        return None;
    }
    let fi0 = ((lat_min - global.lat0) / global.dlat).round();
    let fi1 = ((lat_max - global.lat0) / global.dlat).round();
    if !fi0.is_finite() || !fi1.is_finite() {
        return None;
    }
    let (i0, i1) = (fi0 as isize, fi1 as isize);
    if i0 < 0 || i1 < 0 || i1 < i0 || i1 as usize >= global.nlat {
        return None;
    }
    let (i0, i1) = (i0 as usize, i1 as usize);
    let nlat = i1 - i0 + 1;
    let lon_last = global.lon0 + (global.nlon as f64 - 1.0) * global.dlon;
    let period = if (lon_last - global.lon0 - 360.0).abs() <= 0.5 * global.dlon {
        global.nlon - 1
    } else {
        global.nlon
    };
    let x0 = wrap_lon(lon_min, global.lon0);
    let fj = ((x0 - global.lon0) / global.dlon).round();
    if !fj.is_finite() {
        return None;
    }
    let j_start = fj as isize;
    if j_start < 0 || j_start as usize >= global.nlon {
        return None;
    }
    let j_start = j_start as usize;
    let nlon = ((lon_max - lon_min) / global.dlon).round() as usize + 1;
    let mut depths = Vec::with_capacity(nlon * nlat);
    for i in 0..nlat {
        let gi = i0 + i;
        for j in 0..nlon {
            let k = j_start + j;
            let gj = if period == global.nlon {
                if k >= global.nlon {
                    return None;
                }
                k
            } else {
                k % period
            };
            depths.push(global.depths[gi * global.nlon + gj]);
        }
    }
    Some(DepthGrid {
        depths,
        nlon,
        nlat,
        lon0: global.lon0 + j_start as f64 * global.dlon,
        lat0: global.lat0 + i0 as f64 * global.dlat,
        dlon: global.dlon,
        dlat: global.dlat,
    })
}

#[derive(Clone, Debug)]
pub struct SourceNode {
    pub node: usize,
    pub epicenter_node: usize,
    pub epicenter_depth: f32,
    pub displaced: bool,
    pub offset_km: f64,
}

pub fn source_node(grid: &DepthGrid, lat: f64, lon: f64) -> Option<SourceNode> {
    let epicenter_node = node_at(grid, lat, lon)?;
    let epicenter_depth = *grid.depths.get(epicenter_node)?;
    if epicenter_depth > 0.0 {
        return Some(SourceNode {
            node: epicenter_node,
            epicenter_node,
            epicenter_depth,
            displaced: false,
            offset_km: 0.0,
        });
    }
    let mut best: Option<(usize, f64)> = None;
    for idx in 0..grid.depths.len() {
        if !(grid.depths[idx] > 0.0) {
            continue;
        }
        let Some((la, lo)) = grid.node_lat_lon(idx) else {
            continue;
        };
        let km = haversine_km(lat, lon, la, lo);
        if !km.is_finite() {
            continue;
        }
        best = match best {
            Some((_, b)) if b <= km => best,
            _ => Some((idx, km)),
        };
    }
    let (node, offset_km) = best?;
    Some(SourceNode {
        node,
        epicenter_node,
        epicenter_depth,
        displaced: true,
        offset_km,
    })
}

pub fn edge_seconds(grid: &DepthGrid, a: usize, b: usize) -> Option<f32> {
    let da = *grid.depths.get(a)?;
    let db = *grid.depths.get(b)?;
    if !(da > 0.0 && db > 0.0) {
        return None;
    }
    let (lat_a, lon_a) = grid.node_lat_lon(a)?;
    let (lat_b, lon_b) = grid.node_lat_lon(b)?;
    let lat_mid = 0.5 * (lat_a + lat_b);
    let dlat = (lat_b - lat_a).to_radians();
    let dlon = (lon_b - lon_a).to_radians();
    let dy = dlat * EARTH_RADIUS_M;
    let dx = dlon * lat_mid.to_radians().cos() * EARTH_RADIUS_M;
    let arc = (dx * dx + dy * dy).sqrt();
    let d_bar = 0.5 * (da as f64 + db as f64);
    let c = (G * d_bar).sqrt();
    if !(c > 0.0) {
        return None;
    }
    let t = arc / c;
    if t.is_finite() && t > 0.0 {
        Some(t as f32)
    } else {
        None
    }
}

pub fn arrival_at(grid: &DepthGrid, times: &[f32], lat: f64, lon: f64) -> Option<f32> {
    let idx = node_at(grid, lat, lon)?;
    if !(grid.depths.get(idx).copied()? > 0.0) {
        return None;
    }
    let t = *times.get(idx)?;
    if t.is_finite() {
        Some(t)
    } else {
        None
    }
}

struct QueueEntry {
    t: f32,
    idx: u32,
}

impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.idx == other.idx
    }
}

impl Eq for QueueEntry {}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .t
            .total_cmp(&self.t)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

pub fn dijkstra_times(grid: &DepthGrid, source: usize) -> Vec<f32> {
    dijkstra_times_boundary(grid, source).0
}

pub fn dijkstra_times_boundary(grid: &DepthGrid, source: usize) -> (Vec<f32>, Vec<bool>) {
    let n = grid.depths.len();
    let mut times = vec![f32::INFINITY; n];
    let mut touched = vec![false; n];
    if source >= n || !(grid.depths[source] > 0.0) {
        return (times, touched);
    }
    times[source] = 0.0;
    touched[source] = grid.is_boundary(source);
    let mut heap = BinaryHeap::new();
    heap.push(QueueEntry {
        t: 0.0,
        idx: source as u32,
    });
    while let Some(QueueEntry { t, idx }) = heap.pop() {
        let u = idx as usize;
        if t > times[u] {
            continue;
        }
        let nlon = grid.nlon;
        let row = u / nlon;
        let col = u % nlon;
        for dr in -1isize..=1 {
            for dc in -1isize..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let r = row as isize + dr;
                let c = col as isize + dc;
                if r < 0 || c < 0 {
                    continue;
                }
                let (r, c) = (r as usize, c as usize);
                if r >= grid.nlat || c >= grid.nlon {
                    continue;
                }
                let v = r * nlon + c;
                let Some(w) = edge_seconds(grid, u, v) else {
                    continue;
                };
                let nt = t + w;
                if nt < times[v] {
                    times[v] = nt;
                    touched[v] = touched[u] || grid.is_boundary(v);
                    heap.push(QueueEntry { t: nt, idx: v as u32 });
                }
            }
        }
    }
    (times, touched)
}

fn var_attr(file: &NetcdfFile, v: &NetcdfVar, key: &str) -> Option<f64> {
    v.attrs
        .iter()
        .find(|a| a.name == key)
        .and_then(|a| file.attr_num(a))
}

fn coord_values(file: &NetcdfFile, bytes: &[u8], names: &[&str]) -> Option<Vec<f64>> {
    for name in names {
        let Some(v) = file.var(name) else {
            continue;
        };
        let vals = match v.nc_type {
            NetcdfType::Double => file.values_f64(bytes, name),
            NetcdfType::Float => file
                .values_f32(bytes, name)
                .map(|xs| xs.into_iter().map(|x| x as f64).collect()),
            NetcdfType::Int => file
                .values_i32(bytes, name)
                .map(|xs| xs.into_iter().map(|x| x as f64).collect()),
            NetcdfType::Short => file
                .values_i16(bytes, name)
                .map(|xs| xs.into_iter().map(|x| x as f64).collect()),
            NetcdfType::Byte => file
                .values_i8(bytes, name)
                .map(|xs| xs.into_iter().map(|x| x as f64).collect()),
            NetcdfType::Char => None,
        };
        if let Some(vals) = vals {
            if vals.len() >= 2 {
                return Some(vals);
            }
        }
    }
    None
}

pub fn decode_etopo1(bytes: &[u8]) -> Result<DepthGrid, EikonalNote> {
    let is_gzip = bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b;
    let gunzipped = if is_gzip {
        let mut out = Vec::new();
        match gunzip_stream(bytes, |chunk| out.extend_from_slice(chunk)) {
            Ok(_) => Some(out),
            Err(_) => return Err(EikonalNote::Gunzip),
        }
    } else {
        None
    };
    let data: &[u8] = match &gunzipped {
        Some(v) => v,
        None => bytes,
    };
    let file = NetcdfFile::parse(data).map_err(|_| EikonalNote::Parse)?;
    let z = file.var("z").ok_or(EikonalNote::AbsentZ)?;
    if z.nc_type != NetcdfType::Short {
        return Err(EikonalNote::TypeZ);
    }
    let scale = var_attr(&file, z, "scale_factor").ok_or(EikonalNote::AbsentScale)?;
    let offset = var_attr(&file, z, "add_offset").ok_or(EikonalNote::AbsentOffset)?;
    let shape = file.var_shape(z).map_err(|_| EikonalNote::Shape)?;
    if shape.len() != 2 {
        return Err(EikonalNote::Shape);
    }
    let xv = coord_values(&file, data, &["x", "lon", "longitude"]).ok_or(EikonalNote::AbsentCoord)?;
    let yv = coord_values(&file, data, &["y", "lat", "latitude"]).ok_or(EikonalNote::AbsentCoord)?;
    let nlon = xv.len();
    let nlat = yv.len();
    let (s0, s1) = (shape[0] as usize, shape[1] as usize);
    let lat_major = if s0 == nlat && s1 == nlon {
        true
    } else if s0 == nlon && s1 == nlat {
        false
    } else {
        return Err(EikonalNote::Shape);
    };
    let raw = file.values_i16(data, "z").ok_or(EikonalNote::RawZ)?;
    if raw.len() != nlat * nlon {
        return Err(EikonalNote::Shape);
    }
    let mut depths = Vec::with_capacity(nlat * nlon);
    for i in 0..nlat {
        for j in 0..nlon {
            let k = if lat_major { i * nlon + j } else { j * nlat + i };
            let elevation = raw[k] as f64 * scale + offset;
            depths.push((-elevation) as f32);
        }
    }
    let lon0 = xv[0];
    let lat0 = yv[0];
    let dlon = xv[1] - xv[0];
    let dlat = yv[1] - yv[0];
    if !(dlon > 0.0 && dlat > 0.0) {
        return Err(EikonalNote::Shape);
    }
    Ok(DepthGrid {
        depths,
        nlon,
        nlat,
        lon0,
        lat0,
        dlon,
        dlat,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat_grid(nlon: usize, nlat: usize, depth: f32) -> DepthGrid {
        DepthGrid {
            depths: vec![depth; nlon * nlat],
            nlon,
            nlat,
            lon0: 0.0,
            lat0: 0.0,
            dlon: 0.01,
            dlat: 0.01,
        }
    }

    #[test]
    fn synthetic_constant_depth_matches_shallow_water_speed() {
        let grid = flat_grid(16, 16, 4000.0);
        let times = dijkstra_times(&grid, 0);
        let target = 15 * 16 + 15;
        let t = times[target] as f64;
        let mut path = 0.0;
        for i in 0..15 {
            let lat_mid = (i as f64 + 0.5) * 0.01;
            let dlat = 0.01f64.to_radians();
            let dlon = 0.01f64.to_radians();
            let arc = EARTH_RADIUS_M
                * (dlat * dlat + (lat_mid.to_radians().cos() * dlon).powi(2)).sqrt();
            path += arc / (G * 4000.0).sqrt();
        }
        assert!(
            (t - path).abs() / path < 1e-3,
            "eikonal {t} s against the shallow-water path {path} s"
        );
        let lat_a = 0.0f64;
        let lat_b = 0.15f64;
        let dphi = (lat_b - lat_a).to_radians();
        let dlam = (0.15f64 - 0.0).to_radians();
        let h = (dphi / 2.0).sin().powi(2)
            + lat_a.to_radians().cos() * lat_b.to_radians().cos() * (dlam / 2.0).sin().powi(2);
        let great = 2.0 * EARTH_RADIUS_M * h.sqrt().atan2((1.0 - h).sqrt());
        let euclid = great / (G * 4000.0).sqrt();
        assert!(
            (t - euclid).abs() / euclid < 1e-2,
            "eikonal {t} s against the great-circle {euclid} s"
        );
    }

    #[test]
    fn land_wall_detour_uses_the_named_route() {
        let n = 3usize;
        let mut depths = vec![1000.0f32; n * n];
        depths[n + 1] = -1.0;
        let grid = DepthGrid {
            depths,
            nlon: n,
            nlat: n,
            lon0: 0.0,
            lat0: 0.0,
            dlon: 0.01,
            dlat: 0.01,
        };
        let times = dijkstra_times(&grid, 0);
        let target = 2 * n + 2;
        let t = times[target] as f64;
        let route = [(0usize, 0usize), (0, 1), (1, 2), (2, 2)];
        let mut named = 0.0;
        for w in route.windows(2) {
            let a = w[0].0 * n + w[0].1;
            let b = w[1].0 * n + w[1].1;
            named += edge_seconds(&grid, a, b).unwrap() as f64;
        }
        let c = (G * 1000.0).sqrt();
        let theta = 0.01f64.to_radians();
        let formula = (2.0 + 2.0f64.sqrt()) * theta * EARTH_RADIUS_M / c;
        assert!(
            (t - named).abs() / named < 1e-3,
            "eikonal {t} s against the named detour {named} s"
        );
        assert!(
            (t - formula).abs() / formula < 1e-2,
            "eikonal {t} s against the detour formula {formula} s"
        );
    }

    #[test]
    fn shallow_ridge_prefers_the_deep_channel() {
        let n = 5usize;
        let mut depths = vec![1.0f32; n * n];
        for i in 0..n {
            depths[i * n] = 4000.0;
        }
        let grid = DepthGrid {
            depths,
            nlon: n,
            nlat: n,
            lon0: 0.0,
            lat0: 0.0,
            dlon: 0.01,
            dlat: 0.01,
        };
        let source = 1usize;
        let target = 4 * n + 1;
        let times = dijkstra_times(&grid, source);
        let t = times[target] as f64;
        let mut flat = 0.0;
        for i in 0..4 {
            flat += edge_seconds(&grid, i * n + 1, (i + 1) * n + 1).unwrap() as f64;
        }
        assert!(t < flat, "deep channel {t} s against flat straight {flat} s");
        let mut deep_bound = 0.0;
        for i in 0..4 {
            deep_bound += edge_seconds(&grid, i * n, (i + 1) * n).unwrap() as f64;
        }
        assert!(
            t >= deep_bound - 1e-3,
            "eikonal {t} s faster than the all-deep lower bound {deep_bound} s"
        );
        let mut channel = edge_seconds(&grid, source, 0).unwrap() as f64;
        for i in 0..4 {
            channel += edge_seconds(&grid, i * n, (i + 1) * n).unwrap() as f64;
        }
        channel += edge_seconds(&grid, 4 * n, target).unwrap() as f64;
        assert!(
            t <= channel + 1e-3,
            "eikonal {t} s above the channel route {channel} s"
        );
    }

    #[test]
    fn land_station_is_absent() {
        let n = 3usize;
        let mut depths = vec![1000.0f32; n * n];
        depths[n + 1] = -1.0;
        let grid = DepthGrid {
            depths,
            nlon: n,
            nlat: n,
            lon0: 0.0,
            lat0: 0.0,
            dlon: 1.0,
            dlat: 1.0,
        };
        let times = dijkstra_times(&grid, 0);
        assert!(arrival_at(&grid, &times, 1.0, 1.0).is_none());
        assert!(times[n + 1].is_infinite());
        assert!(arrival_at(&grid, &times, 2.0, 2.0).is_some());
    }

    #[test]
    fn unwrap_window_joins_the_two_x_bands() {
        let mut depths = Vec::new();
        for i in 0..2 {
            for j in 0..5 {
                depths.push((i * 10 + j) as f32);
            }
        }
        let global = DepthGrid {
            depths,
            nlon: 5,
            nlat: 2,
            lon0: 0.0,
            lat0: 0.0,
            dlon: 90.0,
            dlat: 10.0,
        };
        let window = unwrap_window(&global, 180.0, 360.0, 0.0, 10.0).unwrap();
        assert_eq!(window.nlon, 3);
        assert_eq!(window.nlat, 2);
        assert!((window.lon0 - 180.0).abs() < 1e-9);
        assert_eq!(window.depths, vec![2.0, 3.0, 0.0, 12.0, 13.0, 10.0]);
    }

    fn u32b(x: u32) -> Vec<u8> {
        x.to_be_bytes().to_vec()
    }

    fn f64b(x: f64) -> Vec<u8> {
        x.to_bits().to_be_bytes().to_vec()
    }

    fn name(s: &str) -> Vec<u8> {
        let mut b = u32b(s.len() as u32);
        b.extend_from_slice(s.as_bytes());
        while b.len() % 4 != 0 {
            b.push(0);
        }
        b
    }

    fn absent(b: &mut Vec<u8>) {
        b.extend(u32b(0));
        b.extend(u32b(0));
    }

    fn attr_double(b: &mut Vec<u8>, nm: &str, val: f64) {
        b.extend(name(nm));
        b.extend(u32b(6));
        b.extend(u32b(1));
        b.extend(f64b(val));
    }

    fn build_etopo_cdf(nlon: usize, nlat: usize, dlon: f64, dlat: f64, depth_m: f64) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(0));
        b.extend(u32b(0x0A));
        b.extend(u32b(2));
        b.extend(name("y"));
        b.extend(u32b(nlat as u32));
        b.extend(name("x"));
        b.extend(u32b(nlon as u32));
        absent(&mut b);
        b.extend(u32b(0x0B));
        b.extend(u32b(3));
        let mut slots = Vec::new();
        b.extend(name("x"));
        b.extend(u32b(1));
        b.extend(u32b(1));
        absent(&mut b);
        b.extend(u32b(6));
        b.extend(u32b((nlon * 8) as u32));
        slots.push(b.len());
        b.extend(u32b(0));
        b.extend(name("y"));
        b.extend(u32b(1));
        b.extend(u32b(0));
        absent(&mut b);
        b.extend(u32b(6));
        b.extend(u32b((nlat * 8) as u32));
        slots.push(b.len());
        b.extend(u32b(0));
        b.extend(name("z"));
        b.extend(u32b(2));
        b.extend(u32b(0));
        b.extend(u32b(1));
        b.extend(u32b(0x0C));
        b.extend(u32b(2));
        attr_double(&mut b, "scale_factor", 1.0);
        attr_double(&mut b, "add_offset", 0.0);
        b.extend(u32b(3));
        b.extend(u32b((nlat * nlon * 2) as u32));
        slots.push(b.len());
        b.extend(u32b(0));
        let begin_x = b.len() as u32;
        let begin_y = begin_x + (nlon * 8) as u32;
        let begin_z = begin_y + (nlat * 8) as u32;
        b[slots[0]..slots[0] + 4].copy_from_slice(&begin_x.to_be_bytes());
        b[slots[1]..slots[1] + 4].copy_from_slice(&begin_y.to_be_bytes());
        b[slots[2]..slots[2] + 4].copy_from_slice(&begin_z.to_be_bytes());
        for j in 0..nlon {
            b.extend(f64b(j as f64 * dlon));
        }
        for i in 0..nlat {
            b.extend(f64b(i as f64 * dlat));
        }
        let zval = (-depth_m).round() as i16;
        for _ in 0..(nlat * nlon) {
            b.extend(zval.to_be_bytes());
        }
        b
    }

    fn crc32(data: &[u8]) -> u32 {
        let mut crc = 0xFFFF_FFFFu32;
        for &byte in data {
            crc ^= byte as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = 0xEDB8_8320 ^ (crc >> 1);
                } else {
                    crc >>= 1;
                }
            }
        }
        crc ^ 0xFFFF_FFFF
    }

    fn gzip_stored(content: &[u8]) -> Vec<u8> {
        let mut gz = vec![0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 0xff];
        let len = content.len() as u16;
        gz.push(0x01);
        gz.extend_from_slice(&len.to_le_bytes());
        gz.extend_from_slice(&(!len).to_le_bytes());
        gz.extend_from_slice(content);
        gz.extend_from_slice(&crc32(content).to_le_bytes());
        gz.extend_from_slice(&(content.len() as u32).to_le_bytes());
        gz
    }

    #[test]
    fn synthetic_cdf1_round_trip() {
        let nlon = 16usize;
        let nlat = 16usize;
        let dlon = 0.01;
        let dlat = 0.01;
        let depth = 4000.0;
        let cdf = build_etopo_cdf(nlon, nlat, dlon, dlat, depth);
        let gz = gzip_stored(&cdf);
        let grid = decode_etopo1(&gz).expect("cdf1 round trip");
        assert_eq!(grid.nlon, nlon);
        assert_eq!(grid.nlat, nlat);
        assert!((grid.depths[0] - 4000.0).abs() < 1e-3);
        let times = dijkstra_times(&grid, 0);
        let t = times[(nlat - 1) * nlon + (nlon - 1)] as f64;
        let mut expected = 0.0;
        for i in 0..(nlat - 1) {
            let lat_mid = (i as f64 + 0.5) * dlat;
            let dlat_rad = dlat.to_radians();
            let dlon_rad = dlon.to_radians();
            let arc = EARTH_RADIUS_M
                * (dlat_rad * dlat_rad
                    + (lat_mid.to_radians().cos() * dlon_rad).powi(2))
                .sqrt();
            expected += arc / (G * depth).sqrt();
        }
        assert!(
            (t - expected).abs() / expected < 1e-3,
            "round-trip eikonal {t} s against the analytic {expected} s"
        );
    }
}
