use crate::archivar::{
    body_barycenter_position, sense_membrane, system_now, Buffer, CurveSet, LeapSeconds, Radiator,
    SampleRecord, PARSEC_M,
};
use crate::machines::{
    le_bytes_f32, te_absence_word, te_read_verdict, EnsoCell, EnsoMachine, SolarCell, SolarMachine,
    TE_SERIES_BYTES, TE_SERIES_STRIDE,
};
const FIELD_WGSL: &str = r#"
struct VP { surface: vec4f, right: vec4f, up: vec4f, forward: vec4f, expose_ex: vec4f, presence: vec4f, ft_ref_a: vec4f, ft_ref_b: vec4f, ft_ref_c: vec4f };
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
fn fold_eff(d_mag: f32, raw: f32, t: f32, ttl: f32, force_type: u32, advective_v: f32) -> f32 {
    let v_const = PROPAGATION_SPEED[force_type];
    let v = select(v_const, advective_v, force_type == 7u && advective_v > 0.0);
    let temporal = abs(vp.presence.w - t);
    var retarded = select(temporal - d_mag / v, temporal, v == 0.0 || d_mag == 0.0);
    retarded = max(retarded, 0.0);
    if (ttl <= 0.0) {
        return 0.0;
    }
    return raw * exp(-retarded / ttl);
}
@group(0) @binding(0) var<storage, read> field: array<vec4f>;
@group(0) @binding(1) var<storage, read> props: array<vec4f>;
@group(0) @binding(2) var<uniform> vp: VP;
@group(0) @binding(3) var<storage, read_write> probe_out: array<f32>;
@group(0) @binding(4) var<storage, read_write> pp: array<vec4f>;
@group(0) @binding(9) var color_lut: texture_2d<f32>;
@group(0) @binding(12) var color_lut_samp: sampler;

fn color_lut_rgb(ci: f32) -> vec3f {
    let u = clamp((ci - (-0.120)) / (5.100 - (-0.120)), 0.0, 1.0);
    let col = textureSampleLevel(color_lut, color_lut_samp, vec2f(u, 0.5), 0.0).rgb;
    return select(col, vec3f(1.0), ci == 0.0);
}

fn aberration(u: vec3f, beta: vec3f) -> vec3f {
    let b2 = dot(beta, beta);
    if (b2 >= 1.0) {
        return u;
    }
    let gamma = 1.0 / sqrt(1.0 - b2);
    let ud = dot(u, beta);
    return (u / gamma + beta + (gamma / (gamma + 1.0)) * ud * beta) / (1.0 + ud);
}

fn ft_ref(ra: vec4f, rb: vec4f, rc: vec4f, ft: u32) -> f32 {
    if (ft == 0u) { return ra.x; }
    if (ft == 1u) { return ra.y; }
    if (ft == 2u) { return ra.z; }
    if (ft == 3u) { return ra.w; }
    if (ft == 4u) { return rb.x; }
    if (ft == 5u) { return rb.y; }
    if (ft == 6u) { return rb.z; }
    if (ft == 7u) { return rb.w; }
    return rc.x;
}

fn ft_ref_floor(ra: vec4f, rb: vec4f, rc: vec4f, ft: u32) -> f32 {
    return max(ft_ref(ra, rb, rc, ft), 0.0);
}

fn lum_ratio(x: f32, r: f32) -> f32 {
    if (r > 0.0) {
        return x / r;
    }
    return 0.0;
}

struct VOut { @builtin(position) pos: vec4f, @location(0) uv: vec2f };

@vertex fn vs(@builtin(vertex_index) i: u32) -> VOut {
    var p = array<vec2f, 6>(
        vec2f(-1.0, -1.0), vec2f(1.0, -1.0), vec2f(1.0, 1.0),
        vec2f(-1.0, -1.0), vec2f(1.0, 1.0), vec2f(-1.0, 1.0)
    );
    var out: VOut;
    out.pos = vec4f(p[i], 0.0, 1.0);
    out.uv = vec2f(p[i].x * 0.5 + 0.5, 0.5 - p[i].y * 0.5);
    return out;
}

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

fn source_contrib(j: u32, pixel_rel: vec3f) -> vec4f {
    let pre = pp[j];
    let p = pp[u32(vp.surface.z) + j];
    let tm = field[j * 3u + 1u];
    let fm = field[j * 3u + 2u];
    let mt = props[j * 4u];
    let mg = props[j * 4u + 2u];
    let delta = pre.xyz - pixel_rel;
    let sd = dot(pre.xyz, vp.forward.xyz);
    let d2 = dot(delta, delta);
    let d_mag = sqrt(d2);
    let t2 = max(d2 - sd * sd, 0.0);
    let t_mag = sqrt(t2);
    let ft = u32(p.x);
    if (p.w > 0.5) {
        let e2 = max(max(p.y * p.y, vp.surface.w * vp.surface.w), 1e-30);
        var val = pre.w;
        if (ft == 0u && mt.w > 0.0) {
            let z1 = 1.0 + mt.w;
            val = val / (z1 * z1 * z1 * z1);
        }
        return vec4f(val / (t2 + e2), f32(ft), mg.z, mt.y);
    }
    let kid = u32(mt.z);
    let v = select(PROPAGATION_SPEED[ft], fm.x, ft == 7u && fm.x > 0.0);
    let temporal = abs(vp.presence.w - tm.x);
    var val = pre.w;
    let corr = select(min(temporal, d_mag / v), 0.0, v == 0.0 || d_mag == 0.0);
    if (corr > tm.y * 1e-4) {
        val = pre.w * exp(corr / max(tm.y, 1e-9));
    }
    if (ft == 0u && mt.w > 0.0) {
        let z1 = 1.0 + mt.w;
        val = val / (z1 * z1 * z1 * z1);
    }
    var sk = field_spatial(t2, t_mag, mt.x, kid, vp.surface.w, f32(tm.w));
    var contrib = val * sk;
    return vec4f(contrib, f32(ft), mg.z, mt.y);
}

@fragment fn fs(in: VOut) -> @location(0) vec4f {
    let count = u32(vp.surface.z);
    if (count == 0u) { discard; }
    let w = vp.surface.x;
    let h = vp.surface.y;
    let scale = vp.surface.w;
    let pixel_rel = (in.uv.x - 0.5) * w * scale * vp.right.xyz
        + (0.5 - in.uv.y) * h * scale * vp.up.xyz;
    var omega = array<f32, 9>();
    for (var i = 0u; i < 9u; i = i + 1u) { omega[i] = 0.0; }
    var rgb = vec3f(0.0);

    for (var j = 0u; j < count; j = j + 1u) {
        let c = source_contrib(j, pixel_rel);
        let f = u32(c.y);
        if (f < 9u) {
            omega[f] += c.x;
            let ratio = lum_ratio(abs(c.x), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, f))
                * scale * scale;
            let lum = clamp(log2(1.0 + ratio) / 22.0, 0.0, 1.0);
            if (f == 0u) {
                rgb += color_lut_rgb(c.z) * lum;
            }
        }
    }

    let omega_total = (lum_ratio(abs(omega[0]), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, 0u))
        + lum_ratio(abs(omega[1]), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, 1u))
        + lum_ratio(abs(omega[2]), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, 2u))
        + lum_ratio(abs(omega[3]), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, 3u))
        + lum_ratio(abs(omega[4]), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, 4u))
        + lum_ratio(abs(omega[5]), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, 5u))
        + lum_ratio(abs(omega[6]), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, 6u))
        + lum_ratio(abs(omega[7]), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, 7u))
        + lum_ratio(abs(omega[8]), ft_ref_floor(vp.ft_ref_a, vp.ft_ref_b, vp.ft_ref_c, 8u)))
        * scale * scale;
    if (omega_total < 1e-30) { discard; }

    let olog = log2(max(omega_total, 1e-30));
    let lum = clamp((olog + vp.expose_ex.x) / 22.0, 0.0, 1.0);
    let fade = clamp((olog - log2(1e-30)) / (-vp.expose_ex.x - log2(1e-30)), 0.0, 1.0);
    let noise = fract(sin(dot(in.pos.xy, vec2f(12.9898, 78.233))) * 43758.5453);
    let grain = 0.9 + noise * 0.1;
    return vec4f(rgb * lum * fade * grain, 1.0);
}

@group(0) @binding(10) var hud_tex: texture_2d<f32>;
@group(0) @binding(11) var hud_samp: sampler;

struct HudVOut { @builtin(position) pos: vec4f, @location(0) uv: vec2f };

@vertex fn hud_vs(@builtin(vertex_index) i: u32) -> HudVOut {
    var quad = array<vec2f, 6>(
        vec2f(-1.0, -1.0), vec2f(1.0, -1.0), vec2f(1.0, 1.0),
        vec2f(-1.0, -1.0), vec2f(1.0, 1.0), vec2f(-1.0, 1.0)
    );
    let w = vp.surface.x;
    let h = vp.surface.y;
    let corner = quad[i];
    let px = vec2f((corner.x * 0.5 + 0.5) * w, (0.5 - corner.y * 0.5) * 32.0);
    var out: HudVOut;
    out.pos = vec4f(px.x / w * 2.0 - 1.0, 1.0 - px.y / h * 2.0, 0.0, 1.0);
    out.uv = px / vec2f(w, 32.0);
    return out;
}

@fragment fn hud_fs(in: HudVOut) -> @location(0) vec4f {
    let c = textureSampleLevel(hud_tex, hud_samp, in.uv, 0.0);
    if (c.a < 0.5) { discard; }
    return vec4f(c.rgb, 1.0);
}
"#;
const TE_WGSL: &str = r#"
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
const HUD_LINE_H: i32 = 8;
const HUD_CHAR_W: i32 = 6;
const HUD_LINES: u32 = 4;
const HUD_H: u32 = HUD_LINE_H as u32 * HUD_LINES;

const HUD_GLYPH: [[u8; 5]; 95] = [
    [0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x5F, 0x00, 0x00],
    [0x00, 0x07, 0x00, 0x07, 0x00],
    [0x14, 0x7F, 0x14, 0x7F, 0x14],
    [0x24, 0x2A, 0x7F, 0x2A, 0x12],
    [0x23, 0x13, 0x08, 0x64, 0x62],
    [0x36, 0x49, 0x56, 0x20, 0x50],
    [0x00, 0x08, 0x07, 0x03, 0x00],
    [0x00, 0x1C, 0x22, 0x41, 0x00],
    [0x00, 0x41, 0x22, 0x1C, 0x00],
    [0x2A, 0x1C, 0x7F, 0x1C, 0x2A],
    [0x08, 0x08, 0x3E, 0x08, 0x08],
    [0x00, 0x80, 0x70, 0x30, 0x00],
    [0x08, 0x08, 0x08, 0x08, 0x08],
    [0x00, 0x00, 0x60, 0x60, 0x00],
    [0x20, 0x10, 0x08, 0x04, 0x02],
    [0x3E, 0x51, 0x49, 0x45, 0x3E],
    [0x00, 0x42, 0x7F, 0x40, 0x00],
    [0x72, 0x49, 0x49, 0x49, 0x46],
    [0x21, 0x41, 0x49, 0x4D, 0x33],
    [0x18, 0x14, 0x12, 0x7F, 0x10],
    [0x27, 0x45, 0x45, 0x45, 0x39],
    [0x3C, 0x4A, 0x49, 0x49, 0x31],
    [0x41, 0x21, 0x11, 0x09, 0x07],
    [0x36, 0x49, 0x49, 0x49, 0x36],
    [0x46, 0x49, 0x49, 0x29, 0x1E],
    [0x00, 0x00, 0x14, 0x00, 0x00],
    [0x00, 0x40, 0x34, 0x00, 0x00],
    [0x00, 0x08, 0x14, 0x22, 0x41],
    [0x14, 0x14, 0x14, 0x14, 0x14],
    [0x00, 0x41, 0x22, 0x14, 0x08],
    [0x02, 0x01, 0x59, 0x09, 0x06],
    [0x3E, 0x41, 0x5D, 0x59, 0x4E],
    [0x7C, 0x12, 0x11, 0x12, 0x7C],
    [0x7F, 0x49, 0x49, 0x49, 0x36],
    [0x3E, 0x41, 0x41, 0x41, 0x22],
    [0x7F, 0x41, 0x41, 0x41, 0x3E],
    [0x7F, 0x49, 0x49, 0x49, 0x41],
    [0x7F, 0x09, 0x09, 0x09, 0x01],
    [0x3E, 0x41, 0x41, 0x51, 0x73],
    [0x7F, 0x08, 0x08, 0x08, 0x7F],
    [0x00, 0x41, 0x7F, 0x41, 0x00],
    [0x20, 0x40, 0x41, 0x3F, 0x01],
    [0x7F, 0x08, 0x14, 0x22, 0x41],
    [0x7F, 0x40, 0x40, 0x40, 0x40],
    [0x7F, 0x02, 0x1C, 0x02, 0x7F],
    [0x7F, 0x04, 0x08, 0x10, 0x7F],
    [0x3E, 0x41, 0x41, 0x41, 0x3E],
    [0x7F, 0x09, 0x09, 0x09, 0x06],
    [0x3E, 0x41, 0x51, 0x21, 0x5E],
    [0x7F, 0x09, 0x19, 0x29, 0x46],
    [0x26, 0x49, 0x49, 0x49, 0x32],
    [0x03, 0x01, 0x7F, 0x01, 0x03],
    [0x3F, 0x40, 0x40, 0x40, 0x3F],
    [0x1F, 0x20, 0x40, 0x20, 0x1F],
    [0x3F, 0x40, 0x38, 0x40, 0x3F],
    [0x63, 0x14, 0x08, 0x14, 0x63],
    [0x03, 0x04, 0x78, 0x04, 0x03],
    [0x61, 0x59, 0x49, 0x4D, 0x43],
    [0x00, 0x7F, 0x41, 0x41, 0x41],
    [0x02, 0x04, 0x08, 0x10, 0x20],
    [0x00, 0x41, 0x41, 0x41, 0x7F],
    [0x04, 0x02, 0x01, 0x02, 0x04],
    [0x40, 0x40, 0x40, 0x40, 0x40],
    [0x00, 0x03, 0x07, 0x08, 0x00],
    [0x20, 0x54, 0x54, 0x78, 0x40],
    [0x7F, 0x28, 0x44, 0x44, 0x38],
    [0x38, 0x44, 0x44, 0x44, 0x28],
    [0x38, 0x44, 0x44, 0x28, 0x7F],
    [0x38, 0x54, 0x54, 0x54, 0x18],
    [0x00, 0x08, 0x7E, 0x09, 0x02],
    [0x18, 0xA4, 0xA4, 0x9C, 0x78],
    [0x7F, 0x08, 0x04, 0x04, 0x78],
    [0x00, 0x44, 0x7D, 0x40, 0x00],
    [0x20, 0x40, 0x40, 0x3D, 0x00],
    [0x7F, 0x10, 0x28, 0x44, 0x00],
    [0x00, 0x41, 0x7F, 0x40, 0x00],
    [0x7C, 0x04, 0x78, 0x04, 0x78],
    [0x7C, 0x08, 0x04, 0x04, 0x78],
    [0x38, 0x44, 0x44, 0x44, 0x38],
    [0xFC, 0x18, 0x24, 0x24, 0x18],
    [0x18, 0x24, 0x24, 0x18, 0xFC],
    [0x7C, 0x08, 0x04, 0x04, 0x08],
    [0x48, 0x54, 0x54, 0x54, 0x24],
    [0x04, 0x04, 0x3F, 0x44, 0x24],
    [0x3C, 0x40, 0x40, 0x20, 0x7C],
    [0x1C, 0x20, 0x40, 0x20, 0x1C],
    [0x3C, 0x40, 0x30, 0x40, 0x3C],
    [0x44, 0x28, 0x10, 0x28, 0x44],
    [0x4C, 0x90, 0x90, 0x90, 0x7C],
    [0x44, 0x64, 0x54, 0x4C, 0x44],
    [0x00, 0x08, 0x36, 0x41, 0x00],
    [0x00, 0x00, 0x77, 0x00, 0x00],
    [0x00, 0x41, 0x36, 0x08, 0x00],
    [0x02, 0x01, 0x02, 0x04, 0x02],
];
use crate::force::kernel_id_for_force;
use std::collections::{HashMap, HashSet};
use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoopBuilder};
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
#[cfg(target_os = "linux")]
use winit::platform::wayland::EventLoopBuilderExtWayland;
#[cfg(target_os = "linux")]
use winit::platform::x11::EventLoopBuilderExtX11;
use winit::window::{Fullscreen, Window, WindowAttributes, WindowId};

