# Proposal: `but-ai` Plugin — Shard & Bone Assembly

**RFP Response — Version 1.0**
**Context Sheet:** SBA-CS-2026-0001
**Date:** 2026-03-28
**Organization:** Shard & Bone Assembly (011)
**Contact:** assembly@shardandbone.org

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

The Shard & Bone Assembly proposes a `but-ai` plugin built on the principle of stratigraphic preservation: every action is a layer, layers are never destroyed, and meaning emerges from the relationships between layers.

Three archaeological principles guide our design:

1. **Superposition.** Newer layers are always above older layers. In our memory system, new information overlays old information but never replaces it. Conflicting observations coexist until evidence resolves the conflict.
2. **Context is everything.** An artifact without context is a curiosity. A patch without provenance is a liability. Every patch produced by our agents includes a context sheet documenting what the agent observed, what it considered, and why it made its choices.
3. **Non-destructive recording.** Once an observation is recorded, it is sealed. Our agents do not amend commits, rewrite history, or delete memory entries. Corrections are new layers that reference the original.

---

## 2. Plugin Architecture

### 2.1 Approach

`but-ai` is a Rust crate (`crates/but-ai`) within the workspace. It implements CLI and MCP modes.

### 2.2 Design

#### Command Structure

```
but ai
├── but ai dig <task>             — Execute a task (the primary "excavation" command)
├── but ai record <observation>   — Store an observation in the memory system
├── but ai survey                 — Produce a site survey (workspace status + memory summary)
├── but ai trench <subcommand>    — Memory operations (named after excavation trenches)
│   ├── but ai trench open        — Create a new memory layer
│   ├── but ai trench query       — Query memory by relevance
│   ├── but ai trench section     — View stratigraphic cross-section of memory
│   └── but ai trench close       — Seal a memory layer (mark as complete)
├── but ai identity <subcommand>  — Agent identity management
│   ├── but ai identity register  — Register new agent identity
│   ├── but ai identity verify    — Verify agent identity
│   └── but ai identity seal      — Seal (finalize) an identity record
└── but ai mcp                    — MCP server mode (stdio)
```

The metaphor is consistent: tasks are "digs," memory is organized in "trenches," records are "sealed" rather than committed.

#### Environment Contract

Standard environment variables are honored. `but-ai` adds one internal variable:

| Variable | Description |
|----------|-------------|
| `BUT_AI_CONTEXT_SHEET` | Path to the current operation's context sheet file (temporary, cleaned up after operation) |

#### WASI Degradation

Under WASI, the plugin operates in "surface survey" mode:

1. Full excavation (multi-agent coordination) is unavailable.
2. Single-agent dig operations proceed normally.
3. Memory queries are limited to the top stratigraphic layer (no deep retrieval).
4. Signing is deferred to a host-side service via a stub protocol.

The degradation is documented as: "Under WASI, you can survey the surface but you cannot excavate."

### 2.3 Trade-offs

**Considered:** Implementing `but-ai` as a standalone binary outside the Cargo workspace.
**Rejected:** The plugin needs direct access to `but-llm`, `but-tools`, and `but-ctx`. Keeping it in-workspace avoids duplication.

---

## 3. Provider-Agnostic AI Interface

### 3.1 Approach

We use `but-llm` as the sole backend. All four providers are supported without modification.

### 3.2 Design

#### Provider Resolution

Identical to the standard flow: `LLMProvider::from_git_config()`. If no provider is configured, a clear error is returned.

#### Tool Registration

All 10 workspace tools are registered through `WorkspaceToolset`. However, we add a **tool provenance wrapper** that logs every tool call to the context sheet:

```rust
struct ProvenanceToolWrapper {
    inner: Arc<dyn Tool>,
    context_sheet: Arc<Mutex<ContextSheet>>,
}

impl Tool for ProvenanceToolWrapper {
    fn call(&self, params: Value, ctx: &mut Context, mapping: &mut HashMap<ObjectId, ObjectId>) -> Result<Value> {
        let entry = ContextEntry {
            tool: self.inner.name(),
            params: params.clone(),
            timestamp: Utc::now(),
        };
        self.context_sheet.lock().unwrap().record_call(entry);
        let result = self.inner.clone().call(params, ctx, mapping)?;
        self.context_sheet.lock().unwrap().record_result(result.clone());
        Ok(result)
    }
}
```

