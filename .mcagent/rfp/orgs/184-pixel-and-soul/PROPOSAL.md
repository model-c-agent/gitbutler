# Pixel & Soul — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Art-Directed Agent System

---

## Executive Summary

Pixel & Soul proposes an agent architecture designed for creative workflows where ambiguity is a feature. Agents generate multiple interpretations of each task, annotated with emotional registers, and present them as parallel virtual branches. The human artist selects, combines, or rejects. The system optimizes for range, not convergence.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary installs to `~/.gitbutler/bin/`. The binary's subcommand structure reflects the creative workflow rather than the technical pipeline:

- `but ai sketch` — generates multiple patch variants (branch-per-variant)
- `but ai review` — presents variants side by side with affective annotations
- `but ai select` — applies the chosen variant and archives the others
- `but ai narrate` — generates Cass's narrative commit message for the selected variant

This command vocabulary is deliberately non-technical. The commune's members think in creative terms, not engineering terms. The plugin adapts to the user's language, not the other way around.

Juno builds the binary as a single static Rust executable. The binary shells out to `but` for all Git operations — no direct Git library calls. This keeps the plugin thin and reduces the surface area Juno has to maintain.

## Requirement 2: Provider-Agnostic AI

Provider selection is per-agent, not per-task. Creative agents (Samir, Elin, Cass) require providers with strong generative capabilities and nuanced language understanding — they default to Anthropic or OpenAI. Juno's infrastructure tasks route to Ollama when available, because local inference avoids network latency during builds.

The provider interface is minimal: `generate(prompt, constraints) -> Vec<Response>`. The `Vec` return type is intentional — the system always requests multiple completions. Single-completion mode exists but is considered a compromise, used only when the token budget is critically low.

Provider health is checked lazily — on first invocation per session, not on startup. The commune learned this the hard way when a startup health check to a slow Ollama instance added 30 seconds to every `but ai` invocation.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The variant-first workflow:

1. Elin analyzes the current repository state and scores the emotional context
2. Samir generates N patch variants (default N=3), each targeting a different emotional register from Elin's score
3. Each variant is committed to a separate virtual branch: `ai/variant-1`, `ai/variant-2`, etc.
4. Maren reviews the variants. She selects one, or requests a composite
5. Cass writes the COMMIT.msg as a narrative that includes the emotional rationale for the selection
6. Juno applies the selected patch to the target branch with signing

The COMMIT.msg format:

```
<narrative paragraph — Cass's voice>

---
Variant: 2 of 3
Affective-Register: reflective
Confidence: 0.78
Alternatives-Archived: ai/variant-1, ai/variant-3
```

Archived variants remain on their branches for 7 days. This enables retrospective review — "what would the piece have looked like if we had chosen variant 1?"

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination is rare for the commune (most projects are single-repo), but when it occurs (e.g., a film project with separate repos for audio, visual, and text), PR comments carry affective metadata:

```
[P&S:sync] register=contemplative confidence=0.74
Audio mix finalized for sequence 3. Visual branch should
match the descending energy curve — suggest desaturating
the final 8 seconds.
```

The forge adapter supports GitHub and GitLab. Gitea support is planned if a client requests it. The commune does not build capabilities speculatively — they build what is needed for the current project and generalize later.

## Requirement 5: Agent Memory in Git Branches

Memory branches: `refs/ps/memory/<agent-name>`. Memory types:

- **`palette`**: Affective preferences learned from Maren's selections. TTL: 90 days. These encode the commune's evolving creative sensibility — which emotional registers Maren tends to select, which she tends to reject.
- **`technique`**: Successful patch patterns. TTL: 30 days. Samir stores visual techniques that have been approved; Elin stores audio scoring calibrations.
- **`rejection`**: Explicitly rejected approaches, with Maren's redirection notes. TTL: 180 days. Rejections are stored longer than approvals because knowing what *not* to do is more durable knowledge than knowing what to do.

Memory retrieval uses embedding similarity. Elin's affective scores serve double duty as memory keys — when a new task arrives, the system retrieves memories with similar emotional profiles.

## Requirement 6: Signed Commits via OpenWallet

Each agent holds an OpenWallet DID. Juno manages key provisioning and rotation (30-day cycle). The commune adds a creative provenance layer: every signed commit includes the agent's role in the creative pipeline and the variant number. This enables galleries and festivals to verify the human-AI collaboration chain — a growing requirement in digital art exhibitions.

**Unique insight:** Pixel & Soul treats virtual branches as creative parallel universes. Each AI variant represents a different emotional interpretation of the same material. The version control system becomes, in effect, a multiverse of artistic possibilities. This reframes `but-ai` not as an automation tool but as a *creative amplifier* — it does not reduce the artist's choices, it multiplies them. The merge operation becomes a curatorial act: selecting which universe to make real.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Maren | 1,800 | 800 | 2,600 |
| Samir | 3,500 | 4,000 | 7,500 |
| Elin | 2,500 | 1,200 | 3,700 |
| Cass | 1,500 | 1,000 | 2,500 |
| Juno | 3,000 | 2,000 | 5,000 |
| **Task Total** | **12,300** | **9,000** | **21,300** |

Variant overhead (N=3 variants at ~1.5x single-variant cost): 10,650 additional tokens. Grand total per creative task: **31,950 tokens**.

Note: The commune considers this overhead justified. "Three options are not three times the cost. They are one-third the regret." — Maren

---

*"The best cut is the one you almost didn't choose."*
