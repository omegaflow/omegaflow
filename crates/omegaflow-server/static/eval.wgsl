struct state {
    pos: vec3f,
    acc_gravity: vec3f,
    B_field: vec3f,
    terrain_h: f32,
    dist_earth: f32,
    r_hat: vec3f,
    sin_lat: f32,
    cos_lat: f32,
    lon_rad: f32,
    earth_center: vec3f,
    potential: f32,
    time_dilation: f32,
}

fn eval_state(pos: vec3f, capacity: f32) -> state {
    var st: state;
    st.pos = pos;
    st.earth_center = vec3f(WMM(0), WMM(1), WMM(2));
    let r_vec = pos - st.earth_center;
    st.dist_earth = length(r_vec);
    st.r_hat = r_vec / max(st.dist_earth, 1.0);
    st.sin_lat = st.r_hat.z;
    st.cos_lat = sqrt(1.0 - st.sin_lat * st.sin_lat);
    st.lon_rad = atan2(st.r_hat.y, st.r_hat.x);

    let mass_limit = i32(capacity * 256.0);
    let mass_fade = 1.0 - fract(capacity * 256.0);
    st.acc_gravity = vec3f(0.0);
    st.potential = 0.0;
    for (var i:i32 = 0; i < mass_limit; i = i + 1) {
        let m = MASS(i);
        let r = m.xyz - pos;
        let rl = length(r);
        let r3 = max(dot(r, r) * rl, 1.0);
        let effect = m.w * r / r3;
        let phi = m.w / max(rl, 1.0);
        if (i == mass_limit - 1 && mass_limit > 0) {
            st.acc_gravity = st.acc_gravity + effect * mass_fade;
            st.potential = st.potential - phi * mass_fade;
        } else {
            st.acc_gravity = st.acc_gravity + effect;
            st.potential = st.potential - phi;
        }
    }
    let c = 299792458.0;
    st.time_dilation = sqrt(max(1.0 + 2.0 * st.potential / (c * c), 0.0));

    let mag_limit = min(i32(capacity * 133.0), n_max_raw);
    let mag_fade = 1.0 - fract(capacity * 133.0);
    let sin_theta = st.cos_lat;
    let cos_theta = st.sin_lat;
    let inv_sin_theta = 1.0 / max(sin_theta, 1e-6);
    let time_delta = WMM(3);
    let n_max_raw = i32(WMM(4));
    let a_over_r:f32 = 6378137.0 / st.dist_earth;

    if (mag_limit <= 12) {
        var B_r:f32 = 0.0; var B_t:f32 = 0.0; var B_p:f32 = 0.0;
        for (var mm:i32 = 0; mm <= mag_limit; mm = mm + 1) {
            let cml = cos(f32(mm) * st.lon_rad);
            let sml = sin(f32(mm) * st.lon_rad);
            var p_pp:f32 = 0.0; var p_pr:f32 = 0.0; var p_cu:f32 = 0.0;
            var arn:f32 = pow(a_over_r, f32(mm + 2));
            for (var n:i32 = mm; n <= mag_limit; n = n + 1) {
                if (n == mm) {
                    if (mm == 0) { p_cu = 1.0; }
                    else { p_cu = sqrt(1.0 - 1.0 / (4.0 * f32(mm) * f32(mm))) * sin_theta * p_pr; }
                } else if (n == mm + 1) {
                    p_cu = cos_theta * p_pr;
                } else {
                    p_cu = (f32(2*n-1) * cos_theta * p_pr - f32(n+mm-1) * p_pp) / f32(n-mm);
                }
                let ci = 5 + (n*(n+1)/2+mm-1)*4;
                let gt = WMM(ci) + time_delta * WMM(ci+2);
                let ht = WMM(ci+1) + time_delta * WMM(ci+3);
                let ch = gt*cml + ht*sml;
                let sh = gt*sml - ht*cml;
                let fd = select(mag_fade, 1.0, n < mag_limit);
                B_r = B_r + arn * f32(n+1) * p_cu * ch * fd;
                let dP = select(f32(n)*cos_theta*p_cu*inv_sin_theta, (f32(n)*cos_theta*p_cu - f32(n+mm)*p_pp)*inv_sin_theta, n > mm);
                B_t = B_t - arn * dP * ch * fd;
                B_p = B_p + arn * f32(mm) * p_cu * sh * inv_sin_theta * fd;
                p_pp = p_pr; p_pr = p_cu;
                arn = arn * a_over_r;
            }
        }
        st.B_field = vec3f(
            B_r*st.sin_lat*cos(st.lon_rad) + B_t*st.cos_lat*cos(st.lon_rad) - B_p*sin(st.lon_rad),
            B_r*st.sin_lat*sin(st.lon_rad) + B_t*st.cos_lat*sin(st.lon_rad) + B_p*cos(st.lon_rad),
            B_r*st.cos_lat - B_t*st.sin_lat
        );
    } else {
        var B_r:f32 = 0.0; var B_t:f32 = 0.0; var B_p:f32 = 0.0;
        for (var mm:i32 = 0; mm <= mag_limit; mm = mm + 1) {
            let cml = cos(f32(mm) * st.lon_rad);
            let sml = sin(f32(mm) * st.lon_rad);
            var p_pp:f32 = 0.0; var p_pr:f32 = 0.0; var p_cu:f32 = 0.0;
            var arn:f32 = pow(a_over_r, f32(mm+2));
            for (var n:i32 = mm; n <= mag_limit; n = n + 1) {
                if (n == mm) {
                    if (mm == 0) { p_cu = 1.0; }
                    else { p_cu = sqrt(1.0 - 1.0 / (4.0 * f32(mm) * f32(mm))) * sin_theta * p_pr; }
                } else if (n == mm + 1) {
                    p_cu = cos_theta * p_pr;
                } else {
                    p_cu = (f32(2*n-1) * cos_theta * p_pr - f32(n+mm-1) * p_pp) / f32(n-mm);
                }
                let ci = 5 + (n*(n+1)/2+mm-1)*4;
                let gt = WMM(ci) + time_delta * WMM(ci+2);
                let ht = WMM(ci+1) + time_delta * WMM(ci+3);
                let ch = gt*cml + ht*sml;
                let sh = gt*sml - ht*cml;
                let fd = select(mag_fade, 1.0, n < mag_limit);
                B_r = B_r + arn * f32(n+1) * p_cu * ch * fd;
                let dP = select(f32(n)*cos_theta*p_cu*inv_sin_theta, (f32(n)*cos_theta*p_cu - f32(n+mm)*p_pp)*inv_sin_theta, n > mm);
                B_t = B_t - arn * dP * ch * fd;
                B_p = B_p + arn * f32(mm) * p_cu * sh * inv_sin_theta * fd;
                p_pp = p_pr; p_pr = p_cu;
                arn = arn * a_over_r;
            }
        }
        st.B_field = vec3f(
            B_r*st.sin_lat*cos(st.lon_rad) + B_t*st.cos_lat*cos(st.lon_rad) - B_p*sin(st.lon_rad),
            B_r*st.sin_lat*sin(st.lon_rad) + B_t*st.cos_lat*sin(st.lon_rad) + B_p*cos(st.lon_rad),
            B_r*st.cos_lat - B_t*st.sin_lat
        );
    }

    let earth_radius = 6378137.0;
    let terrain_fade = smoothstep(earth_radius * 1.3, earth_radius, st.dist_earth);
    if (terrain_fade <= 0.0) {
        st.terrain_h = 0.0;
    } else {
        let lat = asin(st.r_hat.z);
        let lon = atan2(st.r_hat.y, st.r_hat.x);
        let x = fract(lon * 57.2957795) * 1200.0;
        let y = (1.0 - fract(lat * 57.2957795)) * 1200.0;
        let x0 = i32(clamp(floor(x), 0.0, 1199.0));
        let y0 = i32(clamp(floor(y), 0.0, 1199.0));
        let fx = x - f32(x0); let fy = y - f32(y0);
        let h = TERRAIN(x0,y0)*(1.0-fx)*(1.0-fy) + TERRAIN(min(x0+1,1200),y0)*fx*(1.0-fy)
                + TERRAIN(x0,min(y0+1,1200))*(1.0-fx)*fy + TERRAIN(min(x0+1,1200),min(y0+1,1200))*fx*fy;
        let undulation = EGM96((lon*57.2957795+180.0)*0.0027777777, (lat*57.2957795+90.0)*0.0055555555);
        st.terrain_h = (h + undulation) * terrain_fade;
    }

    return st;
}

