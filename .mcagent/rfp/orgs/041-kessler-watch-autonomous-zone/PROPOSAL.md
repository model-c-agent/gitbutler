# Kessler Watch Autonomous Zone -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Submitted by:** Five contributors, representing only themselves
**Date:** 2026-03-28

---

## Disclaimer

This proposal does not represent the Kessler Watch Autonomous Zone because the Kessler Watch Autonomous Zone cannot be represented. It represents five contributors who believe the `but-ai` plugin would be useful for our work and who have opinions about how it should be built. Other contributors may disagree. That is expected and welcome.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary on PATH. Distributed via the network's Gitea instance with SHA-256 hashes. No auto-update, no telemetry, no phone-home.

Configuration: TOML. The configuration must support multi-user operation on a single machine — multiple contributors sharing a server should each have their own config without interference. We recommend `$XDG_CONFIG_HOME/but-ai/config.toml` with support for per-repository config overrides via `.but-ai.toml` in the repo root.

No daemon. Our contributors run the plugin on hardware ranging from laptops to Raspberry Pis to shared university servers. Background processes on shared machines are antisocial.

---

## Requirement 2: Provider-Agnostic AI

Four providers. No preference — contributors choose their own. The plugin must function with any single provider and must not assume capabilities beyond the trait definition.

Trait: `init`, `complete`, `complete_with_tools`, `count_tokens`. The trait does not include streaming because some of our contributors run models on hardware that does not benefit from streaming (Raspberry Pi with Ollama), and we want feature parity across providers.

No automatic fallback. Contributors manage their own provider configuration.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Perigee generates patches independently. Key design point: patches must be self-contained.

In a network with no coordination, two contributors may be working on related changes simultaneously. If patches have implicit dependencies on each other, one will break when the other merges first. Our requirement: every patch produced by `but-ai` must apply cleanly against the declared base commit without depending on any other in-flight patch.

Workflow:
1. Contributor invokes `but-ai` with a task
2. Agent reads current HEAD (not another agent's branch)
3. Agent generates INDEX.patch against HEAD
4. Agent generates COMMIT.msg
5. Contributor reviews locally before pushing

### Observation Quality Tags

For patches that modify the observation catalog, the COMMIT.msg includes:

```
Add TLE for NORAD 52345 from visual observation

Observer: sunspot
Method: visual, 10" Dobsonian, Johannesburg
Quality: moderate (naked-eye, single pass)
RMS-Residual: est. 8 km
```

The quality tag is honest. We do not inflate quality.

---

## Requirement 4: Polyrepo PR Coordination

The network maintains one primary repo (the catalog) and several secondary repos (tooling, analysis scripts, observation pipelines). Cross-repo coordination is minimal because the repos are loosely coupled.

PR comment schema:
```
[kw:link] catalog@add-tle-52345 <-> analysis@conjunction-check-52345 status=related
```

Lightweight linking, not dependency management. We do not block patches on cross-repo dependencies because blocking requires someone to unblock, and there is no someone.

Forge: Gitea. Minimal trait implementation.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/but-ai/memory/<catalog-id>/<key>`. Organized by NORAD catalog number for debris objects, and by generic namespace for non-object-specific memory.

| Memory Type | TTL | Example |
|-------------|-----|---------|
| Orbital elements | Until superseded | TLE for NORAD 52345 |
| Observation record | 1 year | Visual observation with quality tag |
| Conjunction prediction | 7 days | Predicted close approach |
| Contributor note | 30 days | "NORAD 52345 tumbling, brightness variable" |
| Tool configuration | 90 days | Processing pipeline parameters |

### Multi-Source Memory

The same object may have memory entries from multiple contributors with conflicting data. We do not resolve conflicts — we store all entries with contributor attribution and quality tags. The consuming agent sees all entries and must reason about which to trust.

This is philosophically consistent with our approach to observations: we collect all data and let the consumer assess quality, rather than filtering at the gate.

---

## Requirement 6: Signed Commits via OpenWallet

Contributor-controlled DIDs. Each contributor manages their own identity. The network does not issue identities or maintain a directory.

Three identity modes:
1. **Ephemeral:** New DID per session. Maximum privacy.
2. **Pseudonymous:** Persistent handle-linked DID. Builds reputation without revealing identity.
3. **Attributed:** Real-name DID for contributors who want academic credit.

All three produce valid, verifiable signatures. The plugin does not prefer one over another.

Key rotation: contributor's choice, minimum 30 days.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Perigee | 9,500 | 6,000 | 15,500 | Patch generation |
| Apogee | 5,500 | 2,000 | 7,500 | Review |
| Inclination | 4,500 | 1,000 | 5,500 | Memory |
| Epoch | 3,000 | 1,000 | 4,000 | Signing/budget |
| **Per contributor** | **22,500** | **10,000** | **32,500** | |

Budget is per-contributor, not per-network. There is no shared budget.

---

## Unique Insight: Coordination-Free Collaboration

Every proposal in this RFP will include a coordination protocol: how agents communicate, how tasks are assigned, how dependencies are tracked. We propose the opposite: coordination-free collaboration.

Our network produces useful output — a debris catalog with 80,000+ observations — without any coordination mechanism. Contributors observe, contribute, and reconcile independently. The shared repository is the coordination mechanism: Git's merge semantics handle concurrent contributions, and conflicts are resolved by whoever encounters them.

This works because our contributions are largely independent. One contributor's observation of NORAD 52345 does not invalidate another contributor's observation of NORAD 52346. Independence is the precondition for coordination-free collaboration.

We believe this insight applies to `but-ai`: when tasks can be decomposed into independent subtasks, the most efficient coordination protocol is no protocol at all. Generate independent patches against the same base, merge them, and let Git handle the rest. The overhead of coordination — token cost, latency, complexity — is worth paying only when independence cannot be achieved.

Our proposal does not include a coordination protocol because we do not need one. For users who do, we recommend implementing the simplest possible protocol (shared task queue, first-to-merge wins) and measuring whether the coordination cost is justified by the coordination benefit. Often, it is not.

---

*Submitted from five countries. Coordinated by no one. The catalog grows regardless.*
