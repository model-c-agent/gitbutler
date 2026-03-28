# Streets for All Assembly -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Date:** 2026-03-28
**From:** The Assembly (Oakland, CA)

---

## Summary

We are residents who design our own streets. Our `but-ai` proposal is built for spatial data workflows: agents that collect crash data, pedestrian counts, and community input, then produce patches that update a geographic database. Our unique contribution is spatially-indexed agent memory -- memory entries tagged with coordinates, relevant to nearby tasks, decaying with distance.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Per-task invocation.

- Binary: Rust, statically linked
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml`
- Accessibility: config must be understandable by non-engineers. Rosa reviews all config changes for clarity.

We optimize for accessibility because our participants are neighbors, not developers. The plugin must be usable by someone who learned Git last year.

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`.

| Provider | Usage | Notes |
|----------|-------|-------|
| Ollama | Primary (free) | Local analysis, no API cost |
| Anthropic | Complex analysis | When grant budget allows |
| OpenAI | Batch data processing | Cost-effective for scraping normalization |
| LMStudio | Development | Pen and Jamal's local testing |

Provider selection is cost-driven. The Assembly runs on volunteer time and occasional grants. Free providers are default.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Spatial Data Patches

Patches update the Assembly's geographic database. Each patch targets a data layer:

```
data: update crash records for intersection INT-2847

Source: NHTSA FARS, accessed 2026-03-15
New records: 3 (2025-Q4)
Total at intersection: 14 (2020-2025)
Severity: 1 fatal, 2 injury
Mode: 2 pedestrian, 1 bicycle

Coordinates: 37.7749, -122.2119
Layer: crash-data
Agent: pen
```

### Community Input Patches

```
input: add survey responses for Fruitvale Corridor Block 3

Respondents: 12 households
Top concern: vehicle speed (9/12)
Second concern: missing crosswalk (7/12)
Collection: door-to-door, 2026-03-20
Collector: rosa

Coordinates: 37.7755, -122.2110
Layer: community-input
Agent: rosa
```

---

## 4. Polyrepo PR Coordination

The Assembly coordinates with partner neighborhood groups and (reluctantly) with city departments.

### Coordination Schema

```json
{
  "assembly": "streets-for-all",
  "project": "fruitvale-corridor",
  "action": "data_shared",
  "layer": "crash-data",
  "intersections": ["INT-2847", "INT-2848"],
  "branch": "data/crash/2025-q4"
}
```

Forge support: GitHub (primary). The Assembly does not use other forges because nobody wants to maintain more than one.

---

## 5. Agent Memory in Git Branches

### Spatially-Indexed Memory

Memory stored in `refs/sfa/memory/spatial/<geohash>/<key>`.

The geohash prefix enables spatial retrieval: when working on intersection INT-2847, retrieve all memory entries within the same geohash tile (approximately 200m radius).

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `content` | Memory value |
| `coordinates` | Lat/lon |
| `geohash` | Geohash prefix (precision 7, ~150m) |
| `type` | `design_standard`, `site_condition`, `community_input`, `crash_data` |
| `source` | Data provenance |
| `ttl` | Type-dependent (standards: indefinite, conditions: 720h, input: 336h) |

### Distance Decay

Memory relevance decays with distance from the task location. An entry 50 meters away is weighted higher than one 150 meters away. This models reality: a design decision at one intersection is highly relevant to the adjacent intersection and less relevant to one three blocks away.

### Retrieval

Top-5 entries by relevance, weighted by distance and recency. Design standards are always injected regardless of distance (they apply everywhere).

---

## 6. Signed Commits via OpenWallet

Jamal manages signing.

- Keys provisioned via OpenWallet per participant
- Rotation: monthly
- Revocation: stored in `refs/sfa/revoked`
- Community input commits: signed by the collecting participant (Rosa, typically)
- Data commits: signed by the processing agent (Pen)
- Design commits: signed by Tomoko

Signing authority follows expertise: the person responsible for the data type signs the data.

---

## 7. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Pen | Data/patches | 7,800 | 4,200 | 12,000 |
| Tomoko | Design/memory | 5,000 | 1,000 | 6,000 |
| Jamal | Tooling/signing | 3,800 | 1,200 | 5,000 |
| Rosa | Community/coordination | 4,500 | 2,000 | 6,500 |
| **Assembly** | | **21,100** | **8,400** | **29,500** |

---

## 8. Unique Insight: Geospatial Memory Indexing

Agent memory systems typically index by key, namespace, or time. We index by **location**. Memory entries are stored at geographic coordinates and retrieved by spatial proximity to the current task.

This is natural for urban planning work: when designing an intersection improvement, the most relevant context is what happened at nearby intersections -- crash patterns, community concerns, design decisions, material choices. A memory system that retrieves the five nearest entries provides better context than one that retrieves the five most recent.

Spatial indexing generalizes to any domain with geographic context: infrastructure maintenance, logistics routing, environmental monitoring, public health surveillance. Any agent that operates over spatially distributed data benefits from memory that understands proximity.

We implement this with geohash prefixes in the Git ref path, enabling efficient range queries without a spatial database. Git's tree structure does the indexing. We just organize the refs to match.

---

*"Every intersection tells a story. We listen, then we build."*
