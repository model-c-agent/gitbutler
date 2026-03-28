# The Acute Care Simulation Centre — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation where every feature is **simulation-tested before deployment**. Our system includes a built-in simulation framework that replays recorded agent runs, tests new configurations against historical scenarios, and prevents regressions by maintaining a growing library of test cases derived from real-world usage. Nothing ships until it survives simulation.

---

## Requirement 1: PATH-based Plugin Architecture

Standard PATH-based binary with integrated simulation tooling.

**Design:**
- Binary: `but-ai`, statically linked
- Commands: `but ai patch`, `but ai sim` (run simulation), `but ai memory`, `but ai report`
- Config: `~/.config/but-ai/acsc.toml`
- `but ai sim run <scenario>` executes a named simulation scenario against the current codebase
- `but ai sim record` records a real agent run as a replayable scenario
- `but ai report` generates a simulation results summary

---

## Requirement 2: Provider-Agnostic AI

Provider abstraction with a **simulated provider** for deterministic testing.

**Provider types:**
- **Live providers:** OpenAI, Anthropic, Ollama, LMStudio — real API calls
- **Simulated provider:** Replays cached responses from recorded runs — zero API cost, deterministic output

**Architecture:**
- Provider trait: standard invoke/stream interface
- Provider selection: config-driven, with `simulated` as a first-class option
- Cache format: recorded prompt-response pairs stored in `refs/acsc/sim/cache/<provider>/<hash>`
- Cache validation: monthly 10% sample tested against live providers to detect drift

**Simulation mode:** `BUT_AI_PROVIDER=simulated` runs the entire system against cached responses. Used in CI, testing, and Friday simulation days.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Patches are generated in **30-minute bounded cycles** (based on our golden-hour research).

**Cycle:**
1. Context acquisition — read relevant files within budget
2. Memory retrieval — inject relevant patterns
3. Patch generation — produce INDEX.patch and COMMIT.msg
4. Pre-commit simulation — replay the patch against 3 relevant historical scenarios
5. If simulation passes: commit
6. If simulation fails: flag with `SIM_FAIL` marker and details

**The pre-commit simulation** is our key differentiator. Before every commit, the patch is tested against recorded scenarios that match the task type. This catches approximately 30% of errors that pass basic validation but would fail in production-like conditions.

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination with **simulation-verified merges**.

**Protocol:**
- Structured PR comments: `<!-- acsc:coord:{action}:{payload} -->`
- Actions: `propose`, `ack`, `sim-pass`, `sim-fail`, `merge`
- Before cross-repo merge, all participating repos run a coordinated simulation verifying that the combined changes work together

**Forge adapters:** GitHub, GitLab, Gitea, Forgejo. Standard adapter trait.

**Coordination simulation:** A "dress rehearsal" simulation that applies all cross-repo patches to a combined scratch workspace and runs integration tests. Only if the dress rehearsal passes does the merge proceed.

---

## Requirement 5: Agent Memory in Git Branches

Memory entries are **simulation-derived** — extracted from patterns observed across thousands of simulation runs.

**Memory sources:**
- Manual observation (human-authored)
- Simulation extraction (patterns mined from simulation archive)
- Cross-validation (patterns confirmed in both simulation and production)

**Storage:** `refs/but-ai/memory/<source>/<key>`

**Entry format:**
```toml
[memory]
key = "error-propagation-pattern"
source = "simulation"
summary = "Errors in handler layer must be wrapped in ApiError before propagation"
evidence_sims = ["sim-2026-001", "sim-2026-042", "sim-2026-187"]
evidence_real = ["run-2026-031"]
confidence = 0.89
cross_validated = true
```

**Expiration:** Simulation-only evidence: 30 days. Cross-validated: 90 days. Manual: configurable.

---

## Requirement 6: Signed Commits via OpenWallet

Signing with **adversarial simulation testing**.

**Standard signing:** OpenWallet-managed keys, per-commit signing, rotation every 30 days.

**Adversarial testing:** Quarterly simulation runs that test signing resilience:
- Key compromise scenario: what happens when a signing key is revoked mid-task?
- Replay attack: what happens when a signed commit is replayed from a different branch?
- Clock skew: what happens when the signing timestamp drifts?

Results from adversarial simulations inform signing policy updates.

**Key lifecycle:** Generation -> qualification simulation -> production use -> rotation -> retirement.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Park | Simulation | 3,500 | 1,200 | 4,700 |
| Asante | Patch | 9,800 | 4,500 | 14,300 |
| Callaghan | Provider | 5,500 | 2,200 | 7,700 |
| Ishida | Memory | 5,800 | 700 | 6,500 |
| Anwar | Signing | 3,500 | 800 | 4,300 |
| **Subtotal** | | **28,100** | **9,400** | **37,500** |

### Simulation Overhead

| Phase | Budget |
|-------|--------|
| Pre-commit simulation (per patch) | 5,000 |
| Dress rehearsal (cross-repo) | 12,000 |
| Adversarial testing (quarterly) | 25,000 |

### Scaling

| Complexity | Multiplier | Budget (incl. sim) |
|------------|-----------|-------------------|
| Minor | 0.5x | 21,250 |
| Standard | 1.0x | 42,500 |
| Complex | 2.0x | 85,000 |
| Architecture | 2.5x | 106,250 |

---

## Unique Insight: Pre-Commit Simulation as Quality Gate

Every CI system tests code after it is committed. We test code *before* it is committed, using simulation.

Our pre-commit simulation replays the proposed patch against historical scenarios — recorded agent runs that represent known-good and known-bad patterns. If the patch matches a known-bad pattern (e.g., it modifies error handling in a way that a previous simulation showed produces cascading failures), it is flagged before it enters the repository.

This inverts the traditional feedback loop. Instead of "commit, test, fix, commit again," our loop is "generate, simulate, fix, commit once." The first commit is usually the final commit.

We derived this from emergency medicine simulation: you do not let a trainee intubate a real patient until they have intubated a mannequin 50 times. You do not let an agent commit to a real codebase until its patch has survived simulation. The principle is identical: rehearsal before performance.

---

*Submitted by The Acute Care Simulation Centre, Manchester.*
*"Run it again."*
