# Ranger League Proposal — `but-ai` Plugin

**Submitted by:** Ranger League
**Date:** 2026-03-28

---

## Governing Principle

Fairness is the non-negotiable. Every agent output must be impartial, auditable, and reproducible. If you cannot replay the scoring, the score does not count.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary. CLI mode: `but ai score` (apply algorithm), `but ai validate` (verify data), `but ai standings` (compute rankings), `but ai replay` (re-execute scoring for audit). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `FairnessCheck` (randomized-label score verification).

WASI: Replay mode only. Auditors can re-execute scoring in a sandboxed WASI environment without write access — ideal for dispute resolution.

Config: `[but-ai]` in Git config. Scoring rules at `refs/league/rules/<championship-year>`.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` only. Provider via `gitbutler.aiModelProvider`. The League adds a reproducibility requirement: scoring must produce identical results regardless of provider. This is achieved by using structured output (`but-llm`'s `structured_output` method) with a strict JSON schema that eliminates provider-dependent formatting.

Provider fallback order configured per-championship. If the primary provider is unavailable, the system falls to the next in the list. All fallbacks are verified for scoring consistency before championship day.

---

## 3.3 The But Agent

Scoring pipeline:

1. **Ingest:** Teams submit data to their branches
2. **Validate** (Nguyen): Cross-reference against species databases. Flag or provisionally accept.
3. **Score** (Kimathi): Apply algorithm. Produce INDEX.patch with scored results. Include fairness check.
4. **Record** (Torres): Update historical database with new results.
5. **Announce** (Abdi): Publish standings via PR comments.

Patches only. Branch naming: `league/<championship>/<team>/<discipline>` — e.g., `league/2026-wetland/team-serengeti/census`. Final standings merge to `league/<championship>/standings`.

Budget: allocated per scoring round. Kimathi (scoring) gets 40%, Nguyen (validation) 25%, Torres (records) 15%, Abdi (comms) 15%, Oduya (oversight) 5%.

---

## 3.4 Polyrepo PR-Based Coordination

24 team repos + 1 central scoring repo. Abdi coordinates all cross-repo communication.

```json
{
  "league_schema": "1.0",
  "type": "submission|score|dispute|standings",
  "championship": "2026-wetland",
  "team": "team-serengeti",
  "discipline": "census",
  "match_clock": "M23",
  "body": "...",
  "fairness_check_passed": true
}
```

Every `score` message includes a `fairness_check_passed` flag. Scores without the flag are not included in standings.

Forge adapter: trait with `submit` (team PR), `score_report` (comment), `standings_board` (list by championship), `certify` (final merge). GitHub implementation.

Cross-repo: Team repos submit to the central scoring repo via PRs. Dependencies are one-directional: scoring depends on submissions, not the reverse.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/league/memory/<championship>/<discipline>/`. Organized by championship and discipline — each championship is a self-contained season.

**Structure:**
```json
{
  "key": "2026-wetland-census-baseline",
  "championship": "2026-wetland",
  "discipline": "census",
  "value": "Baseline: 12 target species, 340 known individuals in survey area",
  "biome": "temperate-wetland",
  "ttl": "365d",
  "tags": ["baseline", "census", "wetland"]
}
```

**Relevance scoring:** Championship + discipline match is primary. Biome match provides a 1.5x boost (wetland championships can draw on previous wetland data). Historical baselines from the same biome are always retrieved. Maximum 5 entries.

**Expiration:** Championship-season data expires after 1 year. Historical baselines tagged `precedent` persist indefinitely for trend analysis.

**Compaction survival:** Current-championship entries always survive. Previous championships survive if biome matches. Unrelated championships are dropped.

**Identity:** At `refs/league/identity/<agent>`:
```json
{
  "name": "Kimathi",
  "role": "chief-scorer",
  "certifications": ["species-id-level-3", "gps-verification"],
  "signing_key_id": "league:kimathi:2026"
}
```

**Unique insight:** Biome-indexed historical memory. Conservation data is biome-specific — what you know about savannas is irrelevant in wetlands, and vice versa. The League's memory system uses biome as a primary index, ensuring agents draw on relevant ecological context rather than superficially similar but ecologically irrelevant history.

---

## 3.6 Signed Commits via OpenWallet

Every scoring commit signed. Championship results require dual signature: Kimathi (scorer) + Oduya (commissioner).

**Authorization:**
```json
{
  "Kimathi": { "allow": ["league/*/scoring/*"], "deny": ["main", "league/*/standings"] },
  "Nguyen": { "allow": ["league/*/validation/*"], "deny": ["league/*/scoring/*"] },
  "Torres": { "allow": ["refs/league/memory/*"], "deny": ["league/*/scoring/*"] },
  "Abdi": { "allow": ["league/*/announcements/*"], "deny": ["league/*/scoring/*"] },
  "Oduya": { "allow": ["*"], "role": "commissioner" }
}
```

No agent except Oduya can modify scoring results after they are committed. This is the anti-tampering guarantee.

**Key lifecycle:**
- Provisioning: at League registration
- Rotation: annual, before championship season
- Compromise: championship results signed by the compromised key are invalidated and re-scored by a backup scorer. This has never happened.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,300 | 0 | Once/session | Identity, tools, championship rules |
| Data ingestion | 2,500 | 300 | Once/task | Team submission parsing |
| Validation | 2,000 | 1,000 | Once/task | Species cross-reference |
| Scoring | 2,500 | 3,000 | Once/task | Algorithm application + fairness check |
| Tool call (per call) | 1,000 | 500 | 3/task avg | Branch ops |
| Commit message | 400 | 300 | Once/task | With match clock |
| Memory retrieval | 1,500 | 200 | 2/task avg | Biome-indexed lookup |
| Standings update | 1,000 | 600 | Once/round | Aggregate + announce |
| Coordination | 1,500 | 600 | 1/task avg | Cross-team PR |
| **TOTAL (typical)** | **18,200** | **8,000** | -- | Single team, single discipline |

---

## Testing Strategy

1. **Fairness:** Run scoring with randomized team labels. Verify identical scores.
2. **Replay:** Re-execute historical championship scoring. Verify exact match.
3. **Validation:** Submit known-invalid species IDs. Verify rejection.
4. **Cross-repo:** 3 mock team repos submitting to central scoring repo.
5. **Dual signature:** Verify standings commit fails without commissioner countersignature.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 45000 | Per-round budget |
| `but-ai.agent.memoryRef` | `refs/league/memory` | Memory prefix |
| `but-ai.agent.fairnessCheck` | true | Require randomized-label verification |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.replayEnabled` | true | Allow WASI-mode replay |
| `but-ai.agent.matchClockEnabled` | true | Include match time in commits |

---

*"The best conservationists deserve to know they are the best."*
