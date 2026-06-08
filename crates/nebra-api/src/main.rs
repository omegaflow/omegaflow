use axum::{Json, Router, routing::{get, post}, http::header, response::IntoResponse};
use glam::DVec3;
use serde::Deserialize;

#[derive(Deserialize)]
struct FieldReq {
    jd: f64, cx: f64, cy: f64, cz: f64, scale: f64, rx: usize, ry: usize,
}

async fn index() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/html")], HTML)
}

async fn field(Json(req): Json<FieldReq>) -> impl IntoResponse {
    let center = DVec3::new(req.cx, req.cy, req.cz);
    let data = nebra_render::compute::field(req.jd, center, req.scale, req.rx, req.ry);
    let bytes: Vec<u8> = data.iter().flat_map(|f| f.to_le_bytes()).collect();
    ([(header::CONTENT_TYPE, "application/octet-stream")], bytes)
}

#[tokio::main]
async fn main() {
    println!("Loading ephemeris...");
    tokio::task::spawn_blocking(|| nebra_core::init()).await.ok();
    println!("Ready. http://localhost:3000");
    let app = Router::new().route("/", get(index)).route("/field", post(field));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

static HTML: &str = r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Nebra v3</title>
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
const RX=160,RY=120;
let cx=0,cy=0,cz=0,scale=3e10,jd=2460000.5;
let drag=false,lx=0,ly=0;
canvas.addEventListener('mousedown',e=>{drag=true;lx=e.clientX;ly=e.clientY});
canvas.addEventListener('mousemove',e=>{if(!drag)return;cx-=(e.clientX-lx)*scale;cy+=(e.clientY-ly)*scale;lx=e.clientX;ly=e.clientY});
canvas.addEventListener('mouseup',()=>drag=false);
canvas.addEventListener('wheel',e=>{e.preventDefault();scale*=e.deltaY>0?1.1:0.9});
const shader=`@group(0) @binding(0) var t: texture_2d<f32>;
struct V { @builtin(position) p: vec4f, @location(0) u: vec2f }
@vertex fn vs(@builtin(vertex_index) i: u32) -> V {
  var p = array<vec2f, 3>(vec2f(-1,-1), vec2f(3,-1), vec2f(-1,3));
  var o: V; o.p = vec4f(p[i], 0, 1);
  o.u = vec2f(p[i].x*0.5+0.5, 0.5-p[i].y*0.5);
  return o;
}
@fragment fn fs(i: V) -> @location(0) vec4f {
  let tc = vec2u(clamp(i.u, vec2f(0.0), vec2f(1.0)) * vec2f(159.0, 119.0));
  let f = textureLoad(t, tc, 0);
  let o = f.r;
  if o <= 0.0 { discard; }
  let t2 = clamp((log2(o) + 14.0) / 22.0, 0.0, 1.0);
  let c = mix(vec3f(0.0,0.02,0.1), vec3f(0.0,0.3,0.8), clamp(t2*4.0, 0.0, 1.0));
  let c2 = mix(c, vec3f(0.2,0.8,1.0), clamp((t2-0.25)*4.0, 0.0, 1.0));
  let c3 = mix(c2, vec3f(1.0,0.7,0.1), clamp((t2-0.5)*4.0, 0.0, 1.0));
  let c4 = mix(c3, vec3f(1.0,1.0,1.0), clamp((t2-0.75)*4.0, 0.0, 1.0));
  return vec4f(c4, 1.0);
}`;
const sm=device.createShaderModule({code:shader});
const bgl=device.createBindGroupLayout({entries:[{binding:0,visibility:GPUShaderStage.FRAGMENT,texture:{sampleType:'unfilterable-float'}}]});
const pl=device.createPipelineLayout({bindGroupLayouts:[bgl]});
const pipe=device.createRenderPipeline({layout:pl,vertex:{module:sm,entryPoint:'vs'},fragment:{module:sm,entryPoint:'fs',targets:[{format:fmt}]},primitive:{topology:'triangle-list'}});
let tex=device.createTexture({size:[RX,RY],format:'rgba32float',usage:GPUTextureUsage.TEXTURE_BINDING|GPUTextureUsage.COPY_DST});
let bg=device.createBindGroup({layout:bgl,entries:[{binding:0,resource:tex.createView()}]});
let lastData=null;
let fetching=false;
async function fetchField(){
  if(fetching)return;
  fetching=true;
  try{
    const r=await fetch('/field',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({jd,cx,cy,cz,scale,rx:RX,ry:RY})});
    const b=await r.arrayBuffer();
    const d=new Float32Array(b);
    if(d.some(v=>v!==0))lastData=d;
  }catch(e){}
  fetching=false;
}
function render(){
  if(lastData)device.queue.writeTexture({texture:tex},lastData,{bytesPerRow:RX*16},[RX,RY]);
  const enc=device.createCommandEncoder();
  const pass=enc.beginRenderPass({colorAttachments:[{view:ctx.getCurrentTexture().createView(),clearValue:{r:0,g:0,b:0,a:1},loadOp:'clear',storeOp:'store'}]});
  pass.setPipeline(pipe);pass.setBindGroup(0,bg);pass.draw(3);pass.end();
  device.queue.submit([enc.finish()]);
}
setInterval(()=>{jd+=0.001;fetchField();},200);
function loop(){render();requestAnimationFrame(loop);}
loop();
})();
</script></body></html>"#;
