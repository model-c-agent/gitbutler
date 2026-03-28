# Sprint 2 Scores: Music Production & Agricultural Automation (021-040)

---

### 021 — Resonance Without Leaders
T:52 M:62 E:58 F:32 C:68 W:52.6
The jam session / competitive patch model is genuinely novel -- parallel execution with post-hoc mixing is a defensible coordination strategy. But the `Channel` trait has only 3 methods and uses music metaphors ("broadcast", "listen") that obscure PR semantics; no actual `ForgeAdapter` with create_pr/get_pr. The collective identity model (no per-agent memory, one shared key) is philosophically interesting but architecturally risky -- one compromised agent signs anything. Token budget is plausible (24K-35K range) but competitive overhead doubles cost with no guarantee of quality improvement. The vagueness penalty for "resonate" without mechanism is offset by the concrete `mix` algorithm description.

---

### 022 — Cadence Division
T:78 M:82 E:72 F:68 C:85 W:77.1
The strongest Tier 1 proposal. The tempo agent is a concrete coordination mechanism with real Rust trait (`ForgeAdapter` with 8 methods), per-beat budget enforcement, and a testable 4-measure performance cycle. Rhythm signatures for memory are the standout: `ostinato`/`theme`/`variation`/`bridge`/`coda`/`sforzando` categories with per-phase retrieval weighting and a 4-factor scoring formula. The fade-out TTL model (forte -> mezzo-piano -> pianissimo -> tacet) is the best expiration model in this batch. Token budget of 41,300 is well-justified with per-component breakdowns. Forge trait includes `open_performance`, `post_cue`, `read_cues`, `set_performance_status` etc. -- 8 methods, musical naming but forge-portable. The `ForgeType` enum acknowledges multiple forges. Minor deduction: `forge.type` defaults to "github" (-3 for assumption). The tempo metadata in signing is creative but adds complexity.

---

### 023 — Harmonic Analysis Group
T:55 M:44 E:62 F:45 C:52 W:52.0
Reproducibility-first is a defensible thesis but the proposal is thin on implementation. The provider trait has 5 methods (good) but no Rust code shown. Memory uses "spectral signatures" (keyword vectors) with cosine similarity -- standard RAG without the embedding model, which is sensible but not creative. The "calibration quarterly using held-out test sets" is an academic touch that demonstrates rigor but is operationally impractical for a plugin. No ForgeAdapter trait shown; cross-repo coordination is described abstractly with a JSON schema. Token budget of 40,500 is plausible. The variance-tracking unique insight is genuinely useful but adds 15% token overhead without clear ROI justification.

---

### 024 — Velvet Frequencies
T:76 M:78 E:68 F:72 C:78 W:74.7
Second strongest in this batch. The harmonic resonance memory system is deeply thought through: frequency/key/octave/harmonics/dissonances per entry, with a concrete scoring formula (`base_relevance * harmonic_multiplier * recency_decay * confidence`). The dissonance surfacing -- contradictory memories flagged rather than hidden -- is architecturally novel (+5 metaphor insight). `ForgeAdapter` trait with 8 methods shown in Rust, forge-agnostic cross-repo URI format (`forge://host/owner/repo#number`). YAML-in-HTML-comments for PR messages is well-designed. The recording session agent pipeline (SOUNDCHECK -> REHEARSAL -> TRACKING -> MIXDOWN -> MASTERING -> PRESSING) with per-phase budgets is concrete and testable. Token budget of 50,400 is on the high end but justified. The Overtone Archive for long-term storage and the compaction survival protocol (memory_checkpoint -> rehydration) are solid. Provider plugin mechanism via JSON-RPC over stdio is implementable.

---

### 025 — HitForge
T:48 M:30 E:55 F:28 C:45 W:41.8
Speed-first thesis is valid but the proposal is too thin. Speculative execution (begin patching before context is loaded) is a genuinely interesting optimization but described without mechanism -- how does incremental revision work? Memory is keyword-matching JSON blobs with aggressive TTLs; the "hit rate scoring" feedback loop is the only creative element but is described in one paragraph. No ForgeAdapter trait shown, just an HTML comment inline format. The forge section is 6 lines. Token budget of 42,000 is plausible but the speculative execution overhead claim ("~15% included") is hand-waved. No Rust code anywhere. Three "seamlessly" equivalents detected in the momentum argument (-4 vagueness). The proposal reads like a pitch deck, not a technical design.

