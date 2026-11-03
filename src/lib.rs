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
  <meta name="description" content="Get Vapor for Windows or Linux, understand the Player and Developer setups, and get started with locally owned games.">
  <meta name="theme-color" content="#10161d">
  <title>Get Vapor · Vapor</title>
  <style>
    :root { color-scheme: dark; font-family: Inter, ui-sans-serif, system-ui, sans-serif; background: #10161d; color: #eef3f7; font-synthesis: none; }
    * { box-sizing: border-box; }
    body { margin: 0; min-height: 100vh; background: radial-gradient(ellipse at 50% -24%, #263b3d 0, #10161d 54%); }
    a { color: #a9d8cb; }
    a:focus-visible, button:focus-visible { outline: 2px solid #9fe1ce; outline-offset: 3px; }
    main { width: min(1040px, calc(100% - 36px)); margin: 0 auto; padding: 28px 0 64px; }
    nav { display: flex; align-items: center; gap: 22px; padding-bottom: 54px; }
    .brand { margin-right: auto; color: #eef3f7; font-weight: 750; letter-spacing: .02em; text-decoration: none; }
    nav a:not(.brand) { color: #bac7ce; text-decoration: none; font-size: .94rem; }
    nav a:hover { color: #fff; }
    .hero { max-width: 760px; margin-bottom: 38px; }
    .eyebrow { color: #9fe1ce; font-size: .78rem; font-weight: 700; letter-spacing: .16em; text-transform: uppercase; }
    h1 { margin: 12px 0 16px; font-size: clamp(2.8rem, 8vw, 5.2rem); letter-spacing: -.06em; line-height: .98; }
    h2 { margin: 0; font-size: 1.05rem; letter-spacing: -.01em; }
    h3 { margin: 0 0 8px; font-size: .98rem; }
    .lead { max-width: 700px; margin: 0; color: #c3cfd4; font-size: 1.12rem; line-height: 1.65; }
    .section-heading { display: flex; align-items: end; justify-content: space-between; gap: 16px; margin: 0 0 14px; }
    .section-heading p { margin: 0; color: #91a1a8; font-size: .86rem; }
    .release-grid, .setup-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 14px; }
    .release, .setup, .steps { border: 1px solid #34434a; border-radius: 12px; background: rgba(20, 29, 35, .91); }
    .release { min-height: 200px; padding: 20px; }
    .release-head { display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 14px; }
    .platform { color: #92a3aa; font-size: .78rem; font-weight: 650; }
    .release-status { min-height: 42px; margin: 0 0 14px; color: #bbc8ce; line-height: 1.5; }
    .button { display: inline-flex; align-items: center; justify-content: center; min-height: 42px; padding: 0 16px; border: 0; border-radius: 7px; background: #9fe1ce; color: #10201d; font-weight: 750; text-decoration: none; }
    .button:hover { background: #b8f0df; }
    .button[hidden] { display: none; }
    .release-details { margin-top: 12px; color: #94a5ac; font-size: .82rem; }
    .release-details summary { width: fit-content; cursor: pointer; }
    .release-details p { overflow-wrap: anywhere; line-height: 1.5; }
    .release-details a { margin-right: 12px; }
    .setup-section, .start-section { margin-top: 38px; }
    .setup { padding: 20px; }
    .setup p, .steps p { margin: 0; color: #b9c6cc; font-size: .92rem; line-height: 1.6; }
    .badge { display: inline-flex; margin-bottom: 12px; padding: 4px 8px; border: 1px solid #40565a; border-radius: 999px; color: #a8ddd0; font-size: .72rem; font-weight: 700; letter-spacing: .06em; text-transform: uppercase; }
    .steps { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 0; overflow: hidden; }
    .step { padding: 20px; }
    .step + .step { border-left: 1px solid #34434a; }
    .step-number { display: block; margin-bottom: 12px; color: #9fe1ce; font-size: .76rem; font-weight: 700; letter-spacing: .1em; }
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
