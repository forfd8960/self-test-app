<!--
Sync Impact Report:
- Version Change: Template -> 1.0.0
- Added Principles:
  - I. Strict Type Safety
  - II. Automated Quality Gates
  - III. Modular Layered Architecture
  - IV. Secure by Design
  - V. User-Centric Reliability
- Added Sections:
  - Technology Standards
  - Development Workflow
- Templates requiring updates:
  - ✅ specs/backend_dev_practice.md (Source of Truth)
  - ✅ specs/frontend_best_practice.md (Source of Truth)
  - ✅ specs/rust_backend_guidelines.md (Aligned)
  - ✅ specs/typescript_frontend_guidelines.md (Aligned)
-->

# Self Test App Constitution

## Core Principles

### I. Strict Type Safety
Enforce strict typing across the entire technical stack to eliminate runtime errors.
- **Backend (Rust)**: Must use `Result<T, E>` for error propagation (no panics), newtypes for domain identifiers (e.g., `struct UserId(Uuid)`), and SQLx compile-time query verification. `unwrap()` and `expect()` are forbidden in production code.
- **Frontend (TypeScript)**: Must use `"strict": true` in compiler settings. Define explicit interfaces for component props and API responses. Use Zod schemas to validate runtime data at API boundaries. The `any` type is prohibited.

### II. Automated Quality Gates
Quality is enforced by tooling, not just human review.
- Every commit must pass a localized CI check (Lint + Format + Test).
- **Tooling**: Rust uses `cargo clippy` (pedantic warnings enabled) and `rustfmt`. TypeScript uses `eslint` and `prettier`.
- **Security**: Automated audits (`cargo audit`, `npm audit`) must run on CI.
- **Policy**: Warnings are treated as errors in CI pipelines. Broken builds or failed tests block merging.

### III. Modular Layered Architecture
Maintain a rigid separation of concerns to ensure testability and maintainability.
- **Backend Layers**: Routes (HTTP) → Handlers (Orchestration) → Services (Business Logic) → infra/DB (Persistence). Dependencies flow downwards only.
- **Frontend Structure**: Feature-based organization (`features/auth`, `features/quiz`) takes precedence over technical types. Shared logic lives in `components/` or `hooks/`.
- Logic must be decoupled from frameworks where possible (e.g., Services testable without HTTP server; Hooks testable without UI rendering).

### IV. Secure by Design
Security is proactive and non-negotiable.
- **Input/Output**: Validate all external inputs at the system boundary using strict schemas. Sanitize all HTML outputs to prevent XSS.
- **Authentication**: Use Industry Standard protocols (JWT for stateless auth, Argon2 for password hashing).
- **Data Protection**: Never commit secrets or API keys to version control. Use `.env` and secret managers. Use parameterized queries (SQLx) exclusively to prevent SQL injection.

### V. User-Centric Reliability
The system must be robust and helpful, never failing silently.
- **Backend**: Errors must be mapped to standardized HTTP codes and structured JSON responses (never expose internal stack traces to users).
- **Frontend**: Must utilize Error Boundaries to catch render crashes. Async operations must handle loading and error states explicitly (e.g., via React Query); providing user-friendly feedback (Toasts/Alerts).
- **Observability**: All flows must be traceable via structured logs/request IDs to facilitate rapid debugging.

## Technology Standards

- **Backend Stack**: Rust (Latest Stable), Axum (Web Framework), SQLx (PostgreSQL), async-openai (AI Integration), Tokio (Runtime).
- **Frontend Stack**: React (Functional + Hooks), TypeScript, Vite, Tailwind CSS. 
- **Data Persistence**: PostgreSQL for relational data; Local File System for document uploads (initially).
- **AI Integration**: OpenAI GPT models for content generation (Questions/Feedback).

## Development Workflow

1.  **Branching Strategy**: Use short-lived feature branches targeting `main`.
2.  **Code Review**: All Pull Requests require at least one peer approval and a passing CI check. Code must be reviewed for correctness, security, and performance.
3.  **Testing Strategy**:
    *   **Unit**: Required for all business logic (Services/Hooks).
    *   **Integration**: Required for API endpoints and critical UI Flows.
4.  **Documentation**: Public APIs and complex logic must be documented (Rustdoc/TSDoc). Architectural decisions must be recorded in ADRs.

## Governance

This Constitution supersedes all other practice documents.
- **Amendments**: Changes to these principles require a specific "Constitution Amendment" Pull Request with a clear motivation and impact assessment.
- **Versioning**: Principles follow Semantic Versioning. Breaking changes to governance require a MAJOR version bump.
- **Compliance**: All code contributions must adhere to these principles. Deviations must be explicitly justified in code comments or ADRs.

**Version**: 1.0.0 | **Ratified**: 2026-01-29 | **Last Amended**: 2026-01-29
