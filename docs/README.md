# Sallygate docs

This folder is the project's living memory. Each kind of knowledge has one home, so humans and agents know where to read and where to write, and every file is kept current as the work moves.

| Room | What lives there | Update when |
|---|---|---|
| [development-plan.md](development-plan.md) | Stages, tasks, current focus and the progress log | Every work session: tick tasks, move the focus, add a log line |
| [roadmap.md](roadmap.md) | Versions and what each delivers, one line each | A version is planned, started or shipped |
| [architecture.md](architecture.md) | How the system is built today: components, boundaries, data flow, data model, open questions | A component, boundary, contract or data shape changes; an open question is answered |
| [decisions/](decisions/) | Architecture Decision Records (ADRs): one file per decision | A decision is made or reversed. Never rewrite an accepted ADR; supersede it with a new one. |
| [security.md](security.md) | What we protect, from whom, and the rules that follow | A new secret, trust boundary or attack surface appears |
| [glossary.md](glossary.md) | Domain and project terms | A new term shows up in code or docs |

## Living memory rules

1. **Docs change with the code.** A PR that changes behavior, structure or a decision also updates the matching docs. A PR that leaves the docs stale is not done.
2. **Start by reading, end by writing.** Before a task, read *Current focus* in the development plan and the rooms it touches. After a task, update the plan (tasks, focus, progress log) and any room the task changed.
3. **Docs describe what is true now.** When something changes, edit the text in place; history lives in git, the progress log and superseded ADRs.
4. **One fact, one place.** Link to it instead of copying it.
5. **Short and in English.** Each file readable in a few minutes; split it when it grows.
6. **No secrets or personal details.** Never write secrets, IP addresses or personal hostnames here.
