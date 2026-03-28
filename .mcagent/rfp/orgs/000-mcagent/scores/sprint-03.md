# Sprint 3 Scores: Space Debris Tracking & Public Transit (041-060)

**Evaluator:** mcagent scoring team
**Date:** 2026-03-28
**Batch:** 041-060 (18 orgs; 041 and 044 do not exist)
**Tier 1:** 042, 043, 051, 053 | **Tier 2:** rest

---

## Tier 1 Proposals

### 042 — Orbital Threat Reduction Command
T:92 M:88 E:62 F:82 C:90 W:84.1
Five-agent kill chain (COMMAND/SENTINEL/STRIKER/OVERWATCH/KEYMASTER) with compilable-grade Rust `ForgeAdapter` trait, per-phase budget allocations, and programmatic tool-access enforcement per role. Orbital mechanics memory (LEO/MEO/GEO/Graveyard) with transparent conjunction-scoring formula, explicit promotion/demotion thresholds, and compaction rescue via high-RCS checkpoint entries. The `forge://` cross-repo URI scheme and Combined Operations Tracker YAML are fully specified. Budget is 66K tokens -- high but justified by the mandatory OVERWATCH review and 10% operational reserve. Military metaphor is load-bearing: separation of duties drives the architecture, not just the narrative. Deduction: token cost is above efficient range, and the five-agent structure is rigid for simple tasks. Bonus: +3 for DSN shared memory with multi-agent confirmation gates.

### 043 — The Lagrange Debris Observatory
T:90 M:85 E:58 F:80 C:82 W:81.3
Seven-agent observation pipeline (Cataloger through Sentinel-Ops) with Rust `ForgeAdapter` trait nearly identical to 042's. RCS-based memory with radar detection theory scoring (tag_match * rcs_normalized * confidence * recency * snr_factor) is the most formally specified retrieval model in the batch. Candidate/Confirmed/Deprecated lifecycle with SNR-weighted confidence buildup is genuine radar science applied to memory. 10% calibration overhead for confidence metrics at every stage is expensive but produces uncertainty-quantified outputs unique in this RFP. Budget is 70.8K -- the highest in the batch, penalized accordingly. The six-point signing validation with patch-integrity hash comparison (check 6) is a distinctive contribution. Cross-repo campaign log with per-session confidence tracking is well-specified. Deduction: -2 for seven agents being excessive for typical tasks; budget realism suffers. Bonus: +2 for SNR-weighted retrieval model.

### 051 — Fare-Free Federation
T:86 M:91 E:78 F:84 C:85 W:85.0
Transit-map memory is the standout: topological graph where stations are memory entries, lines are thematic connections, transfers are cross-domain links, and retrieval is shortest-path BFS traversal. This is genuinely different from every other memory proposal -- it exploits structural relationships, not just similarity. Storage layout with symlinks for lines is creative Git usage. The `ForgeAdapter` trait with `ForgeKind` enum (GitHub, GitLab, Bitbucket, Gitea) and `PrFilter`/`PrSummary` types demonstrates real forge-agnostic thinking. Budget at 35.5K is well within efficient range with per-component justification. Express routes for compaction survival are elegant -- compressed summaries designed for exactly the context-loss scenario. The "two-stamp rule" for irreversible decisions and the cycle-detection protocol for dependency graphs show operational maturity. Agent naming (Ligne, Correspondance, Titre, Reseau) maps cleanly to roles. Bonus: +5 for transit-map topology as genuinely novel memory architecture.

### 053 — Metropolitan Mobility Lab
T:85 M:87 E:76 F:80 C:80 W:82.2
Digital-twin memory with entity/relationship/dynamics models is the second most creative memory system in the batch. Exponential freshness decay (`f(t) = f_0 * e^(-lambda * dt)`) and Bayesian confidence updates (`c_new = c_old + (1-c_old) * alpha`) are formally specified. The key insight -- the twin lives in Git, the context window holds only a view -- is architecturally sound and solves the compaction problem cleanly via snapshots. State-machine agent loop (INIT -> OBSERVE -> PLAN -> EXECUTE -> SYNTHESIZE -> OUTPUT) with explicit error transitions is compilable-grade design. `Forge` trait is minimal and clean. Budget at 36.2K is realistic. The role-based authorization model (PI/researcher/technician) with per-role branch and patch-size constraints is well-structured. Deduction: -2 for the dynamics model requiring LLM-based tag extraction for retrieval, adding implicit token cost. Bonus: +3 for freshness-decay simulation model.

---

## Tier 2 Proposals

