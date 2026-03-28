# Signal Dominance Group — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Classification:** Defense Communications Infrastructure

---

## Executive Summary

SDG proposes a redundancy-first agent architecture designed for military-grade communications configuration management. Every agent has a failover. Every commit has triple attestation. Every configuration change is evaluated against a minimum redundancy threshold. The system is built for environments where downtime is measured in casualties.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is distributed as a FIPS 140-2 validated build (required for DoD systems). Installed to `/opt/sdg/bin/but-ai`, which is added to PATH on provisioned workstations. The binary is compiled with `--cfg sdg_hardened`, which enables: stack canaries, position-independent execution, and runtime integrity checks.

Subcommands: `but ai assess` (analyze network topology for vulnerabilities), `but ai propose` (generate configuration patch), `but ai verify` (triple-attestation verification), `but ai rollback` (revert last N configuration changes without service interruption).

The `rollback` subcommand generates a reverse patch and verifies that applying it restores the previous configuration state. Rollback is tested as part of every proposal — Cole's pipeline generates the forward patch and the rollback patch simultaneously, and the test suite verifies both before the forward patch is offered for review.

## Requirement 2: Provider-Agnostic AI

Provider selection is constrained by CMMC Level 2 requirements. Currently approved: Azure OpenAI (FedRAMP High) and a locally deployed Llama model running on SDG-owned hardware within the SCIF. Cloud providers are used for unclassified configuration work. Classified work uses local inference exclusively.

The provider interface includes a `classification_level` parameter: `UNCLASSIFIED`, `CUI`, `SECRET`. The adapter refuses to send data above the provider's authorized classification level. This is enforced at the API boundary, not as a policy — the code physically prevents classified data from reaching a cloud endpoint.

Fallback: if the primary provider is unavailable, the system falls back to local inference. If local inference is unavailable, the operation fails. No degradation to a less secure provider.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The configuration management pipeline:

1. Briggs issues a requirement (e.g., "Increase bandwidth between nodes 7 and 12 without reducing redundancy")
2. Okafor's analysis agent evaluates the current topology and identifies viable approaches
3. Cole's patch agent generates INDEX.patch modifying configuration files
4. Sorokin's redundancy checker evaluates the proposed change against the minimum redundancy threshold
5. If redundancy check passes, the patch proceeds to signing
6. Triple attestation: Cole (generator), Briggs (authorizer), Ikeda (signer)

COMMIT.msg format:

```
Config: increase bandwidth node-7 to node-12 via secondary satellite link

Requirement: SDG-REQ-2026-0142
Redundancy-Before: 3 independent paths
Redundancy-After: 3 independent paths (maintained)
Bandwidth-Delta: +40Mbps
Rollback-Verified: yes
Generator: Agent/Cole
Authorizer: Agent/Briggs
Signer: Agent/Ikeda
```

## Requirement 4: Polyrepo PR Coordination

SDG maintains separate repos per contract. Cross-contract coordination occurs when infrastructure is shared (e.g., a satellite uplink serving two networks under different contracts). PR comments follow a structured format with contract references:

```
[SDG:coord] contract-alpha#18 ↔ contract-bravo#7
Shared satellite link SATCOM-3. Bandwidth reallocation in
contract-alpha affects available capacity for contract-bravo.
Coordination required before merge. Classification: CUI.
```

The forge adapter supports GitHub Enterprise (current) and GitLab (planned for a DoD client that mandates GitLab). Classification markings are included in every PR comment per DoD marking requirements.

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/sdg/memory/<contract>`. Memory types mapped to DoD records management:

- **`topology-state`**: Network topology snapshots. TTL: until superseded. Retained per contract records schedule (typically 7 years after contract close).
- **`failure-pattern`**: Documented failure modes and their mitigations. TTL: permanent. Failure knowledge is never expired — it is too valuable.
- **`optimization-history`**: Previous configuration changes and their performance impact. TTL: contract duration + 3 years.

Memory retrieval is key-based with contract-scoping. An agent working on contract-alpha cannot access contract-bravo's memory without explicit cross-contract authorization from Briggs. This isolation is a security requirement, not an architectural preference.

## Requirement 6: Signed Commits via OpenWallet

SDG uses a dual signing scheme: OpenWallet DIDs for technical attestation and DoD CAC (Common Access Card) signatures for official authorization. Ikeda manages the bridge between the two systems. Agent commits carry both signatures — the OpenWallet DID proves which agent generated the patch, and the CAC signature proves which human authorized it.

Key rotation follows DoD PKI certificate lifecycle (typically 3-year certificates with annual revalidation). Emergency revocation uses the DoD PKI CRL (Certificate Revocation List), which propagates within 24 hours — faster than SDG's operational tempo requires.

**Unique insight:** SDG's minimum redundancy threshold — a hard constraint that prevents agents from proposing changes that reduce system resilience — addresses a blind spot in optimization-focused agent systems. Most agent architectures optimize for a stated objective (performance, cost, speed) without constraints on what may be sacrificed to achieve it. SDG's architecture makes *what must be preserved* as explicit as *what should be improved*. This "preservation constraint" pattern is applicable beyond military communications: any system where certain properties (uptime, data integrity, safety margins) must not degrade during optimization benefits from a hard floor that agents cannot breach.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Briggs | 1,500 | 600 | 2,100 |
| Sorokin | 3,000 | 2,000 | 5,000 |
| Cole | 4,000 | 4,500 | 8,500 |
| Ikeda | 1,800 | 600 | 2,400 |
| Okafor | 2,500 | 1,000 | 3,500 |
| **Task Total** | **12,800** | **8,700** | **21,500** |

Triple-attestation overhead: 1,800 tokens. Redundancy verification: 2,200 tokens. Grand total per configuration task: **25,500 tokens**.

---

*"Resilience is not a feature. It is the product."*
— Dr. Nadia Sorokin, SDG founding charter
