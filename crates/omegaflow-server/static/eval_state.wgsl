type df64 = vec2f;

fn df64_add(a: df64, b: df64) -> df64 {
    let s = a.x + b.x;
    let v = s - a.x;
    let err = (a.x - (s - v)) + (b.x - v) + a.y + b.y;
    return vec2f(s + err, err - (s + err - s));
}

fn df64_sub(a: df64, b: df64) -> df64 {
    let s = a.x - b.x;
    let v = s - a.x;
    let err = (a.x - (s - v)) - (b.x + v) + a.y - b.y;
    return vec2f(s + err, err - (s + err - s));
}

fn df64_mul(a: df64, b: df64) -> df64 {
    let p = a.x * b.x;
    let err = ((a.x * b.x - p) + a.y * b.x + a.x * b.y) + a.y * b.y;
    return vec2f(p + err, err - (p + err - p));
}

fn df64_scale(a: df64, s: f32) -> df64 {
    let p = a.x * s;
    let err = (a.x * s - p) + a.y * s;
    return vec2f(p + err, err - (p + err - p));
}

struct VP { 
    center_scale: vec4f, 
    res_count: vec4f, 
    observer_state: vec4f, 
    device_accel: vec4f, 
    device_mag: vec4f, 
    rotation: vec4f,
    device_local: vec4f,
    device_geo: vec4f
}

@group(0) @binding(0) var<storage, read> masses: array<vec4f>;
@group(0) @binding(1) var<uniform> vp: VP;
@group(0) @binding(2) var<storage, read> wmm: array<f32>;
@group(0) @binding(3) var terrain_raw: texture_2d<i32>;
@group(0) @binding(4) var egm96_tex: texture_2d<f32>;
@group(0) @binding(5) var camera_tex: texture_2d<f32>;
@group(0) @binding(6) var camera_sampler: sampler;

struct V { @builtin(position) p: vec4f, @location(0) u: vec2f }

@vertex fn vs(@builtin(vertex_index) i: u32) -> V {
    var p = array<vec2f, 3>(vec2f(-1.0, -1.0), vec2f(3.0, -1.0), vec2f(-1.0, 3.0));
    var o: V;
    o.p = vec4f(p[i], 0.0, 1.0);
    o.u = vec2f(p[i].x * 0.5 + 0.5, 0.5 - p[i].y * 0.5);
    return o;
}

fn eval_gravitational_state(pos: vec3f, capacity: f32) -> f32 {
    let mass_limit_f = 5.0 + capacity * 251.0;
    let current_mass_limit = i32(mass_limit_f);
    let limit_fade = 1.0 - fract(mass_limit_f);

    var acc = vec3f(0.0);
    for (var i: i32 = 0; i < current_mass_limit; i++) {
        let r_vec = masses[i].xyz - pos;
        let r = length(r_vec);
        let r_cubed = max(r * r * r, 1.0);
        let mass_effect = masses[i].w * r_vec / r_cubed;
        
        if (i == current_mass_limit - 1 && current_mass_limit > 0) {
            acc += mass_effect * limit_fade;
        } else {
            acc += mass_effect;
        }
    }
    acc += vp.device_accel.xyz;
    return length(acc);
}

