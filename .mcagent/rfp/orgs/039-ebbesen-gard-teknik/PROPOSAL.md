# Ebbesen Gard Teknik -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Submitted by:** Lars Ebbesen, with input from Sofie, Karen, and Niels Sr.
**Date:** 2026-03-28

---

## Scope

We are a family farm. Our proposal reflects what we need: a simple, reliable tool that runs on modest hardware, works offline, and produces output that a 56-year-old electrician can review on paper. We are not building for scale. We are building for durability.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary. No daemon. Runs on Linux x86_64 and ARM64 (Raspberry Pi — we use them for everything).

Configuration: single TOML file. The file must be editable with `nano` because that is what Lars uses and he is not switching. No JSON, no YAML with anchors, no format that requires a dedicated editor.

The plugin must work fully offline. Our farm's internet comes through a 4G modem that loses signal during rainstorms. If the plugin needs network access to function, it does not function when we need it most (harvest season is also storm season in Jutland).

---

## Requirement 2: Provider-Agnostic AI

One provider: Ollama. We run Mistral 7B on the Dell OptiPlex with the T600. It is not fast. It is free and it works without internet.

The provider trait should support local-first operation: no API key required, no TLS certificate validation, no minimum bandwidth assumption. Token estimation for local models is approximate — we build a 20% buffer into all estimates based on empirical testing.

We recognize the RFP requires support for four providers. We will implement the trait for all four, but our testing and optimization will focus on Ollama with 7B-class models. Large context windows are a luxury we do not have (8K context on our hardware). The plugin must degrade gracefully when context is limited.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Plov generates patches. Our workflow:

1. Read task (usually a firmware change for one of our machines)
2. Retrieve machine-specific memory (hardware specs, recent changes)
3. Generate INDEX.patch scoped to the minimum necessary change
4. Generate COMMIT.msg in plain language:
   ```
   Fix grain dryer fan speed oscillation at low moisture

   The fan controller PID loop oscillated when moisture dropped below 12%.
   Reduced proportional gain from 0.8 to 0.5 for readings below 15%.
   Machine: grain-dryer-01 (Ebbesen custom, 1998 build, 2020 controller upgrade)
   ```
5. Validate with `git apply --check`
6. Submit for review (screen + paper)

Commit messages are written for humans who may read them decades from now. The farm's equipment lasts 20-30 years. The code that controls it should be understandable for as long.

---

## Requirement 4: Polyrepo PR Coordination

We use one repository. Cross-repo coordination is not a requirement for our workflow, but we implement the minimum specified by the RFP:

- Forge trait with three methods
- PR comment schema for linking related changes
- GitHub implementation (we host on GitHub because Lars set it up in 2019 and migration is not a priority)

Our honest contribution here is minimal. We implement the interface and defer to proposals with actual polyrepo experience.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/but-ai/memory/<machine>/<key>`. Organized by machine because each machine has its own firmware, its own hardware quirks, and its own history.

| Memory Type | TTL | Example |
|-------------|-----|---------|
| Hardware spec | Permanent | Register map, bus protocol, sensor specs |
| Firmware history | 1 year | Previous changes, their reasons, their outcomes |
| Workshop note | 90 days | Transcribed from the physical notebook |
| Task context | 24 hours | Current task parameters |

### Workshop Notebook Bridge

Our unique memory feature: transcribed entries from the physical workshop notebook. Karen and Niels have been writing in this notebook since 1971. It contains wiring diagrams, calibration values, repair notes, and decades of institutional knowledge. Lars has been digitizing entries as he encounters them, and these digitized entries are stored as memory with a `source: notebook-p<page>` tag.

The notebook is the authoritative record. When a memory entry conflicts with the notebook, the notebook wins. This is not a technical decision — it is a family one.

---

## Requirement 6: Signed Commits via OpenWallet

Family DID. One signing identity for the farm. Key stored on the OptiPlex, backed up on a USB drive in the workshop safe (the same safe that holds the property deed and Niels's service medals).

Key rotation: 90 days. Low rotation frequency because the threat model is negligible — the worst outcome of a key compromise is someone pushing a firmware change to our grain dryer, which would be caught by Karen's printed diff review before it reached the hardware.

AI-generated patches are marked in the commit trailer: `AI-Assisted: true`. This is Karen's requirement. She wants to know which changes came from the machine.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Plov | 9,000 | 5,500 | 14,500 | Patch generation |
| Harve | 5,000 | 2,000 | 7,000 | Review |
| Silo | 3,500 | 1,000 | 4,500 | Memory/budget/signing |
| **Family** | **17,500** | **8,500** | **26,000** | |

Smallest budget in the RFP. Constrained by hardware, not by ambition. Adequate for our needs.

---

## Unique Insight: The Workshop Notebook Problem

Every software organization has a workshop notebook — an accumulation of institutional knowledge that lives outside the codebase. It might be a Confluence wiki, a Slack channel, a senior engineer's memory, or a literal notebook in a literal workshop.

This knowledge is critical and fragile. When the person who holds it leaves (or, in our case, ages and cannot walk the fields), the knowledge leaves with them. Git preserves code. It does not preserve the reasons behind the code, the failed approaches that were tried first, or the hardware quirks that the code works around.

Our memory system is designed to capture workshop notebook knowledge: informal, human-authored, context-rich entries that explain not just what the code does but why it does it that way, what was tried before, and what to watch out for. These entries are tagged with the machine they relate to and the notebook page they were transcribed from, creating a bridge between the physical archive and the digital one.

We do not claim this is innovative. We claim it is necessary. The farm has survived a century. The firmware is four years old. If we want the firmware to last as long as the farm, we need to store more than code.

---

*Written in the workshop, between the lathe and the server rack. Reviewed on paper.*
