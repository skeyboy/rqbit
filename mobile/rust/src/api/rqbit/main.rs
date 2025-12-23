
pub use  std::
    sync::Arc
;
pub use  std::sync::Mutex;
pub use  flutter_rust_bridge::frb;

pub use  axum::{Router, extract::State};
pub use  librqbit::{
    ApiError,
     session_stats::snapshot::SessionStatsSnapshot,
 };
 pub use  tower_http::trace::TraceLayer;
pub use  tracing::{  info,  };
  
pub use  crate::api::rqbit::config::AppState;


async fn stats(State(state): State<Arc<AppState>>
) -> Result<SessionStatsSnapshot, ApiError> {
    state.api()?.api_session_stats();
    Ok(state.api()?.api_session_stats())
}


fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}


#[frb]
#[tokio::main]
pub async fn start_service(
    dest_dir: &str,
    addr: Option<String>,
    port: Option<i32>,
) -> Result<(), std::io::Error> {
    let app_state = Arc::new(AppState {
        shared: Arc::new(Mutex::new(None)),
    });
    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        addr.unwrap_or(String::from("0.0.0.0")),
        port.unwrap_or(8888)
    ))
        .await?;
    info!("rust_demo listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await
}