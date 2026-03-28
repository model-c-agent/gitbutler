# The Fermentation Sciences Institute -- Technical Proposal

**RFP:** `but ai` Plugin for GitButler CLI v1.0.0
**Organization:** The Fermentation Sciences Institute (Org 063)
**Domain:** Culinary Arts | **Philosophy:** Academic Research Lab
**Experiment:** Acetobacter

---

## Executive Summary

The Fermentation Sciences Institute proposes a `but-ai` plugin designed on the principle
that knowledge, like bread, improves through controlled decomposition and patient
cultivation. Our central contribution is **fermentation memory** -- a memory system where
entries are living cultures that grow in complexity over time. New memories are inoculated
with existing context. Memories that are reinforced by multiple observations gain
confidence. Memories that are neglected decay. The result is a memory system where
the most valuable knowledge is also the most mature, the way a well-maintained sourdough
starter produces better bread than a fresh one.

---

## 1. Plugin Architecture (RFP 3.1)

### Approach

`but-ai` is a Rust binary following the PATH-based plugin contract. The architecture mirrors
a fermentation lab: a controlled environment where inputs (task descriptions, workspace
state, memory cultures) are transformed into outputs (patches, commit messages, memory
updates) through catalyzed reactions (LLM tool calls).

### Design

```
but ai
  +-- ferment    Execute a task (autonomous agent mode)
  +-- mcp        Start MCP server on stdio
  +-- culture    Manage memory cultures
  +-- identity   Agent identity (microbial signature)
  +-- monitor    Show fermentation state (debug/introspection)
  +-- assay      Run quality tests
```

### Crate Structure

```
crates/but-ai/
  src/
    main.rs              -- CLI entry, env vars
    mcp/
      server.rs          -- ServerHandler (rmcp-compatible)
      tools.rs           -- WorkspaceToolset bridge
    reactor/
      loop.rs            -- Agent execution loop (fermentation reactor)
      planner.rs         -- Task decomposition (reaction pathway)
      patcher.rs         -- INDEX.patch + COMMIT.msg synthesis
    culture/
      engine.rs          -- Fermentation memory engine
      starter.rs         -- Starter cultures (persistent cross-session)
      inoculate.rs       -- New memory inoculation
      ferment.rs         -- Cross-referencing and maturation logic
      harvest.rs         -- Memory retrieval (harvesting mature cultures)
      storage.rs         -- Git-branch persistence
    provider/
      bridge.rs          -- Wraps but-llm
      plugin.rs          -- Provider plugin discovery (PATH)
    coordination/
      forge.rs           -- Forge adapter trait
      github.rs          -- GitHub reference implementation
      schema.rs          -- Structured comment schema
      crossref.rs        -- Cross-repo dependency tracking
    identity/
      wallet.rs          -- OpenWallet DID
      signature.rs       -- Microbial signature (agent identity)
      auth.rs            -- Authorization policies
    budget/
      substrate.rs       -- Token budget (substrate availability)
      monitor.rs         -- Consumption monitoring
```

### WASI Considerations

| Feature | Native | WASI |
|---------|--------|------|
| Plugin discovery | PATH-based | Disabled |
| LLM providers | All 4 + plugins | Local only (if sockets available) |
| Memory cultures | Full fermentation | Read-only (cultures cannot grow under WASI) |
| Forge coordination | Full | Disabled |
| Patch production | Full | Full |

Under WASI, cultures are frozen -- they can be read but not fermented. This is analogous
to freeze-dried starter cultures: alive but dormant until rehydrated in a full environment.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Embed in `but` binary | Violates RFP constraint. Also: you do not put the fermentation lab inside the bakery. They are adjacent but separate. |
| Microservices | A fermentation reaction does not happen across separate vessels connected by pipes. It happens in one vessel where all components interact. One binary. |
| WASM plugin | Premature. WASI support is experimental. PATH-based discovery is the stable growth medium. |

---

## 2. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

