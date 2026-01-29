# Self Test App

Self Test APP - AI-powered personalized test generation from user-uploaded learning materials.

## Current Tech Stack

**Backend**
- Rust (Axum)
- SQLx (PostgreSQL)
- Time/Serde, UUID
- AI client wrapper (OpenAI-compatible)

**Frontend**
- React + Vite
- TypeScript
- MUI (Material UI)
- Zustand

**Database**
- PostgreSQL

## Architecture

**High-level flow**
1. User authenticates (JWT).
2. User uploads learning material.
3. AI generates a question set.
4. User takes test; backend scores and generates feedback.
5. History endpoints provide attempts and detailed review.

**Backend layers**
- `api/`: Axum routes and handlers
- `services/`: business logic (generation, scoring, feedback, history)
- `infra/`: repositories, DB, storage, AI client
- `domain/`: core models and error types

**Frontend structure**
- `features/`: domain-specific UI (auth, generation, tests, history)
- `lib/`: API client and shared utilities
- `layouts/`: page shells

## How to Start

**Backend**
1. Ensure PostgreSQL is running.
2. Create a database (e.g., `selftestapp`) and run migrations.
3. Start the backend server:

```bash
cd backend
cargo run
```

**Frontend**
```bash
cd frontend
npm install
npm run dev
```

## Common Issues

- **Invalid Date in UI**: Ensure backend serializes timestamps as ISO 8601. `OffsetDateTime` must use serde ISO8601 annotations.
- **Route panic**: Axum uses `/{id}` path syntax (not `/:id`).
- **MUI Grid mismatch**: Use a consistent MUI Grid version (v1 `Grid` with `xs/md` props).
- **AI feedback includes `<think>` blocks**: Strip or sanitize before display.
- **DB column mismatch**: Keep schema and repository insert/select columns in sync (`user_answer`, `completed_at`, `score`).

## Feature Improvements (Recommended)

- Add contract tests for history endpoints and test submission.
- Store detailed answer options and selected choices for MCQ (not just `String`).
- Add pagination and filtering on history list.
- Persist AI feedback in a structured format (summary + strengths + weaknesses).
- Add retry/backoff for AI calls and surface errors in UI.
- Add role-based access control or admin review tools.
