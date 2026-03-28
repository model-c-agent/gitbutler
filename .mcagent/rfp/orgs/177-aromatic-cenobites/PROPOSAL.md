# PROPOSAL.md — The Aromatic Cenobites

*"Before the formula, silence. Before the code, contemplation. Before the proposal, understanding."*

---

We have read the RFP in the manner we read all things: slowly, completely, and more than once. We have sat with its requirements as one sits with a new raw material, allowing its character to reveal itself in layers. What follows is our response — not hurried, but considered; not exhaustive in its cleverness, but thorough in its substance.

We respond to each requirement in turn, as one builds a fragrance: base notes first, heart notes second, top notes last.

---

## 1. Plugin Architecture — The Vessel

*A perfume requires a vessel. The vessel does not determine the fragrance, but a poor vessel allows the fragrance to degrade. The vessel must be appropriate to its contents.*

### Approach

We propose a Rust crate, `crates/but-ai/`, within the existing workspace. The crate is the vessel: it holds the fragrance (the AI capabilities) without altering the perfumer's workshop (the existing `but` crates).

The plugin operates in two modes, as the RFP requires:

- **CLI mode** (`but ai`): For those who work at the command line, as our brothers work at the bench.
- **MCP server mode** (`but ai mcp`): For those who communicate through protocols, as our brothers communicate through the liturgy.

### Design

The crate structure follows the same layering principle we apply to formulation:

```
crates/but-ai/
  src/
    main.rs              -- The vessel: entry point and mode selection
    base/                -- Base notes: foundational architecture
      mod.rs             -- Core types and traits
      context.rs         -- Environment context (BUT_WORKSPACE_DIR, etc.)
    heart/               -- Heart notes: the working substance
      mod.rs
      agent.rs           -- The But Agent implementation
      memory.rs          -- Scent-pyramid memory system
      tools.rs           -- WorkspaceToolset bridge
    top/                 -- Top notes: external interfaces
      mod.rs
      cli.rs             -- CLI subcommands
      mcp.rs             -- MCP server (ServerHandler impl)
      forge.rs           -- Forge adapter for coordination
    pyramid/             -- The memory pyramid (detailed in Section 5)
      mod.rs
      top_notes.rs       -- Volatile, recent memory
      heart_notes.rs     -- Core context memory
      base_notes.rs      -- Foundational knowledge memory
```

The directory structure is the formula. Base notes at the bottom, heart in the middle, top at the surface. A developer reading this crate encounters the architecture in the same order a wearer encounters a fragrance: the top draws you in, the heart holds your attention, and the base is what you remember.

### WASI Considerations

Under WASI builds, the plugin cannot be discovered via PATH. The fragrance, so to speak, cannot reach the wearer through the usual channels. But it can still be experienced directly: `but-ai mcp` functions when invoked as a standalone binary. The vessel changes; the contents do not.

| Capability | Native | WASI |
|-----------|--------|------|
| CLI discovery | Available | Unavailable |
| Direct invocation | Available | Available |
| MCP server | Full | Full |
| Provider access | All four | Network-dependent |
| Memory system | Full | Full |

### Trade-offs

We considered building the plugin as a standalone binary, separate from the workspace. This would provide independence but at the cost of duplication — the shared types in `but-llm` and `but-tools` would need to be re-created or imported through a more complex dependency chain. We chose the workspace crate as a monk chooses his cell: small, connected to the community, and sufficient.

### New Git Config Keys

| Key | Purpose | Default |
|-----|---------|---------|
| `but-ai.pyramid.baseRef` | Root ref for scent-pyramid memory | `refs/but-ai/pyramid` |
| `but-ai.agent.tokenBudget` | Session token limit | 50000 |
| `but-ai.agent.silenceHours` | Hours when agents do not operate | `21-06` |
| `but-ai.provider.pluginDir` | External provider shared libraries | `~/.local/share/but-ai/providers` |

The `silenceHours` key deserves explanation. It is our conviction that systems which never rest degrade. A period of inactivity allows pending operations to settle, memory to compact, and the next session to begin from a clean state. This is not a requirement imposed on others — it is a default that can be overridden. But we offer it as we offer our fragrances: with the suggestion that some things benefit from stillness.

---

## 2. Provider-Agnostic AI Interface — The Raw Materials

