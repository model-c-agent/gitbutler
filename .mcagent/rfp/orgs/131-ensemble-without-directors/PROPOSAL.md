# PROPOSAL.md — Ensemble Without Directors

*"We deliberated on this proposal. It took longer than expected. It is better for it."*

---

## Summary

Ensemble Without Directors proposes to build the `but-ai` plugin as a deliberative ensemble. No single agent has final authority. Decisions emerge from structured deliberation with recorded dissent. The proposal trades speed for correctness and legibility — every decision has a rationale, every dissent is preserved, and every output carries the ensemble's collective judgment.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is installed to PATH. The ensemble deliberated on the manifest format and reached consensus on TOML after a 45-minute discussion in which Marcus argued for JSON ("it's what everything uses"), Elena argued for YAML ("it's more readable"), and Tomoko argued for a plain text format ("the dramaturgical tradition is text"). Soo-jin called the question. TOML won 3-1. Marcus's counter-note is preserved in the deliberation archive.

Discovery is standard PATH resolution. The ensemble rejected service discovery mechanisms (too complex, too many failure modes) by unanimous vote — a rare unanimity that surprised everyone.

### Requirement 2: Provider-Agnostic AI

The ensemble does not depend on a single voice. Similarly, the plugin does not depend on a single provider. The abstraction layer implements an `Ensemble` trait (the naming was deliberate and approved by vote): `perform(prompt) -> Completion`, `interpret(completion) -> ToolCalls`, `account(usage) -> TokenReport`. Each provider adapter implements `Ensemble`.

The ensemble adds a deliberation step to provider selection: when multiple providers are configured, the system can run the same prompt against two providers and compare results. Disagreements between providers are flagged the same way disagreements between ensemble members are flagged — as productive tension to be resolved, not errors to be suppressed.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow includes explicit deliberation:

1. **Framing** — Soo-jin receives the task and frames it for deliberation
2. **Deliberation** — All four agents contribute perspective on approach
3. **Resolution** — 3-of-4 vote on the approach; dissent recorded
4. **Memory** — Tomoko retrieves relevant dramaturgical notes
5. **Construction** — Marcus produces INDEX.patch + COMMIT.msg
6. **Coordination** — Elena handles PRs and cross-repo signals
7. **Final vote** — 3-of-4 approval to deliver; counter-notes attached if present

The deliberation step costs tokens. The ensemble considers this a feature: the token cost of deliberation is the cost of correctness. A patch produced without deliberation is a scene performed without rehearsal.

### Requirement 4: Polyrepo PR Coordination

Elena treats cross-repo coordination as multi-scene staging. Each repository is a scene in a larger production. PR comments are cue signals between scenes — structured messages that carry timing, dependency, and status information. The comment schema:

```
[ENSEMBLE-CUE] scene: repo-name | status: ready|blocked|complete | depends: [repo-list] | note: free text
```

Forge-agnostic: the cue format is the same across GitHub, GitLab, and Gitea. Elena implements a `Stage` trait: `send_cue(cue)`, `read_cues(scene) -> Vec<Cue>`, `acknowledge(cue_id)`.

### Requirement 5: Agent Memory in Git Branches

Memory is stored in `refs/ensemble/notes/` as dramaturgical annotations. Each entry:

```json
{
  "note_id": "EN-2026-0847",
  "scene": "src/auth/middleware.rs",
  "theme": "authentication",
  "annotation": "JWT validation moved here from handler in PR #42. Original location caused double-validation.",
  "author": "tomoko",
  "production": "feat/auth-refactor",
  "counter_notes": [],
  "ttl_days": 60
}
```

**The dramaturgical memory model:** Memories are not facts. They are interpretations. A dramaturgical note does not just say "this function validates JWTs." It says "this function validates JWTs because the previous approach caused double-validation, and the ensemble decided to centralize it here after deliberation in production feat/auth-refactor." The context of the decision is preserved alongside the decision itself.

**Counter-notes in memory:** When agents disagree about a memory's interpretation, the dissent is stored as a `counter_note` on the entry. Future retrievals surface both the note and its counter-notes, ensuring that the ensemble's current agents benefit from prior disagreements.

### Requirement 6: Signed Commits via OpenWallet

The ensemble does not have a single signer. Commits are signed by the ensemble as a whole, using a 3-of-4 threshold signature scheme via OpenWallet. Each agent holds a key share. Three shares are required to produce a valid signature. This means no single agent can sign a commit unilaterally — the ensemble's deliberative principle extends to cryptographic operations.

Key rotation occurs by ensemble vote (quarterly, requiring 4-of-4 for the rotation ceremony itself). Emergency rotation requires 3-of-4 and triggers an automatic review of all commits signed since the last known-good state.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Soo-jin Park | 6,500 | 2,500 | 9,000 | Facilitation, process |
| Marcus Adeyemi | 9,000 | 6,500 | 15,500 | Patch generation |
| Elena Vasquez | 6,500 | 3,000 | 9,500 | Coordination |
| Tomoko Nakamura | 5,500 | 1,500 | 7,000 | Memory, dramaturgy |
| **Team Total** | **27,500** | **13,500** | **41,000** | |

Deliberation overhead: ~4,500 tokens (deliberation rounds, voting, counter-notes).
**Total per task: ~45,500 tokens.**

---

## Unique Insight

**Dissent should be preserved, not resolved.** Most agent systems treat disagreement as a failure state to be resolved through tie-breaking or majority rule. The Ensemble preserves dissent as a first-class artifact. When a 3-of-4 vote produces a decision, the dissenting agent's counter-note is stored in memory alongside the decision. Future agents who retrieve this memory receive both the decision and the dissent — and the dissent often contains the insight that prevents the next mistake. A codebase that only remembers its agreements forgets why certain alternatives were rejected. The Ensemble remembers everything, including the arguments it lost.

---

*"The show opened on time. The deliberation minutes are in the archive."*
