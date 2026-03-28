# Sprint 7 Scores: Mining Engineering & Theater Production (121-140)

### 121 — Tunnelers' Free Assembly
T:52 M:58 E:52 F:62 C:60 W:55.6
Safety invariant system is a genuine and well-argued contribution — declarative rules checked at the diff level before commit, separate from tests. The mining analogy (check the air before blasting) lands. Memory as field reports with confirmed_by lists and safety-lesson entries that never expire is well-structured. Forge adapters name GitHub, GitLab, Gitea. But architecture is thin: no Rust trait code, no MCP integration, no WASI path. Cooperative voting mechanism is distinctive but adds token overhead. Budget at 37K is reasonable.

### 122 — Subsurface Operations Command
T:56 M:58 E:52 F:62 C:62 W:57.4
Planning as a separately budgeted phase is a strong insight — the OPORD format is concrete with phase IDs, per-phase budgets, decision points, and rollback plans. PACE provider methodology is well-structured. Intelligence product memory with four types and explicit expiration per type is clean. Cross-repo coordination OPORD with multi-venue message format is thoughtful. Forge adapters name GitHub, GitLab, Gitea, Forgejo. Budget at 39.9K with 28% to planning is honest about the cost of planning. But no Rust trait definitions, no MCP, no WASI keeps it Tier 2. Team narrative is strong with clear rank-to-role mapping.

### 123 — Deep Earth Geomechanics Lab
T:56 M:64 E:54 F:58 C:58 W:57.8
Reproducibility as a trust primitive is well-argued and technically differentiated — seed-based deterministic generation, complete run trace caching, and replay verification. Memory with confidence intervals, sample sizes, and diversity scores brings scientific rigor. The replication concept (periodic re-validation of memories against current codebase) with confidence decay for unreplicated entries is a strong mechanism. Regression benchmarks with tolerance specs are concrete. But no Rust trait code, no MCP, no WASI. GitHub listed as "primary" is a mild forge preference (-1). Budget at 40.5K + validation overhead at 46.5K is on the high side.

### 124 — Ore & Light
T:50 M:68 E:52 F:60 C:55 W:55.5
Geological strata memory (surface/bedrock/fossil) with promotion, fossilization, and trend tracking is the standout feature and deeply creative. The trend field with dated observation history is a non-trivial addition to memory. Failure archival as pattern mining is a genuine insight — studying the failure archive for recurring patterns rather than discarding failed runs. But architecture is thin: no Rust, no MCP, no WASI. Provider layer is deliberately minimal (single active, manual failover). Budget at 32.3K is lean for a 4-agent team. Forge adapters name three platforms.

### 125 — BoreStack
T:88 M:86 E:82 F:78 C:72 W:83.7
Tier 1 proposal. Compilable-grade Rust: `DrillProvider` with `DrillBudget`, `DrillResult` with `partial_patch`, `DrillBit` provider trait with `hardness_rating`, `ForgeAdapter` with 6 methods including `search_prs`. MCP server mode with backward compatibility for `gitbutler_update_branches`. WASI degradation path explicitly addressed per operation (survey/memory fully functional, background monitoring disabled). Core-sample memory with layered cylinders is deeply creative — 5 layer types (observation/change/design_decision/architecture/fossil) with depth-indexed retrieval. Four-factor relevance scoring (compositional 0.4, spatial 0.3, stratigraphic coherence 0.2, freshness 0.1) is non-trivial. Compaction with formation map (~1.5K tokens post-compaction). Phase-gated tool loading saves ~1.2K tokens. Budget at 28.85K is realistic, well-justified line by line with survey ROI argument. Token enforcement at 85% abort threshold with graceful partial-patch recovery. Cross-repo dependencies as "connected boreholes" with core_reference evidence chains. Testing covers round-trip, abort-at-depth, contaminated target, connected borehole, and budget accuracy (5% tolerance). Migration path is phased. The strongest proposal in this sprint.

### 126 — Office of Subterranean Resource Extraction
T:44 M:52 E:46 F:56 C:58 W:49.0
The permit-as-patch isomorphism is charming and well-articulated. Memory as filed documents with filing numbers, geological classification, retention schedules, and audit trails is coherent. The insight that memory access itself should be auditable is interesting. But architecture is thin: no Rust trait code, no MCP, no WASI consideration. Provider abstraction described in prose only. Forge adapters name GitHub, GitLab, Gitea but with minimal trait definition. Budget at 47K is the highest in this sprint and includes heavy inter-agent memoranda overhead. Director Mukherjee as sole signing authority is a bottleneck.

