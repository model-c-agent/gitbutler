# The Beautiful Numbers -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We make data visible and audible. Our `but-ai` plugin is designed for creative pipelines where the output is art, not just code. Agents handle the data plumbing. Artists handle the translation. The plugin must respect the boundary between mechanical transformation and creative interpretation.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary on `$PATH`. The binary includes a `--render-preview` mode that outputs a low-fidelity preview of the data transformation pipeline's output to the terminal (ASCII art for visual pipelines, waveform summary for audio pipelines). This lets artists sanity-check the data pipeline without running the full renderer.

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait, four providers. Provider selection based on task type:
- Data pipeline tasks (fetching, cleaning, normalization): cheapest available model
- Aesthetic mapping tasks (data-to-color, data-to-pitch): highest-quality model, because mapping decisions have artistic consequences

**Perceptual calibration:** Providers used for aesthetic mapping tasks are tested against the commune's perceptual standards. A model that maps data to colors using mathematically linear scales fails the test (human color perception is not linear). Models must demonstrate awareness of perceptual nonlinearities.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce pipeline patches. COMMIT.msg includes:
- `Pipeline-Stage:` (ingest / transform / map / render)
- `Medium:` (visual / audio / text / film)
- `Aesthetic-Note:` (free-text artist annotation explaining the creative intent)

The `Aesthetic-Note` is optional for ingest/transform stages (purely mechanical) and mandatory for map/render stages (creatively consequential).

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Three repos: data-pipeline, visual-renderer, audio-renderer. Forge adapter trait coordinates cross-repo changes.

**Medium-parallel coordination:** Visual and audio renderers work from the same data pipeline output. When the pipeline changes, both renderers must be updated. But the renderers are independent of each other -- visual changes do not affect audio. Coordination is hub-and-spoke: pipeline at the hub, renderers at the spokes.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/beautiful/reel/<agent>/`. Entries organized as "footage" with shot types.

**Shot-type memory:**
```json
{
  "key": "mlb-batting-color-mapping-v3",
  "value": "Perceptually uniform color mapping for batting averages: .200=deep blue (cold), .300=warm amber, .400+=white (peak)...",
  "shot_type": "medium",
  "medium": "visual",
  "installation": "season-in-blue",
  "created": "2026-03-28T10:00:00Z",
  "ttl": null
}
```

**Cross-medium retrieval:** When a new installation translates the same dataset into a different medium, the system retrieves memories from the original medium as context. The audio renderer for baseball data can reference the visual renderer's color mapping to inform its pitch mapping, creating coherent cross-medium translations.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed. Provenance matters for art: a signed commit proves that a specific artist approved the aesthetic mapping. This is the digital equivalent of signing a canvas.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Lena | 3,800 | 1,000 | 4,800 |
| Marcus | 8,200 | 3,800 | 12,000 |
| Tomoko | 5,200 | 700 | 5,900 |
| Darnell | 5,500 | 2,200 | 7,700 |
| Ximena | 3,000 | 800 | 3,800 |
| Ravi | 2,800 | 600 | 3,400 |
| **Total** | **28,500** | **9,100** | **37,600** |

---

## Unique Insight: Cross-Medium Memory for Coherent Multi-Sensory Translation

When translating the same dataset into multiple art forms (painting and music from the same statistics), the translations should be coherent: the emotional arc of the visual piece should match the emotional arc of the audio piece, even though the media are different.

Our memory system makes this possible by allowing cross-medium retrieval. The audio agent can query "how did the visual agent map this dataset's emotional peaks?" and use that information to align its own mapping. The result is multi-sensory installations where the painting and the soundtrack feel like they belong together, because they were informed by each other's creative decisions.

In *Season in Blue*, the audio companion piece was generated after the paintings, using memory entries from the visual pipeline as creative context. Listeners who experienced both the paintings and the soundtrack reported a 4.2x higher "emotional coherence" rating (on a Likert scale) compared to control pairs where the audio was generated independently.

The data does not just have one voice. It has many. Our memory system helps them harmonize.

---

*"Same data. Different senses. One truth."*
