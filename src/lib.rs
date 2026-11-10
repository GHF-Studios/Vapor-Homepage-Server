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
        r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="Vapor is an independent platform for discovering, installing, and managing game ecosystems, content, and tools.">
  <meta name="theme-color" content="#10161d">
  <title>Vapor · Game ecosystems, managed</title>
  <style>
    :root { color-scheme: dark; font-family: Inter, ui-sans-serif, system-ui, sans-serif; background: #101415; color: #eef2ef; font-synthesis: none; }
    * { box-sizing: border-box; }
    body { margin: 0; min-height: 100vh; background: #101415; }
    a { color: #a7d5c5; }
    a:focus-visible { outline: 2px solid #9fe1ce; outline-offset: 4px; border-radius: 4px; }
    main { width: min(1120px, calc(100% - 48px)); margin: 0 auto; padding: 24px 0 32px; }
    .topbar { display: flex; align-items: center; gap: 24px; min-height: 40px; }
    .brand { margin-right: auto; color: #eef2ef; font-size: 1rem; font-weight: 600; text-decoration: none; }
    .topbar nav { display: flex; align-items: center; gap: 20px; }
    .topbar nav a, footer a { color: #b8c2bd; font-size: .9rem; text-decoration: none; }
    .topbar nav a:hover, footer a:hover { color: #fff; }
    .hero { display: grid; grid-template-columns: minmax(0, 1.08fr) minmax(300px, .92fr); align-items: center; gap: 48px; padding: 88px 0 72px; }
    .eyebrow { color: #b6c2bb; font-size: .84rem; font-weight: 500; }
    h1 { max-width: 690px; margin: 12px 0 20px; font-size: clamp(2.7rem, 6.4vw, 4.6rem); font-weight: 500; letter-spacing: -.045em; line-height: 1.03; }
    .lead { max-width: 640px; margin: 0; color: #bdc7c2; font-size: clamp(1rem, 1.8vw, 1.15rem); line-height: 1.6; }
    .actions { display: flex; flex-wrap: wrap; align-items: center; gap: 20px; margin-top: 28px; }
    .button { display: inline-flex; min-height: 44px; align-items: center; justify-content: center; padding: 0 16px; border: 1px solid #9fe1ce; border-radius: 8px; background: #9fe1ce; color: #13201b; font-weight: 600; text-decoration: none; }
    .button:hover { background: #b2eadb; }
    .secondary { font-size: .92rem; font-weight: 500; text-decoration: none; }
    .mount-preview { padding: 20px; border: 1px solid #35413b; border-radius: 14px; background: #171c19; }
    .mount-head { display: flex; align-items: flex-start; justify-content: space-between; gap: 12px; padding-bottom: 16px; border-bottom: 1px solid #303a34; }
    .mount-head p, .mount-head h2 { margin: 0; }
    .mount-head p { color: #9daaa2; font-size: .8rem; }
    .mount-head h2 { margin-top: 4px; font-size: 1.05rem; font-weight: 550; }
    .mount-state { color: #a7d5c5; font-size: .78rem; white-space: nowrap; }
    .manifest-row { display: flex; align-items: center; gap: 10px; margin: 16px 0 10px; padding: 10px 12px; border: 1px solid #35413b; border-radius: 8px; background: #1d2420; }
    .manifest-mark { width: 8px; height: 8px; border-radius: 2px; background: #9fe1ce; }
    .manifest-row code, .target-row code { color: #d3ddd7; font: 12px/1.4 ui-monospace, "Cascadia Code", monospace; }
    .target-list { display: grid; gap: 6px; }
    .target-row { display: flex; align-items: center; justify-content: space-between; gap: 12px; min-height: 40px; padding: 0 12px; border: 1px solid #303a34; border-radius: 8px; }
    .target-name { color: #b8c2bd; font-size: .84rem; }
    .mount-foot { margin: 14px 0 0; color: #9daaa2; font-size: .8rem; line-height: 1.5; }
    .intro-grid { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 12px; }
    .card { min-height: 148px; padding: 20px; border: 1px solid #303a34; border-radius: 12px; background: #171c19; }
    .card h2 { margin: 0 0 8px; font-size: 1rem; font-weight: 550; }
    .card p { margin: 0; color: #b8c2bd; font-size: .9rem; line-height: 1.55; }
    .local-note { margin-top: 12px; padding: 16px 18px; border: 1px solid #303a34; border-radius: 10px; background: #151a17; }
    .local-note h2 { margin: 0 0 4px; font-size: .94rem; font-weight: 550; }
    .local-note p { margin: 0; color: #b8c2bd; font-size: .86rem; line-height: 1.55; }
    .bottom-cta { display: flex; align-items: center; justify-content: space-between; gap: 24px; margin: 40px 0 44px; padding: 20px 22px; border: 1px solid #303a34; border-radius: 12px; background: #171c19; }
    .bottom-cta h2 { margin: 0 0 4px; font-size: 1.1rem; font-weight: 550; }
    .bottom-cta p { margin: 0; color: #b8c2bd; font-size: .9rem; line-height: 1.5; }
    footer { display: flex; flex-wrap: wrap; gap: 18px; padding: 16px 0 8px; border-top: 1px solid #303a34; }
    footer .copyright { margin-right: auto; color: #9daaa2; font-size: .82rem; }
    @media (max-width: 720px) {
      .topbar { align-items: flex-start; flex-wrap: wrap; gap: 14px; }
      .topbar nav { width: 100%; justify-content: space-between; gap: 10px; }
      main { width: min(100% - 32px, 560px); }
      .hero { grid-template-columns: 1fr; gap: 28px; padding: 56px 0 40px; }
      .intro-grid { grid-template-columns: 1fr; }
      .card { min-height: 0; }
      .bottom-cta { align-items: flex-start; flex-direction: column; margin: 28px 0; }
      .target-row { align-items: flex-start; flex-direction: column; justify-content: center; gap: 2px; padding: 8px 10px; }
    }
    @media (prefers-reduced-motion: reduce) { *, *::before, *::after { scroll-behavior: auto !important; } }
  </style>
</head>
<body>
  <main>
    <header class="topbar">
      <a class="brand" href="/" aria-label="Vapor home">VAPOR</a>
      <nav aria-label="Main navigation">
        <a href="/dashboard">Your Projects</a>
        <a href="/downloads">Get Vapor</a>
        <a href="/docs/">Documentation</a>
        <a href="/license">License</a>
      </nav>
    </header>

    <section class="hero" aria-labelledby="hero-title">
      <div>
        <span class="eyebrow">A standalone runtime for game ecosystems</span>
        <h1 id="hero-title">Install the runtime.<br>Mount the ecosystem.</h1>
        <p class="lead">Vapor installs and manages games, tools, and content. Each ecosystem defines its own packages and lifecycle; licensed game files stay on your machine.</p>
        <div class="actions">
          <a class="button" href="/downloads">Get Vapor</a>
          <a class="secondary" href="/docs/">Explore the documentation</a>
        </div>
      </div>
      <div class="mount-preview" aria-label="Example of the Loo Cast Steam product mount">
        <div class="mount-head"><div><p>Steam product mount</p><h2>Loo Cast Packagepack</h2></div><span class="mount-state">First-party</span></div>
        <div class="manifest-row"><span class="manifest-mark" aria-hidden="true"></span><code>Packagepack.vapor.toml</code></div>
        <div class="target-list">
          <div class="target-row"><span class="target-name">Linux · x86_64</span><code>loo-cast-packagepack</code></div>
          <div class="target-row"><span class="target-name">Windows · x86_64</span><code>loo-cast-packagepack.exe</code></div>
        </div>
        <p class="mount-foot">Vapor and its tools remain in Vapor home, separate from the Steam product.</p>
      </div>
    </section>

    <section class="intro-grid" aria-label="What Vapor does">
      <article class="card">
        <h2>Use supported games</h2>
        <p>Install Vapor once, then add games you own and manage the content their ecosystem supports.</p>
      </article>
      <article class="card">
        <h2>Build across targets</h2>
        <p>The Developer setup includes Vapor’s pinned Rust toolchain and the Linux and Windows build tools.</p>
      </article>
      <article class="card">
        <h2>Keep ownership clear</h2>
        <p>Games supply their licensed files locally. Vapor manages the runtime and only distributes authorized content.</p>
      </article>
    </section>

    <aside class="local-note" aria-labelledby="local-title">
      <div>
        <h2 id="local-title">Your game files stay yours</h2>
        <p>Commercial game files come from your own installation. They are not included in Vapor downloads or hosted as community content.</p>
      </div>
    </aside>

    <section class="bottom-cta" aria-labelledby="start-title">
      <div>
        <h2 id="start-title">Start with Vapor</h2>
        <p>Choose your setup and see current download availability for Windows and Linux.</p>
      </div>
      <a class="button" href="/downloads">View setup options</a>
    </section>

    <footer>
      <span class="copyright">Vapor · an independent platform</span>
      <a href="/docs/">Docs</a>
      <a href="/license">License</a>
      <a href="/impressum">Impressum</a>
      <a href="/operator/login">Operator sign-in</a>
    </footer>
  </main>
</body>
</html>
"##,
    )
}

async fn downloads_page() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="Get Vapor for Windows or Linux, understand the Player and Developer setups, and get started with locally owned games.">
  <meta name="theme-color" content="#10161d">
  <title>Get Vapor · Vapor</title>
  <style>
    :root { color-scheme: dark; font-family: Inter, ui-sans-serif, system-ui, sans-serif; background: #10161d; color: #eef3f7; font-synthesis: none; }
    * { box-sizing: border-box; }
    body { margin: 0; min-height: 100vh; background: #101415; }
    a { color: #a9d8cb; }
    a:focus-visible, button:focus-visible { outline: 2px solid #9fe1ce; outline-offset: 3px; }
    main { width: min(1040px, calc(100% - 36px)); margin: 0 auto; padding: 28px 0 64px; }
    nav { display: flex; align-items: center; gap: 22px; padding-bottom: 54px; }
    .brand { margin-right: auto; color: #eef3f7; font-weight: 500; text-decoration: none; }
    nav a:not(.brand) { color: #bac7ce; text-decoration: none; font-size: .94rem; }
    nav a:hover { color: #fff; }
    .hero { max-width: 760px; margin-bottom: 38px; }
    .eyebrow { color: #b8c2bd; font-size: .86rem; font-weight: 500; }
    h1 { margin: 12px 0 16px; font-size: clamp(2.8rem, 8vw, 5.2rem); font-weight: 500; letter-spacing: -.04em; line-height: 1.02; }
    h2 { margin: 0; font-size: 1.05rem; letter-spacing: -.01em; }
    h3 { margin: 0 0 8px; font-size: .98rem; }
    .lead { max-width: 700px; margin: 0; color: #c3cfd4; font-size: 1.12rem; line-height: 1.65; }
    .section-heading { display: flex; align-items: end; justify-content: space-between; gap: 16px; margin: 0 0 14px; }
    .section-heading p { margin: 0; color: #91a1a8; font-size: .86rem; }
    .release-grid, .setup-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 12px; }
    .release, .setup, .steps { border: 1px solid #303a34; border-radius: 10px; background: #171c19; }
    .release { min-height: 168px; padding: 16px; }
    .release-head { display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 12px; }
    .platform { color: #a1aaa5; font-size: .8rem; font-weight: 500; }
    .release-status { min-height: 42px; margin: 0 0 14px; color: #bbc8ce; line-height: 1.5; }
    .button { display: inline-flex; align-items: center; justify-content: center; min-height: 40px; padding: 0 16px; border: 1px solid #9fe1ce; border-radius: 8px; background: #9fe1ce; color: #10201d; font-weight: 500; text-decoration: none; }
    .button:hover { background: #b8f0df; }
    .button[hidden] { display: none; }
    .release-details { margin-top: 12px; color: #94a5ac; font-size: .82rem; }
    .release-details summary { width: fit-content; cursor: pointer; }
    .release-details p { overflow-wrap: anywhere; line-height: 1.5; }
    .release-details a { margin-right: 12px; }
    .setup-section, .start-section { margin-top: 32px; }
    .setup { padding: 16px; }
    .setup p, .steps p { margin: 0; color: #b9c6cc; font-size: .92rem; line-height: 1.6; }
    .badge { display: inline-flex; margin-bottom: 8px; color: #b8c2bd; font-size: .82rem; font-weight: 500; }
    .steps { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 0; overflow: hidden; }
    .step { padding: 16px; }
    .step + .step { border-left: 1px solid #34434a; }
    .step-number { display: block; margin-bottom: 8px; color: #b8c2bd; font-size: .8rem; font-weight: 500; }
    .step p { color: #b9c6cc; font-size: .9rem; line-height: 1.6; }
    .notice { margin-top: 18px; padding: 14px 16px; border-left: 3px solid #9fe1ce; background: rgba(159, 225, 206, .07); color: #bfccd1; font-size: .9rem; line-height: 1.55; }
    footer { display: flex; gap: 18px; margin-top: 48px; color: #91a1a8; font-size: .88rem; }
    footer a { color: #a9d8cb; }
    @media (max-width: 680px) { main { padding-top: 20px; } nav { padding-bottom: 40px; } .release-grid, .setup-grid { grid-template-columns: 1fr; } .steps { grid-template-columns: 1fr; } .step + .step { border-left: 0; border-top: 1px solid #34434a; } .section-heading { align-items: start; flex-direction: column; } }
  </style>
</head>
<body>
  <main>
    <nav aria-label="Main navigation">
      <a class="brand" href="/">Vapor</a>
      <a href="/dashboard">Your Projects</a>
      <a href="/docs/">Documentation</a>
      <a href="/license">License</a>
    </nav>
    <header class="hero">
      <p class="eyebrow">Vapor · standalone runtime</p>
      <h1>Get started with Vapor.</h1>
      <p class="lead">One runtime for games, tools, and community content. Vapor installs and updates independently; your games and their licensed files stay on your machine.</p>
    </header>

    <section aria-labelledby="download-heading">
      <div class="section-heading">
        <h2 id="download-heading">Choose your platform</h2>
        <p>Stable releases are listed only after Artifact publishes a complete signed bundle.</p>
      </div>
      <div class="release-grid">
        <article class="release" aria-labelledby="windows-heading">
          <div class="release-head"><h2 id="windows-heading">Windows</h2><span class="platform">x86_64</span></div>
          <p class="release-status" id="windows-status" role="status" aria-live="polite">Checking the stable release feed…</p>
          <a id="windows-download" class="button" hidden>Download Vapor</a>
          <details class="release-details" id="windows-details" hidden>
            <summary>Release verification details</summary>
            <p id="windows-integrity"></p>
            <div id="windows-links"></div>
          </details>
        </article>
        <article class="release" aria-labelledby="linux-heading">
          <div class="release-head"><h2 id="linux-heading">Linux</h2><span class="platform">x86_64</span></div>
          <p class="release-status" id="linux-status" role="status" aria-live="polite">Checking the stable release feed…</p>
          <a id="linux-download" class="button" hidden>Download Vapor</a>
          <details class="release-details" id="linux-details" hidden>
            <summary>Release verification details</summary>
            <p id="linux-integrity"></p>
            <div id="linux-links"></div>
          </details>
        </article>
      </div>
      <p class="notice">A download appears only when that platform has a complete release. The current Artifact bundle is an application archive; a guided installer and one-click update flow are still being completed. Do not treat an archive as an installer.</p>
    </section>

    <section class="setup-section" aria-labelledby="setup-heading">
      <div class="section-heading"><h2 id="setup-heading">Choose your setup</h2><p>Vapor’s planned setup offers two roles with different local tooling.</p></div>
      <div class="setup-grid">
        <article class="setup">
          <span class="badge">Player</span>
          <h3>Use Vapor to manage games and content</h3>
          <p>Choose Player during setup for the everyday runtime. Add a game you already own and manage its supported content locally.</p>
        </article>
        <article class="setup">
          <span class="badge">Developer</span>
          <h3>Build and work on ecosystems</h3>
          <p>When Developer setup ships, it is intended to include the development toolchain and cross-platform build tools as required parts of the role, not separate optional installs.</p>
        </article>
      </div>
    </section>

    <section class="start-section" aria-labelledby="start-heading">
      <div class="section-heading"><h2 id="start-heading">After downloading</h2><p>What happens next</p></div>
      <div class="steps">
        <article class="step"><span class="step-number">01 · DOWNLOAD</span><h3>Choose the right next step</h3><p>The download above is an application archive, not an installer. Use it only if you already know how to work with application archives; otherwise, wait for Vapor’s guided installer. Vapor’s install, updates, and removal are independent of Steam.</p></article>
        <article class="step"><span class="step-number">02 · CHOOSE</span><h3>Pick Player or Developer</h3><p>Player is for using games and content. When Developer setup is available, it will install the managed tools needed to work on ecosystems.</p></article>
        <article class="step"><span class="step-number">03 · ADD A GAME</span><h3>Connect a local game install</h3><p>Vapor does not distribute Mad Max or Loo Cast game files. You must acquire those products separately; Vapor uses local files and does not upload them.</p></article>
      </div>
      <p class="notice">Loo Cast’s first-party products are intended to come from its local Steam depot. Mad Max is an external game that must already be installed locally. Neither game’s files are part of a Vapor download or release feed.</p>
    </section>

    <footer><a href="/docs/">Documentation</a><a href="/license">License</a><a href="/impressum">Impressum</a></footer>
  </main>
  <script>
    const releaseTargets = [
      { platform: 'windows-x86_64', key: 'windows', label: 'Windows' },
      { platform: 'linux-x86_64', key: 'linux', label: 'Linux' }
    ];
    const safeSegment = value => typeof value === 'string' && /^[A-Za-z0-9._+-]{1,100}$/.test(value);
    const addLink = (parent, href, label) => {
      const link = document.createElement('a');
      link.href = href;
      link.rel = 'noopener noreferrer';
      link.textContent = label;
      parent.append(link);
    };
    async function loadRelease(target) {
      const status = document.getElementById(`${target.key}-status`);
      try {
        const response = await fetch(`/api/updates/v1/vapor/stable/${target.platform}/latest.json`, {cache: 'no-store'});
        if (response.status === 404) throw new Error(`No stable ${target.label} release has been published yet.`);
        if (!response.ok) throw new Error('Release information is temporarily unavailable. Please try again later.');
        const release = await response.json();
        if (release.channel !== 'stable' || release.platform !== target.platform || !safeSegment(release.version) ||
            !release.artifact || !Number.isSafeInteger(release.artifact.size) || release.artifact.size <= 0 ||
            typeof release.artifact.sha256 !== 'string' || !/^[a-f0-9]{64}$/i.test(release.artifact.sha256)) {
          throw new Error('The published release metadata is incomplete or invalid. Please contact Vapor support.');
        }
        const expectedPath = `/api/updates/v1/vapor/releases/${release.version}/${target.platform}/vapor.tar.gz`;
        const artifactUrl = new URL(release.artifact.url, window.location.origin);
        if (artifactUrl.origin !== window.location.origin || artifactUrl.pathname !== expectedPath || artifactUrl.search || artifactUrl.hash) {
          throw new Error('The published download link is invalid. Please contact Vapor support.');
        }
        const link = document.getElementById(`${target.key}-download`);
        link.href = artifactUrl.href;
        link.textContent = `Download ${release.version}`;
        link.hidden = false;
        status.textContent = `Stable release ${release.version} is available.`;
        document.getElementById(`${target.key}-integrity`).textContent = `${release.artifact.size.toLocaleString()} bytes · SHA-256 ${release.artifact.sha256}`;
        const links = document.getElementById(`${target.key}-links`);
        addLink(links, `/api/updates/v1/vapor/releases/${release.version}/${target.platform}/manifest.json`, 'Signed manifest');
        addLink(links, `/api/updates/v1/vapor/releases/${release.version}/${target.platform}/manifest.json.sig`, 'Detached signature');
        if (typeof release.release_notes_url === 'string') {
          const notesUrl = new URL(release.release_notes_url, window.location.origin);
          if (notesUrl.protocol === 'https:' || notesUrl.origin === window.location.origin) addLink(links, notesUrl.href, 'Release notes');
        }
        document.getElementById(`${target.key}-details`).hidden = false;
      } catch (error) {
        status.textContent = error instanceof Error ? error.message : 'Release information is temporarily unavailable.';
      }
    }
    releaseTargets.forEach(loadRelease);
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

#[cfg(test)]
mod tests {
    use super::app_router;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn homepage_has_a_discreet_operator_sign_in_entry() {
        let response = app_router()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 16 * 1024)
            .await
            .unwrap();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("href=\"/operator/login\">Operator sign-in</a>"));
    }

    #[tokio::test]
    async fn homepage_explains_the_runtime_and_uses_a_product_specific_mount_preview() {
        let response = app_router()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), 16 * 1024)
            .await
            .unwrap();
        let html = std::str::from_utf8(&body).unwrap();

        assert!(html.contains("Vapor installs and manages games, tools, and content"));
        assert!(html.contains("Packagepack.vapor.toml"));
        assert!(html.contains("Steam product mount"));
        assert!(html.contains("class=\"mount-preview\""));
        assert!(!html.contains("radial-gradient"));
        assert!(!html.contains("box-shadow: 0 0 70px"));
    }

    #[tokio::test]
    async fn downloads_page_uses_quiet_surfaces_and_sentence_case_labels() {
        let response = app_router()
            .oneshot(Request::get("/downloads").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), 64 * 1024)
            .await
            .unwrap();
        let html = std::str::from_utf8(&body).unwrap();

        assert!(!html.contains("radial-gradient"));
        assert!(!html.contains("text-transform: uppercase"));
        assert!(!html.contains("font-weight: 750"));
        assert!(html.contains("Choose your platform"));
    }

    #[tokio::test]
    async fn account_project_pages_are_not_served_by_the_public_homepage_service() {
        for path in ["/dashboard", "/projects/ghf-studios/loo-cast"] {
            let response = app_router()
                .oneshot(Request::get(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND, "{path}");
        }
    }
}
