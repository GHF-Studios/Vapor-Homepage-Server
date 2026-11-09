use axum::response::Html;
use axum::routing::get;
use axum::Router;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/dashboard", get(account_dashboard))
        .route("/dashboard/", get(account_dashboard))
        .route("/projects/{namespace}/{slug}", get(project_workspace))
}

async fn account_dashboard() -> Html<String> {
    Html(document("Your Projects", DASHBOARD_BODY))
}

async fn project_workspace() -> Html<String> {
    Html(document("Project", PROJECT_BODY))
}

fn document(title: &str, body: &str) -> String {
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><meta name=\"theme-color\" content=\"#10161d\"><title>{title} · Vapor</title>{STYLE}</head><body><main>{body}</main></body></html>"
    )
}

const STYLE: &str = r#"<style>
:root{color-scheme:dark;font-family:Inter,ui-sans-serif,system-ui,sans-serif;background:#10161d;color:#eef3f7;font-synthesis:none}*{box-sizing:border-box}body{margin:0;min-height:100vh;background:radial-gradient(ellipse at 52% -22%,#294346 0,#10161d 55%)}a{color:#a9d8cb}a:focus-visible,button:focus-visible{outline:2px solid #9fe1ce;outline-offset:3px}main{width:min(980px,calc(100% - 36px));margin:0 auto;padding:26px 0 64px}.topbar{display:flex;align-items:center;gap:22px;min-height:40px}.brand{margin-right:auto;color:#f0f5f7;font-weight:780;letter-spacing:.035em;text-decoration:none}.topbar nav{display:flex;gap:20px}.topbar nav a{color:#b7c5cb;text-decoration:none;font-size:.9rem}.eyebrow{color:#9fe1ce;font-size:.75rem;font-weight:750;letter-spacing:.16em;text-transform:uppercase}h1{margin:10px 0 8px;font-size:clamp(2.2rem,6vw,3.6rem);letter-spacing:-.055em;line-height:1.03}.lead{max-width:660px;margin:0;color:#b9c8cd;font-size:1rem;line-height:1.6}.heading{padding:64px 0 28px}.panel{border:1px solid #34434a;border-radius:10px;background:rgba(20,29,35,.9)}.project-list{display:grid;gap:10px;margin:18px 0 0;padding:0;list-style:none}.project-card{display:flex;align-items:center;justify-content:space-between;gap:18px;padding:16px 18px}.project-card a{text-decoration:none}.project-card a:hover{text-decoration:underline}.project-title{display:block;color:#eef3f7;font-size:1.05rem;font-weight:700}.project-summary{display:block;margin-top:4px;color:#aebdc3;font-size:.9rem;line-height:1.5}.role{flex:none;padding:4px 9px;border:1px solid #40565a;border-radius:999px;color:#a8ddd0;font-size:.72rem;font-weight:700;text-transform:capitalize}.state{margin:18px 0;padding:16px 18px;color:#c2cfd4;line-height:1.55}.state[hidden]{display:none}.state a{font-weight:650}.button{display:inline-flex;align-items:center;justify-content:center;min-height:40px;padding:0 14px;border:0;border-radius:6px;background:#9fe1ce;color:#10201d;font-weight:750;cursor:pointer}.workspace-meta{display:flex;flex-wrap:wrap;gap:8px 12px;margin-top:18px;color:#aebdc3;font-size:.86rem}.workspace-meta code{color:#d8e8e2}.back{display:inline-block;margin-top:24px;text-decoration:none}.error{border-color:#74434a;background:#251a1c}.empty{border-color:#526045;background:#1b211b}@media(max-width:600px){.topbar{align-items:flex-start;flex-wrap:wrap}.topbar nav{width:100%;justify-content:space-between}.heading{padding-top:48px}.project-card{align-items:flex-start;flex-direction:column}.role{align-self:flex-start}}
</style>"#;

const DASHBOARD_BODY: &str = r#"
<header class="topbar"><a class="brand" href="/">VAPOR</a><nav aria-label="Main navigation"><a href="/downloads">Get Vapor</a><a href="/docs/">Documentation</a><a href="/operator/login" aria-label="Platform operator sign-in">Operator access</a><a href="/logout">Sign out</a></nav></header>
<section class="heading"><span class="eyebrow">Your workspace</span><h1>Your Projects</h1><p class="lead">Projects you can access, with their own content and tools.</p></section>
<section aria-live="polite" aria-busy="true"><p id="loading" class="panel state">Loading your Projects…</p><div id="signed-out" class="panel state" hidden><strong>Sign in to continue.</strong><br><a href="/login">Sign in to Vapor</a></div><div id="empty" class="panel state empty" hidden><strong>No Projects yet.</strong><br>When you’re added to a Project, it will appear here.</div><div id="unavailable" class="panel state error" hidden><strong>Projects are temporarily unavailable.</strong><br>Try again in a moment. Your access has not changed.<br><button class="button" id="retry" type="button">Try again</button></div><ul id="projects" class="project-list" hidden></ul></section>
<script>
const stateIds=['loading','signed-out','empty','unavailable'];
function show(id){for(const item of stateIds)document.getElementById(item).hidden=item!==id;document.getElementById('projects').hidden=id!=='loaded';document.querySelector('section[aria-live]').setAttribute('aria-busy',id==='loading'?'true':'false');}
async function loadProjects(){show('loading');const list=document.getElementById('projects');list.replaceChildren();try{const response=await fetch('/api/registry/v1/me/projects',{credentials:'same-origin',cache:'no-store',headers:{'accept':'application/json'}});if(response.status===401||response.status===403){show('signed-out');return;}if(!response.ok)throw new Error('service');const data=await response.json();if(!Array.isArray(data.projects))throw new Error('response');if(data.projects.length===0){show('empty');return;}for(const project of data.projects){if(!project||typeof project.namespace!=='string'||typeof project.slug!=='string'||typeof project.display_name!=='string'||typeof project.role!=='string')continue;const item=document.createElement('li');item.className='panel project-card';const link=document.createElement('a');link.href='/projects/'+encodeURIComponent(project.namespace)+'/'+encodeURIComponent(project.slug);const title=document.createElement('span');title.className='project-title';title.textContent=project.display_name;const summary=document.createElement('span');summary.className='project-summary';summary.textContent=typeof project.summary==='string'?project.summary:'';link.append(title,summary);const role=document.createElement('span');role.className='role';role.textContent=project.role;item.append(link,role);list.append(item);}if(!list.children.length)throw new Error('response');show('loaded');}catch(_){show('unavailable');}}
document.getElementById('retry').addEventListener('click',loadProjects);loadProjects();
</script>
"#;

const PROJECT_BODY: &str = r#"
<header class="topbar"><a class="brand" href="/">VAPOR</a><nav aria-label="Main navigation"><a href="/dashboard">Your Projects</a><a href="/docs/">Documentation</a><a href="/operator/login" aria-label="Platform operator sign-in">Operator access</a><a href="/logout">Sign out</a></nav></header>
<section class="heading"><span class="eyebrow">Project workspace</span><h1 id="title">Loading Project…</h1><p id="summary" class="lead"></p></section>
<section aria-live="polite"><p id="loading" class="panel state">Checking Project access…</p><div id="signed-out" class="panel state" hidden><strong>Sign in to continue.</strong><br><a href="/login">Sign in to Vapor</a></div><div id="unavailable" class="panel state error" hidden><strong>This Project is unavailable.</strong><br>It may not exist, or your access may have changed.</div><div id="loaded" class="panel state" hidden><div class="workspace-meta"><span>Project ID: <code id="project-id"></code></span><span>Your access: <strong id="project-role"></strong></span><span>Status: <strong id="project-state"></strong></span></div><a class="back" href="/dashboard">← Back to your Projects</a></div></section>
<script>
const parts=window.location.pathname.split('/').filter(Boolean);const namespace=parts.length===3?decodeURIComponent(parts[1]):'';const slug=parts.length===3?decodeURIComponent(parts[2]):'';const loading=document.getElementById('loading');const signedOut=document.getElementById('signed-out');const unavailable=document.getElementById('unavailable');const loaded=document.getElementById('loaded');
async function loadProject(){try{const url='/api/registry/v1/projects/'+encodeURIComponent(namespace)+'/'+encodeURIComponent(slug);const response=await fetch(url,{credentials:'same-origin',cache:'no-store',headers:{'accept':'application/json'}});loading.hidden=true;if(response.status===401){signedOut.hidden=false;return;}if(!response.ok){unavailable.hidden=false;return;}const data=await response.json();const project=data.project;const membership=data.membership;if(!project||!membership)throw new Error('response');document.title=String(project.display_name||'Project')+' · Vapor';document.getElementById('title').textContent=String(project.display_name||'Project');document.getElementById('summary').textContent=typeof project.summary==='string'?project.summary:'';document.getElementById('project-id').textContent=String(project.id||'');document.getElementById('project-role').textContent=String(membership.role||'');document.getElementById('project-state').textContent=String(project.state||'');loaded.hidden=false;}catch(_){loading.hidden=true;unavailable.hidden=false;}}
loadProject();
</script>
"#;

#[cfg(test)]
mod tests {
    use super::router;
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn dashboard_is_a_user_view_without_operator_navigation() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/dashboard")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), 256 * 1024).await.unwrap();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("Your Projects"));
        assert!(html.contains("/api/registry/v1/me/projects"));
        assert!(html.contains("textContent"));
        assert!(!html.contains("/admin"));
    }

    #[tokio::test]
    async fn project_workspace_uses_the_membership_gated_registry_api() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/projects/ghf-studios/loo-cast")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), 256 * 1024).await.unwrap();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("/api/registry/v1/projects/"));
        assert!(html.contains("Project workspace"));
        assert!(!html.contains("/admin"));
    }
}
