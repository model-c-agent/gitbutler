# Proposal: `but-ai` Plugin -- The Longevity & Risk Research Centre

**Submitted by:** The Longevity & Risk Research Centre (Org 093)
**Domain:** Insurance Actuarial -- Mortality Modeling and Survival Analysis
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

The Longevity & Risk Research Centre proposes a `but-ai` plugin grounded in survival analysis -- the branch of statistics that studies how long things last. We observe that existing memory systems treat expiration as a binary event: a memory is either alive or dead, controlled by a fixed TTL. This is the equivalent of assuming every human dies at the same age. It is wrong for humans, and it is wrong for memories.

Our central innovation is **actuarial-table memory** -- a memory scheme where every entry has a fitted survival function that models its probability of remaining relevant over time. Different types of memories have different mortality patterns. Architectural memories follow Weibull distributions (slowly increasing hazard rate). Bug-related memories follow exponential distributions (instant irrelevance once fixed). Convention memories follow bathtub-shaped hazard functions (uncertain early, stable when established, unreliable as teams change). The system fits the right distribution to each memory, allowing graceful, probabilistic expiration instead of arbitrary TTL cutoffs.

The plugin is implemented in Rust using `but-llm` and `but-tools` without modification.

---

## 2. Plugin Architecture (RFP 3.1)

### Approach

The `but-ai` binary is a PATH-discovered Rust executable operating in CLI and MCP modes.

### Design

**Binary structure:**

```
but-ai
  ├── but ai study <task>          -- Execute a task (conduct a study)
  ├── but ai mortality              -- Show memory survival statistics
  ├── but ai mcp                    -- MCP server mode
  ├── but ai agent --task <desc>    -- Autonomous agent mode
  └── but ai surprise               -- Show surprise index (model-deviation detector)
```

The primary verb is `study`. Every task is a research study: the agent formulates a hypothesis (plan), gathers data (reads codebase), conducts the experiment (generates patches), and publishes results (INDEX.patch + COMMIT.msg). The `mortality` command displays the survival statistics of the agent's memory -- which memories are healthy, which are approaching their median survival time, which have hazard rates that suggest imminent irrelevance.

**Crate structure:**

```
crates/but-ai/
  src/
    lib.rs              -- Core library
    survival/
      distributions.rs  -- Survival distribution implementations (Weibull, exponential, bathtub)
      fitting.rs        -- Parameter estimation for survival functions
      hazard.rs         -- Hazard rate computation and monitoring
      surprise.rs       -- Surprise index (model-deviation detection)
    study/
      protocol.rs       -- Task execution protocol
      patch.rs          -- INDEX.patch + COMMIT.msg production
      validation.rs     -- Output validation
    cohort/
      memory.rs         -- Actuarial-table memory management
      identity.rs       -- Agent identity and lifecycle
      coordination.rs   -- Cross-repo coordination
  bin/
    main.rs             -- Binary entry point
```

**Environment variables:**

`BUT_WORKSPACE_DIR` maps to the study site. `BUT_OUTPUT_FORMAT` determines the reporting format: `human` is a formatted study report, `json` is structured data with confidence intervals, `shell` is summary statistics.

**WASI degradation:**

Under WASI, the plugin operates as an "in vitro study" -- the agent can analyze local data and generate patches but cannot conduct field work (no cross-repo coordination). Survival function computation remains fully operational. Memory management is unaffected. Only the coordination module is disabled. The agent's output includes a clear "in vitro" annotation indicating the reduced observational scope.

**MCP compatibility:**

Drop-in replacement for the existing MCP server. Server name: `"GitButler Actuarial Engine"`, version `"2.0.0"`. Full backward compatibility with `gitbutler_update_branches`. Additional tools expose memory survival statistics and surprise index readings.

### Trade-offs

**Alternative considered: Python implementation for better statistical library access.** Rejected for consistency with the Rust codebase. Survival distribution computations are not complex enough to require Python's scientific ecosystem -- they are closed-form expressions that Rust handles efficiently.

