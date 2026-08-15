struct VP { surface: vec4f, right: vec4f, up: vec4f, forward: vec4f, expose_lo: vec4f, expose_hi: vec4f, expose_ex: vec4f, presence: vec4f };
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
    return raw * exp(-retarded / max(ttl, 1e-9));
}
@group(0) @binding(0) var<storage, read> field: array<vec4f>;
@group(0) @binding(1) var<storage, read> props: array<vec4f>;
@group(0) @binding(2) var<uniform> vp: VP;
@group(0) @binding(3) var<storage, read_write> probe_out: array<f32>;

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

    if (kernel_id == 0u || kernel_id == 6u) {
        return 1.0 / (d2 + e2);
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
    } else {
        return exp(-d2 / (2.0 * max(e2, s2))) / max(d2 + s2, s2);
    }
}

fn osc_field(j: u32, rel: vec3f, dt: f32) -> vec2f {
    let m = field[j * 3u];
    let tm = field[j * 3u + 1u];
    let fm = field[j * 3u + 2u];
    let mt = props[j * 3u];
    let mp = props[j * 3u + 1u];
    let mg = props[j * 3u + 2u];
    let v_obs = vec3f(vp.right.w, vp.up.w, vp.forward.w) * C_VACUUM;
    let v_rel = fm.yzw - v_obs;
    let propagated = m.xyz + v_rel * dt;
    let delta = propagated - rel;
    let d2 = dot(delta, delta);
    let d_mag = sqrt(d2);
    let ft = u32(tm.z);
    let kid = u32(mt.z);
    let val_eff = fold_eff(d_mag, m.w, tm.x, tm.y, ft, fm.x);
    var sk = field_spatial(d2, d_mag, mt.x, kid, vp.surface.w, f32(tm.w));
    if ((kid == 0u || kid == 6u) && ft == 1u) {
        let dhat = delta / max(d_mag, 1e-9);
        let cos_t = clamp(dot(dhat, mp.xyz), -1.0, 1.0);
        let p2 = (3.0 * cos_t * cos_t - 1.0) * 0.5;
        let p4 = (35.0 * cos_t * cos_t * cos_t * cos_t - 30.0 * cos_t * cos_t + 3.0) * 0.125;
        let rd = mg.y / max(d_mag, 1.0);
        sk *= 1.0 - mp.w * rd * rd * p2 - mg.x * rd * rd * rd * rd * p4;
    }
    return vec2f(val_eff * sk, f32(ft));
}

@compute @workgroup_size(1)
fn presence_probe() {
    let count = u32(vp.surface.z);
    var omegas = array<f32, 9>();
    for (var i = 0u; i < 9u; i = i + 1u) { omegas[i] = 0.0; }
    let dt = vp.presence.w - vp.expose_ex.y;
    for (var j = 0u; j < count; j = j + 1u) {
        let c = osc_field(j, vec3f(0.0), dt);
        let ft = u32(c.y);
        if (ft < 9u) { omegas[ft] += c.x; }
    }
    for (var i = 0u; i < 9u; i = i + 1u) { probe_out[i] = omegas[i]; }
}

@fragment fn fs(in: VOut) -> @location(0) vec4f {
    let count = u32(vp.surface.z);
    if (count == 0u) { discard; }
    let w = vp.surface.x;
    let h = vp.surface.y;
    let scale = vp.surface.w;
    let pixel_rel = (in.uv.x - 0.5) * w * scale * vp.right.xyz
        + (0.5 - in.uv.y) * h * scale * vp.up.xyz;
    let dt = vp.presence.w - vp.expose_ex.y;

    var omegas = array<f32, 9>();
    for (var k = 0u; k < 9u; k = k + 1u) { omegas[k] = 0.0; }
    for (var j = 0u; j < count; j = j + 1u) {
        let osc = osc_field(j, pixel_rel, dt);
        let ft = u32(osc.y);
        if (ft < 9u) { omegas[ft] += osc.x; }
    }

    var omega_total = 0.0;
    for (var k = 0u; k < 9u; k = k + 1u) { omega_total += abs(omegas[k]); }
    if (omega_total < 1e-30) { discard; }

    let t2 = clamp((log2(omega_total) + 14.0 + vp.expose_ex.x) / 22.0, 0.0, 1.0);

    let c = mix(vec3f(0.0, 0.02, 0.1), vec3f(0.0, 0.3, 0.8), clamp(t2 * 4.0, 0.0, 1.0));
    let c2 = mix(c, vec3f(0.2, 0.8, 1.0), clamp((t2 - 0.25) * 4.0, 0.0, 1.0));
    let c3 = mix(c2, vec3f(1.0, 0.7, 0.1), clamp((t2 - 0.5) * 4.0, 0.0, 1.0));
    let c4 = mix(c3, vec3f(1.0, 1.0, 1.0), clamp((t2 - 0.75) * 4.0, 0.0, 1.0));

    return vec4f(c4, 1.0);
}
