# TypeScript Frontend Development Guidelines

## Toolchain & Build

- Use modern Node.js (LTS) and a reliable package manager (npm, pnpm, or yarn). Lock versions with `package-lock.json` or equivalent.
- Use strict TypeScript settings: `"strict": true` in `tsconfig.json`.
- Enable strict lints: Use `eslint` with generic recommended configs and `prettier` for formatting.
- Run `npm run lint` and `npm run typecheck` before finishing tasks.
- Run tests and build (e.g., `npm run build`) on CI.
- Use `npm audit` to check for security vulnerabilities in dependencies.
- Prefer `vite` or `next.js` for fast development feedback loops.
- Configure path aliases in `tsconfig.json` (e.g., `@/components/*`) for cleaner imports.
- Treat warnings as errors in CI (`CI=true` often defaults to this in some frameworks, or configure explicitly).

## Error Handling

- Never rely solely on console logs for errors. Display user-friendly error messages (Toasts, Alerts).
- Use Error Boundaries to catch render-time errors and prevent whitespace screens (React).
- Handle Promise rejections globally [`window.onunhandledrejection`] and locally.
- Map API error codes to localized messages. Don't show raw backend traces to users.
- Use `zod` or similar libraries to validate external data at runtime; fail fast if the API contract is violated.
- Wrap risky third-party calls in `try/catch` logic.

## Async & Concurrency

- Use `React Query` (TanStack Query) or `SWR` for server state management instead of manual `useEffect` fetching.
- Prefer `async/await` over Promise chains (`.then().catch()`) for readability.
- Handle loading states explicitly (`isLoading`, `isFetching`).
- Use `AbortController` to cancel stale requests when components unmount or dependencies change (React Query handles this by default).
- Avoid "waterfalls" in data fetching; fetch in parallel where possible.
- Use simple state for simple concurrency; avoid complex custom locking mechanisms on the client.

## Type Design & API

- Share types with the backend if possible (via Monorepo or generated clients like generic OpenAPI generators).
- Define explicitly typed interfaces/types for all Component props.
- Use discriminated unions for handling multiple states (e.g., `type State = { status: 'loading' } | { status: 'success', data: T }`).
- Avoid `any`. Use `unknown` if the type isn't known yet, and narrow it down later.
- Use `zod` schemas to infer TypeScript types for API responses to ensure runtime safety matches compile-time expectations.

## Web Frameworks

- Prefer Functional Components with Hooks over Class Components.
- Custom Hooks should extract complex logic out of UI components.
- Keep components small and focused (Single Responsibility Principle).
- Use `Context` sparingly for global state; prefer dedicated state managers (Zustand, Redux Toolkit, or Atomic state like Jotai) for complex app state.
- Separation of concerns: Keep purely presentational components separate from "Container" or "Page" components that handle data code.

## State Management & Storage

- Distinguish between Server State (React Query), Global Client State (Zustand/Redux), and Local UI State (`useState`).
- Use `localStorage` / `sessionStorage` strictly for non-sensitive persistence (e.g., theme preference).
- Typed wrappers for `localStorage` to avoid string parsing everywhere.
- Do not store derived state in `useState`. Calculate it during render or use `useMemo`.

## Safety & Security

- Sanitize all dangerous HTML. Use `DOMPurify` if you must use `dangerouslySetInnerHTML`.
- Protect against XSS by using default framework escaping.
- Store auth tokens securely (HttpOnly cookies preferred over localStorage for preventing XSS token theft).
- Implement Content Security Policy (CSP) headers where possible.
- Validate all form inputs on the client side (UX) and expect validation on the server side (Security).

## Serialization & Data

- Use standard JSON for API communication.
- Handle Date objects carefully: use libraries like `date-fns` or `dayjs`. Avoid native `Date` parsing quirks.
- Be aware of JavaScript number precision limitations (53-bit). Use strings for 64-bit integers found in JSON from Rust/Go backends.
- Normalize data structure in Redux/State stores if data is highly relational/nested.

## Testing

