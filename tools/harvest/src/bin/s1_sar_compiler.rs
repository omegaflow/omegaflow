use omegaflow::cdn::upload_release;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::process::Command;

const EVENT_EPOCH: &str = "2026-08-26T02:52:10Z";
const SEARCH_START: &str = "2026-01-01T00:00:00Z";
const BBOX: (f64, f64, f64, f64) = (85.40, 28.15, 85.60, 28.40);
const CP_LON: f64 = 85.515;
const CP_LAT: f64 = 28.271;
const CDN_RELEASE: &str = "sentinel1euwest.blob.core.windows.net";
const MAGIC: [u8; 4] = *b"S1SR";

#[derive(Clone, Copy)]
struct Pixel {
    lon: f64,
    lat: f64,
    post: f64,
    vor: f64,
    db: f64,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sL")
        .arg("--max-time")
        .arg("90")
        .arg("-A")
        .arg("omegaflow-gate/1.0")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!("curl {url}: {}", out.status);
        None
    }
}

fn sas_token(collection: &str) -> Option<String> {
    let url = format!("https://planetarycomputer.microsoft.com/api/sas/v1/token/{collection}");
    let body = String::from_utf8(curl(&url)?).ok()?;
    let key = "\"token\":\"";
    let i = body.find(key)? + key.len();
    let tok = body[i..].split('"').next()?.to_string();
    if tok.is_empty() { None } else { Some(tok) }
}

fn scan_features(body: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(i) = rest.find("\"id\":\"S1") {
        rest = &rest[i..];
        let id = rest[6..].split('"').next().unwrap_or("").to_string();
        let dt = rest
            .split("\"datetime\":\"")
            .nth(1)
            .and_then(|s| s.split('"').next())
            .unwrap_or("")
            .to_string();
        let href = rest
            .split("\"vv\":{\"href\":\"")
            .nth(1)
            .and_then(|s| s.split('"').next())
            .unwrap_or("")
            .to_string();
        out.push((id, href, dt));
        rest = &rest[6..];
    }
    out
}

fn search(
    bbox: &(f64, f64, f64, f64),
    dt_range: &str,
    sortby: &str,
    limit: u32,
) -> Vec<(String, String, String)> {
    let url = format!(
        "https://planetarycomputer.microsoft.com/api/stac/v1/search?collections=sentinel-1-grd&bbox={},{},{},{}&datetime={}&sortby={}&limit={}",
        bbox.0, bbox.1, bbox.2, bbox.3, dt_range, sortby, limit
    );
    match curl(&url) {
        Some(body) => scan_features(&String::from_utf8_lossy(&body)),
        None => Vec::new(),
    }
}

fn find_post() -> Option<(String, String, String)> {
    let feats = search(
        &BBOX,
        &format!("{EVENT_EPOCH}/2030-01-01T00:00:00Z"),
        "datetime",
        5,
    );
    feats
        .into_iter()
        .find(|(_, _, dt)| dt.as_str() > EVENT_EPOCH)
}

fn find_vor(post_id: &str) -> Option<(String, String, String)> {
    let feats = search(
        &BBOX,
        &format!("{SEARCH_START}/{EVENT_EPOCH}"),
        "-datetime",
        3,
    );
    feats
        .into_iter()
        .find(|(id, _, dt)| id != post_id && dt.as_str() < EVENT_EPOCH)
}

fn download(url: &str, path: &str) -> bool {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("1800")
        .arg("-o")
        .arg(path)
        .arg(url)
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let sz = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
            eprintln!("COG downloaded: {sz} B -> {path}");
            sz > 0
        }
        Ok(o) => {
            eprintln!(
                "COG download void: {} {}",
                o.status,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("COG download void: {e}");
            false
        }
    }
}

fn type_size(typ: u16) -> usize {
    match typ {
        1 | 2 | 6 | 7 => 1,
        3 | 8 => 2,
        4 | 9 | 11 => 4,
        5 | 10 | 12 => 8,
        _ => 0,
    }
}

struct Cog {
    path: String,
    width: usize,
    height: usize,
    tw: usize,
    th: usize,
    tile_off: Vec<u32>,
    tile_len: Vec<u32>,
    across: usize,
    grid_ni: usize,
    grid_nj: usize,
    gx: Vec<f64>,
    gy: Vec<f64>,
    glon: Vec<f64>,
    glat: Vec<f64>,
    tile_cache_idx: usize,
    tile_cache: Vec<u16>,
}

