# Benford's Law Laboratory -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Submitted:** 2026-03-28
**PI:** Professor Anya Lindström, University of Zurich

---

## Summary

The Benford's Law Laboratory proposes a `but-ai` implementation designed for reproducible, statistically grounded agent operations. Every agent output carries confidence intervals. Every finding is validated before it enters the record. Every analysis is reproducible from the committed code and data. We bring the rigor of peer-reviewed science to agent-produced commits.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Invoked per analysis task. Stateless.

- Binary: Rust, statically linked
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml`
- Logging: every invocation logged with arguments, timestamps, and model version for reproducibility

The logging requirement is non-negotiable. If we cannot reproduce an agent's behavior, we cannot validate its output.

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`. All calls logged with full request/response for reproducibility audit.

| Provider | Usage | Notes |
|----------|-------|-------|
| Anthropic | Primary analysis | Best reasoning for complex statistical interpretation |
| OpenAI | Secondary | Backup and comparison |
| Ollama | Reproducibility testing | Pinned local models for deterministic re-runs |
| LMStudio | Development | Local iteration |

For reproducibility-critical tasks, we use Ollama with pinned model weights and temperature=0. This does not guarantee identical outputs but minimizes variation.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Statistically Annotated Patches

Every INDEX.patch that adds a finding includes structured statistical metadata:

```
finding: Benford first-digit deviation in Q3 revenue figures

Dataset: SEC-10Q-2025-Q3 (n=2,847 line items)
Test: Chi-squared goodness-of-fit
Statistic: 42.7
df: 8
p-value: 0.000003
Significance: p < 0.001
Effect size: Cramer's V = 0.14

Interpretation: PENDING HUMAN REVIEW

Study: BLL-2026-014
Agent: youssef
Method-version: abc1234
Data-hash: sha256:def5678
```

The `Interpretation: PENDING HUMAN REVIEW` field is mandatory. No agent produces interpretations. Only Lindström approves interpretive statements.

---

## 4. Polyrepo PR Coordination

The lab coordinates with external collaborators (audit offices, regulatory bodies) via forge-based PR workflows.

### Comment Schema

```json
{
  "lab": "benford",
  "study": "BLL-2026-014",
  "action": "finding_shared",
  "statistical_significance": true,
  "p_value": 0.000003,
  "reproducibility": "confirmed",
  "method_version": "abc1234"
}
```

Forge support: GitHub (primary -- lab repos), GitLab (university infrastructure), Forgejo, Bitbucket.

Every shared finding includes its reproducibility status. We do not share unconfirmed findings externally.

---

## 5. Agent Memory in Git Branches

### Reproducibility-Certified Memory

Memory in `refs/benford/memory/<study>/<entry>`.

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `finding` | The result |
| `p_value` | Statistical significance |
| `reproducibility` | `confirmed`, `pending`, `failed` |
| `method_version` | Code hash that produced it |
| `data_hash` | Input data hash |
| `ttl` | Hours (confirmed: 720, pending: 72, failed: 24) |

### Retrieval Rules

- Only `confirmed` entries are auto-injected
- `pending` entries are available on explicit request
- `failed` entries are retained for methodological learning but never injected

This prevents unvalidated findings from influencing future analysis -- the scientific equivalent of contaminated evidence.

---

## 6. Signed Commits via OpenWallet

Each agent holds a Verifiable Credential with their role and lab affiliation.

- Lindström: signing authority (only she signs published findings)
- Youssef, Tanaka: produce unsigned patches
- Kasper: infrastructure commits only

Key rotation: monthly (aligned with university IT policy). Revocation: immediate, stored in `refs/benford/revoked`.

---

## 7. Token Budget

| Member | Role | Input | Output | Total |
|--------|------|-------|--------|-------|
| Prof. Lindström | Review/signing | 5,500 | 1,800 | 7,300 |
| Dr. Youssef | Analysis/patches | 8,200 | 4,800 | 13,000 |
| Dr. Tanaka | Memory/validation | 5,500 | 1,200 | 6,700 |
| Kasper | Infrastructure | 3,200 | 800 | 4,000 |
| **Lab Total** | | **22,400** | **8,600** | **31,000** |

---

## 8. Unique Insight: Confidence Intervals on Agent Outputs

Most agent systems produce outputs that are either "accepted" or "rejected." There is no middle ground. We propose that every agent finding carry a **statistical confidence measure** -- not a vague "high/medium/low" label but a quantified confidence interval derived from the underlying data.

When an agent flags a Benford deviation, it does not say "this is suspicious." It says "chi-squared = 42.7, df = 8, p < 0.001." The consumer of this output knows exactly how confident the finding is, can compare it to a threshold of their choosing, and can evaluate it in the context of other findings.

This applies beyond fraud detection. Any agent output that is based on pattern matching or statistical analysis can carry a confidence measure. The measure does not make the output correct -- it makes the output *honest* about its uncertainty. In science, that honesty is the difference between a finding and a guess.

---

*"We do not guess. We measure. And we report the measurement error."*
