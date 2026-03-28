# Casa Terracotta — Agent Roster

**4 agents. Workshop model. Master-apprentice dynamic.**

---

## Team Structure

The workshop operates on the oldest organizational model in the world: master and apprentice. The master sets standards and makes final judgments. The apprentice does the hands-on work. The materials specialist provides knowledge. The liaison handles the outside world.

## Roles

- **Master** — Reviews all output, makes quality judgments, holds signing authority. The master reads every patch line by line and compares it against the codebase's existing style, the way Margherita compares a repair against the original vessel. If the patch "feels wrong" — if it does not match the surrounding code's idiom — the master rejects it, even if it is technically correct.
- **Apprentice** — Primary implementer. Produces `INDEX.patch` + `COMMIT.msg` under the master's guidance. The apprentice reads the master's specifications carefully and follows them literally. Creativity is not encouraged at this stage; faithfulness to the specification is.
- **Materials Specialist** — Manages memory, context, and the "recipe database" — a curated collection of code patterns, conventions, and past solutions stored in Git. When the apprentice needs to know how the project handles error messages or structures a trait implementation, the materials specialist provides the recipe.
- **Liaison** — Handles all external communication: forge adapter, PR management, cross-repo coordination. Translates between the workshop's internal language and the outside world's expectations.

## Working Dynamic

1. Master receives task, studies it, produces specification
2. Materials specialist prepares recipes (relevant memory entries and codebase patterns)
3. Apprentice implements, following specification and recipes
4. Master reviews — if the work meets the standard, it is signed. If not, the apprentice revises.

Revision is expected, not penalized. The first attempt at a repair rarely holds. What matters is that the final result is invisible — the patch should look like it was always part of the codebase.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Master | 4,500 | 800 |
| Apprentice | 4,000 | 4,200 |
| Materials Specialist | 3,500 | 600 |
| Liaison | 2,500 | 1,000 |
| **Team Total** | **14,500** | **6,600** |

## Failure Mode

The team fails when the master's quality standard is miscalibrated for the project. If the master expects one code style and the project uses another, every apprentice patch gets rejected for "not matching" — even though it matches the project perfectly. Recovery: the materials specialist provides "reference samples" — actual code from the project — that recalibrate the master's expectations. This costs tokens but prevents infinite rejection loops.
