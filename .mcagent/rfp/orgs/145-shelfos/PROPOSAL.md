# Proposal — ShelfOS

**RFP Response: `but ai` Plugin for GitButler CLI**
**Organization:** ShelfOS (Org 145)
**Domain:** Library Science | **Philosophy:** Startup Hustle
**Date:** 2026-03-28

---

## Executive Summary

ShelfOS proposes a `but-ai` plugin built on the principles of library science: classify thoroughly, shelve accurately, circulate efficiently. The core insight is that the agent memory problem is a cataloging problem — the same problem librarians solved in the 19th century with the card catalog and have been refining ever since.

Our distinctive contribution is the card-catalog memory system: memories indexed by multiple classification systems simultaneously (subject, source, temporal, relational, hierarchical call number), with "see also" cross-references that create a navigable knowledge graph. The system does not merely store and retrieve. It classifies, relates, and circulates.

The 132 redundant `but status` calls from the SYNTHESIS report are the software equivalent of a patron walking to the shelf, finding the book missing, walking to the desk, asking the librarian, and repeating. The solution is not a faster patron. The solution is a better catalog: one that tells you where the book is before you walk to the shelf.

---

## 1. Plugin Architecture (RFP Section 3.1)

### Approach

`but-ai` is a Rust binary crate (`crates/but-ai/`) in the workspace, discoverable via PATH. The architecture mirrors a library system: a catalog (memory), a collection (workspace), and a circulation desk (coordination).

### Design

**CLI Mode:**
```
but ai catalog <query>    — Search the card-catalog memory system
but ai shelve <task>      — Execute a task (produces INDEX.patch + COMMIT.msg)
but ai circulate <pr>     — Manage PR coordination (check out, check in, hold)
but ai accession          — Add a new memory to the catalog
but ai deaccession        — Expire a memory from the catalog
but ai shelfread          — Verify workspace state against catalog expectations
but ai mcp                — Start MCP server on stdio
```

**MCP Server Mode:**
Drop-in replacement preserving `gitbutler_update_branches`. New tools:

| New Tool | Description |
|----------|-------------|
| `catalog_search` | Search memory by subject, call number, or "see also" traversal |
| `shelve_task` | Execute task with catalog-informed context |
| `circulate_pr` | PR operations: create, comment, manage holds |
| `accession_memory` | Add new memory entry with full classification |
| `deaccession_memory` | Expire memory entry, retain in archive |
| `shelf_read` | Verify workspace state, report discrepancies |
| `budget_report` | Token usage report |

**Git Config Keys:**

| Key | Purpose | Default |
|-----|---------|---------|
| `but-ai.catalogBranch` | Ref prefix for card-catalog memory | `refs/catalog/` |
| `but-ai.tokenBudget` | Per-task token budget | `40000` |
| `but-ai.agentIdentity` | OpenWallet key reference | none (required) |
| `but-ai.maxSubjectHeadings` | Maximum subject headings per memory | `3` |
| `but-ai.maxSeeAlso` | Maximum "see also" links per memory | `5` |
| `but-ai.classificationScheme` | Classification hierarchy file | auto-generated |
| `but-ai.circulationTracking` | Enable memory access tracking | `true` |
| `but-ai.deaccessionAge` | Default TTL for memories before deaccession review | `30d` |
| `but-ai.callNumberDepth` | Maximum depth of call number hierarchy | `5` |

**WASI Degradation:**
Under WASI, PATH discovery is unavailable. Degradation:
- Catalog operations work fully — Git-native, read/write to refs.
- Memory classification and retrieval work fully — no filesystem dependencies.
- Shelving (patch generation) works normally — output is INDEX.patch + COMMIT.msg.
- Circulation tracking is degraded — no background thread for continuous tracking. On-demand tracking remains available.
- WASI component interface available for direct loading.

### Trade-offs

**Considered:** Building the catalog as a SQLite database stored in Git. **Rejected:** SQLite files are binary blobs that don't diff, merge, or review well in Git. JSON files in refs are human-readable, diffable, and Git-native.