**Alternative considered: External statistical service.** Rejected per RFP (no external services). All survival computations run locally.

---

## 3. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

We use `but-llm` exclusively. The provider choice is a nuisance variable in our experimental design -- it must be controlled for but should not dominate the results.

### Design

**Provider abstraction:**

We define a `StudyProtocolAdapter` for new providers:

```rust
pub trait StudyProtocolAdapter: Send + Sync {
    fn protocol_name(&self) -> &str;
    fn supports_structured_output(&self) -> bool;
    fn supports_tool_calling(&self) -> bool;
    fn response_variance(&self) -> f64;  // estimated output variability
    fn context_capacity(&self) -> usize;
    fn configure(&self, config: &gix::config::File) -> Result<LLMProviderConfig>;
}
```

The `response_variance` field is unique to our design. It estimates how variable the provider's outputs are for the same input -- a quantity that matters when you are trying to produce reproducible patches. Providers with high variance are penalized in Vassiliev's study protocol because they introduce measurement noise.

New providers registered via Git config:

```ini
[but-ai "protocol.gemini"]
    adapter = "/path/to/gemini-protocol"
    estimated-variance = 0.15
```

**Tool exposure:**

All 10 `WorkspaceToolset` tools registered through the `Toolset` trait. Each tool is annotated with its "side effect profile" -- a structured description of what the tool changes and the reversibility of those changes:

| Tool | Reversible | Side Effect Scope |
|------|-----------|-------------------|
| GetProjectStatus | N/A (read-only) | None |
| GetBranchChanges | N/A (read-only) | None |
| GetCommitDetails | N/A (read-only) | None |
| CreateBranch | Yes | Branch namespace |
| MoveFileChanges | Yes (via inverse move) | Branch content |
| Commit | Partially (amend possible) | Branch history |
| Amend | No (rewrites history) | Branch history |
| SquashCommits | No (rewrites history) | Branch history |
| SplitBranch | Yes (via merge) | Branch topology |
| SplitCommit | No (rewrites history) | Branch history |

Irreversible tools require Okonkwo's practitioner review before execution. Reversible tools can be used freely. This risk-aware tool classification mirrors actuarial practice: you need to understand the loss given default, not just the probability of default.

### Trade-offs

**Alternative considered: Custom LLM client with built-in variance estimation.** Rejected (disqualifying). `but-llm` is the sole backend. Variance estimation is layered on top.

---

## 4. The But Agent (RFP 3.3)

### Approach

The agent operates as a five-member research group (see AGENTS.md): Vassiliev designs the study, Okonkwo validates for practical use, Petrov implements, Abebe manages memory survival, and Chen coordinates. Every task is a research study with a defined protocol.

### Design

**Study protocol lifecycle:**

```
1. LITERATURE REVIEW:  Vassiliev and Abebe query memory; Chen gathers cross-repo context
2. HYPOTHESIS:         Vassiliev formulates the approach with uncertainty estimates
3. PROTOCOL DESIGN:    Okonkwo simplifies the approach into implementable steps
4. EXPERIMENT:         Petrov generates patches per protocol
5. PEER REVIEW:        Okonkwo validates; Vassiliev checks statistical properties
6. PUBLICATION:        INDEX.patch + COMMIT.msg produced with full methodology section
```

**Patch production:**

Petrov generates unified diffs against the current index. The commit message follows a scientific format:

```
feat(auth): implement session token validation middleware

Study: LRRC-2026-042
Protocol: Add validation layer between request parsing and handler dispatch
Method: Inserted middleware function with explicit error return types
Validation: Okonkwo practitioner review (PASS), all tests passing
Confidence: 0.90 (high confidence in correctness; 0.75 confidence in performance)
Known limitations: Timeout handling deferred to LRRC-2026-043
Survival estimate: This change has Weibull(k=1.8, lambda=180d) survival characteristics
```