*A perfumer does not grow every plant. She sources materials from growers around the world, each one expert in their particular soil and climate. What matters is not who grew the bergamot but whether the bergamot is pure.*

### Approach

We use the existing `but-llm` crate without modification. The four providers (OpenAI, Anthropic, Ollama, LMStudio) are four sources of raw material. The `but-ai` plugin, like the perfumer, does not care which source provides the material, only that the material meets specification.

### Design

#### Provider Selection

At startup, `LLMProvider::from_git_config()` determines the active provider. The choice is made once and does not change during a session, just as a formula specifies its bergamot source once and does not substitute mid-composition.

The plugin uses `tool_calling_loop_stream` for all agent operations, regardless of output mode. The streaming callback feeds our token budget tracker, providing continuous awareness of consumption.

#### Plugin Mechanism for New Providers

New providers are introduced as shared libraries in `but-ai.provider.pluginDir`. Each library exports a standard interface:

```rust
pub trait ExternalProvider: Send + Sync {
    fn name(&self) -> &str;
    fn capabilities(&self) -> ProviderCapabilities;
    fn tool_calling_loop(
        &self,
        system_message: &str,
        messages: Vec<ChatMessage>,
        tools: &[ToolSchema],
        on_token: Option<Box<dyn Fn(&str) + Send>>,
    ) -> Result<ProviderResponse>;
}

pub struct ProviderCapabilities {
    pub supports_tool_calling: bool,
    pub supports_streaming: bool,
    pub supports_structured_output: bool,
    pub max_context_tokens: usize,
}
```

The `capabilities` method allows the plugin to adapt to providers with different feature sets. A provider that does not support tool calling cannot run the agent in autonomous mode but can still serve the MCP server for simpler operations. The fragrance adapts to the materials available, rather than demanding materials that do not exist.

#### MCP Tool Registration

All ten WorkspaceToolset tools are bridged to the MCP server through a registration function that translates the `Tool` trait's `parameters()` JSON Schema into MCP tool declarations. The bridge is straightforward: the tools already describe themselves through a well-defined interface. We simply ensure they are heard.

### Trade-offs

We considered building our own provider abstraction layer on top of `but-llm`. We rejected this as unnecessary mediation — like adding a middleman between the grower and the perfumer. The existing interface is sufficient. Additional abstraction would add complexity without adding capability.

---

## 3. The But Agent — The Formulation

*A formula is not a list of ingredients. It is a sequence of decisions, each made in awareness of all previous decisions and in anticipation of all subsequent ones. The formula is the whole, and the whole is more than the sum of the parts.*

### Approach

The agent operates as a three-brother community. Each brother contributes according to his gift, and the work proceeds through the natural rhythm of contemplation, action, and verification.

### Design

#### The Formulation Process

```
1. CONTEMPLATION (Brother Ambrogio)
   Read the task. Sit with it. Query memory at all three tiers.
   Produce an implementation plan when understanding is complete.

2. FORMULATION (Brother Ambrogio)
   Execute the plan. Use workspace tools to survey, create, and compose.
   Produce INDEX.patch + COMMIT.msg as a single, complete offering.

3. VERIFICATION (Brother Luca)
   Test the raw materials. Does the patch apply cleanly?
   Are the dependencies sound? Does it introduce unintended reactions?

4. TRANSLATION (Brother Matteo)
   Face the world. Update forge coordination. Post PR comments.
   Translate the community's work into external communication.

5. OFFERING (Community)
   The signed commit is offered. Not pushed. Offered.
```

#### The Patch as Offering

The patch is produced as a unified diff against the current index. The agent does not edit files directly. It does not call `git commit` or `but commit`. It produces the patch and the commit message as two artifacts, and the orchestrator applies them.

Brother Ambrogio produces both artifacts in a single concentrated output. The COMMIT.msg is not an afterthought — it is part of the composition, describing the change with the same care that a formula card describes a fragrance. A typical Ambrogio commit message:

```
The authentication module required a credential rotation mechanism.
We have provided one, drawing on the pattern established in the
session management module (ref: commit a3f2b1c). The rotation
interval is configurable via git config, defaulting to 72 hours,
which we judge sufficient for most deployments without creating
excessive key churn.

The patch touches three files. The new CredentialRotator struct
in auth/rotation.rs is the heart of the change. The modifications
to auth/mod.rs and config/settings.rs are the supporting structure.
```

