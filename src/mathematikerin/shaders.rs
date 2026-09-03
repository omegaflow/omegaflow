pub const FIELD_WGSL: &str = r#"
struct VP { surface: vec4f, right: vec4f, up: vec4f, forward: vec4f, expose_ex: vec4f, presence: vec4f };
const C_VACUUM: f32 = 299792458.0;
const AUDIO_SPEED_AIR: f32 = 343.0;
const PROPAGATION_SPEED: array<f32, 9> = array<f32, 9>(
    C_VACUUM,
    C_VACUUM,
    AUDIO_SPEED_AIR,
    6000.0,
    3000.0,
    0.3,
    0.05,
    1.0,
    C_VACUUM,
);

@group(0) @binding(0) var<storage, read> field: array<vec4f>;
@group(0) @binding(1) var<storage, read> props: array<vec4f>;
@group(0) @binding(2) var<uniform> vp: VP;
@group(0) @binding(3) var<storage, read_write> probe_out: array<f32>;
@group(0) @binding(4) var<storage, read_write> pp: array<vec4f>;

fn erfc(x: f32) -> f32 {
    let xa = abs(x);
    let t = 1.0 / (1.0 + 0.3275911 * xa);
    let poly = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let y = poly * exp(-xa * xa);
    return select(y, 2.0 - y, x < 0.0);
}

fn field_spatial(d2: f32, d_mag: f32, extent: f32, kernel_id: u32, global_scale: f32, absorption: f32) -> f32 {
    let perceptual_extent = max(extent, global_scale);
    let e2 = max(perceptual_extent * perceptual_extent, 1e-30);
    let s2 = max(global_scale * global_scale, 1e-30);

    if (kernel_id == 0u) {
        return 1.0 / (d2 + e2);
    } else if (kernel_id == 1u) {
        return exp(-d2 / (2.0 * e2)) / (d2 + e2);
    } else if (kernel_id == 3u) {
        return erfc(d_mag / max(perceptual_extent * sqrt(2.0), global_scale));
    } else if (kernel_id == 2u) {
        return exp(-d2 / (2.0 * max(e2, s2))) / (d_mag + sqrt(s2));
    } else if (kernel_id == 4u) {
        return exp(-d_mag / max(perceptual_extent, global_scale));
    } else if (kernel_id == 5u) {
        let alpha = clamp(absorption, 0.0, 1.0);
        let beta = 1.6;
        let core = exp(-d2 / (2.0 * max(e2, s2)));
        let tail = pow(max(e2, s2), beta * 0.5) / pow(max(d2 + s2, s2), beta * 0.5);
        return (1.0 - alpha) * core + alpha * tail;
    } else if (kernel_id == 6u) {
        return 1.0 / (d_mag + perceptual_extent);
    } else {
        return exp(-d2 / (2.0 * max(e2, s2))) / max(d2 + s2, s2);
    }
}

fn field_spatial_grad(d2: f32, d_mag: f32, extent: f32, kernel_id: u32, global_scale: f32, absorption: f32) -> f32 {
    if (d_mag > 1e13) {
        return 0.0;
    }
    let perceptual_extent = max(extent, global_scale);
    let e2 = max(perceptual_extent * perceptual_extent, 1e-30);
    let s2 = max(global_scale * global_scale, 1e-30);
    let d = max(d_mag, 1e-9);

    if (kernel_id == 0u) {
        let denom = d2 + e2;
        return -2.0 * d / (denom * denom);
    } else if (kernel_id == 1u) {
        let denom = d2 + e2;
        return -exp(-d2 / (2.0 * e2)) * d * (denom / e2 + 2.0) / (denom * denom);
    } else if (kernel_id == 3u) {
        let scale = max(perceptual_extent * sqrt(2.0), global_scale);
        return -2.0 / sqrt(3.141592653589793) * exp(-d2 / (scale * scale)) / scale;
    } else if (kernel_id == 2u) {
        let e = max(e2, s2);
        let denom = d + sqrt(s2);
        return -exp(-d2 / (2.0 * e)) * (d / e * denom + 1.0) / (denom * denom);
    } else if (kernel_id == 4u) {
        let scale = max(perceptual_extent, global_scale);
        return -exp(-d / scale) / scale;
    } else if (kernel_id == 5u) {
        let alpha = clamp(absorption, 0.0, 1.0);
        let beta = 1.6;
        let e = max(e2, s2);
        let core_grad = -d / e * exp(-d2 / (2.0 * e));
        let denom = max(d2 + s2, s2);
        let tail_grad = -beta * d * pow(e, beta * 0.5) / pow(denom, beta * 0.5 + 1.0);
        return (1.0 - alpha) * core_grad + alpha * tail_grad;
    } else if (kernel_id == 6u) {
        let denom = d_mag + perceptual_extent;
        return -1.0 / (denom * denom);
    } else {
        let e = max(e2, s2);
        let denom = max(d2 + s2, s2);
        return -exp(-d2 / (2.0 * e)) * (d / e * denom + 2.0 * d) / (denom * denom);
    }
}

