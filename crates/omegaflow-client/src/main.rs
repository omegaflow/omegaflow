use axum::extract::Query;
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

#[derive(Deserialize)]
struct StreamReq { 
    jd: f64, cx: f64, cy: f64, cz: f64, scale: f64, 
    min_g: f32, n_max: i32, lat0: i32, lon0: i32 
}

async fn index() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/html")], HTML)
}

async fn eval_state_wgsl() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/wgsl")], EVAL_STATE_SHADER)
}

async fn universe_stream(Query(params): Query<StreamReq>) -> impl IntoResponse {
    let t = (params.jd - 2451545.0) * 86400.0;
    let viewport_center = glam::DVec3::new(params.cx, params.cy, params.cz);
    
    let mut masses = omegaflow_server::masses_at(t, params.cx, params.cy, params.cz, params.scale);
    masses.sort_by(|a, b| b.gm.partial_cmp(&a.gm).unwrap_or(std::cmp::Ordering::Equal));
    masses.retain(|m| { let r2 = (m.pos - viewport_center).length_squared().max(1.0); (m.gm / r2) > params.min_g as f64 });

    let mass_data: Vec<f32> = masses.iter().flat_map(|m| {
        [m.pos.x as f32, m.pos.y as f32, m.pos.z as f32, m.gm as f32]
    }).collect();
    let mass_bytes: Vec<u8> = mass_data.iter().flat_map(|f| f.to_le_bytes()).collect();

    let wmm_bytes = match omegaflow_server::almanac().and_then(|alm| omegaflow_server::wmm_at(t, alm)) {
        Some(data) => {
            let effective_n_max = params.n_max.min(data.n_max);
            let wmm_coeffs = (effective_n_max * (effective_n_max + 3)) / 2;
            let mut out = Vec::new();
            out.extend_from_slice(&[data.earth_pos.x as f32, data.earth_pos.y as f32, data.earth_pos.z as f32].iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>());
            out.extend_from_slice(&data.time_delta.to_le_bytes());
            for i in 0..wmm_coeffs as usize {
                let g = *data.g_mfc.get(i).unwrap_or(&0.0);
                let h = *data.h_mfc.get(i).unwrap_or(&0.0);
                let g_s = *data.g_svc.get(i).unwrap_or(&0.0);
                let h_s = *data.h_svc.get(i).unwrap_or(&0.0);
                out.extend_from_slice(&g.to_le_bytes());
                out.extend_from_slice(&h.to_le_bytes());
                out.extend_from_slice(&g_s.to_le_bytes());
                out.extend_from_slice(&h_s.to_le_bytes());
            }
            out
        },
        None => Vec::new()
    };

    let terrain_bytes = omegaflow_server::raw_hgt_tile(params.lat0, params.lon0);
    let egm_bytes = omegaflow_server::raw_egm96();

    let mut stream = Vec::new();
    stream.extend_from_slice(&(mass_bytes.len() as u32).to_le_bytes());
    stream.extend_from_slice(&(wmm_bytes.len() as u32).to_le_bytes());
    stream.extend_from_slice(&(terrain_bytes.len() as u32).to_le_bytes());
    stream.extend_from_slice(&(egm_bytes.len() as u32).to_le_bytes());
    
    stream.extend(mass_bytes);
    stream.extend(wmm_bytes);
    stream.extend(terrain_bytes);
    stream.extend(egm_bytes);

    ([(header::CONTENT_TYPE, "application/octet-stream")], stream)
}

async fn time() -> impl IntoResponse {
    let jd = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64() / 86400.0 + 2440587.5;
    ([(header::CONTENT_TYPE, "text/plain")], jd.to_string())
}