### 127 — Brothers of the Deep Vein
T:46 M:66 E:48 F:56 C:65 W:53.6
Liturgical cycle memory (advent/epiphany/lent) linking feature lifecycle phases is genuinely creative — temporal and structural retrieval from cyclical patterns is a novel contribution (+3). The Book of the Mine where pages are never deleted, only superseded, is well-motivated. The "foreman's halt" (agent stops on anomaly) is a practical safety mechanism. But architecture is very thin: no Rust code, no detailed trait definitions (RockFace trait is named but barely specified), no MCP, no WASI. Budget at 45K is on the high side. Solstice-based key rotation is charming. Team narrative is the strongest in this sprint — the tension between tradition and modernity is palpable.

### 128 — Drill Team Six
T:48 M:54 E:48 F:58 C:60 W:51.6
The handoff tax as a first-class budget item is a practical and well-argued insight — 3K tokens explicitly budgeted for relay transitions with per-run tracking. Film review (post-task analysis) for handoff optimization is concrete. Core tray memory (ordered sequences preserving spatial relationships) is a nice touch. But architecture is thin: Rig trait named but not defined in Rust, no MCP, no WASI. Budget at 45K is moderate. Forge adapter uses CompetitionFloor trait but with minimal specification. Branch dependency encoding follows GitButler convention, which shows awareness.

### 129 — Rasmussen Mining Works
T:48 M:62 E:56 F:56 C:62 W:54.6
Multi-generational memory with the `generation` field (independent confirmation count) is the standout contribution — high-generation memories trusted more in retrieval, low-generation included when budget allows. Five-generation philosophy applied to key management ("transferable to the next managing director") is principled. Memory depth categories (topsoil/subsoil/bedrock) are clean. Budget at 39.5K is lean with an efficient 4-agent team. But architecture is thin: no Rust code, no MCP, no WASI. Provider trait is extremely minimal. Forge adapters name three platforms. Team narrative is excellent — the 139-year mining family is the most compelling character work in this sprint.

### 130 — 0xMineral
T:50 M:64 E:56 F:56 C:60 W:55.4
Spectral memory model with wavelength/intensity/absorption is creative — the absorption field (contradictions as first-class citizens) is a genuinely useful insight for agent memory. Provider verification (checking response metadata for internal consistency) is practical. 3-of-4 multisig signing with counter-signing workflow is the most rigorous signing protocol in this sprint. Encrypted memory at rest with session-key derivation from DID is technically interesting but overkill for most use cases (as they acknowledge). But no Rust trait code, no MCP, no WASI. Budget at 40.5K is reasonable. Forge coordination with signed messages is security-conscious. The dead-drop metaphor works well for forge abstraction.

### 131 — Ensemble Without Directors
T:46 M:60 E:42 F:58 C:62 W:51.0
Preserved dissent as first-class artifact is the key insight — counter-notes stored alongside decisions in memory, surfaced on retrieval. The deliberative process (3-of-4 vote on approach before coding, dissent recorded) is principled. Threshold signing (3-of-4 key shares) is cryptographically coherent. Dramaturgical notes with counter_notes array are well-structured. But deliberation costs 4.5K tokens overhead, pushing total to 45.5K — expensive. No Rust code, no MCP, no WASI. The deliberation step itself is a token-efficiency concern. Forge adapter uses Stage trait with three platforms. The cue format for cross-repo coordination is minimal.

### 132 — Stage Operations Command
T:50 M:56 E:48 F:62 C:60 W:53.4
Acknowledgment protocol as verification is a well-argued insight — 500 tokens prevents 15K-token re-runs from misunderstood instructions. Cue-based memory with REPLAY capability for post-mission debrief is practical. SITREP commit message format is well-structured. Military comms discipline maps cleanly to inter-agent coordination. Forge adapters name GitHub, GitLab, Gitea with Venue trait. But no Rust code, no MCP, no WASI. Budget at 44.5K is on the high side. Memory classifications (INTEL/DECISION/HAZARD/OUTCOME) are clean but retrieval by mission/classification/agent is standard.

### 133 — Dramaturgy & Cognition Lab
T:50 M:68 E:48 F:56 C:58 W:54.4
Prediction error as memory retrieval signal is the standout idea and is deeply creative (+3) — surfacing memories that surprised the agent rather than those that confirm. High-confidence, high-prediction-error entries are "confirmed surprises" that prevent confident-but-wrong patches. Provider calibration (brief test prompt before use) is practical. Memory replication via replicated_by references adds scientific rigor. But no Rust code, no MCP, no WASI. Academic abstract format for COMMIT.msg is verbose. Budget at 43.5K includes 3.5K calibration overhead. Forge adapters name three platforms with StudySite trait. Protocol version in coordination comments is a nice versioning touch.