fn read_ifd_tags(path: &str) -> Result<HashMap<u16, (u16, u32, u32)>, String> {
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path}: {e}"))?;
    let mut hdr = [0u8; 8];
    f.read_exact(&mut hdr)
        .map_err(|e| format!("hdr {path}: {e}"))?;
    if &hdr[0..2] != b"II" {
        return Err("nur little-endian TIFF".to_string());
    }
    let ifd_off = u32::from_le_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]) as u64;
    f.seek(SeekFrom::Start(ifd_off))
        .map_err(|e| format!("seek {e}"))?;
    let mut nbuf = [0u8; 2];
    f.read_exact(&mut nbuf).map_err(|e| format!("n {e}"))?;
    let n = u16::from_le_bytes(nbuf) as usize;
    let mut tags = HashMap::new();
    let mut cur = ifd_off + 2;
    for _ in 0..n {
        f.seek(SeekFrom::Start(cur))
            .map_err(|e| format!("seek {e}"))?;
        let mut e = [0u8; 12];
        f.read_exact(&mut e).map_err(|e| format!("entry {e}"))?;
        let tag = u16::from_le_bytes([e[0], e[1]]);
        let typ = u16::from_le_bytes([e[2], e[3]]);
        let cnt = u32::from_le_bytes([e[4], e[5], e[6], e[7]]);
        let val = u32::from_le_bytes([e[8], e[9], e[10], e[11]]);
        tags.insert(tag, (typ, cnt, val));
        cur += 12;
    }
    Ok(tags)
}

fn tag_u16(tags: &HashMap<u16, (u16, u32, u32)>, path: &str, tag: u16) -> Result<u16, String> {
    let (_, _, v) = tags.get(&tag).ok_or(format!("{path}: tag {tag} absent"))?;
    Ok(*v as u16)
}

fn tag_u32s(
    path: &str,
    tags: &HashMap<u16, (u16, u32, u32)>,
    tag: u16,
) -> Result<Vec<u32>, String> {
    let (typ, cnt, off) = *tags.get(&tag).ok_or(format!("{path}: tag {tag} absent"))?;
    if cnt == 0 {
        return Ok(vec![]);
    }
    let sz = type_size(typ);
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path}: {e}"))?;
    let mut out = Vec::with_capacity(cnt as usize);
    for k in 0..cnt as usize {
        let o = off as u64 + (k * sz) as u64;
        f.seek(SeekFrom::Start(o))
            .map_err(|e| format!("seek {e}"))?;
        let mut b = [0u8; 4];
        f.read_exact(&mut b[..sz])
            .map_err(|e| format!("read {e}"))?;
        let v = match sz {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            4 => u32::from_le_bytes(b),
            _ => 0,
        };
        out.push(v);
    }
    Ok(out)
}

fn tag_f64s(
    path: &str,
    tags: &HashMap<u16, (u16, u32, u32)>,
    tag: u16,
) -> Result<Vec<f64>, String> {
    let (typ, cnt, off) = *tags.get(&tag).ok_or(format!("{path}: tag {tag} absent"))?;
    let sz = type_size(typ);
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path}: {e}"))?;
    let mut out = Vec::with_capacity(cnt as usize);
    for k in 0..cnt as usize {
        let o = off as u64 + (k * sz) as u64;
        f.seek(SeekFrom::Start(o))
            .map_err(|e| format!("seek {e}"))?;
        let mut b = [0u8; 8];
        f.read_exact(&mut b[..sz])
            .map_err(|e| format!("read {e}"))?;
        out.push(if sz == 8 {
            f64::from_le_bytes(b)
        } else if sz == 4 {
            f32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64
        } else {
            b[0] as f64
        });
    }
    Ok(out)
}