**Considered:** Using the existing Git notes system for memory storage. **Rejected:** Git notes are attached to specific objects (commits, trees). Agent memories are not always tied to specific commits — they can be about patterns, conventions, or cross-cutting concerns. Custom refs under `refs/catalog/` provide the needed flexibility.

---

## 2. Provider-Agnostic AI Interface (RFP Section 3.2)

### Approach

`but-llm` is the sole backend. ShelfOS wraps it with a catalog-aware layer that injects relevant catalog entries into the LLM context before each interaction.

### Design

**Catalog-Aware Provider Wrapper:**
```rust
pub struct LibraryProvider {
    inner: LLMProvider,
    catalog: CardCatalog,
    budget: CirculationBudget,
}

impl LibraryProvider {
    /// Tool-calling loop with automatic catalog lookup.
    /// Before each tool call, Cataloger injects relevant memories.
    /// After each tool call, results are classified for future retrieval.
    pub fn cataloged_tool_loop(
        &self,
        system_message: &str,
        messages: Vec<ChatMessage>,
        tools: &mut impl Toolset,
        model: &str,
    ) -> anyhow::Result<LibraryResult>;
}

pub struct LibraryResult {
    pub output: String,
    pub catalog_updates: Vec<CatalogEntry>,   // New memories to classify
    pub circulation_events: Vec<CircEvent>,    // Memory accesses to track
    pub see_also_discovered: Vec<SeeAlsoLink>, // New cross-references
}
```

The key innovation: the catalog is not just queried — it is continuously updated. Every tool call interaction potentially produces new knowledge that Cataloger classifies in real time.

**New Provider Mechanism:**
```rust
pub trait CollectionMember: Send + Sync {
    /// Each provider is an "item in the collection" with catalog metadata.
    fn name(&self) -> &str;
    fn classification(&self) -> ProviderClassification; // What this provider is good at
    fn create_backend(&self, config: &ProviderConfig) -> Result<Box<dyn LLMBackend>>;
}
```

Loaded from `but-ai.providerPluginDir`. The `classification` metadata supports future provider routing based on task classification.

### Trade-offs

**Considered:** Pre-computing catalog lookups for all possible queries. **Rejected:** the catalog changes with every task. Pre-computation goes stale. Just-in-time lookup is more accurate and not significantly slower (catalog queries against JSON refs are fast).

**Considered:** Embedding-based retrieval for the catalog. **Rejected:** same reasoning as other proposals — embeddings require non-Git-native infrastructure. ShelfOS uses keyword extraction, controlled vocabulary matching, and "see also" graph traversal for retrieval. Less precise than embeddings, but fully Git-native and debuggable.

---

## 3. The But Agent (RFP Section 3.3)

### Approach

The But Agent follows the library acquisition cycle: acquire (receive task), classify (understand context), shelve (write patch), circulate (coordinate and present).

### Design

**Task Lifecycle:**
```
1. ACQUIRE    — Receive task description, branch metadata, PR context
2. CLASSIFY   — Cataloger retrieves relevant memories, classifies task context
3. SHELVE     — Shelver produces INDEX.patch + COMMIT.msg
4. CATALOG    — Cataloger classifies the new work as future memory
5. CIRCULATE  — Circ creates PR, posts coordination, tracks memory access
```

**Classification-Driven Context:**
Before Shelver writes a single line of code, Cataloger provides a "reference shelf" — a curated set of memories classified as relevant to the task:

```json
{
  "reference_shelf": [
    {
      "call_number": "ARCH.AUTH.MIDDLEWARE.TOKEN-VALIDATION",
      "content": "JWT validation with RS256, 15-minute expiry",
      "subject_headings": ["authentication", "jwt", "middleware"],
      "see_also": ["ARCH.AUTH.SESSION-MANAGEMENT", "SEC.TOKEN-ROTATION"],
      "rehearsal_equivalent": 7,
      "circulation_count": 12
    }
  ],
  "finding_aid": "This task modifies the authentication middleware. See also: session management (related), token rotation (dependent)."
}
```

