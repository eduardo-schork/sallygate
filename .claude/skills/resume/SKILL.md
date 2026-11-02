---
name: resume
description: Pick up Sallygate work in a new session. Reads the living docs, checks the repo and the local checks, and reports where the work stands and what comes next. Use when the owner types /resume or asks where the project stands.
---

# Resume Sallygate work

Get up to speed without re-deriving anything, then report in a few lines. Change nothing.

1. Read, in this order: `PROJECT.md` (local-only brief; it may be missing), `docs/README.md`, the _Current focus_ block and the latest five progress-log lines of `docs/development-plan.md`, and the _Open questions_ section of `docs/architecture.md`.
2. Check the repo: `git status --short`, `git branch --show-current` (expect `develop`), `git log --oneline -5`, and `git log --oneline origin/develop..develop 2>/dev/null` (unpushed commits).
3. Check the environment, in parallel:
   - `bun run check` (run it in the background; it takes a few minutes);
   - `docker ps --filter name=sallygate-test-ssh-server` (the SSH test server; start it with `scripts/test-ssh-server.sh up` only before integration or manual tests).
4. Report to the owner, in Portuguese, in this shape:
   - **Where we are:** stage and what works today, one or two lines.
   - **Repo:** branch, uncommitted changes, unpushed commits.
   - **Checks:** pass, or the first failure.
   - **Next task:** the first item of _Next tasks_, and what it needs first (an ADR, a design pass with `frontend-design:frontend-design`, a question for the owner).
   - **Waiting on the owner:** the _Owner to do_ items and any open question that blocks the next task.
5. Ask the owner whether to start the next task. Do not start it on your own.

If the docs disagree with the code, say so in the report: the docs are living memory and must be fixed in the same task.
