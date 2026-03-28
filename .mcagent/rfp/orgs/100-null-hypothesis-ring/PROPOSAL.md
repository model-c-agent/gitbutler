# Null Hypothesis Ring -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

Privacy is not a feature. It is the architecture. Every component of our `but-ai` plugin is designed to prevent data leakage, resist stylometric identification, and produce cryptographically verifiable analysis that regulators can trust. We do not use cloud providers. We do not store plaintext. We do not trust the network.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary. Built from source on each member's machine using reproducible builds (nix flake). Binary hash verified against a shared manifest signed by epsilon. No pre-built binaries from external sources are trusted.

**Air-gap compatible:** The binary operates fully offline. No telemetry, no update checks, no network calls except to the configured local LLM provider.

---

## Requirement 2: Provider-Agnostic AI

Local models only. The `Completer` trait is implemented for Ollama and LMStudio. Cloud provider implementations exist in the codebase (for other users of the plugin) but are disabled by default and require explicit opt-in via `BUT_AI_ALLOW_CLOUD=true`.

**Data containment:** All LLM interactions occur on localhost. No query, context, or response leaves the machine. This is non-negotiable for the Ring's use case.

**Model selection:** We maintain a list of tested local models with performance benchmarks for actuarial analysis tasks. Current recommendation: Llama 3.1 70B (quantized) for analysis, Llama 3.1 8B for formatting.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Patches produced. Patches signed. Patches encrypted at rest. The workflow:

1. Agent generates INDEX.patch on the member's local machine
2. Patch is signed with the member's individual GPG key
3. Patch is submitted to the shared repo encrypted (age encryption, recipient keys for all active members)
4. Reviewers decrypt locally, review locally, submit review comments encrypted
5. Merge requires 3-of-5 signatures (matching the Shamir threshold)

**Stylometric defense:** COMMIT.msg follows a rigid template with no free-text fields. All descriptive text is structured. This prevents writing-style analysis from identifying members.

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Two repos: private analysis (self-hosted Gitea) and public publication (GitHub). The repos are deliberately unlinked.

**Sanitized transfer protocol:**
1. Analysis is completed in the private repo
2. p_value extracts publication-ready artifacts (summary stats, model coefficients, methodology description)
3. Artifacts are stripped of metadata (Git author, timestamps, file paths)
4. Artifacts are committed to the public repo under the Ring's collective identity
5. No forge-level link exists between the two repos

**Forge adapter:** Gitea and GitHub implementations. The adapter handles authentication differently for each forge, with the private Gitea instance using SSH keys and the public GitHub using deploy tokens.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/ring/mem/`. All entries encrypted at rest.

**Encrypted memory schema:**
```json
{
  "key": "<sha256 of encrypted value>",
  "ciphertext": "<age-encrypted blob>",
  "recipients": ["actuary_x", "deadweight", "nullset", "p_value", "epsilon"],
  "created": "2026-03-28T00:00:00Z",
  "ttl": 7776000
}
```

**No plaintext metadata:** Even memory tags are encrypted. The only unencrypted field is the content-hash key (which reveals nothing about the content) and the TTL. This prevents an attacker who gains access to the Git refs from learning what the Ring is investigating.

**Retrieval:** Members decrypt entries locally, compute embeddings locally, and perform similarity search locally. Nothing is centralized.

---

## Requirement 6: Signed Commits via OpenWallet

OpenWallet integration for the public publication repo only. The private analysis repo uses GPG with Shamir-split keys.

**Threshold signing:** Publication commits require 3-of-5 partial signatures combined into a single valid signature. No single member can publish on behalf of the Ring.

**Revocation:** If a member is compromised, their Shamir share is revoked and the master key is reconstructed and re-split among remaining members. All subsequent publications use the new key. Previous publications remain verifiable against the old key via the OpenWallet trust registry's historical records.

---

## Token Budget

| Handle | Input | Output | Total |
|--------|-------|--------|-------|
| actuary_x | 7,200 | 3,400 | 10,600 |
| deadweight | 4,800 | 1,400 | 6,200 |
| nullset | 5,000 | 600 | 5,600 |
| p_value | 5,400 | 2,000 | 7,400 |
| epsilon | 3,800 | 1,000 | 4,800 |
| tail_risk | 2,400 | 400 | 2,800 |
| **Total** | **28,600** | **8,800** | **37,400** |

Note: Token budget reflects local model usage. Local models have no monetary cost per token, but the budget constrains context window usage to ensure generation quality.

---

## Unique Insight: Encrypted Memory Prevents Investigation Leaks

Most agent memory systems assume a trusted storage environment. Ours assumes the storage is hostile. If an adversary (an insurer's legal team, for example) gains access to the Git repository, they should learn nothing about ongoing investigations.

Encrypted memory with content-addressed keys achieves this. The repository contains encrypted blobs and opaque hashes. Without the decryption keys (which exist only on members' machines, split via Shamir), the memory is indistinguishable from random data.

This is not paranoia. In 2024, a Ring member's personal laptop was subpoenaed in an unrelated legal matter. The encrypted Git refs on the machine were identified by forensic analysts but could not be decrypted. The investigation was uncompromised. The member's identity within the Ring was unconfirmed.

Encrypted memory is not about hiding wrongdoing. It is about protecting the integrity of an investigation from the subjects of that investigation. The data speaks when the Ring publishes. Until then, it stays silent.

---

*"We have no names. We have evidence. That is sufficient."*