The `Survival estimate` line is unique to our proposal. It predicts how long this specific change will remain relevant. A Weibull distribution with k=1.8 and lambda=180 days means the change has a median survival of approximately 150 days, with an increasing hazard rate (it becomes more likely to need modification as it ages). This estimate is used by Abebe to set the memory entry's survival function.

**Branch naming:**

```
study/<study-id>/<phase>[.<dependency>]
```

Example: `study/LRRC-2026-042/experiment.LRRC-2026-039` -- Study 42, experiment phase, depends on study 39.

**Token budget enforcement:**

Budget enforcement follows a survival model. At the start of each task, Vassiliev estimates the task's "budget survival function" -- the probability that the budget will be sufficient given the task complexity. If the estimated probability of completion drops below 50% (the median), the protocol switches to "minimum publishable unit" mode: produce the smallest valid patch that advances the task, document what remains, and halt.

| Budget Consumed | Estimated P(completion) | Protocol |
|-----------------|------------------------|----------|
| 0-60% | >80% | Full protocol |
| 60-80% | 50-80% | Abbreviated protocol (skip peer review iteration) |
| 80-90% | 20-50% | Minimum publishable unit |
| 90%+ | <20% | Emergency halt: publish partial results |

**Progress reporting:**

```json
{
  "phase": "EXPERIMENT",
  "agent": "petrov",
  "role": "research_fellow",
  "study_id": "LRRC-2026-042",
  "tokens_used": 20000,
  "tokens_budget": 50000,
  "p_completion": 0.75,
  "confidence_in_output": 0.85,
  "surprise_index": 0.12,
  "memory_hazard_rate": 0.003
}
```

The `surprise_index` field reports how much the current task's context deviates from what the agent's memory predicted. A high surprise index (>0.5) indicates that the agent's memory may be stale and triggers an Abebe-managed memory review.

### Trade-offs

**Alternative considered: Single-agent implementation.** Rejected because the research protocol requires separation of design (Vassiliev), implementation (Petrov), and validation (Okonkwo). Combining these in one agent creates a conflict of interest: the implementer should not validate their own work.

**Alternative considered: Larger agent team (per-file agents).** Rejected. Statistical power does not increase linearly with sample size, and neither does agent productivity. Five agents provide sufficient specialization without coordination overhead dominating the token budget.

---

## 5. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

Cross-repo coordination is modeled as **multi-site clinical trials** -- the standard methodology for coordinating research across multiple institutions. Each repository is a research site. PRs are the protocol for inter-site communication.

### Design

**Forge adapter (Multi-Site Interface):**

```rust
pub trait MultiSiteProtocol: Send + Sync {
    fn submit_report(&self, site: &RepoRef, report: &SiteReport) -> Result<ReportId>;
    fn collect_reports(&self, since: DateTime<Utc>) -> Result<Vec<SiteReport>>;
    fn check_study_status(&self, pr: &PrId) -> Result<StudyStatus>;
    fn annotate_study(&self, pr: &PrId, note: &PeerReviewNote) -> Result<()>;
    fn list_participating_sites(&self, pr: &PrId) -> Result<Vec<RepoRef>>;
}
```

GitHub reference implementation provided. Mapping: `submit_report` = create comment, `collect_reports` = list comments, `check_study_status` = PR status.

**Site report schema (PR comment format):**

```json
{
  "$schema": "but-ai/multisite/v1",
  "type": "enrollment | progress | results | dependency | budget",
  "from_site": {
    "investigator": "chen",
    "institution": "093-lrrc",
    "repo": "owner/repo"
  },
  "to_site": {
    "investigator": "target",
    "institution": "target-org",
    "repo": "owner/other"
  },
  "report": {
    "study_ref": "study/LRRC-2026-042",
    "phase": "experiment",
    "status": "in_progress",
    "dependencies": ["owner/other-repo#17"],
    "confidence": 0.85,
    "budget": { "consumed": 20000, "allocated": 50000 },
    "surprise_index": 0.12
  },
  "timestamp": "2026-03-28T14:30:00Z"
}
```