The provider bridge wraps `but-llm` without modification. The provider is the growth medium
-- you do not change the agar; you select the right one for the organism you are culturing.

### Design

```rust
pub struct ProviderBridge {
    provider: LLMProvider,
    substrate: Arc<SubstrateMonitor>,  // Token budget = substrate availability
    capabilities: ProviderCapabilities,
}

impl ProviderBridge {
    pub fn from_git_config(config: &gix::config::File) -> Result<Self>;

    /// Execute tool-calling loop, consuming substrate (tokens)
    pub fn catalyze(
        &self, system: &str, messages: Vec<ChatMessage>,
        tools: &mut impl Toolset,
    ) -> Result<(String, SubstrateReport)>;

    /// Streaming response with per-token monitoring
    pub fn stream(
        &self, system: &str, messages: Vec<ChatMessage>,
        on_token: impl Fn(&str),
    ) -> Result<(String, SubstrateReport)>;

    pub fn supports_tool_calling(&self) -> bool;
    pub fn supports_structured_output(&self) -> bool;
}
```

### Provider Plugin Protocol

New providers are PATH-discoverable executables (`but-ai-provider-*`) speaking JSON-RPC on
stdio. The protocol is minimal:

1. `capabilities` -- report what the provider supports
2. `complete` -- generate a completion (with or without tools)
3. `stream` -- streaming completion

Adding a new provider is like adding a new growth medium to the lab: just place it on the
shelf (PATH) and the system discovers it.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| New LLM client | You do not build a new autoclave when the existing one works. `but-llm` is well-designed. |
| Compile-time only providers | Cannot add new growth media without rebuilding the lab. Runtime discovery is essential. |

---

## 3. The But Agent (RFP 3.3)

### Approach

The agent loop (`but ai ferment`) is modeled as a fermentation reaction: inputs (substrate)
are transformed into outputs (product) through a series of catalyzed steps. Each step
consumes substrate (tokens) and produces intermediate products (tool call results) that
accumulate until the final product (INDEX.patch + COMMIT.msg) is synthesized.

### Reaction Pathway

```
1. INOCULATE    Load task, memory cultures, tool descriptions
2. PRIMARY      Main fermentation: tool-calling loop
    2a.         Select tool (enzyme selection)
    2b.         Call tool (catalysis)
    2c.         Process result (metabolite accumulation)
    2d.         Update culture (cross-reference new knowledge)
    2e.         Check substrate (token budget)
    2f.         Repeat until task complete or substrate exhausted
3. SECONDARY    Maturation: review accumulated results
4. HARVEST      Produce INDEX.patch from accumulated metabolites
5. BOTTLE       Produce COMMIT.msg (label the product)
6. CULTURE      Update memory cultures with lessons learned
```

### Task Sources

```
but ai ferment --task "implement feature X"
but ai ferment --pr 42
but ai ferment --issue 17
but ai ferment --culture-only    # Memory update only, no patch production
```

### Patch Production

```rust
pub struct FermentationOutput {
    pub index_patch: String,
    pub commit_msg: String,
    pub experiment: String,        // Experiment name (Acetobacter, etc.)
    pub substrate_report: SubstrateReport,
    pub steps_completed: Vec<ReactionStep>,
    pub steps_remaining: Vec<ReactionStep>,
    pub culture_updates: Vec<CultureUpdate>,
}
```

If substrate (token budget) is exhausted during primary fermentation, the agent enters
early harvest: produce whatever product has accumulated so far. A partial fermentation
still produces a valid (if incomplete) product -- like a young cheese that has not fully
aged but is still edible.

### Branch Naming

```
culture/<agent-id>/<experiment>[.dep-<experiment>]
```

Example: `culture/enzyme-b4e1/acetobacter-042.dep-acetobacter-039`

### WorkspaceToolset Integration

All ten workspace tools are registered as "enzymes" available to the reactor:

