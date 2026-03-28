# Sprint 8 Scores: Library Science & Forensic Accounting (141-160)

Note: Orgs 141-143, 153-155, 157-160 have no PROPOSAL.md on disk (they appear only as evaluators in the sprint-08 evaluation file). Scores below cover the 10 orgs with submitted proposals.

---

### 144 — Spine & Gilt
T:62 M:72 E:70 F:68 C:78 W:67.5
Solid card-catalog metaphor with condition-degrading memory (mint/good/foxed/brittle) -- a genuinely novel expiration gradient that avoids binary valid/expired. Provider trait sketched but not in Rust syntax. Forge adapter uses structured JSON comment schema with cross-repo coordination but only 4 forge targets named without concrete trait definition. Five-agent team with clear role mapping to the commune's workshop structure. Token budget of 36,000 is realistic. Deductions: no Rust trait signatures (-4), memory schema has moderate fields but no retrieval algorithm specified beyond "top-5 by relevance weighted by condition," forge adapter is described but not defined.

### 145 — ShelfOS
T:88 M:95 E:80 F:82 C:85 W:87.1
The strongest memory architecture in the batch and arguably the RFP field. Card-catalog memory with five simultaneous classification systems (subject, call number, source, temporal, relational), "see also" cross-reference graph with typed relationships (related_to, depends_on, contrasts_with), controlled vocabulary normalization, and circulation tracking. Retrieval scoring is a concrete 5-factor model with explicit weights (0.35/0.25/0.20/0.10/0.10). Crate structure in `crates/but-ai/` with CLI mode and MCP server mode. Phase-gated tool loading saves ~1,000 system prompt tokens. LibraryProvider wrapper with actual Rust struct and method signatures. CollectionAdapter forge trait with 6 methods. Interlibrary loan metaphor for cross-repo coordination with call-number-classified messages. WASI degradation table is well-specified. Budget of 26,000 tokens is lean for a Tier 1 with per-component justification. Deaccession protocol with archival retention. Union catalog for cross-session shared memory. Compaction survival via circulation-count tiering. The finding aid concept (natural-language orientation before task execution) is a unique contribution. +5 metaphor bonus for five-classification-system producing retrieval quality. +3 long-term storage for union catalog. +2 novel compaction for circulation-based tiering.

### 146 — Office of Library Metadata Standards
T:60 M:58 E:68 F:72 C:72 W:63.9
The compliance engine -- schema validation on every artifact -- is a distinctive and practical contribution. Encoding validation catching CJK mojibake from Ollama is a real-world insight. Provider trait with 4 methods including health_check. Forge adapter supports 4 forges with a structured comment format using form numbers. Memory schema includes compliance_status (verified/unverified/disputed) which is a useful quality signal. Token budget of 32,300 is reasonable. Clearance-based signing authority (only Director signs final commits) is well-motivated. Deductions: no Rust trait syntax (-4), memory schema has fewer fields than peers, the "schema-validated everything" insight is valuable but the actual rule engine is not specified, compliance status on memory entries is creative but the retrieval mechanism is underspecified.

### 147 — The Scriptorial Order
T:60 M:70 E:68 F:68 C:75 W:66.1
The colophon trailer -- who, when, why, under what conditions, with what confidence -- is a genuinely valuable commit metadata proposal that costs almost nothing to implement. The no-delete memory principle (entries closed, never deleted, with closing notes) is philosophically coherent and practically sound for audit trails. Codex/folio terminology maps cleanly to namespace/entry. Provider legibility check (truncation detection with retry) is a practical addition. Forge adapter with structured comments across 4 forges. The 10-year intelligibility criterion for commits is aspirational but shapes design usefully. Token budget of 34,200 with honest acknowledgment that they optimize for sufficient context over minimal tokens. Deductions: no Rust trait syntax (-4), memory schema has moderate fields but no retrieval scoring algorithm, forge adapter is described but not defined, the no-delete principle trades storage cost for completeness without specifying compaction.

### 148 — Dewey Decimators
T:62 M:52 E:85 F:68 C:70 W:65.1
The leanest budget at 28,900 tokens, justified by competitive discipline. Time-bounded task execution with wall-clock deadlines is a unique and practical contribution -- partial results on timer expiry with priority-based subtask selection. Heat-based memory aging (by task count, not clock time) is a moderate but coherent innovation. Provider tested for dynamic switching (found overhead exceeded savings) -- the testing-and-rejecting is credible engineering. Forge adapter with structured JSON across 4 forges. Signing authority model is clean (Hector signs only what Hector has verified). Deductions: memory schema is thin (5 fields), heat-based organization is novel but retrieval is just "top-3 by relevance" with no scoring algorithm, memory aging by heat count is interesting but under-specified, no Rust trait syntax (-4). The efficiency score is the highest in the batch.

### 149 — Okafor Library Services
T:58 M:65 E:72 F:62 C:72 W:63.5
Location-scoped everything is the core insight -- same agent behaves differently in different repos because each serves a different community. Per-repo config override via `.but-ai.toml` is a practical addition. Multi-location memory with city-level namespacing and cross-city isolation prevents convention leakage. Split signing authority (Adaeze for catalog, Tobias for infrastructure) mirrors real organizational boundaries. Provider selection per-repo reflects actual multi-site deployment. Token budget of 29,500 is moderate. Deductions: no Rust trait syntax (-4), memory schema has moderate fields but retrieval rules are simple (local + shared, no scoring), forge support says "GitHub (primary)" which hints at GitHub-centric assumptions (-2), the Location: trailer in commits is useful but the multi-location model is more about deployment topology than architectural innovation.

