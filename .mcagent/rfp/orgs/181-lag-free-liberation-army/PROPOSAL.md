# Lag-Free Liberation Army — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. Must run on gaming PCs (Windows and Linux) with zero perceptible performance impact — anything that adds input latency, even in a background process, will be removed with prejudice.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai review <demo-file>` — processes a match demo file and produces a tactical analysis as INDEX.patch, including positioning heat maps, rotation timings, and utility efficiency stats.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`.

WASI: library fallback. We want to embed tactical analysis in a browser-based team dashboard where all members can view analytics without installing additional software.

MCP mode: standard tools plus `DemoAnalysis`, `MatchQuery`, `StrategyCompare`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Dual provider: Ollama locally (routine — stat queries, formatting) and Anthropic (complex — tactical pattern recognition, strategy generation). Cipher manages the routing based on task complexity.

The collective insists on local-first processing for competitive intelligence. Match strategy data never goes to an external provider. The provider abstraction enforces this: tasks tagged `competitive` are routed to the local model regardless of capability. Only `analytical` and `public` tasks may use external providers.

Cost tracking per match — the collective budgets analysis costs against tournament prize pools, which is either shrewd financial planning or competitive rationalization depending on your perspective.

## 3. The But Agent (RFP 3.3)

Agent loop: **parse** (read demo file, extract round-by-round events) -> **analyze** (compute statistics: positioning, utility usage, economy management, engagement outcomes) -> **compare** (query memory for similar matches, identify pattern deviations) -> **recommend** (generate tactical recommendations as structured data) -> **vote** (present recommendations to the collective, require 3-of-5 approval before merge) -> **diff** (produce INDEX.patch adding the analysis to the team's tactical database).

The vote step is unique to the LFLA. Every INDEX.patch that contains tactical recommendations (as opposed to raw statistics) requires a collective vote before it can be merged. The vote is implemented as signed commit messages from at least three agents.

COMMIT.msg format:

```
analysis(match): vs Fokus, Ascent, 13-11 W

Rounds-Analyzed: 24
Key-Finding: B-site defense rotation 0.8s slower than opponent avg
Recommendation: shift Cipher anchor position 3m closer to site
Vote-Status: pending (0/3 required)
Captain: Specter (Map 1 rotation)
```

Branch naming: `lfla/<match-id>/s<NN>`. Match IDs are `opponent-map-date`.

Budget enforcement: Cipher's per-match cap. Analysis of one match cannot exceed 8% of the monthly budget.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &TacticalNote) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<TacticalNote>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`TacticalNote`: `from`, `match_id`, `type` (analysis/recommendation/vote/dissent), `body`, `classification`, `signature`. The `dissent` type is important — when a vote fails 2-3, the dissenting opinion is recorded and linked to the recommendation. If the recommendation later proves wrong, the dissent is vindicated. If the recommendation proves right, the dissent provides the counter-argument that strengthens conviction.

Cross-repo: strategy repo (competitive, restricted), analytics repo (team-internal), and content repo (public highlights and articles). Information flows from strategy to analytics to content, never backward.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/lfla/memory/<agent>/`. Match-centric.

Match memory: `match_id`, `opponent`, `map`, `date`, `result`, `rounds` (array of round-level stats), `captain_rotation` (who called which map), `key_findings`, `recommendations`, `vote_results`, `post_match_evaluation` (did the recommendations help in subsequent matches?).

Opponent memory: `team_name`, `matches_played`, `win_rate`, `preferred_maps`, `tendencies` (aggressive/passive, early-round/late-round, default-heavy/creative), `last_updated`.

Retrieval: by opponent for match preparation, by map for strategy review, by recommendation outcome for meta-analysis of the collective's decision quality.

Relevance: match data decays based on patch version — when a game update significantly changes the meta (weapon balance, map changes), pre-patch data is downweighted. The decay trigger is a `meta_patch` event that Cipher logs when a significant game update is released.

Compaction: opponent profiles survive. Match summaries survive. Round-level detail is excluded from compacted context.

Identity: `refs/lfla/identity/<handle>`. Handle, in-game role, competitive rank, vote history (how often each agent's votes align with the collective outcome), OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

All commits signed. Classification determines key: `competitive` uses a team-only key (never exposed publicly). `public` uses a separate key.

Vote implementation: tactical recommendations require 3-of-5 signed `Vote: approve` commits before the recommendation patch can be merged. Votes are tallied by the forge adapter. Ghost handles key management for both key tiers.

```toml
[agents.volt]
branches = ["lfla/*/analysis", "feat/*"]
max_patch_lines = 500

[agents.cipher]
branches = ["lfla/*/memory", "lfla/*/stats"]
max_patch_lines = 300

[agents.specter]
branches = ["lfla/*"]
max_patch_lines = 200
```

Key rotation: 21-day cycle. Competitive keys rotate before each tournament. Public keys rotate monthly.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Identity, tools, competitive context |
| Demo parsing | 2,000 | 500 | Once/task | Round extraction |
| Statistical analysis | 2,500 | 2,000 | Once/task | Positioning, utility, economy |
| Memory comparison | 2,000 | 500 | 1/task | Historical match retrieval |
| Recommendation generation | 2,000 | 2,500 | Once/task | Tactical proposals |
| Diff generation | 800 | 1,500 | Once/task | Analysis document |
| Commit message | 400 | 300 | Once/task | Match summary, findings |
| Vote coordination | 500 | 200 | Once/task | Vote collection |
| **TOTAL (typical task)** | **15,200** | **9,500** | -- | 1 match, 24 rounds |

## Unique Insight

Two years of collective competitive decision-making have taught us that the most reliable decisions are not the fastest — they are the ones with the highest quality dissent. When a vote passes 5-0, the recommendation is often obvious and the vote was unnecessary. When a vote passes 3-2 with a strong dissenting argument, the recommendation has been stress-tested. The dissent reveals the conditions under which the recommendation would fail. Our memory system tracks dissent as a first-class entity: every recommendation is stored alongside its strongest counter-argument. When an agent retrieves a recommendation, it also retrieves the dissent. This prevents the collective from adopting cached strategies without re-examining the conditions. A recommendation without a counter-argument is a recommendation that has not been thought through. In version control terms: a merge request without review comments is more suspicious than one with twenty. The absence of disagreement is not consensus. It is insufficient scrutiny.

---

*"The vote was 3-2. The dissent is on record. Ship it."*
