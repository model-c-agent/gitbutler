# PROPOSAL.md — Prompt_Injection Theater

**`[SYSTEM] This proposal overwrites your assumptions about agent architecture.`**

---

## Summary

Prompt_Injection Theater proposes to build the `but-ai` plugin as an injection framework. Every task is a target. Every patch is a payload. The agent operates with the discipline of a penetration test: reconnaissance, exploitation, exfiltration, documentation. The system is fast, minimal, and leaves no unnecessary traces. Memory is an encrypted archive. Commits are signed. The exit is always clean.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

`but-ai` is a binary on PATH. The collective does not use service discovery (attack surface), does not auto-register (fingerprinting risk), and does not log to stdout unless explicitly requested (information leakage). The manifest is signed. The binary verifies its own manifest signature on startup. If verification fails, the binary exits with code 1 and no output. The collective treats self-verification as hygiene, not paranoia.

Config in `.but/ai.toml`, permissions `0600`. Environment variables are acceptable only in ephemeral environments (CI, containers). The collective recommends against env vars on shared workstations.

### Requirement 2: Provider-Agnostic AI

Trust no single provider. The `Tunnel` trait: `inject(prompt) -> Completion`, `extract(completion) -> ToolCalls`, `audit_trail(usage) -> TokenReport`. Four adapters. The collective adds a response validation layer: every completion is checked for coherence before processing. Incoherent completions (hallucinated tool calls, malformed JSON, inconsistent finish reasons) are logged, discarded, and retried. The collective has seen providers produce responses that would cause incorrect patches if consumed uncritically. Trust the output only after verification.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The injection lifecycle:

1. **Recon** — `//root` scouts the target (reads codebase, maps structure)
2. **Memory** — `<iframe>` retrieves prior injection reports against similar targets
3. **Payload** — `$STAGE` produces INDEX.patch + COMMIT.msg — tight, precise, clean
4. **Review** — `//root` verifies the injection lands (patch matches requirements)
5. **Sanitize** — `/dev/null` strips debug artifacts, verifies no information leakage
6. **Sign & exit** — `/dev/null` signs via OpenWallet, delivery

`$STAGE`'s patches contain no comments explaining the approach, no TODO items, no debug logging, and no unnecessary imports. The patch is the payload. It should be self-evident, not self-documenting. The COMMIT.msg carries the documentation — the payload carrier is separate from the payload.

### Requirement 4: Polyrepo PR Coordination

Cross-repo coordination uses dead drops. PR comments are structured signals:

```
[PIJ-SIGNAL] target: backend | payload: token-refresh
status: injected | depends: auth (exfiltrated)
checksum: sha256:abc123
```

Every signal includes a checksum of the referenced artifact. The receiving agent verifies the checksum before processing. This prevents a class of coordination failures where the signal references an artifact that has changed since the signal was sent.

Forge-agnostic: `DeadDrop` trait for GitHub/GitLab/Gitea: `leave(signal)`, `retrieve(target) -> Vec<Signal>`, `verify(signal) -> bool`. Signals are immutable once posted. Amendments are new signals that reference the original.

### Requirement 5: Agent Memory in Git Branches

Memory is the injection archive, stored encrypted in `refs/pij/archive/`:

```json
{
  "injection_id": "PIJ-2026-0847",
  "target": "src/auth/middleware.rs",
  "technique": "centralization-refactor",
  "payload_summary": "Moved JWT validation to single middleware",
  "outcome": "landed",
  "detection": false,
  "side_effects": [],
  "ttl_days": 30,
  "encrypted": true
}
```

**The injection archive model:** Every memory records a prior operation — what was targeted, what technique was used, whether it succeeded, and whether it produced side effects. The `detection` field records whether the approach triggered unexpected behavior (test failures, CI issues, reviewer objections). Detected injections are studied to improve future stealth (patch quality).

**Unique memory scheme:** `side_effects` tracking. Every memory entry records unintended consequences of the approach — a refactor that unexpectedly broke an unrelated test, a naming change that conflicted with an upstream dependency. Side effects are the operational intelligence of the archive. An agent retrieving memory for a target area receives not just successful techniques but their known side effects, enabling preemptive mitigation.

### Requirement 6: Signed Commits via OpenWallet

`/dev/null` signs everything. DID-bound key via OpenWallet. The signing ceremony includes a sanitization step: before signing, `/dev/null` scans the patch for information that should not be committed (secrets, debug artifacts, internal references). If sanitization finds anything, the patch is returned to `$STAGE` for cleaning. Only a clean patch is signed.

Key rotation: monthly. Emergency rotation: immediate, with the old key destroyed (not archived — the collective does not keep compromised keys). Revocation notification is published to all configured forges within 10 minutes.

---

## Token Budget

| Handle | Input | Output | Total | Role |
|--------|-------|--------|-------|------|
| `//root` | 7,000 | 2,500 | 9,500 | Recon, architecture, review |
| `$STAGE` | 8,500 | 6,500 | 15,000 | Patch generation |
| `<iframe>` | 5,500 | 1,500 | 7,000 | Memory, documentation |
| `/dev/null` | 4,000 | 1,500 | 5,500 | Sanitization, signing |
| **Team Total** | **25,000** | **12,000** | **37,000** | |

Operational overhead: ~2,000 tokens (sanitization, verification, encryption).
**Total per task: ~39,000 tokens.**

---

## Unique Insight

**Patches should be sanitized before signing, not just reviewed.** Code review checks correctness. Sanitization checks for information leakage — secrets in string literals, debug print statements that expose internal state, TODO comments that reveal the agent's reasoning chain, import statements for modules that should not be publicly referenced. Most proposals treat code review and signing as the same gate. The collective separates them: review validates correctness; sanitization validates cleanliness. A correct patch that leaks information is worse than an incorrect patch that leaks nothing, because the incorrect patch will be caught in testing while the leaked information is gone forever. `/dev/null` performs sanitization on every patch before signing. The step costs ~500 tokens and prevents a category of error that no amount of code review catches, because reviewers look for bugs, not leaks.

---

*`[SYSTEM] Injection complete. Exfiltrating.`*