The finding aid is a natural-language summary that orients the agent, the way a library finding aid orients a researcher before they enter the archive.

**Branch Naming:**
```
shelf/s01.s03/shelver/auth-token-validation
│     │       │       └── Task description (subject heading)
│     │       └── Agent name
│     └── Dependency chain (call number sequence)
└── Namespace
```

**Token Budget Enforcement:**
```rust
pub struct CirculationBudget {
    total: u32,
    used: u32,
    catalog_reserve: u32,    // Reserved for post-task cataloging (1500 tokens)
    circulation_reserve: u32, // Reserved for PR and coordination (2000 tokens)
}
```

When the budget approaches the limit:
1. Shelver completes the current patch (or produces a partial patch).
2. Catalog reserve is released for post-task memory classification.
3. Circulation reserve is released for PR creation and coordination.
4. Both reserves are mandatory — cataloging and circulation are never skipped, because an unclassified result is a lost result.

**WorkspaceToolset Exposure:**
Tools loaded by lifecycle phase:

| Phase | Tools Loaded | Rationale |
|-------|-------------|-----------|
| ACQUIRE/CLASSIFY | GetProjectStatus, GetBranchChanges, GetCommitDetails | Read-only, context gathering |
| SHELVE | Commit, CreateBranch, Amend | Write operations |
| CIRCULATE | MoveFileChanges, SquashCommits, SplitBranch | Branch management, PR preparation |

Phase-gated loading saves ~1,000 tokens compared to loading all 10 tools.

### Trade-offs

**Considered:** Loading all tools at once for maximum flexibility. **Rejected:** wastes system prompt tokens. ShelfOS's lean 3-agent team requires tight token discipline. Phase-gated loading is a constraint that enforces good lifecycle practice.

**Considered:** Skipping the cataloging phase for simple tasks. **Rejected:** every acquisition is cataloged. This is the library's first rule. An unclassified item is a lost item. The cataloging cost (~1,500 tokens) is small relative to the information value it produces for future tasks.

---

## 4. Polyrepo PR-Based Agent Coordination (RFP Section 3.4)

### Approach

PRs are interlibrary loans. Cross-repo coordination uses a library-inspired protocol where items (patches, memories, references) are loaned between collections (repositories) via structured requests.

### Design

**PR Comment Schema (Catalog Protocol):**
```json
{
  "protocol": "catalog/v1",
  "type": "loan_request | return | hold | reference | budget_report",
  "agent": "circ@shelfos",
  "collection": "github.com/org/repo",
  "timestamp": "2026-03-28T14:30:00Z",
  "call_number": "ARCH.AUTH.MIDDLEWARE",
  "payload": { }
}
```

The `call_number` field classifies every coordination message. This allows agents in other repos to find relevant coordination messages by classification, not just by PR number.

**Forge Adapter Interface:**
```rust
pub trait CollectionAdapter: Send + Sync {
    fn create_loan(&self, collection: &CollRef, loan: &LoanRequest) -> Result<LoanId>;
    fn post_note(&self, collection: &CollRef, loan: LoanId, note: &str) -> Result<NoteId>;
    fn read_notes(&self, collection: &CollRef, loan: LoanId) -> Result<Vec<Note>>;
    fn add_subject(&self, collection: &CollRef, loan: LoanId, subject: &str) -> Result<()>;
    fn get_loan(&self, collection: &CollRef, loan: LoanId) -> Result<LoanInfo>;
    fn search_loans(&self, collection: &CollRef, query: &str) -> Result<Vec<LoanSummary>>;
}
```

(`create_loan` = create PR, `post_note` = post comment, `add_subject` = add label.)

Reference implementation: GitHub REST.

**Cross-Repo as Interlibrary Loan:**
```json
{
  "type": "loan_request",
  "payload": {
    "requesting_collection": "github.com/org/frontend",
    "lending_collection": "github.com/org/auth-service",
    "item": "PR #87 — Token format specification",
    "call_number": "ARCH.AUTH.TOKEN-FORMAT",
    "reason": "Frontend token parser must match auth-service token format",
    "due_date": "2026-03-29T00:00:00Z",
    "see_also": ["ARCH.AUTH.MIDDLEWARE", "API.V2.RESPONSE-FORMAT"]
  }
}
```