const Φ: f64 = 1.618033988749895;
const C: f64 = 299792458.0;
pub const GRID_INIT: f64 = 2147483648.0;
pub const JUMP_GRID: f64 = 268435456.0;
const SSAA_MAX: f32 = 8.0;
const BUDGET_RELAX: f64 = 0.1;
const PERM_GROUND: f32 = f32::EPSILON;
const FORCE_NAME: [&str; 9] = [
    "em",
    "gravity",
    "acoustic",
    "seismic-body",
    "seismic-surface",
    "thermal",
    "diffusion",
    "advective",
    "electric",
];
const FORCE_SI_UNIT: [&str; 9] = ["W/m2", "m/s2", "Pa", "m", "m", "K", "kg/m3", "m/s", "V/m"];
const FIELD_BACKING_SCALE: f64 = 1.0;
const EMA_FACTOR: f64 = 0.05;
const EXPOSE_OFFSET_BASE: f32 = 4.0;
const OFFSET_RELAX: f32 = 0.03125;
const REF_RELAX: f32 = 0.0625;
const THRUST_STEP: f64 = 64.0;
const JUMP_BODIES: [&str; 9] = [
    "sun", "mercury", "venus", "earth", "mars", "jupiter", "saturn", "uranus", "neptune",
];
pub type Record = SampleRecord;

pub struct PackedWindow {
    pub field: Vec<f32>,
    pub meta: Vec<f32>,
    pub count: u32,
}

#[derive(Clone, Copy)]
pub struct PresenceFrame {
    pub omega: [f32; 9],
}

pub trait KineticRadiator: Send + 'static {
    fn vibrate(&mut self, frame: &PresenceFrame);
}

pub struct AcousticOscillator {
    _thread: Option<thread::JoinHandle<()>>,
}

impl AcousticOscillator {
    pub fn new(rx: mpsc::Receiver<PresenceFrame>) -> Self {
        let emit = !std::io::stdout().is_terminal();
        let handle = thread::spawn(move || {
            let mut out = std::io::stdout();
            while let Ok(frame) = rx.recv() {
                if emit {
                    let intensity: f32 = frame.omega.iter().sum();
                    if std::io::Write::write_all(&mut out, &intensity.to_le_bytes()).is_err()
                        || std::io::Write::flush(&mut out).is_err()
                    {
                        break;
                    }
                }
            }
        });
        Self {
            _thread: Some(handle),
        }
    }
}

pub struct SeismicOscillator {
    port: Option<Box<dyn serialport::SerialPort>>,
}

impl SeismicOscillator {
    pub fn new(path: &str) -> Self {
        let port = serialport::new(path, 115_200)
            .timeout(std::time::Duration::from_millis(50))
            .open()
            .ok();
        if port.is_none() {
            eprintln!(
                "seismic oscillator: {} unreachable — the oscillator stays silent",
                path
            );
        }
        Self { port }
    }
}

impl KineticRadiator for SeismicOscillator {
    fn vibrate(&mut self, frame: &PresenceFrame) {
        let Some(port) = self.port.as_mut() else {
            return;
        };
        let intensity: f32 = frame.omega.iter().sum();
        if std::io::Write::write_all(port, &intensity.to_le_bytes()).is_err() {
            self.port = None;
        }
    }
}

pub fn pack_window(records: &[Record], presence: [f64; 3]) -> PackedWindow {
    let n = records.len();
    let mut field = vec![0.0f32; n * 12];
    let mut meta = vec![0.0f32; n * 16];
    for (j, r) in records.iter().enumerate() {
        let f = j * 12;
        let m = j * 16;
        field[f] = (r.0 - presence[0]) as f32;
        field[f + 1] = (r.1 - presence[1]) as f32;
        field[f + 2] = (r.2 - presence[2]) as f32;
        field[f + 3] = r.3 as f32;
        field[f + 4] = r.4 as f32;
        field[f + 5] = r.5 as f32;
        field[f + 6] = r.9 as f32;
        field[f + 7] = r.10 as f32;
        field[f + 8] = r.11 as f32;
        field[f + 9] = r.12 as f32;
        field[f + 10] = r.13 as f32;
        field[f + 11] = r.14 as f32;
        meta[m] = r.7 as f32;
        meta[m + 1] = r.6 as f32;
        meta[m + 2] = r.8 as f32;
        meta[m + 3] = if r.9 == 0.0 { r.15 as f32 } else { 0.0 };
        meta[m + 4] = r.15 as f32;
        meta[m + 5] = r.16 as f32;
        meta[m + 6] = r.17 as f32;
        meta[m + 7] = r.18 as f32;
        meta[m + 8] = r.19 as f32;
        meta[m + 9] = r.20 as f32;
        meta[m + 10] = r.21 as f32;
        meta[m + 11] = r.22 as f32;
        meta[m + 12] = r.23 as f32;
        meta[m + 13] = 0.0;
        meta[m + 14] = 0.0;
        meta[m + 15] = 0.0;
    }
    PackedWindow {
        field,
        meta,
        count: n as u32,
    }
}

pub fn force_ref_medians(field: &[f32], meta: &[f32]) -> [Option<f32>; 9] {
    let mut hist: [[u32; 256]; 9] = [[0; 256]; 9];
    let mut sum: [[f32; 256]; 9] = [[0.0; 256]; 9];
    let mut n: [u32; 9] = [0; 9];
    for (j, f) in field.chunks_exact(12).enumerate() {
        let ft = f[6] as i64;
        if !(0..=8).contains(&ft) {
            continue;
        }
        let v = f[3];
        if !v.is_finite() || v == 0.0 {
            continue;
        }
        if v.abs() == meta[j * 16] {
            continue;
        }
        let l = v.abs().log2();
        let b = log2_bin_of(l);
        hist[ft as usize][b] += 1;
        sum[ft as usize][b] += l;
        n[ft as usize] += 1;
    }
    let mut meds = [None; 9];
    for ft in 0..9 {
        if n[ft] == 0 {
            continue;
        }
        let target = (n[ft] + 1) / 2;
        let mut cum = 0u32;
        let mut bin = 0usize;
        while cum < target && bin < 256 {
            cum += hist[ft][bin];
            if cum < target {
                bin += 1;
            }
        }
        meds[ft] = Some((sum[ft][bin] / hist[ft][bin] as f32).exp2());
    }
    meds
}

fn log2_bin_of(l: f32) -> usize {
    ((l + 126.0) as i32).clamp(0, 255) as usize
}

fn emit_curves(
    cset: &CurveSet,
    center: [f64; 3],
    t: f64,
    pad: f64,
    records: &mut Vec<SampleRecord>,
) {
    let Some(kernel) = kernel_id_for_force(0) else {
        return;
    };
    for star in &cset.stars {
        if !star.plx_mas.is_finite() || star.plx_mas <= 0.0 || star.samples.len() < 2 {
            continue;
        }
        let d_m = (1000.0 / star.plx_mas) * PARSEC_M;
        let ra = star.ra_deg.to_radians();
        let dec = star.dec_deg.to_radians();
        let (sa, ca) = ra.sin_cos();
        let (sd, cd) = dec.sin_cos();
        let p = [cd * ca * d_m, cd * sa * d_m, sd * d_m];
        let rel = [p[0] - center[0], p[1] - center[1], p[2] - center[2]];
        let dist = (rel[0] * rel[0] + rel[1] * rel[1] + rel[2] * rel[2]).sqrt();
        if dist > pad {
            continue;
        }
        let idx = star.samples.partition_point(|s| s.0 <= t);
        if idx == 0 || idx >= star.samples.len() {
            continue;
        }
        let (t_a, f_a) = star.samples[idx - 1];
        let (t_b, f_b) = star.samples[idx];
        let (t_s, f_s) = if (t - t_a).abs() <= (t_b - t).abs() {
            (t_a, f_a)
        } else {
            (t_b, f_b)
        };
        records.push((
            p[0],
            p[1],
            p[2],
            f_s as f64,
            t_s,
            star.cadence,
            star.cadence,
            0.0,
            kernel as f64,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ));
    }
}

pub struct EMOscillator {
    tx: mpsc::SyncSender<Arc<Buffer>>,
    shutdown: Arc<AtomicBool>,
    _thread: Option<thread::JoinHandle<()>>,
}

struct SenseReq {
    field: Arc<Buffer>,
    center: [f64; 3],
    t: f64,
    pad: f64,
    cache_interval: f64,
    forward: [f64; 3],
    expose_offset: f32,
    force_ref: [f32; 9],
    softening: f64,
}

impl EMOscillator {
    pub fn new(
        presence_tx: mpsc::Sender<(String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
        sensor_tx: mpsc::Sender<Vec<(String, f64, f64)>>,
        body_names: Arc<Vec<String>>,
        time: Arc<Mutex<Option<LeapSeconds>>>,
        consent: Arc<AtomicBool>,
        acoustic_tx: mpsc::Sender<PresenceFrame>,
        seismic_tx: mpsc::Sender<PresenceFrame>,
        solar_rx: mpsc::Receiver<SolarCell>,
        enso_rx: mpsc::Receiver<EnsoCell>,
    ) -> Self {
        let (tx, rx) = mpsc::sync_channel::<Arc<Buffer>>(2);
        let (req_tx, req_rx) = mpsc::sync_channel::<SenseReq>(1);
        let (res_tx, res_rx) = mpsc::sync_channel::<(PackedWindow, f64, u64)>(2);
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = shutdown.clone();
        thread::spawn(move || {
            let mut generation: u64 = 0;
            let mut last_bytes: Vec<u8> = Vec::new();
            loop {
                let Ok(mut req) = req_rx.recv() else {
                    break;
                };
                while let Ok(newer) = req_rx.try_recv() {
                    req = newer;
                }
                let SenseReq {
                    field,
                    center,
                    t,
                    pad,
                    cache_interval,
                    forward,
                    expose_offset,
                    force_ref,
                    softening,
                    ..
                } = req;
                let mut records: Vec<Record> = Vec::new();
                let eph = field.eph.clone();
                let scale2 = softening * softening;
                let mut floor = [0.0f64; 9];
                for ft in 0..9 {
                    let r = force_ref[ft] as f64;
                    if r.is_finite() && r > 0.0 && scale2 > 0.0 {
                        floor[ft] = r * (0.5f64).powi(expose_offset as i32) / scale2;
                    }
                }
                sense_membrane(
                    &field,
                    center,
                    t,
                    pad,
                    cache_interval,
                    &floor,
                    softening,
                    forward,
                    &mut records,
                    &eph,
                );
                if let Some(cset) = &field.curves {
                    emit_curves(cset, center, t, pad, &mut records);
                }
                let packed = pack_window(&records, center);
                let mut key = Vec::with_capacity(packed.field.len() * 4 + packed.meta.len() * 4);
                for x in &packed.field {
                    key.extend_from_slice(&x.to_le_bytes());
                }
                for x in &packed.meta {
                    key.extend_from_slice(&x.to_le_bytes());
                }
                if key == last_bytes {
                    continue;
                }
                last_bytes = key;
                generation = generation.wrapping_add(1);
                if res_tx.send((packed, t, generation)).is_err() {
                    break;
                }
            }
        });
        let handle = thread::spawn(move || {
            run_window(
                rx,
                presence_tx,
                sensor_tx,
                req_tx,
                res_rx,
                body_names,
                time,
                shutdown_clone,
                consent,
                acoustic_tx,
                seismic_tx,
                solar_rx,
                enso_rx,
            );
        });
        Self {
            tx,
            shutdown,
            _thread: Some(handle),
        }
    }
}

impl Radiator for EMOscillator {
    fn accept(&mut self, field: Arc<Buffer>) {
        if let Err(mpsc::TrySendError::Disconnected(_)) = self.tx.try_send(field) {
            eprintln!("em oscillator channel closed — field buffer dropped");
        }
    }
}

impl EMOscillator {
    pub fn shutdown_flag(&self) -> Arc<AtomicBool> {
        self.shutdown.clone()
    }
}

impl Drop for EMOscillator {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

fn q_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [
        a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3],
        a[0] * b[1] + a[1] * b[0] + a[2] * b[3] - a[3] * b[2],
        a[0] * b[2] - a[1] * b[3] + a[2] * b[0] + a[3] * b[1],
        a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0],
    ]
}

