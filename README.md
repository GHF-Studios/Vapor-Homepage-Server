# Vapor Homepage Server

Public homepage service for Vapor.

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

## Non-goals

- identity authority;
- diagnostics storage;
- docs artifact publishing;
- deployment orchestration.

