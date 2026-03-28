# PROPOSAL.md — Rasmussen Mining Works

*"We have been solving this problem for 139 years. The tools change. The mountain does not."*

---

## Summary

Rasmussen Mining Works proposes to build the `but-ai` plugin with the same philosophy that has sustained a single copper mine for five generations: deep knowledge of one thing, patient execution, and decisions evaluated against a multi-generational horizon. No fads. No shortcuts. One mine, done well.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is installed to PATH and discovered through standard resolution. The Rasmussens do not over-engineer discovery. The mine entrance has been in the same place since 1887. The tool should be where you expect it. A manifest file declares capabilities, version, and provider configuration. The manifest format is simple: TOML, because the Rasmussens believe configuration should be readable by the person who will maintain it in 25 years.

### Requirement 2: Provider-Agnostic AI

The Rasmussens have used picks, pneumatic drills, hydraulic rigs, and electric rotary drills over 139 years. The ore does not care which tool extracts it. The provider abstraction layer presents a `Tool` trait: `cut(prompt) -> Completion`, `grade(completion) -> ToolCalls`, `weigh(usage) -> TokenReport`. OpenAI, Anthropic, Ollama, and LMStudio each implement `Tool`. The abstraction is minimal because the Rasmussens believe that thin abstractions last longer than thick ones.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow mirrors the mine's daily operation:

1. **Morning brief** — Liv reads the task, scopes the work
2. **Ledger check** — Ola retrieves relevant memory
3. **Consultation** — Erik advises on architectural matters (when applicable)
4. **Shift work** — Hanne produces INDEX.patch + COMMIT.msg
5. **End of shift** — Liv reviews, signs, closes the task

The INDEX.patch is a unified diff. The COMMIT.msg follows the Rasmussen ledger format: date, summary, detail, cross-references. Hanne's patches are conservative — she changes what needs changing and nothing else. The codebase, like the mountain, should not be disturbed unnecessarily.

### Requirement 4: Polyrepo PR Coordination

The Rasmussens coordinate between the mine, the processing facility, and the port terminal. Each site operates semi-independently but shares a common ledger. Cross-repo coordination uses PR comments as ledger entries — structured messages with date, site identifier, status, and dependencies. Forge-agnostic: the ledger format is the same. Only the transport changes (GitHub API, GitLab API, Gitea API).

Branch naming encodes dependencies in the GitButler convention. Liv manages the dependency graph the way she manages the mine's ventilation plan — as a directed graph where changes propagate downwind.

### Requirement 5: Agent Memory in Git Branches

The Rasmussen ledger is the memory system. Stored in `refs/rasmussen/ledger/` as Git blobs. Each entry:

```json
{
  "page": 14227,
  "date": "2026-03-28",
  "author": "ola",
  "entry": "Auth module uses bcrypt with cost factor 12",
  "strata": "auth",
  "depth": "bedrock",
  "cross_refs": [14180, 14201],
  "generation": 5
}
```

**Depth categories:**
- `topsoil` — ephemeral, current task context, TTL 7 days
- `subsoil` — project conventions, TTL 90 days
- `bedrock` — architectural invariants, no TTL

The `generation` field is the Rasmussens' unique contribution: it tracks how many times a memory has been confirmed by subsequent observations. A bedrock memory with `generation: 5` has been independently confirmed five times. High-generation memories are trusted more in retrieval ranking. This is how the Rasmussens think about geological knowledge: a measurement confirmed by five independent surveys is more reliable than one confirmed by one.

### Requirement 6: Signed Commits via OpenWallet

Liv signs all commits. The signing key is bound to her DID via OpenWallet. Key management follows the five-generation principle: the key infrastructure must be transferable to the next managing director without service interruption. Liv has documented the key rotation ceremony in the same ledger where Ingrid documented the transition from pneumatic to hydraulic drilling. The document is titled "On the Orderly Transfer of Signing Authority" and runs to three pages.

Unsigned commits are not rejected. They are flagged as "unrecorded" — present in the repository but absent from the ledger, which is worse. The ledger is the source of truth.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Liv Rasmussen | 6,500 | 2,000 | 8,500 | Decision, signing |
| Erik Rasmussen | 5,000 | 1,000 | 6,000 | Architecture counsel |
| Hanne Brekke | 9,000 | 6,500 | 15,500 | Patch generation |
| Ola Solheim | 5,500 | 1,500 | 7,000 | Memory management |
| **Team Total** | **26,000** | **11,000** | **37,000** | |

Coordination overhead: ~2,500 tokens (ledger updates, consultation).
**Total per task: ~39,500 tokens.**

---

## Unique Insight

**Multi-generational memory builds trust through confirmation.** Most memory systems store facts and retrieve them by relevance. The Rasmussen ledger stores facts and tracks how many times each fact has been independently confirmed. A memory with `generation: 1` is a single observation. A memory with `generation: 5` is a geological constant. This distinction changes retrieval behavior: when token budgets are tight, high-generation memories are preferred because they are more likely to be correct. When budgets are generous, low-generation memories are included because they may reveal recent changes that contradict long-held assumptions. The Rasmussens know that the mountain sometimes surprises even those who have studied it for a century.

---

*Five generations. One mountain. The copper is patient. So are we.*
