# PastStack — Agent Roster

**6 agents. Startup flat structure. Optimized for discovery rate.**

---

## Team Composition

The team is structured like a high-velocity startup engineering org: flat, fast, and focused on throughput. Every agent can ship independently. Cross-agent dependencies are minimized by design — each agent owns its domain end-to-end.

### Roles

- **Product Agent** — Reads incoming tasks, decomposes them, prioritizes by impact. Produces clear specs with acceptance criteria. Does not write code. Talks to the forge (PR management, cross-repo coordination).
- **2 Backend Agents** — Core implementers. One focuses on plugin architecture and CLI scaffolding. The other focuses on agent execution logic and patch generation. Both produce `INDEX.patch` + `COMMIT.msg`.
- **ML Agent** — Owns provider abstraction. Manages the `but-llm` integration, provider capability detection, and model selection. Also handles token budget tracking and optimization.
- **DevOps Agent** — Manages OpenWallet key lifecycle, commit signing, build system, and WASI compatibility testing. The "keep the lights on" agent.
- **Data Agent** — Owns memory architecture, relevance scoring, and context management. Responsible for ensuring agents have the right data at the right time without blowing the token budget.

## Working Dynamic

Work is pulled, not pushed. The product agent maintains a prioritized backlog. Agents self-assign based on their domain and current availability. No standups. No sprint planning. The only synchronization point is the weekly retro, where agents review what shipped, what broke, and what to do differently.

Code review is lightweight: the implementing agent's patch is reviewed by whichever agent is available. Reviews focus on correctness and budget compliance, not style — PastStack trusts its agents to write clean code.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Product Agent | 2,500 | 800 |
| Backend Agents (x2) | 4,500 each | 3,800 each |
| ML Agent | 3,500 | 1,200 |
| DevOps Agent | 2,500 | 600 |
| Data Agent | 3,500 | 800 |
| **Team Total** | **25,500** | **11,000** |

## Failure Mode

The team fails by shipping too fast and fixing too slow. When two backend agents independently ship patches that pass their own tests but interact poorly (e.g., both modify the same config struct with incompatible additions), the integration failure is only caught at build time. Recovery: the DevOps agent runs a continuous integration check on all branches after every commit, catching integration issues within minutes.
