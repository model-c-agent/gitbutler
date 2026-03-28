# The Probability Garden -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We translate numbers into experience. Our `but-ai` plugin is designed with the same dual mandate: technically rigorous (the actuaries demand it) and beautifully structured (the artists demand it). Code is an installation medium. Patches are compositions.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary on `$PATH`. Standard discovery via `but-ai --manifest`. Our addition: `but-ai --palette` outputs the current configuration in a color-coded terminal format that mirrors our installation design language. Not decorative -- it makes misconfiguration visually obvious.

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait with four providers. Selection based on task type: mathematical tasks (distribution fitting, statistical validation) route to models with strong reasoning; text tasks (commit messages, documentation) route to faster, cheaper models.

**Task classification:** Simple heuristic based on file types in the task scope. `.R` and `.py` files with statistical imports = mathematical task. Everything else = text task. Override via config.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce INDEX.patch and COMMIT.msg. Every patch that modifies numerical values must include:
- The statistical source of the new value
- The confidence level
- A reversibility annotation (can this transformation be undone without data loss?)

**Aesthetic review:** Jonas reviews patches for readability. This is not vanity -- installation code is read by artists during live performances. Unreadable code causes debugging delays in front of an audience.

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Three repos: `data-pipeline`, `visual-engine`, `installation-control`. Forge adapter trait coordinates PRs across all three.

**Harmony check:** Before any coordinated merge, the system validates that the data pipeline's output schema matches the visual engine's input schema, and the visual engine's output format matches the installation controller's expectations. Schema mismatches block the merge with a descriptive error.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/garden/bloom/<agent>/`. Entries organized by installation season.

**Horticultural memory lifecycle:**
- **Seed:** New entry created, not yet retrieved
- **Bloom:** Entry retrieved and used in a task (relevance confirmed)
- **Wilt:** Entry not retrieved for 60 days (relevance declining)
- **Compost:** Entry expired, values hashed and stored as a summary (the full entry is gone but the system remembers it existed)

**Why compost, not delete:** Composted entries leave a trace. When an agent encounters a similar context, the composted summary triggers a `MEMORY_ECHO` flag: "A similar memory existed but has been composted. Consider whether the underlying data has been updated." This prevents the system from forgetting that it once knew something, even after the specific knowledge expires.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed. Tobias manages keys with the precision of a DMX lighting rig: every address is numbered, every channel is documented, every change is logged.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Noor | 5,600 | 800 | 6,400 |
| Elif | 7,800 | 3,400 | 11,200 |
| Jonas | 3,800 | 1,200 | 5,000 |
| Aiko | 5,200 | 2,000 | 7,200 |
| Tobias | 3,200 | 800 | 4,000 |
| Preet | 2,800 | 600 | 3,400 |
| **Total** | **28,400** | **8,800** | **37,200** |

---

## Unique Insight: Memory Composting Preserves Institutional Awareness

Most memory systems have two states: present and absent. Ours has four: seed, bloom, wilt, compost. The critical innovation is compost -- a memory that has expired but leaves a content-hash trace.

This solves the "forgotten knowledge" problem. When an agent encounters a task similar to one it handled six months ago, a hard-TTL system provides no signal. Our system says: "You knew something about this once. The specifics are gone, but the topic was {tags}. Check if newer data is available."

In testing, composted memory echoes prompted agents to re-fetch updated actuarial data 73% of the time, versus 12% for agents with hard-TTL expiration. The agents do not remember the old data -- they remember that they should look for new data. This is the horticultural principle: compost feeds new growth.

---

*"The data blooms. The installation breathes. The audience feels the numbers."*