fn eval_magnetic_state(pos: vec3f, earth_center: vec3f, sin_lat: f32, cos_lat: f32, lon_rad: f32, capacity: f32) -> vec3f {
    let r_vec = pos - earth_center;
    let r = length(r_vec);
    if (r < 6378137.0 * 0.9) { return vec3f(0.0); }

    let sin_theta = cos_lat;
    let cos_theta = sin_lat;
    let inv_sin_theta = 1.0 / max(sin_theta, 1e-6);
    let time_delta = wmm[3];
    let a_over_r = 6378137.0 / r;

    let mag_limit_f = 1.0 + capacity * 132.0;
    let current_mag_limit = i32(mag_limit_f);
    let limit_fade = 1.0 - fract(mag_limit_f);

    if (current_mag_limit <= 12) {
        var B_r = 0.0; var B_theta = 0.0; var B_phi = 0.0;
        for (var m: i32 = 0; m <= current_mag_limit; m++) {
            let cos_m_lon = cos(f32(m) * lon_rad);
            let sin_m_lon = sin(f32(m) * lon_rad);
            var p_pp = 0.0; var p_pr = 0.0; var p_cu = 0.0;
            var a_r_n = pow(a_over_r, f32(m + 2));
            for (var n: i32 = m; n <= current_mag_limit; n++) {
                if (n == m) {
                    if (m == 0) { p_cu = 1.0; } 
                    else { p_cu = sqrt(1.0 - 1.0 / (4.0 * f32(m) * f32(m))) * sin_theta * p_pr; }
                } else if (n == m + 1) {
                    p_cu = cos_theta * p_pr;
                } else {
                    p_cu = (f32(2*n - 1) * cos_theta * p_pr - f32(n + m - 1) * p_pp) / f32(n - m);
                }

                let idx = n * (n + 1) / 2 + m - 1;
                let coeff_idx = 4 + idx * 4;
                let g = wmm[coeff_idx]; let h = wmm[coeff_idx + 1];
                let g_svc = wmm[coeff_idx + 2]; let h_svc = wmm[coeff_idx + 3];
                let g_t = g + time_delta * g_svc; let h_t = h + time_delta * h_svc;

                let ch = g_t * cos_m_lon + h_t * sin_m_lon;
                let sh = g_t * sin_m_lon - h_t * cos_m_lon;
                
                let fade = select(limit_fade, 1.0, n < current_mag_limit);

                B_r += a_r_n * f32(n + 1) * p_cu * ch * fade;
                var dP = 0.0;
                if (n > m) { dP = (f32(n) * cos_theta * p_cu - f32(n + m) * p_pp) * inv_sin_theta; } 
                else { dP = f32(n) * cos_theta * p_cu * inv_sin_theta; }
                B_theta -= a_r_n * dP * ch * fade;
                B_phi += a_r_n * f32(m) * p_cu * sh * inv_sin_theta * fade;

                p_pp = p_pr; p_pr = p_cu;
                a_r_n *= a_over_r;
            }
        }
        return vec3f(
            B_r * sin_lat * cos(lon_rad) + B_theta * cos_lat * cos(lon_rad) - B_phi * sin(lon_rad),
            B_r * sin_lat * sin(lon_rad) + B_theta * cos_lat * sin(lon_rad) + B_phi * cos(lon_rad),
            B_r * cos_lat - B_theta * sin_lat
        );
    } else {
        var B_r = df64(0.0, 0.0); var B_theta = df64(0.0, 0.0); var B_phi = df64(0.0, 0.0);
        for (var m: i32 = 0; m <= current_mag_limit; m++) {
            let cos_m_lon = cos(f32(m) * lon_rad);
            let sin_m_lon = sin(f32(m) * lon_rad);
            var p_pp = df64(0.0, 0.0); var p_pr = df64(0.0, 0.0); var p_cu = df64(0.0, 0.0);
            var a_r_n = df64(pow(a_over_r, f32(m + 2)), 0.0);
            for (var n: i32 = m; n <= current_mag_limit; n++) {
                if (n == m) {
                    if (m == 0) { p_cu = df64(1.0, 0.0); } 
                    else { p_cu = df64_mul(df64_scale(p_pr, sqrt(1.0 - 1.0 / (4.0 * f32(m) * f32(m)))), df64(sin_theta, 0.0)); }
                } else if (n == m + 1) {
                    p_cu = df64_mul(p_pr, df64(cos_theta, 0.0));
                } else {
                    p_cu = df64_sub(df64_scale(df64_mul(p_pr, df64(cos_theta, 0.0)), f32(2*n - 1) / f32(n - m)), df64_scale(p_pp, f32(n + m - 1) / f32(n - m)));
                }

                let idx = n * (n + 1) / 2 + m - 1;
                let coeff_idx = 4 + idx * 4;
                let g = wmm[coeff_idx]; let h = wmm[coeff_idx + 1];
                let g_svc = wmm[coeff_idx + 2]; let h_svc = wmm[coeff_idx + 3];
                let g_t = g + time_delta * g_svc; let h_t = h + time_delta * h_svc;

                let ch = g_t * cos_m_lon + h_t * sin_m_lon;
                let sh = g_t * sin_m_lon - h_t * cos_m_lon;

                let fade = select(limit_fade, 1.0, n < current_mag_limit);

                B_r = df64_add(B_r, df64_scale(df64_mul(p_cu, df64(ch, 0.0)), a_r_n.x * f32(n + 1) * fade));
                var dP = df64(0.0, 0.0);
                if (n > m) { dP = df64_sub(df64_scale(df64_mul(p_cu, df64(cos_theta, 0.0)), f32(n) * inv_sin_theta), df64_scale(p_pp, f32(n + m) * inv_sin_theta)); } 
                else { dP = df64_scale(df64_mul(p_cu, df64(cos_theta, 0.0)), f32(n) * inv_sin_theta); }
                B_theta = df64_sub(B_theta, df64_scale(df64_mul(dP, df64(ch, 0.0)), a_r_n.x * fade));
                B_phi = df64_add(B_phi, df64_scale(df64_mul(p_cu, df64(sh, 0.0)), a_r_n.x * f32(m) * inv_sin_theta * fade));

                p_pp = p_pr; p_pr = p_cu;
                a_r_n = df64_scale(a_r_n, a_over_r);
            }
        }
        let b_r_f = B_r.x; let b_theta_f = B_theta.x; let b_phi_f = B_phi.x;
        return vec3f(
            b_r_f * sin_lat * cos(lon_rad) + b_theta_f * cos_lat * cos(lon_rad) - b_phi_f * sin(lon_rad),
            b_r_f * sin_lat * sin(lon_rad) + b_theta_f * cos_lat * sin(lon_rad) + b_phi_f * cos(lon_rad),
            b_r_f * cos_lat - b_theta_f * sin_lat
        );
    }
}