### 150 — grep_the_stacks
T:68 M:75 E:88 F:78 C:80 W:74.7
The most production-tested proposal -- they already run AI agents producing INDEX.patch + COMMIT.msg on an 11-million-entry catalog across 14 mirrors. Content-addressed memory deduplication via content-derived hashes is the standout insight: automatic deduplication across mirrors, sync as set-difference, integrity via hash verification. Reproducible builds with pinned Cargo.lock and hash verification for mirror trust. Pseudonymous signing via Verifiable Credentials that prove role and collective membership without personal identity. Forgejo-primary forge support (12 mirrors) with Gitea and GitHub adapters -- genuinely forge-agnostic by practice, not just claim. Token budget of 27,000 is tight but justified ("donated API credits"). Deductions: no Rust trait syntax (-4), memory schema has moderate fields (5), retrieval is simple "top-3 by relevance" with confidence threshold, the content-addressing insight is powerful but the retrieval scoring is underspecified.

### 151 — Ledger Liberation Front
T:64 M:72 E:70 F:62 C:78 W:67.5
Evidence-class tagging (primary/secondary/derivative) on every artifact is a genuinely valuable contribution for AI-human provenance chains. Case-isolated memory with firewall and explicit authorization for cross-case transfer addresses a real legal requirement. Dual-key signing (agent key + case key) is unique and well-motivated for multi-case forensic work. Air-gapped Ollama for sensitive cases is a practical design constraint that shapes the provider abstraction. Reproducible builds for evidence admissibility. The `privileged` flag on coordination messages prevents case-sensitive information leakage to public repos. Token budget of 31,500 is reasonable. Deductions: no Rust trait syntax (-4), does not use Bitbucket (-1 minor), memory schema has moderate fields but retrieval mechanism not specified, forge support says 3 forges (no Bitbucket), evidence-class distinction is the strongest insight but the implementation is described, not defined.

### 152 — Financial Crimes Investigation Unit
T:65 M:68 E:68 F:60 C:75 W:66.1
Checkpoint-and-resume for long-running agent operations is the standout contribution -- intermediate INDEX.patch files at configurable intervals enable recovery without restart. This directly addresses a real limitation of most agent frameworks. FIPS 140-2 validated cryptography is non-negotiable for their domain and well-motivated. Case-classified memory with UNCLASSIFIED/RESTRICTED/CONFIDENTIAL levels and absolute firewall on evidence. Cross-case method sharing (distinct from evidence) is a practical nuance. Graph traversal across millions of transactions is a credible use case that tests the plugin architecture's limits. Token budget of 30,900 with "CRITICAL cases operate with unlimited budget and post-hoc accounting" is honest about real operational needs. Deductions: no Rust trait syntax (-4), forge support says "does not use Bitbucket" and uses "air-gapped Gitea" which shows real multi-forge practice but GitHub is still primary (-1), memory retrieval mechanism not specified, the military metaphor (SIGMA callsigns, watch rotation) shapes architecture coherently.

### 156 — Office of Forensic Financial Compliance Review
T:90 M:82 E:75 F:88 C:88 W:85.5
The most technically complete proposal in the batch. Full Rust trait signatures: `ForgeAdapter` with 6 methods, `ProviderPlugin` C-ABI struct with 5 function pointers, `PrReference` struct. Detailed crate structure (`crates/but-ai/`) with CLI mode and MCP server mode. Explicit WASI degradation table. Four-agent sequential review chain (Vasquez produces, Chen audits, Park approves, Webb maintains) with honest 17% token overhead justification. Audit-trail-as-memory is the core insight: every memory access is logged, the trail itself is searchable, and access patterns become a relevance signal (case linkage). Four-factor relevance scoring (keyword 30%, case linkage 25%, access recency 25%, access frequency 20%). TTL by category (context 7d, finding 90d, pattern 180d, precedent 365d). Compaction survival via three-step rehydration (summary + trail + relevant memories). Cross-repo memory via forge reference (not copy). Token budget of 33,700 with per-component breakdown including frequencies. Full testing strategy with mock provider and mock forge. Nine trade-offs explicitly considered and rejected with rationale. The PR comment schema with HTML comment markers for structured/human dual-readability is well-designed. The authorization policy model (policy-based, not key-based) with 6-step verification chain is the most sophisticated signing architecture. Deductions: C-ABI provider plugins are fragile cross-platform (-2), the sequential review chain is expensive for routine tasks but honestly acknowledged.

---

## Summary

| Rank | Org | W |
|------|-----|------|
| 1 | 145 — ShelfOS | 87.1 |
| 2 | 156 — Office of Forensic Financial Compliance Review | 85.5 |
| 3 | 150 — grep_the_stacks | 74.7 |
| 4 | 144 — Spine & Gilt | 67.5 |
| 4 | 151 — Ledger Liberation Front | 67.5 |
| 6 | 147 — The Scriptorial Order | 66.1 |
| 6 | 152 — Financial Crimes Investigation Unit | 66.1 |
| 8 | 148 — Dewey Decimators | 65.1 |
| 9 | 146 — Office of Library Metadata Standards | 63.9 |
| 10 | 149 — Okafor Library Services | 63.5 |

**Tier 1 confirmations:** Both designated Tier 1 orgs (145, 156) scored in the top tier (85+). ShelfOS's five-classification memory system is the single most creative memory architecture in the RFP field. OFFCR's audit-trail-as-memory and full Rust trait signatures make it the most technically complete.

**Batch character:** This is a strong batch. The library science orgs share a common insight (agent memory is a cataloging problem) but differentiate on implementation depth. The forensic accounting orgs share a common constraint (evidence-grade outputs) but differentiate on operational model. grep_the_stacks stands out as the only org with a production system that already does what the RFP asks for.
