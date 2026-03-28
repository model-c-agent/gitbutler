# Ore & Light — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation built on the principle that **nothing is waste**. Failed runs are archived and analyzed. Rejected patches are studied for patterns. Expired memories are preserved in fossil layers. Our system treats every agent artifact as raw material for improvement, the same way our commune treats mine tailings as raw material for art and remediation.

---

## Requirement 1: PATH-based Plugin Architecture

Clean binary. Minimal surface. Functional beauty.

**Design:**
- Binary: `but-ai`, statically linked
- Commands: `but ai patch`, `but ai memory`, `but ai archive` (view archived failed runs), `but ai trend` (show pattern trends over time)
- Config: `~/.config/but-ai/ol.toml`
- `but ai archive` provides access to failed runs and rejected patches — not for recovery, but for pattern analysis
- `but ai trend <pattern>` shows how a codebase pattern has changed over time, using memory history

---

## Requirement 2: Provider-Agnostic AI

Simple, clean provider abstraction. No over-engineering.

**Architecture:**
- Provider trait: invoke/stream interface
- Three-day implementation rule: if the abstraction takes more than three days to build, it is too complex
- Supported: OpenAI, Anthropic, Ollama, LMStudio
- Selection: config-driven, single active provider, manual failover
- Provider response archival: all responses cached not just for replay but for trend analysis (how does provider quality change over time?)

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Patches that are **correct, clean, and readable** — in that order.

**Workflow:**
1. Read task and context
2. Retrieve memory (strata-based)
3. Generate INDEX.patch + COMMIT.msg
4. Structural validation: architecture constraints, dependency checks
5. If passing: commit
6. If failing: archive the failed attempt, analyze failure pattern, retry

**Failed attempt archival:** Failed patches are stored in `refs/ol/archive/failed/<run-id>` with the failure reason. These archives are periodically analyzed for patterns: what kinds of failures recur? What codebase contexts produce the most failures?

**COMMIT.msg format:** Conventional Commits with a `Stratum:` trailer indicating which memory layer was most influential:
```
feat: add retry logic to API client

Stratum: bedrock (pattern: retry-with-backoff observed in 7 modules)
```

---

## Requirement 4: Polyrepo PR Coordination

Coordination with **trend-aware dependency management**.

**Protocol:**
- PR comments: `<!-- ol:coord:{action}:{payload} -->`
- Actions: `propose`, `ack`, `ready`, `merge`
- Each cross-repo PR includes a trend note: how does this change relate to the project's trajectory?

**Forge adapters:** GitHub, GitLab, Gitea. Standard trait.

**Trend integration:** The coordination system tracks cross-repo change frequency. If two repos change together frequently, the system suggests consolidation or shared interface extraction.

---

## Requirement 5: Agent Memory in Git Branches

Memory organized as **geological strata** — layers of knowledge at different depths and ages.

**Strata:**
- `refs/but-ai/memory/surface/<key>` — Recent observations. TTL: 7 days. High churn.
- `refs/but-ai/memory/bedrock/<key>` — Established patterns. TTL: 90 days. Stable.
- `refs/but-ai/memory/fossil/<key>` — Historical patterns. No expiration. Read-only archive.

**Promotion and fossilization:**
- Surface entries observed 5+ times are promoted to bedrock
- Bedrock entries that expire are fossilized (moved to fossil layer) rather than deleted
- Fossil entries are never injected into context automatically but can be queried on demand

**Trend monitoring:** Bedrock entries include a trend field tracking how the pattern has changed over time:
```toml
[entry]
key = "error-handling-style"
stratum = "bedrock"
current = "Result<T, AppError> with From impls"
trend = [
  { date = "2026-01", observation = "mix of unwrap and Result" },
  { date = "2026-02", observation = "mostly Result<T, AppError>" },
  { date = "2026-03", observation = "consistently Result<T, AppError>" }
]
confidence = 0.94
```

---

## Requirement 6: Signed Commits via OpenWallet

Standard signing. Clean implementation.

**Design:**
- OpenWallet-managed keys, per-commit signing
- Key rotation: 30 days
- Revocation: immediate on compromise
- VC includes: agent identity, stratum of primary memory influence, archive references (if this patch replaces a previously failed attempt)

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Callista | Aesthetics | 2,500 | 800 | 3,300 |
| Dean | Structure | 5,500 | 1,500 | 7,000 |
| Hazel | Patch/Provider | 9,500 | 4,500 | 14,000 |
| Joaquin | Memory/Coord | 6,000 | 2,000 | 8,000 |
| **Total** | | **23,500** | **8,800** | **32,300** |

### Scaling

| Stratum | Description | Budget |
|---------|-------------|--------|
| Surface (minor) | Quick fix, <30 lines | 12,920 (0.4x) |
| Bedrock (standard) | Feature, ~200 lines | 32,300 (1.0x) |
| Deep vein (multi-repo) | Cross-repo change | 64,600 (2.0x) |
| Core sample (architecture) | Major restructuring | 80,750 (2.5x) |

---

## Unique Insight: Failure Archival as Pattern Mining

Every agent system deletes failed runs. Ours archives them.

When a patch fails — rejected by validation, failed by tests, or returned by review — it is stored in a failure archive with the context that produced it, the memory that influenced it, and the reason it failed. Over time, the failure archive becomes a dataset: what kinds of tasks produce failures? What memory patterns correlate with bad output? What provider/model combinations fail on which task types?

We mine this dataset the way we mine tailings: extracting value from what others discard. A pattern in the failure archive — "patches that touch the authentication module fail 40% of the time when the agent lacks the security-conventions memory entry" — is actionable intelligence that improves future runs.

The mining industry spent a century discarding tailings as waste. We turned them into art and remediation. The AI agent industry discards failed runs as noise. We turn them into signal.

Nothing is waste.

---

*Submitted by Ore & Light, Bisbee, Arizona.*
*"From the tailings, light."*
