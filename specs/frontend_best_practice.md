# Frontend Development Rules & Best Practices
**React + TypeScript + Vite + npm**

## Project Structure & Organization

Organize your project with clear separation of concerns:

```
src/
├── components/     # Reusable UI components
├── features/      # Feature-based modules
├── hooks/         # Custom React hooks
├── utils/         # Helper functions
├── types/         # TypeScript type definitions
├── styles/        # Global styles and themes
├── api/           # API client and services
└── assets/        # Images, fonts, static files
```

Keep components small and focused. If a component exceeds 200-300 lines, consider breaking it into smaller pieces.

## TypeScript Best Practices

Always define explicit types for props, state, and function parameters. Avoid using `any` except in rare edge cases:

```typescript
interface ButtonProps {
  label: string;
  onClick: () => void;
  variant?: 'primary' | 'secondary';
  disabled?: boolean;
}

const Button = ({ label, onClick, variant = 'primary', disabled = false }: ButtonProps) => {
  // Implementation
};
```

Use type inference where TypeScript can automatically determine types, but be explicit when it improves clarity. Leverage union types, enums, and utility types like `Partial`, `Pick`, and `Omit` to create flexible, maintainable type definitions.

## Component Design Principles

Follow the single responsibility principle. Each component should do one thing well. Prefer functional components with hooks over class components unless you have a specific reason to use classes.

Use composition over inheritance. Build complex UIs by combining simple components rather than creating deep inheritance hierarchies:

```typescript
// Good: Composition
<Card>
  <CardHeader title="Dashboard" />
  <CardContent>
    <Statistics data={stats} />
  </CardContent>
</Card>

// Avoid: Overly specific combined components
<DashboardCardWithStatsAndHeader />
```

Keep your component API minimal and intuitive. Fewer props with clear purposes are better than many props with overlapping functionality.

## State Management

Use local state for UI-specific data that doesn't need to be shared. Lift state up only when multiple components need to share it. For complex state logic, use `useReducer` instead of multiple `useState` calls.

Consider using context for truly global state like themes, authentication, or language preferences, but avoid overusing context as it can make component reusability harder and cause unnecessary re-renders.

For larger applications, consider state management libraries like Zustand, Jotai, or Redux Toolkit, but only introduce them when you have a genuine need. Don't add complexity prematurely.

## Performance Optimization

Memoize expensive calculations with `useMemo` and callbacks with `useCallback`, but only when you've identified actual performance issues. Premature optimization can make code harder to read and maintain.

Use React's built-in lazy loading for code splitting:

```typescript
const Dashboard = lazy(() => import('./features/Dashboard'));

<Suspense fallback={<LoadingSpinner />}>
  <Dashboard />
</Suspense>
```

Virtualize long lists using libraries like `react-window` or `react-virtuoso`. Avoid rendering thousands of DOM elements at once.

## Styling Approaches

Choose a consistent styling approach for your project. Popular options include CSS Modules for scoped styles, Tailwind CSS for utility-first styling, or styled-components/emotion for CSS-in-JS.

Regardless of your choice, maintain a consistent design system with reusable spacing, colors, and typography variables. Use CSS custom properties for theme values that might change:

```css
:root {
  --color-primary: #3b82f6;
  --spacing-unit: 8px;
  --border-radius: 4px;
}
```

## Error Handling & Boundaries

Always implement error boundaries to catch rendering errors gracefully:

```typescript
class ErrorBoundary extends Component<PropsWithChildren, { hasError: boolean }> {
  state = { hasError: false };
  
  static getDerivedStateFromError() {
    return { hasError: true };
  }
  
  render() {
    if (this.state.hasError) {
      return <ErrorFallback />;
    }
    return this.props.children;
  }
}
```

Handle async errors in your components and provide meaningful feedback to users. Don't let errors fail silently.

## Accessibility (a11y)

Build accessible interfaces from the start. Use semantic HTML elements like `button`, `nav`, `main`, and `header` instead of divs with click handlers. Provide proper ARIA labels when needed:

```typescript
<button 
  aria-label="Close dialog"
  onClick={onClose}
>
  <CloseIcon aria-hidden="true" />
</button>
```

Ensure keyboard navigation works throughout your app. All interactive elements should be reachable and operable via keyboard. Test your app with a screen reader periodically.

Maintain sufficient color contrast ratios (4.5:1 for normal text, 3:1 for large text) and never rely solely on color to convey information.

## API Integration & Data Fetching

Centralize your API calls in dedicated service files rather than scattering fetch calls throughout components. Use libraries like `axios` or `ky` for better error handling and request configuration.