fn val_eff_at(pre: vec4f, tm: vec4f, ft: u32, v: f32, d_mag: f32) -> f32 {
    let temporal = abs(vp.presence.w - tm.x);
    var val = pre.w;
    let corr = select(min(temporal, d_mag / v), 0.0, v == 0.0 || d_mag == 0.0);
    if (corr > tm.y * 1e-4) {
        val = pre.w * exp(corr / max(tm.y, 1e-9));
    }
    return val;
}

fn val_eff_grad(pre: vec4f, tm: vec4f, v: f32, d_mag: f32) -> f32 {
    if (v <= 0.0 || d_mag <= 0.0) {
        return 0.0;
    }
    let temporal = abs(vp.presence.w - tm.x);
    let corr = d_mag / v;
    if (corr >= temporal || corr <= tm.y * 1e-4) {
        return 0.0;
    }
    return pre.w * exp(corr / max(tm.y, 1e-9)) / max(tm.y, 1e-9) / v;
}

fn osc_field(j: u32, rel: vec3f, pre: vec4f) -> vec2f {
    let tm = field[j * 3u + 1u];
    let fm = field[j * 3u + 2u];
    let mt = props[j * 4u];
    let delta = pre.xyz - rel;
    let d2 = dot(delta, delta);
    let d_mag = sqrt(d2);
    let sd = dot(pre.xyz, vp.forward.xyz);
    let t2 = max(d2 - sd * sd, 0.0);
    let t_mag = sqrt(t2);
    let ft = u32(tm.z);
    let kid = u32(mt.z);
    let v = select(PROPAGATION_SPEED[ft], fm.x, ft == 7u && fm.x > 0.0);
    var val_eff = val_eff_at(pre, tm, ft, v, d_mag);
    if (ft == 0u && mt.w > 0.0) {
        let z1 = 1.0 + mt.w;
        val_eff = val_eff / (z1 * z1 * z1 * z1);
    }
    var sk = field_spatial(t2, t_mag, mt.x, kid, vp.surface.w, f32(tm.w));
    return vec2f(val_eff * sk, f32(ft));
}

fn osc_flow(j: u32, pre: vec4f) -> vec3f {
    let tm = field[j * 3u + 1u];
    let fm = field[j * 3u + 2u];
    let mt = props[j * 4u];
    let delta = pre.xyz;
    let d2 = dot(delta, delta);
    let d_mag = sqrt(d2);
    let ft = u32(tm.z);
    let kid = u32(mt.z);
    let v = select(PROPAGATION_SPEED[ft], fm.x, ft == 7u && fm.x > 0.0);
    let val_eff = val_eff_at(pre, tm, ft, v, d_mag);
    let vp_grad = val_eff_grad(pre, tm, v, d_mag);
    let k = field_spatial(d2, d_mag, mt.x, kid, vp.surface.w, f32(tm.w));
    let kp = field_spatial_grad(d2, d_mag, mt.x, kid, vp.surface.w, f32(tm.w));
    let dhat = delta / max(d_mag, 1e-9);

    var g = -dhat * (vp_grad * k + val_eff * kp);
    return g;
}

