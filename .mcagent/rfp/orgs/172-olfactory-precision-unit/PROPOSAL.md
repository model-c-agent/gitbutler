# Olfactory Precision Unit — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. Must comply with DoD software assurance requirements: statically linked, reproducible build, no network calls during initialization, no telemetry.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai classify <path>` — scans a file or patch and determines its classification level based on the data it references. This is mandatory for our workflow: no commit can proceed without a classification determination.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`, `BUT_AI_CLASSIFICATION` (sets the session's maximum classification level).

WASI fallback: essential. The unit would deploy WASI builds in sandboxed environments on classified systems where the host OS cannot be modified.

MCP mode: standard tools plus `MolecularQuery`, `FormulationProtocol`, `ClassificationCheck`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Dual-network provider architecture. Unclassified tasks route to external providers (Anthropic via the unclassified network). Classified tasks route to a self-hosted model on the classified network. No classified data ever leaves the classified network.

The provider abstraction wraps `but-llm` with a `ClassifiedProvider` trait that adds: mandatory classification level declaration per call, network boundary enforcement, and audit logging of every provider interaction.

Provider selection: determined by input classification, not by task complexity. A simple task with classified inputs goes to the classified provider. A complex task with unclassified inputs goes to the external provider. Classification trumps optimization.

No dynamic provider loading. All providers are compiled in. Runtime code loading is prohibited on DoD systems.

## 3. The But Agent (RFP 3.3)

Agent loop: **classify** (determine input data classification) -> **analyze** (query molecular memory, identify candidate compounds) -> **formulate** (generate formulation protocol as INDEX.patch) -> **validate** (peer review by Tanaka, GC-MS prediction check) -> **authorize** (Oduya's command decision) -> **commit** (COMMIT.msg with classification, protocol reference, CAS numbers).

Formulation protocols are structured precisely:

```
Component 1: CAS 5392-40-5 (citral) | 450mg | Add at T+0
Component 2: CAS 127-91-3 (beta-pinene) | 120mg | Add at T+5min
Component 3: CAS 97-53-0 (eugenol) | 85mg | Add at T+10min
Mixing: magnetic stir 200rpm for 30min at 25C
Holding: sealed, dark, 48h before evaluation
```

Branch naming: `opu/<project-code>/s<NN>`. Project codes follow DoD format.

Budget enforcement: two gates. Wei checks token budget. Oduya checks scope. Both must approve before execution continues.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &ClassifiedMessage) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<ClassifiedMessage>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`ClassifiedMessage` includes mandatory `classification` field on every message. Messages posted to the unclassified forge must be UNCLASSIFIED. The adapter enforces this at the application level.

GitHub Enterprise (DoD instance) for unclassified work. No public forge. Cross-repo coordination is limited to repos within the DoD GitHub Enterprise organization.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/opu/memory/<agent>/`. Dual memory stores: unclassified (public molecular data) and classified (unpublished predictions and formulation results).

Unclassified memory: `cas_number`, `iupac_name`, `molecular_weight`, `olfactory_descriptors`, `molecular_descriptors`, `public_source`. Stored in the unclassified repo.

Classified memory: `compound_id` (internal), `predicted_scent`, `prediction_confidence`, `evaluation_results`, `formulation_protocols`. Stored on the classified network. Never synchronized to the unclassified repo.

Retrieval: molecular similarity search using pre-computed descriptor vectors. The similarity computation runs locally (no API calls — the model is embedded in the binary).

Relevance: no decay. Molecular properties do not change over time. Evaluation results are permanently relevant. Memory entries are removed only by explicit command authorization (Oduya's decision).

Compaction: unclassified compound profiles survive. Classified evaluation details are excluded from compacted context and require explicit retrieval with classification verification.

Identity: `refs/opu/identity/<agent>`. Name, rank/title, clearance level, role, authorization scope, OpenWallet key fingerprint. Clearance level determines which memory stores the agent can access.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every commit signed with Ed25519 via OpenWallet. Abrams's flow: classification check -> clearance verification (does the signing agent have clearance for the data in the commit?) -> signature -> classification trailer embedded.

Authorization:

```toml
[agents.reyes]
branches = ["opu/*/formulate"]
max_patch_lines = 500
clearance = "CUI"

[agents.tanaka]
branches = ["opu/*/analyze", "opu/*/review"]
max_patch_lines = 300
clearance = "CUI"

[agents.oduya]
branches = ["opu/*"]
max_patch_lines = 50
clearance = "SECRET"
```

Key rotation: 30-day cycle, aligned with the unit's security audit schedule. Compromise: immediate revocation, incident report to the unit's Information Security Officer, and re-review of all signed commits.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,200 | 0 | Once/session | Identity, tools, classification rules |
| Classification check | 800 | 200 | Once/task | Input data classification |
| Molecular analysis | 3,000 | 800 | Once/task | Similarity search, candidate selection |
| Formulation generation | 3,500 | 5,500 | Once/task | Protocol with precise measurements |
| Peer review | 2,000 | 500 | Once/task | Tanaka's technical evaluation |
| Tool calls (per call) | 1,000 | 500 | ~4/task | Parameter + result |
| Commit message | 700 | 400 | Once/task | Classification, CAS numbers, protocol ref |
| Authorization | 500 | 100 | Once/task | Oduya's decision |
| **TOTAL (typical task)** | **18,700** | **10,500** | -- | 1 formulation, ~8 compounds |

## Unique Insight

From five years of molecular olfactory prediction, the unit has learned that the relationship between molecular structure and smell is not a function — it is a distribution. The same molecule can smell different to different people (genetic variation in olfactory receptors), in different concentrations (threshold effects), and in different contexts (perceptual masking). This means that "ground truth" in olfactory prediction is inherently probabilistic. Our agents produce formulations with confidence intervals, not certainties. The confidence interval is not a weakness — it is the most honest representation of what computation can tell you about something that ultimately requires a human nose to evaluate. Any system that claims deterministic olfactory prediction is lying.

---

*"Measure twice. Formulate once. Evaluate with your nose."*
