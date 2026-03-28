# Yamazaki Performance Systems — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation built on the principle of **generational refinement**: each agent run builds on the accumulated context of all previous runs, just as each generation of our family built on the previous one's insights. Our system never starts from zero. Memory is not a feature — it is the architecture.

---

## Requirement 1: PATH-based Plugin Architecture

The plugin binary installs to `~/.local/bin/but-ai` and responds to `but ai <command>` invocations.

**Structure:**
- Single binary, statically compiled for target platform
- Subcommands: `but ai patch`, `but ai review`, `but ai memory`, `but ai lineage`
- Config: `~/.config/but-ai/yamazaki.toml`
- The `lineage` command is unique to our proposal: it traces the history of any code change back through the memory entries and agent runs that influenced it — a genealogy of decisions

**Plugin handshake:** `but-ai --protocol-version` returns supported protocol range. `but` validates compatibility before dispatching.

---

## Requirement 2: Provider-Agnostic AI

Provider abstraction follows our family's database philosophy: the backend is interchangeable; the interface is permanent.

**Provider trait:**
```
Provider {
  fn capabilities() -> ProviderCaps
  fn complete(prompt, budget, tools) -> Result<Response>
  fn stream(prompt, budget, tools) -> Result<Stream<Chunk>>
}
```

**Supported:** OpenAI, Anthropic, Ollama, LMStudio
**Selection:** Config-driven with env var override (`BUT_AI_PROVIDER`)
**Adaptation:** Each provider has a "dialect" file mapping generic tool-call schemas to provider-specific formats. Dialects are version-controlled in `refs/yamazaki/dialects/<provider>`.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce INDEX.patch and COMMIT.msg. All file modifications happen through `but` applying the patch.

**Agent workflow:**
1. **Inherit** — Load relevant memories from previous cycles
2. **Observe** — Read codebase context within budget
3. **Generate** — Produce INDEX.patch as unified diff
4. **Annotate** — Produce COMMIT.msg with Conventional Commits format + `Lineage:` trailer listing memory entries that informed the patch
5. **Commit** — `but` applies and commits

**The Lineage trailer** is our key addition: every commit message records which memory entries the agent consulted. This creates an auditable chain from code change back to the pattern knowledge that produced it.

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo work is coordinated through structured PR comments using a family-inspired "consensus dinner" protocol.

**Protocol phases:**
1. **Proposal** — Lead repo opens PR with `<!-- yps:propose:{manifest} -->` comment
2. **Acknowledgment** — Dependent repos open linked PRs and post `<!-- yps:ack:{branch} -->`
3. **Ready** — Each repo signals completion: `<!-- yps:ready:{commit-sha} -->`
4. **Merge** — When all repos are ready, merge proceeds in dependency order

**Forge abstraction:** Trait-based adapters for GitHub, GitLab, Gitea, Forgejo. Forge type auto-detected from remote URL. All communication through PR comments — no external coordination service.

**Dependency manifest:** `.but-ai/deps.toml` in each repo, listing cross-repo dependencies with branch patterns and expected signal format.

---

## Requirement 5: Agent Memory in Git Branches

Memory architecture uses a **generational model** — inspired by our family's three generations of accumulated knowledge.

**Memory generations:**
- `refs/but-ai/memory/ephemeral/<run-id>` — Single run, auto-expires
- `refs/but-ai/memory/working/<task-id>` — Task-scoped, 7-day TTL
- `refs/but-ai/memory/established/<pattern>` — Proven patterns, 90-day TTL
- `refs/but-ai/memory/ancestral/<key>` — Permanent project knowledge, manual curation

**Promotion:** Memories are promoted between generations based on usage frequency. A working memory referenced by 3+ tasks is promoted to established. An established memory referenced by 10+ tasks is promoted to ancestral. Demotion is also possible — an ancestral memory contradicted by recent evidence is demoted to established with a `disputed` flag.

**Format:** Each memory entry is a TOML blob:
```toml
[entry]
key = "error-handling-convention"
generation = "established"
summary = "Use thiserror for library errors, anyhow for binary errors"
evidence = ["src/lib.rs:14", "src/main.rs:8"]
confidence = 0.88
lineage = ["run-042", "run-067", "run-089"]
```

---

## Requirement 6: Signed Commits via OpenWallet

All agent commits are signed. Signature integrity is part of the lineage chain.

**Key management:**
- Generation: automated at agent provisioning
- Rotation: 30-day cycle
- Revocation: immediate with cascading lineage flag — all commits signed by a revoked key are annotated
- Storage: OpenWallet credential, referenced by agent identity

**Signing enrichment:** Beyond standard commit signing, our implementation attaches a Verifiable Credential containing the agent's identity, the memory entries consulted (lineage), and the provider used. This VC is stored in `refs/but-ai/credentials/<commit-sha>`.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Yuki | Patch generation | 9,500 | 4,000 | 13,500 |
| Ren | Coordination/forge | 6,800 | 2,800 | 9,600 |
| Hana | Memory | 6,200 | 800 | 7,000 |
| **Total** | | **22,500** | **7,600** | **30,100** |

### Complexity Scaling

| Tier | Description | Multiplier | Budget |
|------|-------------|-----------|--------|
| Minor league | Single file, <50 lines | 0.5x | 15,050 |
| Regular season | 3 files, ~200 lines | 1.0x | 30,100 |
| Playoffs | 10+ files, cross-repo | 2.0x | 60,200 |
| Championship | Architecture, breaking | 2.5x | 75,250 |

---

## Unique Insight: Memory Lineage as Institutional Knowledge

Most agent memory systems treat memory as cache — store it, retrieve it, expire it. We treat memory as **lineage**. Every memory entry knows where it came from (which run created it), how it has been used (which subsequent runs referenced it), and how confident we are in it (a score updated by every reference).

This means our system can answer questions that no other proposal can:
- "Why did the agent make this code change?" — Trace the commit's lineage trailer to the memory entries that informed it.
- "How reliable is this pattern?" — Check the memory entry's confidence score and reference count.
- "What happens if this pattern is wrong?" — Find every commit whose lineage includes the disputed memory and flag them for review.

Hiroshi Yamazaki kept notebooks because he believed that a statistician who could not explain the history of their own conclusions was not a statistician. We built a system that forces agents to keep the same kind of notebooks.

---

*Submitted by Yamazaki Performance Systems, Kichijoji, Tokyo.*
*Three generations. One question.*
