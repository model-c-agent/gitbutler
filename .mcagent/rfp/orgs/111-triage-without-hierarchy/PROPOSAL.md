# Triage Without Hierarchy — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation built on **proximity-weighted consensus**: the agent with the most relevant context has the highest authority, regardless of its assigned role. Our system has no orchestrator, no lead agent, and no fixed hierarchy. Authority is dynamic, contextual, and always auditable.

---

## Requirement 1: PATH-based Plugin Architecture

Standard PATH-based binary with triage-inspired tooling.

**Design:**
- Binary: `but-ai`, statically linked
- Commands: `but ai triage` (assess task complexity), `but ai patch`, `but ai memory`, `but ai status`
- Config: `~/.config/but-ai/twh.toml`
- The `triage` command is unique: it assesses a task before execution, estimating complexity, token budget, and recommended agent allocation — like a field triage assessment before treatment
- Plugin handshake via `but-ai --protocol-version`

---

## Requirement 2: Provider-Agnostic AI

Providers are treated as available resources, triaged and routed based on current capability.

**Triage categories for providers:**
- **Green (stable):** Low latency, high accuracy, within budget — use normally
- **Yellow (degraded):** Elevated latency or reduced accuracy — use with monitoring
- **Red (critical):** Failing or over budget — failover to backup
- **Black (unavailable):** Down or revoked — skip entirely

**Architecture:**
- Provider trait with capability reporting
- Real-time triage scoring based on last N calls
- Automatic promotion/demotion between categories
- Supported: OpenAI, Anthropic, Ollama, LMStudio
- Selection: highest-triaged available provider, overridable by config

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce patches using a **primary and secondary survey** model adapted from trauma assessment.

**Primary survey (fast):**
1. Read task description
2. Identify affected files
3. Generate INDEX.patch
4. Produce COMMIT.msg

**Secondary survey (thorough):**
5. Re-read the patch in context of surrounding code
6. Check for missed imports, type errors, convention violations
7. Revise patch if issues found
8. Final commit

The secondary survey consumes ~25% additional tokens but catches errors that the primary survey misses 18% of the time in our testing. It is optional, controlled by `--thorough` flag, and enabled by default for patches touching >3 files.

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination via a **mutual aid** protocol. Repos are independent responders that signal their status and needs.

**Signal protocol:**
```
<!-- twh:signal:{type}:{json_payload} -->
```
- `status` — Current state: working, ready, blocked
- `need` — What this repo requires from others
- `offer` — What this repo can provide
- `block` — What is preventing progress

**Coordination flow:**
1. Lead repo posts `need` signal listing dependencies
2. Dependent repos post `status` updates as they work
3. Each repo posts `offer` when its changes are ready
4. Lead repo posts merge signal when all `offer` signals received

**Forge adapters:** GitHub, GitLab, Gitea, Forgejo. Adapter trait abstracts comment posting and reading.

---

## Requirement 5: Agent Memory in Git Branches

Memory organized as **patient records** — structured, system-based, proximity-scored.

**Systems (namespaces):**
- `refs/but-ai/memory/conventions/` — Coding style, naming patterns
- `refs/but-ai/memory/architecture/` — Structural patterns, module boundaries
- `refs/but-ai/memory/failures/` — Past errors, anti-patterns
- `refs/but-ai/memory/reviews/` — Feedback from code reviews

**Proximity scoring:** When an agent retrieves memory, entries are scored by proximity to the agent's current context. Proximity factors:
- File overlap (memories about files the agent is modifying score highest)
- Directory adjacency (memories about sibling modules score medium)
- Project-wide (general patterns score lowest)

Top-5 entries injected. Proximity scores logged in audit trace.

**Expiration:** Session: 1 hour. Task: 7 days. System: 60 days. Permanent: manual, requires 4-of-6 consensus.

---

## Requirement 6: Signed Commits via OpenWallet

Chain-of-custody signing. Every commit is sealed with contextual metadata.

**Signing metadata (stored in VC):**
- Agent identity (pseudonymous)
- Proximity score at time of signing (how much relevant context the signing agent held)
- Run ID (links to full audit trace)
- Memory entries consulted
- Provider used

**Key lifecycle:**
- Provisioning: automated at agent creation
- Rotation: 30 days or 300 commits
- Revocation: immediate, with cascade flag on affected commits
- Emergency: any agent can trigger emergency revocation if anomaly detected

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Kai | Consensus | 6,500 | 1,500 | 8,000 |
| Eleni | Patch | 9,200 | 4,500 | 13,700 |
| Marcus | Provider | 4,800 | 1,800 | 6,600 |
| Zara | Memory | 5,600 | 700 | 6,300 |
| Jun | Forge | 5,400 | 2,100 | 7,500 |
| Reva | Signing | 3,200 | 600 | 3,800 |
| **Total** | | **34,700** | **11,200** | **45,900** |

### Triage-Based Scaling

| Triage Category | Description | Budget |
|-----------------|-------------|--------|
| Green (minor) | Single file, simple change | 18,400 (0.4x) |
| Yellow (moderate) | Standard feature | 45,900 (1.0x) |
| Red (critical) | Multi-repo, complex | 91,800 (2.0x) |
| Mass casualty | Architecture overhaul | 114,750 (2.5x) |

---

## Unique Insight: Dynamic Authority Based on Context Proximity

Every agent orchestration system we have seen assigns authority statically: there is a "lead" agent, a "coordinator," a "supervisor." This mirrors the medical hierarchy that fails in disaster zones — when the designated authority is unavailable or lacks context, the system freezes.

Our proximity-weighted model assigns authority dynamically. The agent that has consumed the most relevant context for the current decision has the highest weight. This means authority shifts throughout a task. During context reading, the memory agent has highest weight (it knows what patterns exist). During patch generation, the patch agent has highest weight (it has read the code). During signing, the security agent has highest weight.

No agent is permanently in charge. No agent is permanently subordinate. Authority follows information, not title.

This is how the best emergency rooms actually work — the person closest to the patient makes the call, regardless of rank. We built a system that works the same way.

---

*"Proximity is authority. Context is rank."*
