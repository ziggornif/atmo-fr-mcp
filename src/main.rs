use atmo_fr_mcp::{config::Config, tools::air_quality::AirQuality};
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use tokio::net::TcpListener;

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
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("listening on http://{}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, router)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await;
    Ok(())
}
