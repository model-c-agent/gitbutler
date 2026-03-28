# Benchwarmer Analytics FC — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation built on **sabermetric agent management** — every agent is statistically evaluated, every run produces a measurable stat line, and underperforming components are replaced without sentiment. Our system treats agent operations like a baseball front office treats player management: draft the best available, measure relentlessly, trade or bench when the numbers say so.

---

## Requirement 1: PATH-based Plugin Architecture

`but-ai` ships as a single binary installed to a PATH-discoverable location. Follows the `but <plugin> <command>` convention.

**Design:**
- Binary name: `but-ai`
- Discovery: `but` scans PATH for `but-ai`, validates via `but-ai --handshake`
- Commands: `but ai patch`, `but ai stats`, `but ai memory`, `but ai roster`
- Config: `~/.config/but-ai/lineup.toml` — we call the config file the "lineup card"
- Zero runtime dependencies; statically linked

**The `roster` command** is unique to our proposal: `but ai roster` prints a live table showing each agent's current stats (success rate, avg tokens, avg latency) like a baseball roster card.

---

## Requirement 2: Provider-Agnostic AI

Providers are treated as **free agents** in a market. The system maintains performance stats for each provider and routes tasks based on current form.

**Architecture:**
- Provider trait: `Provider { fn complete(&self, task: Task, budget: Budget) -> Result<Completion> }`
- Provider stats tracked in `refs/benchwarmer/stats/providers/<name>`
- A/B routing: configurable percentage of tasks sent to secondary provider for comparison
- Automatic fallback: if primary provider's error rate exceeds threshold, secondary is promoted
- Supported: OpenAI, Anthropic, Ollama, LMStudio

**Provider "batting average":** After every call, the provider's success rate is updated. A provider below .200 (20% success) for 50 consecutive calls is automatically demoted to backup.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce INDEX.patch and COMMIT.msg exclusively. No direct file modification.

**Agent lifecycle:**
1. Task assignment — agent receives task description and context budget
2. Context read — agent reads relevant files within token budget
3. Patch generation — agent produces unified diff
4. Validation — patch is applied to a scratch worktree and tests run
5. Commit — if validation passes, patch and message are committed

**Stat line per run:** Every agent run records: tokens_in, tokens_out, patch_lines, files_touched, test_pass (bool), latency_ms. These stats are appended to `refs/benchwarmer/stats/agents/<name>`.

**Bench protocol:** If an agent's rolling 10-run success rate drops below 60%, it is "benched" — its system prompt is regenerated from the team's best-performing agent's template, with domain-specific adjustments.

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination uses a **sign-stealing protocol** (the legal kind — encoded signals between cooperating agents).

**Signal schema:** PR comments contain structured signals:
```
<!-- ba:fc:signal:{type}:{payload_json} -->
```

**Signal types:**
- `ready` — This repo's changes are complete and tested
- `waiting` — Blocked on changes in another repo
- `abort` — Cross-repo change is abandoned
- `merge` — All repos report ready; proceed with merge

**Forge abstraction:** Adapter trait for GitHub, GitLab, Gitea. Each adapter implements `post_signal`, `read_signals`, `list_open_prs`. Forge detection is automatic based on remote URL.

**Dependency graph:** `.but-ai/dependencies.toml` in each repo lists cross-repo dependencies with expected branch names and signal endpoints.

---

## Requirement 5: Agent Memory in Git Branches

Memory is organized as **scouting reports** — structured documents describing observed codebase patterns.

**Memory tiers:**
- `refs/but-ai/memory/game/<run-id>` — Single-run context, expires after run
- `refs/but-ai/memory/season/<task-id>` — Task-scoped, expires after 7 days
- `refs/but-ai/memory/career/<pattern>` — Long-lived patterns, expires after 90 days
- `refs/but-ai/memory/hall-of-fame/<key>` — Permanent, manually curated

**Scouting report format:**
```toml
[report]
pattern = "error-handling-style"
domain = "api-layer"
summary = "This codebase uses Result<T, AppError> with From impls, never unwrap"
confidence = 0.92
observed_in = ["src/api/mod.rs", "src/api/handlers.rs"]
last_seen = "2026-03-28"
```

**Retrieval:** Top-K similarity search using TF-IDF over report summaries. Max 5 reports injected per run.

---

## Requirement 6: Signed Commits via OpenWallet

Every commit is signed. No exceptions. Unsigned commits are rejected by the system.

**Key management:**
- Key generation: automated at agent first-run
- Rotation: every 14 days or 200 commits
- Revocation: immediate on anomaly detection (signing pattern deviation)
- Storage: OpenWallet credential store, never in the Git repo

**Signing flow:** Agent produces patch -> patch hash computed -> signature requested from OpenWallet -> signed commit created via `but commit --sign` -> signature recorded in attestation ref.

**Audit trail:** `refs/but-ai/audit/signatures/<date>` contains a daily log of all signing events with agent identity, key fingerprint, and commit SHA.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Maya | Patch generation | 9,000 | 4,200 | 13,200 |
| Jerome | Budget management | 3,500 | 800 | 4,300 |
| Priya | Provider routing | 5,500 | 2,000 | 7,500 |
| Dante | Memory retrieval | 6,000 | 700 | 6,700 |
| Soo-jin | Forge coordination | 5,800 | 2,200 | 8,000 |
| Tomás | Signing | 3,000 | 600 | 3,600 |
| **Total** | | **32,800** | **10,500** | **43,300** |

### Scaling Table

| Task Type | Multiplier | Budget |
|-----------|-----------|--------|
| Relief appearance (hotfix, <30 lines) | 0.4x | 17,320 |
| Standard at-bat (feature, ~200 lines) | 1.0x | 43,300 |
| Extra innings (complex, multi-repo) | 2.0x | 86,600 |
| Postseason (architecture, breaking changes) | 2.5x | 108,250 |

---

## Unique Insight: Agent WAR (Wins Above Replacement)

We propose tracking **Agent WAR** — a composite metric measuring an agent's value relative to a baseline "replacement-level" agent (a zero-shot prompt with no memory and no context beyond the immediate task).

WAR = (agent_success_rate - baseline_success_rate) * tasks_completed - (agent_token_cost - baseline_token_cost) / cost_normalizer

This gives a single number answering: "Is this agent worth its cost?" An agent with WAR < 0 is literally worse than the baseline and should be replaced. An agent with WAR > 5 is elite and its configuration should be studied and replicated.

We will publish WAR leaderboards per provider, per task type, and per codebase. This data benefits the entire `but-ai` ecosystem, not just our team.

Every front office needs a stat that cuts through the noise. WAR is that stat.

---

*"Trust the numbers."*