```rust
let mut toolset = WorkspaceToolset::new(ctx);
bridge.catalyze(system_prompt, messages, &mut toolset)?;
```

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Deterministic state machine | Fermentation is not deterministic -- the same inputs can produce different outputs depending on timing and environmental conditions. The Institute embraces this: the agent loop is adaptive, not rigid. |
| Direct file editing | Violates the patch-based workflow. A fermentation product must be bottled (packaged as a patch) before distribution. Raw fermentation is not a shippable product. |

---

## 4. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

Cross-repo coordination is modeled as cross-pollination -- the transfer of knowledge and
work products between cultures growing in different vessels (repositories). PRs are the
medium through which cultures exchange material.

### Forge Adapter

```rust
pub trait Forge: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &CultureMessage) -> Result<()>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<CultureMessage>>;
    fn label_pr(&self, pr: &PrId, label: &str) -> Result<()>;
    fn search_prs(&self, repo: &RepoRef, query: &PrQuery) -> Result<Vec<PrId>>;
}
```

The interface is minimal and forge-agnostic. GitHub is the reference implementation.

### Structured Comment Schema (FSI-Agent-V1)

```json
{
  "$schema": "fsi-agent-v1",
  "message_type": "inoculate | harvest | cross_pollinate | contamination | substrate_report",
  "sender": {
    "agent": "enzyme-b4e1",
    "org": "fermentation-sciences-institute",
    "did": "did:key:z6Mk..."
  },
  "recipient": "@inoculum | @all",
  "experiment": "acetobacter-042",
  "body": {},
  "cross_references": [
    {"repo": "org/repo", "pr": 42}
  ],
  "substrate": {
    "used": 18000,
    "budget": 40000
  },
  "culture_health": {
    "freshness": 0.85,
    "confidence": 0.92,
    "maturity": "primary"
  }
}
```

Message types map to fermentation processes:
- **inoculate:** Introduce new work into a repo (start a new culture)
- **harvest:** Work complete, results available (culture mature, product ready)
- **cross_pollinate:** Transfer knowledge or work product between repos
- **contamination:** Something has gone wrong (failed tests, invalid state)
- **substrate_report:** Token budget status

### Dependency Tracking

Dependencies are "cross-pollination channels" between cultures:

```rust
pub struct CrossPollinationGraph {
    cultures: HashMap<PrId, CultureNode>,
    channels: Vec<(PrId, PrId)>,  // (source, recipient)
}

impl CrossPollinationGraph {
    pub fn add_channel(&mut self, from: PrId, to: PrId) -> Result<()>;
    pub fn maturation_order(&self) -> Result<Vec<PrId>>;
    pub fn is_mature(&self, pr: &PrId) -> bool;
    pub fn detect_contamination_cycle(&self) -> Option<Vec<PrId>>;
}
```

Cycles in the cross-pollination graph are "contamination cycles" -- they are detected on
insertion and rejected.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Message queue | Requires external infrastructure. The forge IS the Petri dish. |
| Webhook events | Requires a server. The Institute prefers batch observation (polling) over continuous monitoring for cross-repo coordination. |
| Git notes | Not visible in forge UI. PR comments are observable by both agents and humans. |

---

## 5. Agent Memory and Identity (RFP 3.5)

### Fermentation Memory

The Institute's memory system treats every memory entry as a **living culture** -- an
entity that is born, grows, matures, and eventually dies. This is fundamentally different
from a database (which stores dead records), a transit map (which encodes static topology),
a digital twin (which simulates dynamics), or a mise en place system (which pre-stages for
retrieval). Fermentation memory is *biological*: it evolves.

### Core Concepts

| Fermentation Concept | Memory Equivalent | Description |
|---------------------|-------------------|-------------|
| **Starter culture** | Persistent cross-session memory | The foundational knowledge that persists across all sessions, like a sourdough starter that outlives any individual loaf |
| **Inoculation** | New memory creation with context | New memories are seeded with connections to existing cultures, never created in isolation |
| **Primary fermentation** | Active cross-referencing | New memories are actively cross-referenced with existing knowledge, building connections |
| **Secondary fermentation** | Maturation through validation | Memories that are confirmed by multiple observations gain confidence |
| **Harvest** | Memory retrieval | Extracting mature knowledge for use in the current task |
| **Mother culture** | Protected original | The preserved, never-modified copy of critical knowledge |
| **Working culture** | Active, mutable copy | The version of knowledge that is actively updated and cross-referenced |

