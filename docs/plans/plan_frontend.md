static/index.html

=== Replace line 304 ===
old:
        let gpu = null, fieldCtx = null, fieldPipe = null, fieldLayout = null, probePipe = null, vpBuf = null, fieldBuf = null, apertureBuf = null, probeBuf = null, probeRead = null, fieldBind = null, probeBind = null, fieldCap = 0, fieldFrames = 0;
new:
        let gpu = null, fieldCtx = null, fieldPipe = null, fieldLayout = null, probePipe = null, vpBuf = null, fieldBuf = null, extentBuf = null, probeBuf = null, probeRead = null, fieldBind = null, probeBind = null, fieldCap = 0, fieldFrames = 0;

=== Replace line 349 ===
old:
            if (apertureBuf) apertureBuf.destroy();
new:
            if (extentBuf) extentBuf.destroy();

=== Replace line 351 ===
old:
            apertureBuf = gpu.createBuffer({ size: fieldCap * 16, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST });
new:
            extentBuf = gpu.createBuffer({ size: fieldCap * 16, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST });

=== Replace line 354 ===
old:
                { binding: 1, resource: { buffer: apertureBuf } },
new:
                { binding: 1, resource: { buffer: extentBuf } },

=== Replace lines 393-421 (blending loop + xyzval/meta write) ===
old:
        const n = Math.max(rawWindowResponse.length, prevWindowResponse.length);
        if (fieldCap < n) { fieldCap = n * 2; initFieldGPU().catch(() => {}); return; }
        const xyzval = new Float32Array(Math.max(n, 1) * 4);
        const meta = new Float32Array(Math.max(n, 1) * 4);
        const τ = Φ;
        const blend = blendPrev ? Math.pow(2, -dt / τ) : 0;
        for (let i = 0; i < n; i++) {
            const oCur = i < rawWindowResponse.length ? rawWindowResponse[i] : null;
            const oPrev = i < prevWindowResponse.length ? prevWindowResponse[i] : null;
            const o = oCur || oPrev;
            let fx = 0, fy = 0, fz = 0, fv = 0, fa = 0, fdt = 0, ft = 0;
            if (oCur && oPrev) {
                const wOld = blend, wNew = 1.0 - blend;
                fx = oPrev.x * wOld + oCur.x * wNew;
                fy = oPrev.y * wOld + oCur.y * wNew;
                fz = oPrev.z * wOld + oCur.z * wNew;
                fv = oPrev.val * wOld + oCur.val * wNew;
                fa = oPrev.aperture;
                fdt = tPresence - (oCur.t * wNew + oPrev.t * wOld);
                ft = oCur.ttl;
            } else {
                fx = o.x; fy = o.y; fz = o.z;
                fv = o.val; fa = o.aperture;
                fdt = tPresence - o.t; ft = o.ttl;
            }
            xyzval[i * 4] = fx - xPresence;
            xyzval[i * 4 + 1] = fy - yPresence;
            xyzval[i * 4 + 2] = fz - zPresence;
            xyzval[i * 4 + 3] = fv;
            meta[i * 4] = fa;
            meta[i * 4 + 1] = fdt;
            meta[i * 4 + 2] = ft;
            meta[i * 4 + 3] = 0;
        }
new:
        const n = Math.max(rawWindowResponse.length, prevWindowResponse.length);
        if (fieldCap < n) { fieldCap = n * 2; initFieldGPU().catch(() => {}); return; }
        const xyzval = new Float32Array(Math.max(n, 1) * 4);
        const meta = new Float32Array(Math.max(n, 1) * 4);
        const τ = Φ;
        const blend = blendPrev ? Math.pow(2, -dt / τ) : 0;
        const c_js = 299792458.0;
        for (let i = 0; i < n; i++) {
            const oCur = i < rawWindowResponse.length ? rawWindowResponse[i] : null;
            const oPrev = i < prevWindowResponse.length ? prevWindowResponse[i] : null;
            const o = oCur || oPrev;
            let fx, fy, fz, fv, fext, ftau, fdt, fforce;
            if (oCur && oPrev) {
                const wOld = blend, wNew = 1.0 - blend;
                fx = oPrev.x * wOld + oCur.x * wNew;
                fy = oPrev.y * wOld + oCur.y * wNew;
                fz = oPrev.z * wOld + oCur.z * wNew;
                fv = oPrev.val * wOld + oCur.val * wNew;
                fext = oPrev.extent;
                ftau = oPrev.tau;
                fforce = oCur.force_type;
                fdt = tPresence - (oCur.t * wNew + oPrev.t * wOld);
            } else {
                fx = o.x; fy = o.y; fz = o.z;
                fv = o.val;
                fext = o.extent;
                ftau = o.tau;
                fforce = o.force_type;
                fdt = tPresence - o.t;
            }
            const dx = fx - xPresence;
            const dy = fy - yPresence;
            const dz = fz - zPresence;
            const d = Math.sqrt(dx * dx + dy * dy + dz * dz);
            const retarded_dt = Math.max(0.0, fdt - d / c_js);
            const val_eff = fv * Math.exp(-retarded_dt / Math.max(ftau, 1e-9));
            xyzval[i * 4] = dx;
            xyzval[i * 4 + 1] = dy;
            xyzval[i * 4 + 2] = dz;
            xyzval[i * 4 + 3] = val_eff;
            meta[i * 4] = fext;
            meta[i * 4 + 1] = ftau;
            meta[i * 4 + 2] = 0.0;
            meta[i * 4 + 3] = fforce;
        }
