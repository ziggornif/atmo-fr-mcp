use atmo_fr_mcp::{config::config::Config, tools::air_quality::AirQuality};
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};

const BIND_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();
    let service = StreamableHttpService::new(
        move || Ok(AirQuality::new(config.clone())),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let router = axum::Router::new().nest_service("/mcp", service);
    let tcp_listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await?;
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await;
    Ok(())
}