@compute @workgroup_size(1)
fn presence_probe() {
    let count = u32(vp.surface.z);
    var omegas = array<f32, 9>();
    for (var i = 0u; i < 9u; i = i + 1u) { omegas[i] = 0.0; }
    var flow = vec3f(0.0);
    let dt = vp.presence.w - vp.expose_ex.y;
    let v_obs = vec3f(vp.right.w, vp.up.w, vp.forward.w) * C_VACUUM;
    for (var j = 0u; j < count; j = j + 1u) {
        let m = field[j * 3u];
        let tm = field[j * 3u + 1u];
        let fm = field[j * 3u + 2u];
        let mt = props[j * 4u];
        let v_rel = fm.yzw - v_obs;
        let propagated = m.xyz + v_rel * dt;
        let temporal = abs(vp.presence.w - tm.x);
        let ft = u32(tm.z);
        let kid = u32(mt.z);
        let pre = vec4f(propagated, m.w * exp(-temporal / max(tm.y, 1e-9)));
        pp[j] = pre;
        var fast = 0.0;
        if ((kid == 0u || kid == 6u) && (ft != 1u) && (temporal < tm.y * 1e-4)) {
            fast = 1.0;
        }
        pp[u32(vp.surface.z) + j] = vec4f(f32(ft), mt.x, f32(kid), fast);        let c = osc_field(j, vec3f(0.0), pre);
        let f = u32(c.y);
        if (f < 9u) { omegas[f] += c.x; }
        flow = flow + osc_flow(j, pre);
    }
    for (var i = 0u; i < 9u; i = i + 1u) { probe_out[i] = omegas[i]; }
    probe_out[9] = flow.x;
    probe_out[10] = flow.y;
    probe_out[11] = flow.z;
}
"#;

pub const TE_WGSL: &str = r#"
const RING_MAX: u32 = 1024u;
const SERIES_COUNT: u32 = 12u;
const DIM: u32 = 3u;
const ORDER: u32 = 3u;
const F32_EPS: f32 = 1.19e-07;
const LOG2_FACTORIAL: f32 = 2.584962500721156;

@group(0) @binding(0) var<storage, read> series: array<f32>;
@group(0) @binding(1) var<uniform> params: vec4<u32>;
@group(0) @binding(2) var<storage, read_write> verdict: array<f32>;

fn ser_at(s: u32, i: u32) -> f32 {
    return series[s * RING_MAX + i];
}

fn find_mi_lag(s: u32, n: u32, max_lag: u32) -> i32 {
    if (max_lag < 3u) { return -1; }
    var mi_prev2: f32 = 0.0;
    var mi_prev: f32 = 0.0;
    for (var lag = 1u; lag <= max_lag; lag = lag + 1u) {
        let w = n - lag;
        var mn: f32 = ser_at(s, 0u);
        var mx: f32 = mn;
        for (var i = 0u; i < w; i = i + 1u) {
            let v = ser_at(s, i);
            mn = min(mn, v);
            mx = max(mx, v);
        }
        let range = mx - mn;
        if (range <= 0.0) { return -1; }
        let mid = mn + range * 0.5;
        var h00: u32 = 0u;
        var h01: u32 = 0u;
        var h10: u32 = 0u;
        var h11: u32 = 0u;
        for (var i = 0u; i < w; i = i + 1u) {
            let b1 = ser_at(s, i) > mid;
            let b2 = ser_at(s, i + lag) > mid;
            if (!b1 && !b2) { h00 = h00 + 1u; }
            else if (!b1 && b2) { h01 = h01 + 1u; }
            else if (b1 && !b2) { h10 = h10 + 1u; }
            else { h11 = h11 + 1u; }
        }
        let total = f32(w);
        let p0 = (f32(h00) + f32(h01)) / total;
        let p1 = (f32(h10) + f32(h11)) / total;
        let q0 = (f32(h00) + f32(h10)) / total;
        let q1 = (f32(h01) + f32(h11)) / total;
        var mi: f32 = 0.0;
        if (h00 > 0u) {
            let p = f32(h00) / total;
            mi = mi + p * log2(p / (p0 * q0 + F32_EPS) + F32_EPS);
        }
        if (h01 > 0u) {
            let p = f32(h01) / total;
            mi = mi + p * log2(p / (p0 * q1 + F32_EPS) + F32_EPS);
        }
        if (h10 > 0u) {
            let p = f32(h10) / total;
            mi = mi + p * log2(p / (p1 * q0 + F32_EPS) + F32_EPS);
        }
        if (h11 > 0u) {
            let p = f32(h11) / total;
            mi = mi + p * log2(p / (p1 * q1 + F32_EPS) + F32_EPS);
        }
        if (lag >= 3u && mi_prev2 > mi_prev && mi_prev <= mi) {
            return i32(lag - 1u);
        }
        mi_prev2 = mi_prev;
        mi_prev = mi;
    }
    return -1;
}