impl Cog {
    fn open(path: &str) -> Result<Cog, String> {
        let tags = read_ifd_tags(path)?;
        let width = tag_u16(&tags, path, 256)? as usize;
        let height = tag_u16(&tags, path, 257)? as usize;
        let tw = tag_u16(&tags, path, 322)? as usize;
        let th = tag_u16(&tags, path, 323)? as usize;
        let tile_off = tag_u32s(path, &tags, 324)?;
        let tile_len = tag_u32s(path, &tags, 325)?;
        let geo = tag_f64s(path, &tags, 33922)?;
        if geo.is_empty() || geo.len() % 6 != 0 {
            return Err(format!(
                "{path}: geolocation-Grid unerwartet ({})",
                geo.len()
            ));
        }
        let mut gx: Vec<f64> = vec![];
        let mut gy: Vec<f64> = vec![];
        let mut glon = vec![];
        let mut glat = vec![];
        for rec in geo.chunks_exact(6) {
            let (i, j, lon, lat) = (rec[0], rec[1], rec[3], rec[4]);
            if !gx.contains(&i) {
                gx.push(i);
            }
            if !gy.contains(&j) {
                gy.push(j);
            }
            glon.push(lon);
            glat.push(lat);
        }
        gx.sort_by(|a, b| a.partial_cmp(b).unwrap());
        gy.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let grid_ni = gx.len();
        let grid_nj = gy.len();
        if grid_ni * grid_nj != geo.len() / 6 {
            return Err(format!(
                "{path}: Grid {}x{} != {} Recs",
                grid_ni,
                grid_nj,
                geo.len() / 6
            ));
        }
        let across = (width + tw - 1) / tw;
        Ok(Cog {
            path: path.to_string(),
            width,
            height,
            tw,
            th,
            tile_off,
            tile_len,
            across,
            grid_ni,
            grid_nj,
            gx,
            gy,
            glon,
            glat,
            tile_cache_idx: usize::MAX,
            tile_cache: vec![],
        })
    }

    fn tile(&mut self, idx: usize) -> Result<&Vec<u16>, String> {
        if idx == self.tile_cache_idx {
            return Ok(&self.tile_cache);
        }
        let off = self.tile_off[idx] as u64;
        let len = self.tile_len[idx] as usize;
        let mut raw = vec![0u8; len];
        let mut f = std::fs::File::open(&self.path).map_err(|e| format!("open: {e}"))?;
        f.seek(SeekFrom::Start(off))
            .map_err(|e| format!("seek {e}"))?;
        f.read_exact(&mut raw)
            .map_err(|e| format!("read tile {idx}: {e}"))?;
        let dec =
            zstd::stream::decode_all(raw.as_slice()).map_err(|e| format!("zstd {idx}: {e}"))?;
        if dec.len() < self.tw * self.th * 2 {
            return Err(format!(
                "tile {idx}: dekodiert {} < {}B",
                dec.len(),
                self.tw * self.th * 2
            ));
        }
        let mut px = Vec::with_capacity(self.tw * self.th);
        for c in dec[..self.tw * self.th * 2].chunks_exact(2) {
            px.push(u16::from_le_bytes([c[0], c[1]]));
        }
        self.tile_cache_idx = idx;
        self.tile_cache = px;
        Ok(&self.tile_cache)
    }

    fn at(&mut self, x: usize, y: usize) -> Result<u16, String> {
        if x >= self.width || y >= self.height {
            return Ok(0);
        }
        let tx = x / self.tw;
        let ty = y / self.th;
        let idx = ty * self.across + tx;
        let (sx, sy) = (x % self.tw, y % self.th);
        let tw = self.tw;
        let tile = self.tile(idx)?;
        Ok(tile[sy * tw + sx])
    }

    fn value_bilinear(&mut self, x: f64, y: f64) -> Result<f64, String> {
        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let (fx, fy) = (x - x0 as f64, y - y0 as f64);
        let (x1, y1) = ((x0 + 1).min(self.width - 1), (y0 + 1).min(self.height - 1));
        let v00 = self.at(x0, y0)? as f64;
        let v10 = self.at(x1, y0)? as f64;
        let v01 = self.at(x0, y1)? as f64;
        let v11 = self.at(x1, y1)? as f64;
        Ok((1.0 - fx) * (1.0 - fy) * v00
            + fx * (1.0 - fy) * v10
            + (1.0 - fx) * fy * v01
            + fx * fy * v11)
    }

    fn forward(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let mut ci = None;
        for k in 0..self.grid_ni - 1 {
            if x >= self.gx[k] && x <= self.gx[k + 1] {
                ci = Some(k);
                break;
            }
        }
        let mut cj = None;
        for k in 0..self.grid_nj - 1 {
            if y >= self.gy[k] && y <= self.gy[k + 1] {
                cj = Some(k);
                break;
            }
        }
        let (i, j) = (ci?, cj?);
        let u = (x - self.gx[i]) / (self.gx[i + 1] - self.gx[i]);
        let v = (y - self.gy[j]) / (self.gy[j + 1] - self.gy[j]);
        let g = |a: &Vec<f64>| -> f64 {
            let ll = a[j * self.grid_ni + i];
            let lr = a[j * self.grid_ni + (i + 1)];
            let ul = a[(j + 1) * self.grid_ni + i];
            let ur = a[(j + 1) * self.grid_ni + (i + 1)];
            ll + u * (lr - ll) + v * (ul - ll) + u * v * (ll - lr - ul + ur)
        };
        Some((g(&self.glon), g(&self.glat)))
    }