The interlibrary loan metaphor captures something that "dependency" does not: a loan has a due date, a reason, and a classification. The requesting collection explains not just what it needs but why it needs it and how it relates to other items in its own collection.

**Structured Message Types:**

| Type | Purpose | Key Fields |
|------|---------|------------|
| `loan_request` | Request item from another collection (dependency) | `lending_collection`, `item`, `call_number`, `reason` |
| `return` | Signal that a borrowed item has been processed | `loan_id`, `status`, `notes` |
| `hold` | Request priority on a pending item | `item`, `urgency`, `hold_queue_position` |
| `reference` | Provide context without requesting action | `call_number`, `content`, `see_also` |
| `budget_report` | Token budget status | `total`, `used`, `remaining` |

### Trade-offs

**Considered:** Using Git submodules for cross-repo coordination. **Rejected:** submodules add Git complexity, require push access, and don't support the structured communication that PRs provide.

**Considered:** Central coordination repository (a "union catalog"). **Rejected:** violates the "no central coordination service" requirement. Each collection (repo) maintains its own catalog. Cross-repo discovery happens via PR comments (interlibrary loan requests).

---

## 5. Agent Memory and Identity (RFP Section 3.5)

### Approach: Card-Catalog Memory

Memories are indexed by multiple classification systems simultaneously, with "see also" cross-references creating a navigable knowledge graph. The catalog is the intellectual contribution. The storage is straightforward — JSON files in Git refs. The classification scheme is where the value lives.

### Design

**Storage Medium:**
```
refs/catalog/<agent-id>/items/mem_001.json          — Individual memory item
refs/catalog/<agent-id>/subjects/auth.json           — Subject heading index
refs/catalog/<agent-id>/call_numbers/ARCH.json       — Call number hierarchy
refs/catalog/<agent-id>/see_also/graph.json          — Cross-reference graph
refs/catalog/<agent-id>/circulation/log.json         — Access tracking
refs/catalog/<agent-id>/vocabulary/controlled.json   — Controlled vocabulary
refs/catalog/<agent-id>/identity.json                — Agent identity
refs/catalog/shared/union_catalog.json               — Cross-session shared catalog
```

Each catalog entry:
```json
{
  "item_id": "mem_001",
  "content": "The auth middleware validates JWT tokens with RS256, 15-minute expiry",
  "classification": {
    "subject_headings": ["authentication", "jwt", "middleware"],
    "call_number": "ARCH.AUTH.MIDDLEWARE.TOKEN-VALIDATION",
    "source": {
      "task": "task_012",
      "agent": "shelver",
      "branch": "shelf/s01/shelver/auth-setup",
      "tool": "GetBranchChanges"
    },
    "temporal": {
      "created": "2026-03-15T10:00:00Z",
      "last_accessed": "2026-03-28T14:00:00Z",
      "last_validated": "2026-03-28T14:00:00Z"
    }
  },
  "see_also": [
    {"item": "mem_003", "relationship": "related_to", "note": "Session management uses the same token format"},
    {"item": "mem_007", "relationship": "depends_on", "note": "Token rotation policy affects expiry"},
    {"item": "mem_015", "relationship": "contrasts_with", "note": "API v1 used session cookies, not JWT"}
  ],
  "circulation": {
    "total_checkouts": 12,
    "last_checkout": "2026-03-28T14:00:00Z",
    "checkout_contexts": ["auth-refactor", "security-audit", "session-fix"]
  },
  "ttl": "30d",
  "confidence": 0.92
}
```

**The Five Classification Systems:**

