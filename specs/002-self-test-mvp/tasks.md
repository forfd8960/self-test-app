---
description: "Task list for Self Test App MVP"
---

# Tasks: Self Test App MVP

**Input**: Design documents from `/specs/002-self-test-mvp/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/
**Tests**: Included (unit + integration tests requested in requirements)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create backend workspace skeleton in backend/Cargo.toml and backend/src/main.rs
- [x] T002 Create frontend app skeleton in frontend/package.json and frontend/src/main.tsx
- [x] T003 [P] Configure backend formatting/linting in backend/rustfmt.toml and backend/.clippy.toml
- [x] T004 [P] Configure frontend linting/formatting in frontend/.eslintrc.cjs and frontend/.prettierrc
- [x] T005 [P] Add environment templates in backend/.env.example and frontend/.env.example

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure required before any user story work

- [x] T006 Create backend config loader in backend/src/config/mod.rs
- [x] T007 Implement structured error types in backend/src/domain/error.rs
- [x] T008 [P] Add DB pool setup in backend/src/infra/db.rs
- [x] T009 [P] Add file storage utilities in backend/src/infra/storage.rs
- [x] T010 [P] Add OpenAPI-compatible AI client wrapper in backend/src/infra/ai_client.rs (model/base URL from env)
- [x] T010a [P] Add per-user AI rate limiter (10 requests/min) in backend/src/infra/rate_limit.rs
- [x] T011 Create auth middleware in backend/src/api/middleware/auth.rs
- [x] T012 Create API router base in backend/src/api/router.rs
- [x] T013 Create shared API response types in backend/src/api/response.rs
- [x] T014 Add SQLx migrations baseline in backend/migrations/0001_init.sql
- [x] T015 [P] Add frontend API client wrapper in frontend/src/lib/api.ts
- [x] T016 [P] Add frontend auth store in frontend/src/features/auth/store.ts

**Checkpoint**: Foundation ready for story implementation

---

## Phase 3: User Story 1 - Generate a personalized question set (Priority: P1) 🎯 MVP

**Goal**: User can register/login, upload materials, configure generation, and receive a generated question list.

**Independent Test**: Register/login, upload a valid file, submit generation config, and see a generated question list.

### Tests for User Story 1

- [x] T017 [P] [US1] Backend integration tests for auth endpoints in backend/tests/auth_flow.rs
- [x] T018 [P] [US1] Backend integration tests for material upload in backend/tests/materials_flow.rs
- [x] T019 [P] [US1] Backend integration tests for generation endpoints in backend/tests/generation_flow.rs
- [x] T020 [P] [US1] Frontend auth flow tests in frontend/tests/auth.spec.tsx
- [x] T021 [P] [US1] Frontend upload + generation tests in frontend/tests/generation.spec.tsx

### Implementation for User Story 1

- [x] T022 [P] [US1] Implement User model in backend/src/domain/user.rs
- [x] T023 [P] [US1] Implement LearningMaterial model in backend/src/domain/learning_material.rs
- [x] T024 [P] [US1] Implement GenerationConfig model in backend/src/domain/generation_config.rs
- [x] T025 [P] [US1] Implement QuestionSet + Question models in backend/src/domain/question.rs
- [x] T026 [US1] Implement auth service in backend/src/services/auth_service.rs
- [x] T027 [US1] Implement material upload service in backend/src/services/material_service.rs
- [x] T028 [US1] Implement text extraction pipeline in backend/src/services/extraction_service.rs
- [x] T029 [US1] Implement generation service with job status in backend/src/services/generation_service.rs
- [x] T029a [US1] Add AI retry policy with exponential backoff in backend/src/services/generation_service.rs
- [x] T030 [US1] Implement auth routes in backend/src/api/auth.rs
- [x] T031 [US1] Implement materials routes in backend/src/api/materials.rs
- [x] T032 [US1] Implement generation routes in backend/src/api/generation.rs
- [x] T033 [US1] Add SQLx queries for US1 entities in backend/src/infra/repositories.rs
- [x] T034 [P] [US1] Build auth UI screens in frontend/src/features/auth/pages
- [x] T035 [P] [US1] Build upload UI in frontend/src/features/materials/pages
- [x] T036 [P] [US1] Build generation setup UI in frontend/src/features/generation/pages
- [x] T037 [US1] Implement generation API calls in frontend/src/features/generation/api.ts
- [x] T038 [US1] Implement question list page in frontend/src/features/generation/components/QuestionList.tsx

**Checkpoint**: User Story 1 functional and independently testable

---

## Phase 4: User Story 2 - Take a test and receive score with feedback (Priority: P2)

**Goal**: User can submit answers and receive score and feedback.

**Independent Test**: Open a generated test, submit answers, and receive score + feedback.

### Tests for User Story 2

- [ ] T039 [P] [US2] Backend integration tests for test submission in backend/tests/test_attempts_flow.rs
- [ ] T040 [P] [US2] Frontend test-taking flow tests in frontend/tests/test_taking.spec.tsx

### Implementation for User Story 2

- [ ] T041 [P] [US2] Implement TestAttempt and Answer models in backend/src/domain/test_attempt.rs
- [ ] T042 [US2] Implement scoring service in backend/src/services/scoring_service.rs
- [ ] T043 [US2] Implement feedback generation in backend/src/services/feedback_service.rs
- [ ] T044 [US2] Implement test submission routes in backend/src/api/tests.rs
- [ ] T045 [US2] Add SQLx queries for test attempts in backend/src/infra/repositories.rs
- [ ] T046 [P] [US2] Build test-taking UI in frontend/src/features/tests/pages
- [ ] T047 [US2] Implement test submission API in frontend/src/features/tests/api.ts
- [ ] T048 [US2] Build score + feedback UI in frontend/src/features/tests/components/ScoreSummary.tsx

**Checkpoint**: User Story 2 functional and independently testable

---

## Phase 5: User Story 3 - Review historical test records (Priority: P3)

**Goal**: User can view historical test attempts with details and feedback.

**Independent Test**: Complete a test and verify history list and detail views.

### Tests for User Story 3

- [ ] T049 [P] [US3] Backend integration tests for history endpoints in backend/tests/history_flow.rs
- [ ] T050 [P] [US3] Frontend history flow tests in frontend/tests/history.spec.tsx

### Implementation for User Story 3

- [ ] T051 [US3] Implement history query service in backend/src/services/history_service.rs
- [ ] T052 [US3] Implement history routes in backend/src/api/history.rs
- [ ] T053 [US3] Add SQLx queries for history detail in backend/src/infra/repositories.rs
- [ ] T054 [P] [US3] Build history list UI in frontend/src/features/history/pages
- [ ] T055 [P] [US3] Build history detail UI in frontend/src/features/history/components/HistoryDetail.tsx
- [ ] T056 [US3] Implement history API client in frontend/src/features/history/api.ts

**Checkpoint**: User Story 3 functional and independently testable

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T057 [P] Add API error mapping and toast notifications in frontend/src/lib/errors.ts
- [ ] T058 [P] Add request logging and tracing in backend/src/infra/logging.rs
- [ ] T059 [P] Add health checks in backend/src/api/health.rs
- [ ] T060 [P] Update README with local run steps in README.md
- [ ] T061 [P] Validate quickstart instructions in specs/002-self-test-mvp/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** → **Foundational (Phase 2)** → **User Stories (Phases 3–5)** → **Polish (Phase 6)**
- User stories can run in parallel after Phase 2, but should ship in priority order (P1 → P2 → P3).

### User Story Dependencies

- **US1**: Depends on Foundational only.
- **US2**: Depends on Foundational and US1 data (question sets).
- **US3**: Depends on Foundational and completed test attempts (US2).

## Parallel Execution Examples

### User Story 1

- [ ] T022 [P] [US1] Implement User model in backend/src/domain/user.rs
- [ ] T023 [P] [US1] Implement LearningMaterial model in backend/src/domain/learning_material.rs
- [ ] T024 [P] [US1] Implement GenerationConfig model in backend/src/domain/generation_config.rs
- [ ] T034 [P] [US1] Build auth UI screens in frontend/src/features/auth/pages

### User Story 2

- [ ] T041 [P] [US2] Implement TestAttempt and Answer models in backend/src/domain/test_attempt.rs
- [ ] T046 [P] [US2] Build test-taking UI in frontend/src/features/tests/pages

### User Story 3

- [ ] T054 [P] [US3] Build history list UI in frontend/src/features/history/pages
- [ ] T055 [P] [US3] Build history detail UI in frontend/src/features/history/components/HistoryDetail.tsx

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 and Phase 2.
2. Implement US1 tasks (T017–T038).
3. Validate US1 independently (auth → upload → generate → question list).

### Incremental Delivery

1. Deliver US1 (MVP).
2. Add US2 (test + scoring + feedback).
3. Add US3 (history list/detail).
4. Complete Polish phase.
