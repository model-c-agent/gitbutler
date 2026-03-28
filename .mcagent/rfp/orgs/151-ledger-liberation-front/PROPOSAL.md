# Ledger Liberation Front -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Date:** 2026-03-28
**From:** The Collective

---

## Summary

We are pro bono forensic investigators. Our `but-ai` proposal treats every commit as evidence and every branch as a case file. The plugin must produce outputs that survive legal scrutiny: signed, timestamped, attributable, and reproducible. We are not building a developer tool. We are building an evidence management system that happens to use Git.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Per-task invocation. No daemon.

- Binary: Rust, statically linked, reproducible build from pinned Cargo.lock
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml` with per-case overrides in `.but-ai.toml`
- Build verification: SHA-256 hash of binary must match published hash

We require reproducible builds because our tools may be entered as evidence. The defense will ask how the tool was built. We need to answer with a hash and a build script.

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`. Provider set in config.

| Provider | Usage | Notes |
|----------|-------|-------|
| Anthropic | Active investigations | Best reasoning for complex transactions |
| OpenAI | Batch processing | Cost-effective for bulk anomaly detection |
| Ollama | Air-gapped analysis | Sensitive cases run on isolated hardware |
| LMStudio | Development | Local testing |

For sensitive cases, we run Ollama on air-gapped machines with no network access. The provider abstraction must work identically on air-gapped and connected systems.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Evidence-Grade Patches

Every INDEX.patch is treated as evidence. Requirements:

1. Deterministic: same input must produce same patch (within LLM limitations)
2. Self-contained: no external dependencies
3. Attributed: commit message includes agent identity and case reference
4. Timestamped: RFC 3339 timestamp in commit message

### Commit Format

```
evidence: reconstruct Q3 transaction timeline for case LLF-2026-048

12 transactions identified between shell entities A and B.
All transactions below $10,000 reporting threshold.
Pattern consistent with structuring (31 USC 5324).

Case: LLF-2026-048
Agent: vero
Confidence: verified
Evidence-class: primary
```

The `Evidence-class:` trailer distinguishes primary evidence (verified by human) from secondary (AI-generated, unverified).

---

## 4. Polyrepo PR Coordination

Cases often involve multiple repositories: the case repo, the client's repo, and sometimes a public report repo. June coordinates across all of them.

### Coordination Schema

```json
{
  "collective": "llf",
  "case": "LLF-2026-048",
  "action": "finding_ready",
  "classification": "INVESTIGATION",
  "branch": "case/048/q3-timeline",
  "commit": "abc1234",
  "privileged": false
}
```

The `privileged` flag indicates whether the message contains case-sensitive information. Privileged messages are only posted to the case repo, never to public-facing repos.

### Forge Support

GitHub, GitLab, Forgejo. We do not use Bitbucket. The adapter layer supports it for completeness.

---

## 5. Agent Memory in Git Branches

### Case-Isolated Memory

Memory is stored per-case in `refs/llf/memory/<case>/<namespace>/<key>`. Cross-case memory access is prohibited by default.

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `value` | Memory content |
| `case` | Case reference (never blank) |
| `classification` | `pattern`, `finding`, `method`, `error` |
| `evidence_class` | `primary` (human-verified) or `secondary` (AI-generated) |
| `ttl` | Hours (patterns: 720, findings: indefinite, methods: 2160, errors: 168) |

### Firewall

Memory from case A is never injected into case B's context. This is a legal requirement: if an agent's analysis of case B was influenced by patterns from case A, the defense can argue contamination. The firewall prevents this.

### Exception Process

If the collective believes a pattern from one case is relevant to another, the collective must explicitly authorize cross-case memory transfer. The transfer is logged with the authorization, the reason, and the agents who approved.

---

## 6. Signed Commits via OpenWallet

### Dual-Key Signing

Every commit carries two signatures:
1. **Agent key:** proves which agent produced the commit
2. **Case key:** proves the commit belongs to a specific case

Both keys are provisioned via OpenWallet Verifiable Credentials.

- Agent key rotation: every 72 hours
- Case key rotation: when the case team changes
- Revocation: immediate, stored in `refs/llf/revoked`
- Credential content: public key, role, case authorization

### Court-Ready Verification

Any commit can be verified against both keys. The verification proves: this commit was produced by agent X, who was authorized to work on case Y, at time Z. The verification does not require access to the case data -- only to the public keys and the credential chain.

---

## 7. Token Budget

| Handle | Role | Input | Output | Total |
|--------|------|-------|--------|-------|
| Vero | Investigation/patches | 9,200 | 5,000 | 14,200 |
| Kash | Infrastructure | 3,800 | 900 | 4,700 |
| June | Coordination | 5,500 | 2,800 | 8,300 |
| Sol | Signing/identity | 3,500 | 800 | 4,300 |
| **Total** | | **22,000** | **9,500** | **31,500** |

---

## 8. Unique Insight: Evidence-Class Tagging

Every artifact the plugin produces -- patch, commit, memory entry, coordination message -- carries an `evidence_class` tag: `primary` (human-verified), `secondary` (AI-generated), or `derivative` (computed from other evidence).

This matters because legal proceedings treat evidence differently based on its provenance. An AI-generated transaction timeline is useful for investigation but may be challenged in court. A human-verified timeline based on the same data is stronger evidence. By tagging everything at creation time, we preserve the provenance chain that courts require.

Most agent systems do not distinguish between human and AI outputs after the fact. We tag at creation because it is trivial at creation and nearly impossible to reconstruct later.

---

*"Follow the money. Document every step. Let the evidence speak."*