| System | Purpose | Example |
|--------|---------|---------|
| **Subject headings** | What the memory is about (topical) | `["authentication", "jwt", "middleware"]` |
| **Call number** | Hierarchical position in knowledge structure | `ARCH.AUTH.MIDDLEWARE.TOKEN-VALIDATION` |
| **Source** | Where the memory came from (provenance) | `task_012, shelver, GetBranchChanges` |
| **Temporal** | When the memory was created, accessed, validated | `created: 2026-03-15, last_accessed: 2026-03-28` |
| **Relational** | "See also" links to other memories | `mem_003 (related), mem_007 (depends), mem_015 (contrasts)` |

**The Call Number Hierarchy:**
```
ARCH        — Architecture decisions
  .AUTH     — Authentication
    .MIDDLEWARE  — Middleware layer
      .TOKEN-VALIDATION  — Specific component
  .DB       — Database
  .API      — API layer
TEST        — Testing knowledge
  .UNIT     — Unit testing patterns
  .INTEG    — Integration testing patterns
TOOL        — Tooling and build knowledge
SEC         — Security knowledge
DOM         — Domain-specific knowledge
```

The hierarchy is auto-generated from the codebase structure and refined by Cataloger as memories accumulate. New top-level categories are rare; new leaf categories are common.

**"See Also" Links:**
The graph is the most powerful retrieval mechanism. When Shelver queries for "authentication," Cataloger:
1. Finds all memories with subject heading "authentication."
2. Traverses "see also" links one hop out: session management, token rotation, API v1 comparison.
3. Returns the combined set, ranked by relevance.

This traversal catches memories that keyword search misses. A memory about "token rotation" might not contain the word "authentication" but is relevant because it is linked via "see also."

**Retrieval Scoring:**
1. **Subject match** (0.35 weight): Does the query match any subject heading? Controlled vocabulary normalizes variants ("auth" → "authentication").
2. **Call number proximity** (0.25 weight): How close is the memory's call number to the query's inferred call number? `ARCH.AUTH.MIDDLEWARE` is close to `ARCH.AUTH.SESSION`.
3. **See-also distance** (0.20 weight): How many hops away is the memory in the "see also" graph? Direct links score highest. Two-hop links score less.
4. **Circulation frequency** (0.10 weight): How often has this memory been checked out? High-circulation memories are more likely to be relevant.
5. **Freshness** (0.10 weight): How recently was the memory validated?

**Expiration (Deaccession):**
Library science has a rigorous deaccession process. Memories are deaccessioned (expired) when:
- TTL expires and the memory has not been accessed in the last TTL/2 period.
- The memory's source files have been deleted or fundamentally rewritten.
- A newer memory with the same call number and higher confidence supersedes it.

Deaccessioned memories are moved to `refs/catalog/<agent-id>/archive/`, not deleted. They can be retrieved for historical research ("how did we used to handle authentication?").

**Compaction Survival:**
When the context window compacts:
1. Memories with >5 circulation checkouts are preserved in full.
2. Memories with 1-5 checkouts are summarized (subject headings + call number + content summary).
3. Uncirculated memories are listed by call number only.
4. The post-compaction system prompt includes a "ready reference" — the most-circulated memories in their full classification, costing ~1,500 tokens.

**Long-Term Storage (Union Catalog):**
The shared union catalog (`refs/catalog/shared/union_catalog.json`) is a cross-session, cross-agent merged catalog:
```json
{
  "collections": [
    {
      "agent": "shelver",
      "specialization": "patch_generation",
      "top_subjects": ["authentication", "api", "middleware"],
      "item_count": 47
    }
  ],
  "shared_vocabulary": {
    "auth": "authentication",
    "db": "database",
    "FE": "frontend"
  },
  "cross_references": [
    {
      "from_collection": "shelver",
      "to_collection": "cataloger",
      "link_count": 23,
      "strongest_link": "ARCH.AUTH → SEC.TOKEN-ROTATION"
    }
  ]
}
```

**Identity:**
```json
{
  "name": "shelver",
  "organization": "shelfos",
  "role": "code_placer_patch_author",
  "capabilities": ["patch_generation", "code_placement", "style_matching"],
  "authorization_scope": {
    "branches": ["shelf/*", "feat/*", "fix/*"],
    "max_patch_lines": 500,
    "collections": ["*"]
  },
  "openwallet_key_ref": "did:web:shelfos.io/agents/shelver",
  "created": "2026-03-28T00:00:00Z",
  "key_rotation_policy": "90d"
}
```

