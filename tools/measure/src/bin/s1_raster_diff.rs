use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

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
    let (_, _, v) = tags.get(&tag).ok_or(format!("{path}: Tag {tag} absent"))?;
    Ok(*v as u16)
}

fn tag_u32s(
    path: &str,
    tags: &HashMap<u16, (u16, u32, u32)>,
    tag: u16,
) -> Result<Vec<u32>, String> {
    let (typ, cnt, off) = *tags.get(&tag).ok_or(format!("{path}: Tag {tag} absent"))?;
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
    let (typ, cnt, off) = *tags.get(&tag).ok_or(format!("{path}: Tag {tag} absent"))?;
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
        if width > 0 && gx.last().copied().unwrap_or(0.0) as usize + 1 != width {
            eprintln!(
                "{path}: gx-max {} != width {} (toleriert)",
                gx.last().copied().unwrap_or(0.0) as usize,
                width
            );
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: s1_raster_diff POST_COG VOR_COG [lon0 lon1 lat0 lat1] [step_deg]");
        std::process::exit(2);
    }
    let post = match Cog::open(&args[1]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Post-COG: {e}");
            std::process::exit(1);
        }
    };
    let vor = match Cog::open(&args[2]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Vor-COG: {e}");
            std::process::exit(1);
        }
    };
    for (name, c) in [("Post", &post), ("Vor", &vor)] {
        let (mut mlo, mut mla, mut xlo, mut xla) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
        for k in 0..c.grid_nj {
            for i in 0..c.grid_ni {
                let (lo, la) = (c.glon[k * c.grid_ni + i], c.glat[k * c.grid_ni + i]);
                mlo = mlo.min(lo);
                mla = mla.min(la);
                xlo = xlo.max(lo);
                xla = xla.max(la);
            }
        }
        println!(
            "{name}-COG Geogrid: lon {:.3}..{:.3}  lat {:.3}..{:.3}",
            mlo, xlo, mla, xla
        );
        let mut err = 0.0f64;
        let mut nchk = 0usize;
        for j in 0..c.grid_nj {
            for i in 0..c.grid_ni {
                if let Some((lo, la)) = c.forward(c.gx[i], c.gy[j]) {
                    let (rlo, rla) = (c.glon[j * c.grid_ni + i], c.glat[j * c.grid_ni + i]);
                    err = err.max((lo - rlo).abs()).max((la - rla).abs());
                    nchk += 1;
                }
            }
        }
        println!(
            "  forward-selftest: {nchk}/{} Tiepoints, max |err| = {err:.2e}°",
            c.grid_ni * c.grid_nj
        );
        let mut ierr = 0.0f64;
        let mut ihit = 0usize;
        for j in 0..c.grid_nj {
            for i in 0..c.grid_ni {
                if let Some((x, y)) =
                    c.inverse(c.glon[j * c.grid_ni + i], c.glat[j * c.grid_ni + i])
                {
                    ierr = ierr.max((x - c.gx[i]).abs()).max((y - c.gy[j]).abs());
                    ihit += 1;
                }
            }
        }
        println!(
            "  inverse-selftest: {ihit}/{} Tiepoints, max |err| = {ierr:.2e} px",
            c.grid_ni * c.grid_nj
        );
    }
    let (mut lon0, mut lon1, mut lat0, mut lat1) = (85.40, 85.60, 28.15, 28.40);
    if args.len() >= 7 {
        lon0 = args[3].parse().unwrap_or(lon0);
        lon1 = args[4].parse().unwrap_or(lon1);
        lat0 = args[5].parse().unwrap_or(lat0);
        lat1 = args[6].parse().unwrap_or(lat1);
    }
    let mut step: f64 = 0.0015;
    if args.len() >= 8 {
        step = args[7].parse().unwrap_or(step);
    }

    println!("=== SAR-Amplituden-Differenz Post-vs-Vor ===");
    println!("Post: {} ({}x{})", post.path, post.width, post.height);
    println!("Vor:  {} ({}x{})", vor.path, vor.width, vor.height);
    println!(
        "BBox: lon {:.4}..{:.4}  lat {:.4}..{:.4}  step {step}°",
        lon0, lon1, lat0, lat1
    );

    let mut post = post;
    let mut vor = vor;
    let nx = ((lon1 - lon0) / step).round() as usize + 1;
    let ny = ((lat1 - lat0) / step).round() as usize + 1;

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
    let (cp_lon, cp_lat) = (85.515, 28.271);
    let mut post_hit = 0usize;
    let mut vor_hit = 0usize;
    let mut perr = 0usize;
    let mut verr = 0usize;

    for j in 0..ny {
        let lat = lat0 + j as f64 * step;
        let mut row_chars: Vec<char> = Vec::with_capacity(nx);
        for i in 0..nx {
            let lon = lon0 + i as f64 * step;
            let xp = post.inverse(lon, lat);
            let xv = vor.inverse(lon, lat);
            if xp.is_some() {
                post_hit += 1;
            }
            if xv.is_some() {
                vor_hit += 1;
            }
            let mut ch = '.';
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
                    let vp = match post.value_bilinear(px, py) {
                        Ok(v) => v,
                        Err(_) => {
                            perr += 1;
                            0.0
                        }
                    };
                    let vv = match vor.value_bilinear(vx, vy) {
                        Ok(v) => v,
                        Err(e) => {
                            if verr < 3 {
                                eprintln!("vor-err @({:.1},{:.1}): {e}", vx, vy);
                            }
                            verr += 1;
                            0.0
                        }
                    };
                    if vp > 0.0 && vv > 0.0 {
                        let db = 20.0 * (vp / vv).log10();
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
                            if (lon - cp_lon).abs() < 0.02 && (lat - cp_lat).abs() < 0.02 {
                                near_collapse_dark += 1;
                            }
                            ch = 'D';
                        } else if db > 4.0 {
                            n_bright += 1;
                            ch = 'B';
                        } else {
                            ch = '=';
                        }
                    }
                }
            }
            row_chars.push(ch);
        }
        println!("  {}", row_chars.iter().collect::<String>());
    }

    println!("\n=== Befund ===");
    for (tag, lon, lat) in [
        ("SW", 85.40, 28.15),
        ("NE", 85.60, 28.40),
        ("Kollaps", 85.515, 28.271),
        ("Mitte", 85.50, 28.275),
    ] {
        let p = post.inverse(lon, lat);
        let v = vor.inverse(lon, lat);
        let (px, py) = match p {
            Some((a, b)) => (a, b),
            None => (f64::NAN, f64::NAN),
        };
        println!(
            "{tag} ({lon},{lat}): post=({:.2},{:.2}) val={} vor=({:.2},{:.2})",
            px,
            py,
            if px.is_finite() && py.is_finite() {
                post.value_bilinear(px, py).unwrap_or(-1.0)
            } else {
                -1.0
            },
            match v {
                Some((a, _)) => a,
                None => f64::NAN,
            },
            match v {
                Some((_, b)) => b,
                None => f64::NAN,
            },
        );
    }
    println!(
        "post.inverse hits: {post_hit}/{}, vor.inverse hits: {vor_hit}/{} (post-err {perr}, vor-err {verr})",
        nx * ny,
        nx * ny
    );
    if n_valid == 0 {
        println!("no valid pixels in window — bbox outside both scenes?");
        std::process::exit(0);
    }
    let mean_db = sum_db / n_valid as f64;
    let mean_vp = sum_vp / n_valid as f64;
    let mean_vv = sum_vv / n_valid as f64;
    println!(
        "valid pixels: {n_valid}  ({:.1}% of window)",
        100.0 * n_valid as f64 / (nx * ny) as f64
    );
    println!("post-amplitude mean: {mean_vp:.1}");
    println!("pre-amplitude  mean: {mean_vv:.1}");
    println!("mean dB change: {mean_db:+.2} dB");
    println!("darkening (dB < -4, new water/flat surface): {n_dark} pixels");
    if n_dark > 0 {
        println!("   of which near collapse point (±0.02°): {near_collapse_dark}");
        println!(
            "   darkened extent: lon {:.4}..{:.4}, lat {:.4}..{:.4}",
            dark_min_lon, dark_max_lon, dark_min_lat, dark_max_lat
        );
        let w_km = (dark_max_lon - dark_min_lon) * 111.32 * (cp_lat.to_radians().cos());
        let h_km = (dark_max_lat - dark_min_lat) * 111.32;
        println!("   ~footprint: {w_km:.1} km (W-E) x {h_km:.1} km (N-S)");
    }
    println!("brightening (dB > +4, bare/Bar-Scar): {n_bright} pixels");
    println!("Legend: D=darkening B=brightening ==unchanged .=outside/nodata");
}
