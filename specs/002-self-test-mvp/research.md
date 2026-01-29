# Phase 0 Research: Self Test App MVP

## Decisions

### AI Provider Integration (OpenAPI-compatible client)
- **Decision**: Use an OpenAPI-compatible AI client with base URL and model configured via environment variables.
- **Rationale**: Keeps the integration provider-agnostic and allows switching models/providers without code changes.
- **Alternatives considered**: Hard-coded MiniMax endpoints or model identifiers (less flexible).

### AI Request Reliability
- **Decision**: Implement retries with exponential backoff + jitter for 429/5xx and network errors; enforce client-side RPM/TPM budgeting.
- **Rationale**: MiniMax imposes RPM/TPM limits and returns rate-limit errors until window resets; backoff reduces error storms.
- **Alternatives considered**: No retries (lower reliability), immediate retries (amplifies throttling).

### Generation Workflow
- **Decision**: Use background jobs for AI generation and scoring; return a job ID and poll for status.
- **Rationale**: AI calls can be long-running and should not block HTTP request/response lifecycles.
- **Alternatives considered**: Fully synchronous generation (timeouts, poor UX).

### Auth Strategy
- **Decision**: JWT access tokens with short TTL (15–30 min) + refresh token rotation; store hashed refresh tokens server-side.
- **Rationale**: Limits exposure if tokens leak and supports revocation.
- **Alternatives considered**: Long-lived access tokens (simpler, less secure); opaque sessions (higher server state).

### File Upload Handling
- **Decision**: Stream multipart uploads directly to disk with size limits; validate file type via extension, MIME, and magic bytes.
- **Rationale**: Prevents memory spikes and blocks malicious file types.
- **Alternatives considered**: Buffer entire upload in memory (unsafe for large files).

### Text Extraction Pipeline
- **Decision**: Extract text per format: PDF via `pdf_extract` (or equivalent), DOCX via ZIP + XML parsing (`zip` + `quick-xml`), TXT via direct read; run extraction in a background worker.
- **Rationale**: Keeps API latency low and avoids blocking async runtime; format-specific extraction avoids lossy conversions.
- **Alternatives considered**: External CLI tooling (e.g., `pandoc`) with OS dependencies; a single multi-format extractor (less control).

### Backend Architecture
- **Decision**: Axum layered structure: routes → handlers → services → infra (DB, storage, AI client). Use SQLx compile-time checked queries.
- **Rationale**: Enforces separation of concerns and type safety.
- **Alternatives considered**: Handler-only business logic (harder to test);

### Frontend State Management
- **Decision**: React + Zustand with domain-specific stores (`auth`, `materials`, `generation`, `tests`, `history`) and typed API client layer.
- **Rationale**: Keeps global state minimal and predictable; ensures type-safe UI flows.
- **Alternatives considered**: Single monolithic store; Redux Toolkit (heavier setup).

### Form Validation
- **Decision**: Use `react-hook-form` with Zod schemas for client-side validation that mirrors backend contracts.
- **Rationale**: Consistent validation rules and improved UX for multi-step flows.
- **Alternatives considered**: Formik; ad-hoc validation in components.

## Open Questions

- None for MVP; all technical choices defined based on existing stack guidance.
