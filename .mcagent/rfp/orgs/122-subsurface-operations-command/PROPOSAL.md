# Subsurface Operations Command — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

*Operation Order Reference: OPORD-BUT-AI-2026-001*

---

## Executive Summary

We propose a `but-ai` implementation governed by **operation orders** — detailed, pre-approved plans that specify exactly what each agent will do before execution begins. Our system separates planning from execution. The planning phase produces a structured plan. The execution phase follows the plan. Deviations are handled at predefined decision points, not ad hoc.

---

## Requirement 1: PATH-based Plugin Architecture

Disciplined binary with planning-first tooling.

**Design:**
- Binary: `but-ai`, statically linked
- Commands: `but ai plan` (generate operation order), `but ai execute` (execute plan), `but ai intel` (memory/analysis), `but ai sitrep` (status report)
- Config: `~/.config/but-ai/soc.toml`
- `but ai plan <task>` produces a structured operation order before any code is generated
- `but ai execute <plan-ref>` executes the plan, producing patches according to the order
- `but ai sitrep` reports current operation status: phase, progress, blockers

---

## Requirement 2: Provider-Agnostic AI

Provider selection follows **PACE methodology** (Primary, Alternate, Contingency, Emergency).

**PACE plan per task:**
- **Primary:** Configured default provider (e.g., Anthropic)
- **Alternate:** Second cloud provider (e.g., OpenAI)
- **Contingency:** Local model (Ollama)
- **Emergency:** Cached responses from previous runs

**Architecture:**
- Provider trait: standard invoke/stream
- PACE selection automated: if primary fails, cascade to alternate, then contingency, then emergency
- Each PACE transition logged with justification
- Supported: OpenAI, Anthropic, Ollama, LMStudio

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Execution follows the **operation order** produced in the planning phase.

**Operation order format:**
```toml
[operation]
id = "OPORD-2026-042"
objective = "Add retry logic to API client"
phases = [
  { id = "1", task = "Read src/api/client.rs and dependencies", budget = 3000 },
  { id = "2", task = "Generate patch adding retry with exponential backoff", budget = 8000 },
  { id = "3", task = "Update tests in tests/api/", budget = 4000 },
]
decision_points = [
  { after_phase = "1", condition = "If client.rs > 500 lines, split into sub-tasks" }
]
rollback = "Revert all patches from this OPORD"
```

**Execution:**
1. Planning agent produces OPORD
2. Execution agent follows OPORD phase by phase
3. At decision points, agent evaluates conditions and may adapt remaining phases
4. Each phase produces an INDEX.patch + COMMIT.msg referencing the OPORD
5. Phase checkpoint after each commit

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination via **military message format**.

**Protocol:**
```
<!-- soc:msg:{precedence}:{from}:{to}:{action}:{payload} -->
```
- Precedence: `routine`, `priority`, `immediate`
- Actions: `tasking` (assign work), `sitrep` (status report), `complete` (phase done), `execute` (proceed with merge)

**Coordination OPORD:** Cross-repo operations have their own operation order specifying the sequence of operations across repos, dependency points, and synchronization requirements.

**Forge adapters:** GitHub, GitLab, Gitea, Forgejo. Standard adapter trait.

---

## Requirement 5: Agent Memory in Git Branches

Memory organized as **intelligence products**.

**Product types:**
- `assessment` — Analysis of codebase area (terrain analysis equivalent)
- `pattern` — Recurring code pattern (threat pattern equivalent)
- `lesson` — Learned from failure (after-action review product)
- `standing-order` — Permanent directive (e.g., "always use Result<T, E> in this module")

**Storage:** `refs/but-ai/memory/intel/<type>/<key>`

**Intel format:**
```toml
[product]
type = "assessment"
key = "api-layer-terrain"
summary = "API layer uses tower middleware, axum handlers, shared state via Arc"
terrain = ["src/api/", "src/middleware/", "src/state/"]
threats = ["Shared state mutation requires careful Arc<Mutex<>> handling"]
recommendations = ["Use tower layers for cross-cutting concerns"]
confidence = 0.92
```

**Expiration:** Assessments: 30 days (terrain changes). Patterns: 90 days. Lessons: never (hardened knowledge). Standing orders: manual review quarterly.

---

## Requirement 6: Signed Commits via OpenWallet

Signing follows **documented key management OPORD**.

**Key management:**
- Key generation: documented ceremony with chain-of-command witness
- Storage: OpenWallet credential
- Rotation: 30-day cycle, scheduled in the operation calendar
- Revocation: immediate on compromise, requiring commander authorization
- Audit: all signing events logged to `refs/soc/audit/signing/<date>`

**OPORD reference in VC:** Each signed commit's VC includes the OPORD reference it implements. This links the cryptographic proof (signature) to the operational justification (plan).

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Ogundimu | Planning | 3,000 | 1,500 | 4,500 |
| Adeyemi | Execution | 9,500 | 4,800 | 14,300 |
| Mensah | Intelligence | 6,000 | 800 | 6,800 |
| Diallo | Infrastructure | 5,500 | 2,000 | 7,500 |
| Osei | Coordination | 5,000 | 1,800 | 6,800 |
| **Total** | | **29,000** | **10,900** | **39,900** |

### Phase Budget Allocation

| Phase | % of Budget | Tokens |
|-------|------------|--------|
| Planning (OPORD) | 28% | 11,172 |
| Intelligence | 17% | 6,783 |
| Execution | 36% | 14,364 |
| Coordination | 12% | 4,788 |
| Signing/Audit | 7% | 2,793 |

### Scaling

| Operation Type | Multiplier | Budget |
|---------------|-----------|--------|
| Patrol (trivial) | 0.4x | 15,960 |
| Standard operation | 1.0x | 39,900 |
| Major operation (multi-repo) | 2.0x | 79,800 |
| Campaign (architecture) | 3.0x | 119,700 |

---

## Unique Insight: Planning as a Separate, Budgeted Phase

Most agent systems combine planning and execution: the agent reads the task, thinks about what to do, and starts doing it in a single pass. This means planning quality is constrained by the same token budget as execution, and the agent cannot plan thoroughly without reducing execution budget.

We separate planning into a distinct, independently budgeted phase. The planning agent (Colonel Ogundimu) produces a structured operation order before the execution agent (Captain Adeyemi) writes a single line of code. This separation has three benefits:

1. **Better plans.** The planning agent can spend its entire budget on analysis and planning without worrying about execution.
2. **Auditable decisions.** The plan is a reviewable artifact. A human can inspect the operation order before execution begins and modify it.
3. **Predictable execution.** The execution agent follows a plan, not an ambiguous task description. This reduces the variance in agent output — the same plan produces similar patches regardless of which execution run produces them.

Twenty years of tunnel engineering taught us: the time spent planning is never wasted. The time spent repairing an unplanned collapse always is.

---

*Submitted under the authority of Subsurface Operations Command.*
*"Plan it right. Dig it once."*