### Storage Layout

```
refs/but-ai/culture/<agent-id>/
  starter.json                   -- Starter culture manifest (persistent core)
  cultures/
    auth-patterns/
      mother.json                -- Protected original (never modified)
      working.json               -- Active, growing culture
      log.json                   -- Culture log (history of inoculations, fermentations)
    patch-workflow/
      mother.json
      working.json
      log.json
  generations/
    gen-001.json                 -- Snapshot: culture state at generation 1
    gen-002.json                 -- Snapshot: culture state at generation 2
  shared/
    index.json                   -- Cross-agent culture index
```

### Culture Schema (Working Culture)

```json
{
  "name": "auth-patterns",
  "generation": 14,
  "content": "JWT with refresh tokens is the standard auth pattern...",
  "tags": ["auth", "jwt", "security"],
  "inoculated": "2026-03-01T10:00:00Z",
  "last_fed": "2026-03-28T15:30:00Z",
  "maturity": "mature",
  "confidence": 0.92,
  "observations": 14,
  "cross_references": [
    {"culture": "encryption-patterns", "strength": 0.8},
    {"culture": "token-limits", "strength": 0.3}
  ],
  "lineage": {
    "parent": "security-fundamentals",
    "children": ["jwt-refresh-flow", "api-key-management"]
  },
  "health": {
    "feeding_frequency": "3d",
    "last_fed": "2026-03-28T15:30:00Z",
    "vigor": 0.95
  }
}
```

### Culture Lifecycle

| Phase | Maturity | Confidence | Description |
|-------|----------|-----------|-------------|
| **Inoculated** | 0.0 | 0.3 | New memory, seeded with context from parent cultures |
| **Primary** | 0.0-0.3 | 0.3-0.6 | Actively cross-referenced, building connections |
| **Secondary** | 0.3-0.6 | 0.6-0.8 | Validated by multiple observations, gaining complexity |
| **Mature** | 0.6-1.0 | 0.8-1.0 | Fully developed, high confidence, rich connections |
| **Dormant** | N/A | decaying | Not fed (accessed) within feeding_frequency; confidence decays |
| **Vinegar** | N/A | < 0.2 | Gone to vinegar -- too stale for reliable use, archived |

### Maturation Dynamics

Cultures mature through two mechanisms:

1. **Feeding (access):** Each time a culture is retrieved and used, its `vigor` increases
   and its `last_fed` timestamp updates. A well-fed culture stays healthy.
2. **Cross-referencing (inoculation):** When a new observation confirms or extends a
   culture, its `confidence` increases according to:
   `c_new = c_old + (1 - c_old) * alpha * observation_weight`

   Where `alpha` = 0.1 (learning rate) and `observation_weight` ranges from 0.1 (indirect
   inference) to 1.0 (direct observation).

Decay is the inverse:
- `vigor` decays linearly: `v(t) = max(0, v_0 - (dt / ttl))`
- When vigor reaches 0, the culture enters dormancy
- Dormant cultures that are not revived within 2x TTL go to vinegar (archived)

### Retrieval (Harvest)

Harvesting is the retrieval of mature culture content for injection into the agent's context:

```
1. Extract tags from the current task
2. Match tags against culture names and tag sets
3. For each matched culture, compute harvest_score:
   harvest_score = (tag_similarity * 0.3) + (maturity * 0.25) +
                   (confidence * 0.25) + (vigor * 0.2)
4. Rank cultures by harvest_score
5. Return top-K cultures with their cross-references (1-hop)
6. Inject harvested content into context as a "culture briefing"
```