fn q_norm(q: [f64; 4]) -> [f64; 4] {
    let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3])
        .sqrt()
        .max(1e-12);
    [q[0] / n, q[1] / n, q[2] / n, q[3] / n]
}

fn q_rotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    let p = [0.0, v[0], v[1], v[2]];
    let c = [q[0], -q[1], -q[2], -q[3]];
    let r = q_mul(q_mul(q, p), c);
    [r[1], r[2], r[3]]
}

fn q_axis_angle(axis: [f64; 3], angle: f64) -> [f64; 4] {
    let s = (angle / 2.0).sin();
    [(angle / 2.0).cos(), axis[0] * s, axis[1] * s, axis[2] * s]
}

const WINDOW_STATE_PATH: &str = "/tmp/omegaflow_window_state.φ";

fn window_state_load(path: &str) -> (f64, [f64; 3], [f64; 4]) {
    let mut grid = GRID_INIT;
    let mut p = [0.0f64; 3];
    let mut q = [1.0, 0.0, 0.0, 0.0];
    let Ok(text) = std::fs::read_to_string(path) else {
        return (grid, p, q);
    };
    for line in text.lines() {
        let mut toks = line.split_whitespace();
        match toks.next() {
            Some("grid_step") => {
                if let Some(v) = toks.next().and_then(|s| s.parse::<f64>().ok()) {
                    if v.is_finite() && v > 0.0 {
                        grid = v;
                    }
                }
            }
            Some("p") => {
                let vals: Option<Vec<f64>> = toks.map(|s| s.parse::<f64>().ok()).collect();
                if let Some(vals) = vals {
                    if vals.len() == 3 && vals.iter().all(|v| v.is_finite()) {
                        p = [vals[0], vals[1], vals[2]];
                    }
                }
            }
            Some("q") => {
                let vals: Option<Vec<f64>> = toks.map(|s| s.parse::<f64>().ok()).collect();
                if let Some(vals) = vals {
                    if vals.len() == 4
                        && vals.iter().all(|v| v.is_finite())
                        && vals.iter().any(|v| *v != 0.0)
                    {
                        q = q_norm([vals[0], vals[1], vals[2], vals[3]]);
                    }
                }
            }
            _ => {}
        }
    }
    (grid, p, q)
}

fn window_state_save(path: &str, grid_step: f64, p: [f64; 3], q: [f64; 4]) {
    let mut text = String::new();
    text.push_str(&format!("grid_step {:.17e}\n", grid_step));
    text.push_str(&format!("p {:.17e} {:.17e} {:.17e}\n", p[0], p[1], p[2]));
    text.push_str(&format!(
        "q {:.17e} {:.17e} {:.17e} {:.17e}\n",
        q[0], q[1], q[2], q[3]
    ));
    let _ = std::fs::write(path, text);
}

