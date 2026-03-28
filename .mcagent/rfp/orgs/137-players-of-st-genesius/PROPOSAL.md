# PROPOSAL.md — The Players of St. Genesius

*"We propose to build software the way we build plays: with craft, conscience, and care."*

---

## Summary

The Players of St. Genesius propose to build the `but-ai` plugin as a work of craft. Every patch is a scene — it must function, but it must also be well-made. The examination of conscience is applied to code review: not just "does it work?" but "is it the right approach? have we considered the alternatives? does it serve the codebase with the same respect we owe the audience?" The Players trade speed for quality and consider this a moral position.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is installed to PATH. The manifest is TOML. Configuration is explicit and readable — the Players believe that configuration files, like scripts, should be legible to anyone who reads them. No hidden defaults. No implicit behaviors. Every setting is declared, every default is documented.

### Requirement 2: Provider-Agnostic AI

The Players work with whatever theater they are given — thrust, proscenium, black box, outdoor amphitheater. The script adapts to the space. Similarly, the provider abstraction adapts to the LLM: a `Theater` trait: `stage(prompt) -> Completion`, `interpret(completion) -> ToolCalls`, `account(usage) -> TokenReport`. Each provider is a different theater. The script (the prompt) is the same. The staging (the adapter) adjusts.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow follows the Players' production process:

1. **Script reading** — Brennan reads the task, discerns the approach
2. **Research** — Maria retrieves memory, gathers context
3. **Construction** — Joseph produces INDEX.patch + COMMIT.msg with craft
4. **Examination** — Claire reviews the patch: Is it correct? Is it beautiful? Is it honest?
5. **Opening** — Brennan approves and signs

Joseph's COMMIT.msg follows a narrative format: a brief statement of what was done and why, written in complete sentences, with attention to clarity and precision. The Players do not use Conventional Commits. They write commit messages the way they write program notes: for the audience who will read them.

### Requirement 4: Polyrepo PR Coordination

Maria treats cross-repo coordination as inter-department scheduling. Each repo is a department (scenic, lighting, costumes, sound). Changes that span departments require a production meeting — a structured coordination exchange via PR comments:

```
[GENESIUS-PRODUCTION] dept: backend | status: rehearsing | depends: auth (opening night)
note: JWT refresh patch ready for tech rehearsal when auth module lands
```

Forge-agnostic: Maria implements a `Company` trait for GitHub/GitLab/Gitea with `schedule_meeting()`, `read_minutes()`, `confirm_attendance()`.

### Requirement 5: Agent Memory in Git Branches

Memory is stored in `refs/genesius/archive/` as production notes. Each entry:

```json
{
  "note_id": "GEN-2026-0847",
  "production": "feat/auth-refactor",
  "scene": "src/auth/middleware.rs",
  "observation": "JWT validation centralized here after double-validation issue in PR #42",
  "moral": "Centralization reduces error surface but creates single point of failure",
  "author": "maria",
  "ttl_days": 60,
  "examined": true
}
```

**The examination model:** Every memory entry has an `examined` flag. When Claire reviews a patch that references a memory, she examines the memory itself: is it still accurate? does it still reflect the codebase's reality? Examined memories receive an extended TTL. Unexamined memories expire on their original schedule. This creates a natural lifecycle where actively used memories persist and unused memories fade — like production notes that are consulted every performance versus notes that sit in the archive.

**Unique memory feature:** The `moral` field. Every memory includes a one-line lesson — not just what was observed, but what it teaches. "JWT validation was centralized" is an observation. "Centralization reduces error surface but creates single point of failure" is a moral. When agents retrieve memory, the morals guide decision-making in a way that raw observations do not.

### Requirement 6: Signed Commits via OpenWallet

Father Brennan signs all commits via OpenWallet DID key. The signing is the final act of the production — the artistic director's mark on the work. Brennan signs only after Claire's examination is complete. The signing is not perfunctory; Brennan reads the final patch before signing. "I do not sign what I have not read. I would not open a show I have not seen."

Key rotation: annually, at the start of the season. Emergency rotation: at Brennan's discretion, with prayer optional but, he notes, recommended.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Fr. Brennan | 6,000 | 2,000 | 8,000 | Direction, signing |
| Sr. Claire | 6,500 | 2,500 | 9,000 | Examination, review |
| Joseph Abara | 9,000 | 6,500 | 15,500 | Craft, patch generation |
| Maria Cruz | 6,000 | 2,500 | 8,500 | Coordination, memory |
| **Team Total** | **27,500** | **13,500** | **41,000** | |

Examination overhead: ~3,500 tokens (conscience review, moral extraction).
**Total per task: ~44,500 tokens.**

---

## Unique Insight

**Memory should contain morals, not just observations.** An observation tells you what happened. A moral tells you what it means. When an agent retrieves the memory "JWT validation was centralized in middleware," it knows a fact. When it retrieves the moral "centralization reduces error surface but creates single point of failure," it knows a principle. Principles generalize. An agent working on a different module, facing a similar centralization decision, will retrieve the moral and apply it — not because the specifics match, but because the principle transfers. The Players believe that code, like theater, teaches through examples. The moral is the lesson the example teaches.

---

*"Beauty is a moral argument. Craft is devotion. The audience deserves our best."*
