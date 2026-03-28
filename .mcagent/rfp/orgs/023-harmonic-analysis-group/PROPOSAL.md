# The Harmonic Analysis Group -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Confidence Interval:** 87%
**Submission Date:** 2026-03-28

---

## Executive Summary

We propose a deterministic, reproducibility-first implementation of the `but-ai` plugin. Our design treats every AI agent operation as an experiment with defined inputs, expected outputs, and measurable variance. The system guarantees that given identical inputs (codebase state, task description, provider configuration), the plugin produces identical outputs within a bounded error margin.

This guarantee is expensive. We accept that cost because we believe non-deterministic developer tools create debugging nightmares that dwarf the initial implementation savings.

---

## Requirement 1: PATH-Based Plugin Architecture

### Approach

The plugin is a standalone binary (`but-ai`) installed to `$PATH`. It communicates with the `but` CLI exclusively through structured stdin/stdout messages using a line-delimited JSON protocol. No shared memory. No IPC sockets. No environment variable side-channels.

We chose line-delimited JSON over Protocol Buffers because JSON is human-readable and our debugging methodology requires inspecting every message exchanged between plugin and host. Performance benchmarks show the serialization overhead is <2ms per message for payloads under 64KB, which is within our latency budget.

### Determinism Invariant

The plugin's initialization sequence is fully deterministic: given the same `$PATH`, the same configuration file, and the same provider credentials, the plugin produces identical capability advertisements. We enforce this through a startup self-test that hashes the capability set and logs it.

---

## Requirement 2: Provider-Agnostic AI

### Approach

We implement a provider abstraction layer with four backends: OpenAI, Anthropic, Ollama, and LMStudio. Each backend implements a trait with exactly five methods: `initialize`, `complete`, `complete_with_tools`, `estimate_tokens`, and `health_check`.

Provider selection is explicit -- the user configures their provider in `but-ai.toml`. We do not auto-detect providers because auto-detection introduces non-determinism. If the user wants to switch providers, they change the configuration and restart. Hot-switching mid-session is not supported because it would invalidate our budget forecasts.

### Provider Variance Tracking

Each provider's responses are hashed and logged. Over time, the system builds a variance profile per provider: how often does the same prompt produce different outputs? This data is stored in the memory system and used to adjust confidence intervals on budget forecasts.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

### Approach

The agent produces unified diffs (INDEX.patch) and structured commit messages (COMMIT.msg). Patches are generated through a three-phase process:

1. **Context acquisition:** Read the relevant files, branch state, and memory entries. This phase consumes the majority of the input token budget.
2. **Patch synthesis:** Generate the unified diff. The agent is constrained to produce patches that apply cleanly against the current HEAD -- patches that fail `git apply --check` are rejected and regenerated (up to 2 retries).
3. **Verification:** The reviewer agent (Fourier) independently evaluates the patch against the task description and flags any discrepancies.

### Reproducibility Guarantee

For a given (codebase state, task description, provider, temperature=0), the agent produces the same INDEX.patch with >92% consistency across runs. We measure this by running the same task 10 times and computing the diff-similarity score. This metric is logged and available via `but-ai metrics`.

---

## Requirement 4: Polyrepo PR Coordination

### Approach

Cross-repository coordination uses a structured PR comment schema:

```json
{
  "schema": "harmonic/coord/v1",
  "source_repo": "org/repo-a",
  "source_branch": "feat/change",
  "target_repo": "org/repo-b",
  "dependency_type": "blocks | informs | relates",
  "patch_hash": "sha256:...",
  "status": "pending | applied | conflicted"
}
```

The schema is forge-agnostic -- it is transmitted as a PR comment body and parsed by the receiving agent. GitHub, GitLab, Gitea, and Bitbucket all support PR comments; the only forge-specific code is the API client for posting and reading comments.

### Dependency Resolution

Dependencies between cross-repo patches are modeled as a directed acyclic graph. The agent refuses to apply a patch if its declared dependencies have not been resolved. Circular dependencies are detected at declaration time and rejected with a diagnostic message.

---

## Requirement 5: Agent Memory in Git Branches

### Approach

Memory entries are stored as blobs in a dedicated Git ref namespace: `refs/but-ai/memory/<agent>/<key>`. Each entry has:

- A spectral signature (weighted keyword vector, stored as JSON metadata)
- A TTL (default: 7 days, configurable per entry)
- A relevance score (computed at retrieval time, not storage time)

Retrieval uses cosine similarity between the current task context's spectral signature and stored entries' signatures. The top-K entries (K=5 by default) are injected into the agent's context.

### Calibration

The relevance threshold is not hardcoded. It is calibrated quarterly using a held-out test set of (task, memory, relevance judgment) triples labeled by the group. Current threshold: 0.72 cosine similarity. Current precision at this threshold: 0.89. Current recall: 0.76.

---

## Requirement 6: Signed Commits via OpenWallet

### Approach

Every agent commit is signed using a Verifiable Credential (VC) issued by an OpenWallet-compatible identity provider. The signing workflow:

1. Agent requests a short-lived signing credential from the wallet (validity: 1 hour).
2. The credential is used to produce a GPG-compatible signature on the commit.
3. The signature includes the agent's DID, the credential's expiry, and a hash of the INDEX.patch.
4. Verification is performed by any party with access to the wallet's public DID document.

Key rotation occurs every 30 days. Compromised keys are revoked immediately via the wallet's revocation registry. All signing events are logged to the memory system for audit.

### Formal Invariant

We provide a formal proof (in supplementary material) that the signing workflow satisfies non-repudiation: given a valid signature, it is computationally infeasible to deny that the signing agent produced the commit. This proof assumes the wallet's cryptographic primitives are sound (standard assumption).

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Hertz | 11,000 | 6,000 | 17,000 | Patch generation |
| Fourier | 7,000 | 3,000 | 10,000 | Review & verification |
| Mel | 6,000 | 1,500 | 7,500 | Memory retrieval |
| Decibel | 4,000 | 2,000 | 6,000 | Budget enforcement |
| **Team** | **28,000** | **12,500** | **40,500** | |

Budget is per-task for a standard 200-line, 3-file change. Complex tasks scale linearly (verified empirically). Coordination overhead is included in Decibel's budget.

---

## Unique Insight: Reproducibility as Debuggability

Most proposals will optimize for speed or cost. We optimize for reproducibility.

Our experience with `repromix` and `diffwav` taught us that the most expensive bugs in AI-assisted workflows are the non-deterministic ones -- the patches that work on Tuesday and fail on Wednesday with no code change in between. These bugs are invisible to traditional testing because the test suite passes on both days. They surface only in production, at scale, when someone notices that the same task produces different results.

By instrumenting every agent operation with variance tracking and spectral fingerprinting, we make non-determinism visible. When a patch diverges from expected output, the system does not just flag the failure -- it shows you which component introduced the variance, how much variance, and whether it exceeds the historical norm. This is not free. It costs approximately 15% more tokens than a non-instrumented system. We believe the debugging time it saves makes it the cheapest option in total cost of ownership.

---

*CI: 87%. Predictions about system behavior are based on extrapolation from `repromix` deployment data (n=142 sessions) and may not generalize to all codebases.*
