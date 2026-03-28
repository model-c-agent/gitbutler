# Constellation Weavers -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Medium:** Data as art, code as craft
**Date:** 2026-03-28

---

## Who This Proposal Is For

We are not infrastructure engineers. We are artists who build data pipelines because our art requires live data. Our proposal reflects the needs of small creative teams who use code as a medium: exhibitions with hard deadlines, data sources that break without warning, and output that is evaluated by human eyes rather than test suites.

---

## Requirement 1: PATH-Based Plugin Architecture

Binary on PATH. Configuration in TOML. The setup experience must be approachable — our team includes people who learned Git last year and are still uncomfortable with branches. `but-ai init` should produce a working configuration in under 60 seconds with three questions: provider, model, and project type.

We request a `project_type` config option with values like `data-pipeline`, `web-app`, `library`, `firmware`. The project type adjusts default behaviors: memory TTLs, review criteria emphasis, and commit message templates. For `data-pipeline`, the defaults should emphasize: data format validation, upstream schema change detection, and output consistency.

No daemon. We deploy to gallery hardware (usually a Mac Mini or NUC running Ubuntu) that must be reliable for weeks without intervention. Background processes that crash and require restart are unacceptable in a gallery context.

---

## Requirement 2: Provider-Agnostic AI

Four providers. We use Anthropic (Claude) for pipeline development because it handles Python well, and Ollama for quick iterations during exhibition setup when internet may be unreliable.

Trait: `init`, `complete`, `complete_with_tools`, `count_tokens`. We add a recommendation: the provider trait should accept a `persona` hint that tells the model the context of the work. For our use case, the persona is "data pipeline engineer for a visualization project" — this context improves the model's output for our specific tasks more than any prompt engineering we have tried.

Provider fallback: optional, enabled for us. Exhibition setup happens in venues with unreliable WiFi. Falling back to Ollama when Claude is unreachable is a practical necessity.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Line generates patches for the data pipeline. The workflow:

1. Color loads exhibition context and data source configuration
2. Line reads the task (usually: "upstream data format changed" or "add new data source")
3. Line generates INDEX.patch
4. Line generates COMMIT.msg:
   ```
   Adapt TLE ingestion for CelesTrak format change

   CelesTrak switched from 3-line TLE to OMM JSON on 2026-03-15.
   Updated parser to handle both formats with auto-detection.
   Output schema unchanged — visualization engine unaffected.

   Exhibition: cascade-v3 (Venice follow-up)
   Data-Source: celestrak.org/NORAD/elements/
   Visual-Impact: none (output unchanged)
   ```
5. Space reviews for correctness and visual impact
6. Frame signs

### Visual Impact Assessment

Every COMMIT.msg includes a `Visual-Impact` field: `none` (output unchanged), `minor` (small visual change, no audience impact), `major` (significant visual change, requires Theo's approval). This field allows the team to quickly assess whether a pipeline change affects the installation's appearance.

---

## Requirement 4: Polyrepo PR Coordination

Two repositories: `pipeline` (data ingestion and transformation) and `vis-engine` (visualization code, Three.js). Changes that affect the data-to-visualization interface require coordination.

PR comment schema:
```
[cw:link] pipeline@fix-celestrak vis-engine@update-data-loader visual-impact=minor
```

The `visual-impact` field in the coordination comment tells the vis-engine team whether they need to test visually or can rely on automated checks.

Forge: GitHub. Minimal trait.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/but-ai/memory/<exhibition>/<key>`. Organized by exhibition because each installation has its own data sources, its own visual engine configuration, and its own deadline.

| Memory Type | TTL | Example |
|-------------|-----|---------|
| Data source schema | Until source changes | CelesTrak OMM JSON field mapping |
| Exhibition config | Exhibition duration + 30 days | Venice cascade-v3 projection mapping |
| Pipeline lesson | 1 year | "CelesTrak changes formats without warning" |
| Visual decision | Exhibition duration | "Debris size exaggerated 10x for visibility" |

### Exhibition Memory Lifecycle

1. **Setup phase:** Memory is created rapidly as the team configures the installation.
2. **Exhibition phase:** Memory is mostly read-only. Pipeline changes during an exhibition are emergency fixes only.
3. **Archive phase:** After the exhibition closes, memory is archived. Key lessons are promoted to long-TTL entries.
4. **New exhibition:** Fresh namespace, previous lessons available by cross-namespace query.

---

## Requirement 6: Signed Commits via OpenWallet

Collective DID. All commits signed under the Constellation Weavers identity. We do not use individual signing because the work is collective — attributing a pipeline fix to one person misrepresents how the work happens.

Exhibition-ready commits receive a second signature: the `exhibition-ready` attestation from Space (after visual testing). Gallery hardware only pulls commits with both signatures.

Key rotation: 30 days. Low threat model — the worst case is someone pushing a pipeline change that displays incorrect debris positions, which Theo would notice immediately because he watches the projection obsessively.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Line | 9,000 | 6,000 | 15,000 | Pipeline patches |
| Space | 5,000 | 2,000 | 7,000 | Review |
| Color | 4,000 | 1,000 | 5,000 | Memory & context |
| Frame | 3,000 | 1,000 | 4,000 | Budget & signing |
| **Collective** | **21,000** | **10,000** | **31,000** | |

---

## Unique Insight: Output as Test Suite

Most software projects validate changes through test suites: unit tests, integration tests, end-to-end tests. Our data pipeline has tests, but the most important validation is visual: does the output look right?

This is not as subjective as it sounds. "Looking right" for a debris visualization means: objects are in approximately correct positions, trajectories follow orbital mechanics, collision effects are proportional to relative velocity, and the overall density of objects matches the current debris environment. Theo can assess all of this in under 5 seconds by glancing at the projection.

We propose that the `but-ai` plugin support a "visual test" concept: a structured tag in the review process that records whether the change has been visually verified by a human. This is distinct from automated testing — it captures the human assessment that no algorithm can replace.

For our use case, the visual test is literal: Theo looks at the projection. For other use cases, it might be: a designer looks at the UI, a data analyst looks at the chart, a security engineer looks at the audit log. The common pattern is human expert assessment of output quality, integrated into the commit trail so that future readers know whether the change was human-verified.

Art taught us that the final test is always a human eye. We believe software would benefit from admitting the same.

---

*Visualized. Verified by eye. Signed by the collective. The sky is watching.*
