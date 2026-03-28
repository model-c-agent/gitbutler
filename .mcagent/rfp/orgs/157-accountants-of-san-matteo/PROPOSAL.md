# The Accountants of San Matteo -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Submitted:** 2026-03-28
**Filed by:** Brother Giacomo, with the approval of the Chapter

---

## Summary

The Accountants of San Matteo are a lay religious order providing free forensic auditing to defrauded nonprofits. We have completed 100 pro bono cases. Our `but-ai` proposal reflects our values: precision, humility, and accountability. Every agent output is reviewed by a human auditor. Every commit is a professional attestation. Every memory entry distinguishes between machine-generated and human-verified knowledge.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Invoked per task. Stateless.

- Binary: Rust, statically linked
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml`
- Simplicity: the Order's members are accountants, not engineers. The plugin must be installable and configurable by someone whose primary skill is double-entry bookkeeping.

We emphasize simplicity because our tool must be usable by volunteer auditors. Complex configuration is a participation barrier.

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`. Provider set in config.

| Provider | Usage | Notes |
|----------|-------|-------|
| Anthropic | Active cases | Best for transaction analysis |
| OpenAI | Batch processing | Cost-effective for large ledger scans |
| Ollama | Offline auditing | Cases with data sensitivity requirements |
| LMStudio | Development | Giacomo's local testing |

The Order operates on donated API credits and volunteer time. Provider choice is driven by cost and data sensitivity.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Audit-Grade Patches

Every INDEX.patch adds a finding to the case record. Findings follow the Order's audit format:

```
finding: unmatched vendor invoices in FY2023 Q2

47 invoices totaling EUR 128,400 paid to "Servizi Globali SRL."
No corresponding vendor in the client's approved vendor list.
No goods or services received per inventory and service logs.
Pattern consistent with fictitious vendor scheme.

Case: SM-2026-012
Agent: giacomo
Blessed: NO (pending Lucia review)
Confidence: HIGH
```

The `Blessed:` field is our addition. An unblessed finding is a lead. A blessed finding is an attestation. The distinction matters in legal proceedings.

---

## 4. Polyrepo PR Coordination

Beatrice coordinates with client nonprofit repos and, when applicable, with law enforcement referral repos.

### Coordination Message

```json
{
  "order": "san-matteo",
  "case": "SM-2026-012",
  "action": "finding_shared",
  "blessed": false,
  "summary": "47 unmatched vendor invoices, EUR 128,400",
  "branch": "case/012/findings"
}
```

Forge support: GitHub, Forgejo. The Order does not need more. Beatrice writes messages for nonprofit board members: plain language, no jargon, actionable.

---

## 5. Agent Memory in Git Branches

### Blessed/Unblessed Memory

Memory stored in `refs/sanmatteo/memory/<case>/<entry>`.

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `content` | Memory value |
| `case` | Case reference |
| `provenance` | Creating agent and context |
| `blessed` | Boolean: has a human auditor verified this? |
| `ttl` | Blessed: indefinite. Unblessed: 168 hours |

### Retrieval

- Blessed entries are always available for retrieval
- Unblessed entries are available but flagged in context injection
- Expired unblessed entries are archived, not deleted (Tomasso insists)

### The Blessing Ritual

When Lucia reviews and approves a memory entry, she changes `blessed` to `true` and adds her name to the `provenance` field. This is the Order's equivalent of a professional sign-off. An unblessed entry is an observation. A blessed entry is a finding.

---

## 6. Signed Commits via OpenWallet

Lucia signs all case commits. Other agents produce unsigned patches.

- Keys provisioned via OpenWallet Verifiable Credentials
- Lucia's credential includes her professional auditor certification number
- Rotation: quarterly (aligned with the Order's case review cycle)
- Revocation: immediate, via `refs/sanmatteo/revoked`

Only Lucia signs because only Lucia is a licensed auditor. The Order treats signing as professional attestation, not merely cryptographic verification.

---

## 7. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Brother Giacomo | Engineering/patches | 7,800 | 4,000 | 11,800 |
| Sister Lucia | Review/signing | 5,500 | 1,800 | 7,300 |
| Brother Tomasso | Memory/archives | 4,500 | 900 | 5,400 |
| Sister Beatrice | Coordination | 4,200 | 1,800 | 6,000 |
| **Order Total** | | **22,000** | **8,500** | **30,500** |

---

## 8. Unique Insight: The Blessing as a Review Primitive

Most agent systems have a binary review: approved or rejected. We introduce a third state: **unblessed** -- the output exists, is visible, and can be used as a lead, but it has not been attested by a qualified human.

The blessing is not just a flag. It is a record of professional judgment. When Sister Lucia blesses a finding, she attests that she has reviewed the underlying data, verified the agent's reasoning, and is willing to stake her professional reputation on the conclusion. This transforms a machine-generated observation into a professional finding.

The blessing pattern generalizes to any domain where human expertise validates machine output: medical diagnosis (a model's suggestion is not a diagnosis until a physician signs it), legal analysis (a pattern match is not a legal opinion until a lawyer reviews it), and engineering (a generated design is not approved until an engineer stamps it).

The cost of blessing is human time. The cost of not blessing is unattested machine output in a legal proceeding. We choose the former.

---

*"Saint Matthew counted coins before he counted blessings. We do both."*
