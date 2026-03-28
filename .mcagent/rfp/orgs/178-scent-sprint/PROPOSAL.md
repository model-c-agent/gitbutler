# Scent Sprint — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. Must run on competition tablets (Android via Termux is our edge case) as well as the coaching laptops.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai train <athlete-id>` — generates a personalized training session (10-20 compound sequence) based on the athlete's performance history and upcoming competition schedule.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`.

WASI: library fallback. We want to embed training plan generation in a browser-based athlete portal where athletes can view and start training sessions without installing software.

MCP mode: standard tools plus `TrainingPlan`, `AthleteStats`, `CompoundSimilarity`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Dual provider: Ollama locally (routine — stat queries, simple training plans) and Anthropic (complex — multi-week curriculum design, athlete comparison analysis). Rodrigo manages routing based on task complexity and remaining budget.

The provider abstraction adds athlete-level cost tracking: every token spent is attributed to an athlete ID, enabling Rodrigo to report cost-per-athlete to the board.

No dynamic loading. Two providers, compiled in.

## 3. The But Agent (RFP 3.3)

Agent loop: **assess** (read athlete profile, query performance memory, identify weaknesses) -> **design** (generate training session or curriculum update as structured data) -> **sequence** (order compounds by psychophysical difficulty, interleave diagnostic probes) -> **diff** (produce INDEX.patch updating the athlete's training plan) -> **document** (COMMIT.msg with athlete ID, session objective, target compounds).

Training plan patches are structured:

```
+session_2026-03-28:
+  athlete: ATH-0147
+  objective: terpene_discrimination
+  block_1: [linalool, linalyl_acetate, geraniol] x3 reps
+  block_2: [citronellol, geraniol, nerol] x3 reps
+  diagnostic: blind_5 [random from blocks]
+  deload: [bergamot_oil] x2 (maintenance)
```

Branch naming: `sprint/<athlete-id>/s<NN>`. Per-athlete branches.

Budget enforcement: Rodrigo's per-athlete cap. Training plan generation for one athlete cannot exceed 5% of the monthly budget.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &CoachingNote) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<CoachingNote>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`CoachingNote`: `coach`, `athlete_id`, `observation_type` (performance/technique/mental), `body`, `data_reference` (link to stats), `signature`.

Cross-repo: athlete training repos are separate from competition results repos. Training plan changes auto-reference upcoming competitions. Competition results feed back into athlete memory.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/sprint/memory/<agent>/`. Two stores: athlete performance history and compound knowledge base.

Athlete memory: `athlete_id`, `compound`, `session_date`, `identification_time_ms`, `correct`, `session_type`, `coach_notes`, `training_phase`, `competition_name` (if competition).

Compound memory: `cas_number`, `name`, `family`, `difficulty_rating` (aggregate across all athletes), `commonly_confused_with` (array of CAS numbers), `training_priority` (how often this compound appears in competition sets).

Retrieval: athlete queries return performance trends (time series of identification times for a given compound). Compound queries return difficulty and confusion matrices. The confusion matrix is unique to Scent Sprint — it captures which compounds athletes most frequently confuse with each other, directly informing training sequence design.

Relevance: athlete performance data does not decay (improvement trends require full history). Compound difficulty ratings update continuously as new athlete data arrives.

Compaction: aggregate statistics survive (per-athlete per-compound averages). Individual session records are excluded from compacted context.

Identity: `refs/sprint/identity/<agent>`. Name, role, coaching certification level, competition authority (can this agent's outputs be used in competition settings?), OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

All commits signed. Competition results are signed with enhanced verification — Noor's flow includes equipment calibration validation and judge attestation.

```toml
[agents.diallo]
branches = ["sprint/*/training"]
max_patch_lines = 300

[agents.faure]
branches = ["sprint/*"]
max_patch_lines = 100

[agents.yuki]
branches = ["sprint/*/competition"]
max_patch_lines = 200
```

Key rotation: 30-day cycle. During competition season, rotation pauses to avoid key changes mid-tournament (Noor's insistence — key rotation during a championship is "like recalibrating the olfactometer between rounds").

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Identity, tools, training methodology |
| Athlete assessment | 2,000 | 400 | Once/task | Performance history query |
| Compound analysis | 1,500 | 300 | Once/task | Difficulty, confusion matrix |
| Training design | 2,500 | 3,000 | Once/task | Session or curriculum generation |
| Sequencing | 1,000 | 800 | Once/task | Psychophysical ordering |
| Diff generation | 800 | 1,500 | Once/task | Training plan update |
| Commit message | 400 | 300 | Once/task | Athlete, objective, compounds |
| Coordination | 600 | 200 | 0.5/task | Coach-to-coach notes |
| **TOTAL (typical task)** | **13,800** | **8,500** | -- | 1 athlete, 1 session |

## Unique Insight

Competitive olfactory training has taught us that confusion is information. When an athlete confuses linalool with linalyl acetate, they are not making a random error — they are revealing the boundary of their perceptual resolution. The confusion matrix is the most valuable data structure in our system because it maps the edges of what each athlete can distinguish. Training that targets confusion boundaries produces faster improvement than training that targets entirely unfamiliar compounds. In version control terms: the most informative merge conflicts are not the ones between unrelated branches — they are the ones between branches that are almost identical. The near-conflict reveals the fine-grained distinctions that matter. Our memory system is optimized for near-misses, not misses.

---

*"You do not train the nose. You train the brain behind it."*