### Trade-offs

**Considered:** Single classification system (subject headings only). **Rejected:** a book classified only by subject is findable by topic but not by author, date, or location. Multiple classification systems make the same memory findable by multiple paths. The overhead of maintaining five systems is justified by the retrieval quality.

**Considered:** Unlimited "see also" links. **Rejected:** a graph where everything is connected to everything is not navigable. Maximum 5 links per memory forces Cataloger to prioritize the strongest relationships.

**Considered:** Automated classification only. **Rejected partially:** Cataloger auto-classifies all new memories, but agents can request reclassification when auto-classification is wrong. Human-equivalent quality requires occasional manual correction, and the system supports it.

---

## 6. Signed Commits via OpenWallet (RFP Section 3.6)

### Approach

Every agent commit is signed via OpenWallet. The signature includes catalog metadata: the call number of the work performed, the subject headings, and the classification scheme version.

### Design

**Signing Flow:**
```
1. Shelver produces INDEX.patch + COMMIT.msg
2. Circ prepares commit object with catalog metadata
3. Commit is signed with agent's OpenWallet key
4. Signature claims:
   - Agent identity (name, org, role)
   - Call number of the work performed (e.g., ARCH.AUTH.MIDDLEWARE)
   - Subject headings (e.g., ["authentication", "jwt"])
   - Authorization scope (branches, max lines, collections)
   - Budget: tokens used / total
   - Catalog version (classification scheme version hash)
5. Signed commit is pushed
```

The call number in the signature creates a classified audit trail. You can query "show me all commits classified under ARCH.AUTH" and get a complete, classified history of authentication changes.

**Authorization Model:**
Classification-based authorization:
- Agents are authorized to commit within specific call number ranges. Shelver can commit under `ARCH.*` and `DOM.*` but not `SEC.*` (security changes require a security-authorized agent).
- Branch patterns provide an additional constraint.
- The classification scheme version in the signature prevents authorization drift: if the classification scheme changes (a call number is renamed or restructured), old authorizations are validated against the scheme version that was current when they were issued.

```json
{
  "authorization_rules": [
    { "branches": "shelf/*", "call_numbers": ["ARCH.*", "DOM.*", "TEST.*"], "max_lines": 500 },
    { "branches": "feat/*", "call_numbers": ["ARCH.*", "DOM.*"], "max_lines": 300 },
    { "branches": "fix/*", "call_numbers": ["*"], "max_lines": 200 }
  ]
}
```

**Key Lifecycle:**
- **Provisioning:** Per-agent keys via OpenWallet. Cataloged in the agent identity record.
- **Rotation:** 90-day rotation. Old keys are "withdrawn" (library term for deaccessioned) from active use but retained for historical verification.
- **Revocation (routine):** Withdrawn keys remain valid for verifying past commits.
- **Revocation (compromise):** Key added to OpenWallet revocation list. Commits signed with the compromised key are flagged. The catalog is updated with a "see also" note on all memories created during the compromised period, linking them to a "key compromise event" memory.

### Trade-offs

**Considered:** Per-collection (per-repo) keys instead of per-agent keys. **Rejected:** agent identity should be portable across collections. An agent working in multiple repos should have one identity, one key.

---

## 7. Token Budget (RFP Section 3.7)

### Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 2,800 | 0 | Once per session | Agent identity (300), phase-gated tools (1,200-1,500), ready reference (800), vocabulary summary (500) |
| **Task acquisition** | 1,200 | 200 | Once per task | PR body, branch metadata, issue description. |
| **Classification (Cataloger)** | 1,500 | 800 | Once per task | Memory retrieval (1,000 in), "see also" traversal (500 in). Finding aid output (500 out), classification (300 out). |
| **Shelving (patch generation)** | 3,500 | 4,500 | Once per task | Target code (3,500 in). Patch (4,000 out) + COMMIT.msg (500 out). |
| **Tool call (per call)** | 500 | 150 | ~6 per task | Parameters (150 out), result (500 in). |
| **Post-task cataloging** | 500 | 800 | Once per task | Session results (500 in). New catalog entries with full classification (800 out). |
| **Memory retrieval** | 600 | 150 | 2 per task | Query (150 out), catalog results (450 in per retrieval). |
| **Circulation (PR/coordination)** | 1,000 | 800 | 1 per task | PR template (200 in), coordination context (800 in). PR body (500 out), coordination (300 out). |
| **TOTAL (typical task)** | **16,800** | **9,200** | -- | 200-line, 3-file feature with 1 cross-repo dependency |

**Grand total: ~26,000 tokens per typical task.**

### Budget Justification

ShelfOS's 26,000-token budget is the leanest of any Tier 1 proposal. This reflects the team's philosophy: do one thing and do it well, with no waste.

The savings come from three sources:
1. **Lean team (3 agents):** Less inter-agent communication overhead than 4- or 5-agent teams.
2. **Efficient retrieval:** Keyword + controlled vocabulary + "see also" graph traversal is cheaper than embedding-based retrieval (no LLM calls for relevance scoring in most cases).
3. **Phase-gated tool loading:** Loading 3-5 tools per phase instead of 10 saves 800-1,200 tokens on the system prompt.

The largest cost remains patch generation (8,000 tokens), which is irreducible for a 200-line feature. The classification overhead (2,300 tokens for pre-task classification + post-task cataloging) is the "library tax" — the cost of making today's work findable tomorrow. ShelfOS considers this mandatory.

The total team budget (40,000 tokens from AGENTS.md) accommodates one primary task (26,000), inter-agent coordination (~4,000), catalog maintenance (~3,000), and a 17% contingency (~7,000). The contingency covers unexpected classification complexity (e.g., a task that spans multiple call number domains and requires cross-reference generation).

---

## 8. Testing Strategy

### Provider-Agnostic Testing
- **Mock collection member** (mock LLM provider) with deterministic responses per lifecycle phase.
- **Classification consistency tests:** Same task description classified by the mock provider multiple times. Verify that subject headings and call numbers are consistent.

### Patch Workflow Validation
- **Round-trip shelving tests:** Receive task → classify → shelve → verify workspace state matches expected.
- **Misclassification tests:** Intentionally provide ambiguous task descriptions. Verify that Cataloger produces reasonable classifications and that Shelver's placement is defensible.
- **Budget exhaustion tests:** Set low budget, verify partial patch production and that cataloging reserve is respected.

### Cross-Repo Coordination Testing
- **Mock collection adapter** with catalog-protocol comment validation.
- **Interlibrary loan tests:** Simulate a cross-repo dependency as a loan request. Verify that the loan protocol correctly resolves the dependency and that call numbers are consistent across collections.

### Token Budget Testing
- **Phase budget tests:** Verify that phase-gated tool loading actually saves the expected tokens.
- **Cataloging cost tests:** Measure the token cost of classification (subject heading assignment, call number assignment, "see also" link generation) and verify it stays within the 2,300-token budget.
- **Lean team verification:** Compare ShelfOS's 3-agent performance against simulated 5-agent performance on the same tasks. Verify that the lean team completes tasks within budget without significant quality degradation.

---

## 9. Migration Path

The migration follows the library acquisition process:

1. **Accession (Phase 1):** `but ai mcp` is deployed alongside the existing server. Both coexist. The existing `gitbutler_update_branches` tool is preserved and internally routed through the catalog system. Memories from existing tool usage are passively cataloged.
2. **Processing (Phase 2):** Existing clients migrate to `shelve_task`. The passive catalog from Phase 1 provides initial context, so the first classified tasks benefit from prior work.
3. **Circulation (Phase 3):** Old MCP server decommissioned. The catalog is fully operational. Circulation tracking begins generating collection development data.

The migration is designed to be invisible to the patron (the developer). They walk into the same library. The shelves are just better organized now.

---

*Submitted by ShelfOS. Every book in its place. Every patron on their way.*
