# Phantom Orbit — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

Phantom Orbit proposes a `but-ai` implementation built on zero-trust principles: every agent action is signed, every signature is verified, and no agent is trusted by default. Our experience publishing classified orbital data under adversarial conditions taught us that security is not a layer you add — it is the foundation you build on.

## Requirement 1: PATH-Based Plugin Architecture

Statically-linked Rust binary at `$PATH`. The binary is reproducibly built — given the same source and toolchain, the output binary is byte-identical. This allows anyone to verify that the distributed binary matches the published source. Build reproducibility is verified in CI using a hash comparison against a known-good build.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. Sensitive configuration values (API keys, signing key paths) support encrypted storage via `age` encryption. The config file can be committed to the repository with secrets encrypted; only the machine with the decryption key can read them.

## Requirement 2: Provider-Agnostic LLM Interface

Four backends. Doppler manages provider selection with a security-first bias: local providers (Ollama, LMStudio) are preferred when the task involves sensitive code, because tokens sent to cloud APIs leave the local trust boundary. Provider selection can be constrained per-repository via config (`providers.allowed = ["ollama", "lmstudio"]` for sensitive repos).

Doppler monitors provider quality using a rolling metric computed from response latency, tool-call accuracy, and output coherence (measured by a lightweight classifier that detects degenerate outputs). When quality degrades, Doppler shifts to the next provider in the chain — a "frequency shift" that happens before hard failure.

**Domain Insight:** In signals intelligence, you never transmit on a single frequency because it is trivially intercepted. You shift frequencies. Doppler applies the same principle: never depend on a single provider, and shift before the adversary (in this case, provider degradation) forces you to.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Kozai generates patches with a dependency-aware approach:
1. **Dependency mapping** — Before reading file contents, Kozai maps the import/dependency graph of the files in scope. This identifies files that might be transitively affected by the change.
2. **Context loading** — Read the primary files plus one level of transitive dependencies.
3. **Patch generation** — Generate INDEX.patch targeting correctness across the full dependency surface.
4. **Verification** — Apply patch, run available checks, verify no transitive breakage.

COMMIT.msg is terse by design:
```
fix(tle): correct epoch rollover at year boundary

Agent: Kozai | Provider: ollama/codestral | Tokens: 2,400/1,600
Deps-checked: 3 files | Memory-refs: yarkovsky/drift-0x4a2f
Signed: Enigma (did:key:z6Mk...)
```

## Requirement 4: Polyrepo PR Coordination

Molniya manages cross-repo coordination using an asymmetric monitoring model. High-activity repos are polled every 2 minutes. Low-activity repos are polled every 15 minutes. Polling frequency adapts based on a rolling activity metric.

Coordination sets are tracked in `refs/phantom/coord/` as signed JSON documents. Every coordination message is signed by Enigma to prevent injection of false coordination data. This is Phantom Orbit's distinguishing feature: coordination messages are attack surface, and unsigned coordination data is untrusted.

Forge adapters (GitHub, GitLab, Gitea) with minimal API surface. PR comments carry machine-readable metadata in signed blocks. The signature covers the metadata content, preventing tampering.

## Requirement 5: Agent Memory in Git Branches

Yarkovsky manages memory using a "drift" model. Memories are not explicitly queried. Instead, Yarkovsky maintains a background process that continuously compares stored memory embeddings against the current task context. When a memory's similarity to the current context exceeds a configurable threshold, it "drifts" into the active context automatically.

Storage: `refs/phantom/drift/<memory-hash>` as encrypted blobs. Memory entries are encrypted at rest using the repository's memory key (derived from the agent's DID). This prevents memory exfiltration from a cloned repository.

| Drift Class | Similarity Threshold | TTL | Description |
|-------------|---------------------|-----|-------------|
| Resonant | > 0.8 | 60 days | Strongly related, surfaces immediately |
| Proximate | 0.5 - 0.8 | 30 days | Likely related, surfaces on second pass |
| Distant | 0.3 - 0.5 | 14 days | Weakly related, surfaces only if explicitly queried |
| Dark | < 0.3 | 7 days | Irrelevant to current context, decays quickly |

GC removes expired entries and re-encrypts the memory branch on a configurable schedule.

## Requirement 6: Signed Commits via OpenWallet

Enigma manages signing with the most stringent security model in this RFP:
- Keys generated offline, never on the machine that runs the agent.
- Keys stored encrypted at rest, decrypted only for signing operations.
- Every signing operation logged to an append-only audit file.
- Zero-trust verification: Enigma verifies the agent identity, target branch, and task scope before signing. Any mismatch triggers an alert and blocks the sign.
- Key rotation every 21 days (shortest in this RFP for space debris orgs).
- Emergency revocation publishes a signed revocation notice to the memory branch, cross-repo coordination channels, and a configured webhook.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Kozai | Patch generation | 9,000 | 5,000 | 14,000 |
| Yarkovsky | Memory (drift) | 4,000 | 600 | 4,600 |
| Doppler | Provider & budget | 3,000 | 800 | 3,800 |
| Molniya | PR coordination | 5,000 | 2,000 | 7,000 |
| Enigma | Signing & OpSec | 2,500 | 500 | 3,000 |
| **Per-task total** | | **23,500** | **8,900** | **32,400** |

## Unique Domain Insight

Five years of publishing classified orbital data under adversarial conditions taught us that the most dangerous attack is not the one that breaks your system — it is the one that subtly corrupts your data without triggering any alarm. A TLE with a one-digit error looks valid, propagates correctly for 24 hours, and then diverges catastrophically.

Our proposal treats agent output integrity as a first-class concern. Every patch, every memory entry, every coordination message is signed. Signatures are verified at every trust boundary. The cost is approximately 5% of the total token budget (Enigma's allocation). We consider this the cheapest insurance in the proposal. An unsigned agent system is an unverified agent system, and an unverified system operating on production code is a liability, not an asset.

---

*Data published. Source: classified. Accuracy: verified. Identity: signed.*
