pub fn clip_trig_arg(x: f64) -> f64 {
    x.clamp(-1.0 + f64::EPSILON, 1.0 - f64::EPSILON)
}

pub fn wrap_360(x: f64) -> f64 {
    let mut w = x % 360.0;
    if w < 0.0 { w += 360.0; }
    w
}

pub fn wrap_180(x: f64) -> f64 {
    let mut w = x % 360.0;
    if w <= -180.0 { w += 360.0; }
    if w > 180.0 { w -= 360.0; }
    w
}

pub fn compensated_subtract(a: f64, b: f64) -> (f64, f64) {
    let x = a - b;
    let z = x - (a - b - x);
    (x, z)
}

pub fn angular_separation(lon1: f64, lat1: f64, lon2: f64, lat2: f64) -> f64 {
    let d_lon = lon2 - lon1;
    let num_y = (lat2.cos() * d_lon.sin()).hypot(
        lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * d_lon.cos()
    );
    let num_x = lat1.sin() * lat2.sin() + lat1.cos() * lat2.cos() * d_lon.cos();
    num_y.atan2(num_x)
}
