# The Pilgrims' Route Cooperative — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

The Pilgrims' Route Cooperative proposes a `but-ai` implementation optimized for resource-constrained environments: minimal compute requirements, local-first provider preference, frugal memory, and a four-agent architecture that runs on modest hardware. Our domain expertise in providing reliable transit service to underserved communities on a donation budget translates to agents that do more with less.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. Compiled for minimal binary size (LTO, strip, no debug symbols in release). Target binary size under 10MB. The binary has zero runtime dependencies beyond libc and a network stack. No database, no cache layer, no background process.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. Sane defaults for everything — a user should be able to install the binary and start working without editing the config file. Brother Thomas's rule: "If it requires configuration to start, it will never start in the communities that need it."

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend with a strong local-first preference. Default provider ordering: Ollama > LMStudio > Anthropic > OpenAI. Local providers are preferred because:
- No API cost (the Cooperative operates on donations).
- No data leaves the machine (privacy for community-sensitive work).
- No network dependency (the Cooperative's internet is unreliable).

Daraja manages provider selection. When a local provider cannot handle a task (model too small, context too long), Daraja escalates to cloud providers with a warning logged: `CLOUD_ESCALATION: local model insufficient for task complexity`.

**Domain Insight:** The Cooperative learned that the most reliable systems are the ones with the fewest external dependencies. Our shuttle dispatch runs on a local server because cloud hosting costs money the Cooperative does not have and goes down when the internet goes down — which in Kibera is often. Same principle for AI providers: prefer local, escalate to cloud only when necessary.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Huduma generates patches with a "one-shot" approach: read context, generate patch, validate, submit. No iteration. The Cooperative's compute budget does not afford iterative refinement. This means Huduma's context reading phase is more thorough than typical — investing tokens in understanding before generating produces better first-attempt patches.

Patch generation follows:
1. **Read thoroughly** — All files in scope, recent commits, project status.
2. **Recall** — Kumbuka provides relevant memories.
3. **Generate once** — Produce INDEX.patch.
4. **Validate** — `git apply --check`. If validation fails, produce partial patch with explanation.

COMMIT.msg is plain:
```
fix: route overlap on Ngong Road evening schedule

Agent: Huduma | Provider: ollama/mistral | Tokens: 1,400/900
Riders: 184,207
```

## Requirement 4: Polyrepo PR Coordination

Daraja handles cross-repo coordination with minimal overhead. Coordination sets are tracked in a simple manifest file (`refs/pilgrims/manifest/`) — a JSON list of PRs with their repos, branches, and statuses. No complex ledgers, no rulemaking processes. The manifest is checked before merges. If a dependency is not ready, Daraja posts a plain-language comment explaining the wait.

Forge adapter focuses on GitHub (the Cooperative uses GitHub exclusively). GitLab and Gitea adapters are stub implementations that return clear error messages if invoked.

## Requirement 5: Agent Memory in Git Branches

Kumbuka manages memory with extreme simplicity:

| Field | Description |
|-------|-------------|
| `key` | Short keyword identifier |
| `value` | Plain text memory content |
| `created` | ISO 8601 timestamp |
| `last_used` | Updated on retrieval |
| `ttl_days` | 14 default, +7 on reuse (max 60) |

Memory stored in `refs/pilgrims/memory/` as plain text files (not JSON, not binary — plain text that can be read with `cat`). No embeddings. No vector search. Retrieval is keyword match with recency weighting. Brother Thomas tested the keyword approach against semantic search on the Cooperative's workload and found keyword retrieval was 94% as effective at 1% of the compute cost.

GC runs on explicit invocation. Memory footprint stays under 1MB.

## Requirement 6: Signed Commits via OpenWallet

Muhuri handles signing with minimal ceremony:
- Sign the commit.
- Log the signature.
- Move on.

Key storage uses the system keychain. Key rotation every 60 days. No complex attestation fields, no extended metadata. The signature proves the agent produced the commit. That is sufficient for the Cooperative's needs.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Huduma | Patch generation | 6,000 | 3,000 | 9,000 |
| Kumbuka | Memory (plain text) | 3,500 | 500 | 4,000 |
| Daraja | Provider, budget, coordination | 5,000 | 1,500 | 6,500 |
| Muhuri | Signing | 2,000 | 400 | 2,400 |
| **Per-task total** | | **16,500** | **5,400** | **21,900** |

The smallest budget in this RFP. We are proud of this. A tool that requires expensive compute to run is a tool that excludes the communities the Cooperative serves.

## Unique Domain Insight

Nine years of running free transit on a donation budget taught us that the best systems are not the most capable — they are the most accessible. A shuttle that serves one neighborhood reliably is worth more than a fleet that serves a city intermittently.

Our proposal applies this insight to the `but-ai` plugin itself: we optimize for the minimum viable agent configuration. Four agents, keyword memory, local-first providers, one-shot patch generation. This is not the most sophisticated proposal in this RFP. It is the most likely to work on a refurbished ThinkPad in a settlement with unreliable internet. We believe that matters.

The communities that need AI tooling the most are the communities least likely to afford frontier APIs and GPU clusters. A `but-ai` implementation that only works with cloud providers and large budgets is a `but-ai` implementation that serves the already-served. We propose something different: a floor, not a ceiling.

---

*Riders: 184,207. The van leaves at 7. Everyone is welcome. Even if they cannot pay.*