fn eval_terrain_height(pos: vec3f, earth_center: vec3f, r_hat: vec3f, dist: f32) -> f32 {
    let earth_radius = 6378137.0;
    let terrain_fade = smoothstep(earth_radius * 1.5, earth_radius, dist);
    if (terrain_fade <= 0.0) { return 0.0; }
    
    let lat = asin(r_hat.z);
    let lon = atan2(r_hat.y, r_hat.x);
    
    let lat0_deg = floor(lat * 57.2957795);
    let lon0_deg = floor(lon * 57.2957795);
    
    let local_lat = (lat * 57.2957795) - lat0_deg;
    let local_lon = (lon * 57.2957795) - lon0_deg;
    
    let x = (local_lon / 1.0) * 1200.0;
    let y = (1.0 - local_lat / 1.0) * 1200.0;
    
    let x0 = i32(clamp(floor(x), 0.0, 1199.0));
    let y0 = i32(clamp(floor(y), 0.0, 1199.0));
    let x1 = min(x0 + 1, 1200);
    let y1 = min(y0 + 1, 1200);
    
    let fx = x - f32(x0);
    let fy = y - f32(y0);
    
    let h00 = f32(textureLoad(terrain_raw, vec2u(x0, y0), 0).x);
    let h10 = f32(textureLoad(terrain_raw, vec2u(x1, y0), 0).x);
    let h01 = f32(textureLoad(terrain_raw, vec2u(x0, y1), 0).x);
    let h11 = f32(textureLoad(terrain_raw, vec2u(x1, y1), 0).x);
    
    let h = h00 * (1.0-fx) * (1.0-fy) + h10 * fx * (1.0-fy) + h01 * (1.0-fx) * fy + h11 * fx * fy;
    
    let egm96_u = (lon * 57.2957795 + 180.0) * 0.0027777777;
    let egm96_v = (lat * 57.2957795 + 90.0) * 0.0055555555;
    let undulation = textureSample(egm96_tex, camera_sampler, vec2f(egm96_u, egm96_v)).r;
    
    return (h + undulation) * terrain_fade;
}

