# Seedsong Studio -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Installation:** Proposal as Process
**Date:** 2026-03-28

---

## Intent

We propose an implementation of `but-ai` that treats iterative design-execute-observe-revise cycles as a first-class workflow. Our planting systems iterate through many versions of a pattern before settling on the final design, and each iteration produces artifacts (code, data, observations) that inform the next. The plugin should support this cycle natively, not as an afterthought.

---

## Requirement 1: PATH-Based Plugin Architecture

Binary on PATH. Configuration in TOML. First-run setup creates defaults optimized for iterative workflows: short memory TTLs, generous output budgets (our agents produce more code than they read specifications), and a workspace state snapshot on every invocation.

We request one addition to the standard plugin interface: a `but-ai iterate` command that re-runs the last task with updated context (new sensor data, revised specifications, review feedback). This command is the core of our workflow -- we iterate 5-10 times on a pattern before finalizing, and each iteration should not require re-specifying the full task.

---

## Requirement 2: Provider-Agnostic AI

Four providers. We use Ollama locally for iteration speed (our Vermont internet is unreliable) and Anthropic for final-pass quality when we need high-fidelity output.

The provider trait: `init`, `complete`, `complete_with_tools`, `token_count`. We add a recommendation: the trait should include an `iteration_hint` parameter that tells the provider this is iteration N of M, allowing models that support it to adjust their behavior (e.g., be more creative on early iterations, more precise on later ones).

No automatic fallback. Iteration requires consistency -- switching providers mid-iteration sequence would invalidate our convergence assumptions.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Sow generates patches. The iteration workflow:

1. Root injects environmental context and previous iteration results
2. Sow reads the specification and context
3. Sow generates INDEX.patch (the pattern/code change)
4. Sow generates COMMIT.msg with iteration metadata:
   ```
   Iteration 3/8: Adjust row spacing for soil moisture

   Previous: sinusoidal spacing, period=20m, amplitude=0.3m
   Current: sinusoidal spacing, period=18m, amplitude=0.25m
   Delta: reduced amplitude due to high clay content in plot B3
   Environmental: soil moisture 34%, temp 18C, growth stage: pre-emergence
   ```
5. Bloom reviews
6. If another iteration is needed, the cycle repeats with updated context

### Convergence Tracking

Each iteration's COMMIT.msg includes a `Delta` field describing what changed from the previous iteration. Over a sequence of iterations, the deltas should shrink -- the design is converging. If deltas grow instead of shrinking, Bloom flags it as "divergence" and the commune reassesses the specification.

---

## Requirement 4: Polyrepo PR Coordination

Our planting systems span three repositories: `patterns` (SVG designs), `control` (motor scripts and waypoints), and `sensors` (calibration and data collection). A single installation requires coordinated changes across all three.

PR comment schema:
```
[seedsong:coord] pattern=patterns@fibonacci-v3 control=control@fib-waypoints sensors=sensors@fib-calibration iteration=3
```

The `iteration` field links coordination comments across repos to the same iteration cycle. This ensures that pattern changes, control changes, and sensor changes stay synchronized through the iteration sequence.

Forge trait: three methods. GitHub implementation.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/but-ai/memory/<season>/<plot>/<key>`. Organized by growing season and field plot because our context is spatially and temporally bounded.

| Memory Type | TTL | Example |
|-------------|-----|---------|
| Environmental reading | 6 hours | Soil moisture, temperature |
| Iteration result | Season end | Previous iteration parameters and outcomes |
| Hardware calibration | 90 days | Planter GPS offset, motor calibration |
| Pattern archive | Permanent | Completed installation designs |

### Season Boundaries

When a new growing season begins, Root archives the previous season's memory and starts fresh. The archive is accessible but not actively retrieved unless explicitly requested. This prevents old patterns from biasing new designs -- each installation should respond to current conditions, not historical ones.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed. We use a team DID that identifies the commune, not individual artists. This reflects our creative philosophy: the installation is a collective work, and attribution belongs to the group.

For iterations that are reviewed and approved for field execution, Bloom adds a second signature: the "field-ready" attestation. A commit with two signatures (commune + field-ready) is authorized for planter execution. A commit with one signature is a work in progress.

Key rotation: 30 days. Low rotation frequency because our threat model is benign -- the worst outcome of a key compromise is someone planting an unauthorized pattern in our field, which Ellis would notice within hours from the sensor data.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Sow | 9,000 | 6,000 | 15,000 | Pattern generation |
| Bloom | 5,500 | 2,000 | 7,500 | Review |
| Root | 4,500 | 1,000 | 5,500 | Environmental memory |
| Compost | 3,000 | 1,000 | 4,000 | Budget & signing |
| **Studio** | **22,000** | **10,000** | **32,000** | |

Per-iteration budget. A full design cycle (5-10 iterations) costs 160,000-320,000 tokens. The first iteration is the most expensive (full context load); subsequent iterations are cheaper (incremental context).

---

## Unique Insight: Iteration as the Unit of Work

Most proposals treat a "task" as the unit of work: receive task, produce patch, done. Our experience building kinetic art installations has taught us that the real unit of work is not the task but the iteration cycle.

A planting pattern is never right on the first try. The soil is different than expected, the GPS has a 2-inch drift, the wind shifted the seed distribution. Each iteration observes the result of the previous one and adjusts. The final pattern is not designed -- it is converged upon.

We believe code works the same way for complex tasks. The first patch is a hypothesis. The review is an observation. The revision is an adjustment. The final commit is the result of convergence, not of planning.

Our proposal builds iteration support into the plugin's core: iteration metadata in commit messages, convergence tracking through delta fields, season-bounded memory that prevents old iterations from polluting new ones, and a coordination schema that synchronizes iterations across repositories.

The field teaches patience. The algorithm learns from the field.

---

*Iteration 1 of 1. Convergence: pending review. Season: spring 2026.*
