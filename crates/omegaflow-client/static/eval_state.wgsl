override HARDWARE_TIER: i32 = 0;
override N_MAX: i32 = 12; 
override LEGENDRE_ARRAY_SIZE: i32 = 105;

struct VP { center_scale: vec4f, res_count: vec4f }

@group(0) @binding(0) var<storage, read> masses: array<vec4f>;
@group(0) @binding(1) var<uniform> vp: VP;
@group(0) @binding(2) var<storage, read> wmm: array<f32>;

struct V { @builtin(position) p: vec4f, @location(0) u: vec2f }

@vertex fn vs(@builtin(vertex_index) i: u32) -> V {
    var p = array<vec2f, 3>(vec2f(-1.0, -1.0), vec2f(3.0, -1.0), vec2f(-1.0, 3.0));
    var o: V;
    o.p = vec4f(p[i], 0.0, 1.0);
    o.u = vec2f(p[i].x * 0.5 + 0.5, 0.5 - p[i].y * 0.5);
    return o;
}

fn eval_gravitational_state(pos: vec3f) -> f32 {
    var acc = vec3f(0.0);
    for (var i: i32 = 0; i < i32(vp.res_count.z); i++) {
        let r_vec = masses[i].xyz - pos;
        let r2_s = max(dot(r_vec, r_vec), 1.0);
        acc += masses[i].w * r_vec / (r2_s * sqrt(r2_s));
    }
    return length(acc);
}

fn read_icrf_to_ecef() -> mat3x3<f32> {
    let wmm_coeffs = N_MAX * (N_MAX + 3) / 2;
    let b: i32 = 4 + 4 * wmm_coeffs;
    return mat3x3<f32>(
        vec3f(wmm[b+0], wmm[b+3], wmm[b+6]),
        vec3f(wmm[b+1], wmm[b+4], wmm[b+7]),
        vec3f(wmm[b+2], wmm[b+5], wmm[b+8])
    );
}

fn schmidt_factor(n: i32, m: i32) -> f32 {
    if (m == 0) { return 1.0; }
    var prod: f32 = 1.0;
    for (var k: i32 = 1; k <= 2 * m; k++) { prod *= f32(n - m + k); }
    return sqrt(2.0 / prod);
}

fn eval_magnetic_state(pos: vec3f) -> vec3f {
    let wmm_coeffs = N_MAX * (N_MAX + 3) / 2;
    let required_size = 4 + 4 * wmm_coeffs + 9;
    if (i32(arrayLength(&wmm)) < required_size) { return vec3f(0.0); }

    let rel_icrf = pos - vec3f(wmm[0], wmm[1], wmm[2]);
    let R = read_icrf_to_ecef();
    let rel_ecef = R * rel_icrf;
    let r = max(length(rel_ecef), 1.0);

    let sin_phi = clamp(rel_ecef.z / r, -0.9999, 0.9999);
    let cos_phi = sqrt(1.0 - sin_phi * sin_phi);
    let cos_phi_s = max(cos_phi, 1e-7);
    let lam = atan2(rel_ecef.y, rel_ecef.x);
    let td = wmm[3];

    var P: array<f32, LEGENDRE_ARRAY_SIZE>;
    for (var i: i32 = 0; i < LEGENDRE_ARRAY_SIZE; i++) { P[i] = 0.0; }

    P[0] = 1.0;
    if (N_MAX >= 1) { P[2] = sin_phi; P[3] = cos_phi; }

    for (var n: i32 = 2; n <= N_MAX + 1; n++) {
        let idx_nn = n * (n + 1) / 2 + n;
        let idx_nn1 = (n - 1) * n / 2 + (n - 1);
        P[idx_nn] = cos_phi * f32(2 * n - 1) * P[idx_nn1];

        let idx_nm1 = n * (n + 1) / 2 + (n - 1);
        P[idx_nm1] = sin_phi * f32(2 * n - 1) * P[idx_nn1];

        for (var m: i32 = 0; m <= n - 2; m++) {
            let idx_nm = n * (n + 1) / 2 + m;
            let idx_n1m = (n - 1) * n / 2 + m;
            let idx_n2m = (n - 2) * (n - 1) / 2 + m;
            P[idx_nm] = (sin_phi * f32(2 * n - 1) * P[idx_n1m] - f32(n + m - 1) * P[idx_n2m]) / f32(n - m);
        }
    }

    let a = 6371200.0;
    var B_north: f32 = 0.0;
    var B_east: f32 = 0.0;
    var B_down: f32 = 0.0;

    for (var n: i32 = 1; n <= N_MAX; n++) {
        var xn: f32 = 0.0; var yn: f32 = 0.0; var zn: f32 = 0.0;
        for (var m: i32 = 0; m <= n; m++) {
            let ci = n * (n + 1) / 2 + m - 1;
            let g_t = wmm[4 + ci] + td * wmm[4 + wmm_coeffs + ci];
            let h_t = wmm[4 + 2 * wmm_coeffs + ci] + td * wmm[4 + 3 * wmm_coeffs + ci];

            let ml = f32(m) * lam;
            let cosm = cos(ml);
            let sinm = sin(ml);

            let idx_nm = n * (n + 1) / 2 + m;
            let idx_n1m = (n + 1) * (n + 2) / 2 + m;
            let S = P[idx_nm] * schmidt_factor(n, m);
            let S_next = P[idx_n1m] * schmidt_factor(n + 1, m);

            let dS = f32(n + 1) * sin_phi / cos_phi_s * S
                   - sqrt(f32((n + 1) * (n + 1) - m * m)) / cos_phi_s * S_next;

            let c = g_t * cosm + h_t * sinm;
            let s = g_t * sinm - h_t * cosm;

            xn += c * dS;
            yn += f32(m) * s * S;
            zn += c * S;
        }
        let k = pow(a / r, f32(n + 2));
        B_north -= k * xn;
        B_east  += k * yn;
        B_down  -= f32(n + 1) * k * zn;
    }

    let r_hat = normalize(rel_ecef);
    let east_u = normalize(cross(vec3f(0.0, 0.0, 1.0), r_hat));
    let north_u = cross(east_u, r_hat);
    let down_u = -r_hat;

    let B_ecef = B_north * north_u + B_east * east_u + B_down * down_u;
    return transpose(R) * B_ecef;
}

@fragment fn fs(i: V) -> @location(0) vec4f {
    let w = vp.res_count.x;
    let h = vp.res_count.y;
    let scale = vp.center_scale.w;
    let pos = vec3f(vp.center_scale.x + (i.u.x - 0.5) * w * scale, vp.center_scale.y - (i.u.y - 0.5) * h * scale, vp.center_scale.z);

    var omega: f32 = 0.0;
    var flow: vec3f = vec3f(0.0);

    omega += eval_gravitational_state(pos);
    let B = eval_magnetic_state(pos);
    omega += length(B);
    flow += B;

    let brightness = clamp(log(1.0 + omega / 9.81) / log(11.0), 0.0, 1.0);
    let B_norm = clamp(length(B) / 65000.0, 0.0, 1.0);
    let g_norm = clamp(eval_gravitational_state(pos) / 9.81, 0.0, 1.0);

    let grav_col = mix(vec3f(0.0, 0.1, 0.8), vec3f(1.0, 0.9, 0.0), g_norm * g_norm);
    let mag_col = vec3f(B_norm, 0.0, B_norm * 0.7);
    let final_col = mix(grav_col, mag_col, B_norm * 0.6);

    return vec4f(final_col * brightness, 1.0);
}

