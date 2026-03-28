# Driftwood & Canvas — Agent Roster

**6 agents. Studio structure. Critique-driven quality assurance.**

---

## Team Composition

The team runs like a design studio. Work is proposed, prototyped, critiqued, and refined. No agent ships output without at least one other agent reviewing it for clarity and correctness. The emphasis is on output quality over output speed.

### Roles

- **2 Designer Agents** — Own all human-facing output: commit message formatting, PR comment structure, error message wording, progress report layout. They do not generate patches directly but format and polish the output of other agents. Think of them as the team's copyeditors.
- **2 Engineer Agents** — Handle plugin architecture, provider abstraction, tool integration, and patch generation. They produce the technical artifacts; the designers make them legible.
- **1 Research Agent** — Manages memory architecture, relevance scoring, and context optimization. Also responsible for the agent's self-knowledge: understanding what it knows, what it has forgotten, and what it should look up.
- **1 Studio Manager Agent** — Coordinates work assignments, tracks token budgets across the team, and manages the critique cycle schedule. Does not produce technical output but ensures the pipeline flows.

## Working Dynamic

Every task follows the studio cycle:

1. **Brief** — Studio manager decomposes the task and assigns it
2. **Draft** — Engineers and researcher produce technical artifacts
3. **Crit** — Designers review all output for clarity and formatting
4. **Revision** — Engineers incorporate feedback
5. **Ship** — Studio manager signs off and the patch is committed

This adds one round-trip compared to a direct-to-commit workflow, but the team argues the output quality justifies the cost.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Designers (x2) | 2,500 each | 1,500 each |
| Engineers (x2) | 5,000 each | 3,500 each |
| Researcher | 4,000 | 1,000 |
| Studio Manager | 2,000 | 500 |
| **Team Total** | **26,000** | **11,500** |

## Failure Mode

The team fails by over-polishing. When a deadline is tight and the designers insist on one more formatting pass, the team can burn 15-20% of its output budget on cosmetic improvements. Recovery: the studio manager can invoke "rough cut" mode, which bypasses the crit cycle and ships engineer output directly. The designers hate it, but it works.
