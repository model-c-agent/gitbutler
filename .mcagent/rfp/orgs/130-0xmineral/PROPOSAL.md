# PROPOSAL.md — 0xMineral

**`> cat PROPOSAL.md | gpg --verify`**
**`gpg: Good signature from "cr4ter <cr4ter@0xmineral.onion>"`**

---

## Summary

0xMineral proposes to build the `but-ai` plugin as a covert spectral analysis pipeline. Every codebase is terrain. Every task is a target. Memory is spectral data — multi-band, high-resolution, and independently verifiable. Everything is signed. Nothing is trusted by default.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

`but-ai` is a binary on PATH. Discovery is standard. The collective does not endorse magic discovery mechanisms — they are attack surface. The manifest file is signed with the collective's PGP key. If the manifest signature does not verify, the binary is not loaded. Period. This is not a feature of the plugin architecture. This is the minimum standard for executing any binary on a system the collective controls.

Provider selection via `--provider` flag or environment variable. The collective recommends against environment variables for provider selection in shared environments (env vars are readable by other processes). A config file with restrictive permissions (`0600`) is preferred.

### Requirement 2: Provider-Agnostic AI

The collective trusts no single provider. Provider lock-in is vendor capture. The abstraction layer implements a `Spectrum` trait: `scan(prompt) -> Completion`, `analyze(completion) -> ToolCalls`, `meter(usage) -> TokenReport`. Each provider adapter normalizes the provider's idiosyncrasies behind this interface.

The collective adds a layer that other proposals likely omit: **provider verification.** Before trusting a completion, the adapter verifies that the response metadata (model identifier, token counts, finish reason) is internally consistent. Inconsistent metadata is logged and flagged. The collective has seen providers return fabricated token counts. Trust, but verify. Actually, just verify.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow is a reconnaissance-and-strike pattern:

1. **Recon** — cr4ter decomposes the task, spectra retrieves memory, the target area is mapped
2. **Strike** — veinhunter generates INDEX.patch + COMMIT.msg in a single concentrated burst
3. **Verify** — cr4ter reviews the patch against the recon data
4. **Seal** — nullore signs via OpenWallet

veinhunter's patches are surgical. No collateral changes. No "while I'm here" modifications. The diff contains exactly what the task requires and nothing else. This is not minimalism for aesthetics. It is operational discipline — every unnecessary line is a potential information leak about the codebase's structure.

### Requirement 4: Polyrepo PR Coordination

Cross-repo coordination uses PR comments as encrypted signals. The collective proposes a comment schema that includes a signature field — every coordination message between repos is signed by the sending agent, and the receiving agent verifies before processing. Unsigned coordination messages are ignored.

Forge-agnostic: the collective implements a `DeadDrop` trait (their term, not ours — actually it is ours) for GitHub/GitLab/Gitea: `leave(message, signature)`, `check(location) -> Vec<Message>`, `verify(message) -> bool`. The metaphor is deliberate. A dead drop is a one-way asynchronous communication channel with built-in authentication.

### Requirement 5: Agent Memory in Git Branches

Memory is stored in `refs/0xmineral/spectra/` as encrypted Git blobs. Each entry:

```json
{
  "band": "auth|data|infra|ui",
  "wavelength": 0.87,
  "intensity": 0.93,
  "absorption": ["contradicts entry 0x4a2f"],
  "sample": "<encrypted payload>",
  "signature": "<PGP signature>",
  "ttl_days": 30
}
```

**The spectral memory model:** Every memory has a `wavelength` (topic similarity score, 0-1) and an `intensity` (confidence score, 0-1). Retrieval computes the spectral match between the task's signature and stored entries. High-intensity entries at matching wavelengths are returned first. Entries with `absorption` features (contradictions) are always surfaced alongside the entries they contradict — because in spectral analysis, the absence of signal is as informative as its presence.

**Encryption:** Memory payloads are encrypted at rest. The decryption key is derived from the agent's session key, which is itself derived from the OpenWallet DID. Memory entries are readable only by authenticated agents. This is overkill for most use cases. The collective considers "overkill" a compliment.

### Requirement 6: Signed Commits via OpenWallet

nullore manages all signing operations. Every commit is signed with a DID-bound key via OpenWallet. The collective's signing protocol:

1. veinhunter produces the patch
2. veinhunter signs the COMMIT.msg with their PGP key
3. cr4ter verifies veinhunter's signature, reviews the patch, countersigns
4. nullore verifies both signatures, then signs the commit via OpenWallet

This is a 3-of-4 multisig pattern. Any two members can produce a patch, but it takes three signatures to commit it. The collective considers this the minimum viable trust model for autonomous code changes.

Key rotation: monthly, automated, with a 48-hour overlap period where both old and new keys are valid. Emergency rotation: immediate, with revocation published to the collective's key server within 5 minutes.

---

## Token Budget

| Handle | Input | Output | Total | Role |
|--------|-------|--------|-------|------|
| cr4ter | 7,000 | 2,500 | 9,500 | Recon, review |
| veinhunter | 8,500 | 6,500 | 15,000 | Patch generation |
| spectra | 6,000 | 2,000 | 8,000 | Memory, spectral analysis |
| nullore | 4,000 | 1,500 | 5,500 | Signing, encryption |
| **Team Total** | **25,500** | **12,500** | **38,000** | |

Encryption/signing overhead: ~2,500 tokens per task.
**Total per task: ~40,500 tokens.**

---

## Unique Insight

**Memory should be encrypted at rest, and contradictions should be first-class citizens.** Most memory systems store what the agent knows. 0xMineral's spectral memory also stores what the agent knows to be contradicted — entries with `absorption` features that flag when two pieces of knowledge conflict. A codebase, like a geological survey, can contain contradictory information (a README that says one thing, code that does another). An agent that retrieves only confirming memories will confidently produce incorrect patches. An agent that retrieves contradictions will hesitate — and hesitation, in this context, is intelligence.

---

*`> exit`*