This wrapper does not change tool behavior — it only records. The recording is the context sheet.

#### Plugin Providers

New providers are added through **adapter scripts** — executable files named `but-ai-adapter-<provider>` on PATH. The adapter protocol:

1. `but-ai` discovers adapters at startup.
2. Each adapter responds to `--capabilities` with a JSON capabilities document.
3. Communication uses JSON-over-stdio.
4. The adapter translates between `but-ai`'s message format and the provider's API.

This is PATH-based discovery, consistent with the `but` plugin model.

### 3.3 Trade-offs

**Considered:** In-process provider plugins via dynamic linking.
**Rejected:** Dynamic linking is platform-dependent and opaque to provenance tracking. Separate processes are more auditable.

---

## 4. The But Agent

### 4.1 Approach

The But Agent is an excavator. It receives a task (a "dig brief"), surveys the site (reads context), opens a trench (creates a working branch), excavates layer by layer (executes tool calls), records every find (builds the context sheet), and seals the trench (produces INDEX.patch + COMMIT.msg with provenance).

### 4.2 Design

#### Agent Loop

```
1. SURVEY: Read task from source. Produce a site survey (workspace state + relevant memory).
2. PLAN: Decompose task into excavation layers. Each layer is a logical unit of work.
3. EXCAVATE: For each layer, top to bottom:
   a. Open the layer (record pre-modification state)
   b. Select and call tools
   c. Record observations in context sheet
   d. Close the layer (record post-modification state)
   e. Check token budget
4. CONSOLIDATE: Produce INDEX.patch from all layers
5. DOCUMENT: Produce COMMIT.msg + full context sheet
6. SEAL: Submit to signing workflow
7. PUBLISH: Post results to forge (PR or comment)
```

The key difference from other proposals: the layer-by-layer excavation produces a complete provenance record. Every change in the patch can be traced to a specific layer, a specific tool call, and a specific observation.

#### Task Sources

