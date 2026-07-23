use axum::response::Html;
use axum::routing::get;
use axum::Router;
use std::env;
use tokio::net::TcpListener;

const DEFAULT_BIND: &str = "127.0.0.1:7111";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind = env::var("VAPOR_HOMEPAGE_BIND").unwrap_or_else(|_| DEFAULT_BIND.to_string());
    let listener = TcpListener::bind(&bind).await?;

    let app = Router::new()
        .route("/", get(home_page))
        .route("/index.html", get(home_page))
        .route("/healthz", get(healthz))
        .route("/license", get(license_page))
        .route("/impressum", get(impressum_page));

    eprintln!("vapor-homepage-server listening on {bind}");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn healthz() -> &'static str {
    "ok\n"
}

async fn home_page() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Vapor</title>
</head>
<body>
  <main>
    <h1>Vapor</h1>
    <p>Vapor is the open tooling and distribution layer for Loo-Cast.</p>
    <nav>
      <a href="/docs/">Docs</a>
      <a href="/license">License</a>
      <a href="/impressum">Impressum</a>
    </nav>
  </main>
</body>
</html>
"#,
    )
}

async fn license_page() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Vapor License</title></head>
<body>
  <main>
    <h1>License</h1>
    <p>License and product terms are not finalized in this scaffold.</p>
  </main>
</body>
</html>
"#,
    )
}

async fn impressum_page() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Impressum</title></head>
<body>
  <main>
    <h1>Impressum</h1>
    <p>Legal contact details are not finalized in this scaffold.</p>
  </main>
</body>
</html>
"#,
    )
}