    fn inverse(&self, lon: f64, lat: f64) -> Option<(f64, f64)> {
        for j in 0..self.grid_nj - 1 {
            for i in 0..self.grid_ni - 1 {
                let glon = &self.glon;
                let glat = &self.glat;
                let ni = self.grid_ni;
                let bllon = glon[j * ni + i];
                let brlon = glon[j * ni + (i + 1)];
                let ullon = glon[(j + 1) * ni + i];
                let urlon = glon[(j + 1) * ni + (i + 1)];
                let bllat = glat[j * ni + i];
                let brlat = glat[j * ni + (i + 1)];
                let ullat = glat[(j + 1) * ni + i];
                let urlat = glat[(j + 1) * ni + (i + 1)];
                let lomin = bllon.min(brlon).min(ullon).min(urlon) - 1e-6;
                let lomax = bllon.max(brlon).max(ullon).max(urlon) + 1e-6;
                let lamin = bllat.min(brlat).min(ullat).min(urlat) - 1e-6;
                let lamax = bllat.max(brlat).max(ullat).max(urlat) + 1e-6;
                if !(lon >= lomin && lon <= lomax && lat >= lamin && lat <= lamax) {
                    continue;
                }
                let mut u = 0.5;
                let mut v = 0.5;
                for _ in 0..12 {
                    let l0 = bllon
                        + u * (brlon - bllon)
                        + v * (ullon - bllon)
                        + u * v * (bllon - brlon - ullon + urlon);
                    let a0 = bllat
                        + u * (brlat - bllat)
                        + v * (ullat - bllat)
                        + u * v * (bllat - brlat - ullat + urlat);
                    let dl_du = (brlon - bllon) + v * (bllon - brlon - ullon + urlon);
                    let dl_dv = (ullon - bllon) + u * (bllon - brlon - ullon + urlon);
                    let da_du = (brlat - bllat) + v * (bllat - brlat - ullat + urlat);
                    let da_dv = (ullat - bllat) + u * (bllat - brlat - ullat + urlat);
                    let det = dl_du * da_dv - dl_dv * da_du;
                    if det.abs() < 1e-15 {
                        break;
                    }
                    let dlon = lon - l0;
                    let dlat = lat - a0;
                    let du = (dlon * da_dv - dl_dv * dlat) / det;
                    let dv = (dl_du * dlat - dlon * da_du) / det;
                    u += du;
                    v += dv;
                    if du.abs() < 1e-9 && dv.abs() < 1e-9 {
                        break;
                    }
                }
                if (u >= -0.02 && u <= 1.02) && (v >= -0.02 && v <= 1.02) {
                    let x = self.gx[i] + u * (self.gx[i + 1] - self.gx[i]);
                    let y = self.gy[j] + v * (self.gy[j + 1] - self.gy[j]);
                    if let Some((lo, la)) = self.forward(x, y) {
                        if (lo - lon).abs() < 1e-6 && (la - lat).abs() < 1e-6 {
                            return Some((x, y));
                        }
                    }
                }
            }
        }
        None
    }
}