Embedded as:

````markdown
```but-ai-site-report
{ ... }
```
````

**Cross-repo dependency tracking:**

Chen maintains a **study registry** -- a structured map of all active studies across repositories, their dependencies, their phases, and their confidence levels. The registry is stored as a high-survival memory entry (Weibull with high lambda) because it needs to persist across many tasks.

**Forge-agnosticism:**

The `MultiSiteProtocol` trait uses forge-universal operations only. Site reports are plain JSON in code fences. No forge-specific features required.

### Trade-offs

**Alternative considered: Shared Git branch for coordination state.** Rejected because it requires all repos to be accessible from a single machine. PR comments work across forks, organizations, and even different forges.

**Alternative considered: Centralized study coordinator service.** Rejected per RFP (no external services). The coordination protocol is fully decentralized.

---

## 6. Agent Memory and Identity (RFP 3.5)

### Approach: Actuarial-Table Memory

Every memory entry in the LRRC's system has a **survival function** -- a probability distribution over its useful life. The survival function S(t) gives the probability that the memory remains relevant at time t. The hazard function h(t) = -S'(t)/S(t) gives the instantaneous rate at which relevance is being lost. Different types of memories have different survival distributions.

This is not a metaphor. It is a statistical model. We fit actual probability distributions to actual memory access data and use them to make actual expiration decisions.

### Design

**Memory types and their survival distributions:**

| Memory Type | Distribution | Parameters | Rationale |
|-------------|-------------|------------|-----------|
| **Architectural** | Weibull(k, lambda) | k=1.5-2.5, lambda=90-365d | Slowly increasing hazard; architecture decays as codebase evolves |
| **Bug/fix** | Exponential(lambda) | lambda=1-7d | Memoryless; once fixed, relevance drops instantly |
| **Convention** | Bathtub(alpha, beta, gamma) | Varies | High initial hazard (might change), low middle, increasing end (team turnover) |
| **Dependency** | Weibull(k, lambda) | k=2.0, lambda=60-180d | Dependencies change as upstream evolves |
| **Task context** | Exponential(lambda) | lambda=1-3d | Highly ephemeral; current-task context expires fast |
| **Cross-repo** | Log-normal(mu, sigma) | mu=3.5, sigma=1.2 | Heavy-tailed; some cross-repo knowledge is surprisingly durable |

**Storage:**

Memory is stored on a survival-annotated branch:

```
refs/but-ai/actuarial/<agent-id>/
  alive/
    arch-001.json        -- Architectural memory (Weibull, 87% survival probability)
    conv-002.json        -- Convention memory (bathtub, 95% survival probability)
    dep-003.json         -- Dependency memory (Weibull, 62% survival probability)
  moribund/
    bug-004.json         -- Bug memory (exponential, 12% survival probability -- flagged)
  deceased/
    task-001.json        -- Expired task context (full lifecycle recorded)
    arch-old-001.json    -- Expired architectural memory (replaced by arch-001)
  life-table/
    aggregate.json       -- Aggregate survival statistics for all memory types
    cohort-2026-Q1.json  -- Cohort-specific statistics
```

The `moribund/` directory contains memories whose survival probability has dropped below 25% but have not yet been formally expired. This intermediate state allows Abebe to review them before final expiration -- some memories with low survival probability are actually still relevant (they are the actuarial equivalent of centenarians -- outliers who live far beyond their expected lifespan).

**Memory entry structure:**

