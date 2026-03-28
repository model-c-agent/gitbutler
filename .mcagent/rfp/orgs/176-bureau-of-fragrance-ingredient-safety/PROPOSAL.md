# Bureau of Fragrance Ingredient Safety — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. Must comply with ECHA IT security requirements: no external network calls during initialization, no telemetry, audit logging of all operations.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai dossier <cas-number>` — retrieves the current literature review status for a substance, including study counts by endpoint and Klimisch score distribution. This is the Bureau's most frequent query.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`. We also read `ECHA_CLASSIFICATION` for tagging outputs with confidentiality levels.

WASI: library fallback. ECHA is exploring WASI for sandboxed execution on shared infrastructure. The Bureau would deploy WASI builds to ensure that literature review agents cannot access substance data outside their assigned dossier.

MCP mode: standard tools plus `DossierQuery`, `StudySummarise`, `EndpointCheck`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Single approved provider: Azure OpenAI via the EU Government Cloud. The provider abstraction wraps `but-llm` but is, in practice, a wrapper around one provider with the interface designed for eventual multi-provider support when (if) ECHA approves additional providers.

The abstraction adds: audit logging of every API call (required by ECHA's data processing register), confidentiality classification of inputs (CBI material must be stripped before API submission), and cost tracking in EUR (budget is allocated quarterly by ECHA's Finance unit).

No dynamic provider loading. No local models (ECHA's IT governance has not approved any). If the approved provider is unavailable, tasks queue.

## 3. The But Agent (RFP 3.3)

Agent loop: **receive** (accept molecule review assignment, identify CAS number and endpoints) -> **search** (query literature database for relevant studies) -> **summarise** (for each study: produce structured summary with Klimisch assessment) -> **synthesise** (combine study summaries into an endpoint-level evidence synthesis) -> **format** (produce INDEX.patch adding the summary to the substance dossier) -> **review** (internal review cycle: Kowalczyk citation check, MacNeil compliance check, Virtanen authorization).

Every COMMIT.msg includes: `Substance: <CAS number>`, `Endpoint: <endpoint>`, `Studies-Reviewed: <count>`, `Klimisch-Distribution: <1: N, 2: N, 3: N, 4: N>`, `Review-Status: draft | reviewed | authorized`.

Branch naming: `bfis/<cas-number>/<endpoint>/s<NN>`. One branch per substance-endpoint combination.

Budget enforcement: Virtanen allocates a per-molecule budget at assignment. The budget covers the full review cycle for all endpoints. If a single endpoint exhausts more than its proportional share (total budget / number of endpoints), the agent pauses and requests reallocation.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter for ECHA's internal GitLab:

```
trait ForgeAdapter {
    fn create_mr(&self, repo: &RepoRef, spec: &MrSpec) -> Result<MrId>;
    fn post_comment(&self, mr: &MrId, note: &ReviewNote) -> Result<NoteId>;
    fn list_comments(&self, mr: &MrId) -> Result<Vec<ReviewNote>>;
    fn status(&self, mr: &MrId) -> Result<MrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<MrId>;
}
```

Note: the Bureau uses "Merge Request" (GitLab terminology), not "Pull Request." The adapter accommodates both.

`ReviewNote` includes: `reviewer`, `section` (which endpoint), `comment_type` (scientific/procedural/editorial), `klimisch_relevance` (does this comment affect study reliability assessment?), `body`, `signature`, `confidentiality`.

Cross-repo: each substance has its own repo. Cross-substance coordination (e.g., a new study that affects multiple substances) uses cross-reference MRs.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/bfis/memory/<agent>/`. Structured as the Bureau's substance database.

Study memory: `substance_cas`, `study_id`, `authors`, `year`, `journal`, `doi`, `study_type`, `species`, `endpoint`, `dose_response`, `result`, `klimisch_score`, `klimisch_rationale`, `review_date`, `reviewer`.

Substance memory: `cas_number`, `name`, `molecular_weight`, `endpoints_reviewed` (array), `current_status` (in-queue/under-review/authorized/denied/restricted), `decision_date`.

Retrieval: by substance and endpoint. Deterministic — the same query always returns the same results. No semantic search, no relevance scoring. The Bureau's review process requires reproducibility: if two reviewers query the same substance, they must get the same literature base.

Relevance: studies do not decay. A 1985 carcinogenicity study is as relevant as a 2025 one (assuming adequate quality). Relevance is determined by Klimisch score, not by date.

Compaction: substance summaries survive. Individual study records are excluded from compacted context but retrievable via explicit query.

Identity: `refs/bfis/identity/<agent>`. Name, grade (EU staff grade), role, review authorization level, training certifications (GLP, Klimisch assessment), OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every commit signed. MacNeil's flow: GDPR check -> CBI check -> regulatory reference check -> Ed25519 signature via OpenWallet -> compliance trailers added.

Authorization:

```toml
[agents.alexandrou]
branches = ["bfis/*/summarise", "bfis/*/synthesise"]
max_patch_lines = 500
requires_authorization = "virtanen"

[agents.kowalczyk]
branches = ["bfis/*/citations"]
max_patch_lines = 200

[agents.virtanen]
branches = ["bfis/*"]
max_patch_lines = 50
```

Key rotation: 90-day cycle (aligned with ECHA's quarterly IT audit). Compromise: immediate revocation, incident report to ECHA's Data Protection Officer, and re-review of all substance dossiers signed with the compromised key.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,500 | 0 | Once/session | Identity, tools, SOP, classification |
| Assignment receipt | 1,500 | 200 | Once/task | CAS number, endpoint list |
| Literature search | 2,000 | 500 | 1/task | Study retrieval by endpoint |
| Study summarisation | 3,500 | 4,000 | Once/task | ~5 studies per endpoint |
| Evidence synthesis | 2,000 | 1,500 | Once/task | Endpoint-level summary |
| Diff generation | 1,000 | 1,500 | Once/task | Dossier update |
| Commit message | 700 | 400 | Once/task | Substance, endpoint, Klimisch dist. |
| Citation check | 800 | 200 | 1/task | Kowalczyk's verification |
| Compliance check | 500 | 200 | 1/task | MacNeil's review |
| Authorization | 500 | 100 | 1/task | Virtanen's decision |
| **TOTAL (typical task)** | **19,500** | **10,600** | -- | 1 endpoint, ~5 studies |

## Unique Insight

Fourteen years of ingredient review has taught the Bureau that the most dangerous molecules are not the ones that fail testing — they are the ones that were never adequately tested. A molecule with three reliable studies showing safety is less concerning than a molecule with thirty unreliable studies showing safety. Quantity of evidence is not quality of evidence. Our Klimisch-scored memory system embodies this: agent memory is weighted by study quality, not study count. An agent that retrieves ten Klimisch-4 studies has retrieved less useful evidence than an agent that retrieves one Klimisch-1 study. In version control terms: the number of commits is irrelevant. What matters is whether each commit was made with adequate context and sufficient rigour. Velocity without quality is a liability.

---

*"The review is not complete until it is correct."*
