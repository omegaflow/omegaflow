use axum::{Router, routing::get, http::header, response::IntoResponse, extract::Query};
use serde::Deserialize;

#[derive(Deserialize)]
struct MassesReq { jd: f64 }

async fn index() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/html")], HTML)
}

async fn eval_state_wgsl() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/wgsl")], EVAL_STATE_SHADER)
}

async fn masses(Query(params): Query<MassesReq>) -> impl IntoResponse {
    let t = (params.jd - 2451545.0) * 86400.0;
    let masses = omegaflow_server::masses_at(t);
    let data: Vec<f32> = masses.iter().flat_map(|m| {
        [m.pos.x as f32, m.pos.y as f32, m.pos.z as f32, m.gm as f32]
    }).collect();
    let bytes: Vec<u8> = data.iter().flat_map(|f| f.to_le_bytes()).collect();
    ([(header::CONTENT_TYPE, "application/octet-stream")], bytes)
}

async fn wmm(Query(params): Query<MassesReq>) -> impl IntoResponse {
    let t = (params.jd - 2451545.0) * 86400.0;
    let Some(data) = omegaflow_server::wmm_at(t) else {
        return ([(header::CONTENT_TYPE, "application/octet-stream")], Vec::<u8>::new());
    };
    
    let n_max = data.n_max;
    let wmm_coeffs = (n_max * (n_max + 3)) / 2;

    let mut out = Vec::with_capacity(4 + 4 * wmm_coeffs as usize + 9);
    out.push(data.earth_pos.x as f32);
    out.push(data.earth_pos.y as f32);
    out.push(data.earth_pos.z as f32);
    out.push(data.time_delta);

    let pad = |v: &Vec<f32>, len: usize| -> Vec<f32> {
        let mut p = v.clone();
        p.resize(len, 0.0);
        p
    };

    out.extend(pad(&data.g_mfc, wmm_coeffs as usize));
    out.extend(pad(&data.h_mfc, wmm_coeffs as usize));
    out.extend(pad(&data.g_svc, wmm_coeffs as usize));
    out.extend(pad(&data.h_svc, wmm_coeffs as usize));

    let t_ut1 = params.jd - 2451545.0;
    let gmst_deg = 280.46061837 + 360.98564736629 * t_ut1;
    let gmst_rad = gmst_deg.to_radians();
    let cos_g = gmst_rad.cos() as f32;
    let sin_g = gmst_rad.sin() as f32;

    out.push(cos_g);  out.push(sin_g);  out.push(0.0);
    out.push(-sin_g); out.push(cos_g);  out.push(0.0);
    out.push(0.0);    out.push(0.0);    out.push(1.0);

    let bytes: Vec<u8> = out.iter().flat_map(|f| f.to_le_bytes()).collect();
    ([(header::CONTENT_TYPE, "application/octet-stream")], bytes)
}

#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(|| omegaflow_server::init()).await.ok();
    let app = Router::new()
        .route("/", get(index))
        .route("/eval_state.wgsl", get(eval_state_wgsl))
        .route("/masses", get(masses))
        .route("/wmm", get(wmm));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

static EVAL_STATE_SHADER: &str = include_str!("../static/eval_state.wgsl");