#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(|| omegaflow_server::init()).await.ok();
    let app = Router::new()
        .route("/", get(index))
        .route("/eval_state.wgsl", get(eval_state_wgsl))
        .route("/stream", get(universe_stream))
        .route("/time", get(time));
    println!("Omegaflow running on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

static EVAL_STATE_SHADER: &str = include_str!("../static/eval_state.wgsl");

static HTML: &str = r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Omegaflow</title>
<link rel="icon" href="data:,">
<style>*{margin:0;padding:0}body{background:#000;overflow:hidden}canvas{display:block;width:100vw;height:100vh;cursor:none}#splash{color:#fff;font-family:monospace;padding:20px;font-size:14px;position:absolute;z-index:10}#error{color:#000;background:red;font-family:monospace;padding:20px;font-size:20px;position:fixed;z-index:100;display:none;width:100%;height:100%}</style>
</head><body><div id="splash">Omegaflow starting...</div><div id="error"></div><canvas id="c" tabindex="0"></canvas><script>
(async()=>{
try {
const splash = document.getElementById('splash');
const errorDiv = document.getElementById('error');
function showError(msg) { if(errorDiv){errorDiv.style.display='block'; errorDiv.innerText=msg;} if(splash) splash.style.display='none'; }

const canvas=document.getElementById('c');
canvas.focus();
const adapter=await navigator.gpu.requestAdapter();
if(!adapter){ showError('No WebGPU Adapter.'); return; }
const device=await adapter.requestDevice();
if(!device){ showError('No WebGPU Device.'); return; }
device.lost.then(info=>{document.body.innerText='GPU Lost: '+info.message;console.error(info)});
const ctx=canvas.getContext('webgpu');
const fmt=navigator.gpu.getPreferredCanvasFormat();
ctx.configure({device,format:fmt,alphaMode:'opaque'});
canvas.width=window.innerWidth;
canvas.height=window.innerHeight;

let RX=canvas.width, RY=canvas.height;
let cx=0, cy=0, cz=0, scale=3e8;
let yaw=0, pitch=0, camRot=0;
let jd=Date.now()/86400000.0+2440587.5;
let timeMultiplier=1.0;
let lastMoveTime = Date.now();
let dwellTime = 0.0;
let observerCapacity = 0.5;
let smoothedCapacity = 0.1;
let massCount = 0;
let deviceAccX=0.0, deviceAccY=0.0, deviceAccZ=0.0;
let deviceMagX=0.0, deviceMagY=0.0, deviceMagZ=0.0;
let ambientLux = 50.0, micVolume = 0.0, cameraLux = 0.0;
let obsLat=47.0, obsLon=11.0, obsAlt=0.0;
let drag=false, rdrag=false, lx=0, ly=0;
let touches = {}, initialPinchDist = 0, initialScale = 0;
let initialAlpha = null, initialBeta = null;
let videoElement = null;
let observerAwake = false;
let prev_cx=0, prev_cy=0, prev_cz=0;
let lastRenderTime = performance.now();
let lastStreamTime = 0;
let egmLoaded = false;
const TARGET_FRAME_MS = 25.0;
const STREAM_INTERVAL = 500;
const smoothFactor = 0.05;

window.addEventListener('resize',()=>{canvas.width=window.innerWidth;canvas.height=window.innerHeight;RX=canvas.width;RY=canvas.height});

function clamp(val, min, max) { return Math.max(min, Math.min(max, val)); }

function syncHere() {
    let t_ut1 = jd - 2451545.0;
    let earth_x = 1.496e11 * Math.cos(2 * Math.PI * t_ut1 / 365.25);
    let earth_y = 1.496e11 * Math.sin(2 * Math.PI * t_ut1 / 365.25);
    let lat_r = obsLat * Math.PI / 180;
    let lon_r = obsLon * Math.PI / 180;
    let R = 6378137.0 + obsAlt;
    let ox = R * Math.cos(lat_r) * Math.cos(lon_r);
    let oy = R * Math.cos(lat_r) * Math.sin(lon_r);
    let oz = R * Math.sin(lat_r);
    let gmst_deg = 280.46061837 + 360.98564736629 * t_ut1;
    let gmst_rad = gmst_deg * Math.PI / 180;
    cx = earth_x + Math.cos(gmst_rad) * ox - Math.sin(gmst_rad) * oy;
    cy = earth_y + Math.sin(gmst_rad) * ox + Math.cos(gmst_rad) * oy;
    cz = oz;
    scale = 1e4; 
}

window.addEventListener('devicemotion',e=>{
    let acc=e.accelerationIncludingGravity;
    if(acc){deviceAccX=acc.x||0; deviceAccY=acc.y||0; deviceAccZ=acc.z||0;}
});
if('AmbientLightSensor' in window){
    try{ let als=new AmbientLightSensor(); als.addEventListener('reading',()=>{ambientLux=als.illuminance;}); als.start(); }catch(e){}
}
if('Magnetometer' in window){
    try{ let mag=new Magnetometer({frequency:60}); mag.addEventListener('reading',()=>{deviceMagX=mag.x||0; deviceMagY=mag.y||0; deviceMagZ=mag.z||0;}); mag.start(); }catch(e){}
}
window.addEventListener('deviceorientation',e=>{
    if(initialAlpha===null){initialAlpha=e.alpha; initialBeta=e.beta;}
    let dAlpha = e.alpha - initialAlpha;
    if(dAlpha > 180) dAlpha -= 360;
    if(dAlpha < -180) dAlpha += 360;
    yaw = dAlpha * 0.02;
    pitch = (e.beta - initialBeta) * 0.02;
});

async function awaken() {
    if(observerAwake) return;
    observerAwake = true;
    try {
        const stream = await navigator.mediaDevices.getUserMedia({audio:true});
        const actx = new AudioContext();
        const source = actx.createMediaStreamSource(stream);
        const analyser = actx.createAnalyser();
        source.connect(analyser);
        const data = new Uint8Array(analyser.frequencyBinCount);
        setInterval(()=>{ analyser.getByteTimeDomainData(data); let sum=0; for(let i=0;i<data.length;i++){let v=(data[i]-128)/128.0;sum+=v*v;} micVolume=Math.sqrt(sum/data.length); },50);
    } catch(e){}
    try {
        const stream = await navigator.mediaDevices.getUserMedia({video:{width:640, height:480, facingMode: 'environment'}});
        videoElement = document.createElement('video');
        videoElement.srcObject = stream;
        videoElement.play();
        const vctx = document.createElement('canvas').getContext('2d', { willReadFrequently: true });
        setInterval(()=>{ try{ vctx.canvas.width=1;vctx.canvas.height=1; vctx.drawImage(videoElement,0,0,1,1); const p=vctx.getImageData(0,0,1,1).data; cameraLux=(p[0]+p[1]+p[2])/765.0; }catch(e){} },100);
    } catch(e){}
    if('geolocation' in navigator){ navigator.geolocation.watchPosition(p=>{ obsLat=p.coords.latitude; obsLon=p.coords.longitude; obsAlt=p.coords.altitude||0.0; }, e=>{}, {enableHighAccuracy:true, maximumAge:0}); }
    if(!document.fullscreenElement){document.documentElement.requestFullscreen().catch(e=>{});}
    await fetchTime();
    prev_cx=cx; prev_cy=cy; prev_cz=cz;
    fetchUniverse();
}

canvas.addEventListener('contextmenu',e=>e.preventDefault());
canvas.addEventListener('mousedown',e=>{
    lastMoveTime=Date.now();canvas.focus(); awaken();
    lx=e.clientX;ly=e.clientY;
    if(e.button===0)drag=true;
    if(e.button===2)rdrag=true;
});
canvas.addEventListener('mousemove',e=>{
    lastMoveTime=Date.now();
    if(drag){cx-=(e.clientX-lx)*scale;cy-=(e.clientY-ly)*scale;}
    if(rdrag){yaw-=(e.clientX-lx)*0.01;pitch+=(e.clientY-ly)*0.01;}
    lx=e.clientX;ly=e.clientY;
});
canvas.addEventListener('mouseup',e=>{ if(e.button===0)drag=false; if(e.button===2)rdrag=false; });
canvas.addEventListener('dblclick',()=>{
    if(!document.fullscreenElement){document.documentElement.requestFullscreen().catch(e=>{});} else{document.exitFullscreen();}
});
canvas.addEventListener('wheel',e=>{
    e.preventDefault();
    if(e.shiftKey){ jd+=e.deltaY*0.0001*timeMultiplier; } else if(e.ctrlKey){ let z=1.01; scale*=e.deltaY>0?z:1.0/z; } else { let z=e.deltaMode===1?1.1:1.05; scale*=e.deltaY>0?z:1.0/z; }
},{passive:false});
window.addEventListener('keydown',e=>{
    lastMoveTime=Date.now(); awaken();
    const step=scale*0.1; const tStep=0.01*timeMultiplier;
    if(e.key==='a'||e.key==='A'){cx+=step;} if(e.key==='d'||e.key==='D'){cx-=step;}
    if(e.key==='ArrowUp'){cy-=step;} if(e.key==='ArrowDown'){cy+=step;}
    if(e.key==='ArrowLeft'){cx+=step;} if(e.key==='ArrowRight'){cx-=step;}
    if(e.key==='q'||e.key==='Q'){jd-=tStep;} if(e.key==='e'||e.key==='E'){jd+=tStep;}
    if(e.key==='z'||e.key==='Z'){timeMultiplier=Math.max(0.1,timeMultiplier/1.5);}
    if(e.key==='x'||e.key==='X'){timeMultiplier=Math.min(1e10,timeMultiplier*1.5);}
    if(e.key==='c'||e.key==='C'){yaw-=0.1;} if(e.key==='v'||e.key==='V'){yaw+=0.1;}
    if(e.key==='b'||e.key==='B'){pitch-=0.1;} if(e.key==='n'||e.key==='N'){pitch+=0.1;}
    if(e.key==='f'||e.key==='F'){ if(!document.fullscreenElement){document.documentElement.requestFullscreen().catch(e=>{});}else{document.exitFullscreen();} }
    if(e.key==='1'){camRot=0;} if(e.key==='2'){camRot=1;} if(e.key==='3'){camRot=2;} if(e.key==='4'){camRot=3;}
    if(e.key==='h'||e.key==='H'){syncHere();}
    if(e.key==='t'||e.key==='T'){jd=Date.now()/86400000.0+2440587.5;}
});

canvas.addEventListener('touchstart',e=>{
    e.preventDefault();canvas.focus(); lastMoveTime=Date.now(); awaken();
    for(let t of e.changedTouches){touches[t.identifier]={x:t.clientX,y:t.clientY};}
    if(e.touches.length===2){ let t1=e.touches[0],t2=e.touches[1]; initialPinchDist=Math.hypot(t1.clientX-t2.clientX,t1.clientY-t2.clientY); initialScale=scale; }
},{passive:false});
canvas.addEventListener('touchmove',e=>{
    e.preventDefault(); lastMoveTime=Date.now();
    if(e.touches.length===1){ let t=e.touches[0]; let prev=touches[t.identifier]; if(prev){cx-=(t.clientX-prev.x)*scale;cy-=(t.clientY-prev.y)*scale;} touches[t.identifier]={x:t.clientX,y:t.clientY}; } 
    else if(e.touches.length===2){ let t1=e.touches[0],t2=e.touches[1]; let prev1=touches[t1.identifier],prev2=touches[t2.identifier]; let currentPinchDist=Math.hypot(t1.clientX-t2.clientX,t1.clientY-t2.clientY); if(initialPinchDist>0){scale=initialScale*(initialPinchDist/currentPinchDist);} if(prev1&&prev2){ let dx1=t1.clientX-prev1.x,dx2=t2.clientX-prev2.x; let avgDx=(dx1+dx2)/2.0; jd+=avgDx*0.00005*timeMultiplier; let dy1=t1.clientY-prev1.y,dy2=t2.clientY-prev2.y; let avgDy=(dy1+dy2)/2.0; timeMultiplier*=Math.pow(1.05,-avgDy); timeMultiplier=Math.max(0.1,Math.min(timeMultiplier,1e10)); } touches[t1.identifier]={x:t1.clientX,y:t1.clientY}; touches[t2.identifier]={x:t2.clientX,y:t2.clientY}; }
},{passive:false});
canvas.addEventListener('touchend',e=>{for(let t of e.changedTouches){delete touches[t.identifier];}});

const shaderResp = await fetch('/eval_state.wgsl');
const shader = await shaderResp.text();

const bgl=device.createBindGroupLayout({entries:[
  {binding:0,visibility:GPUShaderStage.FRAGMENT,buffer:{type:'read-only-storage'}},
  {binding:1,visibility:GPUShaderStage.FRAGMENT,buffer:{type:'uniform'}},
  {binding:2,visibility:GPUShaderStage.FRAGMENT,buffer:{type:'read-only-storage'}},
  {binding:3,visibility:GPUShaderStage.FRAGMENT,texture:{sampleType:'sint'}} ,
  {binding:4,visibility:GPUShaderStage.FRAGMENT,texture:{sampleType:'unfilterable-float'}},
  {binding:5,visibility:GPUShaderStage.FRAGMENT,texture:{sampleType:'float'}},
  {binding:6,visibility:GPUShaderStage.FRAGMENT,sampler:{type:'filtering'}}
]});
const pl=device.createPipelineLayout({bindGroupLayouts:[bgl]});

async function createPipe() {
  const sm = device.createShaderModule({code: shader});
  try {
    return await device.createRenderPipelineAsync({layout:pl,vertex:{module:sm,entryPoint:'vs'},fragment:{module:sm,entryPoint:'fs',targets:[{format:fmt}]},primitive:{topology:'triangle-list'}});
  } catch(e) { console.error("Pipeline failed:", e); return null; }
}

let pipe = await createPipe();
if(!pipe) return;

const massBuf=device.createBuffer({size:65536,usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST});
const vpBuf=device.createBuffer({size:128,usage:GPUBufferUsage.UNIFORM|GPUBufferUsage.COPY_DST});
const wmmBuf=device.createBuffer({size:65536,usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST});
const terrainTex = device.createTexture({size:[1201,1201],format:'r16sint',usage:GPUTextureUsage.TEXTURE_BINDING|GPUTextureUsage.COPY_DST});
const egm96Tex = device.createTexture({size:[1440,721],format:'r32float',usage:GPUTextureUsage.TEXTURE_BINDING|GPUTextureUsage.COPY_DST});
const cameraTex = device.createTexture({size:[640,480],format:'rgba8unorm',usage:GPUTextureUsage.TEXTURE_BINDING|GPUTextureUsage.COPY_DST|GPUTextureUsage.RENDER_ATTACHMENT});
const cameraSampler = device.createSampler({magFilter:'linear',minFilter:'linear'});

let bg=device.createBindGroup({layout:bgl,entries:[
  {binding:0,resource:{buffer:massBuf}},
  {binding:1,resource:{buffer:vpBuf}},
  {binding:2,resource:{buffer:wmmBuf}},
  {binding:3,resource:terrainTex.createView()},
  {binding:4,resource:egm96Tex.createView()},
  {binding:5,resource:cameraTex.createView()},
  {binding:6,resource:cameraSampler}
]});

async function fetchUniverse(){
    let now = Date.now();
    if (now - lastStreamTime < STREAM_INTERVAL) return;
    lastStreamTime = now;
    
    let futureJd = jd + (0.01 * timeMultiplier); 
    let minGInfluence = 1e-8 / Math.max(observerCapacity, 0.01);
    let lat0 = Math.floor(obsLat);
    let lon0 = Math.floor(obsLon);
    let currentMagNMax = Math.floor(1 + observerCapacity * 132);
    
    try {
        const r = await fetch(`/stream?jd=${futureJd}&cx=${cx}&cy=${cy}&cz=${cz}&scale=${scale}&min_g=${minGInfluence}&n_max=${currentMagNMax + 5}&lat0=${lat0}&lon0=${lon0}`);
        const b = await r.arrayBuffer();
        if(b.byteLength < 16) return;
        
        const view = new DataView(b);
        const mass_len = view.getUint32(0, true);
        const wmm_len = view.getUint32(4, true);
        const terrain_len = view.getUint32(8, true);
        const egm_len = view.getUint32(12, true);
        
        let offset = 16;
        if(mass_len > 0) {
            device.queue.writeBuffer(massBuf, 0, new Uint8Array(b, offset, mass_len));
            massCount = mass_len / 16;
            offset += mass_len;
        }
        if(wmm_len > 0) {
            device.queue.writeBuffer(wmmBuf, 0, new Uint8Array(b, offset, wmm_len));
            offset += wmm_len;
        }
        if(terrain_len > 0 && !egmLoaded) { 
            device.queue.writeTexture({texture:terrainTex}, new Uint8Array(b, offset, terrain_len), {bytesPerRow:2402}, {width:1201,height:1201,depthOrArrayLayers:1});
            offset += terrain_len;
        }
        if(egm_len > 0 && !egmLoaded) {
            device.queue.writeTexture({texture:egm96Tex}, new Uint8Array(b, offset, egm_len), {bytesPerRow:5760}, {width:1440,height:721,depthOrArrayLayers:1});
            egmLoaded = true;
        }
    } catch(e) { console.error(e); }
}

async function fetchTime(){
  try{ const r=await fetch('/time'); const t=await r.text(); jd=parseFloat(t); }catch(e){console.error(e)}
}

function render(){
  try{
    if(videoElement && videoElement.readyState >= videoElement.HAVE_CURRENT_DATA){
        device.queue.copyExternalImageToTexture({source:videoElement},{texture:cameraTex},{width:640,height:480});
    }

    if(!observerAwake){
        const enc=device.createCommandEncoder();
        const pass=enc.beginRenderPass({colorAttachments:[{view:ctx.getCurrentTexture().createView(),clearValue:{r:0.0,g:0.0,b:0.05,a:1.0},loadOp:'clear',storeOp:'store'}]});
        pass.setPipeline(pipe);pass.setBindGroup(0,bg);pass.draw(3);pass.end();
        device.queue.submit([enc.finish()]);
        return;
    }

    let nowTime = performance.now();
    let dtMs = nowTime - lastRenderTime;
    lastRenderTime = nowTime;

    let targetCapacity = 1.0 - clamp((dtMs - TARGET_FRAME_MS) / TARGET_FRAME_MS, 0.0, 1.0);
    observerCapacity += (targetCapacity - observerCapacity) * smoothFactor;

    const timeSinceMove = Date.now() - lastMoveTime;
    let motion = Math.sqrt(deviceAccX**2 + deviceAccY**2 + deviceAccZ**2);
    let rawDwell = clamp(timeSinceMove / 2000.0, 0.0, 1.0);
    dwellTime = rawDwell * 100.0;
    let dwellFactor = 0.1 + 0.9 * rawDwell;
    let targetCap = Math.max(0.1, 1.0 - (motion / 20.0)) * dwellFactor;
    smoothedCapacity += (targetCap - smoothedCapacity) * smoothFactor;

    let dtSeconds = dtMs / 1000.0;
    jd += (dtSeconds / 86400.0) * timeMultiplier;

    let realNow = Date.now() / 86400000.0 + 2440587.5;
    let deltaT = Math.abs(jd - realNow);
    let temporalCertainty = Math.exp(-deltaT * 0.5);

    let dx = cx - prev_cx; let dy = cy - prev_cy; let dz = cz - prev_cz;
    let viewVelocity = Math.sqrt(dx*dx + dy*dy + dz*dz) / scale;
    let localityCertainty = Math.exp(-viewVelocity * 5.0);
    prev_cx=cx; prev_cy=cy; prev_cz=cz;

    const vp=new Float32Array([
        cx,cy,cz,scale, 
        RX,RY,massCount,0.0, 
        dwellTime,motion,ambientLux,observerCapacity, 
        deviceAccX,deviceAccY,deviceAccZ,0.0, 
        deviceMagX,deviceMagY,deviceMagZ,0.0,
        yaw,pitch,0.0,0.0,
        micVolume,cameraLux,temporalCertainty,localityCertainty,
        obsLat,obsLon,obsAlt,camRot
    ]);
    device.queue.writeBuffer(vpBuf,0,vp);
    
    const enc=device.createCommandEncoder();
    const pass=enc.beginRenderPass({colorAttachments:[{view:ctx.getCurrentTexture().createView(),clearValue:{r:0,g:0,b:0,a:1},loadOp:'clear',storeOp:'store'}]});
    pass.setPipeline(pipe);pass.setBindGroup(0,bg);pass.draw(3);pass.end();
    device.queue.submit([enc.finish()]);
  }catch(e){console.error(e)}
}

async function loop(){render();requestAnimationFrame(loop);}
setInterval(fetchUniverse, STREAM_INTERVAL);
if(splash) splash.style.display='none';
loop();
} catch(e) { showError(e.message); console.error(e); }
})();
</script></body></html>"#;