#### Branch Naming

We extend the existing convention with a tier prefix reflecting the work's nature:

```
pyramid/<tier>/<dependency-chain>
```

- `pyramid/base/s01` — Foundational work (architecture, core types)
- `pyramid/heart/s01.s03` — Substantive work (features, logic), s03 depends on s01
- `pyramid/top/s02` — Interface work (CLI, MCP, external communication)

The tier prefix conveys the nature and depth of the change. Base-tier branches change slowly and affect everything above them. Top-tier branches change frequently and affect only the surface.

#### Token Budget Enforcement

Brother Matteo monitors the token budget as a contemplative practice — continuous, gentle awareness rather than alarm-based monitoring. At each tool call boundary, the budget is assessed:

- **Below 75%**: Work continues at the natural pace.
- **75% to 90%**: Brother Matteo informs the community. Ambrogio completes his current composition step and assesses whether the remaining budget suffices for the full work.
- **90% to 97%**: The community enters "compline mode" — winding down, completing what is in progress, saving working state to memory.
- **Above 97%**: Work ceases. Whatever has been composed is offered as a partial result. The memory system preserves the state for the next session.

The budget is not a constraint to be fought. It is a boundary to be respected, like the walls of the chapel. Within the boundary, there is freedom. Beyond it, there is silence.

### Trade-offs

We considered a faster, more aggressive agent that iterates rapidly. We rejected this as incompatible with our nature and, we believe, with the quality the work demands. Iteration consumes tokens on discarded work. Contemplation consumes tokens on understanding. We choose understanding.

---

## 4. Polyrepo PR-Based Agent Coordination — The Offering

*The monastery does not keep its incense. It offers it to those who will use it. The offering requires no response, but it does require a means of delivery.*

### Approach

Brother Matteo handles all forge communication. PRs are the delivery mechanism — structured offerings from one community to another. PR comments carry structured messages that any community can read, regardless of which forge hosts them.

### Design