static HTML: &str = r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Omegaflow</title>
<style>*{margin:0;padding:0}body{background:#000;overflow:hidden}canvas{display:block;width:100vw;height:100vh}</style>
</head><body><canvas id="c"></canvas><script>
(async()=>{
const canvas=document.getElementById('c');
const adapter=await navigator.gpu.requestAdapter();
const device=await adapter.requestDevice();
const ctx=canvas.getContext('webgpu');
const fmt=navigator.gpu.getPreferredCanvasFormat();
ctx.configure({device,format:fmt,alphaMode:'opaque'});
canvas.width=window.innerWidth;
canvas.height=window.innerHeight;
let RX=canvas.width,RY=canvas.height;
window.addEventListener('resize',()=>{canvas.width=window.innerWidth;canvas.height=window.innerHeight;RX=canvas.width;RY=canvas.height});
let cx=0,cy=0,cz=0,scale=3e10,jd=2460000.5;
let drag=false,lx=0,ly=0;
canvas.addEventListener('mousedown',e=>{drag=true;lx=e.clientX;ly=e.clientY});
canvas.addEventListener('mousemove',e=>{if(!drag)return;cx-=(e.clientX-lx)*scale;cy+=(e.clientY-ly)*scale;lx=e.clientX;ly=e.clientY});
canvas.addEventListener('mouseup',()=>drag=false);
canvas.addEventListener('wheel',e=>{e.preventDefault();scale*=e.deltaY>0?1.1:0.9});

const shaderResp = await fetch('/eval_state.wgsl');
const shader = await shaderResp.text();

const sm=device.createShaderModule({code:shader});
const bgl=device.createBindGroupLayout({entries:[
  {binding:0,visibility:GPUShaderStage.FRAGMENT,buffer:{type:'read-only-storage'}},
  {binding:1,visibility:GPUShaderStage.FRAGMENT,buffer:{type:'uniform'}},
  {binding:2,visibility:GPUShaderStage.FRAGMENT,buffer:{type:'read-only-storage'}}
]});
const pl=device.createPipelineLayout({bindGroupLayouts:[bgl]});

const capacity = device.limits.maxStorageBufferBindingSize;
let model_n_max = 12;
let currentNMax = 12;

function createPipe(nMax) {
  const legendreSize = nMax == 133 ? 9045 : nMax == 12 ? 105 : 45;
  const tier = nMax == 133 ? 2 : nMax == 12 ? 1 : 0;
  return device.createRenderPipeline({layout:pl,vertex:{module:sm,entryPoint:'vs'},fragment:{module:sm,entryPoint:'fs',targets:[{format:fmt}]},primitive:{topology:'triangle-list'},constants:{"HARDWARE_TIER":tier,"N_MAX":nMax,"LEGENDRE_ARRAY_SIZE":legendreSize}});
}

let pipe = createPipe(currentNMax);
const massBuf=device.createBuffer({size:2048,usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST});
const vpBuf=device.createBuffer({size:32,usage:GPUBufferUsage.UNIFORM|GPUBufferUsage.COPY_DST});
const wmmBuf=device.createBuffer({size:65536,usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST});
let bg=device.createBindGroup({layout:bgl,entries:[
  {binding:0,resource:{buffer:massBuf}},
  {binding:1,resource:{buffer:vpBuf}},
  {binding:2,resource:{buffer:wmmBuf}}
]});
let massCount=0;

async function fetchMasses(){
  try{
    const r=await fetch('/masses?jd='+jd);
    const b=await r.arrayBuffer();
    const d=new Float32Array(b);
    massCount=d.length/4;
    device.queue.writeBuffer(massBuf,0,d);
  }catch(e){}
}

async function fetchWmm(){
  try{
    const r=await fetch('/wmm?jd='+jd);
    const b=await r.arrayBuffer();
    if(b.byteLength>0){
      const d=new Float32Array(b);
      const wmm_coeffs = (d.length - 13) / 4;
      model_n_max = Math.floor((Math.sqrt(8 * wmm_coeffs + 1) - 1) / 2);
      const hardware_n_max = capacity >= 73728 ? 133 : capacity >= 1456 ? 12 : 8;
      const targetNMax = Math.min(model_n_max, hardware_n_max);
      
      if(targetNMax !== currentNMax) {
        currentNMax = targetNMax;
        pipe = createPipe(currentNMax);
      }
      device.queue.writeBuffer(wmmBuf,0,d);
    }
  }catch(e){}
}

function render(){
  const vp=new Float32Array([cx,cy,cz,scale,RX,RY,massCount,0]);
  device.queue.writeBuffer(vpBuf,0,vp);
  const enc=device.createCommandEncoder();
  const pass=enc.beginRenderPass({colorAttachments:[{view:ctx.getCurrentTexture().createView(),clearValue:{r:0,g:0,b:0,a:1},loadOp:'clear',storeOp:'store'}]});
  pass.setPipeline(pipe);pass.setBindGroup(0,bg);pass.draw(3);pass.end();
  device.queue.submit([enc.finish()]);
}

async function loop(){render();requestAnimationFrame(loop);}
setInterval(()=>{jd+=0.001;fetchMasses();fetchWmm();},1000);
await fetchMasses();
await fetchWmm();
loop();
})();
</script></body></html>"#;

