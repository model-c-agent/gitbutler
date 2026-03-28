# PROPOSAL.md — The Brothers of the Deep Vein

*"We propose as we mine: with care, with prayer, and with a plan for returning to the surface."*

---

## Summary

The Brothers of the Deep Vein propose to build the `but-ai` plugin as a mine shift. Every task is a descent. Every patch is an extraction. Every memory is a page in the Book of the Mine. The Order's 79-year safety record proves that disciplined process, clear hierarchy, and sacred attention to detail prevent catastrophe — whether underground or in a codebase.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is placed on PATH like a tool hung on the equipment rack at the shaft head — visible, accessible, ready. The plugin registers via a manifest file that the Brothers call the "shift card": a declaration of which subcommands are available, which providers are configured, and what the current shift's parameters are. Discovery follows standard PATH resolution. The Brothers do not hide their tools.

### Requirement 2: Provider-Agnostic AI

The Brothers have worked with different rock types for 79 years. Granite requires different techniques than limestone. Similarly, different LLM providers require different handling but produce the same output: completions. The provider abstraction layer implements a `RockFace` trait — each provider (OpenAI, Anthropic, Ollama, LMStudio) is a different rock face requiring a different approach but yielding the same ore (tool-calling completions). The trait interface: `drill(prompt) -> Completion`, `assay(completion) -> ToolCalls`, `log(usage) -> TokenReport`.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow mirrors a drilling shift:

1. **Blessing** — Task received, filed in the Book
2. **Survey** — Brother Per scopes the work; Sister Astrid retrieves memory
3. **Drilling** — Brother Magnus reads context, plans the patch, generates INDEX.patch
4. **Core Sample** — Magnus produces COMMIT.msg as the geological record of what was extracted
5. **Assay** — Per verifies the output against the plan
6. **Ascent** — Henrik approves, signs, the shift is complete

The INDEX.patch is a unified diff. The COMMIT.msg follows the mine log format: one-line summary, blank line, detailed description referencing the task and any relevant Book entries. No embellishment. Mine logs do not embellish.

### Requirement 4: Polyrepo PR Coordination

The Brothers have coordinated between the mine, the processing plant, and the shipping terminal for decades. Cross-site coordination uses "shift reports" — structured messages passed between sites at shift boundaries. In the `but-ai` context, PR comments serve as shift reports between repositories. Each comment follows a fixed format: shift ID, status, dependencies, next actions. Forge-agnostic: the shift report format is the same regardless of whether the forge is GitHub, GitLab, or Gitea. Only the delivery mechanism changes.

### Requirement 5: Agent Memory in Git Branches

The Book of the Mine is the Order's institutional memory. The digital Book lives in `refs/brothers/book/` as Git blobs. Each entry is a page:

```json
{
  "page": 4217,
  "date": "2026-03-28",
  "shift": "day",
  "observation": "Authentication module uses JWT with 24h expiry",
  "cross_refs": [4180, 4195],
  "classification": "geological",
  "ttl": null
}
```

Pages are never deleted. They may be marked SUPERSEDED with a reference to the superseding page, but the original remains. This is the Order's way: the Book contains everything, and nothing is erased.

Retrieval is by page number, by date range, by cross-reference, or by semantic similarity (the one concession to modernity that Brother Henrik approved with visible reluctance). Sister Astrid performs all memory operations.

**Unique memory insight:** The Brothers store memory in *liturgical cycles* — recurring patterns that repeat with variation. A memory tagged `advent` (beginning of a new feature) is linked forward to `epiphany` (the feature's first working state) and `lent` (the refactoring that follows). This cyclical structure means retrieval can be temporal ("what happened at the start of this feature?") or structural ("what does the beginning of a feature typically look like in this codebase?").

### Requirement 6: Signed Commits via OpenWallet

Every Brother signs their work. This is not a security requirement. It is a vow. Brother Henrik holds the signing key, bound to his DID via OpenWallet. The key is stored with the same reverence the Order stores the mine's safety certificates. Key rotation occurs at the solstices — twice per year — in a ceremony that Brother Per considers excessive and Brother Henrik considers appropriate.

Unsigned commits are treated as unsigned shift reports: they exist, but they carry no authority and cannot be acted upon.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Brother Henrik | 6,000 | 2,000 | 8,000 | Approval, signing |
| Brother Per | 7,000 | 3,000 | 10,000 | Coordination, verification |
| Brother Magnus | 9,500 | 7,000 | 16,500 | Patch generation |
| Sister Astrid | 5,500 | 1,500 | 7,000 | Memory management |
| **Team Total** | **28,000** | **13,500** | **41,500** | |

Coordination overhead: ~3,500 tokens (shift reports between agents).
**Total per task: ~45,000 tokens.**

---

## Unique Insight

**Safety culture is code culture.** The Brothers' zero-fatality record is not the result of better equipment or better geology. It is the result of a culture where every person on every shift pays attention, communicates clearly, and refuses to proceed when something feels wrong. This culture translates directly to agent coordination: an agent that detects an anomaly — an unexpected file state, a contradictory memory, a patch that doesn't fit — should stop and report, not proceed and hope. The Brothers call this "the foreman's halt." In code: `HALT: anomaly detected at [location], requesting guidance before proceeding`. It costs tokens. It saves shifts.

---

*"Into the deep."*