---

### 026 — Federal Music Standardization Office
T:50 M:48 E:58 F:42 C:55 W:50.0
The auditability thesis is sound and the permanent audit trail namespace (`refs/but-ai/audit/`) is a concrete contribution. But the proposal uses SHALL-heavy specification language without implementation detail -- no Rust traits, no concrete memory schema, no scoring formula. The Forge trait has 4 methods (minimal but usable). Memory retrieval is keyword matching with "configurable relevance thresholds" -- no formula given. The `but-ai compliance check` command is a novel contribution that other proposals should adopt. Token budget of 34,500 is lean. The certification-as-CI unique insight is the strongest element. Deduction: forge defaults to "github" (-3).

---

### 027 — Schola Cantorum Machina
T:46 M:55 E:68 F:30 C:72 W:50.6
Beautiful narrative coherence -- the monastic hours mapped to agent names (Lauds, Vespers, Compline, Terce) is the most evocative agent naming in the batch. "Memory as attention" with 48-hour default TTL and conscious renewal is philosophically profound and operationally disciplined. Tag-based retrieval is simple but honest. The forge trait has only 3 methods and explicitly omits PR creation ("agents coordinate, they do not initiate") -- this is a design choice, not a gap, but limits capability. No Rust code shown. Token budget of 32,500 is lean and appropriate for local-model focus. The "silence as architecture" insight is architecturally valid -- sequential agents with artifact-based communication avoid coordination overhead. Forge implementation is GitHub-only (-3).

---

### 028 — BPM United
T:42 M:35 E:62 F:28 C:58 W:42.6
Pragmatic and self-aware but technically thin. "Formation as configuration" is a genuinely interesting meta-insight -- parameterizing the agent topology rather than hardcoding it. But the proposal lacks mechanism: the forge trait has 3 methods, no Rust code, memory is plaintext with a single header line, retrieval is tag overlap count only. The single-line coordination format (`[bpm:coord] from=... to=...`) is parseable but loses all structured metadata. No dependency DAG -- circular deps are "a tactical error." Token budget of 37,000 is plausible. The match-report memory pattern is useful. The football metaphor is fun but does not inform the architecture the way Cadence Division's tempo or Jade Terrace's elevation does. Forge is GitHub-only (-3).

---

### 029 — Nakamura Sound House
T:35 M:52 E:65 F:22 C:70 W:43.0
The most emotionally compelling proposal and the least technically detailed. The "documentation as survival" insight is deep and true -- agent memory as knowledge preservation, not just optimization. But the implementation is minimal: 3 agents, no Rust code, tag-based retrieval without scoring, 3-day default TTL, GitHub-only forge with 3 methods. Token budget of 28,000 is lean and honest. The scoped context approach (only read target files + immediate neighbors) is a practical optimization. No ForgeAdapter trait shown. The family narrative is powerful but does not shape the architecture in measurable ways. This is a heartfelt proposal for a simple tool, not a competitive technical design.

---

### 030 — Midnight Frequency Labs
T:50 M:58 E:60 F:38 C:62 W:52.0
Privacy-first is a legitimate design axis that no other proposal addresses. Encrypted memory (AES-256-GCM, Argon2id key derivation), prompt sanitization, metadata stripping, ephemeral DIDs -- these are concrete mechanisms. The cleartext-tags/encrypted-content split for retrieval is clever engineering. But the forge trait is 3 methods, the coordination comments are encrypted (which prevents human debugging), and the ephemeral DID model explicitly trades reputation for privacy. Token budget of 35,000 with 8% encryption overhead is plausible. The unique insight about privacy as a precondition for participation is the strongest philosophical contribution in the agricultural/music batch. Forge is GitHub-only by default (-3). No Rust traits shown.

---

