# Crash Cart Collective — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation built on **local-first, anti-lock-in principles**. The system runs entirely on local hardware by default, uses cloud providers only as an opt-in fallback, and never creates dependencies on external services that could be revoked, rate-limited, or surveilled. Every component can be self-hosted. Every artifact can be verified locally.

---

## Requirement 1: PATH-based Plugin Architecture

Single binary. Zero cloud dependencies at startup.

**Design:**
- Binary: `but-ai`, statically linked, no network calls at startup
- Commands: `but ai patch`, `but ai unlock` (diagnose and remove artificial limitations), `but ai memory`, `but ai audit`
- Config: `~/.config/but-ai/crashcart.toml`
- `but ai unlock` is unique to our proposal: it analyzes the current `but-ai` configuration and identifies unnecessary restrictions — rate limits, feature gates, provider locks — and suggests or applies configuration changes to remove them
- All config is local. No remote config fetching.

---

## Requirement 2: Provider-Agnostic AI

Local-first provider architecture. Cloud is the fallback, not the default.

**Default priority:**
1. Ollama (local)
2. LMStudio (local)
3. Anthropic (cloud, anonymized)
4. OpenAI (cloud, anonymized)

**Anonymization layer:** All prompts sent to cloud providers are stripped of:
- Repository paths (replaced with hashes)
- Author names (replaced with pseudonyms)
- Organization identifiers
- File paths beyond the filename

**Architecture:**
- Provider trait: standard invoke/stream
- Local-first by default; cloud opt-in via config
- No provider telemetry. No usage reporting. No analytics callbacks.
- Quality gate: if local model output self-scores below 60%, cloud provider is used automatically (with anonymization)

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce surgical patches — minimal, precise, single-purpose.

**Patch philosophy:** Every patch should do one thing. If a task requires multiple changes, it produces multiple patches, each with its own COMMIT.msg. This mirrors our firmware work: a single patch unlocks one lock. If the device has three locks, you apply three patches.

**Workflow:**
1. Analyze task — decompose into atomic changes
2. For each atomic change:
   a. Read relevant context
   b. Generate INDEX.patch
   c. Generate COMMIT.msg with edge-case checklist
   d. Validate (apply to scratch, run tests)
   e. Commit if passing
3. If any atomic change fails, preceding changes are preserved (no rollback of successful patches)

**Edge-case checklist:** Every COMMIT.msg includes:
```
Edge-Cases-Checked:
- [ ] Error path handled
- [ ] Null/empty input handled
- [ ] Concurrent access considered
```

---

## Requirement 4: Polyrepo PR Coordination

Minimal coordination protocol. Encoded for privacy.

**Protocol:**
- PR comments: `<!-- cc:{action}:{base64_payload} -->`
- Payload is base64 to prevent casual scraping
- Actions: `propose`, `ack`, `ready`, `merge`
- Plaintext summary included for debugging: `<!-- cc:summary:{text} -->`

**Forge adapters:** Gitea (primary), GitHub, GitLab. Gitea preferred for self-hosted deployments.

**Self-hosted coordination:** The system supports a fully self-hosted coordination mode where no data touches third-party forges. All coordination happens via Git refs in a shared coordination repo.

---

## Requirement 5: Agent Memory in Git Branches

Memory as **lock/unlock pattern pairs**.

**Structure:** Each memory entry describes a pattern (the "lock") and a recommended approach (the "unlock"):

```toml
[pattern]
hash = "c4f2a8..."
lock = "Error handling in this module uses panic! for recoverable errors"
unlock = "Replace panic! with Result<T, E> propagation using ? operator"
evidence = ["src/handlers/mod.rs:42", "src/handlers/auth.rs:18"]
confidence = 0.88
observations = 4
```

**Storage:** `refs/but-ai/memory/<category>/<content-hash>`
- Categories: `conventions`, `anti-patterns`, `architecture`, `dependencies`
- Content-addressed: keys are hashes, not names (no metadata leakage)

**Expiration:** Ephemeral: 1 hour. Task: 7 days. Established: 90 days. Permanent: requires 3-of-5 consensus.

---

## Requirement 6: Signed Commits via OpenWallet

Pseudonymous signing with hardware token support.

**Design:**
- Signing keys generated on air-gapped machines when possible
- Hardware token support (YubiKey, Nitrokey)
- Pseudonymous identity: signatures are verifiable without revealing real identity
- Key rotation: per flatline's undisclosed schedule (minimum every 21 days)
- Revocation: immediate, with revocation notice in `refs/crashcart/revocations/`

**Anti-tampering:** Signed commits include a hash of the audit trail entry for that run. Modifying the audit trail after signing invalidates the commit signature.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Handle | Role | Input | Output | Total |
|--------|------|-------|--------|-------|
| sparks | Patch | 8,500 | 3,800 | 12,300 |
| flatline | Security | 3,500 | 900 | 4,400 |
| paddles | Provider | 5,500 | 2,000 | 7,500 |
| rhythm | Memory | 5,200 | 600 | 5,800 |
| joules | Coordination | 5,000 | 1,800 | 6,800 |
| **Total** | | **27,700** | **9,100** | **36,800** |

### Scaling

| Type | Multiplier | Budget |
|------|-----------|--------|
| Single lock (hotfix) | 0.3x | 11,040 |
| Standard unlock (feature) | 1.0x | 36,800 |
| Multi-device (multi-repo) | 1.8x | 66,240 |
| Full firmware (architecture) | 2.5x | 92,000 |

---

## Unique Insight: Anti-Lock-In as an Architectural Principle

Every AI agent system we have evaluated creates lock-in — to a cloud provider, to a specific model, to a coordination service, to a signing infrastructure. Lock-in is the DRM of developer tools: it restricts what you can do with your own system for the benefit of a vendor.

Our architecture has zero lock-in by design:
- **Provider:** Local-first. Cloud is opt-in. Switch providers by changing one config line.
- **Forge:** Self-hostable. No dependency on GitHub or any specific platform.
- **Memory:** Stored in Git refs. Portable to any Git host. No external database.
- **Signing:** Standard OpenWallet credentials. No proprietary signing service.
- **Coordination:** Protocol is in PR comments. Works on any forge that supports comments.

We have spent five years removing artificial locks from medical devices. We know what lock-in looks like, how it is implemented, and how to design systems that avoid it. An AI agent system that requires a specific cloud provider to function is a defibrillator that requires a subscription to save lives.

We do not build those systems. We unlock them.

---

*No signature. Verify the key. Apply the patch.*