@fragment fn fs(i: V) -> @location(0) vec4f {
    let w = vp.res_count.x;
    let h = vp.res_count.y;
    let scale = vp.center_scale.w;
    let yaw = vp.rotation.x;
    let pitch = vp.rotation.y;
    let acoustic_pressure = vp.device_local.x;
    let local_lux = vp.device_local.y;
    let temporal_certainty = vp.device_local.z;
    let locality_certainty = vp.device_local.w;
    let capacity = vp.observer_state.w;

    let cosY = cos(yaw); let sinY = sin(yaw);
    let cosP = cos(pitch); let sinP = sin(pitch);

    let offset = vec3f((i.u.x - 0.5) * w * scale, (i.u.y - 0.5) * h * scale, 0.0);
    let rotated_y = vec3f(offset.x * cosY + offset.z * sinY, offset.y, -offset.x * sinY + offset.z * cosY);
    let rotated = vec3f(rotated_y.x, rotated_y.y * cosP - rotated_y.z * sinP, rotated_y.y * sinP + rotated_y.z * cosP);

    let pos = vec3f(vp.center_scale.x, vp.center_scale.y, vp.center_scale.z) + rotated;

    let dwellTime = vp.observer_state.x;
    let motion = vp.observer_state.y;
    let lux = vp.observer_state.z;

    let earth_center = vec3f(wmm[0], wmm[1], wmm[2]);
    let r_vec = pos - earth_center;
    let dist = length(r_vec);
    let r_hat = r_vec / max(dist, 1.0);
    let sin_lat = r_hat.z;
    let cos_lat = sqrt(1.0 - sin_lat * sin_lat);
    let lon_rad = atan2(r_hat.y, r_hat.x);

    let noise = vec3f(sin(pos.x * 12.9898 + pos.y * 78.233), cos(pos.y * 43.758 + pos.z * 39.346), sin(pos.z * 23.456 + pos.x * 93.138));
    let total_disturbance = motion + acoustic_pressure * 10.0 + (1.0 - temporal_certainty) * 5.0 + (1.0 - locality_certainty) * 5.0;
    let noisy_pos = pos + noise * total_disturbance * scale * 0.01; 

    let g_omega = eval_gravitational_state(noisy_pos, capacity);
    let B_universe = eval_magnetic_state(noisy_pos, earth_center, sin_lat, cos_lat, lon_rad, capacity);
    let B_local = vp.device_mag.xyz;
    let total_B = length(B_universe + B_local * 1e-5);
    
    let total_lux = lux + local_lux * 100.0;
    let omega = max(0.0, g_omega - total_lux * 0.001);

    let certainty = temporal_certainty * locality_certainty;
    let luxCompensation = 1.0 / (1.0 + total_lux * 0.0001);

    let g_norm = clamp(omega / 9.81, 0.0, 10.0);
    let gravity_alpha = smoothstep(0.5, 5.0, g_norm) * capacity * certainty * luxCompensation;
    
    let t = clamp(g_norm / 5.0, 0.0, 1.0);
    let r_col = smoothstep(0.2, 0.6, t);
    let g_col = smoothstep(0.0, 0.3, t) * (1.0 - smoothstep(0.6, 0.8, t));
    let b_col = 1.0 - smoothstep(0.0, 0.4, t);
    let white_add = smoothstep(0.8, 1.0, t);
    let gravity_col = (vec3f(r_col, g_col, b_col) + white_add) * luxCompensation;

    let B_norm = clamp(total_B / 6.0e-5, 0.0, 1.0);
    let mag_brightness = smoothstep(0.1, 0.6, B_norm);
    let mag_glow = vec3f(0.1, 0.9, 1.0) * mag_brightness * capacity * certainty * luxCompensation * 0.8;

    let earth_radius = 6378137.0;
    let terrain_height = eval_terrain_height(pos, earth_center, r_hat, dist);
    let surface_dist = dist - earth_radius - terrain_height;
    let earth_atmo = smoothstep(10000.0, 0.0, surface_dist);
    var atmo_col = vec3f(0.1, 0.3, 0.8);
    var atmo_alpha = earth_atmo * capacity * certainty;

    let cam_rot = i32(vp.device_geo.w);
    var cam_uv = vec2f(i.u.x, 1.0 - i.u.y);
    if (cam_rot == 1) { cam_uv = vec2f(1.0 - i.u.y, i.u.x); }
    else if (cam_rot == 2) { cam_uv = vec2f(1.0 - i.u.x, i.u.y); }
    else if (cam_rot == 3) { cam_uv = vec2f(i.u.y, 1.0 - i.u.x); }
    let cam_sample = textureSample(camera_tex, camera_sampler, cam_uv).rgb;
    
    var cam_color = cam_sample;
    let cam_lum = dot(cam_sample, vec3f(0.299, 0.587, 0.114));
    if (cam_lum < 0.01) { cam_color = vec3f(0.02, 0.02, 0.05); }
    
    let cam_alpha = (1.0 - gravity_alpha - atmo_alpha) * certainty;

    let final_output = cam_color * cam_alpha + gravity_col * gravity_alpha + atmo_col * atmo_alpha + mag_glow;

    return vec4f(final_output, 1.0);
}

