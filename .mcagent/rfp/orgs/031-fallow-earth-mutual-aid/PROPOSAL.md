# Fallow Earth Mutual Aid -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Submitted by:** Five contributors who do not represent the network
**Date:** 2026-03-28

---

## Context

This proposal comes from five individuals, not from Fallow Earth as an institution (there is no institution). We write from our experience building firmware translation tools for the open-source tractor repair network. Our perspective is shaped by working with low-resource hardware, unreliable connectivity, and contributors who are farmers first and programmers second.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary on PATH. No runtime dependencies. The binary must run on: x86_64 Linux, ARM64 Linux (Raspberry Pi), and x86_64 macOS. ARM64 Linux is non-negotiable -- many contributors run agents on Raspberry Pis mounted in their tractors.

Configuration: single TOML file. First-run setup auto-generates sensible defaults. The setup asks one question: which provider? Everything else gets defaults that work for resource-constrained environments.

Offline operation: the plugin must handle network absence gracefully. If the provider is unreachable, the plugin should queue tasks locally and process them when connectivity returns. This is not a theoretical requirement -- contributors in rural Kenya and Saskatchewan have intermittent connectivity measured in hours, not minutes.

---

## Requirement 2: Provider-Agnostic AI

Four providers. Ollama is our primary recommendation because it runs locally and does not require internet. The provider trait has four methods: `init`, `complete`, `complete_with_tools`, `count_tokens`.

### Resource-Constrained Operation

Local models on Raspberry Pi 5 (8GB RAM) can run small models (Phi-3, Gemma 2B). The plugin must function within these constraints: reduced context windows, slower inference, and approximate token counting. We design for the floor, not the ceiling.

Provider configuration includes a `resource_profile` setting: `full` (cloud or powerful local), `constrained` (Pi or low-memory), and `minimal` (sub-4GB RAM). Each profile adjusts default context sizes, memory retrieval limits, and budget allocations.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Plow generates patches. The workflow for translation tasks:

1. Read source patch (the fix on the source platform)
2. Retrieve platform memory (target platform specs, previous translations)
3. Generate translated INDEX.patch for target platform
4. Generate COMMIT.msg with translation notes
5. Validate: `git apply --check` and platform-specific linting
6. Submit to Harrow for review

### Translation Notes

Every cross-platform patch includes a structured translation note in the COMMIT.msg:

```
Source: jd8r@firmware-v4.2/fix-hydraulic-timing
Target: mf8s@firmware-v3.1
Adaptations:
  - Memory address 0x4A00 -> 0x5200 (hydraulic controller base)
  - Timer register width 16-bit -> 32-bit
  - Added endianness conversion for bus protocol
Confidence: medium (hardware untested)
```

The `Confidence` field is an honest self-assessment. We do not pretend AI-translated firmware patches are as reliable as human-verified ones.

---

## Requirement 4: Polyrepo PR Coordination

Each tractor platform has its own repository. Cross-platform translations require coordination between repos. Our approach:

PR comments with a simple schema:
```
[fallow:xplat] source=jd8r-firmware#fix-timing target=mf8s-firmware#port-timing status=review
```

Forge abstraction: three methods. GitHub implementation first. Most network repos are on GitHub because that is where the README was first pushed in 2020. We will add Gitea support because several contributors self-host to avoid corporate platforms.

Dependency semantics: `source` patches must be merged before `target` translations are finalized. If the source patch changes after translation, the translation is invalidated and regenerated.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/but-ai/memory/<platform>/<key>`. Organized by platform because platform context is the most valuable type of memory for translation tasks.

Memory types:
| Type | TTL | Example |
|------|-----|---------|
| Platform spec | 90 days | JD8R memory map, bus protocol |
| Translation record | 30 days | Previous JD8R->MF8S translations |
| Contributor note | 7 days | "This register is undocumented, found by trial and error" |
| Task context | 24 hours | Current task description and scope |

### Contributor Notes

The most valuable memory entries are informal notes from contributors who have physical access to the hardware. "This register is undocumented but controls the PTO speed limiter" is information that no spec sheet contains. These notes are tagged with the contributor's handle and the hardware version they tested against.

Retrieval: tag matching with platform affinity. Queries against `mf8s` retrieve `mf8s`-tagged entries first, then `massey-ferguson`-tagged entries, then generic entries.

---

## Requirement 6: Signed Commits via OpenWallet

Two signing modes:

1. **Named signing:** Contributor's personal DID. Used when the contributor can safely be identified.
2. **Pseudonymous signing:** Network's shared DID (`did:web:fallow.earth:contributors`). Used when the contributor needs anonymity.

Both modes produce valid, verifiable signatures. The only difference is attribution granularity: named signing says "Hazel signed this," pseudonymous signing says "a Fallow Earth contributor signed this."

Key rotation: 30 days for named DIDs, 7 days for the shared DID (higher rotation frequency because more people have access).

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Plow | 10,000 | 7,000 | 17,000 | Patch generation/translation |
| Harrow | 6,000 | 2,500 | 8,500 | Review |
| Seed | 4,500 | 1,000 | 5,500 | Memory |
| Fence | 3,500 | 1,000 | 4,500 | Budget & signing |
| **Network** | **24,000** | **11,500** | **35,500** | |

Budget assumes `constrained` resource profile. `Full` profile allows 1.5x expansion for complex translations.

---

## Unique Insight: Cross-Platform Translation as Code Review

Every proposal will discuss patch generation -- how to turn a task description into a unified diff. Our experience adds a dimension: patch translation -- how to turn a patch for platform A into an equivalent patch for platform B.

This is not a niche problem. In a polyrepo world, the same conceptual change often needs to land in multiple repositories with different codebases, different languages, and different conventions. A bug fix in the Python client may need a corresponding fix in the Rust SDK. A schema change in the API repo may need matching changes in three consumer repos.

Our firmware translation workflow -- read source patch, extract conceptual fix, generate target patch, annotate the translation -- is a general-purpose pattern for cross-codebase coordination. The `Confidence` field in the translation note is the honest answer to the question every cross-repo patch raises: "How sure are we that this translation preserves the original fix's intent?"

We are not sure. We say so. We believe that is more valuable than claiming certainty we do not have.

---

*Pushed from a tractor cab in Nakuru County. Connectivity permitting.*
