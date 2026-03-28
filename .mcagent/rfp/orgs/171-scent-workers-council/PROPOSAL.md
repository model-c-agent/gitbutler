# Scent Workers' Council — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. The Council's lab runs a mix of Linux and macOS machines. Cross-platform builds are essential.

Subcommands: `agent` (formula tasks), `memory` (ingredient database queries), `status` (budget, pending evaluations), `mcp` (MCP server). We add `but ai formulate <brief>` — generates a candidate formula from a natural language olfactory brief, leveraging the ingredient memory system.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`.

WASI fallback: `but-ai-core` linked statically. The Council is interested in running formula generation agents in a browser for public demonstrations where attendees submit briefs and receive candidate formulas in real time.

MCP tools: standard plus `FormulaGenerate`, `IngredientQuery`, `AccordAnalyze`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Provider abstraction over `but-llm`. The Council uses Anthropic for formula generation (requires nuanced understanding of olfactory vocabulary) and a local Ollama instance for routine tasks (commit messages, PR comments).

Provider selection is task-phased: creative phases (brief interpretation, formula composition) route to the best available provider regardless of cost. Mechanical phases (formatting, signing prep) route to the cheapest.

Capability requirements: structured output support is essential. Formula generation must produce structured JSON (ingredient list with CAS numbers and concentrations), not free text. Providers without structured output support are excluded from formula generation tasks.

## 3. The But Agent (RFP 3.3)

Agent loop: **interpret** (read olfactory brief, identify target accord families) -> **research** (query ingredient memory for matching profiles) -> **compose** (generate candidate formula as structured data) -> **evaluate** (check formula balance: top/heart/base ratio, total concentration, cost estimate) -> **diff** (produce INDEX.patch against Council's formula template) -> **document** (COMMIT.msg with formula name, brief summary, olfactory pyramid).

Formula-specific COMMIT.msg format:

```
feat(formula): Terre Nouvelle — warm woody with bergamot opening

Top: bergamot (15%), pink pepper (3%), cardamom (2%)
Heart: iris butter (8%), cedar atlas (12%), labdanum (5%)
Base: vetiver (18%), musk (4%), amber (3%)
Total: 70% | Alcohol: 30%

License: CC-BY-SA-4.0
Provenance: original
Brief: "warm, earthy, gender-neutral, autumn"
```

Branch naming: `swc/<formula-name>/s<NN>`. Formula names are slugified French.

Budget enforcement: Kofi gates at the composition phase. If budget is insufficient for full formula generation, the agent produces a "brief note" — a structured outline of the intended formula's pyramid and key accords, tagged `SKETCH: compounding not recommended without full generation`.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &FormulaMessage) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<FormulaMessage>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`FormulaMessage` includes: `type`, `from`, `formula_id`, `accord_notes` (olfactory observations), `supply_chain_impact`, `body`, `signature`. Cross-lab coordination between the Council's Grasse lab and partner labs (they collaborate with indie perfumers in Tokyo and Brooklyn) uses PR comments as structured evaluation notes.

Evaluation protocol: when a formula is proposed via PR, other labs compound it independently and post evaluation comments with dry-down observations at 15 minutes, 1 hour, and 4 hours. Merge requires at least two independent evaluations.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/swc/memory/<agent>/` as Git blobs. Two memory subsystems: the ingredient database (Rashid's domain) and the formula history (collective).

Ingredient memory: `cas_number` (primary key), `name`, `family`, `descriptors` (controlled vocabulary), `volatility`, `natural_source`, `synthetic_available`, `price_per_kg`, `suppliers`, `last_verified`.

Formula memory: `formula_id`, `name`, `brief`, `pyramid` (top/heart/base ingredients with concentrations), `total_concentration`, `evaluations` (array of dry-down observations), `status` (draft/evaluated/published).

Retrieval: for ingredients, descriptor-based similarity. For formulas, brief-based similarity (finding formulas that responded to similar briefs).

Relevance: ingredient profiles do not decay (benzaldehyde still smells like almonds). Formula memories decay slowly — a formula brief from 2023 is still relevant for understanding the Council's aesthetic direction, but its specific ingredients may have price or availability changes.

Compaction: ingredient profiles survive (they are compact and universally useful). Formula details survive only if the formula has been published. Draft formulas are excluded from compacted context.

Identity: `refs/swc/identity/<agent>`. Name, specialization (ingredient families), authorization scope, CC license assertion, OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every formula commit signed and licensed. Camille's flow: formula produced -> evaluated -> Camille verifies provenance (original vs. derived) -> license assertion attached -> Ed25519 signature via OpenWallet -> commit finalized.

Authorization:

```toml
[agents.ines]
branches = ["swc/*/compose"]
max_patch_lines = 300

[agents.rashid]
branches = ["swc/*/ingredients"]
max_patch_lines = 200

[agents.yelena]
branches = ["swc/*/coord"]
max_patch_lines = 100
```

Key rotation: 30-day cycle. Revocation triggers re-verification of all formulas signed with the compromised key — critical because published formulas carry licensing assertions that must remain valid.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,800 | 0 | Once/session | Identity, tools, olfactory vocabulary |
| Brief interpretation | 1,500 | 400 | Once/task | Olfactory brief parsing |
| Ingredient research | 2,500 | 600 | 2/task | Descriptor-based retrieval |
| Formula composition | 3,000 | 4,000 | Once/task | Structured formula generation |
| Evaluation check | 1,200 | 300 | Once/task | Balance, concentration, cost |
| Diff generation | 1,000 | 2,000 | Once/task | Against formula template |
| Commit message | 600 | 500 | Once/task | Full pyramid notation |
| Coordination | 800 | 300 | 0.5/task | Cross-lab evaluation |
| Licensing | 300 | 200 | Once/task | Provenance + CC assertion |
| **TOTAL (typical task)** | **16,700** | **10,300** | -- | 1 formula, ~15 ingredients |

## Unique Insight

Perfumery teaches you that memory is not information — it is evocation. The smell of vetiver does not make you think "terpenoid compound with earthy-woody character." It makes you think of wet soil after rain, or your grandmother's garden, or a specific afternoon you have not thought about in years. Our ingredient memory system stores chemical data, but its real value is associative: when an agent retrieves vetiver, it also retrieves the olfactory associations that make vetiver meaningful in a formula context. An agent that knows vetiver's CAS number but not its emotional resonance will compose technically correct formulas that smell like nothing. Memory without association is a catalog. Memory with association is a palette.

---

*"Smell is the last sense that cannot be digitized. We are not trying to digitize it. We are trying to share it."*
