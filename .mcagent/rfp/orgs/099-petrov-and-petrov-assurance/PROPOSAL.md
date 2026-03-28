# Petrov & Petrov Assurance -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We are a firm built on disagreement. Our `but-ai` plugin formalizes productive disagreement between agents: when two agents produce conflicting patches, both are preserved, compared, and presented to a human for resolution. Consensus is not always the goal. Sometimes the goal is a clear articulation of why two reasonable approaches differ.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary on `$PATH`. `cargo binstall but-ai`. The binary must support bilingual codebases (R and Python in the same repo) without requiring language-specific configuration.

**Dual-language support:** The plugin detects file types in the task scope and adjusts context reading accordingly. R files and Python files have different tokenization characteristics; the budget allocator accounts for this.

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait, four providers. The brothers insist on reproducibility: identical inputs to the same model version must produce outputs within a defined tolerance. Providers that fail reproducibility checks are flagged.

**Dual-provider verification:** For critical patches, the same task is sent to two different providers. If the outputs agree, high confidence. If they disagree, the disagreement is surfaced to the human reviewer with both outputs and a diff between them. This mirrors the firm's dual-valuation approach.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce patches. When two agents produce conflicting patches for the same task, the system does not pick one. It produces a comparison report.

**Conflict resolution protocol:**
1. Agent-Nikolai generates patch using approach A
2. Agent-Alexei generates patch using approach B
3. If patches are identical: commit with high confidence
4. If patches differ: produce a `COMPARISON.md` artifact showing both approaches, the delta, and Ivanka-bot's precedent analysis
5. Human reviews the comparison and chooses (or combines)

**Why this matters:** In actuarial work, disagreement between independent calculations is a signal, not an error. Suppressing it hides risk. Surfacing it reveals assumptions.

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Forge adapter trait. Two repos: valuation engine and report generator. Coordination ensures that engine changes are reflected in report templates before either repo merges.

**Dual-branch coordination:** When the brothers work on competing approaches, both approaches get their own cross-repo coordination chain. The merge happens only after the client (or the human reviewer) selects an approach.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/petrov/memory/`. Entries include a `perspective` field indicating which brother's methodology the memory relates to.

**Dual-perspective memory:**
```json
{
  "key": "discount-rate-methodology-2026",
  "value_nikolai": "Market-consistent: use current BGN government yield curve",
  "value_alexei": "Best-estimate: use 10-year average yield of 3.2%",
  "precedent": "Client chose best-estimate in 4 of last 6 engagements",
  "created": "2026-03-28T09:00:00Z",
  "ttl": null
}
```

When either agent retrieves this memory, it sees both perspectives and the historical precedent. The memory system does not take sides.

---

## Requirement 6: Signed Commits via OpenWallet

Both brothers' agents have independent signing keys. When a dual-valuation is resolved and merged, the merge commit is signed by whichever brother's approach was selected, and co-signed by Desislava (who verifies the client authorized the selection).

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Nikolai | 7,500 | 3,500 | 11,000 |
| Alexei | 7,500 | 3,500 | 11,000 |
| Ivanka-bot | 4,800 | 600 | 5,400 |
| Borislav | 5,000 | 2,000 | 7,000 |
| Desislava | 3,200 | 800 | 4,000 |
| **Total** | **28,000** | **10,400** | **38,400** |

Note: The dual-agent approach means the per-task budget is approximately 1.5x a single-agent approach. The brothers consider this a feature, not a cost.

---

## Unique Insight: Productive Disagreement as a Feature

Every other proposal in this RFP will aim for agent consensus. Ours aims for structured disagreement. When two agents produce different patches for the same task, most systems would resolve the conflict automatically (pick the "better" one). Ours preserves both, compares them, and asks a human to choose.

This is not indecision. It is dual-control. In actuarial practice, independent parallel calculations are a standard quality assurance technique. If two actuaries agree, you have confidence. If they disagree, you have discovered an assumption worth examining.

In our testing, the dual-agent approach caught 11 assumption errors over 100 tasks that a single-agent approach would have missed entirely. The "cost" was 50% more tokens. The value was catching errors before they reached production. The brothers would tell you: two opinions are always worth more than one.

---

*"Mama would be proud. We still argue, but now the arguments ship."*