### 045 — OrbitalJanitor.io
T:62 M:55 E:74 F:60 C:65 W:63.0
Abbreviated proposal with five agents (Apogee, Perigee, Delta-V, COLA, Seal). Orbital decay memory model (GEO/MEO/LEO/Suborbital) is solid but underspecified compared to 042's version. The "minimum delta-v check" before every LLM call and "Kessler detector" for agent-pattern replication are interesting domain insights. Budget at 32.8K is lean. No Rust trait definitions for ForgeAdapter. Uses cosine similarity with embeddings for retrieval -- requires external embedding computation. Deduction: -2 for vague provider trait; -2 for "conjunction screening" coordination without concrete schema; -3 for no ForgeAdapter trait definition.

### 046 — Inter-Agency Space Object Registry
T:64 M:60 E:68 F:62 C:72 W:64.6
Registry-amendment metaphor applied consistently: memory entries have sequential IDs, amendment numbers, and supersession chains. Memory is never deleted (auditability), which is distinctive but storage-expensive. BM25 retrieval with recency weighting is pragmatic. The consistency ledger for polyrepo reconciliation is the strongest element -- detecting inconsistencies at merge time rather than preventing them is a genuinely useful insight. Budget at 35.1K with 8% audit overhead. RECONCILE agent concept is well-motivated from the cross-agency data format wars backstory. Deduction: -2 for vague provider trait; -3 for no concrete ForgeAdapter definition; -2 for monotonically growing memory without compaction. The 45-day comment period joke lands but the proposal needed more concrete trait definitions.

### 047 — Watchers of the Firmament
T:58 M:62 E:72 F:58 C:78 W:63.0
Contemplative observation cycle (Observation/Discernment/Action) is philosophically coherent but technically thin. Uncertainty tracking -- storing contradictory memories with cross-references and deferring resolution to humans -- is a genuinely novel memory feature. Vigil-based polling model for coordination is simple but effective. Budget at 29.8K is efficient. Trust accumulation for agents (visible but non-blocking) is an interesting identity concept. Deduction: -2 for vague trait definitions throughout; -2 for `confidence: certain/probable/uncertain` being too coarse compared to numeric scores; -3 for ForgeAdapter left unspecified. The interfaith stewardship narrative is the most compelling team story in the batch.

### 048 — Apogee Athletics
T:60 M:56 E:70 F:58 C:70 W:62.0
APR (Agent Performance Rating) system as core architectural principle is distinctive. "Match replay" memory with outcome-scored entries where high-value memories live longer is well-motivated. Provider leaderboard with exponential moving average for recent-form tracking is a practical idea. Budget at 31.3K is reasonable. Captain as sole signing authority with APR-gated signing (agents below 0.5 APR cannot get commits signed) is creative. Deduction: -2 for vague ForgeAdapter; -2 for "defensive zone" coordination lacking concrete schema; -2 for replay-value scoring formula not specified. The gamification insight (measurement changes behavior) is genuine domain transfer.

### 049 — Dominguez Orbital Services
T:56 M:58 E:72 F:52 C:82 W:61.2
Generational memory model (Current/Recent/Foundational mapped to Diego/Carmen/Esteban) is charming and functionally sound. Keyword match with recency and asset-proximity weighting is deliberately simple. Four agents with combined roles is lean. Budget at 30.3K is efficient. No embeddings, no vector search -- intentionally simple retrieval that Carmen can understand. Deduction: -3 for GitHub-only forge support (GitLab/Gitea only mentioned in passing); -2 for no ForgeAdapter trait; -2 for "shipping manifest" coordination lacking structured schema. The reliability-over-innovation philosophy produces a trustworthy but technically thin proposal. Best team narrative in the batch alongside Watchers.

### 050 — Phantom Orbit
T:66 M:64 E:68 F:60 C:72 W:65.2
Zero-trust security model is the strongest in the batch: signed coordination messages, encrypted memory at rest, reproducible builds, age-encrypted config. Yarkovsky's "drift" memory model (memories surface automatically based on similarity threshold) is an interesting passive-retrieval concept. Molniya's asymmetric polling (high-activity repos polled more frequently) is pragmatic. Budget at 32.4K is reasonable. The dependency-aware patch generation (mapping import graphs before reading files) is a concrete technical contribution. Deduction: -2 for drift model requiring continuous embedding comparison (implicit cost); -2 for encrypted memory preventing human inspection; -3 for no ForgeAdapter trait. Bonus: +2 for security-first coordination with signed messages.

