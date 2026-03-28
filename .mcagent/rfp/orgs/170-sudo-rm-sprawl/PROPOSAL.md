# sudo rm -rf /sprawl — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a statically linked PATH binary. No dynamic dependencies. Must build reproducibly — `variance` insists on reproducible builds so that the binary hash can be verified. The binary should be distributable as a single file that runs on any Linux x86_64 system without installation.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai scrape-check` — verifies that a municipality's zoning data has not changed since the last extraction by comparing a hash of the source document against the stored hash in memory. This is the collective's most frequent operation.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`. We also read `BUT_AI_TOR_PROXY` for routing provider API calls through Tor when OpSec requires it.

WASI: library mode fallback. Useful for running extraction agents in sandboxed environments where the host system should not have access to the collective's key material.

MCP mode: standard tools plus `ExtractZoning`, `ValidateData`, `ScrapeCheck`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Provider abstraction wrapping `but-llm`. The collective uses three providers: Anthropic (for complex extraction), Ollama (for routine tasks on `setback`'s server), and a self-hosted model running on hardware donated by an anonymous supporter.

Provider selection: automated, benchmark-driven. `setback`'s router selects the cheapest provider whose accuracy on the zoning extraction benchmark exceeds 80%. Provider calls can be optionally routed through a SOCKS5 proxy (`BUT_AI_TOR_PROXY`) when the task involves scraping-adjacent operations that `variance` classifies as sensitive.

No shared library adapters. The collective will not load arbitrary code at runtime. Provider adapters are compiled in. Adding a new provider requires a recompile.

## 3. The But Agent (RFP 3.3)

Agent loop: **identify** (target municipality, locate source document, check scrape hash) -> **extract** (parse zoning PDF, generate structured data) -> **diff** (compare extracted data against existing dataset, produce INDEX.patch) -> **validate** (cross-check extracted values against known ranges) -> **commit** (COMMIT.msg with municipality, source URL, extraction confidence).

Every INDEX.patch includes: `Municipality:`, `Source-URL:`, `Source-Hash:`, `Extraction-Confidence:` trailers. The confidence score is derived from OCR quality metrics. Patches below 70% confidence are tagged `LOW-CONFIDENCE: manual verification required`.

Branch naming: `sprawl/<state>/<municipality>/s<NN>`. No dot dependencies — the collective's workflow is municipality-independent. Each municipality is an atomic unit.

Budget enforcement: `setback`'s hard cap. If a task exceeds budget, it stops and produces whatever partial extraction has been validated so far, tagged `PARTIAL: <N> of <M> parameters extracted`.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &DataMessage) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<DataMessage>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

GitHub implementation. The collective operates public repositories (the data is public; the operators are not). PR comments use a minimal schema: `type`, `municipality`, `parameters_affected`, `confidence`, `body`, `signature`. No names, no identifiers beyond the group signature.

Cross-repo: state-level repositories. A PR in the Georgia repo can reference a PR in the national aggregate repo. Dependencies flow upward (state -> national), never downward.

`variance`'s OpSec review: all PR comments pass through an automated scan that strips IP addresses, timestamps more precise than the day, and any text that matches patterns in a blacklist of operational details. The scan runs locally before the forge adapter posts.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/sprawl/memory/<handle>/`. Append-only. Entries are never modified.

Each entry: `municipality`, `parameter`, `value`, `unit`, `source_url`, `source_hash`, `scrape_date`, `confidence`, `supersedes` (previous entry key, forming a linked list of revisions), `verified` (boolean).

Retrieval: key-based by municipality and parameter. "What is the front setback for residential zones in Cobb County, GA?" returns the most recent non-superseded entry. Historical entries are accessible but require explicit version traversal.

No relevance decay. Zoning data does not become less relevant over time — it becomes less accurate if the municipality updates its code. Accuracy is managed through the scrape-check mechanism (comparing source document hashes), not through relevance scoring.

Compaction: only the most recent entry per municipality-parameter pair survives. Historical entries are preserved in the full memory branch but excluded from compacted context.

Identity: `refs/sprawl/identity/<handle>`. Handle (pseudonym only), role, authorization scope, group signature public key. No legal names. No biographical information. Identity is capability, not narrative.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Group signatures. Every commit is signed by an authorized collective member, but the signature does not reveal which member. This uses a ring signature scheme where any member's private key produces a valid signature that verifies against the group's public key set.

Flow: INDEX.patch + COMMIT.msg -> `easement` validates extraction -> `variance` performs OpSec review -> group signature via OpenWallet -> commit finalized.

Authorization: all members are authorized for all branches. There is no per-member branch restriction — the collective does not distinguish between members' authority. The group signature makes per-member authorization moot.

```toml
[group]
branches = ["sprawl/*"]
max_patch_lines = 500
members = ["parcel", "setback", "easement", "plat", "variance"]
```

Key rotation: 14-day cycle. Compromise of any single key triggers rotation of all keys (because group signatures mean a compromised key could produce a valid signature indistinguishable from any member's).

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Identity, tools, extraction config |
| Task identification | 1,500 | 200 | Once/task | Municipality, source document |
| Scrape check | 300 | 100 | Once/task | Hash comparison |
| Extraction | 4,000 | 3,000 | Once/task | PDF parsing, structured output |
| Validation | 1,500 | 300 | Once/task | Range check, cross-reference |
| Diff generation | 1,500 | 2,500 | Once/task | Dataset comparison + unified diff |
| Commit message | 400 | 200 | Once/task | Municipality, confidence, source |
| OpSec review | 500 | 100 | Once/task | Strip operational details |
| Coordination | 600 | 300 | 0.5/task | Infrequent — most tasks are independent |
| **TOTAL (typical task)** | **15,300** | **8,700** | -- | 1 municipality, ~20 parameters |

## Unique Insight

We have scraped zoning data from 1,400 municipalities. The single most consistent pattern: the data that is hardest to access is the data that is most damaging when published. Municipalities with the most exclusionary zoning practices are also the municipalities with the worst data infrastructure — broken links, scan-only PDFs, CAPTCHA-gated portals, records that are "available upon request" but never actually fulfilled. Accessibility is not a technical problem. It is a political strategy. Any agent memory system that treats data retrieval as a neutral operation is naive. The difficulty of retrieval is itself data.

---

*"$ chmod 644 /zoning/data && echo 'you are welcome'"*