fn eval_perception(uv: vec2f, res: vec2f, scale: f32, center: vec3f,
    rotation: vec2f, obs: vec4f, accel: vec4f, mag: vec4f, local: vec4f, geo: vec4f) -> vec3f {

    let capacity = obs.w;
    let dwell = obs.x;
    let motion = obs.y;
    let lux = obs.z;
    let acoustic = local.x;
    let camera_lux = local.y;
    let temporal_certainty = local.z;
    let spatial_certainty = local.w;

    let certainty = temporal_certainty * spatial_certainty;

    let cosY = cos(rotation.x); let sinY = sin(rotation.x);
    let cosP = cos(rotation.y); let sinP = sin(rotation.y);

    let offset = vec3f((uv.x - 0.5) * res.x * scale, (uv.y - 0.5) * res.y * scale, 0.0);
    let ry = vec3f(offset.x*cosY + offset.z*sinY, offset.y, -offset.x*sinY + offset.z*cosY);
    let pos = center + vec3f(ry.x, ry.y*cosP - ry.z*sinP, ry.y*sinP + ry.z*cosP);

    let st = eval_state(pos, capacity);

    let g = length(st.acc_gravity);
    let B = st.B_field + mag.xyz * 1e-5;
    let b = length(B);

    let g_energy = g / 9.81;
    let b_energy = b / 6.0e-5;

    let surface_dist = st.dist_earth - 6378137.0 - st.terrain_h;
    let atmo = clamp(1.0 - surface_dist / 100000.0, 0.0, 1.0);

    // camera is a photon sensor, not a display
    let cam_rot = i32(geo.w);
    var cam_uv = vec2f(uv.x, 1.0 - uv.y);
    if (cam_rot == 1) { cam_uv = vec2f(1.0 - uv.y, uv.x); }
    else if (cam_rot == 2) { cam_uv = vec2f(1.0 - uv.x, uv.y); }
    else if (cam_rot == 3) { cam_uv = vec2f(uv.y, 1.0 - uv.x); }
    let photon = CAMERA(cam_uv);

    // photon becomes measurement: intensity, frequency, direction
    let photon_intensity = dot(photon, vec3f(0.2126, 0.7152, 0.0722));
    let photon_r = photon.r / max(photon_intensity, 1e-6);
    let photon_b = photon.b / max(photon_intensity, 1e-6);

    // perception: what arrives at the observer through the field
    let g_perception = g_energy * capacity * certainty;
    let redshift = 1.0 - st.time_dilation;
    let gravity = vec3f(
        g_perception * (1.0 + redshift),
        g_perception * 0.4 * (1.0 - g_perception) * (1.0 - redshift * 0.5),
        (1.0 - g_perception * 0.6) * g_perception * (1.0 - redshift)
    );

    let magnetic = B / max(b, 1e-10) * b_energy * capacity * certainty;

    let atmosphere = vec3f(0.1, 0.3, 0.8) * atmo * capacity * certainty;

    // the observer sees the field + what the photon sensor measured
    let field_weight = length(gravity) + length(atmosphere);
    let photon_weight = max(0.0, 1.0 - field_weight) * certainty;

    return photon * photon_weight + gravity + atmosphere + magnetic;
}
