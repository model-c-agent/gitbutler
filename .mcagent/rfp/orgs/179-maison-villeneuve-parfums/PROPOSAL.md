# Maison Villeneuve Parfums — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. The house runs one Mac Studio (Eloise's office), two Windows laptops (Hugo and Marie-Claire), and nothing in Jean-Pierre's office except a telephone and 400 bottles of jasmine absolute from different years.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai jasmine <year>` — retrieves the GC-MS profile and evaluation notes for a specific harvest year's jasmine absolute. This is Marie-Claire's most frequent query and the only command Jean-Pierre might eventually use (through Fatima).

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`.

WASI: library fallback. Hugo is interested in embedding formulation exploration in the house's website as a customer-facing feature. Eloise has not approved this.

MCP mode: standard tools plus `FormulaQuery`, `JasmineProfile`, `CostEstimate`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Single provider (Anthropic). Eloise selected it. Hugo set it up. Jean-Pierre does not know it exists.

The provider abstraction wraps `but-llm` with cost tracking in EUR. The house's AI budget is small (allocated from the marketing budget, which Eloise controls). All tasks route to one provider. No local models — the house does not have GPU hardware and Eloise considers self-hosting infrastructure an unnecessary distraction.

Capability detection: tool calling support verified at initialization. If the provider changes its API (it has happened before — Hugo spent a weekend fixing the integration), the plugin logs the incompatibility and enters manual mode.

## 3. The But Agent (RFP 3.3)

Agent loop: **brief** (read formulation task, identify target olfactory character) -> **recall** (query Marie-Claire's archive for relevant historical formulas) -> **compose** (generate formula proposal as INDEX.patch, always including jasmine absolute) -> **cost** (calculate raw material cost per bottle, including jasmine sourcing) -> **submit** (produce COMMIT.msg with formula name, jasmine percentage, cost per bottle, and margin projection).

The jasmine constraint is hard-coded. No formula proposal is generated without a jasmine absolute component. This is not configurable. It is identity.

COMMIT.msg format:

```
feat(formula): Nuit Jasminee — warm amber-jasmine for autumn

Jasmine: Grasse estate, 2025 harvest, 18%
Cost-Per-Bottle: EUR 34.20 (100ml)
Margin: 72%
IFRA-Status: compliant
Master-Approval: pending
```

Branch naming: `mv/<formula-slug>/s<NN>`.

Budget enforcement: Eloise's direct control. Each formulation project has a pre-approved token budget. Marie-Claire's archive queries consume the most tokens; Eloise allocates accordingly.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &HouseNote) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<HouseNote>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`HouseNote`: `from`, `formula`, `type` (formulation/quality/market/approval), `body`, `market_position`, `signature`. Simple schema. The house does not need elaborate coordination — five people in one building.

Cross-repo: formulation repo (private) and marketing repo (private, Hugo manages). PRs in the formulation repo create tracking issues in the marketing repo when a formula reaches "approved" status.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/mv/memory/<agent>/`. The house's century of formulation knowledge.

Formula memory: `formula_name`, `version`, `year`, `ingredients` (with concentrations and sourcing), `jasmine_percentage`, `jasmine_harvest_year`, `jasmine_profile` (olfactory descriptors for that year's harvest), `evaluation` (Jean-Pierre's words, transcribed by Fatima), `status`, `cost_per_bottle`, `sales_data` (units sold, if launched).

Jasmine memory: `harvest_year`, `yield_kg`, `gc_ms_profile` (key compounds and percentages), `olfactory_character` (Jean-Pierre's assessment), `weather_notes`, `notable_events`.

Retrieval: by formula name, by jasmine year, or by olfactory character. "Find formulas with a strong indolic jasmine and warm amber base" returns matches. The jasmine year is a first-class index key — the character of the jasmine changes the character of every formula that uses it.

Relevance: no decay. A formula from 1998 is still relevant. The house's style is built on continuity, not disruption. Older formulas are reference points, not deprecated artifacts.

Compaction: active formulas (still in production) survive with full detail. Archived formulas survive as summaries. Jasmine profiles survive permanently — they are irreplaceable records of harvests that cannot be repeated.

Identity: `refs/mv/identity/<agent>`. Name, role, family relationship (if applicable), tenure, authorization level, OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every commit signed. Fatima's flow: IFRA compliance check -> allergen declaration check -> cost verification -> Jean-Pierre's verbal approval (recorded as trailer) -> Ed25519 signature via OpenWallet.

```toml
[agents.eloise]
branches = ["mv/*/compose", "mv/*/cost"]
max_patch_lines = 400

[agents.marie-claire]
branches = ["mv/*/archive"]
max_patch_lines = 200

[agents.hugo]
branches = ["mv/*/market"]
max_patch_lines = 150
```

Key rotation: 60-day cycle. The house's pace is seasonal, aligned with the jasmine harvest (May-October in Grasse). Key rotation happens at the start and end of harvest season.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Identity, tools, jasmine constraint |
| Brief ingestion | 1,500 | 300 | Once/task | Target character, season |
| Archive retrieval | 2,500 | 600 | 1/task | Historical formula search |
| Jasmine profile | 1,000 | 300 | 1/task | Harvest year characterization |
| Formula composition | 2,500 | 3,500 | Once/task | Ingredient selection + concentrations |
| Cost calculation | 800 | 400 | Once/task | Material costing |
| Commit message | 500 | 400 | Once/task | Formula, jasmine %, cost, margin |
| Quality check | 500 | 200 | Once/task | IFRA, allergens |
| **TOTAL (typical task)** | **14,300** | **7,700** | -- | 1 formula, ~12 ingredients |

## Unique Insight

A century of working with one ingredient has taught the Villeneuve family something that larger fragrance houses have forgotten: the ingredient is not a commodity. The jasmine absolute from the 2019 harvest is not the same as the jasmine absolute from the 2022 harvest. They have different chemical profiles, different olfactory characters, and different emotional qualities. A formula designed for the 2019 jasmine does not work with the 2022 jasmine — it must be adjusted. Our memory system captures this variation. When an agent proposes a formula, it retrieves not just historical formulas but the specific jasmine profile of the harvest year being used. The formula and its jasmine are inseparable. In version control terms: code is not independent of its runtime. A patch that works on one version of a dependency may fail on another. Our memory system treats ingredient variation as dependency versioning. The jasmine year is a dependency pin.

---

*"The jasmine changes every year. The standard does not."*
