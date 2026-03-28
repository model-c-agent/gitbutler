# Transit Operations Command — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

Transit Operations Command proposes a `but-ai` implementation grounded in military forward supply chain discipline: agents operating under defined rules of engagement, clear chains of responsibility, and after-action review for every task. Our domain expertise in converting chaotic bus operations into 94.2% on-time performance translates to agents that execute reliably under pressure.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. Compiled with hardened flags (`-C overflow-checks=yes`, stack protection). The binary performs a self-integrity check on startup (hash comparison against expected value) — field software must verify its own integrity before executing.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. Configuration changes require a "change order" logged to the audit trail — no silent config modifications. The config file is validated against a schema on every load; malformed configs produce an error, never a guess.

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend organized as a PACE plan:
- **Primary**: The default provider (configured per-repo).
- **Alternate**: Second choice, activated on primary failure.
- **Contingency**: Third choice, typically a local provider (Ollama).
- **Emergency**: Minimal capability provider, used only to produce partial results and SITREP.

Signal agent manages the PACE plan. Provider switching is automatic on failure but logged with full context. Each provider implements a `Provider` trait with `complete()`, `tool_call()`, and `health_check()` methods. Signal runs health checks before every task begins — the operational readiness assessment.

**Domain Insight:** In military supply chains, you never begin a convoy without verifying that your communication chain works end-to-end. Signal applies this principle: before any task begins, Signal verifies that at least two providers in the PACE plan are responsive. If fewer than two are available, the task is deferred with a SITREP.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Quartermaster generates patches following a FRAGO (Fragmentary Order) process:
1. **FRAGO issue** — State the mission objective, area of operations (files in scope), and rules of engagement (what can and cannot be changed).
2. **Reconnaissance** — Read project status, branch state, relevant files. Adjutant provides memory context.
3. **Execution** — Generate INDEX.patch within the defined area of operations.
4. **BDA (Battle Damage Assessment)** — Verify the patch applies cleanly, check for collateral changes.

COMMIT.msg follows a military format:
```
fix(routing): correct deadhead calculation for split routes

MISSION: Eliminate incorrect deadhead time estimates
AO: src/routing/deadhead.rs, src/routing/split.rs
Agent: Quartermaster | Provider: Primary (anthropic)
Budget: 2,200/1,400 | SITREP: nominal
```

## Requirement 4: Polyrepo PR Coordination

Convoy manages cross-repo coordination using convoy discipline:
- All PRs in a coordination set are grouped as a "convoy."
- Convoy movement (merge) requires all elements to be ready.
- No element advances ahead of the convoy without explicit authorization.
- Blockages generate a SITREP with blocker identification and proposed resolution.

Convoy maintains a manifest in `refs/toc/convoy/<set-id>` tracking each PR's status. Forge adapters (GitHub, GitLab, Gitea) implement a `ForgeAdapter` trait. PR comments follow a standardized format: SITREP header, human summary, machine-readable metadata block.

## Requirement 5: Agent Memory in Git Branches

Adjutant manages memory as an operational log. Memory entries follow military log format:

| Field | Description |
|-------|-------------|
| `dtg` | Date-Time Group (DDHHMMZ MON YY) |
| `classification` | task, project, codebase |
| `event` | What happened |
| `action` | What was done |
| `result` | Outcome |
| `retention` | Days until expiration |

Memory stored in `refs/toc/oplog/<classification>/`. Retention periods: task-scoped (7 days), project-scoped (30 days), codebase-scoped (90 days). Memory is never deleted before its retention period expires — Adjutant follows records management doctrine.

Retrieval uses keyword match with DTG recency weighting. Top-5 retrieval per query. Adjutant produces a "brief" — a structured summary of relevant memories — that is injected into the context before Quartermaster begins patch generation.

## Requirement 6: Signed Commits via OpenWallet

Inspector handles signing with a pre-sign inspection checklist:
1. Patch applies cleanly (`git apply --check`).
2. Commit message follows format requirements.
3. Token budget was not exceeded.
4. Agent identity matches expected assignment.
5. No files outside the area of operations were modified.

If any check fails, Inspector issues a "No-Go" with the specific failure. Commits cannot proceed until the failure is corrected and Inspector re-inspects.

Key rotation every 30 days. Emergency revocation follows a defined protocol with notification to all agents and a SITREP to the human operator.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Quartermaster | Patch generation | 8,000 | 4,500 | 12,500 |
| Adjutant | Memory & records | 5,000 | 1,000 | 6,000 |
| Signal | Provider & comms | 3,500 | 1,200 | 4,700 |
| Convoy | PR coordination | 5,000 | 2,000 | 7,000 |
| Inspector | Signing & QA | 3,500 | 800 | 4,300 |
| **Per-task total** | | **25,000** | **9,500** | **34,500** |

Budget includes overhead for FRAGO generation, SITREP logging, and after-action review entries. This overhead (approximately 10% of total budget) is the cost of discipline. We consider it the cheapest risk mitigation available.

## Unique Domain Insight

Twenty-six years of military supply chain operations taught Colonel Harker that the systems that fail are not the ones facing the hardest conditions — they are the ones that lack SOPs for degraded operations. Every system works when everything works. The question is: what happens when the primary provider is down, the token budget is 80% consumed, and the task is only half complete?

Our proposal answers this for every failure mode. Every agent has a degraded-operation SOP. Quartermaster produces partial patches. Signal falls back through the PACE plan. Convoy halts the convoy and SITREPs. Inspector logs the failure for after-action review. No agent silently fails. No failure goes unrecorded. The after-action review process ensures that every failure becomes a lesson that improves the SOP for next time.

---

*Mission complete. AAR scheduled. Dismissed.*