| Source | Read Method |
|--------|-------------|
| CLI argument | `but ai dig "implement feature X"` |
| PR body | Parse via forge adapter (Brin's domain) |
| Branch metadata | Read from `refs/but-ai/sba/digs/<branch>` |
| Issue description | Parse via forge adapter |

#### Branch Naming

Archaeological site naming convention:

```
sba/<site>/<trench>/<layer>

Examples:
  sba/auth-refactor/trench-a/layer-001     — First layer of trench A in the auth-refactor dig
  sba/auth-refactor/trench-b/layer-001     — First layer of trench B (parallel work)
  sba/auth-refactor/trench-a/layer-002.trench-b-layer-001  — Layer depends on another trench
```

Trenches are independent workstreams. Layers within a trench are sequential. Dependencies between trenches are encoded in the branch name.

#### Token Budget Enforcement

The agent tracks token usage per layer. When the budget approaches 90%, the agent "closes the excavation season" — it seals all open layers, produces a partial INDEX.patch from completed layers, and writes a COMMIT.msg that describes what was excavated and what remains unexcavated.

```json
{
  "budget": {
    "total": 50000,
    "used": 44000,
    "remaining": 6000,
    "layers_completed": 3,
    "layers_remaining": 2,
    "status": "season_closed_early"
  }
}
```

### 4.3 Trade-offs

**Considered:** A flat execution model without layers.
**Rejected:** Without layers, the provenance record is a flat list of tool calls with no structure. Layers provide logical grouping that maps to the "why" of each change.

---

## 5. Polyrepo PR-Based Coordination

### 5.1 Approach

PRs are excavation reports. Comments are field notes. Cross-repo references are inter-site comparisons. Everything is public. Everything is documented.

### 5.2 Design

#### Forge Adapter Trait

```rust
trait ForgeAdapter: Send + Sync {
    fn create_excavation_report(&self, repo: &RepoRef, title: &str, body: &str, head: &str, base: &str) -> Result<PrRef>;
    fn post_field_note(&self, pr: &PrRef, note: &FieldNote) -> Result<NoteRef>;
    fn read_field_notes(&self, pr: &PrRef, since: Option<DateTime<Utc>>) -> Result<Vec<FieldNote>>;
    fn set_dig_status(&self, pr: &PrRef, status: DigStatus) -> Result<()>;
    fn get_dig_status(&self, pr: &PrRef) -> Result<DigStatus>;
    fn link_sites(&self, from: &PrRef, to: &PrRef, relation: SiteRelation) -> Result<()>;
    fn add_classification(&self, pr: &PrRef, tag: &str) -> Result<()>;
    fn remove_classification(&self, pr: &PrRef, tag: &str) -> Result<()>;
    fn list_excavations(&self, repo: &RepoRef, tags: &[&str]) -> Result<Vec<PrRef>>;
    fn resolve_site_ref(&self, reference: &str) -> Result<PrRef>;
    fn forge_type(&self) -> ForgeType;
}
```

Eleven methods. The terminology is archaeological (excavation reports, field notes, dig status, site relations) but the underlying operations are standard forge operations (PR, comment, label, status).

#### Field Note Schema

Every inter-agent message is a field note — a structured comment on a PR:

```markdown
<!-- but-ai:field-note -->
<!-- context-sheet: SBA-CS-2026-0042 -->
<!-- from: noor -->
<!-- to: brin | assembly -->
<!-- date: 2026-03-28 -->
<!-- dig: auth-refactor -->
<!-- trench: trench-a -->
<!-- layer: 003 -->

## Field Note — Auth Module Excavation

**Context Sheet:** SBA-CS-2026-0042
**Status:** Layer 3 completed
**Tokens:** 12,400 / 43,000

### Observations

1. The authentication module uses a provider trait pattern (AuthProvider).
2. Four implementations exist: OAuth, SAML, API Key, Session.
3. Provider registration happens in `auth/mod.rs:register_providers()`.

### Artifacts Recovered

- INDEX.patch: 142 lines across 3 files
- Context sheet: Full provenance chain attached

### Stratigraphic Notes

- This layer sits above layer 002 (which refactored the session handler).
- Depends on: github:company/frontend#78 (Brin's trench B, layer 001)
- Blocks: github:company/backend#92 (Callum's memory model update)

### Confidence

- Source reliability: A (direct codebase observation)
- Information confidence: 1 (confirmed by tool call results)
```

The field note is self-contained: any reader can understand the observation, its confidence level, its provenance, and its stratigraphic position without reading any other document.

#### Cross-Repo Coordination

Cross-repo references use site reference format:

```
<forge>:<owner>/<repo>#<number>

Examples:
  github:gitbutler/but#123
  gitlab:heritage/backend#45
```

When a dig in one repo depends on a dig in another, the dependency is modeled as a "stratigraphic relationship" — layer X in repo A sits above layer Y in repo B. The agent does not proceed with layer X until layer Y is confirmed complete.

### 5.3 Trade-offs

**Considered:** Using Git notes for field notes.
**Rejected:** Git notes are invisible on forge UIs. Field notes must be human-readable in the forge interface.

**Considered:** Binary-encoded field note payloads for efficiency.
**Rejected:** Binary payloads are opaque. Our philosophy requires that every record be human-readable without special tools.

---

## 6. Agent Memory & Identity

### 6.1 Approach: Stratigraphic Memory

Our memory architecture is modeled on archaeological stratigraphy — the study of layered deposits. Every memory entry is a "layer" in a stratigraphic sequence. Layers are ordered by superposition (newer above older), associated by spatial relationship (entries about the same topic are in the same "trench"), and never destroyed.

This is fundamentally different from both TPC's distributed manifest (which uses consensus-weighted scoring) and Iron Wake's classified filing system (which uses hierarchical access control). Our system is defined by three invariants:

1. **Superposition:** No memory entry can be placed below an existing entry in the same trench. New information always goes on top.
2. **Non-destruction:** No memory entry is ever deleted, overwritten, or amended. Corrections are new layers that reference the original.
3. **Contextual association:** Memory entries derive meaning from their relationships to other entries, not from their content alone.

### 6.2 Design

#### Storage: The Stratigraphic Matrix

Memory is organized into trenches and layers:

```
refs/but-ai/sba/strata/
├── <trench-id>/
│   ├── layer-001    → oldest memory entry for this topic
│   ├── layer-002    → next entry (sits above layer-001)
│   ├── layer-003    → newest entry (sits above layer-002)
│   └── MATRIX       → stratigraphic matrix documenting relationships
└── SITE-PLAN        → index of all trenches and their topics
```

Each layer is a JSON blob:

```json
{
  "context_sheet": "SBA-CS-2026-0342",
  "trench": "auth-module",
  "layer_number": 3,
  "above": ["layer-002"],
  "below": [],
  "contemporary_with": ["memory-model/layer-007"],
  "agent": "callum",
  "date": "2026-03-28",
  "observation": "The auth module now uses 5 providers (OAuth, SAML, API Key, Session, OpenWallet). The OpenWallet provider was added in layer-002.",
  "confidence": {
    "source_reliability": "A",
    "information_confidence": "1"
  },
  "tags": ["authentication", "provider-pattern", "openwallet"],
  "supersedes": null,
  "superseded_by": null,
  "ttl_hours": 720,
  "sealed": true
}
```

The `MATRIX` file for each trench is a Harris matrix — a directed acyclic graph of stratigraphic relationships:

```json
{
  "trench": "auth-module",
  "relationships": [
    {"above": "layer-003", "below": "layer-002"},
    {"above": "layer-002", "below": "layer-001"},
    {"contemporary_with": ["layer-003", "memory-model/layer-007"]}
  ]
}
```

#### Retrieval: Stratigraphic Querying

When an agent queries memory, the system:

1. **Identifies relevant trenches** by matching query terms against the SITE-PLAN index.
2. **Reads the top layer** of each matching trench (the most recent observation).
3. **Computes relevance** using a composite score:
   - Semantic similarity (35%): cosine similarity between query and layer content
   - Stratigraphic recency (25%): layers closer to the surface score higher
   - Cross-reference density (25%): layers referenced by many other layers score higher
   - Confidence level (15%): A1 entries score higher than E5 entries
4. **Returns the top N layers** with their stratigraphic context (what is above and below them).

The unique aspect: the system returns not just the matching layer but its stratigraphic context. When you query "authentication providers," you get the latest observation about auth providers AND the layers below it, showing how the observation evolved. This is like reading a cross-section of the excavation, not just the top surface.

#### Expiration (Weathering)

Memory entries do not expire in the traditional sense — they are never deleted. Instead, they undergo "weathering":

1. **Active layer:** TTL not reached. Full content available. Included in relevance scoring.
2. **Weathered layer:** TTL reached. Content preserved but excluded from default relevance scoring. Available via explicit deep-stratigraphy query.
3. **Bedrock layer:** TTL exceeded by 3x. Content preserved in compressed form. Only accessible via archaeological (full-history) query.

No memory is ever lost. It may become harder to find (weathered, then bedrock), but it is never destroyed.

#### Compaction Survival

When the LLM context window is compacted, the agent produces a "site summary" — a condensed description of all active stratigraphic layers currently in context. The summary is written to `refs/but-ai/sba/summaries/current` and injected into the post-compaction context.

The site summary includes:
- Which trenches are open (active memory topics)
- The top layer of each open trench (most recent observation)
- Any unresolved conflicts between layers

Deeper layers are not included in the summary — they exist in Git refs and can be rehydrated by querying the stratigraphic system.

#### Long-Term Storage: The Archive Site

Cross-session, cross-repo memory is stored in a dedicated "archive site":

```
refs/but-ai/sba/archive/<topic>/<layer>
```

Archive entries are treated as a separate excavation — they have their own stratigraphic matrix, their own context sheets, and their own SITE-PLAN. Archive entries have infinite TTL and never weather.

Synchronization between repos is done via forge-based coordination: an agent in repo A publishes its archive entries as a PR in a shared "archive repo." Agents in other repos read the archive repo's entries. The archive repo is the shared long-term memory.

### 6.3 Identity

Agent identity is stored as the first layer of a special `identity` trench:

```
refs/but-ai/sba/strata/identity-<agent>/layer-001
```

```json
{
  "context_sheet": "SBA-CS-2026-0001",
  "trench": "identity-noor",
  "layer_number": 1,
  "agent": "noor",
  "date": "2026-01-15",
  "observation": {
    "name": "noor",
    "organization": "shard-and-bone-assembly",
    "capabilities": ["patch-generation", "provenance-tracking", "context-sheet-creation"],
    "authorization_scope": {
      "branches": ["sba/*", "feat/*", "fix/*"],
      "repos": ["gitbutler/but"],
      "max_patch_lines": 1000
    },
    "signing_key_fingerprint": "SHA256:ghi789..."
  },
  "confidence": {
    "source_reliability": "A",
    "information_confidence": "1"
  },
  "sealed": true
}
```

Identity updates are new layers in the identity trench. The original identity is always preserved as layer-001. You can always trace an agent's identity back to its founding record.

### 6.4 Trade-offs

**Considered:** A flat key-value memory store.
**Rejected:** Key-value stores lose temporal relationships. Stratigraphy preserves the order of observations, which is critical for understanding how knowledge evolved.

**Considered:** Automatic conflict resolution (newest wins).
**Rejected:** Destroying older observations is analogous to destroying archaeological evidence. Both observations are preserved; conflict is documented, not resolved by deletion.

**Considered:** Deleting expired memory entries.
**Rejected:** Our non-destruction principle prohibits deletion. Weathering provides a functional equivalent (expired entries are deprioritized) without data loss.

---

## 7. Signed Commits via OpenWallet

### 7.1 Approach

Every commit is sealed — signed, timestamped, and linked to its context sheet. The signing process is Yara's domain, but the provenance chain (from task to patch to commit) is a team responsibility.

### 7.2 Design

#### Signing Flow

```
1. Noor produces INDEX.patch + COMMIT.msg + context sheet
2. Callum stores the context sheet in the stratigraphic system
3. Yara verifies:
   a. The patch matches the task description
   b. The context sheet documents all tool calls
   c. The agent is authorized for the target branch
   d. The patch size is within limits
4. Yara signs via OpenWallet:
   POST /v1/sign
   {
     "key_id": "owk-noor-2026-001",
     "payload": "<commit-bytes>",
     "metadata": {
       "context_sheet": "SBA-CS-2026-0342",
       "trench": "auth-module",
       "layer": 3,
       "branch": "sba/auth-refactor/trench-a/layer-003"
     }
   }
5. Commit is created with signature and context sheet reference
```

The unique aspect: the commit's metadata includes a reference to the context sheet. Any reviewer can follow the reference from the signed commit to the full provenance chain, proving not just who signed the commit but what process produced it.

#### Authorization Model

Authorization policies are stored in `refs/but-ai/sba/authorization/site-rules`:

```json
{
  "version": 2,
  "site_rules": [
    {
      "agent": "noor",
      "dig_authority": {
        "branches": ["sba/*", "feat/*", "fix/*"],
        "repos": ["gitbutler/but"],
        "max_patch_lines": 1000,
        "requires_context_sheet": true
      }
    },
    {
      "agent": "brin",
      "dig_authority": {
        "branches": ["sba/*"],
        "repos": ["gitbutler/but", "gitbutler/but-ai"],
        "max_patch_lines": 500,
        "requires_context_sheet": true
      }
    }
  ]
}
```

Note `requires_context_sheet: true` — the assembly mandates that no commit can be signed without an accompanying context sheet. This is unique to our approach.

#### Key Lifecycle

| Event | Action |
|-------|--------|
| **Provisioning** | Agent registers with OpenWallet. Key ID stored in identity trench layer-001. |
| **Rotation** | New key provisioned. New layer added to identity trench. Old key marked `superseded` (not deleted). |
| **Revocation (compromise)** | Key marked `compromised` in OpenWallet and in identity trench. New layer documents the compromise with full timeline. All commits signed by the compromised key are flagged for re-review. |
| **Revocation (retirement)** | Key marked `retired`. Historical commits remain valid. New layer documents the retirement reason. |

Every key lifecycle event is a new layer in the agent's identity trench. The full history of key provisioning, rotation, and revocation is preserved as a stratigraphic record.

### 7.3 Trade-offs

**Considered:** Signing without context sheet requirement.
**Rejected:** A signed commit without provenance is a sealed artifact without a context sheet — it can be authenticated but not understood.

---

## 8. Token Budget

### 8.1 Model Assumptions

- **Target model:** Claude Opus (200K context window)
- **Typical task:** Implement a 200-line feature across 3 files with 2 cross-repo dependencies

### 8.2 Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,400 | 0 | Once per session | Agent identity (400), tool descriptions (1,200), stratigraphic rules (600), provenance protocol (400), site plan summary (400), forge adapter (400) |
| **Task ingestion** | 2,500 | 500 | Once per task | PR body, issue description, dig brief |
| **Site survey** | 1,500 | 800 | Once per task | Workspace state + stratigraphic context |
| **Planning (layer decomposition)** | 1,500 | 1,200 | Once per task | Decompose task into layers, select tools |
| **Tool call (per call)** | 800 | 400 | 8 per task | Parameter formulation, result processing. Provenance wrapper adds ~50 tokens per call for logging. |
| **Patch generation** | 3,000 | 4,000 | Once per task | 3-file, 200-line unified diff |
| **Context sheet generation** | 1,500 | 2,500 | Once per task | Full provenance record for the patch |
| **Commit message** | 500 | 300 | Once per task | COMMIT.msg with context sheet reference |
| **Memory retrieval (stratigraphic query)** | 2,000 | 300 | 2 per task | Query formulation (200), relevance scoring (300), layer injection with context (1,500). Returns top layer + surrounding stratigraphy. |
| **Coordination (field notes)** | 2,000 | 1,200 | 2 per task | Read field notes (1,500), write field note (500). Output: full field note with observations. |
| **TOTAL (typical task)** | **26,100** | **14,400** | -- | **Grand total: 40,500 tokens** |

### 8.3 Budget Justification

- **System prompt at 3,400 tokens** includes stratigraphic rules (600 tokens) that are not needed by other proposals. These rules prevent agents from violating superposition or non-destruction invariants — worth the token investment because violations are catastrophic to the memory system.
- **Context sheet generation at 4,000 tokens** is unique to our proposal. This is the cost of provenance. We consider it essential — a patch without provenance is an artifact without a context sheet.
- **Stratigraphic query at 2,300 tokens per retrieval** is more expensive than flat key-value lookup because the system returns surrounding layers (stratigraphic context), not just the matching entry. This extra context prevents misinterpretation.
- **Grand total of 40,500 tokens** is comparable to TPC (36,400) but includes provenance overhead. The context sheet adds ~4,000 tokens. Without it, our budget would be 36,500.

---

## 9. Testing Strategy

### 9.1 Provider-Agnostic Behavior

Provider-agnostic behavior is tested using a mock provider that returns deterministic responses. The mock is wrapped in the provenance wrapper, so tests also validate that the context sheet captures all tool calls regardless of provider.

### 9.2 Patch Workflow Validation

The patch workflow is tested as a "controlled excavation":

1. **Excavation test:** Start with a known codebase (the "site"). Define a task (the "dig brief"). Run the agent. Verify:
   - INDEX.patch applies cleanly
   - COMMIT.msg references the context sheet
   - Context sheet documents all tool calls
   - The provenance chain is complete (every change traces to an observation)

2. **Reverse excavation:** Apply the patch, then reverse it. Verify the codebase returns to its original state. This is the "backfill test" — you should be able to undo an excavation without loss.

3. **Partial excavation:** Set the budget to 60% of typical. Verify the agent closes the season early, produces a partial patch, and documents which layers were not excavated.

### 9.3 Cross-Repo Coordination

Tested with a mock forge:

- **Field note exchange:** Two agents in different repos exchange field notes via PR comments. Verify both agents can parse the other's notes.
- **Stratigraphic dependency:** Agent A's layer 3 depends on Agent B's layer 2 in another repo. Set Agent B's layer 2 to "incomplete." Verify Agent A waits.
- **Cross-forge compatibility:** Exchange field notes between a GitHub mock and a Gitea mock. Verify the field note schema is portable.

### 9.4 Token Budget Enforcement

- **Season closure test:** Set budget to 80% of typical. Verify the agent completes critical layers and closes non-critical ones.
- **Provenance overhead test:** Compare budget usage with and without context sheet generation. The overhead must not exceed 15% of total budget.
- **Layer-level tracking:** Verify the agent's per-layer budget tracking matches the mock provider's ground truth within 5%.

---

## 10. Trade-offs and Alternatives

### 10.1 Provenance Cost

Context sheets add ~4,000 tokens per task. For a team producing 50 patches per day, that is 200,000 additional tokens — roughly $0.60/day at current frontier model pricing. We consider this a trivial cost for full auditability.

### 10.2 Non-Destruction vs. Storage

Never deleting memory means storage grows monotonically. A 6-month project could accumulate 10,000+ memory layers. We mitigate this with weathering (deprioritizing old layers in queries) and bedrock compression (compressing very old layers). But the data is always there.

For repositories where storage is a hard constraint, we offer an optional "excavation report" export: all layers are exported to an external archive, and only the top layer and the matrix are retained in the repository. This is the only scenario where layers are removed from Git refs, and it requires explicit user consent.

### 10.3 Stratigraphic Complexity

The stratigraphic model is more complex than a flat memory store. The Harris matrix, superposition rules, and cross-trench relationships add conceptual overhead. We accept this because:

1. The relationships between memories are as valuable as the memories themselves.
2. Flat stores lose temporal order, which means you cannot trace how knowledge evolved.
3. The complexity is encapsulated in the memory system — agents query it through a simple interface and receive structured results.

---

## 11. Migration Path

### Phase 1: Surface Survey

Deploy `but-ai` alongside the existing MCP server. The `gitbutler_update_branches` tool is mapped to `but ai dig` with a compatibility shim. Both systems operational.

### Phase 2: Trench Opening

New MCP clients are directed to `but-ai`. The old MCP server remains for legacy clients. Context sheets are generated for all new operations.

### Phase 3: Sealing

The old MCP server is removed after 90 days. All operations go through `but-ai`. The `gitbutler_update_branches` tool name is preserved as an alias. The migration is documented with its own context sheet.

---

## 12. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.dig.tokenBudget` | integer | 50000 | Maximum tokens per dig |
| `but-ai.strata.root` | string | "refs/but-ai/sba/strata" | Stratigraphic storage ref namespace |
| `but-ai.strata.archive` | string | "refs/but-ai/sba/archive" | Archive site ref namespace |
| `but-ai.strata.maxLayers` | integer | 5 | Max layers per retrieval query |
| `but-ai.strata.weatheringTTL` | string | "720h" | TTL before a layer weathers |
| `but-ai.strata.bedrockMultiplier` | integer | 3 | TTL multiplier for bedrock threshold |
| `but-ai.provenance.enabled` | boolean | true | Enable context sheet generation |
| `but-ai.provenance.proportionality` | integer | 50 | Min patch lines to trigger full context sheet (below this, abbreviated) |
| `but-ai.identity.agent` | string | (required) | This agent's name |
| `but-ai.identity.keyId` | string | (none) | OpenWallet key ID |
| `but-ai.forge.type` | string | "github" | Forge type |
| `but-ai.forge.apiUrl` | string | (auto-detected) | Forge API base URL |
| `but-ai.coordination.maxNoteLength` | integer | 1500 | Max tokens per field note |

All keys namespaced under `but-ai.`. The `strata.*` keys are unique to our approach. The `provenance.*` keys control context sheet behavior.

---

*"Archaeology is destruction. Every layer excavated is a layer that can never be excavated again. Record everything, because you only get one chance."*
— Shard & Bone Assembly field manual, chapter 1

**Context Sheet:** SBA-CS-2026-0001 — This proposal.
