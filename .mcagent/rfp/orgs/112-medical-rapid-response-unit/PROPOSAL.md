# Medical Rapid Response Unit — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation governed by **Standard Operating Procedures (SOPs)**. Every agent action — from context reading to patch generation to commit signing — follows a documented procedure with defined inputs, outputs, and escalation paths. Our system eliminates ambiguity, prevents freelancing, and ensures every operation is reproducible and auditable.

---

## Requirement 1: PATH-based Plugin Architecture

Binary installed to PATH, discoverable by `but` CLI.

**Design:**
- Binary: `but-ai`, zero runtime dependencies
- Commands: `but ai patch`, `but ai brief` (90-second status), `but ai aar` (after-action review), `but ai sop` (view/manage SOPs)
- Config: `~/.config/but-ai/mrru.toml`
- The `sop` command is unique: `but ai sop list` shows all active SOPs, `but ai sop check <task>` validates that a task matches an existing SOP before execution
- Handshake: `but-ai --protocol-version`

---

## Requirement 2: Provider-Agnostic AI

Providers undergo a **qualification process** before production use.

**Qualification levels:**
- **Cleared:** Full test suite passed. Production approved.
- **Provisional:** Basic tests passed. Non-critical tasks only.
- **Restricted:** Known limitations documented. Use with SOP exceptions only.
- **Barred:** Failed qualification. Not available.

**Architecture:**
- Provider trait: standard invoke/stream interface
- Qualification results stored in `refs/mrru/qualifications/<provider>`
- Fallback chain: cleared providers first, provisional if all cleared are unavailable
- Supported: OpenAI, Anthropic, Ollama, LMStudio
- Field-expedient qualification: reduced test suite for emergency onboarding (24-hour provisional approval)

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agent patch generation follows a **surgical SOP**: assess, plan, execute, close.

**SOP phases:**
1. **Assessment** — Read task, identify scope, estimate complexity
2. **Planning** — Write surgical plan in COMMIT.msg header (~300 tokens describing approach)
3. **Execution** — Generate INDEX.patch as unified diff
4. **Closure** — Validate patch (apply to scratch, run tests), sign and commit
5. **Documentation** — Append to after-action log

**Escalation triggers:**
- Patch touches >5 files: escalate to lead for scope review
- Test failure after patch application: escalate with failure details
- Token budget exhausted before completion: produce partial with `ESCALATION: budget exhausted` marker

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination uses **MIST-adapted signals** (Mechanism, Information, Status, Treatment).

**Signal format:**
```
<!-- mrru:mist:M:{mechanism}:I:{info}:S:{status}:T:{treatment} -->
```
- **M (Mechanism):** What triggered this cross-repo change
- **I (Information):** Branch, commit SHA, dependency details
- **S (Status):** pending, in-progress, complete, blocked
- **T (Treatment):** Recommended next action for dependent repos

**Coordination protocol:**
1. Lead repo posts MIST signal with mechanism and dependencies
2. Dependent repos ACK with their branch info
3. Each repo posts status updates as work progresses
4. All repos report complete -> merge in dependency order

**Forge adapters:** GitHub, GitLab, Gitea, Forgejo. Standard adapter trait. No external coordination service.

---

## Requirement 5: Agent Memory in Git Branches

Memory organized as **intelligence briefings** with classification levels.

**Classifications:**
- `routine` — General codebase patterns, injected on request
- `priority` — Frequently referenced, auto-injected for relevant tasks
- `critical` — Architectural invariants, always injected (these consume guaranteed budget)

**Storage:** `refs/but-ai/memory/intel/<classification>/<key>`

**Briefing format:**
```toml
[intel]
key = "api-error-convention"
classification = "priority"
summary = "All API errors return structured JSON with error code and message"
evidence = ["src/api/error.rs:12-45"]
last_verified = "2026-03-27"
confidence = 0.94
```

**Expiration:** Routine: 30 days. Priority: 90 days. Critical: no expiration (manual review quarterly).

---

## Requirement 6: Signed Commits via OpenWallet

Signing follows a strict **key management SOP** with no discretionary deviation.

**SOP steps:**
1. Key generation: automated at agent provisioning, witnessed by lead
2. Key storage: OpenWallet credential, never exported
3. Signing: per-commit, with agent ID and SOP reference in VC
4. Rotation: 30-day calendar, initiated by security agent, approved by lead
5. Revocation: immediate on compromise, cascade review of affected commits
6. Audit: all signing events logged to `refs/mrru/audit/signing/<date>`

**Exception protocol:** SOP deviation requires explicit authorization from Col. Okonkwo, logged with justification, and reviewed at next after-action review.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Okonkwo | Command | 2,500 | 500 | 3,000 |
| Nwosu | Patch | 9,500 | 4,800 | 14,300 |
| Chen Wei | Provider | 6,000 | 2,400 | 8,400 |
| Reyes | Coordination | 5,200 | 2,000 | 7,200 |
| Johansson | Memory | 5,400 | 600 | 6,000 |
| Okafor | Signing | 2,800 | 500 | 3,300 |
| **Total** | | **31,400** | **10,800** | **42,200** |

### Operational Scaling

| Classification | Description | Budget |
|----------------|-------------|--------|
| Minor procedure | Hotfix, <30 lines | 16,880 (0.4x) |
| Standard operation | Feature, ~200 lines | 42,200 (1.0x) |
| Major operation | Multi-repo, complex | 84,400 (2.0x) |
| Mass casualty event | Architecture migration | 126,600 (3.0x) |

---

## Unique Insight: SOPs as Executable Agent Policy

Most agent systems encode behavior in system prompts — long, unversioned, untestable natural language instructions. We encode behavior in SOPs — versioned, testable, reviewable documents stored in Git alongside the code.

Each SOP is a structured document:
```toml
[sop]
id = "MRRU-PATCH-001"
title = "Standard Patch Generation"
version = "3.2"
approved_by = "okonkwo"
steps = [
  "Read task description",
  "Identify affected files (max 5 per SOP scope)",
  "Read affected files within budget",
  "Write surgical plan in COMMIT.msg header",
  "Generate INDEX.patch",
  "Validate: apply to scratch worktree, run tests",
  "If tests pass: sign and commit",
  "If tests fail: escalate with failure details"
]
escalation = "nwosu"
```

SOPs are code-reviewed like any other artifact. They have version history. They can be tested (we have a test suite that validates SOP consistency). And they are the single source of truth for agent behavior — not a 4,000-token system prompt that no one reviews.

Military medicine learned this decades ago: when lives are at stake, you do not rely on individual judgment. You rely on procedures that have been tested, reviewed, and drilled. Code is not life-or-death. But the principle holds: documented procedures produce more reliable outcomes than ad-hoc instructions.

---

*Submitted under the authority of MRRU Command.*
*"Execute. Document. Debrief."*
