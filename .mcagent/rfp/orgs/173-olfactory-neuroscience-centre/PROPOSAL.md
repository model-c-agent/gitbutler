# Olfactory Neuroscience Centre — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. The Centre runs Linux workstations and accesses HPC via SSH. The binary must work in both interactive and batch modes (for HPC job submission).

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai pipeline <stage>` — runs a specific stage of the Centre's computational pipeline (preprocessing, encoding, training, analysis) as an agent task. This maps our existing workflow directly onto the agent framework.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`.

WASI: library fallback. Potentially useful for running lightweight preprocessing agents on resource-constrained nodes in the HPC cluster.

MCP mode: standard tools plus `PipelineRun`, `OdourQuery`, `AssociationPredict`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Provider abstraction wrapping `but-llm`. The Centre uses a single provider (Anthropic) for all generative tasks and local compute for all model-training tasks. Model training does not go through the LLM provider — it runs on the Centre's GPU workstation.

The provider abstraction includes a `TaskType` classifier that separates generative tasks (patch generation, commit messages, coordination) from computational tasks (model training, data preprocessing). Generative tasks route to the LLM provider. Computational tasks route to local execution. This distinction is important: the Centre's token budget covers only generative tasks. Computational costs are funded separately through the Centre's HPC allocation.

Capability detection: minimal. The Centre uses one provider and knows its capabilities.

## 3. The But Agent (RFP 3.3)

Agent loop: **contextualise** (read task, retrieve relevant odour-memory findings from memory) -> **design** (plan computational approach, estimate token cost) -> **implement** (generate INDEX.patch for pipeline code) -> **validate** (run pipeline stage in test mode, check output format) -> **document** (COMMIT.msg with Method-Section trailer).

The Method-Section trailer is our signature feature. Every COMMIT.msg includes a paragraph written in the style of a scientific paper's methods section. This serves double duty: it documents the change for code review and it drafts text for the eventual publication. Yuki writes the method section; Sophie refines it during review.

Branch naming: `onc/<pipeline-stage>/s<NN>`. Pipeline stages: `preprocess`, `encode`, `train`, `analyze`.

Budget enforcement: Obi's information-gain analysis. Before each generative task, the agent estimates the expected information gain (how much the output will reduce uncertainty about the pipeline's correctness). Tasks with low expected information gain are deferred or simplified.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_review(&self, pr: &PrId, review: &AcademicReview) -> Result<CommentId>;
    fn list_reviews(&self, pr: &PrId) -> Result<Vec<AcademicReview>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`AcademicReview` includes: `reviewer`, `method_critique`, `statistical_concerns`, `suggestions`, `contribution_statement`, `signature`. This maps directly onto the peer review process the Centre's researchers already use for paper drafts.

Cross-repo: the Centre collaborates with three external labs. Each lab has its own repository. Coordination happens through PR review comments formatted as academic review. The review format is familiar to all collaborators.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/onc/memory/<agent>/`. Structured as a research findings database.

Each entry: `stimulus` (molecular descriptor vector, stored as comma-separated floats), `context_type` (lab/field/clinical), `finding` (plain-text description), `effect_size` (Cohen's d or equivalent), `p_value`, `sample_size`, `association_strength` (model prediction, 0.0-1.0), `publication` (DOI if published), `tags`.

Retrieval: association-strength-weighted. Entries with higher association strength (stronger odour-memory links) are prioritised. This mirrors the brain's own retrieval bias — strongly encoded memories are more accessible.

Relevance: findings do not decay. A replicated finding from 2016 is more relevant than an unreplicated finding from 2025. The relevance score is weighted by replication count: `relevance = association_strength * (1 + 0.5 * replication_count)`.

Compaction: published findings (with DOIs) survive compaction. Unpublished findings are summarised. Raw experimental data is excluded from compacted context but available via explicit retrieval.

Identity: `refs/onc/identity/<agent>`. Name, position, ORCID (yes, the academic identifier), research focus, ethics training date, OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every commit signed. James's flow: ethics check (does the commit reference human data? if so, is the ethics approval number present?) -> data classification check -> Ed25519 signature via OpenWallet -> commit finalized.

Authorization:

```toml
[agents.yuki]
branches = ["onc/*/implement"]
max_patch_lines = 500
requires_ethics = false

[agents.ashworth]
branches = ["onc/*"]
max_patch_lines = 100
requires_ethics = true
```

Key rotation: 60-day cycle (aligned with the university term). Compromise revocation: immediate, with notification to the university's information security office and ethics committee (because compromised keys could theoretically be used to sign commits referencing human subjects data).

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,000 | 0 | Once/session | Identity, tools, pipeline context |
| Task ingestion | 1,800 | 300 | Once/task | Pipeline stage, parameters |
| Memory retrieval | 2,000 | 500 | 1/task | Association-weighted findings search |
| Design/planning | 1,200 | 600 | Once/task | Approach, budget estimate |
| Tool calls (per call) | 900 | 400 | ~4/task | Parameter + result |
| Patch generation | 3,500 | 5,000 | Once/task | Pipeline code + method section |
| Validation | 1,000 | 200 | Once/task | Test-mode execution check |
| Commit message | 800 | 500 | Once/task | Method section + conventional commit |
| Coordination | 1,000 | 400 | 0.5/task | Academic review exchange |
| **TOTAL (typical task)** | **18,800** | **10,300** | -- | 1 pipeline stage, 3-4 files |

## Unique Insight

The Centre's decade of odour-memory research has revealed a property of human memory that most computational memory systems ignore: memories are not retrieved — they are reconstructed. Every time you recall a memory, your brain rebuilds it from fragments, and the rebuilt version is slightly different from the original. This means that the "same" memory retrieved twice is not identical. Our agent memory system deliberately models this: retrieval includes a stochastic perturbation step that slightly varies the context injected for each retrieval. This is not noise — it is exploration. A memory system that returns identical results for identical queries will converge on a single interpretation. A system with controlled reconstruction variance explores the space of possible interpretations, occasionally surfacing connections that a deterministic system would never find.

---

*"To remember is to reconstruct. To reconstruct is to discover."*
