use axum::{Router, routing::get, http::header, response::IntoResponse, extract::Query};
use serde::Deserialize;

#[derive(Deserialize)]
struct MassesReq { jd: f64 }

async fn index() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/html")], HTML)
}

async fn masses(Query(params): Query<MassesReq>) -> impl IntoResponse {
    let t = (params.jd - 2451545.0) * 86400.0;
    let masses = nebra_core::masses_at(t);
    let data: Vec<f32> = masses.iter().flat_map(|m| {
        [m.pos.x as f32, m.pos.y as f32, m.pos.z as f32, m.gm as f32]
    }).collect();
    let bytes: Vec<u8> = data.iter().flat_map(|f| f.to_le_bytes()).collect();
    ([(header::CONTENT_TYPE, "application/octet-stream")], bytes)
}

async fn wmm(Query(params): Query<MassesReq>) -> impl IntoResponse {
    let t = (params.jd - 2451545.0) * 86400.0;
    let Some(data) = nebra_core::wmm_at(t) else {
        return ([(header::CONTENT_TYPE, "application/octet-stream")], Vec::<u8>::new());
    };
    let mut out = Vec::with_capacity(364);
    out.push(data.earth_pos.x as f32);
    out.push(data.earth_pos.y as f32);
    out.push(data.earth_pos.z as f32);
    out.push(data.time_delta);
    out.extend_from_slice(&data.g_mfc);
    out.extend_from_slice(&data.h_mfc);
    out.extend_from_slice(&data.g_svc);
    out.extend_from_slice(&data.h_svc);
    let bytes: Vec<u8> = out.iter().flat_map(|f| f.to_le_bytes()).collect();
    ([(header::CONTENT_TYPE, "application/octet-stream")], bytes)
}