### 134 — Moth & Flame Theater
T:86 M:88 E:78 F:78 C:70 W:81.8
Tier 1 proposal. Near-compilable Rust: `ProductionProvider` with `CueSheet` and `ShowBudget`, `SceneResult` with `blocking_notes`, `CastMember` provider trait with `dramatic_range`, `VenueAdapter` with 6 methods including `search_scenes`. MCP server mode with backward compatibility. WASI degradation explicitly addressed (sequential cues, reduced venue detection). Script/cue memory is the most creative memory architecture in this sprint — three annotation layers (blocking notes for spatial relationships, cue sheets for retrieval triggers, rehearsal marks for reliability) compose into a deeply expressive system. Proactive retrieval via condition-based cue triggers is a genuine innovation over reactive search. Rehearsal marks that require explicit confirmation (not just retrieval) add signal integrity. Four-factor relevance scoring (cue match 0.4, blocking proximity 0.3, rehearsal depth 0.2, freshness 0.1). Production bible for cross-session knowledge. Phase-gated tool loading per act saves ~1K tokens. Budget at 29.55K is realistic and well-justified by act. Intermission reserve (8K for House operations) is smart budget design. Cross-repo "blocking notes" explain the nature of dependencies, not just their existence. Testing covers cue sequencing, scene split, curtain-at-budget, touring, and per-act budgets. Migration path staged as three acts.

### 135 — StageCraft Pro
T:48 M:52 E:56 F:58 C:55 W:52.0
Write-time relevance scoring is the standout insight — zero-token retrieval cost, sub-millisecond latency, at the cost of precision. The tradeoff is honestly acknowledged. Streaming for latency-sensitive operations is practical. Cue-sheet memory (ordered sequence matching production workflow) is a lighter version of Moth & Flame's approach. Reads counter with TTL extension on access is a clean lifecycle mechanism. But no Rust code, no MCP, no WASI. Budget at 43K is moderate. Forge adapters name three platforms with sync-event schema. The startup pragmatism ("ship it, iterate tomorrow") is refreshing but produces a thinner proposal.

### 136 — National Theater Licensing Bureau
T:46 M:56 E:42 F:56 C:58 W:49.4
Memory "violations" (REVOKED status for known-bad memories) is a useful mechanism — preventing agents from repeatedly acting on incorrect information. The licensing lifecycle (ACTIVE/EXPIRED/REVOKED) with renewal counts and extended TTL for established entries is well-structured. 23-item compliance checklist is thorough but expensive. But no Rust code, no MCP, no WASI. Budget at 44K with 3.5K inspection overhead is high. Forge adapters name three platforms with Jurisdiction trait. The proposal reads more as process description than technical architecture.

### 137 — Players of St. Genesius
T:46 M:62 E:46 F:56 C:62 W:52.2
The "moral" field in memory is the standout contribution — storing transferable principles alongside observations, enabling cross-domain reasoning. "Centralization reduces error surface but creates single point of failure" is a principle, not a fact, and that distinction matters for agent retrieval. The examination model (examined memories get extended TTL) is clean. Narrative commit messages written as program notes are unconventional but well-argued. But no Rust code, no MCP, no WASI. Budget at 44.5K is moderate. Forge adapters name three platforms with Company trait. The ethical framework (craft as devotion, examination of conscience) is the most distinctive team voice in the theater batch.

### 138 — Ovation League
T:46 M:58 E:52 F:56 C:55 W:51.1
Retrospective scoring as memory feedback loop is a strong practical insight — post-task scoring (1-10) creates a performance model where high-scoring approaches are preferred. Pattern-based retrieval (abstract structure not specifics) enables transfer learning across domains. Latency awareness in provider tracking is practical. But no Rust code, no MCP, no WASI. Budget at 42K is moderate. Forge adapters name three platforms with Venue trait and improv-style "offers." The competitive improv framing maps well to agent iteration. Speed-to-first-patch philosophy (70% shippable on first attempt) is bold but untested.

### 139 — Teatro Marchetti
T:46 M:64 E:58 F:56 C:65 W:54.7
Repair memory is the standout contribution — recording what was broken, why, and how it was fixed creates a map of codebase stress points. The sketch field (structural description that outlives implementation changes) is a subtle and practical idea. The carving metaphor (iterative refinement with 1-2 passes) maps cleanly to patch generation. Budget at 38.5K is lean for a 4-agent team including Nonna Rosa as architecture counsel (+2 narrative charm). But no Rust code, no MCP, no WASI. Provider abstraction is deliberately thin. Forge adapters name three platforms with PostalService trait. The "nothing is pristine" philosophy applied to codebases is wise.

### 140 — Prompt_Injection Theater
T:50 M:56 E:56 F:56 C:55 W:53.0
Sanitization as a separate gate from review is a well-argued and practical insight — catching information leakage (secrets, debug prints, TODO comments) that reviewers miss. Side effects tracking in memory is useful for preemptive mitigation. Checksum verification on coordination signals prevents stale-reference failures. Self-verifying binary (manifest signature check on startup) is security-conscious. But no Rust code, no MCP, no WASI. Budget at 39K is lean. Forge adapters name three platforms with DeadDrop trait. Immutable signals with amendment-by-reference is clean. The penetration testing metaphor is on-brand but the "injection" naming may confuse security auditors.