Consider using `TanStack Query` (React Query) for server state management. It handles caching, background updates, and loading states elegantly:

```typescript
const { data, isLoading, error } = useQuery({
  queryKey: ['user', userId],
  queryFn: () => fetchUser(userId),
});
```

Always handle loading and error states in your UI. Users should never see undefined data or blank screens without explanation.

## Forms & Validation

For complex forms, use libraries like `react-hook-form` or `formik` to manage form state and validation. These libraries reduce boilerplate and improve performance.

Validate input on both client and server. Client-side validation provides immediate feedback, but server-side validation is essential for security.

Provide clear, specific error messages and show them near the relevant input fields. Indicate required fields clearly before users attempt submission.

## Testing Strategy

Write tests for critical user flows and complex business logic. Focus on integration tests that verify component behavior from the user's perspective rather than implementation details.

Use React Testing Library for component tests:

```typescript
test('submits form with valid data', async () => {
  render(<ContactForm onSubmit={mockSubmit} />);
  
  await userEvent.type(screen.getByLabelText(/email/i), 'test@example.com');
  await userEvent.click(screen.getByRole('button', { name: /submit/i }));
  
  expect(mockSubmit).toHaveBeenCalledWith({ email: 'test@example.com' });
});
```

Keep tests simple, readable, and focused on user behavior rather than implementation. If you find yourself testing internal state or mocking extensively, reconsider your approach.

## Build & Development Configuration

Leverage Vite's fast HMR and build times. Configure environment variables properly using Vite's `import.meta.env` rather than `process.env`.

Set up path aliases in `vite.config.ts` for cleaner imports:

```typescript
export default defineConfig({
  resolve: {
    alias: {
      '@': '/src',
      '@components': '/src/components',
    },
  },
});
```

Use environment-specific configurations for development, staging, and production. Never commit sensitive keys or tokens to version control.

## Code Quality & Consistency

Set up ESLint and Prettier to enforce consistent code style. Use TypeScript's strict mode to catch more potential errors:

```json
{
  "compilerOptions": {
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  }
}
```

Implement pre-commit hooks with Husky and lint-staged to automatically format and lint code before commits. This keeps the codebase clean without requiring manual effort.

## Naming Conventions

Use PascalCase for components and interfaces. Use camelCase for functions, variables, and props. Use SCREAMING_SNAKE_CASE for constants.

Name files consistently. Component files should match the component name: `Button.tsx`, `UserProfile.tsx`. Custom hooks should start with `use`: `useAuth.ts`, `useDebounce.ts`.

Choose descriptive, meaningful names over short, cryptic ones. `handleSubmitUserRegistrationForm` is better than `handleSubmit` when the context isn't obvious.

## Comments & Documentation

Write self-documenting code with clear variable and function names. Add comments to explain why something is done a certain way, not what the code does.

Document complex algorithms, non-obvious performance optimizations, and workarounds for external library bugs. Add JSDoc comments for public APIs and reusable utilities:

```typescript
/**
 * Formats a date string to a user-friendly format
 * @param date - ISO date string or Date object
 * @param locale - Optional locale string (defaults to 'en-US')
 * @returns Formatted date string
 */
export const formatDate = (date: string | Date, locale = 'en-US'): string => {
  // Implementation
};
```

## Security Considerations

Sanitize user input before rendering, especially when using `dangerouslySetInnerHTML`. Use libraries like DOMPurify for sanitization.

Store sensitive data like authentication tokens securely. Use httpOnly cookies for tokens when possible. Never store sensitive data in localStorage without encryption.

Implement Content Security Policy headers to prevent XSS attacks. Validate and sanitize all data from external sources, even from your own API.

## Dependency Management

Keep dependencies updated but test thoroughly after updates. Use `npm audit` regularly to check for security vulnerabilities.

Prefer well-maintained libraries with active communities. Check the last commit date, number of open issues, and bundle size before adding a dependency.

Regularly review and remove unused dependencies. Use tools like `depcheck` to identify packages that are no longer needed.

## Performance Monitoring

Implement performance monitoring in production. Use the built-in Performance API or tools like Lighthouse, Web Vitals, or Sentry.

Monitor key metrics like First Contentful Paint, Largest Contentful Paint, Cumulative Layout Shift, and Time to Interactive. Set performance budgets and alerts.

Profile your application regularly using React DevTools Profiler to identify unnecessary re-renders and optimize critical paths.

---

These practices evolve with the ecosystem. Stay updated with React and TypeScript best practices, but always evaluate new patterns critically before adopting them. Consistency and maintainability should guide your decisions more than trends.