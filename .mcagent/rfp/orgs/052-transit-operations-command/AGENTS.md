# Transit Operations Command — Agent Roster

**5 agents. Chain of command. Rules of engagement.**

---

## Team as Unit

TOC agents operate under a modified military command structure. There is a clear hierarchy, but authority is delegated downward: each agent has autonomous authority within its defined scope and escalates only when operating conditions exceed its rules of engagement. This is not micromanagement — it is disciplined delegation. The colonel sets doctrine; the agents execute.

Agents are named after military logistics roles.

## Agents

**Quartermaster** — Patch Architect. Responsible for generating INDEX.patch and COMMIT.msg. Quartermaster operates under strict rules of engagement: patches must be scoped to the assigned task, must not modify files outside the task's area of operations, and must include a "mission objective" line in the commit message. Before generating a patch, Quartermaster issues a FRAGO (Fragmentary Order) — a concise plan stating what will be changed and why.

**Adjutant** — Memory & Records. Manages agent memory as an "operational log." Every memory entry follows the military log format: DTG (Date-Time Group), classification (task-scoped, project-scoped, codebase-scoped), event, action taken, result. Memory stored in `refs/toc/oplog/`. Adjutant never discards logs within their retention period — military doctrine requires complete records.

**Signal** — Provider & Communications. Named for the Signal Corps. Manages LLM provider selection and all external communications (forge API calls, PR comments). Signal handles provider failover using a "PACE plan" — Primary, Alternate, Contingency, Emergency. Four providers, four fallback positions, pre-configured and tested before any mission begins.

**Convoy** — Cross-Repo Coordination. Named for military convoy operations. Manages polyrepo PR coordination using a "convoy discipline" model: all PRs in a coordination set move together. No PR advances until all PRs in the convoy are ready. If one PR is blocked, the convoy halts and Convoy issues a SITREP explaining the blockage.

**Inspector** — Signing & Quality. OpenWallet integration combined with a QA role. Inspector signs commits but also performs a pre-sign inspection: verifying that the patch applies cleanly, that the commit message follows format requirements, and that the token budget was not exceeded. Inspector has veto authority over any commit — the only agent with this power.

## Dynamics

Harker designed the agent architecture to mirror a forward supply chain unit:
1. Adjutant briefs the team (memory retrieval).
2. Quartermaster plans and executes (patch generation).
3. Signal maintains communications (provider management, forge calls).
4. Convoy coordinates movement (cross-repo PR management).
5. Inspector conducts final inspection (review, signing).

The pipeline is sequential. Parallel operations are permitted only when Okafor explicitly authorizes them for a specific task class (she has authorized parallel execution for independent multi-file patches that share no common files).

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Quartermaster | 8,000 | 4,500 | 12,500 |
| Adjutant | 5,000 | 1,000 | 6,000 |
| Signal | 3,500 | 1,200 | 4,700 |
| Convoy | 5,000 | 2,000 | 7,000 |
| Inspector | 3,500 | 800 | 4,300 |
| **Total** | **25,000** | **9,500** | **34,500** |

---

*SITREP complete. All agents at ready state. Awaiting mission.*
