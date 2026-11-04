use crate::archivar::fits::{FitsHeader, FitsImage};
use crate::mathematikerin::healpix::galactic_to_icrs;

pub const FUGIN_CUBE_NAXIS: i64 = 3;
pub const FUGIN_VELOCITY_CTYPE: &str = "VRAD";
pub const FUGIN_MOMENT0_OUTPUT_PIXEL_CAP: usize = 1 << 21;

#[derive(Clone, Debug)]
pub struct FuginPixel {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub moment0_k_ms: f64,
}

pub fn parse_fugin_cube(buf: &[u8]) -> Option<Vec<FuginPixel>> {
    let (header, _) = FitsHeader::parse(buf, 0)?;
    if header.int("NAXIS")? != FUGIN_CUBE_NAXIS {
        return None;
    }
    if !header
        .str_unescaped("CTYPE3")?
        .contains(FUGIN_VELOCITY_CTYPE)
    {
        return None;
    }
    let delta_v_ms = header.f64("CDELT3").filter(|v| v.is_finite() && *v > 0.0)?;
    let (image, _) = FitsImage::parse(buf, 0)?;
    let nx = image.dims[0];
    let ny = image.dims[1];
    let nz = image.dims[2];
    let n_spatial = nx.checked_mul(ny)?;
    if n_spatial > FUGIN_MOMENT0_OUTPUT_PIXEL_CAP {
        return None;
    }
    let mut pixels = Vec::with_capacity(n_spatial);
    for y in 0..ny {
        for x in 0..nx {
            let mut sum_k = 0.0;
            let mut finite = 0u32;
            for z in 0..nz {
                if let Some(v) = image.value_f64(buf, [x, y, z])
                    && v.is_finite()
                {
                    sum_k += v;
                    finite += 1;
                }
            }
            if finite == 0 {
                continue;
            }
            let Some((glon_deg, glat_deg)) = image.world(x as f64 + 1.0, y as f64 + 1.0) else {
                continue;
            };
            let theta = (90.0 - glat_deg).to_radians();
            let phi = glon_deg.to_radians();
            let (ra_deg, dec_deg) = galactic_to_icrs(theta, phi);
            pixels.push(FuginPixel {
                ra_deg,
                dec_deg,
                moment0_k_ms: sum_k * delta_v_ms,
            });
        }
    }
    Some(pixels)
}
