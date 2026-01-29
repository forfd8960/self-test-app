# Implementation Plan: Self Test App MVP

**Branch**: `002-self-test-mvp` | **Date**: 2026-01-29 | **Spec**: [specs/002-self-test-mvp/spec.md](specs/002-self-test-mvp/spec.md)
**Input**: Feature specification from `/specs/002-self-test-mvp/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Deliver a self-test web app where users register/login with username/password, upload learning materials (PDF/DOCX/TXT), configure question counts, generate AI-based questions, take tests, and receive scoring and feedback with history. The implementation uses a Rust (Axum + SQLx) backend, a React + TypeScript + Vite + Tailwind + Zustand frontend, PostgreSQL for persistence, local file storage, and an OpenAPI-compatible AI client with model and base URL configured via environment variables.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust (latest stable, edition 2024), TypeScript (latest), Node.js LTS  
**Primary Dependencies**: Axum, SQLx, Tokio, serde, jsonwebtoken/argon2, OpenAPI-compatible AI client; React, Zustand, Vite, Tailwind CSS  
**Storage**: PostgreSQL (primary), local file system for uploads  
**Testing**: cargo test + integration tests; Vitest/React Testing Library; optional Playwright for flows  
**Target Platform**: Local development deployment (macOS/Linux)  
**Project Type**: Web application (backend + frontend)  
**Performance Goals**: Generate questions for standard docs (≤20 pages) within 5 minutes; score/feedback within 1 minute of submission  
**Constraints**: JWT auth (username/password), local file storage, latest dependencies, AI calls may be long-running (use background job/polling). AI model and base URL must be configurable via environment variables.  
**Scale/Scope**: MVP for single-tenant use; tens to hundreds of active users; 1–2 primary flows (generate, take test, history)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Strict Type Safety**: PASS — Rust `Result` with typed errors; TypeScript `strict` and schema validation planned.
- **II. Automated Quality Gates**: PASS — lint/format/test + audits planned in CI.
- **III. Modular Layered Architecture**: PASS — backend layers (routes/handlers/services/infra) and frontend feature modules defined.
- **IV. Secure by Design**: PASS — JWT auth, Argon2 hashing, input validation, safe file storage.
- **V. User-Centric Reliability**: PASS — error boundaries, standardized error responses, tracing/logging.

**Post-Design Recheck**: PASS — data model, API contracts, and quickstart align with type safety, security, and reliability requirements.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```sh
backend/
├── src/
│   ├── api/            # Axum routes + handlers
│   ├── services/       # Business logic
│   ├── domain/         # Entities + types
│   ├── infra/          # DB, file storage, AI client
│   ├── config/         # App config
│   └── main.rs
└── tests/

frontend/
├── src/
│   ├── features/       # auth, materials, generation, tests, history
│   ├── components/     # shared UI components
│   ├── hooks/
│   ├── lib/            # api client, utilities
│   ├── styles/
│   └── main.tsx
└── tests/
```

**Structure Decision**: Use a backend/ frontend split to reflect the full-stack web application. Feature-based modules on the frontend and layered architecture on the backend align with the constitution.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
