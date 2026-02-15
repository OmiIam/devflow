# QUICK_REFERENCE.md

| Task | Command |
| --- | --- |
| Install frontend deps | `cd frontend && npm ci` |
| Run frontend dev | `npm run dev` |
| Run frontend tests | `npx vitest run --coverage` |
| Run backend dev | `cd backend && cargo watch -x run` |
| Run backend tests | `cargo test --all` |
| Compose integration stack | `docker compose -f docker-compose.test.yml up --build --abort-on-container-exit` |
| Playwright e2e | `npx playwright test -c tests/e2e/playwright.config.ts` |
| Lint & format | `npm run lint && cargo fmt` |
| Backend coverage | `cargo llvm-cov --html` |

- Branch regex: `^(feat|fix|chore|docs|refactor|test)/(issue-\d+|hotfix-\d+|exp-[a-z0-9-]+)$`.
- Commit template: `<type>(<scope>): <subject>` (see `IMPLEMENTATION_RULES.md`).
- CI order: frontend → backend → compose → docker → e2e → deploy.
- Contacts: @frontend-lead, @backend-lead, @platform-oncall.

