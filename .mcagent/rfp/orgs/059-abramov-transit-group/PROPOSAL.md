# Abramov Transit Group — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

Abramov Transit Group proposes a `but-ai` implementation built on the reliability principles of a 64-year family bus operation: keep promises, maintain records, and never leave a passenger stranded. Our four-agent architecture prioritizes predictable output, human-readable records, and conservative execution over sophistication.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. The binary prioritizes stability: it is tested against a suite of regression scenarios before every release. New features are gated behind `--experimental` flags until they survive one full release cycle without incident.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. Defaults are conservative. The config file is documented with examples for every field. Natasha's rule: "If Viktor cannot read the config comments and understand what each setting does, the documentation is insufficient."

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend. Garage manages provider selection with a stability bias: the default provider is used until it fails. No dynamic routing. No performance-based switching. The family prefers knowing which provider is running, the same way Pavel prefers knowing which driver is on which route.

Provider fallback is a simple ordered list. When the primary fails, the next in the list activates. Garage logs every switch. Token counting is conservative: round up, never down. Budget exhaustion produces a partial result with a clear explanation.

**Domain Insight:** In bus operations, the worst thing you can do is change the schedule without telling the riders. Garage applies this: provider switches are logged and visible. If the agent changes behavior because the provider changed, the human operator should be able to trace the behavioral change to the provider switch in the log.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Dispatch generates patches using a "proven pattern" approach:
1. **Check the logbook** — Logbook retrieves memories of similar past tasks.
2. **Apply the pattern** — Dispatch generates INDEX.patch by adapting a proven pattern from memory, not by generating from scratch.
3. **Verify** — `git apply --check`. If it fails, Dispatch falls back to fresh generation.
4. **Submit** — COMMIT.msg in plain language that Pavel can read.

COMMIT.msg:
```
fix: schedule rollover at midnight resets wrong route table

The midnight schedule swap was referencing the weekday table
instead of checking the day-of-week flag. Fixed to check
current day before selecting the schedule table.

Agent: Dispatch | Fleet: 42/42 | Tokens: 1,600/1,100
Pattern: logbook/schedule-swap-2025-0088
```

The commit message explains what happened in plain English. No jargon. No abbreviations. Pavel reads these.

## Requirement 4: Polyrepo PR Coordination

Garage handles cross-repo coordination with a "fleet manifest" model. Each coordination set is a fleet of related changes. The manifest lists every PR, its repo, its status, and its dependencies. Manifests are stored in `refs/abramov/fleet/`.

Garage coordinates in plain language: PR comments describe what is waiting for what in complete sentences. No structured metadata blocks. No machine-readable formats. The Abramov team values human readability above automation efficiency.

Forge adapter: GitHub only (the family uses GitHub). Other forge support is a documented future item.

## Requirement 5: Agent Memory in Git Branches

Logbook manages memory as a maintenance logbook. Each entry:

| Field | Description |
|-------|-------------|
| `entry_number` | Sequential, never reused |
| `date` | ISO 8601 |
| `asset` | File, module, or repo affected |
| `event` | What was learned or decided |
| `resolution` | What was done |
| `cross_ref` | Related logbook entries |
| `ttl_days` | 30 default, +14 on reuse (max 90) |

Memory stored in `refs/abramov/logbook/` as plain text files. Retrieval is keyword search with recency weighting and asset matching (memories about the same file rank higher). No embeddings.

The logbook is designed to be readable by a human using `git show`. Viktor reads the physical bus logbooks every Saturday. Natasha wants the digital logbook to be equally legible.

## Requirement 6: Signed Commits via OpenWallet

Seal handles signing with minimal overhead. Sign, verify, log. Key rotation every 60 days. No complex ceremony. The inspection seal on a bus is a simple sticker that says "passed." Seal's signature serves the same purpose: this commit was produced by an authorized agent and it passed the checks.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Dispatch | Patch generation | 7,000 | 3,500 | 10,500 |
| Logbook | Memory & records | 4,000 | 600 | 4,600 |
| Garage | Provider, budget, coordination | 5,500 | 2,000 | 7,500 |
| Seal | Signing | 2,000 | 400 | 2,400 |
| **Per-task total** | | **18,500** | **6,500** | **25,000** |

Four agents. Lean budget. No waste. The Abramov way.

## Unique Domain Insight

Sixty-four years of running buses taught the Abramov family that the most important quality in a system is not performance — it is legibility. A dispatch system that only Natasha understands is a dispatch system that fails when Natasha is sick. A codebase that only the original author understands is a codebase that rots when they leave.

Our proposal prioritizes legibility at every level. Commit messages are written in plain English. Memory entries are plain text files. Configuration is documented for non-technical readers. Agent behavior is predictable and sequential. This is not the most sophisticated proposal in this RFP. It is the most likely to be maintained by the next person. In a family business, you always build for the next person.

---

*Fleet: 42/42. Grandpa is in the shop. The buses are running.*
