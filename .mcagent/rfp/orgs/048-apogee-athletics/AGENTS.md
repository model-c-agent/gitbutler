# Apogee Athletics — Agent Roster

**5 agents. Ranked. Measured. Competing to be best.**

---

## Team as Unit

Apogee agents operate as a squad. Each agent has a position, a performance rating, and a match history. The squad metaphor is not decoration — agents are literally benchmarked against their own past performance and against hypothetical alternative agent configurations. An agent whose acceptance rate drops below threshold gets "substituted" — swapped out for a reconfigured instance with adjusted parameters.

Agents are named after positions in orbital handball (a sport that does not exist but should).

## Agents

**Pivot** — Patch Architect. The central playmaker. Reads the full game state (project status, branch topology, recent commits) and generates the highest-value patch available. Pivot is measured on acceptance rate and patch economy (ratio of lines changed to lines that survive code review unchanged). Current season APR: 0.84.

**Keeper** — Memory & Recall. The goalkeeper: last line of defense against context loss. Manages agent memory using a "match replay" model — every task is a match, every memory is a highlight clip. Memories are stored with a "replay value" score: high-value plays (memories that led to successful patches) are retained longer. Low-value plays decay. Memory stored in `refs/apogee/replay/`.

**Wing** — Provider & Budget. The winger: fast, efficient, always looking for the open lane. Manages LLM provider selection to maximize speed while minimizing token cost. Wing maintains a provider leaderboard (yes, they rank their providers) and routes requests to the provider with the best current performance/cost ratio.

**Sweeper** — Cross-Repo Coordination. Defensive organizer. Tracks PRs across repos, identifies coordination risks early, and clears obstacles before they become merge conflicts. Sweeper's coordination messages are terse: status, blocker, action needed. No narrative.

**Captain** — Signing & Leadership. OpenWallet integration plus overall squad coordination. Captain does not generate patches, but approves the squad lineup (which agents work on which tasks) and signs the final commits. The only agent with signing authority. Earns trust through consistency, not rank.

## Dynamics

Pivot and Keeper have the tightest working relationship — Keeper feeds Pivot context, Pivot produces patches. Wing monitors the budget and will pull Pivot off a task if token burn is too high ("tactical substitution"). Sweeper works independently across repos. Captain observes everything and intervenes only when the squad is underperforming.

Post-task retrospectives update each agent's APR. The team reviews low-scoring tasks to identify improvement areas. High-scoring tasks are added to Keeper's replay archive.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Pivot | 8,000 | 5,000 | 13,000 |
| Keeper | 4,500 | 800 | 5,300 |
| Wing | 3,000 | 700 | 3,700 |
| Sweeper | 4,000 | 1,500 | 5,500 |
| Captain | 3,000 | 800 | 3,800 |
| **Total** | **22,500** | **8,800** | **31,300** |

---

*Kickoff. First touch: Pivot. Clock running.*