#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(|| nebra_core::init()).await.ok();
    let app = Router::new()
        .route("/", get(index))
        .route("/masses", get(masses))
        .route("/wmm", get(wmm));
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
let RX=canvas.width,RY=canvas.height;
window.addEventListener('resize',()=>{canvas.width=window.innerWidth;canvas.height=window.innerHeight;RX=canvas.width;RY=canvas.height});
let cx=0,cy=0,cz=0,scale=3e10,jd=2460000.5;
let drag=false,lx=0,ly=0;
canvas.addEventListener('mousedown',e=>{drag=true;lx=e.clientX;ly=e.clientY});
canvas.addEventListener('mousemove',e=>{if(!drag)return;cx-=(e.clientX-lx)*scale;cy+=(e.clientY-ly)*scale;lx=e.clientX;ly=e.clientY});
canvas.addEventListener('mouseup',()=>drag=false);
canvas.addEventListener('wheel',e=>{e.preventDefault();scale*=e.deltaY>0?1.1:0.9});
const shader=`
struct VP{center_scale:vec4f,res_count:vec4f};
@group(0)@binding(0) var<storage,read> masses:array<vec4f>;
@group(0)@binding(1) var<uniform> vp:VP;
@group(0)@binding(2) var<storage,read> wmm:array<f32>;
struct V{@builtin(position) p:vec4f,@location(0) u:vec2f}
@vertex fn vs(@builtin(vertex_index) i:u32)->V{
  var p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3));
  var o:V;o.p=vec4f(p[i],0,1);o.u=vec2f(p[i].x*0.5+0.5,0.5-p[i].y*0.5);return o;
}
fn wmm_idx(n:i32,m:i32)->i32{return n*(n+1)/2+m-1;}
fn eval_wmm(pixel_pos:vec3f)->f32{
  if(wmm.length()<364u){return 0.0;}
  let earth=vec3f(wmm[0],wmm[1],wmm[2]);
  let td=wmm[3];
  let rel=pixel_pos-earth;
  let r=length(rel);
  let R_E=6371000.0;
  let alt=r-R_E;
  if(alt<-1000.0||alt>850000.0){return 0.0;}
  let lat=asin(rel.z/r);
  let lon=atan2(rel.y,rel.x);
  let sin_colat=cos(lat);
  let A2=6371200.0;
  let k_ratio=A2/r;
  var psn=vec4f(0.0,0.0,0.0,0.0);
  var xp=0.0;var yp=0.0;var zp=0.0;
  var Pmm=1.0;
  var Pm1m=0.0;
  var Pm2m=0.0;
  for(var m2=0;m2<=12;m2=m2+1){
    let mm=f32(m2);
    if(m2>0){
      Pmm=Pmm*sqrt(1.0-sin_colat*sin_colat)*sqrt(f32(2*m2-1)/f32(2*m2));
    }
    if(m2<=12){
      Pm2m=Pm1m;
      Pm1m=Pmm;
      var sn=1.0;
      if(m2==0){sn=1.0;}else{sn=0.0;}
      for(var n2=m2+1;n2<=12;n2=n2+1){
        let nn=f32(n2);
        var Pnm=0.0;
        if(n2==m2+1){
          Pnm=sin_colat*sqrt(f32(2*m2+1)+1.0)*Pm1m;
        }else{
          let fnm=f32(n2-m2);
          Pnm=(sin_colat*f32(2*n2-1)*Pm1m-f32(n2+m2-1)*Pm2m)/fnm;
        }
        Pm2m=Pm1m;
        Pm1m=Pnm;
      }
    }
    Pm1m=Pmm;
    Pm2m=0.0;
  }
  var Bx=0.0;var By=0.0;var Bz=0.0;
  Pmm=1.0;Pm1m=0.0;
  for(var m2=0;m2<=12;m2=m2+1){
    let mm=f32(m2);
    if(m2>0){Pmm=Pmm*sqrt(1.0-sin_colat*sin_colat)*sqrt(f32(2*m2-1)/f32(2*m2));}
    Pm2m=0.0;Pm1m=Pmm;
    for(var n2=max(m2,1);n2<=12;n2=n2+1){
      let nn=f32(n2);
      var Pnm=Pm1m;
      if(n2>m2+1){
        Pnm=(sin_colat*f32(2*n2-1)*Pm1m-f32(n2+m2-1)*Pm2m)/f32(n2-m2);
      }else if(n2==m2+1&&m2>0){
        Pnm=sin_colat*sqrt(f32(2*m2+1)+1.0)*Pmm;
      }
      let ix=wmm_idx(n2,m2);
      if(ix<0||ix>=90){Pm2m=Pm1m;Pm1m=Pnm;continue;}
      let gt=wmm[f32(4+ix)]+td*wmm[f32(184+ix)];
      let ht=wmm[f32(94+ix)]+td*wmm[f32(274+ix)];
      let cosm=cos(f32(m2)*lon);
      let sinm=sin(f32(m2)*lon);
      let kr=1.0;
      var k=1.0;
      for(var ki=0;ki<n2+2;ki=ki+1){k=k*k_ratio;}
      let schm=sn_factor(n2,m2);
      let S=Pnm*schm;
      let gx=gt*cosm+ht*sinm;
      let gy=gt*sinm-ht*cosm;
      Bz=Bz+gx*S*k;
      Bx=Bx+gy*f32(m2)*S*k;
      Pm2m=Pm1m;Pm1m=Pnm;
    }
  }
  return sqrt(Bx*Bx+By*By+Bz*Bz);
}
fn sn_factor(n:i32,m:i32)->f32{
  if(m==0){return 1.0;}
  return 1.0;
}
@fragment fn fs(i:V)->@location(0) vec4f{
  let count=u32(vp.res_count.z);
  let scale=vp.center_scale.w;
  let w=vp.res_count.x;
  let h=vp.res_count.y;
  let pixel_pos=vec3f(
    vp.center_scale.x+(i.u.x-0.5)*w*scale,
    vp.center_scale.y-(i.u.y-0.5)*h*scale,
    vp.center_scale.z);
  var omega=0.0f;
  for(var j=0u;j<count;j=j+1u){
    let m=masses[j];
    let delta=m.xyz-pixel_pos;
    let dist=length(delta);
    if(dist>1.0){omega=omega+m.w/(dist*dist);}
  }
  let em=eval_wmm(pixel_pos);
  if(omega<=0.0&&em<=0.0){discard;}
  var r=0.0;var g=0.0;var b=0.0;var a=1.0;
  if(omega>0.0){
    let t2=clamp((log2(omega)+14.0)/22.0,0.0,1.0);
    let c=mix(vec3f(0.0,0.02,0.1),vec3f(0.0,0.3,0.8),clamp(t2*4.0,0.0,1.0));
    let c2=mix(c,vec3f(0.2,0.8,1.0),clamp((t2-0.25)*4.0,0.0,1.0));
    let c3=mix(c2,vec3f(1.0,0.7,0.1),clamp((t2-0.5)*4.0,0.0,1.0));
    let c4=mix(c3,vec3f(1.0,1.0,1.0),clamp((t2-0.75)*4.0,0.0,1.0));
    r=c4.x;g=c4.y;b=c4.z;
  }
  if(em>1000.0){
    let et=clamp((em-20000.0)/50000.0,0.0,1.0);
    g=g+et*0.6;
    r=r+et*0.1;
    b=b+et*0.2;
  }
  return vec4f(r,g,b,a);
}`;
const sm=device.createShaderModule({code:shader});
const bgl=device.createBindGroupLayout({entries:[
  {binding:0,visibility:GPUShaderStage.FRAGMENT,buffer:{type:'read-only-storage'}},
  {binding:1,visibility:GPUShaderStage.FRAGMENT,buffer:{type:'uniform'}},
  {binding:2,visibility:GPUShaderStage.FRAGMENT,buffer:{type:'read-only-storage'}}
]});
const pl=device.createPipelineLayout({bindGroupLayouts:[bgl]});
const pipe=device.createRenderPipeline({layout:pl,vertex:{module:sm,entryPoint:'vs'},fragment:{module:sm,entryPoint:'fs',targets:[{format:fmt}]},primitive:{topology:'triangle-list'}});
const massBuf=device.createBuffer({size:2048,usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST});
const vpBuf=device.createBuffer({size:32,usage:GPUBufferUsage.UNIFORM|GPUBufferUsage.COPY_DST});
const wmmBuf=device.createBuffer({size:2048,usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST});
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
