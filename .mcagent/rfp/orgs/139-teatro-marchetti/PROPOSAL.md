# PROPOSAL.md — Teatro Marchetti

*"We have been building things that move for a hundred years. Code is our newest material."*

---

## Summary

Teatro Marchetti proposes to build the `but-ai` plugin as a puppet workshop. Patches are carved, not generated — shaped through iterative refinement from rough form to finished craft. Memory is the carver's notebook: handwritten, personal, and organized by the maker's hands. The Marchettis bring a century of embodied craft knowledge to the problem of autonomous code generation, and their central insight is that repair is more valuable than replacement.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary on PATH. The Marchettis keep their tools in the same place they have always been — on the wall of the workshop, each in its marked position. The plugin manifest is the tool rack: a declaration of what is available and where. TOML format. Discovery by PATH. No complexity that does not serve the craft.

### Requirement 2: Provider-Agnostic AI

Different woods require different chisels, but the carving technique is the same. The provider abstraction implements a `Workshop` trait: `carve(prompt) -> Completion`, `shape(completion) -> ToolCalls`, `measure(usage) -> TokenReport`. Four provider adapters. The trait is thin — the Marchettis have learned over a century that the thinnest tools last longest.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow follows the puppet-making process:

1. **Commission** — Sofia reads the task, determines the form
2. **Material selection** — Alessia retrieves relevant memory (workshop notes)
3. **Consultation** — Rosa advises on deep structure (when the task touches architecture)
4. **Rough carving** — Luca produces a skeleton INDEX.patch
5. **Refinement** — Luca refines in 1-2 passes, shaping the patch to fit the codebase
6. **Inspection** — Sofia checks: does it move correctly? is the craftsmanship sound?
7. **Delivery** — Signed commit

Luca's COMMIT.msg includes repair notes when the patch modifies existing code: what was found, what was changed, why the original approach was insufficient. This follows the workshop tradition: every repair is documented so the next repair can build on the knowledge.

### Requirement 4: Polyrepo PR Coordination

The Marchettis have collaborated with other puppet workshops for traveling productions. Cross-workshop coordination uses "production letters" — structured messages sent between workshops specifying character assignments, staging requirements, and dependencies. In the agent context, PR comments serve as production letters:

```
[MARCHETTI-LETTER] workshop: backend | character: auth-module
status: carved-and-ready | depends: frontend (awaiting-commission)
note: JWT refresh ready, needs frontend token storage
```

Forge-agnostic: implement a `PostalService` trait for GitHub/GitLab/Gitea.

### Requirement 5: Agent Memory in Git Branches

Memory is the carver's notebook, stored in `refs/marchetti/notebook/`:

```json
{
  "page": 847,
  "date": "2026-03-28",
  "carver": "luca",
  "subject": "auth/middleware.rs",
  "observation": "JWT validation centralized, RS256 with JWKS",
  "repair_note": "Moved from handler after double-validation issue",
  "sketch": "middleware -> validate -> extract_claims -> pass_to_handler",
  "material": "rust",
  "ttl_days": 90
}
```

**The notebook memory model:** Every memory includes a `sketch` — a brief structural description of the mechanism, like a carver's sketch of a joint. Sketches are more durable than detailed descriptions because they capture structure, not implementation. When the implementation changes, the sketch may still be accurate.

**Unique memory feature: repair memory.** When a patch modifies existing code (a repair), Luca records both the before and after states. The `repair_note` field captures why the original approach was insufficient. Over time, the notebook accumulates a history of repairs that reveals the codebase's stress points — the joints that crack, the mechanisms that wear. Retrieving repair memories for a file tells the agent not just what the file contains, but where it has broken before and how it was fixed.

### Requirement 6: Signed Commits via OpenWallet

Sofia signs. The master carver's mark on the finished puppet. DID-bound key via OpenWallet. Sofia has signed every Marchetti production for 25 years. The signature is the guarantee of quality — a Marchetti mark means the work meets Marchetti standards.

Key management: annual rotation, aligned with the theater season. The old key is archived in the notebook alongside the signatures it produced.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Sofia Marchetti | 6,000 | 2,000 | 8,000 | Direction, review, signing |
| Luca Marchetti | 9,000 | 6,500 | 15,500 | Carving (patch gen) |
| Nonna Rosa | 5,000 | 1,000 | 6,000 | Architecture counsel |
| Alessia Ferrante | 5,500 | 1,500 | 7,000 | Memory management |
| **Team Total** | **25,500** | **11,000** | **36,500** | |

Workshop overhead: ~2,000 tokens (consultation, repair notes).
**Total per task: ~38,500 tokens.**

---

## Unique Insight

**Repair memory is more valuable than creation memory.** Most memory systems record what was built. The Marchetti notebook records what was *repaired* — and why. A creation tells you the intended design. A repair tells you where the design failed under real-world stress. Over time, repair memories map the codebase's fracture points: the modules that break under load, the patterns that seemed sound but degraded, the assumptions that proved false. An agent with access to repair memory approaches a codebase the way Sofia approaches a puppet: not as a pristine object, but as a living thing with a history of use, stress, and mending. The repairs are the most important part of the story.

---

*"The wood remembers. We just have to listen."*
