use axum::response::Html;
use axum::routing::get;
use axum::Router;
pub fn app_router() -> Router {
    Router::new()
        .route("/", get(home_page))
        .route("/index.html", get(home_page))
        .route("/healthz", get(healthz))
        .route("/downloads", get(downloads_page))
        .route("/downloads/", get(downloads_page))
        .route("/license", get(license_page))
        .route("/impressum", get(impressum_page))
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
    <p>Vapor is an independent platform for discovering, installing, and managing game content and tools.</p>
    <nav>
      <a href="/downloads">Download Vapor</a>
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

async fn downloads_page() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="Download Vapor, an independent platform for discovering and managing game content and tools.">
  <meta name="theme-color" content="#111827">
  <title>Download Vapor · Vapor</title>
  <style>
    :root { color-scheme: dark; font-family: Inter, ui-sans-serif, system-ui, sans-serif; background: #0b1020; color: #edf2ff; }
    * { box-sizing: border-box; }
    body { margin: 0; min-height: 100vh; background: radial-gradient(ellipse at 50% -20%, #24365f 0, #0b1020 58%); }
    main { width: min(760px, calc(100% - 32px)); margin: 0 auto; padding: 72px 0; }
    nav { display: flex; gap: 20px; margin-bottom: 56px; }
    a { color: #a9c4ff; }
    .eyebrow { color: #93a4c7; font-size: .8rem; letter-spacing: .14em; text-transform: uppercase; }
    h1 { margin: 12px 0; font-size: clamp(2.6rem, 8vw, 4.5rem); letter-spacing: -.055em; }
    .lead { max-width: 600px; color: #bdc9e1; font-size: 1.15rem; line-height: 1.65; }
    .release { margin: 32px 0; padding: 24px; border: 1px solid #334263; border-radius: 16px; background: #121a2c; }
    #release-status { margin: 0 0 18px; color: #c2cde2; }
    .button { display: inline-flex; align-items: center; min-height: 48px; padding: 0 20px; border-radius: 9px; background: #87aaff; color: #0b1020; font-weight: 700; text-decoration: none; }
    .button[hidden] { display: none; }
    .notes { margin: 16px 0 0; color: #93a4c7; font-size: .92rem; }
    .details { display: grid; gap: 14px; margin-top: 32px; color: #bdc9e1; line-height: 1.6; }
    .details p { margin: 0; }
    footer { margin-top: 56px; color: #93a4c7; font-size: .9rem; }
    @media (max-width: 520px) { main { padding-top: 36px; } nav { margin-bottom: 40px; } .release { padding: 18px; } }
  </style>
</head>
<body>
  <main>
    <nav aria-label="Main navigation"><a href="/">Vapor</a><a href="/docs/">Documentation</a></nav>
    <p class="eyebrow">Independent release</p>
    <h1>Download Vapor</h1>
    <p class="lead">Discover, install, and manage game content and tools from one place. Vapor is distributed independently of Steam and Loo-Cast.</p>
    <section class="release" aria-labelledby="release-heading">
      <h2 id="release-heading">Windows</h2>
      <p id="release-status" role="status" aria-live="polite">Checking for the latest stable release…</p>
      <a id="download-link" class="button" hidden>Download Vapor</a>
      <p id="release-notes" class="notes"></p>
    </section>
    <section class="details" aria-label="About installation">
      <p><strong>Your game stays yours.</strong> Vapor does not include Mad Max or other game files. It helps you locate and manage a game installation already on your computer.</p>
      <p><strong>Source checkouts stay in your Vapor workspace.</strong> Developer repositories are acquired on demand when you install developer tooling; a normal player install does not need them.</p>
      <p>Vapor updates are verified by the application before installation. Download links appear when a complete release is published.</p>
    </section>
    <footer><a href="/license">License</a> · <a href="/impressum">Impressum</a></footer>
  </main>
  <script>
    fetch('/api/updates/v1/vapor/stable/windows-x86_64/latest.json', {cache: 'no-store'})
      .then(async response => {
        if (!response.ok) throw new Error(response.status === 404 ? 'The first stable Vapor release is not available yet.' : 'Release information is temporarily unavailable. Please try again later.');
        return response.json();
      })
      .then(release => {
        const artifactUrl = new URL(release.artifact.url, window.location.origin);
        if (artifactUrl.origin !== window.location.origin ||
            !artifactUrl.pathname.startsWith('/api/updates/v1/vapor/releases/') ||
            !artifactUrl.pathname.endsWith('/vapor.tar.gz')) {
          throw new Error('The published download link is invalid. Please contact Vapor support.');
        }
        const link = document.getElementById('download-link');
        link.href = artifactUrl.href;
        link.hidden = false;
        link.textContent = `Download Vapor ${release.version}`;
        document.getElementById('release-status').textContent = `Latest stable release · ${release.version}`;
        if (release.release_notes_url) {
          const notesUrl = new URL(release.release_notes_url, window.location.origin);
          if (notesUrl.protocol === 'https:' || notesUrl.origin === window.location.origin) {
            const notes = document.createElement('a');
            notes.href = notesUrl.href;
            notes.rel = 'noopener noreferrer';
            notes.textContent = 'Release notes';
            document.getElementById('release-notes').append(notes);
          }
        }
      })
      .catch(error => { document.getElementById('release-status').textContent = error.message; });
  </script>
</body>
</html>
"##,
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