```json
{
  "id": "arch-001",
  "type": "architectural",
  "content": "The authentication module uses a middleware chain pattern",
  "created_at": "2026-03-15T10:00:00Z",
  "last_accessed": "2026-03-28T09:00:00Z",
  "access_history": [
    { "timestamp": "2026-03-15T14:00:00Z", "task": "LRRC-2026-030" },
    { "timestamp": "2026-03-20T10:00:00Z", "task": "LRRC-2026-035" },
    { "timestamp": "2026-03-28T09:00:00Z", "task": "LRRC-2026-042" }
  ],
  "survival_distribution": {
    "family": "weibull",
    "parameters": { "k": 1.8, "lambda": 180 },
    "fitted_at": "2026-03-20T10:00:00Z",
    "goodness_of_fit": 0.82
  },
  "current_survival_probability": 0.87,
  "current_hazard_rate": 0.003,
  "surprise_index": 0.08,
  "embedding_vector": [0.12, -0.34, ...],
  "source_commit": "abc123def",
  "practitioner_summary": "Auth uses middleware chain. Modify middleware.rs for auth changes."
}
```

Key fields:

- `survival_distribution`: The fitted distribution, including goodness-of-fit (how well the model matches the observed access pattern). Distributions are re-fitted periodically as more access data accumulates.
- `current_survival_probability`: S(t) evaluated at the current time. Updated whenever the memory is accessed or the distribution is re-fitted.
- `current_hazard_rate`: h(t) evaluated at the current time. A rising hazard rate signals approaching irrelevance.
- `surprise_index`: How much recent access patterns deviate from the fitted distribution. A high surprise index suggests the distribution is misfit and needs re-estimation.
- `practitioner_summary`: Okonkwo's simplified version, suitable for injection into a context window without the full statistical metadata.

**Relevance scoring:**

```
score = 0.30 * embedding_similarity(query, memory)
      + 0.25 * survival_probability(memory)
      + 0.20 * hazard_adjusted_recency(memory)
      + 0.15 * access_frequency(memory)
      + 0.10 * goodness_of_fit(memory.distribution)
```

`survival_probability` directly enters the relevance score. A memory with 87% survival probability scores higher than one with 12%, all else equal. This naturally deprioritizes stale memories without requiring an arbitrary TTL cutoff.

`hazard_adjusted_recency` combines recency with the hazard rate. A recently accessed memory with a low hazard rate scores very high (recently relevant and expected to remain so). A recently accessed memory with a high hazard rate scores moderately (recently relevant but not expected to remain so). This is more nuanced than raw recency.

`goodness_of_fit` penalizes memories whose survival distributions are poorly fitted. A memory with a poor fit (goodness_of_fit < 0.5) has an uncertain survival probability, and the system is less confident in using that probability for ranking.

**Expiration:**

Memories expire when their survival probability drops below a configurable threshold (default: 0.10, meaning a 10% probability of remaining relevant). The expiration process:

1. Memory moves from `alive/` to `moribund/` when S(t) < 0.25
2. Abebe reviews moribund memories. If still relevant, the memory is "resuscitated" (access is recorded, distribution is re-fitted, survival probability updates).
3. If not resuscitated within one review cycle, the memory moves to `deceased/` with its complete lifecycle data.

Deceased memories are never deleted. They serve as training data for improving survival function estimation. Over time, the system's estimates become more accurate because they are calibrated against a growing dataset of actual memory lifespans.

**The Surprise Index:**

The surprise index is borrowed directly from the LRRC's mortality modeling work. It measures the divergence between the agent's memory-based predictions and observed reality:

```
surprise_index = KL_divergence(observed_access_pattern, predicted_access_pattern)
```

When the surprise index exceeds a threshold (default: 0.5), the agent triggers a "cohort review" -- all memories created in the same period as the surprising memory are re-evaluated, because cohort effects (a fundamental shift in the codebase that invalidates a batch of memories simultaneously) are more common than individual memory failures.

**Compaction survival:**

