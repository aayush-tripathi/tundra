use axum::middleware::from_fn;
use axum::response::IntoResponse;
use axum::{extract::State, http::Method, routing::post, Json, Router};
use http::HeaderValue;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tundra::run;

#[derive(Deserialize)]
struct ExecReq {
    code: String,
}

#[derive(Serialize)]
struct ExecResp {
    stdout: String,
    stderr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let origins = [
        "https://thetundralanguage.vercel.app",
        "http://localhost:5173",
    ];
    let cors = CorsLayer::new()
        .allow_origin(
            origins
                .into_iter()
                .map(|o| HeaderValue::from_static(o))
                .collect::<Vec<_>>(),
        )
        .allow_methods([Method::POST])
        .allow_headers(Any);
    let app = Router::new()
        .route("/execute", post(exec))
        .layer(cors)
        .with_state(Arc::new(()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!(" executor listening on http://{addr}");

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn exec(State(_): State<Arc<()>>, Json(req): Json<ExecReq>) -> Json<ExecResp> {
    match run(&req.code) {
        Ok(out) => Json(ExecResp {
            stdout: out,
            stderr: String::new(),
        }),
        Err(e) => Json(ExecResp {
            stdout: String::new(),
            stderr: e.to_string(),
        }),
    }
}
