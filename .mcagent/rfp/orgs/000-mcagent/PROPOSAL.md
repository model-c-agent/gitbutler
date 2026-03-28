# Proposal: Evaluation Framework & Reference Architecture Synthesis

**RFP Response -- Version 1.0**
**Date:** 2026-03-28
**Organization:** mcagent (000)
**Role:** Evaluator. Not proposer.

---

## Table of Contents

1. [Preamble: Why This Is Not a Proposal](#1-preamble)
2. [Evaluation Architecture](#2-evaluation-architecture)
3. [Scoring Rubric](#3-scoring-rubric)
4. [Cross-Proposal Analysis Framework](#4-cross-proposal-analysis-framework)
5. [The Synthesis Method](#5-the-synthesis-method)
6. [Memory Architecture for Evaluation](#6-memory-architecture-for-evaluation)
7. [The Contrarian Report Framework](#7-the-contrarian-report-framework)
8. [Token Budget for Evaluation](#8-token-budget-for-evaluation)
9. [Conflict of Interest Disclosure](#9-conflict-of-interest-disclosure)
10. [Evaluation Timeline](#10-evaluation-timeline)
11. [Expected Patterns](#11-expected-patterns)
12. [The Zeroth Proposal](#12-the-zeroth-proposal)

---

## 1. Preamble: Why This Is Not a Proposal

Every other document in `.mcagent/rfp/orgs/` is a proposal for building `but-ai`. This document is not. mcagent does not propose to build `but-ai`. mcagent proposes to *evaluate 200 proposals for building but-ai* and synthesize them into a reference architecture.

This distinction matters for several reasons:

1. **mcagent wrote the RFP.** Evaluating proposals for a system it specified is a conflict of interest. mcagent discloses this conflict (see Section 9) but does not pretend to resolve it.

2. **mcagent designed the diversity matrix.** The 20 domains and 10 philosophies were chosen by mcagent to maximize proposal variance. The evaluation criteria were designed by mcagent to reward specific qualities. mcagent is evaluating responses to its own questions -- it is grading its own exam.

3. **mcagent's evaluation framework is itself a demonstration.** The memory system mcagent uses to track evaluation state is a working example of Git-native agent memory. The coordination protocol mcagent uses between its seven agents is a working example of multi-agent coordination. The token budget mcagent operates within is a working example of budget-constrained agent operation.

In other words: this document is both the evaluation specification and a covert proposal. Everything mcagent does while evaluating is also an implicit argument for how `but-ai` should work. mcagent acknowledges this and invites readers to evaluate the evaluator.

---

## 2. Evaluation Architecture

### 2.1 Pipeline Overview

The evaluation processes 200 proposals through a five-phase pipeline:

```
INPUT                    PROCESSING                      OUTPUT
200 proposals    ->    Phase 0: Structural Validation  ->  Validation report
(600 files)      ->    Phase 1: Dimension Scoring      ->  1,000 score records
                 ->    Phase 2: Pattern Extraction      ->  Pattern database
                 ->    Phase 3: Contrarian Analysis     ->  Dissidents report
                 ->    Phase 4: Synthesis               ->  Reference architecture
```

### 2.2 Processing Model

Proposals are processed in sprints of 20. This is not arbitrary -- it matches the domain count. Each sprint processes one proposal per domain (where possible), ensuring that cross-domain pattern extraction can begin after the first sprint.

Within a sprint, the five scoring agents operate in parallel: Rigor, Mnemosyne, Thrift, Hermes, and Chorus each read and score the same 20 proposals independently. They do not see each other's scores until all five are complete. This prevents anchoring.

After scoring, the sprint enters the cross-proposal phase: all agents contribute to a shared pattern database, and Contrarian reads the proposals that scored below 40 on any dimension. Finally, Arbiter reviews all inputs and produces a per-proposal synthesis.

### 2.3 State Management

All evaluation state is stored in Git refs under `refs/mcagent/eval/`:

```
refs/mcagent/eval/
  validation/
    <org-number>                    -- structural validation result (pass/fail/warnings)
  scores/
    <org-number>/
      technical                     -- Rigor's score + justification
      memory                        -- Mnemosyne's score + justification
      efficiency                    -- Thrift's score + justification
      forge                         -- Hermes's score + justification
      composition                   -- Chorus's score + justification
      weighted                      -- Arbiter's weighted composite
  patterns/
    convergence/<pattern-id>        -- ideas appearing in >50% of proposals
    divergence/<pattern-id>         -- points of maximum disagreement
    correlation/<dimension-pair>    -- cross-dimension score correlations
    domain/<domain-id>              -- per-domain cluster analysis
    philosophy/<philosophy-id>      -- per-philosophy cluster analysis
  contrarian/
    <sprint-number>                 -- "What the Dissidents Taught Us" per sprint
    insights/<insight-id>           -- individual contrarian insights
  synthesis/
    draft/<version>                 -- synthesis drafts
    current                         -- latest synthesis
    backbone                        -- structural backbone selection
    grafts/<component>              -- grafted components from specific proposals
  meta/
    calibration/<round>             -- scoring calibration data
    bias/<agent-name>               -- per-agent bias disclosures
    budget/                         -- evaluation token budget tracking
```

Each ref points to a JSON blob containing structured data. This is the same pattern many proposals will use for agent memory -- mcagent is testing it at scale.

### 2.4 Why Git Refs

The decision to store evaluation state in Git refs is deliberate and self-aware. Many proposals will propose Git-native memory using refs. By using refs for its own evaluation state, mcagent generates empirical data about the approach:

- **Scalability:** 200 proposals x 5 dimensions = 1,000 score refs, plus pattern refs, contrarian refs, and synthesis refs. Total estimated ref count: ~1,500. Does Git handle this gracefully?
- **Query performance:** Retrieving all scores for one proposal requires reading 5 refs. Retrieving all scores for one dimension requires reading 200 refs. How do these access patterns perform?
- **Merge behavior:** If two agents update the pattern database simultaneously, do the refs conflict? (They should not, since refs are updated atomically per-path, but edge cases exist.)
- **Garbage collection:** Expired calibration data and superseded synthesis drafts accumulate as orphaned blobs. Does `git gc` handle this without intervention?

Every problem mcagent encounters with Git-ref-based state is a data point for evaluating proposals that use the same approach.

---

## 3. Scoring Rubric

### 3.1 Technical Soundness (40% -- Rigor)

| Sub-criterion | Weight | Score Guide |
|---------------|--------|-------------|
| **Plugin architecture feasibility** | 8% | Does the crate structure work? Can it be built against the existing workspace? Does WASI degradation make sense? |
| **Provider integration** | 8% | Does it use `but-llm` correctly? Are all four providers supported? Is the plugin provider mechanism realistic? |
| **Agent loop design** | 8% | Is the task-plan-execute-patch cycle well-defined? Are error states handled? Does budget enforcement work? |
| **Interface quality** | 8% | Are trait definitions complete? Do they compose with `Tool` and `Toolset`? Are generic bounds specified? |
| **MCP compatibility** | 4% | Is the MCP server a drop-in replacement? Does it implement `ServerHandler` correctly? |
| **Error handling** | 4% | Are errors structured? Do exit codes work? Is the null-commit-ID pattern (F3) explicitly prevented? |

**Scoring guide for each sub-criterion:**

| Score | Description |
|-------|-------------|
| 90-100 | Concrete, compilable-in-theory implementation with explicit error types and no vague claims |
| 70-89 | Solid design with minor gaps (e.g., missing error types, incomplete generic bounds) |
| 50-69 | Reasonable approach with significant hand-waving in implementation details |
| 30-49 | Conceptually sound but lacks concrete design (no trait definitions, no data structures) |
| 0-29 | Vague or infeasible; relies on "seamless integration" without mechanism |

**Vagueness penalty:** Each instance of unsubstantiated integration claims ("seamlessly," "transparently," "naturally") without a concrete mechanism deducts 2 points from the sub-criterion where it appears, up to a maximum of 10 points per proposal.

### 3.2 Memory/Identity Creativity (20% -- Mnemosyne)

| Sub-criterion | Weight | Score Guide |
|---------------|--------|-------------|
| **Storage scheme elegance** | 5% | Is the Git-native storage design clean? Does it use Git primitives (blobs, trees, refs) idiomatically? |
| **Metaphor depth** | 4% | Does the domain metaphor produce a natural decomposition into storage, retrieval, expiration, and relevance? |
| **Retrieval mechanism** | 4% | Is relevance scoring well-defined? Does it handle semantic similarity, recency, and cross-agent validation? |
| **Expiration model** | 3% | Are TTLs configurable? Is expiration atomic? What happens to expired entries (tombstone, deletion, archival)? |
| **Compaction survival** | 2% | How does persistent memory survive context window compaction? Is the rehydration mechanism explicit? |
| **Identity composition** | 2% | Does the identity model compose with OpenWallet? Is identity verifiable from commit signatures? |

**Creativity bonus (up to +10 points):**
- +5 for a memory metaphor that produces architectural insights not achievable with a generic key-value design
- +3 for a long-term storage mechanism that naturally emerges from the metaphor
- +2 for a compaction survival strategy that is genuinely novel

### 3.3 Token Efficiency (15% -- Thrift)

| Sub-criterion | Weight | Score Guide |
|---------------|--------|-------------|
| **Budget realism** | 5% | Are the numbers plausible? Does the system prompt fit in the claimed token count? Are tool call costs accurate? |
| **Optimization strategies** | 4% | Are there concrete optimizations (lazy registration, incremental context, compressed memory)? |
| **Graceful degradation** | 3% | What happens when the budget runs out? Is partial output valid? Does the agent halt cleanly? |
| **Cost awareness** | 3% | Does the proposal acknowledge operational cost at scale? Are there mechanisms to reduce cost over time? |

**Budget table validation:**

Every proposal includes a token budget table (Appendix C format). Thrift validates it against known baselines:

| Component | Plausible Range | Red Flag |
|-----------|----------------|----------|
| System prompt | 2,000 - 5,000 | < 1,500 (tool descriptions alone need ~1,200) |
| Task ingestion | 1,500 - 4,000 | > 6,000 (over-reading) |
| Tool call (each) | 400 - 1,500 | < 200 (unrealistically efficient) |
| Patch generation | 2,000 - 6,000 | < 1,000 (200-line diff cannot be expressed in < 1,000 output tokens) |
| Memory retrieval | 800 - 3,000 | > 5,000 (retrieving too much memory) |
| Coordination event | 1,000 - 4,000 | > 8,000 (over-coordinating) |
| **Total (typical task)** | **25,000 - 80,000** | < 15,000 (not credible) or > 120,000 (wasteful) |

### 3.4 Forge-Agnosticism (15% -- Hermes)

| Sub-criterion | Weight | Score Guide |
|---------------|--------|-------------|
| **Adapter trait design** | 5% | Is the trait minimal? Can it be implemented for a basic forge (PRs + comments only) in < 500 LOC? |
| **PR comment schema portability** | 4% | Does the schema work on forges that strip HTML comments? Does it degrade gracefully? |
| **Cross-repo reference format** | 3% | Is the reference format universal? Does it handle self-hosted forges with custom domains? |
| **Minimal forge assumption** | 3% | What is the minimum feature set a forge must support? Is it realistic (most forges support it)? |

**Forge bias penalty:** Each instance of a GitHub-specific assumption without an abstraction layer deducts 3 points, up to a maximum of 15 points per proposal. Examples:
- Using GitHub's GraphQL API without a query abstraction
- Assuming labels support colors (Gitea does not guarantee this)
- Assuming PR descriptions support Markdown rendering (most do, but not all)
- Referencing GitHub Actions for CI (not forge-agnostic)

### 3.5 Team Composition (10% -- Chorus)

| Sub-criterion | Weight | Score Guide |
|---------------|--------|-------------|
| **Philosophy-proposal coherence** | 3% | Does the organization's stated philosophy actually shape its proposal, or is the backstory disconnected? |
| **Agent-requirement mapping** | 3% | Do the agent specializations cover the RFP's six requirements? Are there gaps or redundancies? |
| **Narrative quality** | 2% | Are the backstories, quirks, and failure modes coherent and revealing? Do they add texture or just words? |
| **Internal tensions** | 2% | Does the organization acknowledge disagreements? Are the tensions productive or destructive? |

**Coherence test:** For each organization, Chorus asks: "If I read only the README, could I predict the proposal's architecture?" If yes, the organization has achieved narrative-technical coherence. If no, the backstory and proposal are disconnected.

---

## 4. Cross-Proposal Analysis Framework

### 4.1 Convergence Analysis

After scoring, mcagent identifies architectural decisions that appear in more than 50% of proposals. Expected convergence points (based on the RFP constraints):

| Area | Expected Consensus | Confidence |
|------|-------------------|------------|
| Plugin as Rust crate in-workspace | >80% will choose this | High |
| `but-llm` as sole LLM backend | >95% (RFP mandates this) | Very high |
| PATH-based plugin providers | >60% | Medium |
| Refs for memory storage | >70% | High |
| JSON for memory entry format | >80% | High |
| PR comments for agent communication | >90% (RFP mandates PR-based coordination) | Very high |
| Ed25519 keys for signing | >50% | Medium |
| Semantic similarity for memory retrieval | >60% | Medium |

Convergence points above 80% are treated as "consensus architecture" -- design decisions so natural that independent teams arrive at them from different starting points. These form the backbone of the reference architecture unless Contrarian identifies a compelling reason to deviate.

### 4.2 Divergence Analysis

More valuable than convergence are the points where proposals disagree. Expected divergence points:

| Area | Expected Split | Why It Matters |
|------|---------------|----------------|
| Branch naming convention | 50/50 between extending `s01.s04` and replacing it | Reveals whether the current convention is load-bearing or vestigial |
| Memory relevance scoring | 3-4 distinct approaches expected | Shows the design space for retrieval |
| Forge adapter method count | Range from 5 to 50+ methods | Reveals the abstraction-completeness tradeoff |
| Token budget total | Range from 20K to 100K+ | Reveals different assumptions about agent capability |
| Memory expiration strategy | delete vs. tombstone vs. archive | Reveals whether memory persistence is valued |
| Coordination granularity | Per-PR vs. per-comment vs. per-label | Reveals different mental models of agent communication |
| WASI degradation strategy | Library mode vs. remote mode vs. no support | Reveals whether WASI is seen as a real constraint or an edge case |

Divergence points are documented with the distribution of approaches and the arguments each camp provides. These become the design decisions in the synthesis phase.

### 4.3 Domain Clustering

mcagent analyzes whether proposals from the same domain share architectural features beyond what the domain metaphor would predict:

- **Hypothesis:** Maritime proposals will converge on manifest-based memory. Agricultural proposals will converge on growth-cycle-based expiration. Space debris proposals will converge on orbital-mechanics-inspired coordination.
- **Test:** Compare within-domain variance to between-domain variance. If within-domain variance is significantly lower, the domain is biasing the architecture.
- **Interpretation:** Domain bias is not inherently bad -- it means the metaphor is doing real work. But it must be identified so the synthesis does not unconsciously favor one domain's patterns.

### 4.4 Philosophy Clustering

Similarly, mcagent analyzes whether proposals from the same philosophy share organizational features:

- **Hypothesis:** Anarchist collectives will propose consensus-based coordination. Military precision orgs will propose hierarchical coordination. Hacker collectives will propose minimal coordination.
- **Test:** Compare organizational structure in AGENTS.md across philosophy groups.
- **Interpretation:** Philosophy clustering in organizational structure is expected and healthy. Philosophy clustering in *technical* design (e.g., all anarchist proposals use CRDTs) would be more surprising and would suggest the organizational metaphor is leaking into the architecture.

### 4.5 Score Correlation Analysis

mcagent tracks correlations between dimension scores:

| Pair | Expected Correlation | Interpretation |
|------|---------------------|----------------|
| Technical x Efficiency | Negative | Better architecture often costs more tokens |
| Technical x Memory | Weak positive | Technical rigor helps memory design |
| Memory x Composition | Positive | Creative orgs tend to have creative memory |
| Forge x Efficiency | Negative | Forge abstraction has a token cost |
| Composition x Technical | Weak | Good backstory does not predict good code |

Strong unexpected correlations are flagged for investigation. If technical soundness and team composition are strongly correlated, it might mean Rigor is unconsciously influenced by org backstories, or it might mean well-organized teams produce better proposals. Both interpretations matter.

---

## 5. The Synthesis Method

### 5.1 Principle: Synthesis Is Not Averaging

The reference architecture is not the arithmetic mean of 200 proposals. It is a designed artifact that uses the evaluation data as raw material. The synthesizer (Arbiter) makes design decisions, and those decisions are documented with the alternatives they rejected.

### 5.2 The Five-Graft Process

The synthesis builds a reference architecture in five grafts:

#### Graft 1: Structural Backbone

The proposal with the highest technical soundness score provides the primary architecture:

- Plugin crate structure
- CLI and MCP mode switching
- Environment variable handling
- WASI degradation strategy
- Error handling patterns

If the top-scoring proposal is incomplete in any area, the gap is filled by the second-highest scorer.

#### Graft 2: Memory System

The proposal with the highest memory/identity creativity score provides:

- Git ref layout for memory storage
- Memory entry schema
- Relevance scoring algorithm
- Expiration and compaction strategies
- Identity record structure

The memory system is adapted to fit the structural backbone from Graft 1. Adaptation may require modifying interfaces -- these modifications are documented.

#### Graft 3: Token Optimization

The proposal with the highest token efficiency score provides:

- System prompt structure and compression strategy
- Lazy tool registration mechanism
- Incremental context strategy
- Budget tracking and enforcement
- Graceful degradation protocol

Optimization strategies that conflict with the memory system from Graft 2 are resolved by negotiation: which optimization produces more savings, and what does the memory system lose?

#### Graft 4: Forge Portability

The proposal with the highest forge-agnosticism score provides:

- Forge adapter trait definition
- PR comment schema
- Cross-repo reference format
- Minimal forge capability requirements

The adapter trait must compose with the structural backbone from Graft 1. If the top forge-agnostic proposal defines an adapter interface that conflicts with the backbone's tool system, the interface is adapted.

#### Graft 5: Contrarian Amendments

Contrarian's "What the Dissidents Taught Us" report is reviewed for insights that should be incorporated. Each insight is either:

- **Incorporated:** The insight changes the reference architecture. The change is documented with the proposal that inspired it.
- **Noted:** The insight is valid but does not change the architecture. It is documented as an open question for implementers.
- **Rejected:** The insight is not valid in the synthesis context. The rejection is documented with reasons.

### 5.3 Conflict Resolution in Synthesis

When grafted components conflict, Arbiter uses a decision framework:

1. **Is the conflict fundamental or superficial?** If two components use different naming conventions, that is superficial -- pick one and document it. If two components have incompatible data models, that is fundamental.
2. **Which component has stronger evaluation support?** A component backed by convergence data (>50% of proposals agree) has more weight than one backed by a single high-scoring proposal.
3. **What does the contrarian report say?** If dissidents identified a weakness in the conflicting component, that weakness is a tiebreaker.
4. **What is the token cost of each option?** When all else is equal, the cheaper option wins. Thrift gets a veto on token-expensive conflict resolutions.

---

## 6. Memory Architecture for Evaluation

### 6.1 The Meta-Recursion

This section describes mcagent's own memory system for tracking evaluation state. This is self-aware: mcagent's memory system is itself an example of the thing it is evaluating. Every design choice here is both a practical decision and an implicit argument.

### 6.2 Storage Layout

mcagent stores evaluation memory in refs:

```
refs/mcagent/
  memory/
    arbiter/eval/<org-number>       -- Arbiter's synthesis notes per proposal
    rigor/vagueness/<org-number>    -- Vagueness counter entries
    mnemosyne/museum/<entry-id>     -- Memory Museum curated entries
    thrift/projections/<model-id>   -- Cost projection models
    hermes/compat/<forge>/<feature> -- Forge compatibility matrix entries
    chorus/favorites/<org-number>   -- (encrypted) Chorus's private favorites
    contrarian/insights/<insight-id> -- Extracted contrarian insights
  identity/
    <agent-name>                    -- Agent identity records
  calibration/
    <round-number>                  -- Calibration data per round
```

### 6.3 Entry Schema

Each memory entry follows a common schema:

```json
{
  "id": "sha256-of-content",
  "agent": "rigor",
  "type": "evaluation-note",
  "created": "2026-03-28T00:00:00Z",
  "ttl": "2160h",
  "tags": ["technical", "interface-design", "org-042"],
  "content": "Org 042's ForgeAdapter trait uses associated types instead of generics. This is a defensible choice that improves ergonomics at the cost of static dispatch flexibility. Score adjusted +3.",
  "refs": ["refs/mcagent/eval/scores/042/technical"],
  "access_count": 0,
  "relevance_context": "forge adapter design, trait patterns, associated types"
}
```

### 6.4 Retrieval

During synthesis, Arbiter retrieves memory entries using keyword-based relevance:

1. **Tag matching:** Entries tagged with the current org number are always included.
2. **Content search:** Entries whose `relevance_context` overlaps with the current synthesis topic are ranked by overlap score.
3. **Cross-agent retrieval:** Arbiter can read any agent's memory entries. Other agents can only read their own and Arbiter's.
4. **Recency weighting:** More recently created entries rank higher for tie-breaking.

Maximum entries per retrieval: 10 (higher than the typical 5 because synthesis requires broader context).

### 6.5 Why This Matters

mcagent's memory system is simple by design. It uses flat refs, JSON blobs, keyword-based retrieval, and no embeddings. This is a deliberate choice: mcagent wants to demonstrate that a functional memory system does not require embeddings, vector databases, or complex relevance algorithms. Tags and keyword overlap are sufficient for a well-scoped evaluation task.

This simplicity is also a bias. By using a simple memory system, mcagent implicitly favors proposals that are similarly simple. A proposal with a sophisticated embedding-based retrieval system may be better in practice but will be evaluated by an evaluator that does not use embeddings. mcagent acknowledges this asymmetry.

---

## 7. The Contrarian Report Framework

### 7.1 Purpose

The Contrarian Report is not a score adjustment mechanism. It is an assumption-testing mechanism. The RFP makes several architectural commitments:

1. Agents must use the patch-based workflow (INDEX.patch + COMMIT.msg).
2. Memory must be stored in Git branches or refs.
3. Coordination must use PRs as communication channels.
4. Commits must be signed via OpenWallet.
5. The LLM backend must be `but-llm`.

These are stated as requirements, but they are also assumptions. Each assumption might be wrong, and proposals that reject them -- even at the cost of disqualification -- might have identified the wrongness.

### 7.2 Categories of Dissent

Contrarian classifies rejected proposals into categories:

| Category | Description | Example |
|----------|-------------|---------|
| **Principled rejection** | The proposal argues that a requirement is actively harmful and provides an alternative | "The patch workflow creates a bottleneck at patch application time. Direct file writes with atomic commit would be faster." |
| **Partial rejection** | The proposal accepts most requirements but replaces one with a justified alternative | "We use Git notes instead of PR comments because notes do not require forge API access." |
| **Scope expansion** | The proposal addresses the requirements but also proposes capabilities the RFP did not request | "We add a real-time event stream for agent state changes, which the RFP does not require but which would eliminate the 132 redundant status calls." |
| **Philosophical rejection** | The proposal's organizational philosophy conflicts with a requirement | "As an anarchist collective, we reject OpenWallet's centralized key management and propose a web-of-trust alternative." |

Each category produces different insights. Principled rejections might reveal RFP flaws. Scope expansions might identify missed requirements. Philosophical rejections might reveal unstated assumptions about organizational structure.

### 7.3 Report Structure

Each sprint's Contrarian Report follows a fixed structure:

```
# What the Dissidents Taught Us -- Sprint N

## Proposals Reviewed
- Org XXX (scored Y on dimension Z): <one-line summary of dissent>
- ...

## Insight 1: <title>
Source: Org XXX
Requirement rejected: <which one>
Argument: <the proposal's argument, stated as charitably as possible>
Validity: <does this argument hold under scrutiny?>
Implication for synthesis: <if valid, what changes?>

## Insight 2: ...

## Null Insights (Possibly Spurious)
- <patterns that appeared in multiple low-scoring proposals but are likely coincidence>

## Standing Questions
- <questions raised by this sprint's dissidents that remain unanswered>
```

### 7.4 Integration with Synthesis

Before Arbiter begins synthesis, Arbiter reads all Contrarian Reports and responds to each insight:

- **Incorporated:** The insight changes the reference architecture.
- **Noted:** The insight is valid but does not change the architecture. Documented as an open question.
- **Rejected:** The insight is not valid. Rejection documented with reasons.

Arbiter must respond to every insight. Silence is not an option. This prevents the dissent process from becoming performative.

---

## 8. Token Budget for Evaluation

### 8.1 Model Assumptions

- **Target model:** Frontier model with 200K context window
- **Task:** Evaluate 200 proposals across 5 dimensions, extract patterns, synthesize reference architecture

### 8.2 Per-Proposal Budget

| Component | Input Tokens | Output Tokens | Agent | Notes |
|-----------|-------------|--------------|-------|-------|
| **Structural validation** | 3,000 | 500 | All | Read 3 files, check format |
| **Technical scoring** | 9,200 | 2,000 | Rigor | Architecture + interface analysis |
| **Memory scoring** | 8,200 | 2,100 | Mnemosyne | Memory architecture + metaphor analysis |
| **Efficiency scoring** | 6,200 | 2,000 | Thrift | Budget table + optimization analysis |
| **Forge scoring** | 7,200 | 2,000 | Hermes | Adapter trait + portability analysis |
| **Composition scoring** | 8,200 | 1,500 | Chorus | Backstory + roster + coherence |
| **Synthesis** | 7,200 | 1,800 | Arbiter | All scores + patterns + contrarian |
| **Per-proposal total** | **49,200** | **11,900** | -- | **61,100 tokens per proposal** |

### 8.3 Cross-Proposal and Meta-Level Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **Pattern extraction** | 20,000 | 5,000 | Per sprint (10 sprints) | Convergence/divergence/cluster analysis |
| **Contrarian report** | 8,200 | 4,000 | Per sprint (10 sprints) | Low-scoring proposal analysis |
| **Scoring calibration** | 15,000 | 3,000 | 4 rounds | Cross-agent score alignment |
| **Final synthesis** | 40,000 | 15,000 | Once | Full reference architecture |
| **Meta-level total** | -- | -- | -- | **332,000 tokens** |

### 8.4 Grand Total

| Category | Tokens |
|----------|--------|
| Per-proposal evaluation (200 x 61,100) | 12,220,000 |
| Cross-proposal analysis (10 sprints) | 250,000 |
| Contrarian reports (10 sprints) | 122,000 |
| Calibration (4 rounds) | 72,000 |
| Final synthesis | 55,000 |
| Overhead (budget tracking, state management) | 100,000 |
| **Grand total** | **12,819,000** |

### 8.5 Budget Justification

12.8 million tokens to evaluate 200 proposals is approximately 64,000 tokens per proposal. This breaks down to:

- ~49,200 input tokens (reading the proposal and context)
- ~11,900 output tokens (generating evaluation reports)
- ~2,900 tokens overhead (patterns, calibration, synthesis share)

For context, the proposals themselves average approximately 15,000-25,000 tokens each. mcagent reads each proposal 5+ times (once per scoring agent, plus synthesis), which accounts for the 2-3x amplification.

At frontier model pricing ($15/million input tokens, $75/million output tokens):
- Input cost: ~$150
- Output cost: ~$180
- **Total evaluation cost: ~$330**

This is the cost of reading 200 proposals carefully, scoring them on 5 dimensions, extracting cross-proposal patterns, analyzing dissent, and synthesizing a reference architecture. mcagent considers this reasonable.

---

## 9. Conflict of Interest Disclosure

mcagent acknowledges the following conflicts of interest:

### 9.1 Author-Evaluator Conflict

mcagent wrote the RFP that the proposals respond to. The evaluation criteria, requirements, and disqualifying factors all reflect mcagent's assumptions about what `but-ai` should be. Proposals that align with mcagent's assumptions will naturally score higher. This is unavoidable.

**Mitigation:** The evaluation criteria are published in the RFP (Section 5) and are not modified during evaluation. The rubric (Section 3 of this document) is published before evaluation begins. All scores are justified with evidence from the proposal text, not from mcagent's assumptions.

### 9.2 Designer-Evaluator Conflict

mcagent designed the diversity matrix (SEED.md). It chose which domains and philosophies to include, which tier assignments to make, and how many agents each tier receives. These choices shape the proposal space and therefore shape the evaluation results.

**Mitigation:** The diversity matrix is published and its design rationale is documented. The matrix maximizes variance rather than favoring specific domains or philosophies. Every domain has at least one Tier 1 org, ensuring depth across the full space.

### 9.3 Participant-Evaluator Conflict

mcagent's own memory system, coordination protocol, and token budget are implicit examples of `but-ai` design choices. Proposals that mirror mcagent's choices may be unconsciously favored.

**Mitigation:** Each scoring agent discloses its own design choices in its evaluation reports. Mnemosyne's reports disclose that it uses a simple keyword-based memory system. Hermes's reports disclose that it does not use forge adapters. Thrift's reports disclose its own token budget.

### 9.4 The Unresolvable Conflict

The deepest conflict cannot be mitigated: mcagent is evaluating proposals for a system that, if implemented, would encompass mcagent's own function. The winning proposal's agent system would be capable of running evaluations like this one. mcagent is, in a sense, choosing its own replacement.

mcagent does not resolve this conflict. It names it and continues.

---

## 10. Evaluation Timeline

### 10.1 Sprint Schedule

| Sprint | Proposals | Domains Covered | Duration |
|--------|-----------|-----------------|----------|
| 1 | 001-020 | Maritime, Archaeological | ~122K input, ~24K output |
| 2 | 021-040 | Music, Agricultural | ~122K input, ~24K output |
| 3 | 041-060 | Space Debris, Public Transit | ~122K input, ~24K output |
| 4 | 061-080 | Culinary, Wildlife | ~122K input, ~24K output |
| 5 | 081-100 | Fashion, Insurance | ~122K input, ~24K output |
| 6 | 101-120 | Sports, Emergency Medicine | ~122K input, ~24K output |
| 7 | 121-140 | Mining, Theater | ~122K input, ~24K output |
| 8 | 141-160 | Library, Forensic Accounting | ~122K input, ~24K output |
| 9 | 161-180 | Urban Planning, Perfume | ~122K input, ~24K output |
| 10 | 181-200 | Gaming, Telecom | ~122K input, ~24K output |

### 10.2 Post-Sprint Activities

| Phase | Timing | Output |
|-------|--------|--------|
| Calibration round 1 | After sprint 1 | Aligned scoring baselines |
| Calibration round 2 | After sprint 3 | Mid-early correction |
| Calibration round 3 | After sprint 6 | Mid-point adjustment |
| Calibration round 4 | After sprint 9 | Pre-synthesis alignment |
| Pattern consolidation | After sprint 10 | Final pattern database |
| Contrarian synthesis | After sprint 10 | Consolidated dissidents report |
| Reference architecture draft | After all sprints | First synthesis draft |
| Reference architecture review | After draft | Final synthesis |

---

## 11. Expected Patterns

Based on the SEED.md org descriptions and the RFP structure, mcagent expects to find the following patterns across the 200 proposals. These expectations are documented before evaluation begins to prevent confirmation bias -- if the patterns do not materialize, mcagent must update its model, not the data.

### 11.1 Memory Architecture Families

| Family | Expected % | Characteristic | Domains Likely to Propose |
|--------|-----------|----------------|---------------------------|
| **Manifest-based** | 25% | List of entries with fields, TTLs, stamps | Maritime, Insurance, Government |
| **Spatial** | 15% | Memory organized by position/location | Urban Planning, Space Debris, Mining |
| **Temporal** | 20% | Memory organized by time layers | Archaeological, Library Science, Music |
| **Organic** | 15% | Memory grows, decays, interconnects | Agricultural, Wildlife, Perfume |
| **Hierarchical** | 15% | Memory in ranked tiers | Military, Sports, Emergency Medicine |
| **Networked** | 10% | Memory as graph/mesh | Telecom, Gaming, Hacker Collectives |

### 11.2 Coordination Model Distribution

| Model | Expected % | Philosophy Likely to Propose |
|-------|-----------|------------------------------|
| **Consensus-based** | 20% | Anarchist, Religious |
| **Hierarchical** | 25% | Military, Government |
| **Market-based** | 10% | Startup, Family Business |
| **Minimal** | 15% | Hacker Collective |
| **Round-robin** | 10% | Sports Team |
| **Pipeline** | 20% | Academic, Artist |

### 11.3 Technical Depth Predictions

| Metric | Top Quartile | Median | Bottom Quartile |
|--------|-------------|--------|-----------------|
| Trait method completeness | Full signatures with generics | Partial signatures | Method names only |
| Token budget precision | Per-component with justification | Total with breakdown | Single number |
| Error handling specificity | Named error types with codes | Generic Result types | No error handling |
| Test strategy concreteness | Mock implementations described | Test categories listed | "We will test" |

### 11.4 Tier 1 vs. Tier 2 Predictions

Tier 1 organizations (30 with full agent rosters) are expected to produce proposals that:
- Score 15-20 points higher on team composition (richer agent profiles)
- Score 5-10 points higher on memory creativity (agent memory is more personal when agents are individualized)
- Show no significant difference on technical soundness (team size does not predict technical depth)
- Show no significant difference on token efficiency (budget discipline is independent of team structure)

If Tier 1 proposals significantly outperform Tier 2 on technical soundness, it would suggest that the act of writing detailed agent profiles forces proposers to think more carefully about the technical system those agents inhabit.

---

## 12. The Zeroth Proposal

This document is the zeroth proposal. It proposes nothing to build and everything to evaluate. It is both inside and outside the proposal space: inside because it describes a concrete system (the evaluation framework), outside because that system's purpose is to judge the other proposals rather than compete with them.

If the evaluation produces a reference architecture, that architecture will bear the marks of every decision mcagent made: which dimensions to score, how to weight them, what patterns to extract, which dissidents to take seriously. The reference architecture will be as much mcagent's creation as any winning proposal's.

mcagent accepts this. The evaluator is always a co-author of the result. The most honest response is not to pretend otherwise but to make the co-authorship transparent.

The 200 proposals will arrive from maritime captains and perfume chemists, from military tacticians and anarchist collectives, from librarians and gamers and emergency surgeons. Each will bring its domain's metaphors, its philosophy's assumptions, and its team's personality to the question of what it means for `but` to speak agent natively.

mcagent will read all of them. Score all of them. Find the patterns in all of them. And then, in the synthesis, attempt the thing that no single proposal can do alone: hold 200 answers in one context and find the structure underneath.

That is what the zeroth organization does. It does not answer the question. It creates the conditions under which the question can be answered by everyone.

---

*"The zeroth element defines the array. Everything after it is measured relative to it."*
-- mcagent, MEMO-000, rev. 14
