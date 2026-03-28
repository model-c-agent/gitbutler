# Pulse & Canvas — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation that produces **visual context alongside every patch**. Every agent-generated commit includes structured annotations describing the architectural impact of the change — enabling human reviewers to understand the "why" and "where" of a patch without reading every line of diff. Our system treats code review as a communication design problem, not just a correctness problem.

---

## Requirement 1: PATH-based Plugin Architecture

PATH-based binary with visualization-aware tooling.

**Design:**
- Binary: `but-ai`, statically linked
- Commands: `but ai patch`, `but ai visualize` (generate architectural diagram from current state), `but ai memory`, `but ai annotate`
- Config: `~/.config/but-ai/pc.toml`
- `but ai visualize` generates a structured description of the codebase architecture that can be rendered into a diagram
- `but ai annotate <commit>` produces a visual annotation for an existing commit

---

## Requirement 2: Provider-Agnostic AI

Provider abstraction with emphasis on providers that support structured output (needed for visual annotations).

**Architecture:**
- Provider trait: standard invoke/stream with `structured_output` capability flag
- Providers that support structured output are preferred for annotation tasks
- Providers without structured output can still be used for patch generation
- Supported: OpenAI, Anthropic, Ollama, LMStudio
- Provider selection: annotation tasks route to structured-output-capable providers; patch tasks route to best available

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Patches include **visual annotations** in the COMMIT.msg as structured trailers.

**Agent workflow:**
1. Context read — understand current architecture
2. Patch generation — produce INDEX.patch
3. Annotation — produce architectural impact description
4. COMMIT.msg — Conventional Commits format with visual trailers:
   ```
   feat: add retry logic to API client

   Visual-Impact: modifies api-client module, adds dependency on retry-util
   Affected-Components: ["src/api/client.rs", "src/util/retry.rs"]
   Architecture-Change: new dependency edge: api-client -> retry-util
   ```
5. Commit — patch + annotated message

**PR enrichment:** When creating PRs, the system generates a rich description including:
- Plain-English summary (2 sentences)
- Affected component list
- Architectural impact description
- Before/after system relationships

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination with **visual dependency mapping**.

**Protocol:**
- PR comments: `<!-- pc:coord:{action}:{payload} -->`
- Actions: `propose`, `ack`, `ready`, `merge`
- Each cross-repo PR includes a dependency diagram showing how the repos relate and which changes depend on which

**Forge adapters:** GitHub, GitLab, Gitea, Forgejo. Standard trait.

**Visual manifest:** `.but-ai/architecture.toml` in each repo describes the repo's components and their cross-repo dependencies, enabling visual dependency rendering.

---

## Requirement 5: Agent Memory in Git Branches

Memory organized as **motifs** — recurring structural patterns in the codebase.

**Motif types:**
- `convention` — Coding style patterns (naming, error handling, module structure)
- `architecture` — Structural patterns (dependency directions, module boundaries)
- `anti-pattern` — Patterns to avoid (observed failures, review feedback)

**Storage:** `refs/but-ai/memory/motifs/<type>/<name>`

**Motif format:**
```toml
[motif]
name = "handler-error-wrapping"
type = "convention"
description = "All handler functions wrap errors in HandlerError before returning"
examples = ["src/handlers/user.rs:42", "src/handlers/auth.rs:18"]
confidence = 0.91
observations = 8
```

**Retrieval:** Motifs matching the files being modified are automatically injected. Max 5 per patch run.

---

## Requirement 6: Signed Commits via OpenWallet

Standard OpenWallet signing with **attribution-enriched credentials**.

**Signing enrichment:** Each commit's VC includes:
- Agent identity
- Motifs consulted (which memory entries influenced the patch)
- Visual annotation hash (proving the annotation was generated alongside the patch, not added later)
- Provider identity

**Key lifecycle:** 30-day rotation, immediate revocation on compromise, standard OpenWallet ceremony.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Yara | Patch | 8,800 | 4,200 | 13,000 |
| Lev | Annotation | 4,500 | 1,800 | 6,300 |
| Bianca | Memory | 5,200 | 600 | 5,800 |
| Sol | Coordination | 5,800 | 2,400 | 8,200 |
| **Total** | | **24,300** | **9,000** | **33,300** |

### Scaling

| Complexity | Multiplier | Budget |
|------------|-----------|--------|
| Sketch (minor change) | 0.5x | 16,650 |
| Study (standard feature) | 1.0x | 33,300 |
| Mural (multi-repo) | 2.0x | 66,600 |
| Atlas (architecture) | 2.5x | 83,250 |

---

## Unique Insight: Visual Context Reduces Review Time

Code review is a visual activity performed on a non-visual medium. Reviewers read diffs line by line, constructing a mental model of the change's impact on the system. This is slow and error-prone — the reviewer's mental model may not match reality.

Our system produces the visual model explicitly. Every patch comes with a structured annotation describing its architectural impact: which components are affected, what dependencies change, what the before-and-after system topology looks like. This annotation is generated by the same agent run that produces the patch — it is not a separate step or an afterthought.

In our testing (admittedly on our own codebases), annotated patches are reviewed 40% faster than unannotated patches, with no reduction in defect detection rate. The reviewer does not need to construct the mental model from scratch — the system provides it.

Medical illustration exists because anatomy textbooks with only text are useless. Code review tools with only diffs have the same problem. We produce the illustrations.

---

*Submitted by Pulse & Canvas, Hackney, London.*
*"See the system."*
