# Vapor Homepage Server

Public homepage service for Vapor.

Initial implementation uses Axum/Tokio.

## Responsibility

- landing page;
- standalone Vapor download page, reading latest and stable history from Artifact;
- product and project overview;
- license/product text;
- Impressum/legal contact page;
- links to docs and other public surfaces.

## Route

Expected public routes behind the root reverse proxy:

```text
/
/downloads
```

The Homepage stays stateless. Its download page reads the latest stable
manifest and immutable stable version index from `Vapor-Artifact-Server` at
request time; the page does not need a deploy when a new stable release is
published.

## State

Expected to remain mostly stateless.

## Runtime binding

The service binds to `127.0.0.1:7111` by default. Deployment tooling may
override this with `VAPOR_HOMEPAGE_BIND`.

## Non-goals

- identity authority;
- diagnostics storage;
- docs artifact publishing;
- deployment orchestration.
