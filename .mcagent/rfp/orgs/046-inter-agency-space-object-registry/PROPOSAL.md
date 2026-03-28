# Inter-Agency Space Object Registry — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated | RA-2026-1847**

---

## Executive Summary

IASOR proposes a `but-ai` implementation built on the principle that every agent action is a registry amendment: proposed, reviewed, reconciled, and ratified before it takes effect. Our domain expertise in multi-agency catalogue reconciliation translates directly to polyrepo coordination where consistency across repositories is non-negotiable.

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary installs to `$PATH` as a statically-linked executable. Invocation follows the `but-tool-ai` pattern. The binary maintains an internal audit log at `$XDG_STATE_HOME/but-ai/audit.log` recording every invocation with timestamp, arguments, and exit code.

Configuration is layered: system defaults, user config (`$XDG_CONFIG_HOME/but-ai/config.toml`), repository config (`.but-ai.toml` in repo root), and environment variables. Each layer's provenance is recorded in the audit log. If a configuration value is ambiguous due to conflicting layers, the binary exits with an error rather than guessing — consistent with IASOR's "no silent resolution" policy.

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend (OpenAI, Anthropic, Ollama, LMStudio). Provider trait with `complete()`, `tool_call()`, and `budget_estimate()` methods. The `budget_estimate()` method is IASOR-specific: before any LLM call, the provider returns a token cost estimate that must be approved by SATCAT agent against the remaining budget.

Provider switching is permitted but audited. Every switch generates a "Provider Amendment" entry in the audit log with justification. Fallback ordering is configured per-repository, not globally — different repositories may have different compliance requirements that constrain provider choice.

**Domain Insight:** In catalogue management, we learned that silent data source switches cause reconciliation failures. If a TLE update comes from a different radar network than expected, the error bars change, and downstream conjunction assessments become unreliable. Same principle for LLM providers: switching from Anthropic to Ollama mid-task changes the quality distribution of outputs. The system must account for this, not hide it.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

NORAD-1 agent generates patches in unified diff format. Every hunk includes a structured annotation comment explaining the change rationale. COMMIT.msg follows Conventional Commits extended with IASOR metadata:

```
feat(tracking): add batch TLE ingestion endpoint

RA-2026-1848
Agent: NORAD-1
Provider: anthropic/claude-sonnet
Tokens: 3,200 in / 1,800 out
Reviewed-By: COSPAR (memory check), RECONCILE (consistency check)
Signed-By: SEAL-V (did:key:z6Mk...)
```

Patch generation follows a four-phase process:
1. **Survey** — Read project status and branch state.
2. **Consult** — Query COSPAR for relevant prior decisions.
3. **Draft** — Generate the patch with annotations.
4. **Reconcile** — RECONCILE verifies cross-repo consistency before SEAL-V signs.

## Requirement 4: Polyrepo PR Coordination

RECONCILE agent maintains a "consistency ledger" — a structured document in `refs/iasor/ledger/` that maps PRs across repositories into coordination sets. Each set has:
- A set identifier (sequential, never reused).
- A list of member PRs with repo, branch, and status.
- A merge ordering constraint (which PRs must merge before which).
- A reconciliation status (pending, consistent, inconsistent).

RECONCILE runs a reconciliation check before any PR in a set can be merged. The check verifies that all dependent PRs are in a mergeable state and that no conflicting changes exist across the set. If reconciliation fails, all PRs in the set are flagged with a structured comment explaining the inconsistency.

Forge adapters for GitHub, GitLab, and Gitea implement a common `ForgeAdapter` trait. PR comments carry machine-readable metadata in HTML comment blocks.

## Requirement 5: Agent Memory in Git Branches

COSPAR manages memory as a "registry of prior decisions." Memory entries are stored in `refs/iasor/registry/<namespace>/<entry-id>` as JSON blobs with mandatory fields:

| Field | Description |
|-------|-------------|
| `id` | Sequential, never reused |
| `created` | ISO 8601 timestamp |
| `amendment` | The RA number that created this entry |
| `content` | The memory payload |
| `provenance` | Agent ID, task ID, provider used |
| `status` | active, deprecated, superseded |
| `superseded_by` | ID of superseding entry, if any |

Memory is never deleted. Deprecated entries remain in the registry with a deprecation notice. This is expensive in storage but ensures complete auditability. Garbage collection compacts the object store but never removes refs.

Relevance scoring uses BM25 over memory content with recency weighting. Top-5 retrieval with a minimum score threshold.

## Requirement 6: Signed Commits via OpenWallet

SEAL-V handles signing with additional compliance layers:
- Pre-sign authorization check against agent permissions.
- CRL (Certificate Revocation List) check before every signature.
- Post-sign audit entry with full signature metadata.
- Key rotation on a 60-day cycle (longer than most proposals, reflecting IASOR's change-averse culture).

Signature verification is embedded in the reconciliation process: RECONCILE will not include an unsigned or improperly signed PR in a coordination set.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| NORAD-1 | Patch generation | 8,000 | 5,000 | 13,000 |
| COSPAR | Memory & compliance | 6,000 | 1,200 | 7,200 |
| SATCAT | Provider management | 3,000 | 800 | 3,800 |
| RECONCILE | PR coordination | 5,500 | 2,000 | 7,500 |
| SEAL-V | Signing | 3,000 | 600 | 3,600 |
| **Per-task total** | | **25,500** | **9,600** | **35,100** |

Budget includes IASOR's mandatory audit overhead (approximately 8% of total budget). This is higher than most proposals. We consider it the cost of compliance.

## Unique Domain Insight

Eighteen years of multi-agency catalogue management taught us that the hardest problem in distributed systems is not consensus — it is reconciliation after independent evolution. Two agencies tracking the same object with different sensors will inevitably diverge. The question is not how to prevent divergence but how to detect and resolve it without data loss.

Our RECONCILE agent applies this lesson to polyrepo coordination. Instead of trying to prevent cross-repo inconsistencies (which is impossible when repositories evolve independently), we detect them at merge time and provide structured resolution paths. The consistency ledger is a reconciliation protocol, not a locking mechanism.

---

*RA-2026-1847. Comment period closes: 2026-05-12. Sincerely, The Registry.*