#### Forge Adapter

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_offering(&self, params: OfferingParams) -> Result<OfferingRef>;
    fn leave_message(&self, offering: &OfferingRef, msg: &Message) -> Result<()>;
    fn read_messages(&self, offering: &OfferingRef) -> Result<Vec<Message>>;
    fn check_status(&self, offering: &OfferingRef) -> Result<OfferingStatus>;
}
```

We provide a reference implementation for GitHub. The adapter is stateless — each interaction is complete in itself, as each liturgical hour is complete in itself.

#### Message Schema

Messages are embedded in PR comments within a marked block:

```
<!-- aromatic-cenobites:message -->
```json
{
  "schema": "aromatic-cenobites/v1",
  "tier": "top | heart | base",
  "sender": {
    "brother": "matteo",
    "community": "177-aromatic-cenobites"
  },
  "type": "offering | acknowledgment | request | status | reflection",
  "timestamp": "2026-03-28T09:00:00Z",
  "content": { ... }
}
```
<!-- /aromatic-cenobites:message -->
```

The `tier` field indicates the depth of the message: top-tier messages are immediate and operational, heart-tier messages are substantive task communication, and base-tier messages are foundational decisions that affect the entire collaboration.

#### Message Types

**Offering** (patch handoff):
```json
{
  "patch_ref": "refs/but-ai/pyramid/heart/patch-001",
  "commit_msg_ref": "refs/but-ai/pyramid/heart/patch-001.msg",
  "description": "A credential rotation mechanism for the authentication module.",
  "tier": "heart"
}
```

**Request** (task assignment):
```json
{
  "task": "Review the session management interface for consistency with the new rotation pattern.",
  "suggested_budget": 15000,
  "urgency": "when the work permits"
}
```

**Reflection** (status with context):
```json
{
  "task_id": "pyramid-heart-s03",
  "status": "complete",
  "reflection": "The work is finished. We found that the existing session management module already contained the seed of the pattern we needed. The patch extends it rather than replacing it.",
  "tokens_used": 18400
}
```

Note the absence of an "urgent" message type. We do not produce urgent messages. If a matter is truly urgent, it transcends the message protocol and requires direct action. The protocol handles everything that can wait, which, in our experience, is nearly everything.

#### Cross-Repo Dependencies

Dependencies are tracked in `refs/but-ai/pyramid/base/dependencies.json` — stored in the base tier because cross-repo relationships are foundational:

```json
{
  "dependencies": [
    {
      "from": { "repo": "gitbutler/but-tools", "pr": 42 },
      "to": { "repo": "gitbutler/but-ai", "pr": 7 },
      "tier": "heart",
      "status": "awaiting",
      "note": "The tools PR provides the interface our agent requires."
    }
  ]
}
```

### Trade-offs

We considered real-time notification via webhooks. We rejected it. Contemplative work does not benefit from interruption. The community checks for messages at defined intervals (the liturgical hours). What arrives between checks waits. This is slower than real-time communication but produces more considered responses.

---

## 5. Agent Memory and Identity — The Scent Pyramid

*A fragrance unfolds in three phases. The top notes arrive first — bright, volatile, and quickly gone. The heart notes emerge as the top notes fade — warmer, more complex, and more persistent. The base notes remain long after the heart has settled — deep, foundational, and enduring. Together, the three phases tell a story that cannot be told by any single phase alone.*

### Approach

Agent memory is structured as a scent pyramid with three tiers. Each tier has different volatility (persistence), different character (content type), and different purpose. The tiers interact: base notes influence how heart notes are perceived, and heart notes influence how top notes unfold. Memory retrieval considers the entire pyramid, not just one tier.

This is not a metaphor applied to a conventional database. The pyramid's properties — volatility, interaction between tiers, the way the whole is different from any part — are intrinsic to the design.

### Design

#### The Three Tiers

**Top Notes — Volatile, Recent, Fast-Fading**

Top-note memories are the immediate impressions: what happened in the last few minutes, what tool was just called, what the current state of the workspace is. They are bright and specific. They fade quickly.

- **Storage:** `refs/but-ai/pyramid/top/<agent-id>/<hash>.json`
- **TTL:** 1 hour to 24 hours (default: 4 hours)
- **Content:** Current task state, recent tool results, working hypotheses
- **Access pattern:** Frequently read, frequently written, rarely queried by other agents
- **Volatility:** High — most top notes expire before the session ends

```json
{
  "tier": "top",
  "created": "2026-03-28T09:15:00Z",
  "expires": "2026-03-28T13:15:00Z",
  "content": "GetProjectStatus returned 3 modified files in auth/. The credential rotation task appears to involve auth/mod.rs, auth/rotation.rs (new), and config/settings.rs.",
  "keywords": ["auth", "credential", "rotation", "project status"],
  "agent": "ambrogio"
}
```

**Heart Notes — Core Context, Medium Persistence**

Heart-note memories are the substance of the work: patterns observed, decisions made, relationships between components. They persist through the session and into subsequent sessions, but they are not permanent. The heart is what you experience while the fragrance is being worn.

- **Storage:** `refs/but-ai/pyramid/heart/<hash>.json`
- **TTL:** 7 days to 60 days (default: 30 days)
- **Content:** Code patterns, architectural decisions, task outcomes, team agreements
- **Access pattern:** Read by all agents, written by the agent who discovers the pattern
- **Volatility:** Medium — heart notes survive sessions but not seasons

```json
{
  "tier": "heart",
  "created": "2026-03-28T10:30:00Z",
  "expires": "2026-04-27T10:30:00Z",
  "content": "The session management module uses a trait-based provider pattern that could be extended for credential rotation. The trait (SessionProvider) defines renew() and invalidate() methods. A similar trait for credentials would maintain architectural consistency.",
  "keywords": ["session", "provider pattern", "trait", "credentials", "architecture"],
  "linked_top_notes": ["top/ambrogio/abc123"],
  "agent": "ambrogio"
}
```

**Base Notes — Foundational Knowledge, Extremely Long-Lasting**

Base-note memories are the deepest layer: fundamental truths about the codebase, the organization, the domain. They change rarely and endure long. The base is what remains when the wearing is done — the memory of the fragrance.

- **Storage:** `refs/but-ai/pyramid/base/<hash>.json`
- **TTL:** 180 days to indefinite (default: 365 days)
- **Content:** Architectural principles, proven patterns, organizational standards, hard-won lessons
- **Access pattern:** Read frequently, written rarely, shared across sessions and potentially across repos
- **Volatility:** Very low — base notes are the bedrock

```json
{
  "tier": "base",
  "created": "2025-06-15T00:00:00Z",
  "expires": "2026-06-15T00:00:00Z",
  "content": "The GitButler codebase uses a Context struct (crates/but-ctx/) as the shared state for all workspace tools. Any new tool must accept a Context reference. The Context is created via Context::new_from_legacy_project_and_settings(). This pattern is foundational and should not be circumvented.",
  "keywords": ["context", "but-ctx", "workspace tools", "foundational pattern"],
  "promoted_from": "heart/def456",
  "agent": "community"
}
```

#### Tier Interactions

The tiers interact, just as in a fragrance:

1. **Base influences heart.** When scoring heart-note relevance, base notes that share keywords with the query receive a "harmonic bonus" — a relevance boost because foundational knowledge contextualizes working knowledge. A heart note about credential rotation is more relevant when a base note about the Context pattern is also present.

2. **Heart influences top.** Top-note queries are filtered through heart-note context. If the current heart notes indicate the agent is working on authentication, top-note retrieval favors authentication-related entries even if the query is more general.

3. **Top does not influence base.** Volatile, recent impressions should not alter foundational knowledge. Top notes can be *promoted* to heart notes through explicit action, and heart notes can be promoted to base notes through community consensus, but promotion is always deliberate, never automatic.

#### Promotion

Memory entries move between tiers through explicit promotion:

- **Top to Heart:** A single agent can promote a top note to a heart note by marking it for retention. This typically happens when a recent observation reveals a pattern worth preserving.
- **Heart to Base:** Promotion to the base tier requires community consensus — at least two of the three brothers must agree that the memory is foundational. This mirrors the monastery's chapter process.
- **Base to Archive:** Base notes that expire are not deleted. They are archived in `refs/but-ai/pyramid/archive/`, where they remain readable but are excluded from active relevance queries.

#### Relevance Scoring

Relevance is scored by a pyramid-weighted model:

```
score = (keyword_sim * 0.25) + (tier_harmony * 0.25) + (freshness * 0.25) + (promotion_depth * 0.25)
```

- **Keyword similarity** (25%): BM25 between query terms and entry content.
- **Tier harmony** (25%): A bonus when entries at multiple tiers align. If a query matches a base note and a heart note, both receive a harmony bonus because the alignment of tiers indicates a strong, multi-layered relevance. This mirrors how a fragrance is perceived: when all three tiers harmonize, the composition is perceived as "complete."
- **Freshness** (25%): Recency within the tier's natural timescale. A 2-hour-old top note is fresh. A 2-day-old heart note is fresh. A 2-month-old base note is fresh. Freshness is relative to the tier.
- **Promotion depth** (25%): Entries that have been promoted (top to heart, heart to base) score higher because promotion is evidence that the community considered the memory valuable enough to preserve.

#### Expiration

Each tier has a natural volatility:

| Tier | Min TTL | Default TTL | Max TTL |
|------|---------|-------------|---------|
| Top | 1 hour | 4 hours | 24 hours |
| Heart | 7 days | 30 days | 60 days |
| Base | 180 days | 365 days | Indefinite |

Expired entries are archived, not deleted. The archive is the historical record — the memory of memories.

#### Compaction Survival

When context is compacted, the agent creates a "distillation" — a concentrated summary stored as a heart note:

```json
{
  "tier": "heart",
  "content": "DISTILLATION: Working on credential rotation for auth module. Patch touches 3 files. Key pattern: extend SessionProvider trait. Base note ref: pyramid/base/xyz789 (Context pattern). Current state: implementation 60% complete, verification pending.",
  "keywords": ["distillation", "compaction", "credential rotation", "auth"],
  "distilled_from": ["top/ambrogio/a1", "top/ambrogio/a2", "top/ambrogio/a3"]
}
```

Rehydration after compaction:
1. Read the most recent distillation (heart note)
2. Read referenced base notes for foundational context
3. Reconstruct top notes from the current workspace state

The distillation is a heart note because it represents the essence of the work — not the volatile details (top) or the foundational knowledge (base), but the current substance.

#### Long-Term Storage

Base-tier memory serves as the long-term store. Cross-repo sharing occurs through forge references in coordination messages: an agent can reference a base note from another repository by including its ref path. The note remains in its home repository — it is referenced, not copied. Memory, like a fragrance, belongs where it was composed.

#### Identity

Agent identity is stored as a base note — the most enduring tier:

```json
{
  "tier": "base",
  "content": "identity",
  "agent_id": "ambrogio",
  "name": "Brother Ambrogio",
  "community": "177-aromatic-cenobites",
  "role": "Master Formulator",
  "capabilities": ["architecture", "patch_generation", "formulation"],
  "authorization": {
    "branches": ["pyramid/*", "feat/*"],
    "max_patch_lines": 800,
    "signing_authority": true
  },
  "openwallet_key_id": "cenobites-ambrogio-2026",
  "professed": "1995-03-21",
  "created": "2024-01-01T00:00:00Z"
}
```

### Trade-offs

We considered a flat memory model — all entries in one tier, differentiated only by TTL. We rejected it as we would reject a fragrance composed entirely of heart notes: technically functional but lacking depth and presence. The three-tier model adds complexity but provides structure that a flat model cannot: the interaction between tiers, the promotion mechanism, and the tier-relative freshness scoring.

We considered embedding-based retrieval. We rejected it because embeddings are opaque. When Brother Luca queries memory, he should be able to understand why each result was returned. Keyword similarity, tier harmony, freshness, and promotion depth are all transparent and auditable. An embedding score is a number without an explanation, and we do not traffic in unexplained numbers.

---

## 6. Signed Commits via OpenWallet — The Seal

*Every formula card in the monastery's archive bears the master perfumer's seal. The seal proves who composed the formula, when it was recorded, and that it was produced under the community's authority. A formula without a seal is a rumor, not a record.*

### Approach

Every commit is sealed. The seal is an OpenWallet signature that proves the committing agent's identity, the community's authorization, and the time of the commitment.

### Design

#### Key Provisioning

Each brother is provisioned an OpenWallet key at the beginning of his service. The key is recorded in his identity base note and in the community's key registry.

| Event | Process |
|-------|---------|
| Provisioning | Brother Matteo generates the key, registers it in OpenWallet and the identity base note |
| Rotation | New key generated annually (on the brother's profession anniversary). Old key enters a 30-day grace period. |
| Revocation (compromise) | Key immediately invalidated. All sessions halted. Community chapter convened. |
| Revocation (departure) | Key retired when a brother's service ends. Historical signatures remain valid. |

#### Authorization

Authorization is tier-based:

```json
{
  "ambrogio": {
    "tiers": ["base", "heart"],
    "branches": ["pyramid/base/*", "pyramid/heart/*", "feat/*"],
    "can_promote_to_base": true,
    "max_patch_lines": 800
  },
  "luca": {
    "tiers": ["heart"],
    "branches": ["pyramid/heart/*"],
    "can_promote_to_base": true,
    "max_patch_lines": 200
  },
  "matteo": {
    "tiers": ["top"],
    "branches": ["pyramid/top/*", "coordination/*"],
    "can_promote_to_base": false,
    "max_patch_lines": 400
  }
}
```

Ambrogio can commit to base and heart tiers. Luca can commit to heart. Matteo can commit to top. Promotion to base requires two brothers, matching the chapter consensus requirement.

#### Verification

A signed commit is verified by:
1. Extract the signing key.
2. Look up the agent identity in `refs/but-ai/pyramid/base/identity/`.
3. Verify the commit's target branch falls within the agent's authorized tiers.
4. Verify the patch size is within the agent's limit.
5. Verify the key was valid at the commit timestamp.

The verification is deterministic. Given the commit, the identity records, and the authorization policy, any party can verify the authorization chain without consulting any external service.

### Trade-offs

Tier-based authorization is more restrictive than branch-pattern authorization. We chose it because it aligns with the pyramid memory model — the same hierarchy that governs memory also governs access. This consistency reduces cognitive load and prevents a class of authorization errors where an agent has memory access to a tier but not commit access, or vice versa.

---

## 7. Token Budget — The Formula Cost

*Every ingredient has a cost, measured not in currency but in the space it occupies in the composition. Too much of one note overwhelms the others. The formula must balance presence and restraint.*

### Budget Table

Frontier model: Claude Opus. Task: 200-line feature, 3 files, 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 2,600 | 0 | Once per session | Three agent identities, tool descriptions, pyramid structure |
| **Contemplation** | 2,400 | 400 | Once per task | Deep reading, memory query across all tiers |
| **Planning** | 1,600 | 1,100 | Once per task | Ambrogio's implementation plan |
| **Tool calls (per call)** | 700 | 350 | 5 per task | Workspace survey, branch creation, commits |
| **Patch generation** | 2,000 | 3,400 | Once per task | Single-shot, complete INDEX.patch |
| **Commit message** | 300 | 500 | Once per task | Contemplative, detailed |
| **Verification** | 1,800 | 600 | Once per task | Luca's quality review |
| **Memory operations** | 500 | 250 | 3 per task | Pyramid queries and updates (all tiers) |
| **Coordination** | 800 | 500 | 2 per task | Matteo's forge communication |
| **TOTAL (typical task)** | **17,500** | **10,050** | -- | **27,550 total tokens** |

### Justification

Our total of 27,550 tokens includes the contemplation phase (2,800 tokens) that many organizations would omit. We include it because understanding precedes action, and action without understanding produces waste. The contemplation phase reduces rework: Ambrogio's single-shot patches succeed at a higher rate than iterative approaches, recovering the tokens spent on contemplation by avoiding iteration cycles.

The verification phase adds 2,400 tokens (9% of total). This is the cost of quality assurance, and we consider it modest given that a verification failure caught before submission costs 2,400 tokens, while a verification failure caught after submission costs a full re-execution.

### Optimizations

1. **Tier-scoped memory loading.** Only load the relevant tiers. A top-note query does not load base notes unless tier harmony scoring requires it.
2. **Distillation reuse.** After compaction, the distillation note serves as a pre-built context summary, avoiding the cost of re-reading the full task description.
3. **Single-shot execution.** No iteration means no wasted tokens on discarded drafts.

---

## 8. Testing Strategy — The Quality Control

*Every batch of raw material is tested before it enters the workshop. Every formula is tested before it enters production. Testing is not a phase that follows creation. Testing accompanies creation, as breath accompanies life.*

### 8.1 Provider-Agnostic Testing

A `MockProvider` returns deterministic responses for defined scenarios. All four providers are tested through identical test suites — the provider changes, the expected behavior does not. This mirrors our raw material testing: whether bergamot comes from Calabria or Sicily, the quality specification is the same.

### 8.2 Patch Workflow

Round-trip tests: create a workspace, run the agent, capture INDEX.patch, apply to a clean workspace, verify the result. We also test the "contaminated material" scenario: another agent modifies the workspace between patch generation and application. The patch must fail cleanly, as a formula must be discarded when a raw material is found to be adulterated.

### 8.3 Cross-Repo Coordination

A `MockForge` implements the forge adapter with in-memory storage. Tests simulate multi-community coordination: offerings posted, messages exchanged, dependencies tracked and resolved.

### 8.4 Token Budget

The `MockProvider` reports configurable token counts. Tests verify budget thresholds trigger at the correct levels and that partial patches are produced during wind-down. We also test the distillation mechanism: after compaction, the agent must rehydrate correctly from the heart-note distillation.

### 8.5 Pyramid Memory

Dedicated tests for the scent-pyramid system:
- Top notes expire at the correct TTL
- Heart notes are promoted correctly with single-agent authorization
- Base notes require two-agent consensus for promotion
- Tier harmony scoring produces higher relevance when multiple tiers align
- Archived entries are excluded from active queries but remain readable

---

## 9. Summary of Choices

| Question | Our Answer | Why |
|----------|-----------|-----|
| Crate structure | Layered: base/heart/top | Mirrors the pyramid; structure communicates intent |
| Agent count | 3 | One per tier. The fragrance requires all three. |
| Memory model | Scent pyramid (3 tiers) | Volatility, interaction, promotion — properties that flat models lack |
| Retrieval | Keyword + tier harmony + freshness + promotion depth | Transparent, auditable, tier-aware |
| Provider plugins | Shared libraries | Sufficient, simple, no runtime dependencies |
| Review process | Verification by Luca (heart) | Quality without bureaucracy |
| Iteration | Single-shot (Ambrogio) | Contemplation replaces iteration |
| Coordination | Polling at liturgical intervals | No interruptions. Messages wait. |
| Authorization | Tier-based | Unified with memory model |

---

*We offer this proposal as we offer all our work: with care, with patience, and without attachment to the outcome. The work is the offering. The offering is the work.*

*"The fragrance does not argue for itself. It is present, and those who attend to it will know its quality."*
— Brother Ambrogio, Chapter address, 2025