fn write_bin(pixels: &[Pixel]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + pixels.len() * 40);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(pixels.len() as u32).to_le_bytes());
    for p in pixels {
        for v in [p.lon, p.lat, p.post, p.vor, p.db] {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<Pixel>> {
    if bytes.len() < 8 || &bytes[..4] != &MAGIC {
        return None;
    }
    let count = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    if bytes.len() != 8 + count * 40 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for k in 0..count {
        let o = 8 + k * 40;
        let r = &bytes[o..o + 40];
        let mut vals = [0.0f64; 5];
        for (i, v) in vals.iter_mut().enumerate() {
            let b = &r[i * 8..i * 8 + 8];
            *v = f64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
        }
        out.push(Pixel {
            lon: vals[0],
            lat: vals[1],
            post: vals[2],
            vor: vals[3],
            db: vals[4],
        });
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "s1_sar_diff.bin".to_string());

    let mut lon0 = BBOX.0;
    let mut lon1 = BBOX.2;
    let mut lat0 = BBOX.1;
    let mut lat1 = BBOX.3;
    let mut step: f64 = 0.0015;
    if let Some(v) = arg_value(&args, "--lon0") {
        lon0 = v.parse().unwrap_or(lon0);
    }
    if let Some(v) = arg_value(&args, "--lon1") {
        lon1 = v.parse().unwrap_or(lon1);
    }
    if let Some(v) = arg_value(&args, "--lat0") {
        lat0 = v.parse().unwrap_or(lat0);
    }
    if let Some(v) = arg_value(&args, "--lat1") {
        lat1 = v.parse().unwrap_or(lat1);
    }
    if let Some(v) = arg_value(&args, "--step") {
        step = v.parse().unwrap_or(step);
    }

    let mut post_path = arg_value(&args, "--post-cog");
    let mut vor_path = arg_value(&args, "--vor-cog");

    if post_path.is_none() || vor_path.is_none() {
        eprintln!("=== S1-SAR-COG-Beschaffung (CI) ===");
        let tok = match sas_token("sentinel-1-grd") {
            Some(t) => t,
            None => {
                eprintln!("SAS-token pending — the kernel stays unwritten (0 honored)");
                std::process::exit(1);
            }
        };
        let dir = "/tmp/opencode/s1sar";
        std::fs::create_dir_all(dir).ok();
        if post_path.is_none() {
            let (id, href, dt) = match find_post() {
                Some(x) => x,
                None => {
                    eprintln!(
                        "post-scene: PENDING since {EVENT_EPOCH} — the kernel stays unwritten"
                    );
                    std::process::exit(1);
                }
            };
            eprintln!("Post-Scene: {id} @ {dt}");
            let path = format!("{dir}/post_{id}.tif");
            if !download(&format!("{href}?{tok}"), &path) {
                eprintln!("post-COG-download void — the kernel stays unwritten");
                std::process::exit(1);
            }
            post_path = Some(path);
        }
        if vor_path.is_none() {
            let post_id = post_path
                .as_ref()
                .and_then(|p| p.split('/').next_back())
                .unwrap_or("")
                .to_string();
            let (id, href, dt) = match find_vor(&post_id) {
                Some(x) => x,
                None => {
                    eprintln!(
                        "pre-scene: PENDING before {EVENT_EPOCH} — the kernel stays unwritten"
                    );
                    std::process::exit(1);
                }
            };
            eprintln!("Pre-Scene: {id} @ {dt}");
            let path = format!("{dir}/vor_{id}.tif");
            if !download(&format!("{href}?{tok}"), &path) {
                eprintln!("pre-COG-download void — the kernel stays unwritten");
                std::process::exit(1);
            }
            vor_path = Some(path);
        }
    }

    let (post_path, vor_path) = (post_path.unwrap(), vor_path.unwrap());
    let mut post = match Cog::open(&post_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Post-COG: {e}");
            std::process::exit(1);
        }
    };
    let mut vor = match Cog::open(&vor_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Vor-COG: {e}");
            std::process::exit(1);
        }
    };

    let nx = ((lon1 - lon0) / step).round() as usize + 1;
    let ny = ((lat1 - lat0) / step).round() as usize + 1;

    let mut pixels: Vec<Pixel> = Vec::new();
    let mut n_valid = 0usize;
    let mut sum_db = 0.0f64;
    let mut sum_vp = 0.0f64;
    let mut sum_vv = 0.0f64;
    let mut n_dark = 0usize;
    let mut n_bright = 0usize;
    let mut dark_min_lon = f64::MAX;
    let mut dark_max_lon = f64::MIN;
    let mut dark_min_lat = f64::MAX;
    let mut dark_max_lat = f64::MIN;
    let mut near_collapse_dark = 0usize;
    let mut verr = 0usize;

    eprintln!("=== SAR-Amplituden-Differenz Post-vs-Vor ===");
    eprintln!("Post: {post_path} ({}x{})", post.width, post.height);
    eprintln!("Vor:  {vor_path} ({}x{})", vor.width, vor.height);
    eprintln!(
        "BBox: lon {lon0:.4}..{lon1:.4}  lat {lat0:.4}..{lat1:.4}  step {step}°  grid {nx}x{ny}"
    );

    for j in 0..ny {
        let lat = lat0 + j as f64 * step;
        for i in 0..nx {
            let lon = lon0 + i as f64 * step;
            let xp = post.inverse(lon, lat);
            let xv = vor.inverse(lon, lat);
            if let (Some((px, py)), Some((vx, vy))) = (xp, xv) {
                if px >= 0.0
                    && py >= 0.0
                    && vx >= 0.0
                    && vy >= 0.0
                    && px < post.width as f64
                    && py < post.height as f64
                    && vx < vor.width as f64
                    && vy < vor.height as f64
                {
                    let vp = post.value_bilinear(px, py).unwrap_or(0.0);
                    let vv = match vor.value_bilinear(vx, vy) {
                        Ok(v) => v,
                        Err(e) => {
                            if verr < 3 {
                                eprintln!("vor-err @({vx:.1},{vy:.1}): {e}");
                            }
                            verr += 1;
                            0.0
                        }
                    };
                    if vp > 0.0 && vv > 0.0 {
                        let db = 20.0 * (vp / vv).log10();
                        pixels.push(Pixel {
                            lon,
                            lat,
                            post: vp,
                            vor: vv,
                            db,
                        });
                        n_valid += 1;
                        sum_db += db;
                        sum_vp += vp;
                        sum_vv += vv;
                        if db < -4.0 {
                            n_dark += 1;
                            dark_min_lon = dark_min_lon.min(lon);
                            dark_max_lon = dark_max_lon.max(lon);
                            dark_min_lat = dark_min_lat.min(lat);
                            dark_max_lat = dark_max_lat.max(lat);
                            if (lon - CP_LON).abs() < 0.02 && (lat - CP_LAT).abs() < 0.02 {
                                near_collapse_dark += 1;
                            }
                        } else if db > 4.0 {
                            n_bright += 1;
                        }
                    }
                }
            }
        }
    }

    if n_valid == 0 {
        eprintln!("no valid pixels in window — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }

    eprintln!(
        "valid pixels: {n_valid} ({:.1}% of window)",
        100.0 * n_valid as f64 / (nx * ny) as f64
    );
    eprintln!("post-amplitude mean: {:.1}", sum_vp / n_valid as f64);
    eprintln!("pre-amplitude  mean: {:.1}", sum_vv / n_valid as f64);
    eprintln!("mean dB change: {:+.2} dB", sum_db / n_valid as f64);
    eprintln!("darkening (dB < -4): {n_dark} pixels");
    if n_dark > 0 {
        eprintln!("   nahe Kollabpunkt (±0.02°): {near_collapse_dark}");
        eprintln!(
            "   Ausdehnung: lon {dark_min_lon:.4}..{dark_max_lon:.4}, lat {dark_min_lat:.4}..{dark_max_lat:.4}"
        );
        let w_km = (dark_max_lon - dark_min_lon) * 111.32 * (CP_LAT.to_radians().cos());
        let h_km = (dark_max_lat - dark_min_lat) * 111.32;
        eprintln!("   ~Footprint: {w_km:.1} km (W-O) x {h_km:.1} km (N-S)");
    }
    eprintln!("Aufhellung (dB > +4): {n_bright} Pixel");

    let bytes = write_bin(&pixels);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) if parsed.len() == pixels.len() => {
            eprintln!(
                "{out}: {} Pixel, {} B, roundtrip parses",
                pixels.len(),
                bytes.len()
            );
        }
        _ => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_RELEASE, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip() {
        let px = vec![
            Pixel {
                lon: 85.5,
                lat: 28.27,
                post: 1234.5,
                vor: 1200.0,
                db: 0.246,
            },
            Pixel {
                lon: 85.6,
                lat: 28.4,
                post: 1.0,
                vor: 2.0,
                db: -6.0206,
            },
        ];
        let bytes = write_bin(&px);
        let parsed = parse_bin(&bytes).expect("roundtrip");
        assert_eq!(parsed.len(), px.len());
        for (a, b) in parsed.iter().zip(px.iter()) {
            assert!((a.lon - b.lon).abs() < 1e-9);
            assert!((a.db - b.db).abs() < 1e-6);
        }
    }

    #[test]
    fn bin_rejects_junk() {
        assert!(parse_bin(b"xxxx").is_none());
        assert!(parse_bin(b"S1SR\x01\x00\x00\x00").is_none());
    }
}
