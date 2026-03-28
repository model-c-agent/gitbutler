# Bandwidth Blitz — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Competition-Grade Configuration Optimization

---

## Executive Summary

Bandwidth Blitz proposes an agent system designed for speed-constrained configuration optimization. Agents analyze historical deployment data to generate pre-staged configuration templates that minimize manual parameter entry during competitive tower deployments. The system balances speed with quality — a fast but broken configuration scores zero.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary installs to `~/.gitbutler/bin/`. Anya builds it in Rust. The binary operates in two modes: `studio` (pre-event template generation, full agent capabilities) and `field` (event-day use, read-only analysis, no mutations). Field mode exists because modifying the scoring repository during a live event is prohibited by league rules.

Subcommands: `but ai template` (generate configuration template from historical data), `but ai validate` (check template against equipment specs and deployment context), `but ai score` (compute deployment score from event data), `but ai coach` (real-time advisory during practice — not available on event day).

The `coach` subcommand is the league's most popular feature. During practice, it monitors deployment progress and suggests configuration adjustments: "You are 3 minutes behind pace on antenna alignment. Consider pre-computing azimuth from the site survey data." It runs on the team's field laptop and does not require network connectivity.

## Requirement 2: Provider-Agnostic AI

Template generation uses Anthropic (best structured output for configuration parameter sets). Coaching uses Ollama locally (no network at field sites). Scoring is deterministic — no AI provider needed.

Provider interface: `optimize(deployment_history, context, constraints) -> Template`. The `context` parameter is mandatory after the Grounding Incident — it specifies deployment type (permanent/temporary), environment (indoor/outdoor), and equipment model. Templates generated without context are rejected.

Fallback: if Anthropic is unavailable, template generation falls back to rule-based defaults (top-10 configurations from the historical database, unmodified). The league considers AI-generated templates a competitive advantage, not a requirement.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The template generation pipeline:

1. Sam's analysis agent ingests deployment history for the relevant equipment model
2. The agent identifies configuration parameters that correlate with high scores
3. The agent generates a template as INDEX.patch:

```diff
+ [deployment_template]
+ name = "small-cell-competition-v7"
+ equipment = "Ericsson AIR 6419"
+ context = "temporary, outdoor, competition"
+ channel = "auto-select"  # site-specific, set at deployment
+ tx_power_dbm = 23  # optimized for 200m radius
+ antenna_tilt = 4  # median optimal for parking lot venues
+ grounding = "portable-kit-required"  # safety compliance
```

4. Tasha validates field plausibility
5. COMMIT.msg:

```
Template: Ericsson AIR 6419 competition config v7

Based-On: 47 historical deployments, median score 82.4
Speed-Improvement: estimated 8 minutes saved vs. manual config
Quality-Baseline: 94% signal quality at deployment (p50)
Context: temporary outdoor competition deployment
Safety: portable grounding kit required (see Grounding Incident 2025)
```

## Requirement 4: Polyrepo PR Coordination

Repos per season (one repo per competition season, plus a shared `blitz-templates` repo for configuration templates). Cross-repo coordination handles template versioning — a new template in `blitz-templates` must be validated against the current season's scoring engine.

```
[BB:template] blitz-templates#v7 → season-2026#event-dallas
New template v7 validated against scoring engine v3.2.
Compatibility: confirmed.
Speed-delta: estimated -8 minutes vs v6.
Quality-delta: +2.1% signal quality (p50).
```

Forge adapter: GitHub (current and likely permanent — the league's audience is on GitHub).

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/blitz/memory/global`. Memory types:

- **`deployment-record`**: Complete deployment data (timing, configuration, scores, conditions). TTL: permanent. The league's historical database is its primary asset.
- **`template-outcome`**: Which templates were used at which events and how they scored. TTL: permanent. Essential for template optimization.
- **`equipment-profile`**: Equipment-specific configuration characteristics and known issues. TTL: until equipment model is retired from competition.
- **`venue-profile`**: Venue-specific RF characteristics (if repeat events at the same site). TTL: 2 years.

Memory retrieval supports both key-based lookup (`<equipment>:<event>:<team>`) and statistical queries (e.g., "top 10 configurations for Ericsson AIR 6419 in outdoor venues"). Kai implements the statistical queries as pre-computed views refreshed after each event.

## Requirement 6: Signed Commits via OpenWallet

Each staff member has an OpenWallet DID. Template commits require dual signature: Sam (generator) and Tasha (field-plausibility approver). Scoring commits require Diego's signature only — scoring is deterministic and does not require review.

Signing is important for competitive integrity. Teams have disputed scores. The signed commit chain provides tamper-evident proof that a score was computed from the raw data without manual modification.

**Unique insight:** Bandwidth Blitz's split between `studio` mode and `field` mode addresses a design tension in agent systems: the same tool needs to be creative (during preparation) and constrained (during execution). Pre-event, agents should explore freely — generate many templates, try unconventional configurations, learn from failures. During the event, agents should be read-only — observe, analyze, but never modify. This dual-mode pattern maps naturally to software development: during development, agents explore and generate. During release, agents verify and audit. Encoding the mode into the binary (not just the policy) prevents accidental mutations in the wrong phase, the same way the Grounding Incident could have been prevented if "temporary deployment" mode had blocked configurations that assumed permanent grounding.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Tasha | 1,500 | 800 | 2,300 |
| Diego | 3,000 | 2,500 | 5,500 |
| Sam | 4,000 | 3,500 | 7,500 |
| Anya | 3,000 | 2,000 | 5,000 |
| Kai | 2,000 | 1,000 | 3,000 |
| **Task Total** | **13,500** | **9,800** | **23,300** |

Coaching overhead (real-time analysis during practice): 5,000 tokens per session. Grand total per event preparation: **28,300 tokens**.

---

*"The clock does not care about your excuses. And neither does the scoring engine."*
— League motto, printed on every event badge