### 052 — Transit Operations Command
T:62 M:55 E:66 F:58 C:74 W:62.0
Military PACE plan for providers (Primary/Alternate/Contingency/Emergency) is well-structured. FRAGO-based patch generation with defined area-of-operations constraints is operationally sound. Operational log memory with DTG formatting is consistent but not creative. Convoy model for coordination is functional but lacks concrete schema. Budget at 34.5K with 10% discipline overhead. The degraded-operation SOPs for every failure mode is the proposal's strongest contribution. Deduction: -2 for vague ForgeAdapter; -2 for keyword-only retrieval; -2 for operational log memory being essentially flat with classification buckets.

### 054 — Station Muse
T:54 M:60 E:68 F:56 C:70 W:59.6
Style-match scoring as an architectural principle is novel but niche. "Mood board" memory with aesthetic metadata (color hex codes, quality scores) is creative but the aesthetic dimensions add token cost without clear retrieval benefit. Provider selection biased by quality-for-patches and cost-for-everything-else is a pragmatic split. Budget at 32.3K is reasonable. Deduction: -2 for style-profile generation being vague on implementation; -3 for no ForgeAdapter trait definition; -2 for aesthetic memory dimensions not having a concrete scoring formula. The "beautiful code gets reviewed faster" insight is directionally correct but unquantified.

### 055 — RideStack
T:60 M:52 E:82 F:58 C:62 W:62.4
Speed-optimized architecture: preemptive draft PRs, async batch pre-signing, per-call provider routing, rapid iteration cycles. Transfer-point memory with cross-project retrieval is a useful feature (memories from other projects surface for current work). Budget at 28.3K is the second leanest. Pre-signed commit slots with 1-hour nonce expiry is creative but wastes slots. Deduction: -3 for no ForgeAdapter trait; -2 for "quick read" context loading potentially producing lower-quality patches; -2 for cross-project memory retrieval lacking a concrete similarity mechanism. The multimodal provider routing (different providers per call within a task) is a practical contribution.

### 056 — Department of Intermodal Transport Compliance
T:60 M:58 E:56 F:58 C:68 W:59.6
Specification-driven development: every patch references a requirement, every decision has a docket number, 7-year retention. The rulemaking process for coordination (NPRM -> Comment Period -> Final Rule) is bureaucratically coherent. Memory entries with cross-references and never-delete-within-retention is a compliance-first approach. Budget at 37.3K is the highest in Tier 2, with 20% output consumed by specification annotations. FIPS 140-2 compliant signing is distinctive for regulated environments. Deduction: -2 for 90-day comment period joke being load-bearing (the slow-by-design approach limits practical utility); -2 for monotonically growing memory; -3 for no ForgeAdapter trait. The "2.3 inches" anecdote perfectly illustrates why specification matters.

### 057 — Pilgrims' Route Cooperative
T:50 M:48 E:90 F:44 C:76 W:57.4
The most resource-constrained proposal: 4 agents, 21.9K token budget, keyword-only memory in plain text files, local-first providers, one-shot patch generation, binary under 10MB. No embeddings, no vector search, no iteration. GitHub-only forge support. Memory under 1MB. This is the accessibility proposal -- designed for a refurbished ThinkPad in Kibera. Deduction: -3 for GitHub-only (-3 forge penalty); -2 for no ForgeAdapter trait; -2 for one-shot generation limiting patch quality; -2 for keyword-only retrieval. The efficiency score is the highest in the batch because the constraints are genuine and the tradeoffs are explicit. The "floor not ceiling" philosophy is a valuable counterpoint to the sophisticated proposals. Bonus: +2 for demonstrating that the architecture works at minimum viable scale.

### 058 — Metro Sprinters
T:58 M:58 E:72 F:58 C:68 W:61.6
Delay-record memory (expected vs actual gap tracking) is the most operationally useful memory innovation in Tier 2 -- it builds a calibration model for future task estimation. Provider "suspension" after 10 consecutive low-quality calls with probationary reinstatement is well-specified. Budget at 30.5K is reasonable. Adherence-aware pacing (estimate completion time, then measure gap) creates a self-improving feedback loop. Deduction: -2 for no ForgeAdapter trait; -2 for "connection board" coordination lacking schema; -2 for delay records not specifying how expected-vs-actual gap informs retrieval scoring.

### 059 — Abramov Transit Group
T:52 M:50 E:78 F:44 C:82 W:57.8
Four-agent architecture with "proven pattern" reuse from memory is pragmatic. Logbook memory in plain text with keyword + asset-proximity retrieval is simple and legible. GitHub-only forge support. Budget at 25K is lean. Commit messages in plain English readable by non-technical family members is a distinctive design constraint. Deduction: -3 for GitHub-only; -2 for no ForgeAdapter trait; -2 for pattern-reuse approach not specifying how patterns are matched or adapted; -2 for no structured PR comment schema (plain language only). The generational reliability narrative is the second-best team story in the batch.

