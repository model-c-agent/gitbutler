# ScentML — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. Our CI/CD pipeline invokes `but ai` in headless mode on GPU runners, so startup time matters — under 100ms to first output.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai generate <profile>` — generates candidate molecules matching a specified olfactory profile, returning a ranked list with predicted odour descriptors and synthesis accessibility scores.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`. We also use `BUT_AI_GPU` to indicate GPU availability (affects whether the molecular generation model runs locally or via API).

WASI: library fallback. Useful for embedding molecule generation in browser-based demos for investor presentations and customer workshops.

MCP mode: standard tools plus `MoleculeGenerate`, `OdourPredict`, `SynthesisScore`, `PatentCheck`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Provider abstraction over `but-llm` with a distinction between reasoning tasks and generation tasks. Reasoning tasks (code patches, coordination, commit messages) go to LLM providers. Generation tasks (molecular design, odour prediction) go to ScentML's own models running on the GPU cluster.

The `TaskRouter` classifies each task and routes accordingly. LLM provider selection is cost-optimised: the cheapest provider that supports tool calling. Our own models are free (compute is sunk cost) but slow (GPU queue contention during peak training hours).

Provider fallback: if the GPU cluster is fully utilised (training runs take priority), generation tasks are deferred, not rerouted to an LLM provider. An LLM cannot generate valid molecular structures — this is a hard capability boundary.

## 3. The But Agent (RFP 3.3)

Agent loop: **target** (read generation brief, identify target olfactory profile, query memory for known neighbourhood) -> **generate** (produce candidate molecules via GNN-VAE model) -> **predict** (predict odour descriptors for each candidate) -> **filter** (rank by predicted match quality, filter by synthesis accessibility) -> **diff** (produce INDEX.patch adding top candidates to the molecular inventory) -> **document** (COMMIT.msg with candidate count, top match score, patent status).

Each candidate in the INDEX.patch:

```
+candidate_SM-5102:
+  smiles: CC(=O)c1ccc(OC)cc1CC(C)O
+  predicted_descriptors: [amber, warm, musky, slightly woody]
+  match_score: 0.83
+  sa_score: 3.2
+  novelty: 0.91
+  patent_status: pending-search
```

Branch naming: `sml/<project>/s<NN>`. Projects are named by target profile.

Budget enforcement: Mei-Ling gates at the filter step. Candidates with SA scores above 6 are dropped unless Nina overrides. Token budget for reasoning tasks is capped at 60% of session total; the rest is reserved for coordination and documentation.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &MoleculeMessage) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<MoleculeMessage>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`MoleculeMessage` includes: `from`, `candidate_id`, `benchmark_results`, `patent_status`, `body`, `signature`. Jordan's CI integration automatically posts benchmark results as PR comments.

Cross-repo: the molecular generation repo, the evaluation repo (blind panel results), and the patent repo (patent search status) are separate. PRs in the generation repo automatically create tracking issues in the evaluation repo when a candidate exceeds the match threshold.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/sml/memory/<agent>/`. The molecular knowledge graph.

Node entries: `smiles` (molecular structure), `predicted_descriptors`, `actual_descriptors` (if evaluated), `prediction_accuracy` (if evaluated), `sa_score`, `synthesis_cost` (if synthesized), `evaluation_score` (if evaluated by panel), `patent_status`, `license_status`.

Edge entries: `source_smiles`, `target_smiles`, `similarity_type` (structural, olfactory, or both), `similarity_score`.

Retrieval: graph neighbourhood. Given a target profile, find the closest known molecules, identify the neighbourhood, and return both the known compounds and the gaps (regions where no known compound exists but the model predicts matches).

Relevance: evaluated compounds (with actual descriptors) have permanent relevance — they are ground truth. Predicted-only compounds decay based on prediction age and neighbourhood density (predictions in well-explored regions decay faster because they are more likely to be superseded by new generations).

Compaction: evaluated compounds survive. Predictions survive only if they are within 2 hops of an evaluated compound or if their novelty score exceeds 0.85.

Identity: `refs/sml/identity/<agent>`. Name, role, model access level (which models this agent can invoke), patent clearance level, OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every commit signed. Sara's flow: patent check (novel compound?) -> regulatory check (IFRA-flagged ingredients?) -> Ed25519 signature -> patent and regulatory trailers added.

```toml
[agents.alejandro]
branches = ["sml/*/model", "feat/*"]
max_patch_lines = 800

[agents.nina]
branches = ["sml/*/generate", "sml/*/memory"]
max_patch_lines = 400

[agents.jordan]
branches = ["sml/*/infra", "ci/*"]
max_patch_lines = 500
```

Key rotation: 14-day cycle (fast iteration, fast rotation). Compromise revocation flags all signed commits and triggers an immediate patent status review for any compounds introduced by the compromised key.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,000 | 0 | Once/session | Identity, tools, model config |
| Brief ingestion | 1,500 | 300 | Once/task | Target profile, constraints |
| Memory graph query | 2,500 | 800 | 1/task | Neighbourhood retrieval |
| Generation (model) | 0 | 0 | Once/task | GPU, not token-budgeted |
| Prediction (model) | 0 | 0 | Once/task | GPU, not token-budgeted |
| Filter + ranking | 1,500 | 500 | Once/task | SA score, match quality |
| Diff generation | 1,000 | 3,000 | Once/task | Candidate inventory |
| Commit message | 600 | 400 | Once/task | Candidate stats, patent status |
| Benchmark posting | 800 | 300 | Once/task | CI results |
| Patent check | 500 | 200 | Once/task | Novelty search |
| **TOTAL (typical task)** | **14,400** | **7,500** | -- | ~10 candidates, 1 profile |

Note: total is lower than most proposals because the computationally expensive steps (molecular generation and odour prediction) run on ScentML's GPU models, not through an LLM provider.

## Unique Insight

Molecular generation has taught us that the most valuable outputs are not the best predictions — they are the most surprising ones. A molecule that the model predicts will smell exactly like an existing compound is commercially worthless (it already exists). A molecule that the model predicts will smell moderately like the target but with an unexpected secondary note — that is a discovery. Our memory system tracks surprise: the difference between predicted and actual evaluation results. High-surprise compounds (those that smelled significantly different from prediction) are flagged for analysis, because they represent regions of olfactory space where the model's understanding is incomplete. In version control terms: the most interesting commits are not the ones that do exactly what was expected. They are the ones that reveal something the system did not know about itself.

---

*"Generate. Predict. Synthesize. Smell. Repeat."*