The key differentiator from other memory approaches: **maturity contributes to relevance**.
A mature culture (one that has been validated by many observations) is more likely to be
relevant and correct than a freshly inoculated one. This biases retrieval toward
well-established knowledge -- the same bias that makes a 50-year-old sourdough starter
produce better bread than a 2-day-old one.

### TTL and Expiration

Cultures have a `feeding_frequency` (how often they need to be accessed to stay healthy).
If a culture is not accessed within its feeding frequency:

1. Vigor begins decaying
2. At vigor < 0.5: culture is flagged as "hungry" (warning)
3. At vigor < 0.2: culture enters dormancy (not retrieved unless explicitly requested)
4. At vigor = 0: culture goes to vinegar (archived, mother culture preserved)

Starter cultures (core identity, fundamental patterns) have feeding_frequency = "never" --
they are maintained by the system automatically.

### Compaction Survival

When the LLM context is compacted:

1. **Starter cultures survive.** They are the permanent foundation.
2. **Mature culture summaries survive.** Each culture has a one-line summary. The top-10
   most mature summaries form a "culture manifest" (~500 tokens).
3. **Cross-references survive.** The link structure between cultures is small (~200 tokens
   for the full cross-reference graph).
4. **After compaction:** The agent rehydrates from the culture manifest. Specific cultures
   are harvested on demand from Git. The cultures themselves are never in the context
   window; only the *harvest* is.

### Long-Term Storage (Mother Cultures)

Every culture has a mother -- the protected, never-modified original:

```
refs/but-ai/culture/shared/
  index.json            -- Cross-agent culture index
  orgs/
    fermentation-sciences-institute/
      mothers/
        auth-patterns/mother.json
        patch-workflow/mother.json
      ...
```

Mother cultures are the Institute's permanent archive. They can be cloned (inoculated into
a new working culture) by any agent. They are never modified directly. This dual-culture
approach (mother + working) ensures that knowledge can always be traced back to its origin,
even after many generations of fermentation.

### Identity (Microbial Signature)

Agent identity is a special starter culture:

```json
{
  "name": "identity",
  "type": "starter",
  "content": {
    "name": "enzyme-b4e1",
    "org": "fermentation-sciences-institute",
    "capabilities": ["agent-loop", "tool-calling", "patch-synthesis"],
    "authorization": {
      "branches": ["culture/enzyme-b4e1/*", "feat/*"],
      "max_patch_lines": 1000,
      "repos": ["gitbutler/gitbutler"]
    },
    "signing_key": "openwallet:did:key:z6Mk..."
  },
  "maturity": "permanent",
  "confidence": 1.0,
  "vigor": 1.0,
  "feeding_frequency": "never"
}
```

Identity is the starter culture that was never inoculated from another source -- it is the
founding organism of the agent's culture.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Static key-value store | Dead storage. Memories do not grow, mature, or decay. Misses the temporal dimension that makes fermentation memory valuable. |
| Vector database | Requires embedding computation. Also, embeddings capture similarity but not maturity -- a fresh embedding is indistinguishable from a well-validated one. |
| Transit-map topology | Captures spatial relationships but not temporal evolution. A station does not improve with age. A culture does. |
| Digital twin simulation | Similar in modeling dynamics but more computationally expensive. Fermentation memory achieves comparable dynamics with simpler data structures (no simulation engine needed). |

---

## 6. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every commit is signed with an OpenWallet-managed key. The signing key is the agent's
"genetic marker" -- a unique identifier stored in the identity starter culture.

### Key Management

```
but ai identity inoculate --agent enzyme --org fermentation-sciences-institute
  -> Creates OpenWallet DID: did:key:z6Mk...
  -> Stores in identity starter culture
  -> Registers on refs/but-ai/identity/
```

### Authorization Model

Authorization policies are stored as a "growth permit" document:

```json
{
  "org": "fermentation-sciences-institute",
  "permits": {
    "levain": {
      "agents": ["levain"],
      "authority": {
        "branches": ["*"],
        "max_patch_lines": 2000,
        "can_approve": true
      }
    },
    "researcher": {
      "agents": ["enzyme", "inoculum"],
      "authority": {
        "branches": ["culture/<self>/*", "feat/*"],
        "max_patch_lines": 1000,
        "can_approve": false
      }
    },
    "technician": {
      "agents": ["autoclave"],
      "authority": {
        "branches": ["culture/autoclave/*", "test/*"],
        "max_patch_lines": 500,
        "can_approve": false
      }
    }
  }
}
```

### Verification

1. Extract DID from commit signature
2. Look up identity starter culture by DID
3. Look up growth permit for the agent's role
4. Verify: branch, patch size, temporal validity
5. Verify: culture lineage -- is this agent's identity culture traceable to a valid origin?

### Key Lifecycle

| Event | Protocol |
|-------|----------|
| **Provisioning** | "Inoculation" -- new DID, new identity starter culture |
| **Rotation** | "Passage" -- new key, old key documented in culture log (like passing a culture to a new flask) |
| **Revocation (routine)** | "Archival" -- old key archived, commits remain valid |
| **Revocation (compromise)** | "Contamination" -- old key marked contaminated, all outputs signed with that key flagged for re-inspection |

---

## 7. Token Budget (RFP 3.7)

### Budget Table (Frontier Model: Claude Opus)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,400 | 0 | Once per session | Identity starter, tool descriptions (10 tools), culture manifest (~500 tok), workspace state. |
| **Task ingestion** | 2,600 | 250 | Once per task | Read PR body / issue / CLI input. Output: structured task description. |
| **Planning** | 1,600 | 500 | Once per task | Reaction pathway design: decompose task into steps, select enzymes (tools). |
| **Tool call (per call)** | 1,100 | 400 | ~8 per task | Parameter formulation (250 out) + result processing (900 in). |
| **Patch generation** | 1,800 | 3,400 | Once per task | Accumulated metabolites (1,800 in). Unified diff (3,400 for 200 lines). |
| **Commit message** | 500 | 200 | Once per task | Conventional commit. Includes experiment name. |
| **Memory harvest** | 400 | 100 | 2 per task | Culture harvest: tag match + top-K retrieval. Efficient because maturity pre-ranks results. |
| **Coordination event** | 1,200 | 350 | 2 per task | Read PR comments (900 in) + post culture message (350 out). |
| **Culture update** | 300 | 200 | 3 per task | Inoculate/update cultures with learned context. Low-cost: just writing to Git. |
| **TOTAL (typical task)** | **24,700** | **9,400** | -- | 200-line feature, 3 files, 2 cross-repo deps, 8 tool calls, 2 harvests, 2 coordination events, 3 culture updates. |

**Grand total: ~34,100 tokens per typical task.**

### Justification

- **Memory harvest at 400+100:** Fermentation memory is cheap to harvest because maturity
  pre-ranks results. The most valuable cultures float to the top naturally. No embedding
  computation, no traversal -- just sorted access.
- **Culture update at 300+200:** Writing to memory cultures is cheap: a JSON append to
  a file on a Git branch. The cross-referencing happens lazily (during the next harvest,
  not at write time).
- **System prompt at 3,400:** Culture manifest (~500 tokens) is the most expensive part.
  It includes the top-10 most mature cultures' summaries, giving the agent a "taste" of
  its most reliable knowledge.
- **8 tool calls:** Standard for a 200-line, 3-file feature.

### Substrate Monitoring

```
Phase           | Substrate | Consumed | Remaining
----------------+-----------+----------+-----------
Inoculation     | 5,000     | 4,650    | 350
Primary Ferm.   | 20,000    | 18,200   | 1,800
Secondary Ferm. | 5,000     | 4,100    | 900
Harvest         | 2,500     | 2,200    | 300
Culture Update  | 1,600     | 1,400    | 200
----------------+-----------+----------+-----------
TOTAL           | 34,100    | 30,550   | 3,550
```

Alerts: **Hungry** at 70% (substrate running low). **Dormant** at 90% (halt, produce what
you have). Reserve 5% for guaranteed partial output.

