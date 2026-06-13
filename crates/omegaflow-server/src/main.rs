use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::LazyLock;

static EVAL_WGSL: &str = include_str!("../static/eval.wgsl");
static DICT_WGSL: &str = include_str!("../static/dict_wgsl.wgsl");
static DICT_GLSL: &str = include_str!("../static/dict_glsl.glsl");
static EVAL_STATE: &str = include_str!("../static/eval_state.glsl");
static EVAL_PERCEPTION: &str = include_str!("../static/eval_perception.glsl");
static MANIFEST: &str = include_str!("../static/manifest.json");
static SW: &str = include_str!("../static/sw.js");

static FULL_WGSL: LazyLock<String> = LazyLock::new(|| DICT_WGSL.replace("EVAL", EVAL_WGSL));
static FULL_GLSL: LazyLock<String> = LazyLock::new(|| {
    let vertex = r#"#version 300 es
precision highp float;
layout(location=0) out vec2 vUv;
const vec2 pos[3] = vec2[3](vec2(-1,-1), vec2(3,-1), vec2(-1,3));
void main() {
    vec2 p = pos[gl_VertexID];
    vUv = vec2(p.x * 0.5 + 0.5, 0.5 - p.y * 0.5);
    gl_Position = vec4(p, 0.0, 1.0);
}"#;
    let fragment = DICT_GLSL.replace("STATE", EVAL_STATE).replace("PERCEPTION", EVAL_PERCEPTION);
    format!("{}\n{}", vertex, fragment)
});

async fn index() -> impl IntoResponse { ([(header::CONTENT_TYPE, "text/html")], include_str!("../static/index.html")) }
async fn client_js() -> impl IntoResponse { ([(header::CONTENT_TYPE, "application/javascript")], include_str!("../static/client.js")) }
async fn webgpu_js() -> impl IntoResponse { ([(header::CONTENT_TYPE, "application/javascript")], include_str!("../static/webgpu.js")) }
async fn webgl_js() -> impl IntoResponse { ([(header::CONTENT_TYPE, "application/javascript")], include_str!("../static/webgl.js")) }
async fn eval_state_wgsl() -> impl IntoResponse { ([(header::CONTENT_TYPE, "text/wgsl")], FULL_WGSL.clone()) }
async fn eval_state_glsl() -> impl IntoResponse { ([(header::CONTENT_TYPE, "text/glsl")], FULL_GLSL.clone()) }
async fn manifest() -> impl IntoResponse { ([(header::CONTENT_TYPE, "application/json")], MANIFEST) }
async fn service_worker() -> impl IntoResponse { ([(header::CONTENT_TYPE, "application/javascript")], SW) }

async fn time() -> impl IntoResponse {
    let t_pool = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64() - 946728000.0;
    ([(header::CONTENT_TYPE, "text/plain")], t_pool.to_string())
}

async fn serve_omega(path: &str) -> Vec<u8> {
    let p = path.to_string();
    tokio::task::spawn_blocking(move || std::fs::read(p).unwrap_or_default()).await.unwrap_or_default()
}

async fn sync_de440s() -> impl IntoResponse {
    let data = serve_omega("pool/de440s.omega").await;
    ([(header::CONTENT_TYPE, "application/octet-stream")], data)
}

async fn sync_wasm() -> impl IntoResponse {
    match std::fs::read("crates/omegaflow-wasm/pkg/omegaflow_wasm_bg.wasm") {
        Ok(data) => ([(header::CONTENT_TYPE, "application/wasm")], data).into_response(),
        Err(_) => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}

async fn sync_wasm_js() -> impl IntoResponse {
    match std::fs::read_to_string("crates/omegaflow-wasm/pkg/omegaflow_wasm.js") {
        Ok(data) => ([(header::CONTENT_TYPE, "application/javascript")], data).into_response(),
        Err(_) => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/eval_state.wgsl", get(eval_state_wgsl))
        .route("/eval_state.glsl", get(eval_state_glsl))
        .route("/sync/de440s", get(sync_de440s))
        .route("/omegaflow_wasm.wasm", get(sync_wasm))
        .route("/omegaflow_wasm.js", get(sync_wasm_js))
        .route("/time", get(time))
        .route("/manifest.json", get(manifest))
        .route("/sw.js", get(service_worker))
        .route("/client.js", get(client_js))
        .route("/webgpu.js", get(webgpu_js))
        .route("/webgl.js", get(webgl_js));
    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(80);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