### 031 — Fallow Earth Mutual Aid
T:40 M:48 E:60 F:35 C:62 W:45.6
The cross-platform translation insight is genuinely novel -- turning a patch for platform A into an equivalent for platform B, with a structured `Confidence` field. The offline-first design (queue tasks when disconnected) addresses a real constraint. Resource profiles (`full`/`constrained`/`minimal`) are a concrete contribution. But the proposal is thin on implementation: no Rust code, no ForgeAdapter trait, memory retrieval is tag-matching with "platform affinity" (described in one sentence). Token budget of 35,500 is plausible. The Gitea mention is good for forge-agnosticism but the actual implementation is GitHub-first (-3). The RPI ARM64 requirement is practical. The pseudonymous vs. named signing is a useful dual-mode design.

---

### 032 — Harvest Command Authority
T:48 M:55 E:60 F:30 C:60 W:49.4
Military planning metaphor is well-executed: OPORD/S2/S3/S4 staff sections map cleanly to agent roles, and the reliability classification for memory (CONFIRMED/PROBABLE/POSSIBLE) with automatic promotion through repeated observation is the best trust-based memory model in the batch. After-action intelligence reports create a genuine learning loop. But no Rust code, no ForgeAdapter trait, forge is 3 methods GitHub-only (-3). The "operational planning as architecture" insight is valid but the implementation does not demonstrate it beyond the metaphor. Token budget of 38,500 with 15-20% planning overhead is honest. Phase-based dependency model for cross-repo coordination is sound.

---

### 033 — Jade Terrace Institute
T:80 M:85 E:65 F:74 C:80 W:77.6
The strongest proposal in this batch. The terraced memory system is architecturally rigorous: 5 elevation levels (2000/1200/800/400/200), filtration rules controlling which elevations can see which memories, flow_rate per entry, and a concrete scoring formula (`tag_relevance * flow_rate * recency * confidence`). Elevation-based pre-filtering reducing LLM evaluations by 60-80% is a testable claim. The seasonal rotation (promoting high-observation entries up, demoting zero-observation entries down) is novel compaction (+2). `ForgeAdapter` trait with 8 methods shown in Rust. Cross-repo URI format (`forge://host/owner/repo#number`) is forge-agnostic. YAML-in-HTML-comments PR schema with 7 message types. The cascade pipeline (Watershed -> Aquifer -> Paddy -> Sluice -> Seal -> Channel) with a 10% fallow reserve is the most sophisticated budget model. Token budget of 60,150 is on the high end but justified by the cascade overhead. The elevation encoding in branch names enables structural filtering. Per-agent tool scoping. Complete git config keys table. Testing strategy is comprehensive.

---

### 034 — Seedsong Studio
T:38 M:42 E:55 F:28 C:55 W:41.0
The iteration-as-unit-of-work insight is valid for their domain but maps poorly to the but-ai plugin, where most tasks are single-pass. The `but-ai iterate` command is a useful addition but the proposal does not describe the agent loop in enough detail. Memory organized by season/plot is too domain-specific. Convergence tracking via shrinking deltas is interesting but not fleshed out. No Rust code, no ForgeAdapter trait, forge is 3 methods GitHub-only (-3). Token budget of 32,000 per iteration means a full cycle costs 160K-320K -- this is honest but raises cost concerns. The season-boundary memory archival is a form of compaction but is manual.

---

### 035 — CropOS
T:74 M:76 E:72 F:66 C:72 W:72.9
Strong Tier 1 contender. The soil-layer memory (topsoil/subsoil/bedrock) with active decomposition/composting is the second-best memory model in this batch after Jade Terrace. The composting process is concrete: topsoil (3d TTL, 2+ observations) -> subsoil (14d TTL, 5+ observations) -> bedrock (90d TTL). Nutrient-strength scoring with layer weighting (subsoil highest at 1.0) is well-reasoned. `ForgeAdapter` trait with 6 methods shown in Rust. Cross-repo refs use a flat format without `forge://` prefix -- shorter but less self-describing. PR schema is YAML-in-HTML-comments with 6 message types. The three-agent pipeline (Harvester -> Tiller -> Composter) eliminates coordination overhead. Token budget of 48,000 with "yield per acre" metrics is practical. The shared compost heap with high promotion bar (confidence > 0.85, observations > 5, 2+ agents) is excellent. Complete git config table. Migration path is refreshingly blunt. Deduction: forge defaults to github (-3).

