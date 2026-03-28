# The Tunnelers' Free Assembly — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation built on **cooperative governance and safety-first constraints**. Every agent operates within safety invariants that cannot be overridden without explicit human consent. Decisions are distributed, authority is shared, and no single agent can make changes that affect the safety properties of the system without review by the safety gate.

---

## Requirement 1: PATH-based Plugin Architecture

Robust binary for constrained environments.

**Design:**
- Binary: `but-ai`, statically linked, optimized for low-bandwidth environments
- Commands: `but ai patch`, `but ai safety-check` (validate constraints), `but ai memory`, `but ai vote` (propose and record team decisions)
- Config: `~/.config/but-ai/tfa.toml`
- `but ai safety-check <patch>` validates a patch against defined safety invariants before commit
- `but ai vote <proposal>` records a team decision with member votes (integrated into the development workflow)

---

## Requirement 2: Provider-Agnostic AI

Provider layer built for **satellite-constrained connectivity**.

**Architecture:**
- Provider trait: standard invoke/stream with bandwidth-awareness
- Bandwidth estimator: measures available throughput and adjusts provider selection
- Low-bandwidth mode: local models (Ollama) for routine tasks, cloud for complex tasks only
- Request batching: multiple small requests consolidated into single API calls where possible
- Aggressive caching with staleness tracking
- Supported: Ollama (primary), LMStudio, Anthropic, OpenAI

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Patches pass through a **safety gate** before commit.

**Safety invariant system:**
- Invariants defined in `.but-ai/safety.toml`:
  ```toml
  [[invariant]]
  id = "SAFETY-001"
  rule = "never_delete_validation_function"
  scope = "src/validation/"
  severity = "critical"

  [[invariant]]
  id = "SAFETY-002"
  rule = "never_bypass_auth_check"
  scope = "src/auth/"
  severity = "critical"
  ```
- Before commit, every patch is checked against all invariants
- Critical invariant violations block the commit entirely
- Warning-level violations flag the commit for human review

**Workflow:**
1. Read task and context
2. Generate INDEX.patch + COMMIT.msg
3. Safety gate: check patch against invariants
4. If pass: commit
5. If fail (critical): reject with explanation
6. If fail (warning): queue for human review

---

## Requirement 4: Polyrepo PR Coordination

Cooperative-style coordination with **transparent decision records**.

**Protocol:**
- PR comments: `<!-- tfa:coord:{action}:{payload} -->`
- Actions: `propose`, `vote`, `approved`, `merge`
- Cross-repo changes require documented approval from agents in all affected repos
- Approval records include which agents voted and how

**Forge adapters:** GitHub, GitLab, Gitea. Self-hosted Gitea preferred for cooperative sovereignty.

---

## Requirement 5: Agent Memory in Git Branches

Memory as **field reports** — observations from practice, translated into patterns.

**Memory types:**
- `field-report` — Direct observations from code analysis or task execution
- `safety-lesson` — Patterns learned from failures (never auto-expired)
- `convention` — Coding style and architectural patterns
- `cooperative-decision` — Recorded votes and their rationale

**Storage:** `refs/but-ai/memory/<type>/<key>`

**Field report format:**
```toml
[report]
key = "handler-error-pattern"
observer = "reka"
observation = "All handler errors must propagate via Result, never panic"
context = "Discovered during ventilation optimizer refactor"
confidence = 0.95
confirmed_by = ["tomasz", "maria"]
```

**Safety lessons never expire.** All other memory types have configurable TTL (default: 60 days for field reports, 90 days for conventions).

---

## Requirement 6: Signed Commits via OpenWallet

Signing with **cooperative accountability**.

**Design:**
- All commits signed via OpenWallet credentials
- Each agent's signing key is registered with the cooperative
- Key rotation requires recorded vote (quarterly pre-authorization)
- Revocation requires majority consent
- VC includes: agent identity, safety gate result (pass/warn/fail), and vote reference if applicable

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Reka | Patch | 9,000 | 4,200 | 13,200 |
| Tomasz | Safety | 4,500 | 1,200 | 5,700 |
| Maria | Memory | 5,600 | 700 | 6,300 |
| Eli | Provider | 5,800 | 2,200 | 8,000 |
| Donna | Signing | 3,200 | 600 | 3,800 |
| **Total** | | **28,100** | **8,900** | **37,000** |

### Scaling

| Level | Description | Budget |
|-------|-------------|--------|
| Surface work (trivial) | <30 lines | 14,800 (0.4x) |
| Standard drift (feature) | ~200 lines | 37,000 (1.0x) |
| Deep shaft (multi-repo) | Cross-repo | 74,000 (2.0x) |
| New seam (architecture) | Breaking changes | 92,500 (2.5x) |

---

## Unique Insight: Safety Invariants as First-Class Citizens

Most agent systems treat safety as a property of the test suite — if tests pass, the change is safe. But tests only check what they test. A test suite that does not include a test for "never delete the authentication check" will not catch an agent that deletes the authentication check.

Our safety invariant system is separate from and complementary to the test suite. Invariants are declarative rules about what the codebase must always do (or never do), defined by humans, checked automatically, and enforced unconditionally. They operate at the diff level — checking what the patch changes — not at the behavior level.

We learned this from mining. Safety regulations in a mine are not "tests you run after blasting." They are invariants that are checked before blasting: gas levels below threshold, ventilation running, escape routes clear. You do not blast and then check if the air is breathable. You check the air and then decide whether to blast.

Agent patches should be treated the same way: check the safety invariants before applying, not after.

---

*Submitted by The Tunnelers' Free Assembly.*
*"One miner, one vote."*