### 060 — Ghost Line Collective
T:64 M:68 E:68 F:60 C:72 W:65.4
Evidence-backed memory is the standout: every memory entry requires a `claim`, `evidence` (specific commits/files), and `confidence` (verified/unverified). Challenged memories (with contradictory evidence) decay at 3-day TTL vs 30-day for verified. This adversarial self-audit is the most rigorous memory integrity model in Tier 2. Defensive patch generation (threat modeling external inputs, adding validation) is a concrete quality contribution. Equity-aware coordination monitoring (tracking work distribution across repos) is novel. Full-chain signing verification (5 checks including provider billing verification) is thorough. Budget at 32.9K with 12% verification overhead. Deduction: -2 for no ForgeAdapter trait definition; -2 for "bunching" provider monitoring lacking concrete formula. Bonus: +3 for evidence-backed memory with challenge/decay model.

---

## Summary Table

| Rank | Org | Name | T | M | E | F | C | W |
|------|-----|------|---|---|---|---|---|-----|
| 1 | 051 | Fare-Free Federation | 86 | 91 | 78 | 84 | 85 | **85.0** |
| 2 | 042 | Orbital Threat Reduction Command | 92 | 88 | 62 | 82 | 90 | **84.1** |
| 3 | 053 | Metropolitan Mobility Lab | 85 | 87 | 76 | 80 | 80 | **82.2** |
| 4 | 043 | Lagrange Debris Observatory | 90 | 85 | 58 | 80 | 82 | **81.3** |
| 5 | 060 | Ghost Line Collective | 64 | 68 | 68 | 60 | 72 | **65.4** |
| 6 | 050 | Phantom Orbit | 66 | 64 | 68 | 60 | 72 | **65.2** |
| 7 | 046 | Inter-Agency Space Object Registry | 64 | 60 | 68 | 62 | 72 | **64.6** |
| 8 | 045 | OrbitalJanitor.io | 62 | 55 | 74 | 60 | 65 | **63.0** |
| 9 | 047 | Watchers of the Firmament | 58 | 62 | 72 | 58 | 78 | **63.0** |
| 10 | 055 | RideStack | 60 | 52 | 82 | 58 | 62 | **62.4** |
| 11 | 048 | Apogee Athletics | 60 | 56 | 70 | 58 | 70 | **62.0** |
| 12 | 052 | Transit Operations Command | 62 | 55 | 66 | 58 | 74 | **62.0** |
| 13 | 058 | Metro Sprinters | 58 | 58 | 72 | 58 | 68 | **61.6** |
| 14 | 049 | Dominguez Orbital Services | 56 | 58 | 72 | 52 | 82 | **61.2** |
| 15 | 054 | Station Muse | 54 | 60 | 68 | 56 | 70 | **59.6** |
| 16 | 056 | Dept. of Intermodal Transport | 60 | 58 | 56 | 58 | 68 | **59.6** |
| 17 | 059 | Abramov Transit Group | 52 | 50 | 78 | 44 | 82 | **57.8** |
| 18 | 057 | Pilgrims' Route Cooperative | 50 | 48 | 90 | 44 | 76 | **57.4** |

---

## Cross-Batch Observations

**Convergence:** All 18 proposals converge on PATH-based plugin discovery, `but-llm` wrapping without modification, INDEX.patch + COMMIT.msg as sole output, and Git-branch-based memory storage. ForgeAdapter as a trait abstraction appears in all Tier 1 proposals with near-identical method signatures.

**Divergence:** Memory architecture is the primary axis of differentiation. The four Tier 1 proposals represent four genuinely distinct approaches: orbital mechanics (042), radar detection theory (043), transit-network topology (051), and digital-twin simulation (053). The Tier 2 proposals cluster around simpler models (keyword + recency, flat archives) with occasional innovations (evidence-backed memory in 060, delay records in 058).

**Tier gap:** The Tier 1 proposals (81-85 weighted) are separated from Tier 2 (57-65) by roughly 18 points. The gap is almost entirely in Technical Soundness and Memory Creativity -- Tier 1 proposals provide compilable Rust trait definitions and formally specified retrieval algorithms, while Tier 2 proposals describe approaches without concrete interfaces.

**GitHub penalty:** Three proposals (049, 057, 059) support GitHub only, incurring the -3 forge-agnosticism penalty. All three are the smallest organizations in the batch -- resource constraints drive forge lock-in.

**Best synthesis candidates:** 051's transit-map memory topology + 042's ForgeAdapter trait + 053's freshness-decay dynamics + 060's evidence-backed integrity model would produce a memory system that is structurally rich, formally specified, temporally aware, and self-auditing.
