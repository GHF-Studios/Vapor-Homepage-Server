use std::env;
use tokio::net::TcpListener;

const DEFAULT_BIND: &str = "127.0.0.1:7111";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind = env::var("VAPOR_HOMEPAGE_BIND").unwrap_or_else(|_| DEFAULT_BIND.to_string());
    let listener = TcpListener::bind(&bind).await?;
    eprintln!("vapor-homepage-server listening on {bind}");
    axum::serve(listener, vapor_homepage_server::app_router()).await?;
    Ok(())
}
