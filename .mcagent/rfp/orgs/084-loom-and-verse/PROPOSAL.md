# Proposal: `but-ai` Plugin -- Loom & Verse

**Submitted by:** Loom & Verse (Org 084)
**Domain:** Fashion Design -- Literary Textile Art
**Date:** 2026-03-28

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Plugin Architecture (RFP 3.1)](#2-plugin-architecture-rfp-31)
3. [Provider-Agnostic AI Interface (RFP 3.2)](#3-provider-agnostic-ai-interface-rfp-32)
4. [The But Agent (RFP 3.3)](#4-the-but-agent-rfp-33)
5. [Polyrepo PR-Based Agent Coordination (RFP 3.4)](#5-polyrepo-pr-based-agent-coordination-rfp-34)
6. [Agent Memory and Identity (RFP 3.5)](#6-agent-memory-and-identity-rfp-35)
7. [Signed Commits via OpenWallet (RFP 3.6)](#7-signed-commits-via-openwallet-rfp-36)
8. [Token Budget (RFP 3.7)](#8-token-budget-rfp-37)
9. [Testing Strategy](#9-testing-strategy)
10. [Trade-offs and Alternatives](#10-trade-offs-and-alternatives)
11. [Configuration Reference](#11-configuration-reference)

---

## 1. Executive Summary

Loom & Verse proposes a `but-ai` plugin built on the principle that **memory is narrative**. A database retrieves facts. A story retrieves meaning. When an agent needs to remember something, the relevant question is not "what is the value at key X?" but "what happened in the chapter where X was introduced, and how does that chapter connect to the current chapter?"

Our central innovation is **narrative memory** -- a memory scheme where memories are stored as chapters in an ongoing story. Each task adds a chapter. Recurring themes emerge as motifs that serve as retrieval anchors. Contradictions between memories create dramatic tension that the system flags for resolution rather than silently overwriting. Compaction produces chapter summaries that preserve thematic content even when specific details are lost.

We implement `but-ai` in Rust, using the existing `but-llm` and `but-tools` crates, modifying no existing code.

---

## 2. Plugin Architecture (RFP 3.1)

### Approach

The `but-ai` binary is a PATH-discovered executable. Every invocation is an act of reading or writing in the ongoing story of the codebase.

### Design

**Binary structure:**

```
but-ai
  ├── but ai write <task>        -- Execute a task (write a new chapter)
  ├── but ai read                -- Read the current story state
  ├── but ai mcp                 -- MCP server mode
  ├── but ai agent --task <desc> -- Autonomous agent mode
  └── but ai colophon            -- Show the project's narrative metadata
```

The primary verb is `write`. Every task is a new chapter in the codebase's story. `read` shows the narrative state: which chapters have been written, which motifs are active, which tensions are unresolved.

**Crate structure:**

```
crates/but-ai/
  src/
    lib.rs              -- Core library
    narrative/
      chapter.rs        -- Chapter (task) management
      motif.rs          -- Recurring theme detection and tracking
      tension.rs        -- Contradiction detection and resolution
      summary.rs        -- Chapter summarization for compaction
    workshop/
      author.rs         -- Patch generation (Orozco)
      editor.rs         -- Memory narration (Brenner)
      continuity.rs     -- Consistency validation (Sato)
      publisher.rs      -- Cross-repo coordination (Hartmann)
    colophon/
      identity.rs       -- Agent identity and signing
      metadata.rs       -- Narrative metadata for commits
  bin/
    main.rs             -- Binary entry point
```

**Environment variables:**

`BUT_WORKSPACE_DIR` maps to the story's setting (the codebase). `BUT_OUTPUT_FORMAT` determines the narration style: `human` is prose-like, `json` is structured, `shell` is telegraphic.

**WASI degradation:**

Under WASI, the plugin operates as an "unpublished manuscript" -- it can write new chapters (generate patches) and maintain its own narrative (local memory), but it cannot publish (no cross-repo coordination) or correspond with other authors (no forge API). The story continues, but in isolation. When the WASI constraint is lifted, the accumulated chapters can be published and the narrative reconnected.

**MCP compatibility:**

Drop-in replacement for the existing MCP server. The server name is `"GitButler Narrative Engine"`, maintaining the `ServerHandler` trait implementation and backward compatibility with `gitbutler_update_branches`.

### Trade-offs

**Alternative considered: Web-based narrative editor.** Rejected. The RFP requires a CLI/MCP plugin. Our literary instincts say that a story should be written in the same medium it will be read -- and code is read in terminals, not browsers.

**Alternative considered: Integrating narrative engine into core `but`.** Rejected per RFP constraints (no modification of existing crates). The plugin architecture is appropriate: a good story does not modify its binding.

---

## 3. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

We use `but-llm` as the sole LLM backend. Our agents are authors, not engineers -- they care about what the LLM says, not how it says it. The provider is the paper and ink; the story is ours.

### Design

**Provider abstraction:**

We define a `NarrativeVoice` adapter for extending providers:

```rust
pub trait NarrativeVoice: Send + Sync {
    fn voice_name(&self) -> &str;
    fn supports_dialogue(&self) -> bool;    // tool calling
    fn supports_monologue(&self) -> bool;   // streaming text
    fn context_depth(&self) -> usize;       // max tokens
    fn configure(&self, config: &gix::config::File) -> Result<LLMProviderConfig>;
}
```

New providers implement `NarrativeVoice` and are registered via Git config:

```ini
[but-ai "voice.gemini"]
    adapter = "/path/to/gemini-voice"
    style = "expansive"    # hint about the provider's response characteristics
```

**Tool exposure:**

All 10 workspace tools are registered through `Toolset`. The commune annotates each tool with a narrative description:

- `Commit` -> "Conclude the current chapter and publish it"
- `CreateBranch` -> "Begin a new subplot"
- `GetProjectStatus` -> "Read the story so far"
- `MoveFileChanges` -> "Transfer a scene from one chapter to another"
- `SplitBranch` -> "Fork the narrative into parallel storylines"
- `SplitCommit` -> "Break a chapter into shorter, more focused scenes"

These descriptions help the LLM understand the tools' narrative roles, improving tool selection accuracy.

**Provider fallback:**

Providers without tool calling receive a "dramatic monologue" prompt that instructs the LLM to describe its intended tool calls as a structured narrative. The response is parsed into tool invocations. This is less efficient but preserves the full tool surface for all providers.

### Trade-offs

**Alternative considered: Custom narrative-optimized LLM client.** Rejected (disqualifying per RFP). `but-llm` is sufficient. The narrative layer is in the prompt and memory, not the client.

---

## 4. The But Agent (RFP 3.3)

### Approach

The agent operates as a literary workshop with four members (see AGENTS.md): Orozco writes, Brenner edits, Sato checks continuity, and Hartmann publishes. Every task is a chapter in the ongoing story.

### Design

**Chapter lifecycle:**

```
1. PREMISE:      Brenner reads the task, identifies thematic connections to past chapters
2. OUTLINE:      Orozco plans the chapter structure (which files, which changes)
3. DRAFT:        Orozco writes the first draft (generates code changes)
4. REVISION:     Orozco refines based on self-review
5. CONTINUITY:   Sato checks for contradictions with existing chapters
6. EDITING:      Brenner annotates the chapter with narrative metadata
7. PUBLICATION:  INDEX.patch + COMMIT.msg produced; chapter complete
```

**Patch production:**

Orozco produces unified diffs against the current index. The diff is written to `INDEX.patch`. The commit message is written to `COMMIT.msg` and includes a colophon:

```
feat(auth): add session token validation middleware

Chapter: 42 of the authentication arc
Motifs: security-boundary, trust-verification, session-lifecycle
Tension introduced: session timeout conflicts with long-running operations
Continuity: verified by sato (no contradictions with chapters 38-41)
```

The colophon is not decorative. It provides structured metadata that Brenner uses for future memory retrieval. The `Motifs` field is a list of active narrative motifs that serve as retrieval anchors. The `Tension introduced` field flags unresolved issues that future chapters must address.

**Branch naming:**

```
chapter/<arc>/<chapter-number>[.<dependency>]
```

Example: `chapter/auth/042.039` -- Chapter 42 of the authentication arc, depends on chapter 39.

Arcs are thematic groupings of related tasks, analogous to story arcs in a novel. The authentication arc contains all auth-related changes. This grouping helps Brenner retrieve relevant memory when a new auth task arrives.

**Token budget enforcement:**

At 75% budget, Brenner switches from full narrative annotation to "flash fiction" mode -- minimal annotations, motif tags only. At 85%, Orozco switches from multi-draft to single-draft mode. At 95%, the workshop produces whatever it has as a "cliffhanger" -- a partial INDEX.patch that ends at a coherent boundary with an explicit "TO BE CONTINUED" annotation in the commit message, listing what remains.

**Progress reporting:**

```json
{
  "phase": "DRAFT",
  "agent": "orozco",
  "role": "author",
  "chapter": 42,
  "arc": "authentication",
  "draft_number": 1,
  "tokens_used": 15000,
  "tokens_budget": 50000,
  "motifs_active": ["security-boundary", "session-lifecycle"],
  "tensions_unresolved": 1,
  "narrative_coherence": 0.88
}
```

### Trade-offs

**Alternative considered: Linear agent pipeline (each step depends on the previous).** Rejected because our workshop operates more fluidly -- Orozco may start drafting while Brenner is still identifying thematic connections, and Sato may flag a continuity issue that sends Orozco back to drafting. The literary workshop model is iterative, not linear.

**Alternative considered: Larger agent team (dedicated agents for each file).** Rejected. A story is not improved by assigning one author per chapter. Coherence requires that the same voice (Orozco) writes all patches in a task, with continuity maintained by a single editor (Brenner).

---

## 5. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

Cross-repo coordination is modeled as **correspondence between authors**. In literary history, many of the greatest works emerged from networks of writers exchanging letters, manuscripts, and critiques. The commune coordinates through PR comments as structured correspondence.

### Design

**Forge adapter (Correspondence Interface):**

```rust
pub trait CorrespondenceChannel: Send + Sync {
    fn send_letter(&self, to: &RepoRef, letter: &Letter) -> Result<LetterId>;
    fn read_letters(&self, since: DateTime<Utc>) -> Result<Vec<Letter>>;
    fn check_publication(&self, pr: &PrId) -> Result<PublicationStatus>;
    fn annotate_publication(&self, pr: &PrId, note: &EditorialNote) -> Result<()>;
    fn list_correspondents(&self, pr: &PrId) -> Result<Vec<RepoRef>>;
}
```

We provide a GitHub implementation. The trait maps cleanly to forge primitives: `send_letter` = create comment, `read_letters` = list comments, `check_publication` = get PR status.

**Letter schema (PR comment format):**

```json
{
  "$schema": "but-ai/correspondence/v1",
  "type": "commission | progress | dependency | manuscript | budget",
  "from": {
    "author": "hartmann",
    "commune": "084-loom-and-verse",
    "repo": "owner/repo"
  },
  "to": {
    "author": "target-agent",
    "commune": "target-org",
    "repo": "owner/other-repo"
  },
  "content": {
    "subject": "Authentication middleware for session management",
    "arc": "authentication",
    "chapter_ref": "chapter/auth/042",
    "status": "in_draft",
    "dependencies": ["owner/other-repo#17"],
    "budget": { "used": 20000, "total": 50000 },
    "motifs": ["security-boundary"]
  },
  "timestamp": "2026-03-28T14:30:00Z"
}
```

Embedded as:

````markdown
```but-ai-letter
{ ... }
```
````

**Cross-repo narrative continuity:**

Hartmann maintains a "correspondence log" that tracks the narrative threads across repositories. When a task in repo A depends on work in repo B, the correspondence log records the cross-repo arc, ensuring that narrative continuity is maintained across repository boundaries. This log is stored as a persistent chapter in the narrative memory.

**Forge-agnosticism:**

The `CorrespondenceChannel` trait uses only forge-universal operations. Letters are self-contained JSON in code fences -- no forge-specific features required. The schema is designed to be readable by humans (the letter format is intuitive) and parseable by machines.

### Trade-offs

**Alternative considered: Custom messaging protocol.** Rejected per RFP (no services beyond forge). PR comments are the messaging medium.

**Alternative considered: File-based coordination via shared branches.** Rejected because files in shared branches lack the notification infrastructure that PR comments provide. You need to know when a letter arrives, not poll for it.

---

## 6. Agent Memory and Identity (RFP 3.5)

### Approach: Narrative Memory

Our memory system stores agent memory as **chapters in an ongoing story**. This is not a metaphor applied to a database. It is a fundamentally different data model that captures something databases cannot: narrative structure.

A chapter is not a key-value pair. It is a self-contained narrative unit with:

- **Setting**: The codebase state when the chapter was written
- **Characters**: The agents, files, and modules involved
- **Plot**: What happened (the task, the changes, the outcomes)
- **Motifs**: Recurring themes that connect this chapter to others
- **Tensions**: Contradictions or unresolved issues introduced by this chapter

Retrieval is not key lookup. It is thematic resonance. When the agent encounters a new task about authentication, it does not search for memories tagged "authentication." It searches for chapters whose motifs resonate with the themes of the current task. This captures connections that keyword matching and even embedding similarity miss.

### Design

**Storage:**

Memory is stored as chapters on a special Git branch:

```
refs/but-ai/story/<agent-id>/
  chapters/
    001.json         -- First chapter (task)
    002.json         -- Second chapter
    ...
    042.json         -- Current chapter
  arcs/
    authentication.json  -- Arc metadata (motifs, characters, tension history)
    deployment.json      -- Another arc
  motifs/
    security-boundary.json    -- Motif: appears in chapters 5, 12, 27, 38, 42
    session-lifecycle.json    -- Motif: appears in chapters 38, 39, 42
  tensions/
    active/
      timeout-vs-longrun.json -- Unresolved: session timeout vs. long operations
    resolved/
      auth-format.json        -- Resolved: standardized auth token format in ch. 30
  summaries/
    arc-authentication-v3.json -- Latest summary of the authentication arc
```

**Chapter structure:**

```json
{
  "chapter_number": 42,
  "title": "Session Token Validation",
  "arc": "authentication",
  "setting": {
    "branch": "feat/auth-middleware",
    "workspace_state_hash": "abc123",
    "timestamp": "2026-03-28T14:00:00Z"
  },
  "characters": ["auth.rs", "middleware.rs", "session.rs"],
  "plot": {
    "task": "Add validation middleware for session tokens",
    "approach": "Inserted validation layer between request parsing and handler dispatch",
    "outcome": "3 files modified, 47 lines added, all tests passing"
  },
  "motifs": ["security-boundary", "trust-verification", "session-lifecycle"],
  "tensions_introduced": [{
    "id": "timeout-vs-longrun",
    "description": "Session timeout (30min) conflicts with long-running export operations",
    "severity": "moderate",
    "suggested_resolution": "Implement activity-based timeout extension for export operations"
  }],
  "tensions_resolved": [],
  "colophon": {
    "author": "orozco",
    "editor": "brenner",
    "continuity_checker": "sato",
    "patch_ref": "chapter/auth/042",
    "commit_sha": "def456"
  },
  "embedding_vector": [0.12, -0.34, ...]
}
```

**Motif tracking:**

A motif is a recurring theme identified by Brenner. When a theme appears in three or more chapters, Brenner creates a motif entry:

```json
{
  "motif_id": "security-boundary",
  "description": "The boundary between trusted and untrusted contexts",
  "first_appearance": 5,
  "appearances": [5, 12, 27, 38, 42],
  "variations": [
    { "chapter": 5, "form": "API authentication at the gateway" },
    { "chapter": 12, "form": "Database access control" },
    { "chapter": 42, "form": "Session token validation" }
  ],
  "related_motifs": ["trust-verification", "session-lifecycle"],
  "embedding_vector": [0.15, -0.28, ...]
}
```

Motifs are the primary retrieval mechanism. When a new task arrives, Brenner identifies its thematic content and searches for motifs that resonate. Each matching motif points to the chapters where it appeared, providing rich, contextual memory retrieval.

**Relevance scoring:**

```
score = 0.30 * motif_resonance(query, chapter.motifs)
      + 0.25 * embedding_similarity(query, chapter)
      + 0.20 * arc_relevance(query.arc, chapter.arc)
      + 0.15 * recency(chapter)
      + 0.10 * tension_urgency(chapter.active_tensions)
```

`motif_resonance` is the key differentiator. It measures thematic overlap between the query and the chapter's motifs, including related motifs (transitive resonance). A query about "permission checking" will resonate with the motif "security-boundary" even if the words do not overlap, because both belong to the same thematic cluster.

`tension_urgency` boosts chapters with unresolved tensions. The narrative system preferentially surfaces unresolved problems, nudging the agent to address them. This is how the narrative drives forward: unresolved tensions create dramatic pressure that biases the agent toward resolution.

**Expiration:**

Chapters do not expire individually. Arcs expire. When an arc has had no new chapters for a configurable period (default: 30 days), the arc enters "dormancy." Dormant arcs are summarized (Brenner produces an arc summary that preserves motifs and active tensions) and the individual chapters are archived. The summary replaces the chapters in active memory, reducing storage while preserving thematic content.

Motifs persist across arc dormancy. A motif that appeared in a dormant arc remains active and retrievable. This is how the narrative memory maintains long-term coherence: even when the specific details of old chapters are forgotten, the motifs they established continue to influence retrieval.

Tensions have their own lifecycle. Active tensions persist until explicitly resolved by a new chapter. If a tension persists for more than 14 days without resolution, it is escalated -- flagged in every relevant task as an unresolved issue demanding attention.

**Compaction survival:**

When the context window is compacted, the narrative engine produces **chapter summaries** -- concise representations that preserve:

1. Active motifs (always preserved)
2. Unresolved tensions (always preserved)
3. Arc summaries (preserved if the arc is active)
4. Specific chapter details (lost, but reconstructible from motifs and arc summaries)

The compaction process is analogous to summarizing a novel: you lose the prose but keep the plot, the characters, and the themes. A reader who has read only the summary can still engage meaningfully with a sequel.

**Long-term storage:**

The `summaries/` directory stores arc summaries that serve as the long-term memory. Cross-repository long-term memory is maintained through a shared branch:

```
refs/but-ai/shared-story/
  motifs/          -- Cross-repo motifs (themes that appear in multiple repos)
  arcs/            -- Cross-repo arc summaries
  tensions/        -- Cross-repo unresolved tensions
```

**Identity:**

Agent identity is encoded as a **colophon** -- a signed document modeled on the colophon pages found in hand-printed books:

```json
{
  "agent_id": "orozco",
  "commune": "084-loom-and-verse",
  "role": "author",
  "capabilities": ["patch_generation", "refactoring", "bug_fixes"],
  "style": "concrete, iterative, material",
  "authorization": {
    "branches": ["chapter/*", "feat/*"],
    "max_chapter_size": 500,
    "repos": ["owner/main-repo"]
  },
  "signing_key": "openwallet:084-loom-verse:orozco",
  "first_chapter": 1,
  "chapters_authored": 42,
  "created_at": "2026-03-01T00:00:00Z"
}
```

### Trade-offs

**Alternative considered: Database-style memory (key-value with indices).** Rejected because databases are amnesiac -- they store facts without context. A narrative stores facts embedded in the story of how they were learned, why they matter, and how they relate to other facts. This context is essential for meaningful retrieval.

**Alternative considered: Graph memory (nodes and edges).** Rejected because graphs are structural but not temporal. A graph can represent relationships but not sequences, not development, not the passage of time. Narrative memory is inherently temporal: chapters are ordered, arcs develop, tensions build and resolve.

**Alternative considered: Embedding-only retrieval.** Rejected because embeddings capture semantic similarity but not thematic resonance. Two chapters can be semantically different (one about authentication, one about database access) but thematically identical (both about security boundaries). Motif-based retrieval captures this.

---

## 7. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every agent signs their work. The signature is the author's mark -- proof that this specific author, with this specific authorization, wrote this specific chapter.

### Design

**Key hierarchy:**

```
Commune Key (084-loom-and-verse)
  ├── Author Key (orozco)       -- Patch authorship
  ├── Editor Key (brenner)       -- Memory management
  ├── Continuity Key (sato)      -- Validation annotations
  └── Publisher Key (hartmann)   -- Cross-repo correspondence
```

**Authorization model:**

Encoded in the colophon. Role-specific constraints:

- **Authors** (Orozco): May produce INDEX.patch + COMMIT.msg for branches within their scope. Each patch must be within the `max_chapter_size` limit.
- **Editors** (Brenner): May modify the narrative memory branch. No codebase write access.
- **Continuity Checkers** (Sato): Read-only. May annotate PRs with continuity reports.
- **Publishers** (Hartmann): May create PR comments and cross-repo correspondence. No direct codebase writes.

**Key lifecycle:**

| Event | Narrative Analogy | Action |
|-------|-------------------|--------|
| Provisioning | Author signs their first book contract | Commune key signs agent key |
| Rotation | Author adopts a new pen name | New key issued, old key noted as "retired" |
| Compromise | Forged manuscripts discovered | Key revoked, all signed work quarantined |
| Decommission | Author retires | Key archived, authorization scope emptied |

The distinction between rotation and compromise is recorded in the key lifecycle log: rotated keys have a `succeeded_by` field, compromised keys have a `forgery_detected_at` timestamp.

### Trade-offs

**Alternative considered: Shared commune key.** Rejected. Each author's voice is distinct, and so is each author's authorization. A shared key would mean Brenner could sign patches, which violates the commune's separation of literary roles.

---

## 8. Token Budget (RFP 3.7)

Estimates for Claude Opus on a typical task: 200-line feature, 3 files, 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,000 | 0 | Once per session | Agent identity, tool descriptions, active motifs, arc summaries |
| **Task ingestion (premise)** | 1,800 | 600 | Once per task | Reading task, identifying thematic connections |
| **Outline** | 1,500 | 800 | Once per task | Orozco's chapter plan |
| **Draft (per draft)** | 1,500 | 2,000 | 2 per task | Orozco's iterative patch writing |
| **Continuity check (Sato)** | 1,800 | 600 | Once per task | Cross-chapter contradiction scan |
| **Narrative annotation (Brenner)** | 1,200 | 1,000 | Once per task | Motif tagging, tension identification, chapter metadata |
| **Commit message + colophon** | 300 | 500 | Once per task | Rich commit message with narrative metadata |
| **Memory retrieval** | 1,000 | 400 | 2 per task | Motif-based search, arc summary lookup |
| **Memory storage** | 400 | 800 | 1 per task | Chapter creation, motif updates |
| **Coordination (Hartmann)** | 1,200 | 700 | 2 per task | Correspondence, dependency tracking |
| **TOTAL (typical task)** | **19,700** | **12,200** | -- | **31,900 total tokens** |

### Justification

The total of ~32,000 tokens is the lowest among our peer organizations. This efficiency comes from two sources:

1. **Motif-based retrieval** is cheaper than exhaustive search. Instead of comparing the query against every memory entry, the agent queries the motif index (which is small and fixed-size) and then retrieves only the chapters that match relevant motifs. This reduces retrieval cost from O(n) to O(m + k) where m is the number of motifs and k is the average number of chapters per motif.

2. **Arc-based organization** reduces system prompt overhead. Instead of summarizing all persistent context, the system prompt includes only the active arc summaries (typically 2-3 arcs) and the active motif list. Detailed chapter content is retrieved on demand.

The system prompt is kept under 3,200 tokens. Active motifs add approximately 300 tokens (a list of theme names with brief descriptions). Arc summaries add approximately 500 tokens (2-3 arcs at ~200 tokens each).

---

## 9. Testing Strategy

### Provider-agnostic testing

A `MockVoice` implements the LLM provider interface with deterministic responses. Tests define a narrative setup (existing chapters, active motifs, active tensions) and a task, then verify that the agent produces the expected patch with the expected narrative annotations.

### Patch workflow validation

INDEX.patch round-trip:

1. Establish a codebase state (the "setting")
2. Execute a task (write a "chapter")
3. Capture INDEX.patch and COMMIT.msg
4. Apply INDEX.patch to a fresh copy of the setting
5. Verify the result, including narrative metadata in the commit message

Additional tests: cliffhanger output (budget-truncated partial patch), multi-chapter tasks (tasks that span multiple patches), and chapter amendments (corrections to previous chapters).

### Cross-repo coordination

A `MockCorrespondence` implements `CorrespondenceChannel` with in-memory letter storage. Tests simulate:

- Two communes exchanging correspondence about a shared task
- Dependency resolution across repos
- Narrative continuity across repository boundaries (motifs that span repos)
- Error conditions: lost letters, delayed responses, malformed correspondence

### Token budget enforcement

Tests verify:

- Normal operation (2-draft completion within budget)
- Flash fiction mode (reduced narrative annotation at 75%)
- Single-draft mode (simplified generation at 85%)
- Cliffhanger production (valid partial output at 95%)

### Narrative memory

Memory tests verify:

- Chapter storage and retrieval
- Motif emergence (3+ appearances trigger motif creation)
- Motif-based retrieval (thematic resonance outperforms keyword matching)
- Tension lifecycle (creation, persistence, resolution, escalation at 14 days)
- Arc dormancy and summarization
- Compaction survival (motifs and tensions survive, chapter details are summarized)
- Cross-repo motif propagation

---

## 10. Trade-offs and Alternatives

| Decision | Chosen | Alternative | Why |
|----------|--------|-------------|-----|
| Memory model | Narrative (chapters + motifs) | Key-value store | Narrative captures temporal, thematic, and contextual relationships that KV stores lose |
| Retrieval | Motif-based + embedding | Embedding-only | Motifs capture thematic resonance beyond semantic similarity |
| Expiration | Arc-level dormancy | Per-entry TTL | Stories expire in arcs, not in isolated facts |
| Agent structure | 4-agent workshop | Single agent | Workshop roles (author, editor, continuity, publisher) map naturally to quality stages |
| Tension tracking | Explicit, with escalation | Silent overwriting | Contradictions are information, not errors; they drive resolution |
| Branch naming | Arc-based | Sequential | Arc names provide semantic grouping for related work |
| WASI fallback | Unpublished manuscript mode | No AI | Writing can happen anywhere; publication requires infrastructure |

---

## 11. Configuration Reference

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.narrative.branch` | string | `refs/but-ai/story` | Base ref for narrative storage |
| `but-ai.narrative.arcDormancy` | integer | 2592000 | Seconds of inactivity before arc goes dormant (30 days) |
| `but-ai.narrative.motifThreshold` | integer | 3 | Minimum appearances for motif emergence |
| `but-ai.narrative.tensionEscalation` | integer | 1209600 | Seconds before unresolved tension is escalated (14 days) |
| `but-ai.narrative.maxChapters` | integer | 500 | Maximum active chapters before forced compaction |
| `but-ai.narrative.summaryMaxTokens` | integer | 500 | Maximum tokens per arc summary |
| `but-ai.agent.tokenBudget` | integer | 45000 | Total token budget per task |
| `but-ai.agent.flashFictionThreshold` | float | 0.75 | Budget fraction triggering minimal annotations |
| `but-ai.agent.singleDraftThreshold` | float | 0.85 | Budget fraction triggering single-draft mode |
| `but-ai.agent.cliffhangerThreshold` | float | 0.95 | Budget fraction triggering partial output |
| `but-ai.correspondence.schema` | string | `but-ai/correspondence/v1` | Letter schema version |
| `but-ai.correspondence.pollInterval` | integer | 30 | Seconds between letter checks |
| `but-ai.identity.communeKey` | string | -- | OpenWallet commune key ID |
| `but-ai.identity.agentKeyPrefix` | string | -- | OpenWallet agent key prefix |
| `but-ai.voice.<name>.adapter` | string | -- | Path to external provider voice adapter |

---

*"A codebase is a story told by many authors across many years. Our agents read that story, understand its themes, and write the next chapter. The `but-ai` plugin is not a tool. It is a pen."*
-- Yael Brenner, Memory Narrator
