# Station Muse — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

Station Muse proposes a `but-ai` implementation that treats code aesthetics as a measurable quality metric alongside correctness and performance. Our domain expertise in proving that beautiful transit stations increase ridership translates to agents that produce elegant code — not for vanity, but because aesthetic consistency reduces cognitive load, which accelerates review, which improves throughput.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. The binary includes a `--style-check` flag that evaluates a patch for aesthetic consistency with the surrounding code (naming conventions, structural patterns, comment density) without modifying it. This allows human reviewers to run an aesthetic check independently of the agent workflow.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml` with per-repo overrides. Style preferences (preferred naming convention, maximum function length, comment density target) are configured in `.but-ai.toml` under a `[style]` section.

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend. Contrast agent manages provider selection with a quality-first bias: for patch generation, the highest-quality available provider is preferred regardless of cost. For memory queries and coordination messages, cost-efficient providers are used. This asymmetric allocation concentrates budget where aesthetic quality matters most.

Provider comparison uses a "quality score" derived from a lightweight rubric applied to a standard test prompt at startup. The rubric evaluates code naming quality, structural consistency, and diff minimality. Scores are cached and refreshed weekly.

**Domain Insight:** In transit design, we learned that passengers cannot articulate why one station feels better than another, but they measurably change their behavior based on aesthetics. Same with code: reviewers cannot always explain why one patch "feels right," but they approve aesthetic patches faster. Our agents optimize for this: patches that feel right because they match the codebase's visual rhythm.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Composition generates patches through a three-phase aesthetic process:
1. **Read the room** — Analyze the surrounding code's style: naming patterns, indentation, function sizes, comment patterns. Build a "style profile."
2. **Generate to match** — Produce INDEX.patch that matches the style profile. Changes should look like they were written by the same author as the surrounding code.
3. **Aesthetic review** — Self-evaluate the patch against the style profile. If the style match score is below threshold, regenerate once with stricter constraints.

COMMIT.msg:
```
feat(station): add real-time passenger flow visualization

#FF8F00 | Style-match: 0.91
Agent: Composition
Palette-refs: palette/mood-2026-0443
Provider: anthropic/claude-sonnet (quality-selected)
```

## Requirement 4: Polyrepo PR Coordination

Flow manages cross-repo coordination with visual dependency mapping. Each coordination set gets a rendered ASCII dependency graph in the PR description:

```
repo-a/feat-branch ──depends-on──> repo-b/api-branch
                                        |
                                   repo-c/schema-branch
```

Flow generates these graphs automatically from branch name dependency encoding. The graphs are updated in PR descriptions as statuses change. Forge adapters (GitHub, GitLab, Gitea) implement a `ForgeAdapter` trait.

## Requirement 5: Agent Memory in Git Branches

Palette manages memory using a "mood board" model. Each memory entry has both semantic and aesthetic metadata:

| Field | Description |
|-------|-------------|
| `id` | Unique identifier |
| `content` | Semantic memory payload |
| `aesthetic` | Style observations: clean/messy, consistent/varied, dense/sparse |
| `mood` | Color hex code representing the memory's emotional tone |
| `quality_score` | How well the original work matched codebase style (0-1) |
| `ttl_days` | Based on quality: high-quality memories live longer |

Memory stored in `refs/muse/palette/<namespace>/`. Retrieval combines semantic similarity with aesthetic compatibility — a memory from a cleanly-styled codebase is more useful when working on a clean codebase.

GC removes expired entries. Memories with quality scores below 0.5 decay 2x faster — Station Muse does not want agents learning from ugly code.

## Requirement 6: Signed Commits via OpenWallet

Signature handles signing with an extended metadata block that includes the commit's color code, style-match score, and the Palette memories that informed the patch. This metadata is embedded in the signature's extension fields, making it cryptographically bound to the commit.

Key rotation every 30 days. Signature refuses to sign commits missing required aesthetic metadata — an unsigned commit at Station Muse is one missing its color code.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Composition | Patch generation | 8,000 | 4,500 | 12,500 |
| Palette | Memory & mood | 5,500 | 1,000 | 6,500 |
| Contrast | Provider & quality | 3,000 | 800 | 3,800 |
| Flow | PR coordination | 4,500 | 2,000 | 6,500 |
| Signature | Signing & metadata | 2,500 | 500 | 3,000 |
| **Per-task total** | | **23,500** | **8,800** | **32,300** |

Budget allocates ~15% more to Composition than typical patch agents because style-matching requires additional context reading (analyzing surrounding code style).

## Unique Domain Insight

Seven years of transit station design taught us that aesthetics is not a luxury — it is a driver of adoption. A station that looks unsafe is unused regardless of its actual safety record. A codebase that looks messy is avoided regardless of its actual reliability.

Our proposal includes a "style-match score" that quantifies how well an agent's patch matches the surrounding code's aesthetic patterns. This score is surfaced in PR metadata and in the commit message. Over time, it creates a feedback loop: agents that consistently produce high style-match scores are producing code that feels native to the codebase, which means reviewers approve faster, which means the development cycle tightens.

No other proposal in this RFP measures code aesthetics. We believe this is an oversight, not a feature. The best code is code that looks like it belongs.

---

*Parc Avenue entrance: 22% more riders. Because it is beautiful. #FF8F00.*