new:
        for (let i = 0; i < n; i++) {
            const oCur = i < rawWindowResponse.length ? rawWindowResponse[i] : null;
            const oPrev = i < prevWindowResponse.length ? prevWindowResponse[i] : null;
            const o = oCur || oPrev;
            let fx, fy, fz, fv, fext, ftau, fdt, fforce;
            if (oCur && oPrev) {
                const wOld = blend, wNew = 1.0 - blend;
                fx = oPrev.x * wOld + oCur.x * wNew;
                fy = oPrev.y * wOld + oCur.y * wNew;
                fz = oPrev.z * wOld + oCur.z * wNew;
                fv = oPrev.val * wOld + oCur.val * wNew;
                fext = oPrev.extent;
                ftau = oPrev.tau;
                fforce = oCur.force_type;
                fdt = tPresence - (oCur.t * wNew + oPrev.t * wOld);
            } else {
                fx = o.x; fy = o.y; fz = o.z;
                fv = o.val;
                fext = o.extent;
                ftau = o.tau;
                fforce = o.force_type;
                fdt = tPresence - o.t;
            }
            xyzval[i * 4] = fx - xPresence;
            xyzval[i * 4 + 1] = fy - yPresence;
            xyzval[i * 4 + 2] = fz - zPresence;
            xyzval[i * 4 + 3] = fv;
            meta[i * 4] = fext;
            meta[i * 4 + 1] = ftau;
            meta[i * 4 + 2] = fdt;
            meta[i * 4 + 3] = fforce;
        }

=== Replace line 428 ===
old:
            gpu.queue.writeBuffer(apertureBuf, 0, meta);
new:
            gpu.queue.writeBuffer(extentBuf, 0, meta);

=== Replace line 429 ===
old:
            gpu.queue.writeBuffer(vpBuf, 0, new Float32Array([evalW, evalH, n, gridStep * Math.pow(2, evalShift), fr[0], fr[1], fr[2], 0, fu[0], fu[1], fu[2], 0, exposure, 0, 0, 0]));
new:
            const softening = Math.min(Math.max(windowMedianAperture(), 0.0), 1.0);
            gpu.queue.writeBuffer(vpBuf, 0, new Float32Array([evalW, evalH, n, gridStep * Math.pow(2, evalShift), fr[0], fr[1], fr[2], 0, fu[0], fu[1], fu[2], 0, exposure, softening, 0, 0]));

=== Replace line 442-449 ===
old:
        function windowMedianAperture() {
            if (maRef !== rawWindowResponse) {
                maRef = rawWindowResponse;
                const aps = [];
                for (const o of rawWindowResponse) { if (o.aperture > 0) aps.push(o.aperture); }
                maVal = aps.length === 0 ? 1 : aps.sort((a, b) => a - b)[aps.length >> 1];
            }
            return maVal;
        }
new:
        function windowMedianAperture() {
            if (maRef !== rawWindowResponse) {
                maRef = rawWindowResponse;
                const exts = [];
                for (const o of rawWindowResponse) { if (o.extent > 0) exts.push(o.extent); }
                maVal = exts.length === 0 ? 1 : exts.sort((a, b) => a - b)[exts.length >> 1];
            }
            return maVal;
        }

static/constants.js

=== Replace protocol reader (the function that reads WebSocket binary response) ===
old:
const oscCount = dvRes.getUint32(o, true); o += 4;
const result = [];
for (let i = 0; i < oscCount; i++) {
    if (o + 56 > bytes.length) break;
    const x = dvRes.getFloat64(o, true); o += 8;
    const y = dvRes.getFloat64(o, true); o += 8;
    const z = dvRes.getFloat64(o, true); o += 8;
    const val = dvRes.getFloat64(o, true); o += 8;
    const aperture = dvRes.getFloat64(o, true); o += 8;
    const t = dvRes.getFloat64(o, true); o += 8;
    const ttl = dvRes.getFloat64(o, true); o += 8;
    result.push({ x, y, z, val, aperture, t, ttl });
}
return result;
new:
const oscCount = dvRes.getUint32(o, true); o += 4;
const result = [];
for (let i = 0; i < oscCount; i++) {
    if (o + 72 > bytes.length) break;
    const x = dvRes.getFloat64(o, true); o += 8;
    const y = dvRes.getFloat64(o, true); o += 8;
    const z = dvRes.getFloat64(o, true); o += 8;
    const val = dvRes.getFloat64(o, true); o += 8;
    const extent = dvRes.getFloat64(o, true); o += 8;
    const t = dvRes.getFloat64(o, true); o += 8;
    const ttl = dvRes.getFloat64(o, true); o += 8;
    const tau = dvRes.getFloat64(o, true); o += 8;
    const force_type = dvRes.getFloat64(o, true); o += 8;
    result.push({ x, y, z, val, extent, t, ttl, tau, force_type });
}
return result;

phi/sources.φ

=== Transform every source entry ===
For each source block (source ... until next source or EOF):
  1. Delete any line starting with "res ".
  2. Delete any line starting with "ap ".
  3. After the "ttl N" line, insert "force X" where X is determined by the source name:

source name contains                     | force
-----------------------------------------|-------
earthquake|quake|emsc|usgs|seismic      | seismic-surface
metar|open_meteo|rain_radar|ndbc_buoy   | acoustic
pegel|streamflow|tsunami|tide           | gravity
waqi|aqi                                 | diffusion
_all other sources_                     | em

bash command to run after edits:
  sed -i '/^res /d; /^ap /d' phi/sources.φ

=== WGSL Viewport struct — softening via expose.y ===
The Viewport uniform buffer is 16 f32 (4 × vec4f). Index 13 (expose.y) carries softening.
In the WGSL shader string, the line:
    let softening = vp.expose.y;
replaces the original scale-dependent softening. No struct field added.
The JS buffer write at index 13 already contains `softening`.
