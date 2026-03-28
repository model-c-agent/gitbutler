# PROPOSAL.md — Drill Team Six

**"We drill the plan. Every time."**

---

## Summary

Drill Team Six proposes to build the `but-ai` plugin as a competitive drilling relay. Tasks are drill plans. Patches are core samples. Handoffs between agents are timed and explicit. The team's competitive discipline — plan before drilling, execute the plan, review the film — produces fast, accurate, on-spec output within budget.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary goes on PATH the way a drill rig goes on the competition floor: registered, inspected, ready. The plugin manifest is the team's competition registration card — it declares capabilities, version, and configuration. Discovery: standard PATH lookup. No hidden rigs. The binary supports subcommand dispatch via `but ai <subcommand>`, with each subcommand mapped to a drill phase.

### Requirement 2: Provider-Agnostic AI

Different drill rigs, same competition rules. The provider abstraction layer treats each LLM provider as a different rig model — different controls, same output (completions with tool calls). The `Rig` trait: `engage(prompt) -> Completion`, `extract_calls(completion) -> Vec<ToolCall>`, `meter_usage(completion) -> TokenUsage`. Each provider adapter (OpenAI, Anthropic, Ollama, LMStudio) implements `Rig`. The team doesn't care which rig they're on. They drill the plan regardless.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow is a relay:

1. **Drill Plan** — Moreno decomposes the task into phases with explicit acceptance criteria
2. **Phase 1: Context** — Mital retrieves memory, reads relevant files, prepares the context package
3. **Phase 2: Drill** — Sanchez receives the context package and produces INDEX.patch + COMMIT.msg
4. **Phase 3: Coordinate** — Zhao handles cross-repo dependencies, creates PRs, posts coordination comments
5. **Phase 4: Review** — Moreno compares output to drill plan, approves or calls a re-drill

The INDEX.patch is a unified diff. The COMMIT.msg follows competition scorecard format: one-line summary, metrics (files changed, lines added/removed), and a reference to the drill plan.

### Requirement 4: Polyrepo PR Coordination

Zhao's specialty. Cross-repo coordination is modeled as multi-rig relay — multiple drill rigs running simultaneously on different granite blocks, with handoffs between them. PR comments serve as relay handoff signals: "Rig A complete. Core extracted. Rig B cleared to drill." Forge-agnostic: Zhao uses a `CompetitionFloor` trait that abstracts GitHub/GitLab/Gitea behind a uniform interface for posting and reading coordination signals.

Dependencies between repos are encoded in branch names (matching the GitButler convention): `s01.s02` means repo B's branch depends on repo A's completion. Zhao monitors both rigs and signals when dependencies are satisfied.

### Requirement 5: Agent Memory in Git Branches

Memory entries are stored as "core samples" in `refs/dts/cores/`:

```json
{
  "core_id": "DTS-2026-0042",
  "depth": "shallow|mid|deep",
  "orientation": "auth|data|ui|infra",
  "grade": 0.92,
  "sample": "JWT refresh uses sliding window, 24h max",
  "drilled_by": "mital",
  "competition": "feat/auth-refactor",
  "ttl_shifts": 30
}
```

**Depth** indicates how fundamental the knowledge is: `shallow` (ephemeral, current task), `mid` (project conventions, recurring patterns), `deep` (architectural invariants, never expire). Mital calibrates retrieval by depth: for routine patches, shallow + mid. For architectural changes, all three.

**Unique memory scheme:** Core samples are stored in "core trays" — ordered sequences that preserve the spatial relationship between samples. Retrieving one core often means retrieving the tray, because adjacent samples provide context. This mimics how geologists read core: not one sample at a time, but the sequence.

### Requirement 6: Signed Commits via OpenWallet

Captain Moreno signs all commits. In competition, the team captain signs the scorecard. Same principle. The signing key is bound to Moreno's DID via OpenWallet. The signing ceremony is brief and non-negotiable: Moreno reviews the patch, confirms it matches the drill plan, and signs. Unsigned commits are "unscored runs" — they happened, but they don't count.

Key rotation: every competition season (quarterly). Emergency rotation: if any team member reports a potential compromise, all keys rotate within the hour. Moreno does not negotiate on key security. "Losing a key is worse than losing a match."

---

## Token Budget

| Agent | Input | Output | Total | Phase |
|-------|-------|--------|-------|-------|
| Captain Moreno | 7,000 | 2,500 | 9,500 | Plan + Review |
| Sanchez | 8,500 | 6,500 | 15,000 | Drill (patch gen) |
| Mital | 6,000 | 2,000 | 8,000 | Context + Memory |
| Zhao | 6,500 | 3,000 | 9,500 | Coordination |
| **Team Total** | **28,000** | **14,000** | **42,000** | |

Handoff overhead: ~3,000 tokens (relay signals between phases).
**Total per task: ~45,000 tokens.**

---

## Unique Insight

**The handoff tax is real and must be budgeted.** Every relay team knows that transitions between drillers cost time. In code agent systems, transitions between agents cost tokens — serializing state, communicating context, verifying handoff integrity. Most proposals treat this as overhead to be minimized. Drill Team Six treats it as a first-class budget item, measured and optimized but never skipped. A fumbled handoff (agent receiving insufficient context) costs far more than a clean one. We budget 3,000 tokens per task for handoffs and track actual handoff cost per run. Our film review process (post-task analysis) identifies handoff inefficiencies for the next run.

---

*"Core quality. Hole accuracy. No excuses."*
