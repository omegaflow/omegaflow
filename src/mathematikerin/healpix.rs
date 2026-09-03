use std::f64::consts::PI;

const JRLL: [i64; 12] = [2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
const JPLL: [i64; 12] = [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7];

const GAL2EQ: [[f64; 3]; 3] = [
    [-0.0548755604162154, 0.4941094278755837, -0.8676661490190047],
    [
        -0.8734370902348850,
        -0.4448296299600112,
        -0.1980763734312015,
    ],
    [-0.4838350155487132, 0.7469822444972189, 0.4559837761750669],
];

pub fn nside_from_npix(npix: i64) -> Option<i64> {
    if npix <= 0 || npix % 12 != 0 {
        return None;
    }
    let n = npix / 12;
    let s = isqrt(n);
    if s * s == n && s > 0 && (s & (s - 1)) == 0 {
        Some(s)
    } else {
        None
    }
}

fn isqrt(x: i64) -> i64 {
    let mut r = (x as f64).sqrt() as i64;
    while (r + 1) * (r + 1) <= x {
        r += 1;
    }
    while r * r > x {
        r -= 1;
    }
    r
}

fn compress_bits(v: u64) -> u64 {
    let mut r = 0u64;
    for i in 0..32 {
        r |= ((v >> (2 * i)) & 1) << i;
    }
    r
}

pub fn pix2ang_nest(nside: i64, pix: i64) -> Option<(f64, f64)> {
    let npix = 12 * nside * nside;
    if pix < 0 || pix >= npix {
        return None;
    }
    let order = nside.trailing_zeros() as u32;
    let npface = nside * nside;
    let face = (pix >> (2 * order)) as usize;
    let ipf = (pix & (npface - 1)) as u64;
    let ix = compress_bits(ipf) as i64;
    let iy = compress_bits(ipf >> 1) as i64;
    let jr = (JRLL[face] << order) - ix - iy - 1;
    let n = nside as f64;
    let fact1 = 2.0 / (3.0 * n);
    let fact2 = 1.0 / (3.0 * n * n);
    let (nr, z) = if jr < nside {
        let nr = jr;
        (nr, 1.0 - (nr as f64) * (nr as f64) * fact2)
    } else if jr > 3 * nside {
        let nr = 4 * nside - jr;
        (nr, (nr as f64) * (nr as f64) * fact2 - 1.0)
    } else {
        (nside, (2 * nside - jr) as f64 * fact1)
    };
    let mut tmp = JPLL[face] * nr + ix - iy;
    if tmp < 0 {
        tmp += 8 * nr;
    }
    let phi = if nr == nside {
        (tmp as f64) * PI / (4.0 * n)
    } else {
        (tmp as f64) * PI / (4.0 * nr as f64)
    };
    let theta = z.clamp(-1.0, 1.0).acos();
    Some((theta, phi))
}

pub fn galactic_to_icrs(theta: f64, phi: f64) -> (f64, f64) {
    let (st, ct) = theta.sin_cos();
    let (sp, cp) = phi.sin_cos();
    let xg = [st * cp, st * sp, ct];
    let xe = [
        GAL2EQ[0][0] * xg[0] + GAL2EQ[0][1] * xg[1] + GAL2EQ[0][2] * xg[2],
        GAL2EQ[1][0] * xg[0] + GAL2EQ[1][1] * xg[1] + GAL2EQ[1][2] * xg[2],
        GAL2EQ[2][0] * xg[0] + GAL2EQ[2][1] * xg[1] + GAL2EQ[2][2] * xg[2],
    ];
    let ra = xe[1].atan2(xe[0]).rem_euclid(2.0 * PI);
    let dec = xe[2].clamp(-1.0, 1.0).asin();
    (ra.to_degrees(), dec.to_degrees())
}

pub fn icrs_to_galactic(ra_deg: f64, dec_deg: f64) -> (f64, f64) {
    let (sd, cd) = dec_deg.to_radians().sin_cos();
    let (sr, cr) = ra_deg.to_radians().sin_cos();
    let xe = [cd * cr, cd * sr, sd];
    let xg = [
        GAL2EQ[0][0] * xe[0] + GAL2EQ[1][0] * xe[1] + GAL2EQ[2][0] * xe[2],
        GAL2EQ[0][1] * xe[0] + GAL2EQ[1][1] * xe[1] + GAL2EQ[2][1] * xe[2],
        GAL2EQ[0][2] * xe[0] + GAL2EQ[1][2] * xe[1] + GAL2EQ[2][2] * xe[2],
    ];
    let theta = xg[2].clamp(-1.0, 1.0).acos();
    let phi = xg[1].atan2(xg[0]).rem_euclid(2.0 * PI);
    (theta, phi)
}

fn spread_bits(v: i64, order: u32) -> i64 {
    let mut r = 0i64;
    for i in 0..order {
        r |= ((v >> i) & 1) << (2 * i);
    }
    r
}

fn xyf2nest(ix: i64, iy: i64, face: i64, order: u32) -> i64 {
    (face << (2 * order)) + spread_bits(ix, order) + (spread_bits(iy, order) << 1)
}

pub fn ang2pix_nest(nside: i64, theta: f64, phi: f64) -> Option<i64> {
    let z = theta.cos();
    let za = z.abs();
    let order = nside.trailing_zeros() as u32;
    let n = nside as f64;
    let tt = (phi * (2.0 / PI)).rem_euclid(4.0);
    let pix = if za <= 2.0 / 3.0 {
        let temp1 = n * (0.5 + tt);
        let temp2 = n * (z * 0.75);
        let jp = (temp1 - temp2) as i64;
        let jm = (temp1 + temp2) as i64;
        let ifp = jp >> order;
        let ifm = jm >> order;
        let face = if ifp == ifm {
            ifp | 4
        } else if ifp < ifm {
            ifp
        } else {
            ifm + 8
        };
        let ix = jm & (nside - 1);
        let iy = nside - (jp & (nside - 1)) - 1;
        xyf2nest(ix, iy, face, order)
    } else {
        let ntt = tt as i64;
        let tp = tt - ntt as f64;
        let tmp = n * (3.0 * (1.0 - za)).sqrt();
        let jp = ((tp * tmp) as i64).min(nside - 1);
        let jm = (((1.0 - tp) * tmp) as i64).min(nside - 1);
        if z >= 0.0 {
            xyf2nest(nside - jm - 1, nside - jp - 1, ntt, order)
        } else {
            xyf2nest(jp, jm, ntt + 8, order)
        }
    };
    if pix < 0 || pix >= 12 * nside * nside {
        None
    } else {
        Some(pix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn nside_from_npix_roundtrip() {
        assert_eq!(nside_from_npix(12), Some(1));
        assert_eq!(nside_from_npix(48), Some(2));
        assert_eq!(nside_from_npix(12 * 2048 * 2048), Some(2048));
        assert_eq!(nside_from_npix(13), None);
        assert_eq!(nside_from_npix(24), None);
    }

    #[test]
    fn nested_nside1_faces() {
        let n = 1;
        let faces: Vec<(f64, f64)> = (0..12).map(|p| pix2ang_nest(n, p).unwrap()).collect();
        assert!(near(faces[0].1, PI / 4.0, 1e-12));
        assert!(near(faces[0].0, (2.0f64 / 3.0).acos(), 1e-12));
        assert!(near(faces[4].1, 0.0, 1e-12));
        assert!(near(faces[4].0, PI / 2.0, 1e-12));
        assert!(near(faces[8].1, PI / 4.0, 1e-12));
        assert!(near(faces[8].0, PI - (2.0f64 / 3.0).acos(), 1e-12));
    }

    #[test]
    fn nested_nside64_pole_and_bounds() {
        let n = 64;
        let (theta, _) = pix2ang_nest(n, 0).unwrap();
        assert!(
            (theta - PI / 2.0).abs() < 0.03,
            "pixel 0 sits at the face's equatorial corner"
        );
        let (theta, _) = pix2ang_nest(n, n * n - 1).unwrap();
        assert!(theta < 0.02, "face 0 top corner sits at the north pole");
        let (theta, _) = pix2ang_nest(n, 11 * n * n).unwrap();
        assert!(
            theta > PI - 0.02,
            "face 11 bottom corner sits at the south pole"
        );
    }

    #[test]
    fn galactic_center_and_pole() {
        let (ra, dec) = galactic_to_icrs(PI / 2.0, 0.0);
        assert!(near(ra, 266.4051, 1e-3));
        assert!(near(dec, -28.9362, 1e-3));
        let (ra, dec) = galactic_to_icrs(0.0, 0.0);
        assert!(near(ra, 192.85948, 1e-3));
        assert!(near(dec, 27.12825, 1e-3));
    }

    #[test]
    fn galactic_icrs_roundtrip() {
        let (theta, phi) = (0.7, 2.4);
        let (ra, dec) = galactic_to_icrs(theta, phi);
        let (t2, p2) = icrs_to_galactic(ra, dec);
        assert!(near(t2, theta, 1e-9));
        assert!(near(p2, phi, 1e-9));
    }

    #[test]
    fn ang2pix_roundtrip_all_nside() {
        for n in [1i64, 2, 4, 8] {
            for p in 0..(12 * n * n) {
                let (theta, phi) = pix2ang_nest(n, p).unwrap();
                let back = ang2pix_nest(n, theta, phi).unwrap();
                assert_eq!(back, p, "nside {n} pix {p}");
            }
        }
    }

    #[test]
    fn ang2pix_roundtrip_sample_nside64() {
        let n = 64;
        let mut rng = 0x1234_5678_9abc_def0u64;
        for _ in 0..2000 {
            rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let p = (rng % ((12 * n * n) as u64)) as i64;
            let (theta, phi) = pix2ang_nest(n, p).unwrap();
            let back = ang2pix_nest(n, theta, phi).unwrap();
            assert_eq!(back, p);
        }
    }
}
