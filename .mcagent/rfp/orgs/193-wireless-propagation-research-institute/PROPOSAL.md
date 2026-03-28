# WPRI — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Uncertainty-Aware Prediction Agents

---

## Executive Summary

WPRI proposes a prediction agent system where every output carries an uncertainty estimate. Agents generate propagation predictions as versioned, diffable artifacts stored in Git. When the physical environment changes, stale predictions are automatically identified and regenerated. The system is designed for environments where agent predictions inform expensive infrastructure decisions and must be honest about their limitations.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary installs to `~/.gitbutler/bin/`. Petridou builds it in Rust. The binary structure reflects the prediction workflow:

- `but ai predict` — generate propagation prediction for a specified location
- `but ai validate` — compare prediction against field measurement
- `but ai stale` — identify predictions invalidated by environmental changes
- `but ai uncertainty` — display confidence intervals for existing predictions

The `stale` subcommand is central to the Institute's workflow. It compares the building geometry timestamp in the prediction's metadata against the current geometry data. If the geometry has changed (new construction, demolition, renovation), the prediction is marked stale and queued for regeneration. This ensures the prediction database never silently contains outdated information.

## Requirement 2: Provider-Agnostic AI

WPRI uses a hybrid provider strategy. Physics-based computations (ray tracing, diffraction modeling) run on the Institute's HPC cluster — no AI provider needed. Statistical predictions (gap-filling between measurements, material classification) use AI providers.

Provider selection: Anthropic for complex reasoning (material subtype classification from building metadata), Ollama for routine predictions (simple path-loss estimation). The provider interface: `predict(features, model_version) -> (prediction, uncertainty)`. The `uncertainty` return value is mandatory — the interface rejects providers that do not support uncertainty estimation.

Fallback: if the cloud provider is unavailable, the system falls back to Ollama with wider uncertainty bands (local models are less accurate, so the uncertainty estimate increases to compensate).

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The prediction pipeline:

1. Brenner's geometry agent processes building data for the target area
2. Tanaka's propagation agent generates predictions as structured JSON
3. Diallo's uncertainty agent annotates each prediction with confidence intervals
4. Predictions are committed as INDEX.patch modifying the coverage database:

```diff
+ {
+   "location": {"lat": 47.3769, "lon": 8.5417},
+   "frequency_mhz": 3500,
+   "predicted_rsrp_dbm": -78.4,
+   "confidence_80pct": [-82.1, -74.7],
+   "model_version": "wpri-prop-v3.2",
+   "geometry_timestamp": "2026-03-15T00:00:00Z",
+   "features_dominant": ["building_height", "material_metallized_glass"],
+   "validation_required": false
+ }
```

5. COMMIT.msg includes the prediction rationale:

```
Predict: 3500MHz coverage at Zurich North block 4

Model: wpri-prop-v3.2 (physics-informed, trained on 14M measurements)
Dominant-Features: building height (12m), metallized glass facade
Confidence: 80% interval spans 7.4dB — acceptable for planning
Stale-If: geometry changes within 200m radius
Geometry-Source: OpenStreetMap + municipal GIS, timestamp 2026-03-15
```

6. Marchetti reviews predictions flagged by Diallo (wide uncertainty or unusual feature combinations)

## Requirement 4: Polyrepo PR Coordination

WPRI maintains separate repos per city (prediction databases are large and city-specific). Cross-city coordination occurs when models are updated — a model improvement validated in Zurich should be propagated to other cities' prediction pipelines.

PR comments carry uncertainty metadata:

```
[WPRI:model-update] zurich-predictions#34 → barcelona-predictions
Model wpri-prop-v3.2 validated in Zurich (RMSE improvement: 2.1dB).
Propagation to Barcelona requires re-validation against local
measurements. Uncertainty bands will be 30% wider until Barcelona
field validation is complete.
```

Forge adapter supports GitHub (current). Petridou will add GitLab support when the ETH IT department completes their GitLab migration (estimated: 2027, realistically: 2028).

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/wpri/memory/<city>`. Memory types:

- **`measurement`**: Field measurement data indexed by location and frequency. TTL: permanent (measurements do not expire; the physical measurement is always valid for the conditions under which it was taken).
- **`prediction`**: Agent-generated predictions with uncertainty estimates. TTL: until the geometry timestamp is superseded.
- **`validation`**: Comparison of prediction vs. measurement. TTL: permanent. These entries are the feedback loop — they show where the model is accurate and where it fails.
- **`model-state`**: Serialized model parameters. TTL: until next model version.

Memory retrieval uses spatial indexing — given a location, retrieve all memories (measurements, predictions, validations) within a configurable radius. This is the one case where WPRI uses something beyond key-based lookup: spatial queries are essential for propagation analysis.

## Requirement 6: Signed Commits via OpenWallet

Each researcher has an OpenWallet DID linked to their ETH institutional identity. Key management follows ETH IT policy (annual rotation, institutional CA). Predictions that inform infrastructure investments exceeding CHF 100,000 require dual signatures: the generating agent and the validating researcher.

Commit signatures include a `Model-Version` header. If a model version is later found to have a systematic bias (like the metallized glass issue), all commits signed with that model version can be identified and their predictions flagged for re-evaluation.

**Unique insight:** WPRI's staleness detection system — automatically identifying predictions invalidated by environmental changes — solves a problem that affects any agent system operating on a changing codebase. When the code that an agent's patch was generated against has changed, the patch may no longer be valid. WPRI's approach of embedding a "staleness trigger" in each prediction (geometry_timestamp, stale-if conditions) can be generalized: every agent-generated artifact should declare the conditions under which it becomes invalid. This transforms the version control system from a passive record into an active validity monitor.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Marchetti | 1,500 | 500 | 2,000 |
| Tanaka | 4,000 | 3,000 | 7,000 |
| Diallo | 2,500 | 1,500 | 4,000 |
| Brenner | 2,000 | 1,000 | 3,000 |
| Petridou | 3,500 | 2,500 | 6,000 |
| Sundaram | 1,800 | 800 | 2,600 |
| **Task Total** | **15,300** | **9,300** | **24,600** |

Uncertainty quantification overhead: 3,000 tokens. Staleness checking: 1,500 tokens. Grand total per prediction task: **29,100 tokens**.

---

*"A prediction without error bars is not a prediction. It is a guess."*
— Prof. Marchetti, lecture to first-year students
