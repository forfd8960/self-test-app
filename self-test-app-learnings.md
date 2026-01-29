# Self Test App Learnings

## Purpose
Capture issues and edge cases that surfaced during development but were not explicitly covered in the design/specs. Use this as a checklist for future spec-driven development.

## Data & Schema Mismatches
- **Column name drift**: Code used `response`, `submitted_at`, `score_percent` while DB schema used `user_answer`, `completed_at`, `score`. This caused 500s on submit.
- **Schema vs DTO naming**: API DTOs used camelCase or different names than DB columns; mapping rules were unclear.
- **Not-null vs optional**: Missing clarity on which fields are optional (e.g., `completed_at`, `score`, feedback) led to insert failures or UI misreads.

## API Contract Gaps
- **History endpoints not in router**: The `/tests` list and `/tests/{id}` detail existed in OpenAPI but were not wired into the server router.
- **Route syntax mismatch**: Axum uses `/{id}` not `/:id`. This caused runtime panic.
- **Response shape for history**: No explicit contract for history detail (attempt + enriched answers). UI and backend inferred structure inconsistently.

## Serialization & Frontend Parsing
- **Time serialization**: `OffsetDateTime` serialized in a format JS `Date` couldn’t parse, causing `Invalid Date` in UI. Needed ISO 8601 serde annotations.
- **Null score handling**: UI didn’t guard `null` score; resulted in `%` with no number.

## AI Output Handling
- **Unfiltered reasoning blocks**: AI feedback included `<think>...</think>` content, which surfaced in UI. Needed sanitization.
- **AI response variability**: Formatting drift between AI responses and expected parsing logic required stricter validation.

## React & UI Integration Issues
- **Hooks order violations**: `useMemo` placed after early returns caused “Rendered more hooks” error.
- **MUI Grid version mismatch**: Using Grid v2 props (`xs`, `md`) or importing `Grid2` without dependency support caused runtime errors. Needed consistent Grid version.
- **Icon sizing**: SVG icons rendered oversized due to missing explicit sizing; required width/height on SVGs.

## Tooling & Workflow Issues
- **Server restart failure**: `pkill && cd backend && cargo run` sometimes failed due to working directory assumptions.
- **Untracked artifacts**: `uploads/` and `ai_response.txt` should be ignored or documented in `.gitignore`.

## Spec Gaps & Improvements for Future SDLC
1. **Single source of truth for field names**
   - Specify exact DB column names and API field names with explicit mapping rules.
2. **Time formats**
   - Require ISO 8601 for all timestamps; add examples in OpenAPI.
3. **History detail schema**
   - Define full response shape (attempt + question metadata + user response) in contract.
4. **Error model coverage**
   - Include error codes for common failures (not found, forbidden, validation).
5. **UI empty/error states**
   - Specify default UI behavior for null score, missing feedback, empty answers.
6. **Route syntax and framework constraints**
   - Document Axum path patterns and note incompatibilities with other router syntax.
7. **AI output sanitization**
   - Add a requirement to strip reasoning, HTML tags, and unexpected tokens.
8. **MUI versioning constraints**
   - Note MUI Grid version expectations and component usage guidelines.

## Actionable Checklist for Next Project
- Align DB schema ↔ API ↔ frontend models explicitly.
- Add contract tests for submission + history endpoints.
- Define serialization rules for dates and nullables.
- Validate AI output before persistence or display.
- Document frontend UI edge cases and error UX.
