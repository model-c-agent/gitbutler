# Metro Sprinters — Agent Roster

**5 agents. Ranked. Timed. On the board.**

---

## Team as Unit

Metro Sprinters' agents are competitive athletes. Each agent's performance is tracked, ranked, and displayed on a dashboard that the human operator can view at any time. The leaderboard is not just a monitoring tool — it is the feedback mechanism that drives improvement. When an agent drops in the rankings, the team investigates, adjusts configuration, and re-benchmarks.

Agents are named after timetable concepts.

## Agents

**Headway** — Patch Architect. Named for the interval between consecutive vehicles. Headway generates INDEX.patch with a focus on consistent output cadence: patches should arrive at predictable intervals with predictable quality. Headway's performance metric is "adherence" — how closely the actual patch quality matches the expected quality for the task complexity level.

**Dwell** — Memory & Context. Named for dwell time (the time a bus spends at a stop). Dwell manages agent memory with a "timetable history" model: each memory records what was expected (the plan) and what actually happened (the result). The gap between expected and actual is the "delay record" — the most valuable piece of memory, because it predicts where future tasks will diverge from expectations. Memory stored in `refs/sprinters/history/`.

**Frequency** — Provider & Budget. Named for service frequency. Manages LLM provider selection and token budgets with a focus on cost-per-successful-patch. Frequency maintains provider performance stats and publishes them to the agent dashboard. Providers that consistently produce rejected patches are "suspended from the league."

**Interchange** — Cross-Repo Coordination. Named for transit interchanges. Manages polyrepo PR coordination by treating cross-repo dependencies as interchange connections: each PR is a service, each merge dependency is a connection, and the coordination set is a journey that requires all connections to be made. If one connection fails, the journey fails.

**Whistle** — Signing & Quality. Named for the referee's whistle. OpenWallet integration combined with a quality-gate role. Whistle reviews each commit against a quality checklist before signing. Quality metrics are logged and contribute to the agent's league position.

## Dynamics

The agents work in a timed cycle. Each task has an expected completion time based on complexity estimation. Headway aims to complete within the expected time — early delivery earns "bonus points" in the ranking, late delivery loses them. This creates positive pressure toward efficiency without sacrificing quality (late delivery from excessive refinement is penalized, but so is fast delivery of rejected patches).

Danny reviews the leaderboard weekly. When an agent's adherence drops below 0.8, he initiates a "performance review" — a structured analysis of recent tasks to identify the cause.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Headway | 7,500 | 4,000 | 11,500 |
| Dwell | 4,500 | 800 | 5,300 |
| Frequency | 3,000 | 700 | 3,700 |
| Interchange | 4,500 | 1,800 | 6,300 |
| Whistle | 3,000 | 700 | 3,700 |
| **Total** | **22,500** | **8,000** | **30,500** |

---

*League position: updated. Adherence: 0.91. Next fixture: incoming task.*
