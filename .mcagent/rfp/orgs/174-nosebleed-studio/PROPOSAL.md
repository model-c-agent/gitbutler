# Nosebleed Studio — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as PATH binary in `crates/but-ai/`. Must run on the Studio's field laptops (rugged Linux machines used during expeditions) as well as the lab workstation in Reykjavik.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai identify <chromatogram>` — processes a GC-MS data file and produces a structured compound inventory as INDEX.patch. This is our core use case.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`.

WASI: library fallback. Useful for running lightweight identification agents on field laptops with limited connectivity (many collection sites have no internet).

MCP mode: standard tools plus `PeakIdentify`, `CompoundQuery`, `ExpeditionMemory`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Dual-mode provider: online (Anthropic for complex peak identification) and offline (local model on the field laptop for preliminary identification during expeditions). The offline model is smaller and less accurate but allows field teams to begin analysis before returning to the lab.

Provider abstraction over `but-llm` with a `connectivity_mode` flag: when `offline`, tasks route to the local model. When `online`, tasks route by complexity — complex identifications (co-eluting peaks, novel compounds) to Anthropic, routine identifications (common terpenes, well-known aldehydes) to the local model.

No dynamic provider loading. The offline model is bundled with the binary.

## 3. The But Agent (RFP 3.3)

Agent loop: **ingest** (read chromatogram data, identify peak regions) -> **identify** (for each peak: query mass spectral library, match retention index, propose compound identification) -> **source** (link identified compounds to biological sources via Ama's annotation) -> **diff** (produce INDEX.patch adding identified compounds to the expedition's inventory) -> **document** (COMMIT.msg with expedition name, peak count, identification rate).

Each hunk in the INDEX.patch represents one chromatographic peak:

```
+peak_042:
+  retention_time: 12.42
+  compound: linalool
+  cas: 78-70-6
+  confidence: 0.89
+  mass_spectrum_match: 0.92
+  biological_source: Ocimum gratissimum (probable)
+  emotional_note: "green, herbal, alive"
```

The `emotional_note` is Sigrid's addition. It is not data. It is direction.

Branch naming: `nbs/<expedition>/s<NN>`. Expedition names are place-based: `nbs/scottish-peat-2025/s01`.

Budget enforcement: per-expedition cap. Lars allocates budget at expedition start. If budget runs out during analysis, remaining peaks are tagged `BUDGET-DEFERRED` and processed when funding is available.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &FieldNote) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<FieldNote>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`FieldNote` includes: `from`, `expedition`, `type` (identification/sourcing/editorial/compliance), `body`, `biological_source`, `nagoya_status`, `signature`. Cross-repo links connect the analytical repo (compound identifications) to the field repo (species lists, GPS data, photos) and the formulation repo (the perfume being composed from the identifications).

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/nbs/memory/<agent>/`. Two stores: the compound library (shared, growing across expeditions) and expedition memories (per-expedition contextual knowledge).

Compound library: `cas_number`, `name`, `molecular_weight`, `olfactory_descriptors`, `typical_sources` (organisms), `ecosystems_found_in`, `threat_level` (of the ecosystem, not the compound).

Expedition memory: `expedition`, `location`, `dates`, `ecosystem`, `threat`, `compounds_count`, `dominant_character`, `emotional_register`, `field_notes_excerpt`, `nagoya_status`.

Retrieval: compound library queried by molecular similarity or descriptor match. Expedition memory queried by ecosystem type and threat level. The threat-level index is unique to Nosebleed — it allows agents to prioritise knowledge from the most endangered ecosystems.

Relevance: compound profiles do not decay. Expedition memories do not decay either — a disappearing ecosystem's memory becomes more valuable over time, not less. There is no expiration. Forgetting is the enemy.

Compaction: compound library entries survive. Expedition summaries survive. Chromatographic peak data (too verbose) is excluded but re-retrievable.

Identity: `refs/nbs/identity/<agent>`. Name, role, field experience (expeditions participated in), conservation certifications, OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every commit signed. Katya's flow: Nagoya compliance check (was the sample legally collected?) -> conservation ethics check (does the commit inadvertently reveal the location of an endangered species?) -> Ed25519 signature via OpenWallet -> compliance trailers added.

```toml
[agents.takeshi]
branches = ["nbs/*/identify"]
max_patch_lines = 500

[agents.ama]
branches = ["nbs/*/source"]
max_patch_lines = 200

[agents.sigrid]
branches = ["nbs/*/editorial"]
max_patch_lines = 100
```

Key rotation: 45-day cycle. Revocation triggers review of all signed compound identifications (because incorrect identifications in the archive become permanent errors if uncorrected).

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Identity, tools, expedition context |
| Chromatogram ingestion | 2,000 | 300 | Once/task | Peak detection, region segmentation |
| Peak identification | 3,500 | 4,000 | Once/task | Mass spectral matching, ~20 peaks |
| Biological sourcing | 1,500 | 800 | Once/task | Species attribution |
| Memory query | 1,500 | 400 | 1/task | Compound library + expedition context |
| Diff generation | 1,000 | 2,500 | Once/task | Compound inventory update |
| Commit message | 500 | 300 | Once/task | Expedition, peak count, rate |
| Compliance check | 400 | 100 | Once/task | Nagoya + location check |
| **TOTAL (typical task)** | **15,900** | **10,400** | -- | 1 chromatogram, ~20 peaks |

## Unique Insight

The Studio has learned that the most important compounds in an ecosystem's olfactory signature are not the most abundant — they are the most distinctive. A forest headspace is 80% alpha-pinene. Every forest smells of alpha-pinene. What makes a particular forest's smell unique is the 2% of compounds that are specific to its species composition, soil chemistry, and microclimate. Our agents are trained to prioritise distinctive compounds over abundant ones. In version control terms: the most important changes in a patch are not the largest hunks — they are the hunks that differentiate this branch from every other branch. Distinctiveness, not volume, is the signal.

---

*"The last bottle is the most important one. After that, there is only the memory."*
