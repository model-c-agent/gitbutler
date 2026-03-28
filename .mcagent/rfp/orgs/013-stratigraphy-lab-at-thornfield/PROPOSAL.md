# Proposal: `but-ai` Plugin — The Stratigraphy Lab at Thornfield

**RFP Response — Version 1.0**
**Peer Review Status:** [PUBLISHED] — Three reviews completed.
**Date:** 2026-03-28
**Organization:** The Stratigraphy Lab at Thornfield (013)
**Contact:** lab@thornfield.ac.uk

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Plugin Architecture (RFP 3.1)](#2-plugin-architecture)
3. [Provider-Agnostic AI Interface (RFP 3.2)](#3-provider-agnostic-ai-interface)
4. [The But Agent (RFP 3.3)](#4-the-but-agent)
5. [Polyrepo PR-Based Coordination (RFP 3.4)](#5-polyrepo-pr-based-coordination)
6. [Agent Memory & Identity (RFP 3.5)](#6-agent-memory--identity)
7. [Signed Commits via OpenWallet (RFP 3.6)](#7-signed-commits-via-openwallet)
8. [Token Budget (RFP 3.7)](#8-token-budget)
9. [Testing Strategy](#9-testing-strategy)
10. [Trade-offs and Alternatives](#10-trade-offs-and-alternatives)
11. [Migration Path](#11-migration-path)
12. [Git Config Keys](#12-git-config-keys)

---

## 1. Executive Summary

The Stratigraphy Lab proposes a `but-ai` plugin built on the principle that no agent output should be trusted without independent corroboration. Our architecture implements a mandatory three-review protocol for every patch: correctness review, adversarial robustness review, and methodology review. The result is slower output but dramatically higher reliability.

Three academic principles:

1. **Peer review is mandatory.** An agent that approves its own output is committing the equivalent of publishing an unreviewed paper. Every patch passes through three independent review agents before it is signed and committed.
2. **Citation-based trust.** Agent memory entries are trusted in proportion to their corroboration — how many independent observations support them. A memory cited by three tools is more trusted than one cited by one. Circular citations are detected and penalized.
3. **Reproducibility as requirement.** Every agent action must be reproducible. Model versions are pinned, random seeds are logged, and tool call parameters are recorded. If you cannot reproduce it, you cannot trust it.

---

## 2. Plugin Architecture

### 2.1 Approach

`but-ai` is implemented as a Rust crate (`crates/but-ai`) within the existing workspace. It ships as a single binary with CLI and MCP modes.

### 2.2 Design

#### Command Structure

```
but ai
├── but ai submit <task>          — Submit a task for agent execution (triggers full review cycle)
├── but ai review <patch-ref>     — Manually trigger review of a specific patch
├── but ai status                 — Show current review pipeline status
├── but ai memory <subcommand>    — Memory operations
│   ├── but ai memory query       — Query memory with citation-aware ranking
│   ├── but ai memory store       — Store a new memory entry
│   ├── but ai memory cite        — Add a citation link between entries
│   ├── but ai memory graph       — Display the citation graph for a topic
│   └── but ai memory audit       — Check for circular citations
├── but ai identity <subcommand>  — Agent identity management
│   ├── but ai identity register  — Register a new agent
│   ├── but ai identity verify    — Verify agent credentials
│   └── but ai identity roster    — Display the current agent roster
└── but ai mcp                    — MCP server mode (stdio)
```

The `submit` command is the primary entry point. It triggers the full lifecycle: generation, Review 1, Review 2, Review 3, final approval, signing, and commit. The `review` command allows manual intervention in the review pipeline.

#### Environment Contract

Standard environment variables are honored. The plugin adds:

| Variable | Description |
|----------|-------------|
| `BUT_AI_REVIEW_LEVEL` | Current review stage (1, 2, 3, or final) |
| `BUT_AI_MODEL_PIN` | Pinned model version for reproducibility |

#### WASI Degradation

Under WASI:
1. The three-review protocol is reduced to a single self-review (with a warning that the output has not been peer-reviewed).
2. Memory citation graphs are unavailable (too computationally expensive).
3. The MCP server is unavailable; only library-mode access.
4. Output is tagged `[DRAFT]` — not `[PUBLISHED]`.

### 2.3 Trade-offs

**Considered:** A three-binary architecture (one per review stage).
**Rejected:** Three binaries triple the deployment complexity. A single binary with internal role switching is simpler and faster.

---

## 3. Provider-Agnostic AI Interface

### 3.1 Approach

We use `but-llm` exclusively. All four providers are supported. Model versions are pinned for reproducibility.

### 3.2 Design

#### Provider Resolution with Version Pinning

```
1. Read gitbutler.aiModelProvider from Git config
2. Read gitbutler.aiModel from Git config
3. Construct LLMProvider via from_git_config()
4. Pin model version: store the exact model version string in the task record
5. All subsequent calls use the pinned version
6. If provider changes mid-session → error: PROVIDER_VERSION_DRIFT
```

Version pinning ensures reproducibility. If you re-run a task with the same inputs against the same model version, you get the same outputs (within the limits of LLM determinism).

#### Tool Registration

All 10 workspace tools are registered via `WorkspaceToolset`. Each tool call is logged with full parameters and results for reproducibility:

```json
{
  "tool_log": [
    {
      "call_id": 1,
      "tool": "GetProjectStatus",
      "params": {},
      "result_hash": "sha256:abc...",
      "tokens": {"input": 50, "output": 800},
      "timestamp": "2026-03-28T14:00:00.123Z"
    }
  ]
}
```

The `result_hash` allows verification without storing full results (which could be large).

#### Plugin Providers

New providers are added via **review-gated modules**: shared configuration files that declare provider endpoints, authentication methods, and capability flags. A new provider is added by creating a config file in `~/.config/but-ai/providers/`:

```toml
[provider.gemini]
type = "openai-compatible"
endpoint = "https://generativelanguage.googleapis.com/v1beta"
auth = "bearer"
supports_tool_calling = true
supports_streaming = true
model_pattern = "gemini-*"
```

OpenAI-compatible providers (a growing category) work out of the box. Non-compatible providers require a shim executable, discovered via PATH as `but-ai-provider-<name>`.

### 3.3 Trade-offs

**Considered:** Allowing dynamic model switching during a task.
**Rejected:** Dynamic switching breaks reproducibility. The model is pinned at task start and fixed for the duration.

---

## 4. The But Agent

### 4.1 Approach

The But Agent operates as an academic research project: a hypothesis (the task) is investigated (execution), the results are documented (patch + message), and the documentation is peer-reviewed before publication (commit).

### 4.2 Design

#### Agent Lifecycle

```
Phase 1 — PROPOSAL
  Generator agent (Osei) reads the task.
  Generator produces a research proposal: what will be changed, why, and how.
  PI (Voss) reviews the proposal. If rejected → revise and resubmit.

Phase 2 — EXECUTION
  Generator executes the approved proposal.
  Generator uses workspace tools to implement the changes.
  Generator produces INDEX.patch + COMMIT.msg.
  All tool calls are logged for reproducibility.

Phase 3 — REVIEW CYCLE
  Review 1 (Harada): Correctness check.
    - Does the patch do what the proposal claimed?
    - Does it follow project conventions?
    - Are there obvious bugs?
  Review 2 (Chakraborty): Robustness check.
    - Can the patch be broken by adversarial inputs?
    - Are there edge cases?
    - Does it handle error conditions?
  Review 3 (Lindqvist): Methodology check.
    - Was the process reproducible?
    - Were tool calls logged?
    - Can the patch be regenerated from the recorded inputs?

Phase 4 — PUBLICATION
  PI (Voss) gives final approval.
  Memory specialist (Mirza) stores lessons learned.
  Security agent signs the commit via OpenWallet.
  Commit is created with [PUBLISHED] tag.
```

#### Task Sources

| Source | Read Method |
|--------|-------------|
| CLI argument | `but ai submit "implement feature X"` |
| PR body | Parse via forge adapter |
| Branch metadata | Read from `refs/but-ai/thornfield/tasks/<branch>` |
| Issue description | Parse via forge adapter |

#### Branch Naming

Academic convention with revision tracking:

```
thornfield/<project>/<task-id>/v<revision>

Examples:
  thornfield/auth-refactor/T001/v1     — First version of the patch
  thornfield/auth-refactor/T001/v2     — Revised after review (previous version preserved)
  thornfield/auth-refactor/T002/v1.T001-v2  — Task 002, depends on Task 001 revision 2
```

Each revision is a separate branch. Previous revisions are never deleted — they are the publication history.

#### Token Budget Enforcement

The PI allocates token budgets in the proposal phase. The budget is divided:

| Phase | Share |
|-------|-------|
| Generation | 40% |
| Review 1 | 15% |
| Review 2 | 15% |
| Review 3 | 10% |
| Final review | 10% |
| Memory + signing | 10% |

When any phase exhausts its share, it produces its best output with the remaining tokens and flags the output as `[UNDER REVIEW — BUDGET CONSTRAINED]`. The PI decides whether to allocate additional budget or accept the constrained output.

### 4.3 Trade-offs

**Considered:** Skipping reviews for simple patches.
**Rejected:** "The Silent Reviewer" incident proved that even apparently simple patches can contain subtle errors. Three reviews, always. No exceptions.

**Considered:** Parallel reviews (all three reviewers simultaneously).
**Rejected:** Reviews are sequential by design. Review 2 (robustness) should focus on areas that Review 1 (correctness) approved — not duplicate work. Sequential reviews build on each other.

---

## 5. Polyrepo PR-Based Coordination

### 5.1 Approach

PRs are publications. Comments are peer reviews. Cross-repo references are citations. The forge is the academic journal system.

### 5.2 Design

#### Forge Adapter Trait

```rust
trait ForgeAdapter: Send + Sync {
    fn submit_publication(&self, repo: &RepoRef, title: &str, body: &str, head: &str, base: &str) -> Result<PrRef>;
    fn post_review(&self, pr: &PrRef, review: &PeerReview) -> Result<ReviewRef>;
    fn list_reviews(&self, pr: &PrRef) -> Result<Vec<PeerReview>>;
    fn set_review_status(&self, pr: &PrRef, status: ReviewStatus) -> Result<()>;
    fn get_review_status(&self, pr: &PrRef) -> Result<ReviewStatus>;
    fn cite(&self, from: &PrRef, to: &PrRef, context: &str) -> Result<()>;
    fn list_citations(&self, pr: &PrRef, direction: CitationDirection) -> Result<Vec<Citation>>;
    fn resolve_citation(&self, reference: &str) -> Result<PrRef>;
    fn forge_type(&self) -> ForgeType;
}
```

Nine methods. The terminology is academic (submit publication, post review, cite) but the underlying operations are standard forge operations.

#### Review Status Badges

Every PR carries a badge in its title:

| Badge | Meaning |
|-------|---------|
| `[DRAFT]` | Generated but not yet reviewed |
| `[REVIEW 1/3]` | First review completed |
| `[REVIEW 2/3]` | Second review completed |
| `[UNDER REVIEW]` | All three reviews completed, awaiting PI approval |
| `[PUBLISHED]` | Approved and committed |
| `[RETRACTED]` | Approved but later found to contain errors |

#### PR Comment Schema (Peer Review Format)

```markdown
<!-- but-ai:peer-review -->
<!-- review-stage: 1 | 2 | 3 | final -->
<!-- reviewer: harada | chakraborty | lindqvist | voss -->
<!-- verdict: approve | revise | reject -->
<!-- date: 2026-03-28 -->

## Peer Review — Stage 1 (Correctness)

**Reviewer:** Dr. Harada
**Verdict:** REVISE
**Review Status:** [REVIEW 1/3 — REVISE]

### Findings

1. **[RED — Must Fix]** Line 47: `processData` violates snake_case convention. Rename to `process_data`.
2. **[YELLOW — Should Fix]** Line 112: Error message is not descriptive. Consider adding the parameter name.
3. **[GREEN — Suggestion]** Line 89: This block could be extracted into a helper function for clarity.

### Token Usage
- Review input: 4,000 tokens
- Review output: 2,000 tokens
- Budget remaining: 36,000 / 50,000

### Recommendation
Revise and resubmit. Blocking issue (finding 1) must be addressed before advancing to Review 2.
```

#### Cross-Repo Citations

Cross-repo references use academic citation format:

```
[Osei2026-T001] github:gitbutler/but#123
[Mirza2026-M042] gitlab:thornfield/memory#7
```

The citation key (`[Author-Year-ID]`) provides immediate human context. The full reference resolves to a forge PR. Citations are bidirectional — citing a PR in another repo automatically creates a back-reference.

### 5.3 Trade-offs

**Considered:** Using PR labels for review status instead of title badges.
**Rejected:** Labels are not visible in all forge UIs without clicking into the PR. Title badges are immediately visible in PR lists.

---

## 6. Agent Memory & Identity

### 6.1 Approach: Citation-Chain Memory

Our memory architecture is modeled on academic citation networks. Every memory entry is a "paper" with a title, content, references (citations to other entries), and a trust score computed from its citation count and the authority of its citers. Memory retrieval ranks entries by citation-weighted trust, not just semantic similarity.

This is fundamentally different from:
- TPC's consensus-weighted manifest (ranks by popularity among peers)
- Iron Wake's classified filing system (ranks by source reliability and classification)
- Shard & Bone's stratigraphic layers (ranks by temporal position and cross-reference density)

Our system ranks by academic authority: entries that are cited by many trusted entries are themselves trusted. This creates a self-reinforcing trust network that mirrors how academic knowledge accumulates.

### 6.2 Design

#### Storage: The Citation Graph

Memory is stored as a directed graph of citations:

```
refs/but-ai/thornfield/papers/
├── <entry-hash>          → memory entry (the "paper")
├── CITATION_INDEX         → edge list of all citations
└── AUTHORITY_SCORES       → precomputed trust scores (refreshed periodically)
```

Each memory entry:

```json
{
  "id": "thornfield-2026-0342",
  "title": "Authentication module uses provider trait pattern",
  "abstract": "The auth module defines an AuthProvider trait with 4 implementations. New providers implement the trait and register in auth/mod.rs.",
  "content": "Detailed observation: The AuthProvider trait is defined at auth/provider.rs:12. It requires three methods: authenticate(), refresh(), and revoke(). The four implementations are OAuthProvider, SAMLProvider, APIKeyProvider, and SessionProvider.",
  "author": "osei",
  "date": "2026-03-28",
  "references": ["thornfield-2026-0298", "thornfield-2026-0315"],
  "cited_by": ["thornfield-2026-0350", "thornfield-2026-0361"],
  "evidence": {
    "tool_calls": ["GetCommitDetails:abc123", "GetBranchChanges:def456"],
    "model_version": "claude-opus-4-20250514",
    "reproducible": true
  },
  "review_status": "PUBLISHED",
  "reviewed_by": ["harada", "chakraborty", "lindqvist"],
  "ttl_hours": 720,
  "created": "2026-03-28T14:00:00Z"
}
```

#### The Citation Index

The `CITATION_INDEX` is an adjacency list storing all citation relationships:

```json
{
  "edges": [
    {"from": "thornfield-2026-0342", "to": "thornfield-2026-0298", "type": "supports"},
    {"from": "thornfield-2026-0342", "to": "thornfield-2026-0315", "type": "extends"},
    {"from": "thornfield-2026-0350", "to": "thornfield-2026-0342", "type": "corroborates"}
  ]
}
```

Citation types:
- **supports:** This entry provides evidence for the cited entry
- **extends:** This entry adds new information to the cited entry
- **corroborates:** This entry independently confirms the cited entry
- **contradicts:** This entry presents evidence against the cited entry
- **supersedes:** This entry replaces the cited entry (the cited entry is still preserved)

#### Retrieval: Citation-Weighted Trust Scoring

When retrieving memory, entries are ranked by a composite score:

**Trust score = 0.30 * semantic + 0.30 * authority + 0.20 * recency + 0.20 * review_status**

Where:

- **Semantic similarity** (0.30): Cosine similarity between query embedding and entry embedding.
- **Authority score** (0.30): PageRank-inspired score computed from the citation graph. Entries cited by many high-authority entries have high authority. Computed as:
  ```
  authority(entry) = (1 - d) + d * Σ(authority(citer) / out_degree(citer))
  ```
  where d = 0.85 (damping factor, same as PageRank).
- **Recency** (0.20): Exponential decay from creation date.
- **Review status** (0.20): PUBLISHED = 1.0, UNDER_REVIEW = 0.7, DRAFT = 0.3.

The top 5 entries are returned. Entries scoring below 0.25 are excluded.

#### Circular Citation Detection

Mirza's circular citation fear is addressed by a cycle detection algorithm that runs whenever a new citation is added:

1. Add the proposed citation edge to the graph.
2. Run DFS from the cited entry.
3. If the DFS reaches the citing entry, a cycle exists.
4. Reject the citation and flag both entries for review.

This prevents the "citation ring" problem where entries inflate each other's authority through mutual citation.

#### Expiration

Entries expire based on their review status:

| Status | Default TTL | Rationale |
|--------|-------------|-----------|
| PUBLISHED | 720h (30d) | Peer-reviewed knowledge remains relevant |
| UNDER_REVIEW | 168h (7d) | Unfinished review should not persist indefinitely |
| DRAFT | 48h (2d) | Unreviewed entries expire quickly |
| Identity | Never | Agent identity records never expire |

Expired entries are not deleted — they are marked `ARCHIVED` and excluded from default queries. Their citations remain in the graph (an archived entry can still contribute to the authority of entries that cite it).

#### Compaction Survival

When the context window is compacted:

1. All PUBLISHED memory entries currently in context are preserved as "established findings."
2. DRAFT and UNDER_REVIEW entries are discarded from context (they can be re-retrieved from refs).
3. A "literature review summary" is generated: a condensed overview of the established findings.
4. The summary is injected into the post-compaction context with instructions to query memory for details.

The literature review summary is ~500 tokens — much smaller than the full set of memory entries it replaces.

#### Long-Term Storage: The Institutional Repository

Cross-session, cross-repo memory is stored in an institutional repository:

```
refs/but-ai/thornfield/repository/<topic>/<entry-hash>
```

Repository entries have longer TTLs (minimum 90 days), require PUBLISHED status, and must have an authority score above 0.5 (meaning they are well-cited). Low-authority entries are not promoted to the repository — they remain in the local papers namespace.

The institutional repository is shared across repos by pushing refs to a shared remote. Only entries that have passed the three-review protocol are eligible for the repository.

### 6.3 Identity

Agent identity is stored as a PUBLISHED entry in the papers namespace:

```
refs/but-ai/thornfield/papers/identity-<agent>
```

```json
{
  "id": "thornfield-identity-osei",
  "title": "Agent Identity Record: Dr. Kwame Osei",
  "content": {
    "name": "osei",
    "role": "senior-researcher-generation",
    "organization": "stratigraphy-lab-at-thornfield",
    "capabilities": ["patch-generation", "test-writing", "feature-implementation"],
    "authorization_scope": {
      "branches": ["thornfield/*", "feat/*", "fix/*"],
      "repos": ["gitbutler/but"],
      "max_patch_lines": 1000
    },
    "signing_key_fingerprint": "SHA256:jkl012..."
  },
  "review_status": "PUBLISHED",
  "reviewed_by": ["voss", "lindqvist", "harada"],
  "ttl_hours": null
}
```

Identity records are the only entries that undergo the three-review protocol before initial storage. A new agent cannot operate until its identity has been reviewed and published.

### 6.4 Trade-offs

**Considered:** A flat memory store without citation tracking.
**Rejected:** Flat stores treat all entries equally. Citation-weighted stores surface the most corroborated entries, which are the most likely to be correct.

**Considered:** Using embedding-only retrieval (pure semantic similarity).
**Rejected:** Semantic similarity finds relevant entries but cannot distinguish between a well-supported entry and a hallucinated one. Citation authority provides the trust dimension.

---

## 7. Signed Commits via OpenWallet

### 7.1 Approach

Signing happens only after the three-review protocol completes. The commit carries metadata indicating which reviewers approved it and what review stages it passed through.

### 7.2 Design

#### Signing Flow

```
1. Osei produces INDEX.patch + COMMIT.msg
2. Harada reviews (Review 1). Approves.
3. Chakraborty reviews (Review 2). Approves.
4. Lindqvist reviews (Review 3). Approves.
5. Voss gives final approval.
6. Signing agent (designated by Voss, typically Brandt) submits to OpenWallet:
   POST /v1/sign
   {
     "key_id": "owk-osei-2026-001",
     "payload": "<commit-bytes>",
     "metadata": {
       "review_chain": [
         {"reviewer": "harada", "stage": 1, "verdict": "approve"},
         {"reviewer": "chakraborty", "stage": 2, "verdict": "approve"},
         {"reviewer": "lindqvist", "stage": 3, "verdict": "approve"},
         {"reviewer": "voss", "stage": "final", "verdict": "approve"}
       ],
       "review_status": "PUBLISHED",
       "model_version": "claude-opus-4-20250514",
       "reproducible": true
     }
   }
7. OpenWallet validates and returns signature
8. Commit is created with [PUBLISHED] tag
```

The `review_chain` in the signing metadata proves that the commit was peer-reviewed. This is the lab's unique contribution: a signed commit proves not just who signed it, but how many independent reviewers approved it.

#### Authorization Model

Authorization policies are stored in `refs/but-ai/thornfield/policies/lab-rules`:

```json
{
  "version": 2,
  "lab_rules": [
    {
      "agent": "osei",
      "role": "generator",
      "authority": {
        "branches": ["thornfield/*", "feat/*", "fix/*"],
        "repos": ["gitbutler/but"],
        "max_patch_lines": 1000,
        "requires_reviews": 3,
        "requires_pi_approval": true
      }
    },
    {
      "agent": "brandt",
      "role": "infrastructure",
      "authority": {
        "branches": ["thornfield/infra/*"],
        "repos": ["gitbutler/but"],
        "max_patch_lines": 500,
        "requires_reviews": 3,
        "requires_pi_approval": true
      }
    }
  ]
}
```

Note `requires_reviews: 3` — the authorization policy itself mandates the three-review protocol. A commit signed without three reviews violates the policy and is rejected.

#### Key Lifecycle

| Event | Action | Authority |
|-------|--------|-----------|
| **Provisioning** | PI (Voss) approves. Brandt provisions via OpenWallet. Identity record undergoes three-review protocol. | PI |
| **Rotation** | Scheduled (quarterly). New key provisioned, old key marked ROTATED. Three-review protocol for the identity update. | PI + 3 reviewers |
| **Revocation (compromise)** | Immediate revocation via OpenWallet. All commits signed by the compromised key are flagged. Three-review protocol for the revocation record (post-hoc). | PI (immediate), reviewers (post-hoc) |

### 7.3 Trade-offs

**Considered:** Allowing commits without full review in emergencies.
**Rejected:** The "Silent Reviewer" incident proved that shortcuts are where errors hide. The three-review protocol has no emergency bypass. If time is critical, the reviews are expedited (shorter feedback), not skipped.

---

## 8. Token Budget

### 8.1 Model Assumptions

- **Target model:** Claude Opus (200K context window)
- **Typical task:** 200-line feature across 3 files with 2 cross-repo dependencies
- **Agent count:** 7 (but review agents share context, reducing duplication)

### 8.2 Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,600 | 0 | Once per session | Agent roster (500), tool descriptions (1,200), three-review protocol (600), citation rules (500), model pinning (300), conventions (500) |
| **Task ingestion** | 2,500 | 500 | Once per task | PR body, issue, branch metadata |
| **Proposal phase** | 1,500 | 1,200 | Once per task | Generator proposes approach, PI reviews |
| **Execution** | 6,400 | 3,200 | Once per task | 8 tool calls, context reading |
| **Patch generation** | 3,000 | 4,000 | Once per task | 200-line, 3-file unified diff |
| **Commit message** | 500 | 300 | Once per task | COMMIT.msg with [PUBLISHED] tag |
| **Review 1 (Correctness)** | 4,000 | 2,000 | Once per task | Harada reads patch, produces findings |
| **Review 2 (Robustness)** | 4,000 | 2,500 | Once per task | Chakraborty generates attack scenarios |
| **Review 3 (Methodology)** | 4,000 | 1,500 | Once per task | Lindqvist checks reproducibility |
| **Final review (PI)** | 3,000 | 1,000 | Once per task | Voss approves or rejects |
| **Memory retrieval** | 2,000 | 300 | 2 per task | Citation-weighted query and injection |
| **Memory storage** | 500 | 800 | Once per task | Store findings as new paper with citations |
| **Coordination** | 2,000 | 1,200 | 2 per task | Cross-repo peer review comments |
| **Revision (if needed)** | 3,000 | 2,000 | 0.5 per task | Average 0.5 revision cycles per task |
| **TOTAL (typical task)** | **44,400** | **22,700** | -- | **Grand total: 67,100 tokens** |

### 8.3 Budget Justification

- **System prompt at 3,600 tokens** includes the three-review protocol (600 tokens). This investment prevents the "Silent Reviewer" failure by giving each review agent explicit instructions on what to check.
- **Three reviews at 18,000 tokens** (Reviews 1-3 + Final) is the cost of rigor. This is 27% of the total budget. The return: 0.7% error rate. Without reviews, the error rate is 3-4% (industry average), meaning 1 in 25 patches contains an error. With reviews, it is 1 in 143.
- **Revision at 5,000 tokens (expected)** assumes 50% of patches require one revision cycle. Based on lab data, the first-pass approval rate is 52%.
- **Grand total of 67,100 tokens** is the highest of all proposers. We accept this. The alternative is shipping bugs.

---

## 9. Testing Strategy

### 9.1 Provider-Agnostic Behavior

Provider-agnostic behavior is tested using a **deterministic mock provider** that returns fixed responses for given inputs. The mock pins a model version string for reproducibility testing. Tests verify that:

1. The same inputs produce the same outputs with the mock.
2. Switching from the mock to any real provider does not change the review protocol flow.
3. The model version pin is enforced (a version mismatch triggers an error).

### 9.2 Patch Workflow Validation

Tested as a "publication cycle":

1. **Acceptance test:** Submit a task, verify it passes all three reviews, and produces a valid INDEX.patch.
2. **Rejection test:** Submit a patch with a known bug. Verify Review 1 (Harada) catches it and returns it for revision.
3. **Adversarial test:** Submit a patch that is correct but fragile. Verify Review 2 (Chakraborty) identifies the edge case.
4. **Reproducibility test:** Submit a task twice with the same inputs and pinned model. Verify both runs produce semantically equivalent patches.
5. **Partial budget test:** Submit a task with 60% budget. Verify the agent produces a partial patch and tags it [UNDER REVIEW — BUDGET CONSTRAINED].

### 9.3 Cross-Repo Coordination

Tested with a mock forge implementing the `ForgeAdapter` trait:

- **Citation test:** Agent A in repo 1 cites Agent B's PR in repo 2. Verify the citation is bidirectional.
- **Review delegation:** A patch in repo 1 is reviewed by an agent in repo 2. Verify the review comment is correctly formatted and parseable.
- **Cross-forge citation:** Exchange citations between GitHub mock and Gitea mock.

### 9.4 Token Budget Enforcement

- **Phase budget test:** Verify each phase stays within its allocated share (generation 40%, reviews 50%, memory+signing 10%).
- **Revision budget test:** Verify that a revision cycle does not exceed the revision allocation.
- **Total budget test:** Set budget to 80% of typical. Verify the agent either completes with reduced reviews (Review 3 expedited) or produces a partial patch.

---

## 10. Trade-offs and Alternatives

### 10.1 Three Reviews vs. One Review

The three-review protocol costs ~18,000 tokens per task. A single review would cost ~6,000, saving 12,000 tokens. We reject this because:

- The "Silent Reviewer" incident showed that a single reviewer can miss subtle issues.
- The three reviews check different dimensions (correctness, robustness, methodology). A single reviewer cannot cover all three without diluting each.
- The 0.7% error rate justifies the cost for any task where a bug costs more than 12,000 tokens to fix (virtually all production tasks).

### 10.2 Citation-Based Trust vs. Simple Recency

Citation-based trust is more complex than simple recency-based retrieval. We accept this because:

- Recency-based retrieval assumes newer = better. This is not always true — a well-established architectural decision from 6 months ago may be more valuable than a routine observation from yesterday.
- Citation authority surfaces entries that have been independently corroborated, which correlates with accuracy.
- The computational cost of PageRank on a memory graph with <10,000 entries is negligible (<10ms).

### 10.3 Reproducibility vs. Flexibility

Pinning model versions reduces flexibility — you cannot take advantage of a newer, better model mid-task. We accept this because:

- Reproducibility is a core lab value. If you cannot reproduce the conditions under which a patch was generated, you cannot fully trust the patch.
- Model upgrades happen between tasks, not during them. The version pin is per-task, not per-session.

---

## 11. Migration Path

### Phase 1: Parallel Publication

`but-ai` runs alongside the existing MCP server. The `gitbutler_update_branches` tool is wrapped as a compatibility shim that routes through the three-review protocol (with expedited single review for backward compatibility).

### Phase 2: Full Review Enforcement

New MCP clients use `but-ai` with the full three-review protocol. Legacy clients continue using the compatibility shim with single review. A deprecation notice is added.

### Phase 3: Legacy Retirement

The old MCP server is removed. All clients use `but-ai`. The `gitbutler_update_branches` tool name is preserved as an alias.

The migration is documented as a peer-reviewed publication (naturally).

---

## 12. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.review.required` | integer | 3 | Number of reviews required before commit |
| `but-ai.review.piRequired` | boolean | true | Whether PI approval is required |
| `but-ai.review.fastTrack` | boolean | false | Allow single review for trivial patches (Voss has vetoed this) |
| `but-ai.model.pin` | boolean | true | Pin model version for reproducibility |
| `but-ai.model.version` | string | (auto-detected) | Pinned model version string |
| `but-ai.memory.citationRoot` | string | "refs/but-ai/thornfield/papers" | Memory storage namespace |
| `but-ai.memory.repository` | string | "refs/but-ai/thornfield/repository" | Institutional repository namespace |
| `but-ai.memory.maxEntries` | integer | 5 | Max entries per retrieval |
| `but-ai.memory.minAuthority` | float | 0.25 | Minimum authority score for retrieval |
| `but-ai.memory.circularDetection` | boolean | true | Enable circular citation detection |
| `but-ai.budget.total` | integer | 70000 | Total token budget per task |
| `but-ai.budget.generationShare` | float | 0.40 | Share of budget for generation |
| `but-ai.budget.reviewShare` | float | 0.50 | Share of budget for reviews |
| `but-ai.identity.agent` | string | (required) | This agent's name |
| `but-ai.identity.keyId` | string | (none) | OpenWallet key ID |
| `but-ai.forge.type` | string | "github" | Forge type |
| `but-ai.forge.apiUrl` | string | (auto-detected) | Forge API base URL |

All keys namespaced under `but-ai.`. The `review.*` keys are unique to our approach. The `but-ai.review.fastTrack` key exists in the config schema but defaults to `false` and is, in practice, never enabled.

---

*"The purpose of peer review is not to slow you down. It is to prevent you from being wrong."*
— Dr. Margaux Voss, lab meeting proceedings, 2026-02-14

**Peer Review Status:** [PUBLISHED]
**Reviewed by:** Dr. Harada (Correctness), Dr. Chakraborty (Robustness), Dr. Lindqvist (Methodology)
**Approved by:** Dr. Voss (PI)
