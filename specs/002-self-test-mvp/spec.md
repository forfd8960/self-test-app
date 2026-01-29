# Feature Specification: Self Test App MVP

**Feature Branch**: `002-self-test-mvp`  
**Created**: 2026-01-29  
**Status**: Draft  
**Input**: User description: "Self test application that allows users to upload learning materials (PDF, DOCX, TXT, etc.) and generates personalized test questions (multiple-choice and fill-in-the-blank) using AI, with scoring, feedback, and history."

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Generate a personalized question set (Priority: P1)

As a new user, I want to register/login, upload learning materials, and configure question counts so that I can receive a generated question set based on my content.

**Why this priority**: This is the core value of the product—turning learning materials into a personalized test.

**Independent Test**: Can be fully tested by registering, uploading a valid file, setting counts, and verifying a generated question list appears.

**Acceptance Scenarios**:

1. **Given** a new user without an account, **When** they register and log in, **Then** they can access the upload and generation setup.
2. **Given** a logged-in user with a valid PDF/DOCX/TXT uploaded, **When** they set question parameters and start generation, **Then** the system creates and stores a question set and shows a generated list.

---

### User Story 2 - Take a test and receive score with feedback (Priority: P2)

As a learner, I want to answer generated questions and receive a score and constructive feedback so that I can understand what to review.

**Why this priority**: Feedback is the learning outcome and differentiator after the questions are generated.

**Independent Test**: Can be tested by opening a generated test, submitting answers, and verifying score and feedback output.

**Acceptance Scenarios**:

1. **Given** a generated question set, **When** the user submits their answers, **Then** the system computes a score and provides feedback tied to the learning materials.

---

### User Story 3 - Review historical test records (Priority: P3)

As a returning user, I want to view my past test results so that I can track progress over time.

**Why this priority**: History improves long-term learning and retention but depends on test completion.

**Independent Test**: Can be tested by completing a test and confirming that a history entry is visible with details.

**Acceptance Scenarios**:

1. **Given** a user has completed at least one test, **When** they open their history page, **Then** they can view scores, answer details, and feedback for each attempt.

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

- Uploading an unsupported file type or a corrupted file is rejected with a clear message.
- Generation fails or times out; user sees a retry option and the system does not create a partial test.
- User submits a test with unanswered questions; scoring handles blank answers explicitly.
- Network interruption during upload or test submission results in a recoverable state (user can retry without data loss).

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST allow users to register and log in using credentials and issue a JWT for authenticated access.
- **FR-002**: System MUST allow authenticated users to upload learning materials in PDF, DOCX, or TXT format.
- **FR-003**: System MUST reject unsupported file types and provide a user-readable error message.
- **FR-004**: Users MUST be able to configure question generation parameters, including counts for multiple-choice and fill-in-the-blank questions.
- **FR-005**: System MUST generate questions and answers based on uploaded materials and persist them with the user’s configuration.
- **FR-006**: System MUST present a generated test page listing the questions in a consumable format.
- **FR-007**: Users MUST be able to submit answers for a generated test.
- **FR-008**: System MUST compute a score and provide constructive feedback referencing weak and strong areas.
- **FR-009**: System MUST store test attempts, answers, scores, and feedback as historical records.
- **FR-010**: Users MUST be able to view their historical test records with score, answer details, and feedback.

### Key Entities *(include if feature involves data)*

- **User**: Authenticated account holder; owns materials, tests, and history.
- **LearningMaterial**: Uploaded content file with metadata (name, type, size, upload time).
- **GenerationConfig**: User-selected parameters for question counts and types.
- **QuestionSet**: Generated questions and answers linked to a material and configuration.
- **TestAttempt**: A user’s session with answers, score, and feedback.
- **FeedbackSummary**: Structured notes about weak/strong areas tied to a test attempt.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: 90% of users can complete registration, upload a material, and start generation in under 5 minutes on first attempt.
- **SC-002**: 95% of generation requests complete successfully for standard documents (≤ 20 pages) within 5 minutes.
- **SC-003**: 90% of completed tests produce a score and feedback within 1 minute after submission.
- **SC-004**: At least 80% of users can locate and open a past test result within 30 seconds.

## Assumptions

- Authentication uses username + password with JWT-based sessions.
- The initial release supports PDF, DOCX, and TXT uploads only.
- Feedback is generated in the same language as the uploaded material when possible.
