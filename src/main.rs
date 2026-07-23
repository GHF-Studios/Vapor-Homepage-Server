use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

const DEFAULT_BIND: &str = "127.0.0.1:7111";

fn main() -> io::Result<()> {
    let bind = env::var("VAPOR_HOMEPAGE_BIND").unwrap_or_else(|_| DEFAULT_BIND.to_string());
    let listener = TcpListener::bind(&bind)?;
    eprintln!("vapor-homepage-server listening on {bind}");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(error) = handle_connection(&mut stream) {
                    eprintln!("request failed: {error}");
                }
            }
            Err(error) => eprintln!("connection failed: {error}"),
        }
    }

    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    let request = read_request_line(stream)?;
    let Some((method, path)) = request else {
        return Ok(());
    };

    match (method.as_str(), path.as_str()) {
        ("GET", "/healthz") => respond_text(stream, "200 OK", "ok\n"),
        ("GET", "/license") => respond_html(stream, "200 OK", license_page()),
        ("GET", "/impressum") => respond_html(stream, "200 OK", impressum_page()),
        ("GET", "/") | ("GET", "/index.html") => respond_html(stream, "200 OK", home_page()),
        _ => respond_text(stream, "404 Not Found", "not found\n"),
    }
}

fn read_request_line(stream: &mut TcpStream) -> io::Result<Option<(String, String)>> {
    let mut buffer = [0_u8; 4096];
    let read = stream.read(&mut buffer)?;
    if read == 0 {
        return Ok(None);
    }

    let text = String::from_utf8_lossy(&buffer[..read]);
    let Some(line) = text.lines().next() else {
        return Ok(None);
    };

    let mut parts = line.split_whitespace();
    let Some(method) = parts.next() else {
        return Ok(None);
    };
    let Some(path) = parts.next() else {
        return Ok(None);
    };

    Ok(Some((method.to_string(), path.to_string())))
}

fn respond_text(stream: &mut TcpStream, status: &str, body: &str) -> io::Result<()> {
    respond(stream, status, "text/plain; charset=utf-8", body)
}

fn respond_html(stream: &mut TcpStream, status: &str, body: &str) -> io::Result<()> {
    respond(stream, status, "text/html; charset=utf-8", body)
}

fn respond(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) -> io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

fn home_page() -> &'static str {
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
"#
}

fn license_page() -> &'static str {
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
"#
}

fn impressum_page() -> &'static str {
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
"#
}