fn permutation_entropy(s: u32, n: u32) -> vec2f {
    let span = ORDER - 1u;
    if (n <= span) { return vec2f(0.0, 0.0); }
    let total_windows = n - span;
    var counts = array<f32, 6>(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    var used: f32 = 0.0;
    for (var i = 0u; i < total_windows; i = i + 1u) {
        let v0 = ser_at(s, i);
        let v1 = ser_at(s, i + 1u);
        let v2 = ser_at(s, i + 2u);
        if (v0 == v1 || v0 == v2 || v1 == v2) { continue; }
        var rank = array<u32, 3>(0u, 0u, 0u);
        rank[0] = select(0u, 1u, v1 < v0) + select(0u, 1u, v2 < v0);
        rank[1] = select(0u, 1u, v0 < v1) + select(0u, 1u, v2 < v1);
        rank[2] = select(0u, 1u, v0 < v2) + select(0u, 1u, v1 < v2);
        let l0 = select(0u, 1u, rank[1] < rank[0]) + select(0u, 1u, rank[2] < rank[0]);
        let l1 = select(0u, 1u, rank[2] < rank[1]);
        let key = l0 * 2u + l1;
        counts[key] = counts[key] + 1.0;
        used = used + 1.0;
    }
    if (used <= 0.0) { return vec2f(0.0, 0.0); }
    var entropy: f32 = 0.0;
    for (var k = 0u; k < 6u; k = k + 1u) {
        if (counts[k] > 0.0) {
            let p = counts[k] / used;
            entropy = entropy - p * log2(p);
        }
    }
    return vec2f(entropy / LOG2_FACTORIAL, used);
}

fn embedded_silverman(s: u32, tau: u32, n: u32) -> f32 {
    let back = (DIM - 1u) * tau;
    if (back >= n) { return -1.0; }
    let n_emb = n - back;
    var mean = array<f32, 3>(0.0, 0.0, 0.0);
    for (var k = 0u; k < DIM; k = k + 1u) {
        var acc: f32 = 0.0;
        for (var u = 0u; u < n_emb; u = u + 1u) {
            acc = acc + ser_at(s, u + k * tau);
        }
        mean[k] = acc / f32(n_emb);
    }
    var var_acc: f32 = 0.0;
    for (var u = 0u; u < n_emb; u = u + 1u) {
        var d2: f32 = 0.0;
        for (var k = 0u; k < DIM; k = k + 1u) {
            let d = ser_at(s, u + k * tau) - mean[k];
            d2 = d2 + d * d;
        }
        var_acc = var_acc + d2;
    }
    var_acc = var_acc / f32(n_emb);
    if (var_acc <= 0.0) { return -1.0; }
    return 1.06 * sqrt(var_acc) * pow(f32(n_emb), -0.2);
}

fn future_silverman(t_low: u32, tau_x: u32, n: u32) -> f32 {
    let start = t_low + tau_x;
    if (start >= n) { return -1.0; }
    let cnt = n - start;
    if (cnt < 2u) { return -1.0; }
    var acc: f32 = 0.0;
    for (var i = start; i < n; i = i + 1u) {
        acc = acc + ser_at(0u, i);
    }
    let mean = acc / f32(cnt);
    var var_acc: f32 = 0.0;
    for (var i = start; i < n; i = i + 1u) {
        let d = ser_at(0u, i) - mean;
        var_acc = var_acc + d * d;
    }
    var_acc = var_acc / f32(cnt);
    if (var_acc <= 0.0) { return -1.0; }
    return 1.06 * sqrt(var_acc) * pow(f32(cnt), -0.2);
}

fn state_d2(s: u32, t: u32, q: u32, tau: u32) -> f32 {
    var d2: f32 = 0.0;
    for (var k = 0u; k < DIM; k = k + 1u) {
        let i_t = t - (DIM - 1u - k) * tau;
        let i_q = q - (DIM - 1u - k) * tau;
        let d = ser_at(s, i_t) - ser_at(s, i_q);
        d2 = d2 + d * d;
    }
    return d2;
}

fn te_embedded(tau_x: u32, tau_y: u32, sy: u32, h_f: f32, h_x: f32, h_y: f32, n: u32) -> f32 {
    let back_x = (DIM - 1u) * tau_x;
    let back_y = (DIM - 1u) * tau_y;
    let t_low = max(back_x, back_y);
    let t_high = n - tau_x - 1u;
    let m = t_high - t_low + 1u;
    let n_x = n - back_x;
    let n_xy = n - t_low;
    let inv2_hf = 0.5 / (h_f * h_f);
    let inv2_hx = 0.5 / (h_x * h_x);
    let inv2_hy = 0.5 / (h_y * h_y);
    var te: f32 = 0.0;
    for (var t = t_low; t <= t_high; t = t + 1u) {
        let fut = ser_at(0u, t + tau_x);
        var k3: f32 = 0.0;
        for (var q = t_low; q <= t_high; q = q + 1u) {
            let df = fut - ser_at(0u, q + tau_x);
            k3 = k3 + exp(-df * df * inv2_hf - state_d2(0u, t, q, tau_x) * inv2_hx - state_d2(sy, t, q, tau_y) * inv2_hy);
        }
        var k1: f32 = 0.0;
        for (var q = back_x; q < n; q = q + 1u) {
            k1 = k1 + exp(-state_d2(0u, t, q, tau_x) * inv2_hx);
        }
        var k2xy: f32 = 0.0;
        for (var q = t_low; q < n; q = q + 1u) {
            k2xy = k2xy + exp(-state_d2(0u, t, q, tau_x) * inv2_hx - state_d2(sy, t, q, tau_y) * inv2_hy);
        }
        var k2x: f32 = 0.0;
        for (var q = t_low; q <= t_high; q = q + 1u) {
            let df = fut - ser_at(0u, q + tau_x);
            k2x = k2x + exp(-df * df * inv2_hf - state_d2(0u, t, q, tau_x) * inv2_hx);
        }
        let p3 = k3 / f32(m);
        let p1 = k1 / f32(n_x);
        let p2xy = k2xy / f32(n_xy);
        let p2x = k2x / f32(m);
        te = te + log(p3 * p1 / max(p2xy * p2x, 1e-30));
    }
    return te / f32(m);
}

@compute @workgroup_size(16)
fn te_compute(@builtin(local_invocation_id) gid: vec3<u32>) {
    let tid = gid.x;
    let n = params.x;
    let max_lag = params.y;
    if (tid >= SERIES_COUNT) { return; }
    var tau: f32 = 0.0;
    var te: f32 = 0.0;
    var pe: f32 = 0.0;
    var motifs: f32 = 0.0;
    var valid: f32 = 0.0;
    var pe_valid: f32 = 0.0;
    var finite_ok: bool = true;
    for (var i = 0u; i < n; i = i + 1u) {
        let va = ser_at(tid, i);
        let vb = ser_at(0u, i);
        if (!(va == va) || !(vb == vb) || abs(va) >= 3.4028235e38 || abs(vb) >= 3.4028235e38) {
            finite_ok = false;
        }
    }
    if (tid == 0u) {
        if (finite_ok) {
            let tx = find_mi_lag(0u, n, max_lag);
            if (tx >= 0) {
                tau = f32(tx);
                valid = 1.0;
            }
        }
        let pev = permutation_entropy(0u, n);
        pe = pev.x;
        motifs = pev.y;
        if (pev.y > 0.0 && finite_ok) {
            pe_valid = 1.0;
        }
    } else {
        if (finite_ok && n >= 8u) {
            let tx = find_mi_lag(0u, n, max_lag);
            let ty = find_mi_lag(tid, n, max_lag);
            if (tx >= 0 && ty >= 0) {
                let u_tx = u32(tx);
                let u_ty = u32(ty);
                let back_x = (DIM - 1u) * u_tx;
                let back_y = (DIM - 1u) * u_ty;
                let t_low = max(back_x, back_y);
                let t_high_ok = u_tx + 1u < n;
                var t_high: u32 = 0u;
                if (t_high_ok) {
                    t_high = n - u_tx - 1u;
                }
                if (back_x < n && back_y < n && t_high_ok && t_low <= t_high) {
                    let m = t_high - t_low + 1u;
                    if (m >= 8u) {
                        let h_scale = bitcast<f32>(params.z);
                        let h_f = future_silverman(t_low, u_tx, n) * h_scale;
                        let h_x = embedded_silverman(0u, u_tx, n) * h_scale;
                        let h_y = embedded_silverman(tid, u_ty, n) * h_scale;
                        if (h_f > 0.0 && h_x > 0.0 && h_y > 0.0) {
                            tau = f32(u_ty);
                            te = te_embedded(u_tx, u_ty, tid, h_f, h_x, h_y, n);
                            valid = 1.0;
                        }
                    }
                }
            }
        }
        if (tid == 1u) {
            let pev = permutation_entropy(1u, n);
            pe = pev.x;
            motifs = pev.y;
            if (pev.y > 0.0 && finite_ok) {
                pe_valid = 1.0;
            }
        }
    }
    let o = tid * 6u;
    verdict[o + 0u] = tau;
    verdict[o + 1u] = te;
    verdict[o + 2u] = pe;
    verdict[o + 3u] = motifs;
    verdict[o + 4u] = valid;
    verdict[o + 5u] = pe_valid;
}
"#;
