use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::fs::File;
use std::io::Read;

static HGT_CACHE: OnceLock<Mutex<HashMap<(i32, i32), Vec<i16>>>> = OnceLock::new();
static EGM96_GRID: OnceLock<Vec<f32>> = OnceLock::new();

pub fn init() {
    let _ = HGT_CACHE.set(Mutex::new(HashMap::new()));

    if let Ok(mut f) = File::open("data/WW15MGH.DAC") {
        let mut buf = Vec::new();
        if f.read_to_end(&mut buf).ok() == Some(324 * 2) {
            let grid: Vec<f32> = buf.chunks_exact(2)
                .map(|c| i16::from_be_bytes([c[0], c[1]]) as f32 * 0.01)
                .collect();
            let _ = EGM96_GRID.set(grid);
        }
    }
}

fn load_hgt(lat0: i32, lon0: i32) -> Option<Vec<i16>> {
    let ns = if lat0 >= 0 { 'N' } else { 'S' };
    let ew = if lon0 >= 0 { 'E' } else { 'W' };
    let filename = format!("{}{:02}{}{:03}.hgt", ns, lat0.abs(), ew, lon0.abs());
    let path = format!("data/{}", filename);
    
    if let Ok(mut f) = File::open(&path) {
        let mut buf = Vec::new();
        if f.read_to_end(&mut buf).ok() == Some(2884802) {
            return Some(buf.chunks_exact(2).map(|c| i16::from_be_bytes([c[0], c[1]])).collect());
        }
    }
    
    let server_url = std::env::var("OMEGAFLOW_DEM_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());
    let url = format!("{}/{}", server_url, filename);
    if let Ok(resp) = reqwest::blocking::Client::new().get(&url).send() {
        if resp.status().is_success() {
            let buf = resp.bytes().ok()?;
            if buf.len() == 2884802 {
                let data: Vec<i16> = buf.chunks_exact(2).map(|c| i16::from_be_bytes([c[0], c[1]])).collect();
                let _ = std::fs::write(&path, &buf);
                return Some(data);
            }
        }
    }
    
    None
}

fn egm96_undulation(lat: f64, lon: f64) -> f32 {
    let Some(grid) = EGM96_GRID.get() else { return 0.0 };
    let lat_idx = ((lat + 90.0) * 2.0).floor() as usize;
    let lon_idx = if lon >= 0.0 { (lon * 2.0).floor() as usize } else { ((lon + 360.0) * 2.0).floor() as usize };
    let lat_idx = lat_idx.min(359);
    let lon_idx = lon_idx.min(719);
    grid[lat_idx * 720 + lon_idx]
}

pub fn terrain_height(lat: f64, lon: f64) -> f32 {
    let Some(cache) = HGT_CACHE.get() else { return egm96_undulation(lat, lon) };
    let lat0 = lat.floor() as i32;
    let lon0 = lon.floor() as i32;
    
    let tile = {
        let mut guard = cache.lock().unwrap();
        if !guard.contains_key(&(lat0, lon0)) {
            if let Some(data) = load_hgt(lat0, lon0) {
                guard.insert((lat0, lon0), data);
            } else {
                return egm96_undulation(lat, lon);
            }
        }
        guard.get(&(lat0, lon0)).cloned().unwrap()
    };
    
    let local_lat = lat - lat0 as f64;
    let local_lon = lon - lon0 as f64;
    let x = (local_lon * 1200.0) as usize;
    let y = ((1.0 - local_lat) * 1200.0) as usize;
    if x >= 1201 || y >= 1201 { return egm96_undulation(lat, lon) }
    let val = tile[y * 1201 + x];
    let hgt = if val == -32768 { 0.0 } else { val as f32 };
    hgt + egm96_undulation(lat, lon)
}
