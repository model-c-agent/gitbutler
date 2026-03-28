# Fraud Bowl Champions -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Date:** 2026-03-28
**From:** Coach Lin, Team Captain

---

## Summary

We are competitive fraud detectors. Our `but-ai` proposal is optimized for rapid, high-accuracy analysis under time pressure. The plugin produces findings fast, validates them faster, and ships them before the clock runs out. Our unique contribution: a pattern library that grows with every competition and every real-world deployment, turning experience into reusable detection capability.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Per-task invocation.

- Binary: Rust, statically linked
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml`
- Cold start: under 80ms -- competition conditions demand instant readiness

The plugin is designed for burst workloads: many short tasks in rapid succession, not one long task. This matches competition format and most real-world fraud screening workflows.

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`.

| Provider | Usage | Notes |
|----------|-------|-------|
| Anthropic | Complex scheme analysis | Best reasoning for multi-entity fraud |
| OpenAI | Statistical pre-screening | Fast for bulk anomaly detection |
| Ollama | Competition (offline) | No network dependency during events |
| LMStudio | Development | Local testing |

Competition environments sometimes have unreliable network. Ollama ensures the pipeline works offline.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Finding-Per-Commit

Each commit adds one finding:

```
finding: round-number structuring in vendor payments

27 payments to "Global Supplies Ltd" between $9,900 and $9,999.
Period: 2024-Q1 through 2024-Q3.
Pattern: structuring below BSA reporting threshold.

Competition: FB-2026
Round: 2
Time-remaining: 01:47:33
Agent: lin
Confidence: 0.96
False-positive-risk: LOW
```

The `Time-remaining:` trailer records temporal context. Findings produced early in a round have more review time than findings produced late. This metadata informs the review priority.

---

## 4. Polyrepo PR Coordination

Minimal. Competition is single-dataset. Real-world deployments may coordinate with client repos.

### Schema

```json
{
  "team": "fraud-bowl-champions",
  "action": "finding_delivered",
  "finding_type": "structuring",
  "confidence": 0.96,
  "branch": "findings/structuring-vendor-payments"
}
```

Forge support: GitHub, Forgejo. Lightweight because coordination is not our bottleneck -- analysis is.

---

## 5. Agent Memory in Git Branches

### Pattern Library

Memory stored in `refs/fbc/memory/patterns/<category>/<key>`.

| Field | Description |
|-------|-------------|
| `key` | Pattern identifier |
| `signature` | Statistical fingerprint (mean, variance, Benford deviation) |
| `scheme_type` | Fraud classification per ACFE taxonomy |
| `detection_method` | How to find it (test name, threshold, parameters) |
| `competitions_seen` | Count of competitions where this pattern appeared |
| `last_seen` | Timestamp |
| `ttl` | Never expires (patterns are permanent knowledge) |

### Retrieval

At task start, retrieve top-5 patterns most similar to the current dataset's statistical profile. Inject pattern signatures and detection methods into agent context.

### Growth

After every competition and every real-world case, new patterns are added to the library. The library is the team's competitive advantage -- it encodes 5 years of competition experience as reusable detection templates.

---

## 6. Signed Commits via OpenWallet

Devon manages signing.

- Keys provisioned per event/engagement via OpenWallet
- Competition keys: ephemeral, expire at event end
- Real-world keys: 72-hour rotation
- Revocation: immediate, stored in `refs/fbc/revoked`

---

## 7. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Coach Lin | Strategy/findings | 7,200 | 3,800 | 11,000 |
| Rajan | Speed analysis | 5,000 | 1,200 | 6,200 |
| Amara | Complex patterns | 5,800 | 2,500 | 8,300 |
| Devon | Tooling/signing | 3,500 | 800 | 4,300 |
| **Total** | | **21,500** | **8,300** | **29,800** |

---

## 8. Unique Insight: The Reusable Pattern Library

Most agent memory is ephemeral: relevant to one task, expired after. We maintain a **permanent pattern library** that encodes fraud detection knowledge as structured, searchable, reusable templates.

Each pattern entry contains not just the finding but the method: which statistical test, which threshold, which data characteristics make the pattern likely. When an agent encounters a new dataset, it retrieves the most relevant patterns and applies their detection methods before attempting any novel analysis.

This turns every past investigation into a training dataset for future ones. The library currently contains 247 patterns from 30 competitions and 14 real-world cases. It grows with every engagement. The more we compete, the better the library gets. The better the library, the faster we compete.

Traditional fraud detection relies on individual expertise that walks out the door when the expert retires. A pattern library persists. It is institutional memory that does not forget, does not retire, and does not get food poisoning before the finals.

---

*"14 out of 15. The pattern library will find the 15th."*
