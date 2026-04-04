---
title: Technical Debt Tracker
tags: [tech-debt, refactoring]
---
# Technical Debt

## Critical

- [ ] Replace deprecated `crypto` module with `argon2`
- [ ] Fix SQL injection vulnerability in search endpoint
- [ ] Upgrade PostgreSQL from v12 to v15

## Important

- [ ] Refactor monolithic auth module into separate service
- [ ] Remove unused dependencies from package.json
- [ ] Add proper error handling to webhook processor
- [x] Migrate from `moment.js` to `chrono`
- [x] Replace `unsafe` blocks with safe alternatives

## Nice to Have

- [ ] Standardize logging format across services
- [ ] Add structured logging with tracing
- [x] Remove dead code in legacy modules
- [x] Update all **README** files

> Technical debt is like financial debt — the longer you ignore it, the more interest you pay.

## Related

- See [[Sprint Backlog]] for current sprint items
- See [Deployment Guide](../docs/deployment.md) for infrastructure debt

#tech-debt #refactoring #security