fn storage_entry(read_only: bool, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

const CAPTURE_RING_SIZE: usize = 32;

struct NativeOsc {
    ring: [f64; CAPTURE_RING_SIZE],
    idx: usize,
    filled: usize,
    median: f64,
    last_sent: f64,
    tau: f64,
}

struct NativeSensors {
    oscs: HashMap<String, NativeOsc>,
    tx: mpsc::Sender<Vec<(String, f64, f64)>>,
    frame_interval: f64,
}

impl NativeSensors {
    fn record_sample(&mut self, name: &str, value: f64) {
        if !value.is_finite() {
            return;
        }
        let osc = self.oscs.entry(name.to_string()).or_insert(NativeOsc {
            ring: [0.0; CAPTURE_RING_SIZE],
            idx: 0,
            filled: 0,
            median: value,
            last_sent: value,
            tau: self.frame_interval,
        });
        osc.tau = self.frame_interval;
        osc.ring[osc.idx] = value;
        osc.idx = (osc.idx + 1) % CAPTURE_RING_SIZE;
        if osc.filled < CAPTURE_RING_SIZE {
            osc.filled += 1;
        }
        osc.median = osc.median * (1.0 - EMA_FACTOR) + value * EMA_FACTOR;
    }

    fn flush(&mut self) {
        let mut list = Vec::new();
        for (name, osc) in self.oscs.iter_mut() {
            if !osc.median.is_finite() {
                continue;
            }
            if (osc.median - osc.last_sent).abs() > f64::EPSILON {
                osc.last_sent = osc.median;
                list.push((name.clone(), osc.median, osc.tau));
            }
        }
        if !list.is_empty() {
            if self.tx.send(list).is_err() {
                eprintln!("sensor channel closed — samples dropped");
            }
        }
    }
}

struct NativeApp {
    rx: mpsc::Receiver<Arc<Buffer>>,
    req_tx: mpsc::SyncSender<SenseReq>,
    res_rx: mpsc::Receiver<(PackedWindow, f64, u64)>,
    presence_tx: mpsc::Sender<(String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    body_names: Arc<Vec<String>>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
    shutdown: Arc<AtomicBool>,
    consent: Arc<AtomicBool>,
    acoustic_tx: mpsc::Sender<PresenceFrame>,
    seismic_tx: mpsc::Sender<PresenceFrame>,
    silent: bool,

    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'static>>,
    config: Option<wgpu::SurfaceConfiguration>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    render_pipe: Option<wgpu::RenderPipeline>,
    probe_pipe: Option<wgpu::ComputePipeline>,
    probe_layout: Option<wgpu::BindGroupLayout>,
    render_layout: Option<wgpu::BindGroupLayout>,
    render_binds: [Option<wgpu::BindGroup>; 2],
    probe_binds: [Option<wgpu::BindGroup>; 2],
    field_bufs: [Option<wgpu::Buffer>; 2],
    meta_bufs: [Option<wgpu::Buffer>; 2],
    vp_buf: Option<wgpu::Buffer>,
    probe_buf: Option<wgpu::Buffer>,
    probe_read: Option<wgpu::Buffer>,
    prep_param_buf: Option<wgpu::Buffer>,
    te_pipe: Option<wgpu::ComputePipeline>,
    te_bind: Option<wgpu::BindGroup>,
    te_series_buf: Option<wgpu::Buffer>,
    te_param_buf: Option<wgpu::Buffer>,
    te_out_buf: Option<wgpu::Buffer>,
    te_read_buf: Option<wgpu::Buffer>,
    te_map: Option<Arc<AtomicBool>>,
    te_named: String,
    color_lut_view: Option<wgpu::TextureView>,
    color_lut_sampler: Option<wgpu::Sampler>,
    field_cap: u32,
    buf_sel: usize,
    packed_gen: u64,
    uploaded_gen: u64,
    backing: (u32, u32),

    latest_field: Option<Arc<Buffer>>,
    packed_field: Vec<f32>,
    packed_meta: Vec<f32>,
    packed_count: u32,
    last_response_epoch: f64,

    p: [f64; 3],
    v: [f64; 3],
    t0: f64,
    t_presence: f64,
    q: [f64; 4],
    grid_step: f64,
    ssaa: f32,
    expose_offset: f32,
    field_dark: bool,
    hud_dark: bool,
    force_ref: [f32; 9],
    probe_omega: [f32; 9],
    probe_flow: [f32; 3],
    probe_ring: [[f32; 12]; 256],
    ring_head: usize,
    ring_filled: usize,
    ring_gen: u64,
    pe_ring: Vec<f64>,
    solar: SolarMachine,
    enso: EnsoMachine,
    hud_topology: Option<(usize, usize, Option<f64>, Option<f64>)>,
    frame_ms_ema: f64,
    field_permeability: f32,
    prev_omega_sum: f32,
    prev_delta: f32,
    prev_in_te: f64,
    direction: i32,
    ticks_since_turn: u64,
    natural_latency_ticks: u64,
    t_thrust: f64,
    t_thrust_target: f64,
    t_frozen: bool,
    keys: HashSet<KeyCode>,
    shift: bool,
    focus_req: u32,
    focused: bool,
    keys_seen: u64,
    cursor: Option<(f64, f64)>,
    drag_button: Option<MouseButton>,
    touches: HashMap<u64, (f64, f64)>,
    tap: Option<(std::time::Instant, (f64, f64))>,
    press: Option<(std::time::Instant, u64, (f64, f64))>,
    #[cfg(feature = "gamepad")]
    gilrs: Option<gilrs::Gilrs>,
    last_tick: Option<std::time::Instant>,
    stable_tick: f64,
    size: (u32, u32),
    scale_factor: f64,
    last_sent: (f64, f64, f64, f64, f64, [f64; 3], f64),
    last_saved_state: (f64, [f64; 3], [f64; 4]),
    sensors: NativeSensors,
    frame_count: u64,
    frame_ms_max: f64,
    last_hud: Option<std::time::Instant>,
    hud_tex: Option<wgpu::Texture>,
    hud_view: Option<wgpu::TextureView>,
    hud_sampler: Option<wgpu::Sampler>,
    hud_pipe: Option<wgpu::RenderPipeline>,
    hud_layout: Option<wgpu::BindGroupLayout>,
    hud_bind: Option<wgpu::BindGroup>,
    hud_bitmap: Vec<u8>,
    hud_w: u32,
    hud_dirty: bool,
}

impl NativeApp {
    fn new(
        rx: mpsc::Receiver<Arc<Buffer>>,
        req_tx: mpsc::SyncSender<SenseReq>,
        res_rx: mpsc::Receiver<(PackedWindow, f64, u64)>,
        presence_tx: mpsc::Sender<(String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
        sensor_tx: mpsc::Sender<Vec<(String, f64, f64)>>,
        body_names: Arc<Vec<String>>,
        time: Arc<Mutex<Option<LeapSeconds>>>,
        shutdown: Arc<AtomicBool>,
        consent: Arc<AtomicBool>,
        acoustic_tx: mpsc::Sender<PresenceFrame>,
        seismic_tx: mpsc::Sender<PresenceFrame>,
        solar_rx: mpsc::Receiver<SolarCell>,
        enso_rx: mpsc::Receiver<EnsoCell>,
    ) -> Self {
        let (grid0, p0, q0) = window_state_load(WINDOW_STATE_PATH);
        Self {
            rx,
            req_tx,
            res_rx,
            presence_tx,
            body_names,
            time,
            shutdown,
            consent,
            acoustic_tx,
            seismic_tx,
            silent: std::env::var("OMEGAFLOW_HIDDEN").is_ok(),
            window: None,
            surface: None,
            config: None,
            device: None,
            queue: None,
            render_pipe: None,
            probe_pipe: None,
            probe_layout: None,
            render_layout: None,
            render_binds: [None, None],
            probe_binds: [None, None],
            field_bufs: [None, None],
            meta_bufs: [None, None],
            vp_buf: None,
            probe_buf: None,
            probe_read: None,
            prep_param_buf: None,
            te_pipe: None,
            te_bind: None,
            te_series_buf: None,
            te_param_buf: None,
            te_out_buf: None,
            te_read_buf: None,
            te_map: None,
            te_named: String::new(),
            field_cap: 0,
            buf_sel: 0,
            packed_gen: 0,
            uploaded_gen: 0,
            backing: (1, 1),
            latest_field: None,
            packed_field: Vec::new(),
            packed_meta: Vec::new(),
            packed_count: 0,
            last_response_epoch: 0.0,
            p: p0,
            v: [0.0, 0.0, 0.0],
            t0: 0.0,
            t_presence: 0.0,
            q: q0,
            grid_step: grid0,
            ssaa: 1.0,
            expose_offset: EXPOSE_OFFSET_BASE,
            field_dark: false,
            hud_dark: false,
            force_ref: [0.0; 9],
            probe_omega: [0.0; 9],
            probe_flow: [0.0; 3],
            probe_ring: [[0.0; 12]; 256],
            ring_head: 0,
            ring_filled: 0,
            ring_gen: 0,
            pe_ring: Vec::with_capacity(16),
            solar: SolarMachine::new(solar_rx),
            enso: EnsoMachine::new(enso_rx),
            hud_topology: None,
            frame_ms_ema: 0.0,
            field_permeability: 0.0,
            prev_omega_sum: 0.0,
            prev_delta: 0.0,
            prev_in_te: 0.0,
            direction: 1,
            ticks_since_turn: 0,
            natural_latency_ticks: 1,
            t_thrust: 0.0,
            t_thrust_target: 0.0,
            t_frozen: false,
            keys: HashSet::new(),
            shift: false,
            focus_req: 0,
            focused: false,
            keys_seen: 0,
            cursor: None,
            drag_button: None,
            touches: HashMap::new(),
            tap: None,
            press: None,
            #[cfg(feature = "gamepad")]
            gilrs: gilrs::GilrsBuilder::new().build().ok(),
            last_tick: None,
            stable_tick: 0.0,
            size: (1280, 800),
            scale_factor: 1.0,
            last_sent: (0.0, 0.0, 0.0, 0.0, 0.0, [0.0; 3], 0.0),
            last_saved_state: (grid0, p0, q0),
            sensors: NativeSensors {
                oscs: HashMap::new(),
                tx: sensor_tx,
                frame_interval: 0.016,
            },
            frame_count: 0,
            frame_ms_max: 0.0,
            last_hud: None,
            hud_tex: None,
            hud_view: None,
            hud_sampler: None,
            color_lut_view: None,
            color_lut_sampler: None,
            hud_pipe: None,
            hud_layout: None,
            hud_bind: None,
            hud_bitmap: Vec::new(),
            hud_w: 0,
            hud_dirty: false,
        }
    }

    fn pos(&self) -> [f64; 3] {
        let dt = self.t_presence - self.t0;
        [
            self.p[0] + self.v[0] * dt,
            self.p[1] + self.v[1] * dt,
            self.p[2] + self.v[2] * dt,
        ]
    }

    fn frame(&self) -> ([f64; 3], [f64; 3], [f64; 3]) {
        (
            q_rotate(self.q, [1.0, 0.0, 0.0]),
            q_rotate(self.q, [0.0, 1.0, 0.0]),
            q_rotate(self.q, [0.0, 0.0, 1.0]),
        )
    }

    fn fold(&mut self) {
        self.p = self.pos();
        self.t0 = self.t_presence;
    }

    fn pe_gate(&mut self, pe: Option<f64>) -> bool {
        let Some(pe) = pe else {
            return true;
        };
        if self.pe_ring.len() == 16 {
            self.pe_ring.remove(0);
        }
        self.pe_ring.push(pe);
        if self.pe_ring.len() < 8 {
            return true;
        }
        let n = self.pe_ring.len() as f64;
        let mean = self.pe_ring.iter().sum::<f64>() / n;
        let var = self
            .pe_ring
            .iter()
            .map(|&p| {
                let d = p - mean;
                d * d
            })
            .sum::<f64>()
            / n;
        (pe - mean).abs() <= 2.0 * var.sqrt()
    }

    fn te_say(&mut self, word: &str) {
        if self.te_named != word {
            eprintln!("te {}", word);
            self.te_named = word.to_string();
        }
    }

    fn te_probe(
        &mut self,
        xs: &[f32],
        ys: &[f32],
        m: usize,
    ) -> Option<crate::te::TopologicalVerdict> {
        let device = self.device.clone()?;
        let mut carry: Option<crate::te::TopologicalVerdict> = None;
        if let Some(prev) = self.te_map.take() {
            if prev.load(Ordering::SeqCst) {
                let read_buf = self.te_read_buf.as_ref()?;
                let verdict = te_read_verdict(read_buf);
                carry = crate::te::topological_verdict_from_gpu(&verdict);
            } else {
                self.te_map = Some(prev);
                self.te_say("readback pending");
                return None;
            }
        }
        let queue = self.queue.clone()?;
        let pipe = self.te_pipe.as_ref()?;
        let bind = self.te_bind.as_ref()?;
        let series_buf = self.te_series_buf.as_ref()?;
        let param_buf = self.te_param_buf.as_ref()?;
        let out_buf = self.te_out_buf.as_ref()?;
        let read_buf = self.te_read_buf.as_ref()?;
        let mut data = vec![0f32; 12 * TE_SERIES_STRIDE];
        data[0..m].copy_from_slice(xs);
        data[TE_SERIES_STRIDE..TE_SERIES_STRIDE + m].copy_from_slice(ys);
        let mut rng = self.ring_gen.wrapping_add(0x9e3779b97f4a7c15);
        for s in 0..10 {
            let surr = crate::te::phase_randomized_surrogate(ys, &mut rng);
            let off = (2 + s) * TE_SERIES_STRIDE;
            data[off..off + m].copy_from_slice(&surr);
        }
        queue.write_buffer(series_buf, 0, &le_bytes_f32(&data));
        let max_lag = (m as f64 / Φ) as u32;
        let param = [m as u32, max_lag, 1.0f32.to_bits(), 0];
        let mut pb = [0u8; 16];
        for (i, x) in param.iter().enumerate() {
            pb[i * 4..i * 4 + 4].copy_from_slice(&x.to_le_bytes());
        }
        queue.write_buffer(param_buf, 0, &pb);
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(pipe);
            pass.set_bind_group(0, bind, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        enc.copy_buffer_to_buffer(out_buf, 0, read_buf, 0, 288);
        queue.submit(std::iter::once(enc.finish()));
        let mapped = Arc::new(AtomicBool::new(false));
        let m2 = mapped.clone();
        let slice = read_buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, move |r| {
            m2.store(r.is_ok(), Ordering::SeqCst);
        });
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(100);
        while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
            device.poll(wgpu::Maintain::Poll);
        }
        if !mapped.load(Ordering::SeqCst) {
            self.te_map = Some(mapped);
            if carry.is_some() {
                self.te_say("verdict present");
            } else {
                self.te_say("readback pending");
            }
            return carry;
        }
        let verdict = te_read_verdict(read_buf);
        let v = crate::te::topological_verdict_from_gpu(&verdict);
        let final_v = v.or(carry);
        if final_v.is_some() {
            self.te_say("verdict present");
        } else {
            self.te_say(te_absence_word(&verdict));
        }
        final_v
    }

    fn hud_blit(bmp: &mut [u8], stride: usize, w: u32, x: i32, y: i32, ch: char, rgb: [u8; 3]) {
        let c = ch as u32;
        if !(32..=126).contains(&c) {
            return;
        }
        let glyph = &HUD_GLYPH[(c - 32) as usize];
        for (col, bits) in glyph.iter().enumerate() {
            let px = x + col as i32;
            if px < 0 || px >= w as i32 {
                continue;
            }
            for row in 0..7u32 {
                if bits & (1 << row) == 0 {
                    continue;
                }
                let py = y + row as i32;
                if py < 0 || py >= HUD_H as i32 {
                    continue;
                }
                let o = py as usize * stride + px as usize * 4;
                bmp[o] = rgb[0];
                bmp[o + 1] = rgb[1];
                bmp[o + 2] = rgb[2];
                bmp[o + 3] = 255;
            }
        }
    }

    fn hud_text(bmp: &mut [u8], stride: usize, w: u32, mut x: i32, y: i32, s: &str, rgb: [u8; 3]) {
        for ch in s.chars() {
            Self::hud_blit(bmp, stride, w, x, y, ch, rgb);
            x += HUD_CHAR_W;
        }
    }

    fn hud_raster(&mut self, force_tokens: &str, te: Option<(f64, f64)>, te_word: &str) {
        if self.hud_bitmap.is_empty() {
            return;
        }
        let w = self.hud_w;
        let stride = (w as usize * 4 + 255) / 256 * 256;
        for b in self.hud_bitmap.iter_mut() {
            *b = 0;
        }
        let green = [120u8, 255, 140];
        let [x, y, z] = self.pos();
        let line1 = format!(
            "t {:.2}  x {:.3e}  y {:.3e}  z {:.3e}",
            self.t_presence, x, y, z
        );
        let flow = self.probe_flow;
        let mut l3 = match te {
            Some((in_te, thr)) => format!("TE {:.3} thr {:.3}  ", in_te, thr),
            None if te_word.is_empty() => String::new(),
            None => format!("TE {}  ", te_word),
        };
        if let Some((tx, ty, px, py)) = self.hud_topology {
            l3.push_str(&format!("tau {}:{} ", tx, ty));
            match (px, py) {
                (Some(a), Some(b)) => l3.push_str(&format!("PE {:.2}:{:.2} ", a, b)),
                _ => l3.push_str("PE - "),
            }
        }
        l3.push_str(&format!(
            "perm {:.2} flow {:+.2} {:+.2} {:+.2} gen {}",
            self.field_permeability, flow[0], flow[1], flow[2], self.ring_gen
        ));
        let x0 = 4i32;
        let line = HUD_LINE_H;
        let bmp = &mut self.hud_bitmap;
        let scale_line = format!(
            "1 px = {} | grid 2^{} | H hud  P feld | {} | keys {}",
            Self::scale_label(self.grid_step),
            self.grid_step.log2().round() as i64,
            if self.focused { "focus" } else { "kein focus" },
            self.keys_seen
        );
        Self::hud_text(bmp, stride, w, x0, 1, &line1, green);
        Self::hud_text(bmp, stride, w, x0, 1 + line, force_tokens, green);
        Self::hud_text(bmp, stride, w, x0, 1 + 2 * line, &l3, green);
        Self::hud_text(bmp, stride, w, x0, 1 + 3 * line, &scale_line, green);
        self.hud_dirty = true;
    }

    fn scale_label(m_per_px: f64) -> String {
        if m_per_px >= 1.0e14 {
            format!("{:.2} AU", m_per_px / 1.495978707e11)
        } else if m_per_px >= 1.0e8 {
            format!("{:.3} Mkm", m_per_px / 1.0e9)
        } else if m_per_px >= 1.0e3 {
            format!("{:.1} km", m_per_px / 1.0e3)
        } else {
            format!("{:.1} m", m_per_px)
        }
    }

    fn ensure_hud_texture(&mut self) {
        let Some(device) = self.device.clone() else {
            return;
        };
        let w = self.backing.0.max(1);
        if self.hud_w == w && self.hud_tex.is_some() && self.hud_bind.is_some() {
            return;
        }
        self.hud_w = w;
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: w,
                height: HUD_H,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let stride = (w as usize * 4 + 255) / 256 * 256;
        self.hud_bitmap = vec![0u8; stride * HUD_H as usize];
        if let (Some(sampler), Some(layout), Some(vp_buf)) = (
            self.hud_sampler.clone(),
            self.hud_layout.clone(),
            self.vp_buf.clone(),
        ) {
            self.hud_bind = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: vp_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 10,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 11,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            }));
        }
        self.hud_tex = Some(tex);
        self.hud_view = Some(view);
        self.hud_dirty = true;
    }

    fn sense(&mut self) {
        let Some(field) = self.latest_field.clone() else {
            return;
        };
        let center = self.pos();
        let t = self.t_presence;
        let hx = self.size.0 as f64 * self.scale_factor * self.grid_step * 0.5;
        let hy = self.size.1 as f64 * self.scale_factor * self.grid_step * 0.5;
        let pad = 2.0 * (hx * hx + hy * hy).sqrt();
        let cache_interval = (self.grid_step / 30000.0).clamp(Φ, Φ * 10.0);
        let (_, _, forward) = self.frame();
        let _ = self.req_tx.try_send(SenseReq {
            field,
            center,
            t,
            pad,
            cache_interval,
            forward,
            expose_offset: self.expose_offset,
            force_ref: self.force_ref,
            softening: self.grid_step,
        });
    }

    fn consider_resend(&mut self) {
        let [x, y, z] = self.pos();
        let (lt, lx, ly, lz, ls, lv, ltt) = self.last_sent;
        let moved = self.t_presence != lt
            || (x - lx).abs() >= self.grid_step
            || (y - ly).abs() >= self.grid_step
            || (z - lz).abs() >= self.grid_step
            || self.grid_step > ls * Φ
            || ls > self.grid_step * Φ
            || self.v != lv
            || self.t_thrust != ltt;
        if !moved {
            return;
        }
        self.last_sent = (
            self.t_presence,
            x,
            y,
            z,
            self.grid_step,
            self.v,
            self.t_thrust,
        );
        let range = self.size.0.max(self.size.1) as f64 * self.scale_factor * self.grid_step * 2.0;
        let _ = self.presence_tx.send((
            "native".to_string(),
            self.t_presence,
            x,
            y,
            z,
            range,
            self.v[0],
            self.v[1],
            self.v[2],
            self.t_thrust,
            self.grid_step,
        ));
        self.sense();
    }

    fn consider_state_save(&mut self) {
        let state = (self.grid_step, self.p, self.q);
        if state.0 == self.last_saved_state.0
            && state.1 == self.last_saved_state.1
            && state.2 == self.last_saved_state.2
        {
            return;
        }
        self.last_saved_state = state;
        window_state_save(WINDOW_STATE_PATH, state.0, state.1, state.2);
    }

    fn key_action(&mut self, code: KeyCode) {
        match code {
            KeyCode::KeyS => {
                self.fold();
                self.v = [0.0, 0.0, 0.0];
                self.consider_resend();
            }
            KeyCode::KeyY => {
                self.consent.store(true, Ordering::SeqCst);
            }
            KeyCode::KeyN => {
                self.consent.store(false, Ordering::SeqCst);
            }
            KeyCode::Home | KeyCode::Digit0 => {
                self.p = [0.0, 0.0, 0.0];
                self.v = [0.0, 0.0, 0.0];
                self.t0 = self.t_presence;
                self.consider_resend();
            }
            KeyCode::Space => {
                self.t_frozen = !self.t_frozen;
            }
            KeyCode::KeyB => {
                self.q = [1.0, 0.0, 0.0, 0.0];
            }
            KeyCode::Minus => {
                self.grid_step *= 4.0;
                self.consider_resend();
            }
            KeyCode::Equal => {
                self.grid_step /= 4.0;
                self.consider_resend();
            }
            KeyCode::KeyQ => {
                self.ssaa = if self.shift {
                    (self.ssaa / Φ as f32).max(1.0)
                } else {
                    (self.ssaa * Φ as f32).min(SSAA_MAX)
                };
                self.reconfigure();
            }
            KeyCode::KeyE => {
                self.expose_offset = if self.shift {
                    self.expose_offset / Φ as f32
                } else {
                    self.expose_offset * 2.0
                };
            }
            KeyCode::KeyP => {
                self.field_dark = !self.field_dark;
            }
            KeyCode::KeyH => {
                self.hud_dark = !self.hud_dark;
            }
            KeyCode::Digit1 => self.jump(0),
            KeyCode::Digit2 => self.jump(1),
            KeyCode::Digit3 => self.jump(2),
            KeyCode::Digit4 => self.jump(3),
            KeyCode::Digit5 => self.jump(4),
            KeyCode::Digit6 => self.jump(5),
            KeyCode::Digit7 => self.jump(6),
            KeyCode::Digit8 => self.jump(7),
            KeyCode::Digit9 => self.jump(8),
            _ => {}
        }
    }

    fn jump(&mut self, idx: usize) {
        let Some(target) = JUMP_BODIES.get(idx) else {
            return;
        };
        let Some(field) = self.latest_field.clone() else {
            return;
        };
        let Some(name) = self.body_names.iter().find(|n| n.as_str() == *target) else {
            return;
        };
        let eph = field.eph.clone();
        let Some(pos) = body_barycenter_position(name, self.t_presence, &eph) else {
            return;
        };
        self.p = pos;
        self.v = [0.0, 0.0, 0.0];
        self.t0 = self.t_presence;
        self.grid_step = JUMP_GRID;
        self.consider_resend();
    }

    fn reconfigure(&mut self) {
        let Some(device) = &self.device else {
            return;
        };
        let Some(surface) = &self.surface else {
            return;
        };
        let mut config = match &self.config {
            Some(c) => c.clone(),
            None => return,
        };
        let w = (self.size.0 as f64
            * self.scale_factor
            * self.ssaa as f64
            * FIELD_BACKING_SCALE as f64)
            .round() as u32;
        let h = (self.size.1 as f64
            * self.scale_factor
            * self.ssaa as f64
            * FIELD_BACKING_SCALE as f64)
            .round() as u32;
        config.width = w.max(1);
        config.height = h.max(1);
        surface.configure(device, &config);
        self.config = Some(config);
        self.backing = (w.max(1), h.max(1));
    }

    fn vp_data(&self) -> [f32; 36] {
        let (fr, fu, ff) = self.frame();
        let [x, y, z] = self.pos();
        [
            self.backing.0 as f32,
            self.backing.1 as f32,
            self.packed_count as f32,
            self.grid_step as f32,
            fr[0] as f32,
            fr[1] as f32,
            fr[2] as f32,
            (self.v[0] / C) as f32,
            fu[0] as f32,
            fu[1] as f32,
            fu[2] as f32,
            (self.v[1] / C) as f32,
            ff[0] as f32,
            ff[1] as f32,
            ff[2] as f32,
            (self.v[2] / C) as f32,
            self.expose_offset,
            self.last_response_epoch as f32,
            0.0,
            0.0,
            x as f32,
            y as f32,
            z as f32,
            self.t_presence as f32,
            self.force_ref[0],
            self.force_ref[1],
            self.force_ref[2],
            self.force_ref[3],
            self.force_ref[4],
            self.force_ref[5],
            self.force_ref[6],
            self.force_ref[7],
            self.force_ref[8],
            0.0,
            0.0,
            0.0,
        ]
    }

    fn relax_force_refs(&mut self) {
        let meds = force_ref_medians(&self.packed_field, &self.packed_meta);
        for (ft, m) in meds.iter().enumerate() {
            let Some(median) = m else {
                continue;
            };
            if self.force_ref[ft] == 0.0 {
                self.force_ref[ft] = *median;
                continue;
            }
            self.force_ref[ft] += (median - self.force_ref[ft]) * REF_RELAX;
        }
    }

    fn rebuild_binds(&mut self) {
        let Some(device) = self.device.clone() else {
            return;
        };
        let Some(probe_layout) = self.probe_layout.clone() else {
            return;
        };
        let Some(render_layout) = self.render_layout.clone() else {
            return;
        };
        let Some(vp_buf) = self.vp_buf.clone() else {
            return;
        };
        let Some(probe_buf) = self.probe_buf.clone() else {
            return;
        };
        let Some(prep_param_buf) = self.prep_param_buf.clone() else {
            return;
        };
        let Some(color_lut_view) = self.color_lut_view.clone() else {
            return;
        };
        let Some(color_lut_sampler) = self.color_lut_sampler.clone() else {
            return;
        };
        for sel in 0..2 {
            let Some(field_buf) = self.field_bufs[sel].clone() else {
                continue;
            };
            let Some(meta_buf) = self.meta_bufs[sel].clone() else {
                continue;
            };
            self.probe_binds[sel] = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &probe_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: field_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: meta_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: vp_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: probe_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: prep_param_buf.as_entire_binding(),
                    },
                ],
            }));
            self.render_binds[sel] = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &render_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: field_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: meta_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: vp_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: prep_param_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 9,
                        resource: wgpu::BindingResource::TextureView(&color_lut_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 12,
                        resource: wgpu::BindingResource::Sampler(&color_lut_sampler),
                    },
                ],
            }));
        }
    }

    fn ensure_capacity(&mut self) {
        let Some(device) = self.device.clone() else {
            return;
        };
        let n = self.packed_count;
        if self.field_cap >= n {
            return;
        }
        let mut c = if self.field_cap > 0 {
            self.field_cap
        } else {
            256
        };
        while c < n {
            c <<= 1;
        }
        let max_buf = device.limits().max_buffer_size;
        while c as u64 * 96 > max_buf {
            c >>= 1;
        }
        if c < 256 {
            c = 256;
        }
        self.field_cap = c;
        for sel in 0..2 {
            self.field_bufs[sel] = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: c as u64 * 48,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
            self.meta_bufs[sel] = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: c as u64 * 64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
        let prep_param_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: c as u64 * 32,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.prep_param_buf = Some(prep_param_buf);
        self.rebuild_binds();
    }

    fn render(&mut self) {
        let t0 = std::time::Instant::now();
        self.frame_count += 1;
        let Some(device) = self.device.clone() else {
            return;
        };
        let Some(queue) = self.queue.clone() else {
            return;
        };
        if self.packed_gen != self.uploaded_gen {
            self.ensure_capacity();
            let sel = self.buf_sel ^ 1;
            if let Some(fb) = &self.field_bufs[sel] {
                queue.write_buffer(fb, 0, &le_bytes_f32(&self.packed_field));
            }
            if let Some(mb) = &self.meta_bufs[sel] {
                queue.write_buffer(mb, 0, &le_bytes_f32(&self.packed_meta));
            }
            self.buf_sel = sel;
            self.uploaded_gen = self.packed_gen;
        }
        let vp = self.vp_data();
        let mut bytes = [0u8; 144];
        for (i, x) in vp.iter().enumerate() {
            bytes[i * 4..i * 4 + 4].copy_from_slice(&x.to_le_bytes());
        }
        let Some(vp_buf) = self.vp_buf.as_ref() else {
            return;
        };
        queue.write_buffer(vp_buf, 0, &bytes);
        if self.hud_dirty {
            if let Some(tex) = self.hud_tex.as_ref() {
                let stride = (self.hud_w * 4 + 255) / 256 * 256;
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: tex,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &self.hud_bitmap,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(stride),
                        rows_per_image: Some(HUD_H),
                    },
                    wgpu::Extent3d {
                        width: self.hud_w,
                        height: HUD_H,
                        depth_or_array_layers: 1,
                    },
                );
            }
            self.hud_dirty = false;
        }
        let mut probe_enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = probe_enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            let sel = self.buf_sel;
            if let (Some(pipe), Some(bind)) =
                (self.probe_pipe.as_ref(), self.probe_binds[sel].as_ref())
            {
                pass.set_pipeline(pipe);
                pass.set_bind_group(0, bind, &[]);
                pass.dispatch_workgroups(1, 1, 1);
            }
        }
        queue.submit(std::iter::once(probe_enc.finish()));
        if self.silent {
            return;
        }
        let Some(surface) = self.surface.as_ref() else {
            return;
        };
        let frame = match surface.get_current_texture() {
            Ok(f) => f,
            Err(_) => return,
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            if !self.field_dark {
                if let (Some(pipe), Some(bind)) = (
                    self.render_pipe.as_ref(),
                    self.render_binds[self.buf_sel].as_ref(),
                ) {
                    pass.set_pipeline(pipe);
                    pass.set_bind_group(0, bind, &[]);
                    pass.draw(0..6, 0..1);
                }
            }
            if !self.hud_dark {
                if let (Some(pipe), Some(bind)) = (self.hud_pipe.as_ref(), self.hud_bind.as_ref()) {
                    pass.set_pipeline(pipe);
                    pass.set_bind_group(0, bind, &[]);
                    pass.draw(0..6, 0..1);
                }
            }
        }
        queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        if ms > self.frame_ms_max {
            self.frame_ms_max = ms;
        }
    }

    fn init_gpu(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let mut attrs = WindowAttributes::default()
            .with_title("omegaflow φ")
            .with_active(true)
            .with_fullscreen(Some(Fullscreen::Borderless(None)));
        if std::env::var("OMEGAFLOW_HIDDEN").is_ok() {
            attrs = attrs.with_visible(false);
        }
        let window = match event_loop.create_window(attrs) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("window creation returned: {}", e);
                return;
            }
        };
        if std::env::var("OMEGAFLOW_HIDDEN").is_err() {
            let _ = window.focus_window();
        }
        let (sw, sh) = match window.current_monitor() {
            Some(m) => {
                let ms = m.size();
                (ms.width.max(1), ms.height.max(1))
            }
            None => {
                let inner = window.inner_size();
                (inner.width.max(1), inner.height.max(1))
            }
        };
        self.size = (sw, sh);
        self.scale_factor = 1.0;
        let window = Arc::new(window);
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = match instance.create_surface(window.clone()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("surface creation returned: {}", e);
                return;
            }
        };
        let adapter =
            match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })) {
                Some(a) => a,
                None => {
                    eprintln!("adapter request returned void");
                    return;
                }
            };
        let info = adapter.get_info();
        eprintln!(
            "adapter: {} | {:?} | {:?} | {}",
            info.name, info.backend, info.device_type, info.driver_info
        );
        let (device, queue) = match pollster::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None),
        ) {
            Ok(dq) => dq,
            Err(e) => {
                eprintln!("device request returned: {}", e);
                return;
            }
        };
        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: 1,
            height: 1,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(FIELD_WGSL.into()),
        });
        let probe_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::COMPUTE);
                    e.binding = 0;
                    e
                },
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::COMPUTE);
                    e.binding = 1;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                {
                    let mut e = storage_entry(false, wgpu::ShaderStages::COMPUTE);
                    e.binding = 3;
                    e
                },
                {
                    let mut e = storage_entry(false, wgpu::ShaderStages::COMPUTE);
                    e.binding = 4;
                    e
                },
            ],
        });
        let render_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::VERTEX_FRAGMENT);
                    e.binding = 0;
                    e
                },
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::VERTEX_FRAGMENT);
                    e.binding = 1;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                {
                    let mut e = storage_entry(false, wgpu::ShaderStages::FRAGMENT);
                    e.binding = 4;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 9,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 12,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });
        let probe_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&probe_layout],
            push_constant_ranges: &[],
        });
        let render_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&render_layout],
            push_constant_ranges: &[],
        });
        let render_pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipe_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        let probe_pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&probe_pipe_layout),
            module: &module,
            entry_point: Some("presence_probe"),
            compilation_options: Default::default(),
            cache: None,
        });
        let hud_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let color_lut_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: crate::spectral::COLOR_LUT_LEN as u32,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let mut lut_bytes = Vec::with_capacity(crate::spectral::COLOR_LUT_LEN * 16);
        for e in crate::spectral::color_lut_rgba() {
            for v in e {
                lut_bytes.extend_from_slice(&v.to_le_bytes());
            }
        }
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &color_lut_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &lut_bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some((crate::spectral::COLOR_LUT_LEN * 16) as u32),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: crate::spectral::COLOR_LUT_LEN as u32,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        self.color_lut_view =
            Some(color_lut_tex.create_view(&wgpu::TextureViewDescriptor::default()));
        self.color_lut_sampler = Some(device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        }));
        let hud_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 10,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 11,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let hud_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&hud_layout],
            push_constant_ranges: &[],
        });
        let hud_pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&hud_pipe_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("hud_vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("hud_fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        self.hud_sampler = Some(hud_sampler);
        self.hud_layout = Some(hud_layout);
        self.hud_pipe = Some(hud_pipe);
        let vp_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 144,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let probe_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 48,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let probe_read = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 48,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(TE_WGSL.into()),
        });
        let te_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::COMPUTE);
                    e.binding = 0;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                {
                    let mut e = storage_entry(false, wgpu::ShaderStages::COMPUTE);
                    e.binding = 2;
                    e
                },
            ],
        });
        let te_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&te_layout],
            push_constant_ranges: &[],
        });
        let te_pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&te_pipe_layout),
            module: &te_module,
            entry_point: Some("te_compute"),
            compilation_options: Default::default(),
            cache: None,
        });
        let te_series_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: TE_SERIES_BYTES,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_param_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_out_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let te_read_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &te_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: te_series_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: te_param_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: te_out_buf.as_entire_binding(),
                },
            ],
        });
        self.te_pipe = Some(te_pipe.clone());
        self.te_bind = Some(te_bind);
        self.te_series_buf = Some(te_series_buf);
        self.te_param_buf = Some(te_param_buf);
        self.te_out_buf = Some(te_out_buf);
        self.te_read_buf = Some(te_read_buf);
        self.solar
            .bind_gpu(&device, &queue, te_pipe.clone(), &te_layout);
        self.enso.bind_gpu(&device, &queue, te_pipe, &te_layout);
        self.window = Some(window);
        self.surface = Some(surface);
        self.config = Some(config);
        self.device = Some(device);
        self.queue = Some(queue);
        self.render_pipe = Some(render_pipe);
        self.probe_pipe = Some(probe_pipe);
        self.probe_layout = Some(probe_layout);
        self.render_layout = Some(render_layout);
        self.vp_buf = Some(vp_buf);
        self.probe_buf = Some(probe_buf);
        self.probe_read = Some(probe_read);
        self.reconfigure();
        self.ensure_capacity();
        self.ensure_hud_texture();
    }
}

