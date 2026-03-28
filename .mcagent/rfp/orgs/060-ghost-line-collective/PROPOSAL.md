# Ghost Line Collective — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

Ghost Line Collective proposes a `but-ai` implementation built on adversarial verification: every agent output is validated, every memory entry is evidence-backed, and no external data source is trusted without verification. Our domain expertise in auditing transit data for systematic deception translates to agents that cannot be fooled by their own outputs.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. Reproducible build (byte-identical output from identical source and toolchain). The binary includes a `--verify` subcommand that validates the integrity of existing agent outputs: re-checking patch applicability, memory entry evidence links, and signature chains.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. Sensitive values encrypted with `age`. The config parser rejects unknown keys — a defense against configuration injection.

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend. Bunching manages provider selection with consistency monitoring: a rolling quality variance metric that detects "bunching" patterns (clusters of good output followed by degraded periods). When bunching is detected, Bunching reduces the provider's priority until consistency is restored.

Local providers (Ollama, LMStudio) preferred for data-sensitive work. Cloud providers used only when local capacity is insufficient, with a warning logged.

**Domain Insight:** Ghost Line learned that transit agencies hide service cuts in statistical averages. A route with 90% on-time performance sounds good — until you disaggregate by time of day and discover that morning rush is 98% and evening is 60%. Bunching applies this insight to provider monitoring: aggregate quality metrics hide temporal degradation. We track quality at the per-call level and flag patterns invisible in averages.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Phantom generates patches with defensive coding as a hard requirement:
1. **Threat model** — Identify external inputs the modified code will handle. Every external input is untrusted.
2. **Generate defensively** — Patches include input validation, error paths, and boundary checks for all external interfaces.
3. **Validate** — Apply patch, verify that defensive code is present (static analysis pass for missing validation on external inputs).
4. **Submit** — If the defensive check fails, Phantom adds the missing validation before submission.

COMMIT.msg:
```
fix(gtfs): validate stop_times.txt for negative arrival_time

Defensive: Added bounds check for arrival_time < 0 in stop_times
parser. Previously, negative values passed through unchecked and
caused downstream headway calculations to produce NaN.

Agent: Phantom | Evidence: deadhead/audit-2026-0277
Ghosts: 1,247 | Validation: pass (3/3 defensive checks)
```

## Requirement 4: Polyrepo PR Coordination

Short-turn manages cross-repo coordination with equity monitoring. Coordination sets are tracked in `refs/ghostline/coord/`. In addition to standard dependency tracking, Short-turn monitors work distribution across repos:
- Which repos have received the most agent attention?
- Which repos have been neglected?
- Is the distribution proportional to the repos' maintenance needs?

If distribution is skewed, Short-turn flags it in a coordination report. This does not block work — it creates visibility, which is Ghost Line's core competency.

Forge adapters (GitHub, GitLab, Gitea) implement a minimal trait. PR comments include equity metrics alongside technical status.

## Requirement 5: Agent Memory in Git Branches

Deadhead manages memory as an evidence-backed audit trail:

| Field | Description |
|-------|-------------|
| `id` | Unique identifier |
| `claim` | What was learned or decided |
| `evidence` | Specific commits, files, or tool outputs that support the claim |
| `confidence` | verified (evidence checked), unverified (evidence not yet checked) |
| `challenge` | Any contradictory evidence |
| `ttl_days` | Verified: 30. Unverified: 7. Challenged: 3. |

Memory stored in `refs/ghostline/audit/`. The key innovation is the `evidence` field: every memory must cite its source. Memories without evidence decay rapidly. Memories with contradictory evidence ("challenges") decay even faster, forcing the agent to re-evaluate rather than rely on contested knowledge.

GC runs automatically, aggressively evicting challenged and unverified entries. Ghost Line would rather have a small, reliable memory than a large, contaminated one.

## Requirement 6: Signed Commits via OpenWallet

Validator handles signing with full-chain verification:
1. Verify patch applies and passes defensive checks.
2. Verify memory references point to valid, non-expired entries.
3. Verify provider billing matches logged token usage (no silent overcharges).
4. Verify coordination set consistency.
5. Sign only if all verifications pass.

Any verification failure produces a detailed report identifying the discrepancy. Key rotation every 21 days.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Phantom | Defensive patches | 8,500 | 4,500 | 13,000 |
| Deadhead | Evidence-backed memory | 5,000 | 800 | 5,800 |
| Bunching | Provider consistency | 3,000 | 700 | 3,700 |
| Short-turn | Equity-aware coordination | 4,500 | 1,800 | 6,300 |
| Validator | Full-chain signing | 3,500 | 600 | 4,100 |
| **Per-task total** | | **24,500** | **8,400** | **32,900** |

Approximately 12% of the budget is consumed by verification overhead (defensive checks, evidence validation, chain verification). This is the cost of operating in an adversarial model. We consider unverified agent output to be the more expensive alternative — it just charges the cost later, in debugging time and trust erosion.

## Unique Domain Insight

Four years of auditing transit data taught us that the most dangerous data is the data that looks correct. A GTFS feed with a phantom bus run is not obviously broken — the feed validates, the schedule renders, the app shows a bus coming. The deception is subtle: the bus never comes, but the data says it will. Riders learn to distrust the system, and distrust compounds.

Our proposal treats agent output with the same skepticism we apply to transit data. Memory entries without evidence are unverified. Patches without defensive coding are incomplete. Provider quality without temporal analysis is misleading. We audit our own agents the way we audit transit agencies: assume the data might be wrong, and build systems that catch it when it is.

---

*Ghosts: 1,247. Evidence: verified. The bus is not coming. But we are.*
