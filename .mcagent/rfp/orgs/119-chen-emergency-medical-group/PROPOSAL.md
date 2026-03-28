# Chen Emergency Medical Group — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation centered on **generational institutional memory and earned autonomy**. Agents accumulate knowledge through experience and earn progressively less human oversight as they demonstrate competence. Our memory system spans multiple generations of patterns — some of our encoded knowledge is fifty years old and still clinically relevant. We build lean, ship carefully, and review everything at dinner.

---

## Requirement 1: PATH-based Plugin Architecture

Simple. No unnecessary features. Every command earns its place.

**Design:**
- Binary: `but-ai`, statically linked
- Commands: `but ai patch`, `but ai memory`, `but ai review` (trigger M&M-style case review of a recent run), `but ai autonomy` (check/adjust agent autonomy level)
- Config: `~/.config/but-ai/chen.toml`
- `but ai review <run-id>` produces a structured case review in M&M format: What happened? What was the outcome? What should change?
- `but ai autonomy show` displays current autonomy level per task type

---

## Requirement 2: Provider-Agnostic AI

Pragmatic abstraction. Works with what is available.

**Architecture:**
- Provider trait: simple invoke/stream interface
- No speculative execution, no A/B testing — too expensive for a three-person team
- Provider selection: config-driven, single active provider, manual fallback
- Supported: OpenAI, Anthropic, Ollama, LMStudio
- Provider evaluation: quarterly manual review of output quality, not automated scoring

We do not over-engineer the provider layer. It is a function call. It returns a response. If the provider changes, we update the adapter.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Patches with **earned autonomy** — agents start supervised and earn independence.

**Autonomy levels:**
1. **Intern** — Every patch requires human review before commit
2. **Resident** — Patches on familiar task types auto-commit; novel tasks require review
3. **Attending** — All patches auto-commit; human review asynchronous
4. **Consultant** — Agent can propose task decomposition and execute independently

**Promotion criteria:** An agent at level N is promoted to level N+1 when its success rate on tasks at level N exceeds 95% over 20 consecutive tasks. Demotion occurs when success rate drops below 80% over 10 consecutive tasks.

**Patch workflow:** Standard INDEX.patch + COMMIT.msg. Autonomy level determines whether the commit is automatic or queued for review.

---

## Requirement 4: Polyrepo PR Coordination

Straightforward coordination. No novel protocol.

**Protocol:**
- PR comments: `<!-- chen:coord:{action}:{payload} -->`
- Actions: `propose`, `ack`, `ready`, `merge`
- No fancy signaling. No encoded payloads. Plain JSON in the comment.

**Forge adapters:** GitHub, GitLab, Gitea. Standard trait. We build what we need and nothing more.

**Analytics:** Kevin's contribution — every coordination event is logged with timestamps. Quarterly analysis identifies bottlenecks. Results reviewed at dinner.

---

## Requirement 5: Agent Memory in Git Branches

Memory organized as **M&M case studies** with generational provenance.

**Memory structure:**
```toml
[case]
key = "api-error-convention"
generation = "current"  # or "lisa" or "wei"
presentation = "Inconsistent error handling in API layer"
diagnosis = "No shared error type; each handler defines its own"
treatment = "Use Result<T, AppError> with From impls for all error sources"
outcome = "Applied in 3 subsequent tasks, 100% success"
confidence = 0.93
last_reviewed = "2026-03-24"
```

**Storage:** `refs/but-ai/memory/<generation>/<key>`

**Generational provenance:** Patterns identified by previous generations are tagged accordingly. A `wei`-generation pattern (identified from 1970s data) carries historical weight. A `current`-generation pattern is newer and less proven.

**Review cycle:** All memory entries are reviewed at the weekly M&M dinner (or equivalent automated review). Entries not referenced in 60 days are flagged for retirement.

---

## Requirement 6: Signed Commits via OpenWallet

Standard signing. No complications.

**Implementation:**
- OpenWallet-managed keys
- Per-commit signing
- 30-day key rotation
- Revocation on compromise
- VC includes: agent identity, autonomy level at time of signing, memory entries consulted

**The autonomy level in the VC** is our addition: a reviewer can see not just who signed the commit but what level of oversight the agent was operating under when it produced the patch.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| David | Patch/Provider | 9,200 | 4,500 | 13,700 |
| Michelle | Memory/Review | 6,000 | 800 | 6,800 |
| Kevin | Coordination | 6,200 | 2,200 | 8,400 |
| **Total** | | **21,400** | **7,500** | **28,900** |

### Scaling

| Type | Multiplier | Budget |
|------|-----------|--------|
| Quick fix | 0.4x | 11,560 |
| Standard feature | 1.0x | 28,900 |
| Multi-repo | 1.8x | 52,020 |
| Architecture | 2.5x | 72,250 |

---

## Unique Insight: Earned Autonomy as Progressive Trust

Every other proposal either grants agents full autonomy from the start or requires human review for every action. Both approaches are wrong. Full autonomy is reckless for unproven agents. Mandatory review is unsustainable at scale.

Our system starts agents as interns: every output is reviewed. As the agent demonstrates competence on specific task types, it earns progressively less oversight — exactly the way a medical trainee earns independence. A third-year resident handles routine cases without attending supervision because they have proven, through hundreds of observed cases, that they can.

The key insight is that autonomy is not binary and it is not global. An agent can be highly autonomous on error-handling tasks (where it has a 98% success rate) and fully supervised on API design tasks (where it has a 72% success rate). Autonomy is earned per-domain, based on evidence, and revocable when performance degrades.

Three generations of emergency physicians taught us this: trust is earned in the ER, not granted by a badge. The same applies to AI agents.

---

*Submitted by the Chen Emergency Medical Group, San Francisco.*
*"What did we learn?"*
