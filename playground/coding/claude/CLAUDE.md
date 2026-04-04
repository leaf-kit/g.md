# E-Commerce Platform - CLAUDE.md

## 1. Project Overview
- Next.js 14 + TypeScript e-commerce platform
- Target: B2C online shopping with 10K+ daily active users
- Core features: product catalog, cart, checkout, user accounts

## 2. Project Structure
- `app/` : App Router pages and layouts
- `components/` : Shared UI components (PascalCase.tsx)
- `hooks/` : Custom React hooks (useXxx.ts)
- `lib/` : Utility functions and API clients
- `styles/` : Tailwind config and global styles
- `tests/` : Jest + React Testing Library

## 3. Code Style
- TypeScript strict mode, no `any`
- React components: PascalCase, functional only
- Hooks: `useXxx` naming convention
- API calls: wrapped in `lib/api/` with error handling
- No `console.log` in production code

## 4. Testing Rules
- All components must have unit tests
- Jest + React Testing Library
- Minimum 80% coverage for new code
- E2E tests with Playwright for critical flows

## 5. Workflow
1. Analyze request and summarize in 2-3 lines
2. Propose change plan as markdown list
3. Wait for user approval before modifying code
4. After changes, provide summary + 2-3 improvement ideas

## 6. Rule Files
- `rules/code_conventions.md`
- `rules/testing.md`
- `rules/api_design.md`
