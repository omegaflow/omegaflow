use axum::{Json, Router, routing::get};
use nebra_core::verify_integrity;

async fn verify_handler() -> Json<Vec<String>> {
    let result = verify_integrity();
    match result.value {
        Some(stack) => Json(stack.iter().map(|s| s.to_string()).collect::<Vec<String>>()),
        None => Json(vec!["ERROR".to_string()]),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/verify", get(verify_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
