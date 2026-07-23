# Vapor Homepage Server

Public homepage service for Vapor.

Initial implementation uses Axum/Tokio.

## Responsibility

- landing page;
- product and project overview;
- license/product text;
- Impressum/legal contact page;
- links to docs and other public surfaces.

## Route

Expected public route behind the root reverse proxy:

```text
/
```

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
