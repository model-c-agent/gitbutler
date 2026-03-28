# Bread & Pigment — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

Bread & Pigment proposes a `but-ai` implementation built on a layered craft model: context preparation, structural generation, aesthetic refinement, and deliberate release. Our domain expertise in producing artisan bread and food-pigment art translates to agents that work in disciplined layers, where each stage enriches the next.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. The binary supports a `--layers` flag that outputs the current agent pipeline stage (culture, preparation, structure, refinement, release) for debugging and progress monitoring.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. Bread & Pigment adds a `[craft]` section with configurable quality thresholds: `min_structural_score` (Knead's output quality gate) and `min_aesthetic_score` (Proof's refinement threshold). Setting `skip_proof = true` disables the refinement layer for urgent tasks (Maren's escape hatch).

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend. Provider selection is layer-specific:
- **Levain & Autolyse** (context layers): Use cost-efficient providers. These layers read and summarize, requiring comprehension but not generation brilliance.
- **Knead** (structure layer): Use the highest-quality available provider. Structural generation is the critical path.
- **Proof** (refinement layer): Use a quality provider, but can fall back to a cheaper one if budget is tight.
- **Score** (release layer): Minimal provider use — mostly deterministic operations.

This per-layer provider routing maximizes quality where it matters most.

**Domain Insight:** In baking, you use the best flour for the final dough and the leftover flour for dusting. In painting, you use the best pigment for the top layer and the cheaper pigment for the ground. Bread & Pigment applies this to providers: invest quality where it is visible (patch generation), economize where it is not (context reading).

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Patch generation follows the five-layer pipeline:

1. **Levain** — Retrieve relevant memories. Build the cultural context.
2. **Autolyse** — Read all files in scope, branch state, recent commits. Produce a "context digest" — a structured summary of what exists and what the task requires.
3. **Knead** — From the digest, generate INDEX.patch. Focus on correctness and structural integrity. Do not worry about aesthetics.
4. **Proof** — Review the patch for aesthetic quality: naming consistency, structural harmony with surrounding code, unnecessary complexity. Refine once.
5. **Score** — Validate, sign, release.

COMMIT.msg:
```
feat(color): add pigment degradation detection for oil paintings

The crust is golden. The crumb is open. Ship it.

Agent: Knead (refined by Proof)
Layers: culture > prep > structure > refinement > release
Pigment: burnt sienna | Culture-refs: levain/culture-2026-0055
```

## Requirement 4: Polyrepo PR Coordination

Score handles cross-repo coordination as part of the release layer. Coordination sets are tracked in `refs/bread/release/`. Score treats each cross-repo merge as a "batch" — all loaves in the batch go in the oven together. If one is not ready, the batch waits.

Forge adapters (GitHub, GitLab, Gitea) implement a minimal trait. PR descriptions include a "layers completed" status showing which pipeline stages have finished for each PR in the batch.

## Requirement 5: Agent Memory in Git Branches

Levain manages memory as a living culture. The culture metaphor is precise:

| Memory Type | Behavior | TTL |
|-------------|----------|-----|
| Active culture | Frequently accessed, high relevance. Fed (refreshed) on each access. | Indefinite while active |
| Dormant culture | Not accessed recently. Can be revived by a relevant task. | 60 days |
| Dried culture | Archived foundational knowledge. Stable, rarely changes. | 180 days |

Memory stored in `refs/bread/culture/`. Key feature: **tension detection.** When a new memory contradicts an existing one, both are retained with a "tension" tag. Tension entries are surfaced during Autolyse (context preparation) so that Knead can make an informed decision about which direction to follow. Tensions are resolved when a human reviewer approves one direction, at which point the losing memory is reclassified as "dormant."

GC runs on `but-ai memory gc`, converting inactive entries to dormant, dormant to dried, and expired dried entries to deleted.

## Requirement 6: Signed Commits via OpenWallet

Score handles signing as the final layer. Signing metadata includes:
- Which layers were completed (all five, or was Proof skipped?).
- Quality scores from Knead (structural) and Proof (aesthetic).
- The pigment descriptor for the commit mood.

If Proof was skipped (`skip_proof = true`), the signature metadata marks the commit as "unrefined" — a signal to human reviewers that the aesthetic layer was bypassed. This is transparent by design.

Key rotation every 30 days.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Levain | Memory culture | 4,500 | 800 | 5,300 |
| Autolyse | Context preparation | 6,000 | 1,500 | 7,500 |
| Knead | Structural generation | 5,000 | 4,000 | 9,000 |
| Proof | Aesthetic refinement | 4,000 | 2,000 | 6,000 |
| Score | Signing & coordination | 4,000 | 1,200 | 5,200 |
| **Per-task total** | | **23,500** | **9,500** | **33,000** |

The five-layer pipeline uses more total tokens than a three-stage approach, but produces higher-quality first-pass output. The refinement layer (Proof) costs approximately 18% of the total budget. We consider this the cost of craft.

## Unique Domain Insight

Ten years of artisan baking taught Maren that the most common failure mode is not bad ingredients — it is rushing the process. Under-proofed bread looks fine going into the oven and collapses inside. Over-kneaded dough has perfect structure but no flavor. Every stage needs its time. Skipping a stage does not save time — it wastes the time spent on all previous stages.

Our five-layer pipeline applies this discipline to agent development. Most proposals have three stages: read, generate, sign. We add preparation (Autolyse) between reading and generating, and refinement (Proof) between generating and signing. These additional layers seem like overhead. They are not. Autolyse prevents Knead from generating patches based on incomplete understanding. Proof prevents Score from signing patches that are correct but ugly. The layers compound: each one makes the next more effective.

---

*First bake: 5 AM. Gallery opens: 10 AM. The crust and the canvas are ready.*
