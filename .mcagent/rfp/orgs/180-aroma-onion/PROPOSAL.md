# aroma.onion — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a statically linked PATH binary. Reproducible build. No dynamic dependencies. Must run over Tor — all network calls (provider API, forge operations) route through a SOCKS5 proxy configurable via `BUT_AI_PROXY`.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai deformulate <sample-id>` — runs the full identification pipeline on a GC-MS data file: peak detection, spectral matching, confidence scoring, and preliminary identification as INDEX.patch.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`, `BUT_AI_PROXY` (SOCKS5 proxy for Tor routing).

WASI: library fallback. `thiol` is interested in WASI for sandboxed execution — ensuring that the analysis agent cannot access any data outside its designated sample.

MCP: standard tools plus `PeakIdentify`, `SpectralMatch`, `CostEstimate`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Provider calls route through Tor. This adds latency (~2-5x) but ensures that the provider cannot determine the caller's location or identity.

Two providers: Anthropic (via Tor, for complex identifications) and a self-hosted model on `ester`'s .onion server (for routine tasks). The self-hosted model is smaller but does not require trusting an external party with chromatographic data.

Provider abstraction adds: proxy routing configuration, request anonymization (stripping metadata from API calls), and response verification (checking that the provider's response does not contain tracking data in headers).

No dynamic loading. Compiled-in providers only.

## 3. The But Agent (RFP 3.3)

Agent loop: **ingest** (read GC-MS data file, detect peaks, segment chromatogram) -> **match** (for each peak: query spectral library, compute match scores, check confounder list) -> **rank** (produce ranked identification candidates per peak, with confidence) -> **diff** (produce INDEX.patch adding identifications to sample inventory) -> **cost** (estimate raw material cost based on identified compounds) -> **strip** (metadata removal: no identifying information in patch or commit message).

Every hunk:

```
+peak_031:
+  retention_time: 18.72
+  retention_index: 1268
+  identification: linalyl_acetate
+  cas: 115-95-7
+  confidence: 0.91
+  verification: library
+  confounders: [linalool (0.78), geranyl_acetate (0.65)]
+  cost_per_kg: EUR 28
```

Branch naming: `ao/<sample-id>/s<NN>`. Sample IDs are anonymised hashes.

Budget enforcement: `ketone`'s ceiling. No single analysis consumes more than 10% of the monthly provider budget.

## 4. Polyrepo PR Coordination (RFP 3.4)

Dual forge adapter:

1. Self-hosted Gitea (.onion) for internal coordination — full commit history, internal discussion, OpSec review.
2. GitHub (public mirror, throwaway account) for publication — only confirmed analyses, metadata-stripped.

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &AnonymousNote) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<AnonymousNote>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`AnonymousNote`: `type` (identification/cost/report/security), `body`, `group_signature`. No `from` field — all messages are from "the collective."

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/ao/memory/shared/`. No per-agent memory — the collective shares everything.

Compound library: `cas_number`, `name`, `mass_spectrum_top10`, `retention_index`, `frequency_in_analyses`, `typical_price_per_kg`, `confounders`, `verified` (boolean — has this identification been instrument-confirmed in at least two analyses?).

Analysis memory: `sample_hash`, `product_category` (eau de parfum, eau de toilette, etc.), `peak_count`, `identified_count`, `estimated_cost`, `publication_status` (preliminary/confirmed), `legal_status` (none/cease-and-desist-received/ongoing).

Retrieval: by compound (spectral similarity) or by product category. The `confounders` field is the most queried — before every identification, agents check what else the peak could be.

Relevance: no decay. The compound library is cumulative. Every verified identification makes the library more reliable.

Compaction: verified compound entries survive. Unverified entries are tagged for re-verification. Analysis summaries survive. Raw chromatographic data is excluded.

Identity: handles only. No biographical data. Identity is functional: `refs/ao/identity/<handle>` stores handle, role, group key membership, and nothing else.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Group signatures. Ring signature scheme where any member's key produces a valid signature indistinguishable from any other member's. No commit reveals which member signed it.

`thiol`'s flow: metadata strip -> content review (no identifying information) -> group signature via OpenWallet -> Tor-routed push.

Authorization: all members authorized for all branches. No per-member restrictions.

```toml
[group]
branches = ["ao/*"]
max_patch_lines = 500
members = ["benzyl", "aldehyde", "ester", "ketone", "thiol"]
```

Key rotation: 7-day cycle (aggressive — the collective's threat model assumes active legal adversaries). Compromise of any key triggers full group key rotation.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,200 | 0 | Once/session | Identity, tools, compound library config |
| Data ingestion | 1,500 | 200 | Once/task | Peak detection, chromatogram parse |
| Spectral matching | 3,500 | 3,000 | Once/task | ~30 peaks per sample |
| Confounder check | 1,500 | 500 | Once/task | Verify alternatives |
| Diff generation | 800 | 2,000 | Once/task | Identification inventory |
| Cost estimation | 1,000 | 500 | Once/task | Material pricing |
| Metadata strip | 300 | 100 | Once/task | Remove identifying info |
| Commit message | 300 | 200 | Once/task | Sample hash, peak stats |
| **TOTAL (typical task)** | **13,600** | **8,500** | -- | 1 sample, ~30 peaks |

## Unique Insight

Eighty-seven analyses have taught us that luxury fragrance formulations converge. The same twenty compounds appear in 70% of commercial perfumes. The remaining 30% — the compounds that make each fragrance distinctive — are often inexpensive naturals or specialty synthetics that cost less per kilogram than the base compounds. The myth of luxury perfumery is that rare, expensive ingredients make the fragrance special. The data shows the opposite: the base is expensive (high-quality musks, sandalwood substitutes, amber bases), and the distinctive top is cheap (a trace of a specific citrus, a wisp of an uncommon floral). Our compound library encodes this: the `frequency_in_analyses` field reveals which compounds are structural (shared across many formulations) and which are distinctive (unique to one or two). Agents that understand this distinction produce better identifications — they do not waste confidence on the structural compounds that are obvious and focus their precision on the distinctive ones that define the fragrance.

---

*"open the bottle. open the data."*