- Unit test utility functions and hooks with `Vitest` or `Jest`.
- Integration test components with `React Testing Library`. Priority on "testing as the user would" (findByRole, findByText).
- End-to-End (E2E) test critical flows with `Playwright` or `Cypress`.
- Mock network requests using `MSW` (Mock Service Worker) for reproducible tests.
- Snapshot testing only for stability of small primitive components; avoid for large/complex layouts.

## Logging & Observability

- Use `console.warn` / `console.error` correctly during development.
- Integrate with error tracking services (Sentry, GlitchTip) for production runtime errors.
- Mask PII (Personal Identifiable Information) before sending logs to remote services.
- Add structured context (User ID, commit hash, environment) to error reports.

## Performance

- Profile using React DevTools Profiler to identify wasted renders.
- Memoize expensive calculations (`useMemo`) and stable function references (`useCallback`) when passing to memoized children.
- Lazy load routes and heavy components (`React.lazy`, `next/dynamic`).
- Optimize images (WebP/AVIF, proper sizing).
- Minimize bundle size: analyze with `webpack-bundle-analyzer` or similar tools; import only what you need (tree-shaking).

## Dependencies

- Audit dependencies regularly.
- Prefer well-maintained libraries with TypeScript support (first-party or DefinitelyTyped).
- Avoid adding heavy libraries for simple tasks (e.g., don't import `lodash` for a single map function).

## Documentation

- Use TSDoc (`/** ... */`) for exported functions, types, and hooks.
- Use Storybook for documenting UI component libraries.
- Maintain a `README.md` with setup, build, and contribution instructions.
- Document architectural decisions (e.g., why a certain state management lib was chosen).

## Code Style

- Enforce style via Prettier and Eslint.
- Naming conventions:
    - Components: `PascalCase.tsx`
    - Functions/Variables: `camelCase`
    - Constants: `SCREAMING_SNAKE_CASE`
    - Types/Interfaces: `PascalCase`
- Avoid "prop drilling" > 3 levels. Use Composition or Context.
- Co-locate styles and tests with components (e.g., `Button.tsx`, `Button.test.tsx`, `Button.module.css`).

## Project Structure

- Feature-based architecture often scales better than type-based:
    - `src/features/auth/` (components, api, specific hooks)
    - `src/features/dashboard/`
- `src/components/` for generic shared UI atoms (Button, Input).
- `src/hooks/` for shared generic hooks.
- `src/utils/` or `src/lib/` for pure helper functions.

## Configuration & Secrets

- Use `.env` files for environment variables. Prefix with `VITE_` or `NEXT_PUBLIC_` to expose to the client.
- **NEVER** put real secrets (private API keys, database passwords) in frontend code.
- Provide `.env.example`.

## API Design (Client Side)

- Centralize API calling logic (e.g., in `api/` folder or services).
- Use interceptors (Axios) or wrappers (Fetch) to handle auth tokens and global error codes.
- Type the API responses strictly.

## CI/CD

- Run Lint, Typecheck, and Tests on Pull Requests.
- Deploy previews for PRs (Vercel, Netlify) to facilitate visual review.
- Build production assets with optimization flags enabled.

## Security Practices

- Keep dependencies updated (Dependabot/Renovate).
- Review `package.json` scripts for malicious commands.
- Ensure external links use `rel="noopener noreferrer"`.

## Deployment & Operations

- Use CDNs for static assets.
- Configure proper caching headers (Cache-Control) for build artifacts (immutable for hashed files).
- Implement PWA features if offline support is needed.

## Code Review Checklist

- Accessibility (A11y): Semantic HTML, ARIA labels where needed.
- Responsive design: Works on mobile/tablet/desktop.
- Error states handling (What if the API fails?).
- Loading states handling (What does the user see while waiting?).
- efficient re-renders.

## Common Anti-Patterns

- `useEffect` for derived state calculations.
- Mutating state directly.
- Defining components inside other components.
- Using index as key in lists (when items can accept re-ordering).
- "Any" types everywhere.