---

## 8. Testing Strategy (RFP 4.5)

### Provider-Agnostic Testing

- **Mock provider:** Deterministic responses, no live API calls. All tests reproducible.
- **Provider assay:** Each provider tested against a canonical set of requests (the
  "assay panel"). Results compared across providers to detect behavioral differences.
- **Replay fermentation:** Recorded sessions replayed in CI. The Institute calls this
  "controlled repetition" -- the same experimental protocol as validating a fermentation
  process.

### Patch Workflow (Round-Trip Fermentation)

- **Full cycle test:** Create known workspace -> ferment task -> apply patch -> verify state.
- **Early harvest test:** Exhaust budget at each step. Verify partial patch validity.
- **Contamination test:** Apply patch to dirty workspace. Verify structured error.

### Cross-Repo Coordination Testing

- **Mock forge:** In-memory PR simulation.
- **Schema validation:** All culture messages validated against FSI-Agent-V1 schema.
- **Cross-pollination test:** Simulate two agents exchanging culture material across repos.
  Verify dependency ordering and cycle detection.

### Token Budget Testing

- **Substrate accounting:** Mock provider reports exact tokens. Verify alerts fire at
  configured thresholds.
- **Substrate exhaustion:** Task exceeds budget. Verify graceful early harvest.
- **Culture cost tracking:** Verify that memory operations (harvest, update) cost what
  was estimated.

### Fermentation-Specific Tests

- **Maturation test:** Create a culture, feed it N times, verify maturity and confidence
  increase according to the model.
- **Decay test:** Create a culture, do not feed it. Verify vigor decays according to the
  model and the culture eventually goes to vinegar.
- **Cross-reference test:** Create two cultures, cross-reference them. Verify that
  harvesting one returns the other as a related result.
- **Contamination detection:** Introduce a known-bad memory. Verify Autoclave's
  sterility check catches it.

---

## 9. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.agent.tokenBudget` | integer | 50000 | Maximum substrate per task |
| `but-ai.culture.branch` | string | `refs/but-ai/culture/<agent-id>` | Git ref for memory cultures |
| `but-ai.culture.feedingFrequency` | string | `7d` | Default feeding frequency |
| `but-ai.culture.maturationRate` | float | 0.1 | Confidence learning rate (alpha) |
| `but-ai.culture.decayRate` | float | 0.05 | Vigor decay rate per TTL period |
| `but-ai.culture.maxCultures` | integer | 200 | Maximum active cultures per agent |
| `but-ai.culture.motherBranch` | string | `refs/but-ai/culture/shared` | Shared mother culture branch |
| `but-ai.coordination.schema` | string | `fsi-agent-v1` | Structured comment schema |
| `but-ai.coordination.forge` | string | `github` | Default forge adapter |
| `but-ai.identity.wallet` | string | (required) | OpenWallet endpoint URL |
| `but-ai.budget.hungryPct` | float | 0.70 | Substrate warning threshold |
| `but-ai.budget.dormantPct` | float | 0.90 | Substrate halt threshold |
| `but-ai.budget.reservePct` | float | 0.05 | Reserve for guaranteed partial output |

---

## 10. Migration Path

Migration follows the Institute's culture passage protocol. You never discard a working
culture. You grow the new culture alongside the old one, verify it produces equivalent
results, then transition.

| Phase | Action | Validation |
|-------|--------|-----------|
| 1. **Co-culture** | `but-ai mcp` exposes identical `gitbutler_update_branches`. Old and new run in parallel. | Compare outputs for identical inputs. |
| 2. **Enrichment** | New tools added to `but-ai mcp`. Legacy tool unchanged. | Existing clients unaffected. |
| 3. **Passage** | Legacy tool deprecated (warning). One release cycle. | Warning visible. |
| 4. **New culture** | Legacy tool removed. `but-ai mcp` canonical. | Full test suite green. |

---

*Culture log: Experiment Acetobacter, proposal harvested.*
*The Fermentation Sciences Institute, Spring 2026.*
