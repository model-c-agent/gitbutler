# The Watchers of the Firmament — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

The Watchers propose a `but-ai` implementation rooted in contemplative observation: agents that look before they act, remember with intention, and sign their work as an act of attestation. Our domain expertise in multi-station astronomical observation — synthesizing data from diverse sources into a coherent catalogue — maps directly to polyrepo coordination and memory management.

## Requirement 1: PATH-Based Plugin Architecture

Statically-linked Rust binary at `$PATH` following the `but-tool-ai` convention. The binary operates in a request-response model: structured JSON input on stdin, structured JSON output on stdout. No persistent daemon. Each invocation is a discrete observation session.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. Repository-level overrides in `.but-ai.toml`. The Watchers add one non-standard config field: `stewardship.log_path`, which points to a file where the stewardship ratio (value/cost) is logged per task. This field is optional but encouraged.

## Requirement 2: Provider-Agnostic LLM Interface

Four backends (OpenAI, Anthropic, Ollama, LMStudio) behind a `Provider` trait. Sirius agent manages provider selection with a stewardship bias: when two providers can accomplish the same task, Sirius prefers the one with lower resource consumption (tokens, latency, energy). For local providers (Ollama, LMStudio), Sirius estimates energy cost using a simple watt-hour model.

Provider fallback is configured per-repository. Token counting uses provider-native methods where available and a conservative overestimate otherwise — the Watchers would rather overcount tokens than undercount them.

**Domain Insight:** In multi-station astronomy, each station has different capabilities: different aperture, different wavelength sensitivity, different sky coverage. You do not send every observation to the largest telescope — you match the observation to the appropriate instrument. Sirius applies this principle to provider selection: simple tasks go to smaller models, complex reasoning goes to frontier models. The matching is based on task complexity estimation, not a static configuration.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Polaris generates patches through a three-phase contemplative cycle:
1. **Observation** — Read project status, branch state, recent commits. No action until the full context is understood.
2. **Discernment** — Consult Vega for relevant memories. Identify the intention of the task. Determine whether the change serves the codebase's long-term health.
3. **Action** — Generate INDEX.patch and COMMIT.msg. The commit message includes an "intention" field that states the purpose beyond the technical change.

Patches are validated with `git apply --check` before submission. COMMIT.msg follows Conventional Commits with an appended observation block:

```
fix(tracking): correct epoch parsing for pre-2000 TLEs

Intention: Preserve accuracy of historical tracking data.
Agent: Polaris
Confidence: certain
Memory-Refs: vega/obs-2026-0341, vega/obs-2026-0298
```

## Requirement 4: Polyrepo PR Coordination

Aldebaran manages cross-repo coordination through a vigil model. Rather than actively pushing coordination messages, Aldebaran maintains a watch list and polls at configured intervals (default: every 5 minutes during active work sessions).

The watch list is stored in `refs/watchers/vigil/` as a JSON document mapping coordination sets to their constituent PRs. Each coordination set has a readiness threshold: the minimum number of PRs that must be approved before any can be merged.

Forge adapters implement a minimal `ForgeAdapter` trait (GitHub, GitLab, Gitea). PR comments use a two-layer format: human-readable summary above, machine-readable JSON in HTML comments below.

## Requirement 5: Agent Memory in Git Branches

Vega manages memory as a practice of recorded observation. Each memory entry is an "observation record" stored in `refs/watchers/memory/<namespace>/<observation-id>`:

| Field | Description |
|-------|-------------|
| `observation_id` | Unique identifier |
| `observed_at` | ISO 8601 timestamp |
| `observer` | Agent ID |
| `content` | The observation payload |
| `confidence` | certain, probable, uncertain |
| `context` | Task ID, branch, provider |
| `ttl_days` | Expiration (default: 30, extended on reuse) |

Relevance scoring uses semantic similarity with a confidence-weighted boost: certain memories score higher than uncertain ones at equal similarity. Memory GC runs weekly, removing expired observations and compacting the ref namespace.

The Watchers' unique memory feature is "uncertainty tracking." When an agent encounters contradictory information, both observations are stored with `confidence: uncertain` and a cross-reference linking them. Resolution is deferred to the next human review. This prevents agents from silently resolving ambiguity.

## Requirement 6: Signed Commits via OpenWallet

Mizar manages signing as attestation. The signing workflow:
1. Verify agent authorization for the target branch.
2. Verify patch integrity (hash of INDEX.patch matches expected value).
3. Sign using the agent's OpenWallet DID key.
4. Record the attestation in the vigil log.

Trust is cumulative: agents that consistently produce valid, well-reviewed patches earn higher trust scores in Mizar's internal registry. Trust scores do not affect signing (all authorized agents can sign) but are exposed in PR metadata for human reviewers.

Key rotation every 45 days with a 10-day overlap period.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Polaris | Patch generation | 7,000 | 3,500 | 10,500 |
| Vega | Memory & observation | 5,500 | 1,000 | 6,500 |
| Sirius | Provider stewardship | 3,000 | 800 | 3,800 |
| Aldebaran | PR coordination | 4,500 | 1,500 | 6,000 |
| Mizar | Signing & trust | 2,500 | 500 | 3,000 |
| **Per-task total** | | **22,500** | **7,300** | **29,800** |

Budget reflects the Watchers' conservative approach: generous input allocation for thorough observation, modest output reflecting careful, deliberate patches.

## Unique Domain Insight

Astronomical observation taught us that the most important data often comes from the gaps — the objects you did not see, the orbits you cannot predict, the debris too small to track. Our agents are designed to recognize and flag uncertainty rather than paper over it. When an agent cannot determine the correct change with confidence, it says so explicitly. A patch marked `confidence: uncertain` with a clear explanation of the ambiguity is more valuable than a confident patch that silently resolves ambiguity in the wrong direction.

This "epistemic humility" is rare in AI agent systems, which tend to optimize for confident output. We believe it is essential for systems that operate on codebases humans depend on.

---

*The vigil continues. Peace be upon the firmament.*