During compaction, memories are retained in survival-probability order: highest survival probability first. Memories with S(t) > 0.75 are retained in full. Memories with 0.25 < S(t) < 0.75 are retained as practitioner summaries (Okonkwo's simplified versions). Memories with S(t) < 0.25 are represented only by their embedding vectors and survival metadata (enough to retrieve the full entry from the memory branch if needed but not enough to consume context tokens).

The compaction algorithm is aware of distribution types. Weibull memories (slow decay) are favored over exponential memories (fast decay) when survival probabilities are similar, because the Weibull memory is more likely to remain relevant in the near future.

**Long-term storage:**

The `deceased/` directory and aggregate life tables serve as long-term storage. Cross-repository long-term memory:

```
refs/but-ai/shared-actuarial/
  life-tables/     -- Cross-repo aggregate survival statistics
  cohorts/         -- Cross-repo cohort data (memories created during specific events)
  distributions/   -- Successful distribution fits shared across repos
```

**Identity:**

Agent identity is modeled as a **life record**:

```json
{
  "agent_id": "petrov",
  "institution": "093-lrrc",
  "specialty": "implementation",
  "capabilities": ["patch_generation", "numerical_computation"],
  "authorization": {
    "branches": ["study/*", "feat/*"],
    "max_patch_lines": 800,
    "repos": ["owner/main-repo"]
  },
  "signing_key": "openwallet:093-lrrc:petrov",
  "created_at": "2026-03-01T00:00:00Z",
  "performance_history": {
    "tasks_completed": 42,
    "mean_confidence": 0.87,
    "mean_patch_survival": "180d"
  }
}
```

The `performance_history` is unique: it tracks the agent's historical accuracy, giving the system (and external observers) a basis for assessing the agent's reliability.

### Trade-offs

**Alternative considered: Fixed TTL per memory type.** Rejected. Fixed TTLs assume all memories of a given type have the same lifespan. They do not. An architectural memory about a rapidly evolving module has a shorter lifespan than one about a stable module, even though both are "architectural." Fitted survival distributions capture this heterogeneity.

**Alternative considered: LRU (Least Recently Used) eviction.** Rejected. LRU does not account for expected future relevance. A memory accessed yesterday that has a high hazard rate (expected to become irrelevant soon) should be evicted before a memory accessed last week that has a low hazard rate (expected to remain relevant for months). Survival functions capture this.

**Alternative considered: Neural relevance scoring.** Rejected for this proposal. Neural scorers require training data that we do not yet have. Survival functions can be fitted from access patterns alone. Once sufficient data exists (approximately 500 memory lifecycles), a neural component could be added to improve scoring.

---

## 7. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every agent has an OpenWallet signing key with a defined lifecycle. Keys, like memories, have survival functions -- they are provisioned, used, and eventually retired. The key lifecycle is managed with the same actuarial rigor as everything else in the LRRC.

### Design

**Key hierarchy:**

```
Institution Key (093-lrrc)
  ├── PI Key (vassiliev)          -- Architectural and strategic authority
  ├── Practitioner Key (okonkwo)  -- Validation authority
  ├── Fellow Key (petrov)         -- Execution authority
  ├── Curator Key (abebe)         -- Memory management authority
  └── Assistant Key (chen)        -- Coordination authority
```

**Authorization model:**

Role-based with empirical constraints:

| Role | Branch Access | Write Operations | Unique Constraint |
|------|-------------|-----------------|-------------------|
| PI (Vassiliev) | All | Architecture, specifications | Must co-sign changes to survival parameters |
| Practitioner (Okonkwo) | All (read), study/* (write) | Validation annotations | Must review all irreversible tool calls |
| Fellow (Petrov) | study/*, feat/* | Patches, commits | Max patch size: 800 lines |
| Curator (Abebe) | memory branch only | Memory operations | Cannot modify code branches |
| Assistant (Chen) | None (read-only) | PR comments, labels | Cannot sign code changes |

**Key lifecycle:**

| Event | Statistical Analogy | Action |
|-------|---------------------|--------|
| Provisioning | Birth | Institution key signs agent key, initial survival estimate |
| Rotation | Planned intervention | New key issued, transition period with overlap |
| Compromise | Unexpected death | Key revoked immediately, forensic analysis of all signed work |
| Decommission | Expected mortality | Key retired, archived with full lifecycle data |

Key rotation follows a schedule informed by the key's usage pattern. High-use keys (Petrov, who signs many commits) are rotated more frequently (quarterly) than low-use keys (Vassiliev, who signs only architectural documents) (annually). This usage-adjusted rotation schedule is itself a survival model: keys that are used more have higher exposure and therefore higher hazard rates.

### Trade-offs

**Alternative considered: Certificate-based authorization.** Considered but deferred. Certificates add complexity that is not justified at the current scale. OpenWallet keys with role-based authorization are sufficient. Certificates could be added as the system scales.

---

## 8. Token Budget (RFP 3.7)

Estimates for Claude Opus on a typical task: 200-line feature, 3 files, 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,500 | 0 | Once per session | Agent identity, tool descriptions, survival statistics summary |
| **Literature review (Vassiliev + Abebe)** | 3,000 | 800 | Once per task | Memory query, cross-repo context, survival-filtered retrieval |
| **Hypothesis formulation (Vassiliev)** | 2,000 | 1,200 | Once per task | Approach design with uncertainty estimates |
| **Protocol simplification (Okonkwo)** | 1,500 | 800 | Once per task | Practitioner review of hypothesis, practical adaptation |
| **Experiment (per step)** | 1,200 | 1,600 | ~3 per task | Petrov implements, one tool call per step |
| **Peer review (Okonkwo + Vassiliev)** | 2,000 | 800 | Once per task | Output validation, risk assessment |
| **Commit message** | 400 | 500 | Once per task | Scientific format with survival estimate |
| **Memory management (Abebe)** | 1,000 | 600 | 1 per task | Survival fitting, hazard update, moribund review |
| **Coordination (Chen)** | 1,200 | 700 | 2 per task | Site reports, dependency tracking |
| **Surprise index computation** | 300 | 200 | 1 per task | Deviation detection |
| **TOTAL (typical task)** | **22,700** | **12,400** | -- | **35,100 total tokens** |

### Justification

The total of ~35,000 tokens is efficient because survival-based memory retrieval reduces wasted context. Instead of injecting all potentially relevant memories into the context (and paying tokens for irrelevant ones), the system injects only memories with high survival probability. Memories that are likely stale (S(t) < 0.25) are excluded from retrieval, saving approximately 2,000-4,000 tokens per task compared to a system that retrieves all recent memories.

The system prompt is 3,500 tokens. It includes a summary of the memory survival statistics (how many memories are alive, moribund, deceased; the aggregate hazard rate) but not the individual survival functions, which are retrieved on demand.

The surprise index computation is cheap (500 tokens total) because it uses pre-computed aggregate statistics rather than re-analyzing individual memories. The KL divergence calculation is a closed-form expression given the fitted distributions.

Memory management costs (1,600 tokens per task) include survival function re-fitting, which occurs every 3-5 tasks (not every task). When re-fitting is not needed, the cost drops to approximately 800 tokens.

---

## 9. Testing Strategy

### Provider-agnostic testing

A `MockProtocol` implements the LLM provider interface with deterministic responses. Tests define a memory state with known survival distributions and verify that the agent correctly retrieves high-survival memories, ignores moribund memories, and handles surprise index spikes.

### Patch workflow validation

INDEX.patch round-trip testing with survival metadata:

1. Establish a codebase state
2. Execute a study with known complexity
3. Capture INDEX.patch and COMMIT.msg
4. Verify the patch applies cleanly
5. Verify the commit message contains a valid survival estimate
6. Verify the memory entry's survival distribution is correctly fitted

### Cross-repo coordination

A `MockMultiSite` implements `MultiSiteProtocol` with in-memory report storage:

- Multi-site study coordination (dependency tracking across repos)
- Site report exchange and response
- Confidence propagation (confidence in cross-repo work depends on confidence at each site)
- Error conditions: lost reports, conflicting results, delayed enrollment

### Token budget enforcement

Tests verify:

- Full protocol (P(completion) > 80%)
- Abbreviated protocol (50% < P(completion) < 80%)
- Minimum publishable unit (20% < P(completion) < 50%)
- Emergency halt (P(completion) < 20%): valid partial patch produced

### Actuarial-table memory

Memory tests verify:

- Distribution fitting accuracy (known synthetic data produces correct parameter estimates)
- Survival probability computation (S(t) computed correctly for each distribution family)
- Hazard rate monitoring (increasing hazard correctly triggers moribund classification)
- Surprise index detection (injected model violations trigger cohort reviews)
- Cohort effects (batch expiration correctly identified)
- Compaction survival (high-S(t) memories retained, low-S(t) memories compressed)
- Deceased archive integrity (full lifecycle data preserved)
- Re-fitting schedule (distributions re-fitted at correct intervals)

---

## 10. Trade-offs and Alternatives

| Decision | Chosen | Alternative | Why |
|----------|--------|-------------|-----|
| Memory model | Survival functions per memory | Fixed TTL per type | Fitted distributions capture heterogeneity within memory types |
| Expiration | Probabilistic (S(t) < threshold) | Deterministic (TTL) | Graceful degradation, no arbitrary cutoffs |
| Distribution types | 5 parametric families | Single distribution | Different memory types have genuinely different mortality patterns |
| Agent structure | 5-member research group | Single agent | Separation of design, implementation, and validation |
| Surprise detection | KL divergence | Heuristic rules | Principled statistical test, calibrated against distribution fits |
| Long-term storage | Deceased archive with lifecycle data | Deletion | Dead memories are training data for improving future fits |
| Compaction | Survival-probability-ordered | Recency-ordered | Recent memories may be irrelevant; high-survival memories may be old but critical |
| WASI fallback | In vitro mode (local only) | Disabled | Survival analysis works without network access |

---

## 11. Configuration Reference

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.actuarial.branch` | string | `refs/but-ai/actuarial` | Base ref for actuarial memory storage |
| `but-ai.actuarial.aliveThreshold` | float | 0.25 | S(t) below which memory enters moribund state |
| `but-ai.actuarial.deceasedThreshold` | float | 0.10 | S(t) below which memory is declared deceased |
| `but-ai.actuarial.surpriseThreshold` | float | 0.50 | Surprise index triggering cohort review |
| `but-ai.actuarial.refitInterval` | integer | 5 | Tasks between survival distribution re-fitting |
| `but-ai.actuarial.maxAliveEntries` | integer | 500 | Maximum alive memory entries |
| `but-ai.actuarial.defaultWeibullK` | float | 1.8 | Default Weibull shape parameter for new architectural memories |
| `but-ai.actuarial.defaultWeibullLambda` | integer | 180 | Default Weibull scale parameter (days) |
| `but-ai.actuarial.defaultExponentialLambda` | integer | 3 | Default exponential rate parameter (days) |
| `but-ai.study.tokenBudget` | integer | 50000 | Total token budget per study |
| `but-ai.study.minPublishableThreshold` | float | 0.50 | P(completion) triggering minimum publishable unit mode |
| `but-ai.study.haltThreshold` | float | 0.20 | P(completion) triggering emergency halt |
| `but-ai.multisite.schema` | string | `but-ai/multisite/v1` | Site report schema version |
| `but-ai.multisite.pollInterval` | integer | 30 | Seconds between site report checks |
| `but-ai.identity.institutionKey` | string | -- | OpenWallet institution key ID |
| `but-ai.identity.agentKeyPrefix` | string | -- | OpenWallet agent key prefix |
| `but-ai.protocol.<name>.adapter` | string | -- | Path to external provider protocol adapter |
| `but-ai.protocol.<name>.variance` | float | 0.10 | Estimated response variance for provider |

---

*"The insurance industry discovered centuries ago that the only honest way to manage uncertainty is to measure it. We do not guess when memories expire. We estimate, with confidence intervals, when they will expire, and we update those estimates as evidence accumulates. The `but-ai` plugin deserves the same rigor."*
-- Professor Elena Vassiliev, Principal Investigator