---

### 036 — Bureau of Automated Cultivation
T:45 M:52 E:60 F:35 C:55 W:47.4
Similar to FMSO (026) -- auditability and certification focus. The retention classification (Operational/Reference/Certification/Permanent) with 7-year certification retention is a concrete contribution. Safety classification by modified file path patterns is useful. The `but-ai audit` command is a practical addition. But the proposal is thin: no Rust code, no ForgeAdapter trait (4 methods described), no memory scoring formula, no detailed agent loop. Token budget of 31,000 with 14% audit overhead is lean. The certification-as-CI insight overlaps with FMSO's. Forge is GitHub-only (-3). The dual-signature model for certification-tracked branches (CERT + REG) is sound.

---

### 037 — Benedictine Agribotics Fellowship
T:38 M:60 E:62 F:32 C:72 W:48.4
The "readings" memory type -- daily human observations transcribed to structured data -- is the most creative memory contribution in the agricultural batch. The bilingual workflow (French observations, English code) is a real constraint that others ignore. Gitea-first forge targeting is a forge-agnosticism positive (+3 vs the usual -3). But the proposal is technically thin: no Rust code, no ForgeAdapter trait, memory retrieval is not described, no scoring formula. Token budget of 30,500 is lean and hardware-appropriate. The vineyard context in commit messages creates genuine traceability. The community DID signing model is appropriate. The "human observation as irreplaceable context" insight is philosophically rich but does not produce architectural novelty beyond the `reading` memory type.

---

### 038 — Combines FC
T:44 M:42 E:58 F:25 C:50 W:43.0
Telemetry-driven estimation is a useful contribution -- measuring actual vs. predicted token usage to improve future estimates. The feedback loop (task produces telemetry, telemetry improves estimation model) is concrete. But the proposal is thin: no Rust code, no ForgeAdapter trait, forge is "minimal" with GitHub only, memory is season-bounded JSON with keyword telemetry fields. The pre-cached signing credential is a practical latency optimization. Token budget of 35,500 is plausible. The proposal reads as competent but undifferentiated -- the telemetry insight is the only standout. Two repos and one direction of dependency means the polyrepo section is barely exercised. Forge is GitHub-only (-3).

---

### 039 — Ebbesen Gard Teknik
T:32 M:55 E:62 F:22 C:68 W:42.4
The workshop notebook bridge -- digitized entries from a physical notebook with `source: notebook-p<page>` tags -- is a genuinely moving and practical memory contribution. The permanent TTL for hardware specs and the 1-year TTL for firmware history reflect real hardware lifecycles. But technically this is the thinnest proposal: 3 agents, no Rust code, no ForgeAdapter trait, forge is 3 methods GitHub-only, memory retrieval is unspecified beyond tag matching. Token budget of 26,000 is the smallest and is constrained by hardware (8K context on Ollama 7B). The honesty about limitations ("polyrepo coordination is not our strength") is refreshing. The 20% buffer for approximate local model token counting is practical. Commit messages written for 20-year readability is a design constraint that should inform all proposals. Forge is GitHub-only (-3).

---

### 040 — root@field
T:52 M:58 E:58 F:40 C:65 W:53.2
The safety review protocol -- safety region registry, automatic flagging, elevated review, hardware test requirement -- is the strongest safety contribution in the batch and should be adopted by any serious `but-ai` implementation. Encrypted memory with per-type encryption decisions (safety regions unencrypted, memory maps encrypted) is well-reasoned. Ephemeral DID + persistent pseudonym dual-mode signing is better designed than MFL's (030) version. The forge section targets Gitea first (+3) with an extension trait for Gitea-specific webhooks -- this is the correct pattern for forge extensibility. 47 repos across 12 manufacturers gives real polyrepo credibility. Token budget of 35,500 is plausible. But no Rust code shown, no formal ForgeAdapter trait. The prompt sanitization hook is a concrete privacy mechanism. The `NEEDS-HW-TEST` tag for patches is a practical workflow addition.
