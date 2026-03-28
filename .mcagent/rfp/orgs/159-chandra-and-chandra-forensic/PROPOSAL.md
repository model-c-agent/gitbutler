# Chandra & Chandra Forensic -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Submitted:** 2026-03-28
**From:** Meera Chandra, Managing Partner

---

## Summary

Chandra & Chandra is a mother-daughter forensic accounting firm with $2B in recovered assets. Our `but-ai` proposal implements a hypothesis-driven agent workflow: one agent generates hypotheses, another produces computational proofs, and every finding is partner-reviewed before it enters the record. Our unique contribution: hypothesis-linked commits that connect every code change to the investigative theory it tests.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Per-task. Stateless.

- Binary: Rust, statically linked
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml` with office-specific overrides (Mumbai vs. London providers)
- Multi-timezone: config supports timezone-aware scheduling for cross-office workflows

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`.

| Provider | Office | Usage |
|----------|--------|-------|
| Anthropic | Mumbai | Complex hypothesis testing |
| OpenAI | London | Batch analysis, model building |
| Ollama | Both | Sensitive case data, offline analysis |
| LMStudio | Both | Development |

Office-specific provider defaults in `.but-ai.toml` per repo.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Hypothesis-Linked Commits

Every commit references the hypothesis it tests:

```
proof: revenue inflation hypothesis H-003 confirmed

Statistical test: Kolmogorov-Smirnov
Null hypothesis: Revenue figures follow expected distribution
Result: D=0.187, p=0.00002
Conclusion: Reject null. Revenue distribution inconsistent with
organic growth. Consistent with fabrication hypothesis H-003.

Case: CC-2026-044
Hypothesis: H-003 (revenue inflation via fictitious contracts)
Agent: meera
Model-version: abc1234
Data-hash: sha256:def5678
```

The `Hypothesis:` trailer links the commit to the investigative theory. Reading the commit log for a case reconstructs the entire hypothesis-test-refine cycle.

---

## 4. Polyrepo PR Coordination

Nikhil coordinates between the firm's internal case repos and client repos.

### Schema

```json
{
  "firm": "chandra-chandra",
  "case": "CC-2026-044",
  "action": "finding_delivered",
  "hypothesis": "H-003",
  "status": "confirmed",
  "significance": "p < 0.001",
  "branch": "case/044/proof-h003"
}
```

Forge support: GitHub (clients), GitLab (internal), Forgejo.

---

## 5. Agent Memory in Git Branches

### Hypothesis-Indexed Memory

Memory stored in `refs/chandra/memory/<case>/<category>/<key>`.

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `content` | Memory value |
| `case` | Case reference |
| `hypothesis` | Linked hypothesis (if applicable) |
| `category` | `hypothesis`, `finding`, `method`, `precedent` |
| `jurisdiction` | Legal context |
| `verified_by` | Reviewing partner |
| `ttl` | Category-dependent (see AGENTS.md) |

### Retrieval

Retrieval is context-aware: when working on hypothesis H-003, retrieve memories tagged with H-003 first, then memories from the same case, then precedents from similar cases in the same jurisdiction.

Cross-case memory is allowed for methods and precedents. Cross-case memory is prohibited for case-specific findings (same evidence isolation as other forensic orgs).

---

## 6. Signed Commits via OpenWallet

Dual signing:
- **Meera's key:** signs analytical findings (computational proofs)
- **Priti's key:** signs final case reports (partner attestation)

Both keys are provisioned via OpenWallet. Priti's key carries her professional license number.

- Rotation: monthly
- Revocation: immediate, stored in `refs/chandra/revoked`
- No commit reaches the client without Priti's signature

---

## 7. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Priti | Strategy/review | 5,800 | 2,200 | 8,000 |
| Meera | Modeling/patches | 8,500 | 5,000 | 13,500 |
| Kavya | Memory/research | 4,800 | 1,000 | 5,800 |
| Nikhil | Infrastructure | 3,800 | 1,200 | 5,000 |
| **Firm** | | **22,900** | **9,400** | **32,300** |

---

## 8. Unique Insight: Hypothesis-Linked Commit History

Most commit histories record what changed. They do not record why the change was made in the context of an ongoing investigation. Our commit messages link every change to the hypothesis it tests, creating a narrative thread through the commit log that reads like an investigation journal.

Reading `git log` for a Chandra & Chandra case tells you: which hypotheses were proposed, which were tested, which were confirmed, and which were falsified. This is not just documentation -- it is a reproducibility aid. Any auditor reviewing the case can follow the hypothesis chain from the initial theory to the final proof, verifying each step against the committed evidence.

In a courtroom, this transforms `git log` from a technical artifact into an evidentiary narrative. The prosecution can walk the jury through the investigation chronologically, showing each hypothesis and its corresponding proof as sequential commits. The chain of reasoning is in the version control history, immutable and signed.

---

*"The hypothesis is the question. The model is the answer. The commit is the proof."*
