# Benedictine Agribotics Fellowship -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Submitted by:** The Agribotics Chapter, Abbaye de Saint-Remy
**Date:** 2026-03-28

---

## Preamble

We are monks who tend a vineyard with robots. We respond to this RFP because we need software tools that respect our constraints: local-only operation, limited hardware, bilingual workflows, and a daily rhythm that reserves time for prayer and silence. We offer, in return, a perspective shaped by 800 years of monastic agriculture and four years of autonomous viticulture.

---

## Requirement 1: PATH-Based Plugin Architecture

Single static binary. No daemon, no background processes. The abbey's server runs on a UPS with 2 hours of battery; every background process is a battery drain.

Configuration in TOML. The config file includes a `locale` setting (default: `fr`) that affects the language of user-facing messages, review verdicts, and commit message templates. The Fellowship works in French. The code is in English. The plugin should not force a choice.

Startup: cold only, <500ms. The plugin launches, executes, and exits. No persistent state outside Git refs.

---

## Requirement 2: Provider-Agnostic AI

Two providers in practice: Ollama (primary) and LMStudio (backup). The Fellowship does not use cloud providers. The plugin's provider abstraction must treat local providers as full citizens, not degraded fallbacks.

Trait: `init`, `complete`, `complete_with_tools`, `count_tokens`. Token counting for local models is approximate. We accept this and build a 15% buffer into all budget estimates.

The plugin must function entirely offline. No DNS resolution, no certificate validation against external roots, no license checks. The abbey's internet connection is a 20 Mbps line shared with the retreat house. It is often saturated. The plugin cannot depend on it.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Sarment generates patches. The workflow integrates vineyard observations:

1. Terroir injects seasonal context (current parcel conditions, recent observations)
2. Sarment reads the task — often a vineyard note translated to a firmware requirement
3. Sarment generates INDEX.patch targeting the robot firmware
4. COMMIT.msg includes vineyard context:
   ```
   Adjust Frère Cep pruning pressure for parcel B2

   Observation: vine shoots in B2 show excess vigor (Br. Luc, 2026-03-25)
   Adjustment: pruning arm pressure increased 15% for shoots >8mm diameter
   Parcel: B2 (Grenache, 1987 planting)
   Season: pre-budbreak
   ```
5. Cep reviews for correctness and safety
6. Pressoir signs

The vineyard context in commit messages is not decoration. It creates a traceable link between the biological observation and the firmware change, allowing future brothers to understand why the code is the way it is.

---

## Requirement 4: Polyrepo PR Coordination

The Fellowship uses two repositories: `firmware` (robot control code) and `vineyard` (observation data and configuration). Changes often span both.

PR comment coordination:
```
[baf:link] firmware@adjust-pruning-b2 <-> vineyard@observation-b2-2026 relation=implements
```

Simple, bidirectional linking. No dependency DAG — the Fellowship's repos are small enough that manual coordination suffices for complex cases.

Forge: self-hosted Gitea. The trait implementation targets Gitea first, GitHub second.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/but-ai/memory/<parcel>/<season>/<key>`. Organization reflects the vineyard's physical and temporal structure.

| Memory Type | TTL | Example |
|-------------|-----|---------|
| Vine observation | 30 days | "B2 shoots showing excess vigor" |
| Firmware decision | 1 year | "Pruning pressure set to X for shoots >Ymm" |
| Seasonal summary | Permanent | End-of-season aggregate per parcel |
| Robot calibration | 90 days | Motor speeds, sensor offsets |

### Monastic Memory

The Fellowship adds a category not found in other proposals: the "reading" — a short daily summary written by the brother who walked the vineyard, converted to structured data by Terroir. These readings are the primary input to the AI's understanding of vineyard state. They are written in French, stored in French, and translated to firmware parameters by Sarment.

The readings are the human layer that no sensor can replace. A brother notices that a vine looks "tired" before any sensor detects the chemical signature of stress. These subjective observations, accumulated over seasons, are the Fellowship's most valuable data.

---

## Requirement 6: Signed Commits via OpenWallet

Community DID signing. The abbey's DID identifies the Fellowship, not individual brothers. Key rotation: 30 days. The signing key is stored on the abbey's server, which is physically located in the cellar (naturally cool, below the chapel).

Verification is offline — the DID document is hosted on the abbey's Gitea instance, accessible to anyone with network access to the server.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Sarment | 8,500 | 5,500 | 14,000 | Patch generation |
| Cep | 5,500 | 2,000 | 7,500 | Review |
| Terroir | 4,000 | 1,000 | 5,000 | Memory & context |
| Pressoir | 3,000 | 1,000 | 4,000 | Budget & signing |
| **Fellowship** | **21,000** | **9,500** | **30,500** | |

Budget is constrained by local model context windows (8K-32K tokens depending on model). We design for the floor.

---

## Unique Insight: Human Observation as Irreplaceable Context

Every proposal in this RFP will address how AI agents consume codebases, retrieve memory, and produce patches. Few will address where the most valuable context comes from.

In our vineyard, the most valuable data is not the sensor readings (temperature, moisture, pH). It is the brothers' daily readings — the subjective observations of humans who have walked the same rows for decades. "The Grenache in B2 looks tired" contains information that no sensor array captures: the integrated assessment of a human who knows what a healthy vine looks like, who remembers what this vine looked like last year, and who can sense that something is wrong before it becomes measurable.

We believe code has an equivalent: the human developer's intuition about a codebase. A senior engineer who says "this module feels fragile" is providing the same kind of signal as Brother Luc saying a vine looks tired. Both are irreplaceable inputs that AI agents must learn to consume, not replace.

Our memory system is designed to capture and preserve these human observations alongside structured data. The `reading` memory type exists for exactly this purpose: short, subjective, human-authored context that gives the AI agent access to the kind of knowledge that only comes from sustained, attentive presence.

The vineyard teaches patience. The brothers teach the machines.

---

*Submitted with the blessing of the Abbot. Parcel B2, pre-budbreak, Provence.*
