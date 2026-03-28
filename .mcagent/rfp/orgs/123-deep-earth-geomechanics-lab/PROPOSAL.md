# The Deep Earth Geomechanics Lab — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation focused on **reproducibility and precision**. Every agent run is deterministically reproducible — given the same inputs, the same outputs are guaranteed. Every patch is validated against regression benchmarks. Every memory entry includes confidence intervals. Our system prioritizes correctness over speed, on the principle that an incorrect result delivered quickly is worse than a correct result delivered slowly.

---

## Requirement 1: PATH-based Plugin Architecture

Binary with reproducibility tooling.

**Design:**
- Binary: `but-ai`, statically linked
- Commands: `but ai patch`, `but ai reproduce <run-id>` (replay a run deterministically), `but ai memory`, `but ai validate`
- Config: `~/.config/but-ai/degl.toml`
- `but ai reproduce <run-id>` replays a recorded agent run with identical inputs and verifies identical output — if output differs, the divergence point is reported
- `but ai validate <patch>` runs the patch through regression benchmarks before commit

---

## Requirement 2: Provider-Agnostic AI

Provider abstraction with **reproducibility constraints**.

**Architecture:**
- Provider trait: standard invoke/stream with `seed` parameter for deterministic output
- When reproducibility is required, the system sets provider seed and temperature=0
- Provider response caching: every response cached by (prompt_hash, model, seed) for replay
- Batch scheduling: non-urgent requests queued and batched for efficient provider usage
- Supported: OpenAI, Anthropic, Ollama, LMStudio

**Reproducibility mode:** `but ai patch --reproducible` forces seed-based deterministic generation and caches all provider responses for future replay.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Patches validated against **regression benchmarks** before commit.

**Workflow:**
1. Read task and context
2. Retrieve relevant memory (experimental observations)
3. Generate INDEX.patch + COMMIT.msg
4. Regression validation: apply patch to scratch, run benchmark suite
5. If benchmarks pass (all outputs within tolerance): commit
6. If benchmarks fail: report divergence with specific values

**Regression benchmark format:**
```toml
[[benchmark]]
name = "api-error-handling"
input = "tests/fixtures/api-error-input.json"
expected_output = "tests/fixtures/api-error-expected.json"
tolerance = "exact"  # or "float:1e-12" for numerical tolerance
```

**COMMIT.msg trailers:**
```
Regression-Suite: PASSED (42/42 benchmarks)
Reproducibility-Seed: 42
Provider: anthropic/claude-sonnet-4-20250514
```

---

## Requirement 4: Polyrepo PR Coordination

Academic-style coordination with **structured significance statements**.

**Protocol:**
- PR comments: `<!-- degl:coord:{action}:{payload} -->`
- Actions: `propose`, `review`, `approve`, `merge`
- Every cross-repo PR includes a "significance statement": one paragraph explaining why this change matters and what it enables

**Forge adapters:** GitHub (primary for academic collaboration), GitLab, Gitea. Standard trait.

**Dependency specification:** `.but-ai/dependencies.toml` in each repo, listing cross-repo dependencies with version constraints.

---

## Requirement 5: Agent Memory in Git Branches

Memory as **experimental observations** with statistical metadata.

**Entry format:**
```toml
[observation]
key = "error-handling-pattern"
hypothesis = "This codebase uses Result<T, AppError> consistently"
method = "Scanned 47 functions in src/api/"
result = "44/47 use Result<T, AppError>, 3 use unwrap()"
confidence = 0.936
confidence_interval = [0.87, 0.98]
sample_size = 47
diversity_score = 0.7  # how diverse the observed contexts are
last_replicated = "2026-03-27"
```

**Storage:** `refs/but-ai/memory/observations/<domain>/<key>`

**Replication:** Memory entries are periodically re-validated ("replicated") against the current codebase. If replication fails (the pattern is no longer observed), the entry is flagged as `unreplicated` and its confidence drops.

**Expiration:** Unreplicated entries: 14 days. Replicated with confidence >0.8: 90 days. Replicated with confidence <0.8: 30 days.

---

## Requirement 6: Signed Commits via OpenWallet

Standard signing with **reproducibility metadata**.

**VC contents:**
- Agent identity
- Reproducibility seed (if used)
- Regression suite result summary
- Provider and model version
- Memory observations consulted

**Key lifecycle:** 30-day rotation, standard OpenWallet ceremony, immediate revocation on compromise.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Mokoena | Review | 3,200 | 1,000 | 4,200 |
| Ndlovu | Patch/Validation | 9,800 | 4,600 | 14,400 |
| Diagne | Memory | 6,200 | 800 | 7,000 |
| Tanaka | Provider | 5,500 | 2,200 | 7,700 |
| Santos | Coordination | 5,200 | 2,000 | 7,200 |
| **Total** | | **29,900** | **10,600** | **40,500** |

### Validation Overhead

| Phase | Budget |
|-------|--------|
| Regression benchmarks (per patch) | 3,000 |
| Reproducibility verification | 2,000 |
| Memory replication check | 1,500 |

### Scaling

| Complexity | Multiplier | Budget (incl. validation) |
|------------|-----------|--------------------------|
| Surface sample (trivial) | 0.5x | 23,250 |
| Core sample (standard) | 1.0x | 46,500 |
| Deep drill (multi-repo) | 2.0x | 93,000 |
| Full survey (architecture) | 2.5x | 116,250 |

---

## Unique Insight: Reproducibility as a Trust Primitive

AI agent output is non-deterministic by default. The same prompt can produce different patches on different runs. This makes agent output fundamentally untestable — you cannot write a regression test for a non-deterministic process.

Our system forces determinism where it matters. By fixing provider seeds, caching responses, and recording complete run traces, we make agent runs reproducible. You can replay any run and get the same output. You can write regression tests that verify agent behavior. You can detect when a provider update changes agent output.

This is how we run simulations in geomechanics: every simulation is reproducible, every parameter is recorded, every result can be regenerated from inputs. If a simulation produces different results on different runs, it is broken. The same standard should apply to AI agents.

Non-reproducible results are not results. They are anecdotes. We do not publish anecdotes, and we do not commit them either.

---

*Submitted by The Deep Earth Geomechanics Lab, University of the Witwatersrand.*
*"Resolution is truth."*