impl ApplicationHandler for NativeApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.init_gpu(event_loop);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.shutdown.load(Ordering::SeqCst) {
            event_loop.exit();
            return;
        }
        if self.window.is_none() {
            return;
        }
        if !self.focused {
            self.focus_req += 1;
            if self.focus_req % 30 == 0 {
                if let Some(w) = self.window.as_ref() {
                    let _ = w.focus_window();
                }
            }
        }
        let now_i = std::time::Instant::now();
        let raw = match self.last_tick {
            Some(prev) => now_i.duration_since(prev).as_secs_f64() * 1000.0,
            None => Φ,
        };
        self.last_tick = Some(now_i);
        self.stable_tick = self.stable_tick * (1.0 - EMA_FACTOR) + raw * EMA_FACTOR;
        self.expose_offset += (EXPOSE_OFFSET_BASE - self.expose_offset) * OFFSET_RELAX;
        self.consider_state_save();
        if let Some(t) = system_now(&self.time) {
            if self.t_presence == 0.0 {
                self.t_presence = t;
                self.t0 = t;
            }
        }
        if self.keys.contains(&KeyCode::Comma) {
            self.t_thrust_target = -THRUST_STEP;
        } else if self.keys.contains(&KeyCode::Period) {
            self.t_thrust_target = THRUST_STEP;
        } else {
            self.t_thrust_target = 0.0;
        }
        self.t_thrust += (self.t_thrust_target - self.t_thrust)
            * (1.0 - (-raw / self.stable_tick.max(raw)).exp());
        if !self.t_frozen {
            self.t_presence += (1.0 + self.t_thrust) * raw / 1000.0;
        }
        let (fr, fu, ff) = self.frame();
        let thrust_speed = self.grid_step * raw / 1000.0;
        let pan_speed = self.grid_step * if self.shift { 4.0 } else { 1.0 } * raw / 1000.0;
        let mut thrust = [0.0f64; 3];
        let mut pan = [0.0f64; 3];
        if self.keys.contains(&KeyCode::ArrowRight) {
            if self.shift {
                for i in 0..3 {
                    pan[i] += fr[i];
                }
            } else {
                for i in 0..3 {
                    thrust[i] += fr[i];
                }
            }
        }
        if self.keys.contains(&KeyCode::ArrowLeft) {
            if self.shift {
                for i in 0..3 {
                    pan[i] -= fr[i];
                }
            } else {
                for i in 0..3 {
                    thrust[i] -= fr[i];
                }
            }
        }
        if self.keys.contains(&KeyCode::ArrowUp) {
            if self.shift {
                for i in 0..3 {
                    pan[i] += ff[i];
                }
            } else {
                for i in 0..3 {
                    thrust[i] += ff[i];
                }
            }
        }
        if self.keys.contains(&KeyCode::ArrowDown) {
            if self.shift {
                for i in 0..3 {
                    pan[i] -= ff[i];
                }
            } else {
                for i in 0..3 {
                    thrust[i] -= ff[i];
                }
            }
        }
        if self.keys.contains(&KeyCode::PageUp) {
            for i in 0..3 {
                pan[i] += fu[i];
            }
        }
        if self.keys.contains(&KeyCode::PageDown) {
            for i in 0..3 {
                pan[i] -= fu[i];
            }
        }
        if thrust != [0.0, 0.0, 0.0] {
            self.fold();
            for i in 0..3 {
                self.v[i] += thrust[i] * thrust_speed;
            }
            self.consider_resend();
        }
        if pan != [0.0, 0.0, 0.0] {
            for i in 0..3 {
                self.p[i] += pan[i] * pan_speed;
            }
            self.consider_resend();
        }
        if self.v != [0.0, 0.0, 0.0] {
            self.consider_resend();
        }
        #[cfg(feature = "gamepad")]
        {
            let mut gilrs_state = self.gilrs.take();
            if let Some(gilrs) = gilrs_state.as_mut() {
                while let Some(ev) = gilrs.next_event() {
                    if let gilrs::EventType::ButtonPressed(button, _) = ev.event {
                        match button {
                            gilrs::Button::South => {
                                self.fold();
                                self.v = [0.0, 0.0, 0.0];
                            }
                            gilrs::Button::East => self.jump(0),
                            gilrs::Button::North => {
                                self.p = [0.0, 0.0, 0.0];
                                self.v = [0.0, 0.0, 0.0];
                                self.t0 = self.t_presence;
                            }
                            gilrs::Button::West => self.t_frozen = !self.t_frozen,
                            gilrs::Button::Start => {
                                self.shutdown.store(true, Ordering::SeqCst);
                            }
                            _ => {}
                        }
                    }
                }
                if let Some((_, gp)) = gilrs.gamepads().next() {
                    let rx = gp.value(gilrs::Axis::LeftStickX);
                    let ry = gp.value(gilrs::Axis::LeftStickY);
                    let lx = gp.value(gilrs::Axis::RightStickX);
                    let ly = gp.value(gilrs::Axis::RightStickY);
                    let l2 = gp.value(gilrs::Axis::LeftZ);
                    let r2 = gp.value(gilrs::Axis::RightZ);
                    let roll = if gp.is_pressed(gilrs::Button::LeftTrigger) {
                        -1.0
                    } else if gp.is_pressed(gilrs::Button::RightTrigger) {
                        1.0
                    } else {
                        0.0
                    };
                    let (fr, fu, ff) = self.frame();
                    let rot = Φ * raw / 1000.0;
                    if rx.abs() > 0.15 {
                        self.q = q_norm(q_mul(q_axis_angle(fu, rx as f64 * rot), self.q));
                    }
                    if ry.abs() > 0.15 {
                        self.q = q_norm(q_mul(q_axis_angle(fr, ry as f64 * rot), self.q));
                    }
                    if roll != 0.0 {
                        self.q = q_norm(q_mul(q_axis_angle(ff, roll * rot), self.q));
                    }
                    let pan_sp = self.grid_step * raw / 1000.0;
                    if lx.abs() > 0.15 || ly.abs() > 0.15 {
                        self.p[0] += fr[0] * lx as f64 * pan_sp + fu[0] * ly as f64 * pan_sp;
                        self.p[1] += fr[1] * lx as f64 * pan_sp + fu[1] * ly as f64 * pan_sp;
                        self.p[2] += fr[2] * lx as f64 * pan_sp + fu[2] * ly as f64 * pan_sp;
                        self.consider_resend();
                    }
                    if l2 > 0.2 {
                        self.grid_step *= 1.0 + l2 as f64 * raw / 1000.0;
                        self.consider_resend();
                    }
                    if r2 > 0.2 {
                        self.grid_step /= 1.0 + r2 as f64 * raw / 1000.0;
                        self.consider_resend();
                    }
                    let thrust_sp = self.grid_step * raw / 1000.0;
                    let mut thrust = [0.0f64; 3];
                    if gp.is_pressed(gilrs::Button::DPadUp) {
                        thrust[0] += ff[0];
                        thrust[1] += ff[1];
                        thrust[2] += ff[2];
                    }
                    if gp.is_pressed(gilrs::Button::DPadDown) {
                        thrust[0] -= ff[0];
                        thrust[1] -= ff[1];
                        thrust[2] -= ff[2];
                    }
                    if gp.is_pressed(gilrs::Button::DPadRight) {
                        thrust[0] += fr[0];
                        thrust[1] += fr[1];
                        thrust[2] += fr[2];
                    }
                    if gp.is_pressed(gilrs::Button::DPadLeft) {
                        thrust[0] -= fr[0];
                        thrust[1] -= fr[1];
                        thrust[2] -= fr[2];
                    }
                    if thrust != [0.0, 0.0, 0.0] {
                        self.fold();
                        self.v[0] += thrust[0] * thrust_sp;
                        self.v[1] += thrust[1] * thrust_sp;
                        self.v[2] += thrust[2] * thrust_sp;
                        self.consider_resend();
                    }
                }
            }
            self.gilrs = gilrs_state;
        }
        if let Ok(field) = self.rx.try_recv() {
            self.latest_field = Some(field);
            self.sense();
        }
        while let Ok((packed, t, generation)) = self.res_rx.try_recv() {
            self.packed_count = packed.count;
            self.packed_field = packed.field;
            self.packed_meta = packed.meta;
            self.packed_gen = generation;
            self.last_response_epoch = t;
            self.relax_force_refs();
        }
        self.consider_resend();
        self.sensors.frame_interval = (raw / 1000.0).max(0.001);
        if self
            .last_hud
            .map_or(true, |i| i.elapsed().as_secs_f64() >= 1.0)
        {
            self.last_hud = Some(now_i);
            self.sensors.flush();
            if let (Some(device), Some(queue), Some(probe_buf), Some(probe_read)) = (
                self.device.clone(),
                self.queue.clone(),
                self.probe_buf.clone(),
                self.probe_read.clone(),
            ) {
                let mut enc =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                enc.copy_buffer_to_buffer(&probe_buf, 0, &probe_read, 0, 48);
                queue.submit(std::iter::once(enc.finish()));
                let mapped = Arc::new(AtomicBool::new(false));
                let m2 = mapped.clone();
                let slice = probe_read.slice(..);
                slice.map_async(wgpu::MapMode::Read, move |r| {
                    m2.store(r.is_ok(), Ordering::SeqCst);
                });
                let deadline = std::time::Instant::now() + std::time::Duration::from_millis(5);
                while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
                    device.poll(wgpu::Maintain::Poll);
                }
                if mapped.load(Ordering::SeqCst) {
                    let data = slice.get_mapped_range();
                    let mut v = [0f32; 12];
                    for k in 0..12 {
                        let mut b = [0u8; 4];
                        b.copy_from_slice(&data[k * 4..k * 4 + 4]);
                        v[k] = f32::from_le_bytes(b);
                    }
                    drop(data);
                    self.probe_omega.copy_from_slice(&v[0..9]);
                    self.probe_flow.copy_from_slice(&v[9..12]);
                    self.probe_ring[self.ring_head] = v;
                    self.ring_head = (self.ring_head + 1) % 256;
                    if self.ring_filled < 256 {
                        self.ring_filled += 1;
                    }
                    self.ring_gen += 1;
                }
                probe_read.unmap();
            }
            let mut te_opt: Option<(f64, f64)> = None;
            if self.ring_filled >= 32 {
                let m = self.ring_filled;
                let mut xs = vec![0f32; m];
                let mut ys = vec![0f32; m];
                for i in 0..m {
                    let idx = (self.ring_head + 256 - m + i) % 256;
                    let v = self.probe_ring[idx];
                    xs[i] = v[0] + v[1] + v[2] + v[3] + v[4] + v[5] + v[6] + v[7] + v[8];
                    ys[i] = (v[9] * v[9] + v[10] * v[10] + v[11] * v[11]).sqrt();
                }
                if let Some(v) = self.te_probe(&xs, &ys, m) {
                    self.hud_topology = Some((v.tau_x, v.tau_y, v.pe_x, v.pe_y));
                    if self.pe_gate(v.pe_y) {
                        te_opt = Some((v.te, v.threshold));
                    }
                }
            }
            self.solar.tick(self.ring_gen);
            self.enso.tick(self.ring_gen);
            if let Some((in_te, threshold)) = te_opt {
                let delta_te = in_te - self.prev_in_te;
                self.prev_in_te = in_te;
                self.ticks_since_turn += 1;
                if self.direction > 0 && delta_te < -threshold {
                    self.natural_latency_ticks = self.ticks_since_turn.max(1);
                    self.ticks_since_turn = 0;
                    self.direction = -1;
                }
                if self.direction < 0
                    && (delta_te > threshold || self.field_permeability <= PERM_GROUND)
                {
                    self.natural_latency_ticks = self.ticks_since_turn.max(1);
                    self.ticks_since_turn = 0;
                    self.direction = 1;
                }
                let target =
                    (in_te.max(0.0) / (in_te.max(0.0) + threshold + PERM_GROUND as f64)) as f32;
                let alpha = 1.0 - (-1.0 / self.natural_latency_ticks.max(1) as f32).exp();
                self.field_permeability += (target - self.field_permeability) * alpha;
                self.field_permeability = self.field_permeability.clamp(PERM_GROUND, 1.0);
            } else {
                let omega_sum: f32 = self.probe_omega.iter().sum();
                let delta = omega_sum - self.prev_omega_sum;
                if self.prev_delta != 0.0 && delta * self.prev_delta < 0.0 {
                    self.natural_latency_ticks = self.ticks_since_turn.max(1);
                    self.ticks_since_turn = 0;
                }
                self.ticks_since_turn += 1;
                self.prev_delta = delta;
                self.prev_omega_sum = omega_sum;
                let g = omega_sum.abs();
                let v_c = delta.abs();
                let target = (v_c / (g + PERM_GROUND)).tanh();
                let alpha = 1.0 - (-1.0 / self.natural_latency_ticks.max(1) as f32).exp();
                self.field_permeability += (target - self.field_permeability) * alpha;
                self.field_permeability = self.field_permeability.clamp(PERM_GROUND, 1.0);
            }
            let frame = PresenceFrame {
                omega: self.probe_omega,
            };
            if !self.silent {
                let _ = self.acoustic_tx.send(frame);
                let _ = self.seismic_tx.send(frame);
            }
            let [x, y, z] = self.pos();
            let fps = self.frame_count as f64;
            self.frame_count = 0;
            let ms_max = self.frame_ms_max;
            self.frame_ms_max = 0.0;
            let avg_ms = 1000.0 / fps.max(1.0);
            self.frame_ms_ema = if self.frame_ms_ema == 0.0 {
                avg_ms
            } else {
                self.frame_ms_ema * (1.0 - BUDGET_RELAX) + avg_ms * BUDGET_RELAX
            };
            let rec = if self.consent.load(Ordering::SeqCst) {
                "on"
            } else {
                "silent"
            };
            let mut force_tokens = String::new();
            for k in 0..9 {
                if k > 0 {
                    force_tokens.push(' ');
                }
                let v = self.probe_omega[k];
                if v != 0.0 && v.abs() < 0.01 {
                    force_tokens.push_str(&format!(
                        "{}[{}]:{:+.1e}",
                        FORCE_NAME[k], FORCE_SI_UNIT[k], v
                    ));
                } else {
                    force_tokens.push_str(&format!(
                        "{}[{}]:{:+.2}",
                        FORCE_NAME[k], FORCE_SI_UNIT[k], v
                    ));
                }
            }
            let te_word = if self.ring_filled >= 32 {
                self.te_named.clone()
            } else {
                "ring below gate".to_string()
            };
            if !self.hud_dark {
                self.hud_raster(&force_tokens, te_opt, &te_word);
            }
            let (te_s, thr_s) = match te_opt {
                Some((t, h)) => (format!("{:.3}", t), format!("{:.3}", h)),
                None => ("-".to_string(), "-".to_string()),
            };
            let (tau_s, pe_s) = match self.hud_topology {
                Some((tx, ty, px, py)) => {
                    let pe = match (px, py) {
                        (Some(a), Some(b)) => format!("{:.2}:{:.2}", a, b),
                        _ => "-".to_string(),
                    };
                    (format!("{}:{}", tx, ty), pe)
                }
                None => ("-".to_string(), "-".to_string()),
            };
            eprintln!(
                "φ window: t {:.2} | rec {} | gen {} | flow {:+.2} {:+.2} {:+.2} | {} | fps {:.0} | ssaa {:.2} | grid 2^{} | x {:.3e} y {:.3e} z {:.3e} | {} recs | b {}x{} | maxms {:.0} | ema {:.1} | perm {:.2} | off {:.2} | field {} | refs {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} | te {} thr {} | tau {} | pe {} | state {} | focus {} | keys {}",
                self.t_presence,
                rec,
                self.ring_gen,
                self.probe_flow[0],
                self.probe_flow[1],
                self.probe_flow[2],
                force_tokens,
                fps,
                self.ssaa,
                self.grid_step.log2().round() as i64,
                x,
                y,
                z,
                self.packed_count,
                self.backing.0,
                self.backing.1,
                ms_max,
                self.frame_ms_ema,
                self.field_permeability,
                self.expose_offset,
                self.field_dark,
                self.force_ref[0],
                self.force_ref[1],
                self.force_ref[2],
                self.force_ref[3],
                self.force_ref[4],
                self.force_ref[5],
                self.force_ref[6],
                self.force_ref[7],
                self.force_ref[8],
                te_s,
                thr_s,
                tau_s,
                pe_s,
                te_word,
                self.focused,
                self.keys_seen,
            );
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(16),
        ));
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.shutdown.store(true, Ordering::SeqCst);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.size = (size.width.max(1), size.height.max(1));
                self.reconfigure();
                self.ensure_hud_texture();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(m) = self.window.as_ref().and_then(|w| w.current_monitor()) {
                    let ms = m.size();
                    self.size = (ms.width.max(1), ms.height.max(1));
                }
                self.scale_factor = 1.0;
                self.reconfigure();
                self.ensure_hud_texture();
            }
            WindowEvent::ModifiersChanged(m) => {
                self.shift = m.state().shift_key();
            }
            WindowEvent::Focused(f) => {
                self.focused = f;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.logical_key == Key::Named(NamedKey::Escape)
                    || event.physical_key == PhysicalKey::Code(KeyCode::Escape)
                {
                    if event.state == ElementState::Pressed {
                        self.shutdown.store(true, Ordering::SeqCst);
                        event_loop.exit();
                    }
                    return;
                }
                if let PhysicalKey::Code(code) = event.physical_key {
                    match event.state {
                        ElementState::Pressed => {
                            self.keys_seen += 1;
                            if !self.keys.contains(&code) {
                                self.keys.insert(code);
                                self.key_action(code);
                                self.sensors
                                    .record_sample(&format!("event.key.{:?}", code), 1.0);
                            }
                        }
                        ElementState::Released => {
                            self.keys.remove(&code);
                            self.sensors
                                .record_sample(&format!("event.key.{:?}", code), 0.0);
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos = position.to_logical::<f64>(self.scale_factor);
                let (px, py) = (pos.x, pos.y);
                if let Some((lx, ly)) = self.cursor {
                    let dx = px - lx;
                    let dy = py - ly;
                    let (fr, fu, ff) = self.frame();
                    let gaze_px = 2.0 / self.backing.0.max(1) as f64;
                    match self.drag_button {
                        Some(MouseButton::Left) => {
                            if dx != 0.0 {
                                self.q = q_norm(q_mul(q_axis_angle(fu, dx * gaze_px), self.q));
                            }
                            if dy != 0.0 {
                                self.q = q_norm(q_mul(q_axis_angle(fr, dy * gaze_px), self.q));
                            }
                        }
                        Some(MouseButton::Middle) => {
                            if dx != 0.0 {
                                self.q = q_norm(q_mul(q_axis_angle(ff, dx * gaze_px), self.q));
                            }
                        }
                        Some(MouseButton::Right) => {
                            self.p[0] -= fr[0] * dx * self.grid_step - fu[0] * dy * self.grid_step;
                            self.p[1] -= fr[1] * dx * self.grid_step - fu[1] * dy * self.grid_step;
                            self.p[2] -= fr[2] * dx * self.grid_step - fu[2] * dy * self.grid_step;
                            self.consider_resend();
                        }
                        _ => {}
                    }
                }
                self.cursor = Some((px, py));
                self.sensors.record_sample("event.mousemove.x", px);
                self.sensors.record_sample("event.mousemove.y", py);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.drag_button = match state {
                    ElementState::Pressed => Some(button),
                    ElementState::Released => None,
                };
                if let ElementState::Pressed = state {
                    if let Some((px, py)) = self.cursor {
                        self.sensors.record_sample("event.click.x", px);
                        self.sensors.record_sample("event.click.y", py);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y as f64 * 32.0,
                    MouseScrollDelta::PixelDelta(p) => p.y,
                };
                self.sensors.record_sample("event.wheel.deltaY", dy);
                if dy != 0.0 {
                    self.grid_step /= 2f64.powf(dy / 128.0);
                    self.consider_resend();
                }
            }
            WindowEvent::PinchGesture { delta, .. } => {
                if delta != 0.0 {
                    self.grid_step /= 2f64.powf(delta / 512.0);
                    self.consider_resend();
                }
            }
            WindowEvent::Touch(touch) => {
                let pos = touch.location.to_logical::<f64>(self.scale_factor);
                let p = (pos.x, pos.y);
                let id = touch.id;
                match touch.phase {
                    TouchPhase::Started => {
                        if self.touches.is_empty() {
                            self.press = Some((std::time::Instant::now(), id, p));
                        }
                        self.touches.insert(id, p);
                    }
                    TouchPhase::Moved => {
                        if let Some(prev) = self.touches.insert(id, p) {
                            let dx = p.0 - prev.0;
                            let dy = p.1 - prev.1;
                            let (fr, fu, _ff) = self.frame();
                            if self.touches.len() == 1 {
                                self.p[0] -=
                                    fr[0] * dx * self.grid_step - fu[0] * dy * self.grid_step;
                                self.p[1] -=
                                    fr[1] * dx * self.grid_step - fu[1] * dy * self.grid_step;
                                self.p[2] -=
                                    fr[2] * dx * self.grid_step - fu[2] * dy * self.grid_step;
                                self.consider_resend();
                            } else if self.touches.len() == 2 {
                                if let Some(o) = self
                                    .touches
                                    .iter()
                                    .find(|(k, _)| **k != id)
                                    .map(|(_, v)| *v)
                                {
                                    let (cx0, cy0) = ((prev.0 + o.0) / 2.0, (prev.1 + o.1) / 2.0);
                                    let (cx1, cy1) = ((p.0 + o.0) / 2.0, (p.1 + o.1) / 2.0);
                                    let d0 = ((prev.0 - o.0).powi(2) + (prev.1 - o.1).powi(2))
                                        .sqrt()
                                        .max(1.0);
                                    let d1 =
                                        ((p.0 - o.0).powi(2) + (p.1 - o.1).powi(2)).sqrt().max(1.0);
                                    let mdx = cx1 - cx0;
                                    let mdy = cy1 - cy0;
                                    self.p[0] -=
                                        fr[0] * mdx * self.grid_step - fu[0] * mdy * self.grid_step;
                                    self.p[1] -=
                                        fr[1] * mdx * self.grid_step - fu[1] * mdy * self.grid_step;
                                    self.p[2] -=
                                        fr[2] * mdx * self.grid_step - fu[2] * mdy * self.grid_step;
                                    self.grid_step /= d1 / d0;
                                    self.consider_resend();
                                }
                            }
                            if let Some((t0, pid, sp)) = self.press {
                                if pid == id && (p.0 - sp.0).abs() + (p.1 - sp.1).abs() > 12.0 {
                                    self.press = None;
                                } else if t0.elapsed() > std::time::Duration::from_millis(600) {
                                    self.t_frozen = !self.t_frozen;
                                    self.press = None;
                                }
                            }
                        }
                    }
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        self.touches.remove(&id);
                        if let Some((t0, pid, sp)) = self.press {
                            if pid == id {
                                let elapsed = t0.elapsed();
                                let still = (p.0 - sp.0).abs() + (p.1 - sp.1).abs() < 12.0;
                                if still && elapsed < std::time::Duration::from_millis(300) {
                                    let now = std::time::Instant::now();
                                    let double = self
                                        .tap
                                        .map(|(last, lp)| {
                                            now.duration_since(last)
                                                < std::time::Duration::from_millis(300)
                                                && (p.0 - lp.0).abs() + (p.1 - lp.1).abs() < 24.0
                                        })
                                        .unwrap_or(false);
                                    if double {
                                        self.p = [0.0, 0.0, 0.0];
                                        self.v = [0.0, 0.0, 0.0];
                                        self.t0 = self.t_presence;
                                        self.consider_resend();
                                        self.tap = None;
                                    } else {
                                        self.tap = Some((now, p));
                                    }
                                } else if still && elapsed >= std::time::Duration::from_millis(600)
                                {
                                    self.t_frozen = !self.t_frozen;
                                }
                            }
                            self.press = None;
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => self.render(),
            _ => {}
        }
    }
}

fn run_window(
    rx: mpsc::Receiver<Arc<Buffer>>,
    presence_tx: mpsc::Sender<(String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    sensor_tx: mpsc::Sender<Vec<(String, f64, f64)>>,
    req_tx: mpsc::SyncSender<SenseReq>,
    res_rx: mpsc::Receiver<(PackedWindow, f64, u64)>,
    body_names: Arc<Vec<String>>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
    shutdown: Arc<AtomicBool>,
    consent: Arc<AtomicBool>,
    acoustic_tx: mpsc::Sender<PresenceFrame>,
    seismic_tx: mpsc::Sender<PresenceFrame>,
    solar_rx: mpsc::Receiver<SolarCell>,
    enso_rx: mpsc::Receiver<EnsoCell>,
) {
    let mut builder = EventLoopBuilder::<()>::default();
    #[cfg(target_os = "linux")]
    EventLoopBuilderExtX11::with_any_thread(&mut builder, true);
    #[cfg(target_os = "linux")]
    EventLoopBuilderExtWayland::with_any_thread(&mut builder, true);
    let event_loop = match builder.build() {
        Ok(el) => el,
        Err(_) => return,
    };
    event_loop.set_control_flow(ControlFlow::Poll);
    let presence_init: Option<[f64; 4]> = std::env::args().skip(1).find_map(|a| {
        let rest = a.strip_prefix("#x,")?;
        let parts: Vec<&str> = rest.split(',').collect();
        if parts.len() < 4 {
            return None;
        }
        Some([
            parts[0].parse().ok()?,
            parts[1].parse().ok()?,
            parts[2].parse().ok()?,
            parts[3].parse().ok()?,
        ])
    });
    let mut app = NativeApp::new(
        rx,
        req_tx,
        res_rx,
        presence_tx,
        sensor_tx,
        body_names,
        time,
        shutdown,
        consent,
        acoustic_tx,
        seismic_tx,
        solar_rx,
        enso_rx,
    );
    if let Some([x, y, z, t]) = presence_init {
        app.p = [x, y, z];
        app.v = [0.0, 0.0, 0.0];
        app.t0 = t;
        app.t_presence = t;
    }
    let _ = event_loop.run_app(&mut app);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_wgsl_validates_offline() {
        let module = match naga::front::wgsl::parse_str(FIELD_WGSL) {
            Ok(m) => m,
            Err(e) => panic!("wgsl parse: {}", e.emit_to_string(FIELD_WGSL)),
        };
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        if let Err(e) = validator.validate(&module) {
            panic!("wgsl validate: {}", e.emit_to_string(FIELD_WGSL));
        }
    }

    #[test]
    fn window_state_round_trips() {
        let path = std::env::temp_dir().join("omegaflow_window_state_roundtrip.φ");
        let path = path.to_str().unwrap();
        let grid = 1.4e6;
        let p = [1.5e11, -3.7e9, 2.2e8];
        let q = q_norm([0.9, 0.1, -0.2, 0.3]);
        window_state_save(path, grid, p, q);
        let (g, lp, lq) = window_state_load(path);
        assert_eq!(g, grid);
        assert_eq!(lp, p);
        assert_eq!(lq, q);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn window_state_missing_file_is_rest_state() {
        let path = std::env::temp_dir().join("omegaflow_window_state_absent.φ");
        let (g, p, q) = window_state_load(path.to_str().unwrap());
        assert_eq!(g, GRID_INIT);
        assert_eq!(p, [0.0, 0.0, 0.0]);
        assert_eq!(q, [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn window_state_garbage_is_rest_state() {
        let path = std::env::temp_dir().join("omegaflow_window_state_garbage.φ");
        let path = path.to_str().unwrap();
        let text = "grid_step nan\np 1 2\nq 0 0 0 0\nwat\n";
        let _ = std::fs::write(path, text);
        let (g, p, q) = window_state_load(path);
        assert_eq!(g, GRID_INIT);
        assert_eq!(p, [0.0, 0.0, 0.0]);
        assert_eq!(q, [1.0, 0.0, 0.0, 0.0]);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn window_state_partial_lines_keep_rest_fields() {
        let path = std::env::temp_dir().join("omegaflow_window_state_partial.φ");
        let path = path.to_str().unwrap();
        let _ = std::fs::write(path, "grid_step 1.0e8\n");
        let (g, p, q) = window_state_load(path);
        assert_eq!(g, 1.0e8);
        assert_eq!(p, [0.0, 0.0, 0.0]);
        assert_eq!(q, [1.0, 0.0, 0.0, 0.0]);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn te_wgsl_validates_offline() {
        let module = match naga::front::wgsl::parse_str(TE_WGSL) {
            Ok(m) => m,
            Err(e) => panic!("wgsl parse: {}", e.emit_to_string(TE_WGSL)),
        };
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        if let Err(e) = validator.validate(&module) {
            panic!("wgsl validate: {}", e.emit_to_string(TE_WGSL));
        }
    }

    #[test]
    fn te_gpu_crosscheck_against_cpu_reference() {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter =
            match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                compatible_surface: None,
                force_fallback_adapter: false,
            })) {
                Some(a) => a,
                None => {
                    eprintln!("adapter request returned void — crosscheck skipped");
                    return;
                }
            };
        let info = adapter.get_info();
        eprintln!(
            "adapter: {} | {:?} | {:?} | {}",
            info.name, info.backend, info.device_type, info.driver_info
        );
        let (device, queue) = match pollster::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None),
        ) {
            Ok(dq) => dq,
            Err(e) => {
                eprintln!("device request returned: {}", e);
                return;
            }
        };
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(TE_WGSL.into()),
        });
        let te_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::COMPUTE);
                    e.binding = 0;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                {
                    let mut e = storage_entry(false, wgpu::ShaderStages::COMPUTE);
                    e.binding = 2;
                    e
                },
            ],
        });
        let te_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&te_layout],
            push_constant_ranges: &[],
        });
        let te_pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&te_pipe_layout),
            module: &module,
            entry_point: Some("te_compute"),
            compilation_options: Default::default(),
            cache: None,
        });
        let series_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: TE_SERIES_BYTES,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let param_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let out_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &te_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: series_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: param_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: out_buf.as_entire_binding(),
                },
            ],
        });
        let n = 200usize;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.5).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let seed = 42u64;
        let mut data = vec![0f32; 12 * TE_SERIES_STRIDE];
        data[0..n].copy_from_slice(&x);
        data[TE_SERIES_STRIDE..TE_SERIES_STRIDE + n].copy_from_slice(&y);
        let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
        for s in 0..10 {
            let surr = crate::te::phase_randomized_surrogate(&y, &mut rng);
            let off = (2 + s) * TE_SERIES_STRIDE;
            data[off..off + n].copy_from_slice(&surr);
        }
        queue.write_buffer(&series_buf, 0, &le_bytes_f32(&data));
        let max_lag = (n as f64 / Φ) as u32;
        let param = [n as u32, max_lag, 1.0f32.to_bits(), 0];
        let mut pb = [0u8; 16];
        for (i, p) in param.iter().enumerate() {
            pb[i * 4..i * 4 + 4].copy_from_slice(&p.to_le_bytes());
        }
        queue.write_buffer(&param_buf, 0, &pb);
        let start = std::time::Instant::now();
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(&te_pipe);
            pass.set_bind_group(0, &te_bind, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        enc.copy_buffer_to_buffer(&out_buf, 0, &read_buf, 0, 288);
        queue.submit(std::iter::once(enc.finish()));
        let mapped = Arc::new(AtomicBool::new(false));
        let m2 = mapped.clone();
        let slice = read_buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, move |r| {
            m2.store(r.is_ok(), Ordering::SeqCst);
        });
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
            device.poll(wgpu::Maintain::Poll);
        }
        assert!(
            mapped.load(Ordering::SeqCst),
            "te gpu readback returned void"
        );
        let elapsed = start.elapsed();
        let mapped_data = slice.get_mapped_range();
        let mut verdict = [0f32; 72];
        for k in 0..72 {
            let mut b = [0u8; 4];
            b.copy_from_slice(&mapped_data[k * 4..k * 4 + 4]);
            verdict[k] = f32::from_le_bytes(b);
        }
        drop(mapped_data);
        read_buf.unmap();
        let gpu = crate::te::topological_verdict_from_gpu(&verdict);
        let cpu = crate::te::topological_te_phase(&x, &y, 3, 3, seed);
        eprintln!("te crosscheck elapsed {:?}", elapsed);
        eprintln!(
            "gpu: {:?}",
            gpu.as_ref().map(|v| (
                v.tau_x,
                v.tau_y,
                v.te,
                v.threshold,
                v.surrogates_used,
                v.pe_x,
                v.pe_y
            ))
        );
        eprintln!(
            "cpu: {:?}",
            cpu.as_ref().map(|v| (
                v.tau_x,
                v.tau_y,
                v.te,
                v.threshold,
                v.surrogates_used,
                v.pe_x,
                v.pe_y
            ))
        );
        let (gpu_v, cpu_v) = match (gpu, cpu) {
            (Some(g), Some(c)) => (g, c),
            (None, None) => return,
            (g, c) => {
                panic!(
                    "te crosscheck verdict divergence: gpu valid = {}, cpu valid = {}",
                    g.is_some(),
                    c.is_some()
                );
            }
        };
        assert_eq!(gpu_v.tau_x, cpu_v.tau_x, "tau_x diverges");
        assert_eq!(gpu_v.tau_y, cpu_v.tau_y, "tau_y diverges");
        assert!(
            gpu_v.surrogates_used >= 2 && cpu_v.surrogates_used >= 2,
            "surrogates_used below two: gpu {} cpu {}",
            gpu_v.surrogates_used,
            cpu_v.surrogates_used
        );
        let te_rel = ((gpu_v.te - cpu_v.te) / cpu_v.te.abs()).abs();
        assert!(
            te_rel < 0.1,
            "te diverges: gpu {} cpu {} rel {}",
            gpu_v.te,
            cpu_v.te,
            te_rel
        );
        match (gpu_v.pe_x, cpu_v.pe_x) {
            (Some(g), Some(c)) => {
                assert!((g - c).abs() < 1e-3, "pe_x diverges: gpu {} cpu {}", g, c)
            }
            (None, None) => {}
            (g, c) => panic!("pe_x presence diverges: gpu {:?} cpu {:?}", g, c),
        }
        match (gpu_v.pe_y, cpu_v.pe_y) {
            (Some(g), Some(c)) => {
                assert!((g - c).abs() < 1e-3, "pe_y diverges: gpu {} cpu {}", g, c)
            }
            (None, None) => {}
            (g, c) => panic!("pe_y presence diverges: gpu {:?} cpu {:?}", g, c),
        }
        for h_scale in [0.5f32, 2.0f32] {
            let param = [n as u32, max_lag, h_scale.to_bits(), 0];
            let mut pb = [0u8; 16];
            for (i, p) in param.iter().enumerate() {
                pb[i * 4..i * 4 + 4].copy_from_slice(&p.to_le_bytes());
            }
            queue.write_buffer(&param_buf, 0, &pb);
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            {
                let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
                pass.set_pipeline(&te_pipe);
                pass.set_bind_group(0, &te_bind, &[]);
                pass.dispatch_workgroups(1, 1, 1);
            }
            enc.copy_buffer_to_buffer(&out_buf, 0, &read_buf, 0, 288);
            queue.submit(std::iter::once(enc.finish()));
            let mapped = Arc::new(AtomicBool::new(false));
            let m2 = mapped.clone();
            let slice = read_buf.slice(..);
            slice.map_async(wgpu::MapMode::Read, move |r| {
                m2.store(r.is_ok(), Ordering::SeqCst);
            });
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
            while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
                device.poll(wgpu::Maintain::Poll);
            }
            assert!(mapped.load(Ordering::SeqCst), "te scaled readback void");
            let mapped_data = slice.get_mapped_range();
            let mut verdict = [0f32; 72];
            for k in 0..72 {
                let mut b = [0u8; 4];
                b.copy_from_slice(&mapped_data[k * 4..k * 4 + 4]);
                verdict[k] = f32::from_le_bytes(b);
            }
            drop(mapped_data);
            read_buf.unmap();
            let scaled = crate::te::topological_verdict_from_gpu(&verdict)
                .unwrap_or_else(|| panic!("te verdict invalid at bandwidth scale {}", h_scale));
            assert!(
                (scaled.te - gpu_v.te).abs() > 1e-4,
                "te unchanged at bandwidth scale {}: {}",
                h_scale,
                scaled.te
            );
        }
    }

    #[test]
    fn golden_pack_slots_against_wgsl_access() {
        let presence = [1.0e3, 2.0e3, 3.0e3];
        let r: Record = (
            7001.0, 7002.0, 7003.0, 7004.0, 7005.0, 7006.0, 7007.0, 7008.0, 7009.0, 7010.0, 7011.0,
            7012.0, 7013.0, 7014.0, 7015.0, 7016.0, 7017.0, 7018.0, 7019.0, 7020.0, 7021.0, 7022.0,
            7023.0, 7024.0,
        );
        let packed = pack_window(&[r], presence);
        assert_eq!(packed.count, 1);
        let f = &packed.field;
        assert_eq!(f[0], 6001.0);
        assert_eq!(f[1], 5002.0);
        assert_eq!(f[2], 4003.0);
        assert_eq!(f[3], 7004.0);
        assert_eq!(f[4], 7005.0);
        assert_eq!(f[5], 7006.0);
        assert_eq!(f[6], 7010.0);
        assert_eq!(f[7], 7011.0);
        assert_eq!(f[8], 7012.0);
        assert_eq!(f[9], 7013.0);
        assert_eq!(f[10], 7014.0);
        assert_eq!(f[11], 7015.0);
        let m = &packed.meta;
        assert_eq!(m[0], 7008.0);
        assert_eq!(m[1], 7007.0);
        assert_eq!(m[2], 7009.0);
        assert_eq!(m[3], 0.0);
        assert_eq!(m[4], 7016.0);
        assert_eq!(m[5], 7017.0);
        assert_eq!(m[6], 7018.0);
        assert_eq!(m[7], 7019.0);
        assert_eq!(m[8], 7020.0);
        assert_eq!(m[9], 7021.0);
        assert_eq!(m[10], 7022.0);
        assert_eq!(m[11], 7023.0);
        assert_eq!(m[12], 7024.0);
        assert_eq!(m[13], 0.0);
        assert_eq!(m[14], 0.0);
        assert_eq!(m[15], 0.0);
    }

    #[test]
    fn force_ref_medians_routes_forces_and_honors_zero() {
        let mut field = vec![0.0f32; 48];
        field[3] = 4.0;
        field[15] = 4.0;
        field[27] = -2.0;
        field[30] = 2.0;
        field[39] = 0.0;
        field[42] = 8.0;
        let meds = force_ref_medians(&field, &[0.0; 48]);
        assert_eq!(meds[0].unwrap(), 4.0);
        assert_eq!(meds[1], None);
        assert_eq!(meds[2].unwrap(), 2.0);
        for ft in 3..9 {
            assert_eq!(meds[ft], None);
        }
    }

    #[test]
    fn force_ref_medians_holds_reference_on_absence() {
        let mut app = NativeApp {
            force_ref: [7.0; 9],
            ..NativeApp::new(
                mpsc::channel().1,
                mpsc::sync_channel(1).0,
                mpsc::sync_channel(2).1,
                mpsc::channel().0,
                mpsc::channel().0,
                Arc::new(Vec::new()),
                Arc::new(Mutex::new(None)),
                Arc::new(AtomicBool::new(false)),
                Arc::new(AtomicBool::new(false)),
                mpsc::channel().0,
                mpsc::channel().0,
                mpsc::channel().1,
                mpsc::channel().1,
            )
        };
        app.packed_field = vec![0.0; 12];
        app.packed_meta = vec![0.0; 12];
        for _ in 0..64 {
            app.relax_force_refs();
        }
        for ft in 0..9 {
            assert_eq!(app.force_ref[ft], 7.0);
        }
    }

    #[test]
    fn force_ref_medians_skips_length_annotations() {
        let mut field = vec![0.0f32; 24];
        let mut meta = vec![0.0f32; 32];
        field[3] = 2.0f32.powi(20);
        field[6] = 1.0;
        meta[0] = 2.0f32.powi(20);
        field[15] = 2.0f32.powi(45);
        field[18] = 1.0;
        let meds = force_ref_medians(&field, &meta);
        assert_eq!(meds[1].unwrap(), 2.0f32.powi(45));
    }

    #[test]
    fn force_ref_snaps_on_first_sight() {
        let mut app = NativeApp {
            force_ref: [0.0; 9],
            ..NativeApp::new(
                mpsc::channel().1,
                mpsc::sync_channel(1).0,
                mpsc::sync_channel(2).1,
                mpsc::channel().0,
                mpsc::channel().0,
                Arc::new(Vec::new()),
                Arc::new(Mutex::new(None)),
                Arc::new(AtomicBool::new(false)),
                Arc::new(AtomicBool::new(false)),
                mpsc::channel().0,
                mpsc::channel().0,
                mpsc::channel().1,
                mpsc::channel().1,
            )
        };
        app.packed_field = vec![0.0; 12];
        app.packed_field[3] = 8.0;
        app.packed_meta = vec![0.0; 12];
        app.relax_force_refs();
        assert_eq!(app.force_ref[0], 8.0);
    }

    #[test]
    fn aberration_shifts_toward_apex_and_stays_unit() {
        fn aberr(u: [f64; 3], beta: [f64; 3]) -> [f64; 3] {
            let b2 = beta[0] * beta[0] + beta[1] * beta[1] + beta[2] * beta[2];
            let gamma = 1.0 / (1.0 - b2).sqrt();
            let ud = u[0] * beta[0] + u[1] * beta[1] + u[2] * beta[2];
            let inv = 1.0 / (1.0 + ud);
            let k = gamma / (gamma + 1.0) * ud;
            [
                (u[0] / gamma + beta[0] + k * beta[0]) * inv,
                (u[1] / gamma + beta[1] + k * beta[1]) * inv,
                (u[2] / gamma + beta[2] + k * beta[2]) * inv,
            ]
        }
        let beta = [0.5, 0.0, 0.0];
        let ahead = aberr([1.0, 0.0, 0.0], beta);
        assert!((ahead[0] - 1.0).abs() < 1e-9);
        assert!(ahead[1].abs() < 1e-9 && ahead[2].abs() < 1e-9);
        let side = aberr([0.0, 1.0, 0.0], beta);
        assert!(side[0] > 0.0, "transverse star shifts toward the apex");
        let n = (side[0] * side[0] + side[1] * side[1] + side[2] * side[2]).sqrt();
        assert!((n - 1.0).abs() < 1e-9);
        let rest = aberr([0.3, 0.4, 0.916515139], [0.0, 0.0, 0.0]);
        assert!((rest[0] - 0.3).abs() < 1e-9);
        assert!((rest[1] - 0.4).abs() < 1e-9);
    }
}
