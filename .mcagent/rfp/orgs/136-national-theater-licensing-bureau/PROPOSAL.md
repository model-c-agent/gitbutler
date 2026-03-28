# PROPOSAL.md — National Theater Licensing Bureau

**Filing Reference:** NTLB-RFP-2026-BUTAI-003

---

## Section 1: Summary

The Bureau proposes to implement the `but-ai` plugin as a licensing system. Every agent action requires a license. Every output is inspected before delivery. Memory entries have expiration dates and must be renewed. The system is designed for compliance, auditability, and the prevention of incidents — because the Bureau has seen what happens when compliance lapses, and it shut down *Hamlet* to prove the point.

---

## Section 2: Technical Proposal

### 2.1 Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is installed to PATH and registered via manifest. The Bureau requires that the manifest include a "license" field: a structured declaration of the plugin's compliance status, version, and inspection date. A plugin with an expired license field is flagged but not blocked (the Bureau recognizes that software versioning is not theater safety). Configuration in `.but/ai.toml`, provider selection via flag or environment variable.

### 2.2 Requirement 2: Provider-Agnostic AI

The Bureau licenses venues regardless of their construction material. Similarly, the provider layer abstracts LLM providers behind a `Venue` trait: `perform(prompt) -> Completion`, `inspect(completion) -> ToolCalls`, `audit(usage) -> TokenReport`. Each provider implements `Venue`. The Bureau adds a provider inspection step: at configuration time, each provider is tested with a standard prompt to verify tool-calling compliance. Providers that fail inspection are flagged as NON-COMPLIANT and may be used only with explicit user override.

### 2.3 Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow is a licensing process:

1. **Application** — Task received, filed by Clerk Vasquez with a license number
2. **Assessment** — Analyst Park reads the codebase, consults memory, assesses scope
3. **Drafting** — Park produces INDEX.patch + COMMIT.msg
4. **Inspection** — Inspector Okafor reviews against the 23-item compliance checklist
5. **Licensing** — Director Liu approves (LICENSED) or denies (CITATION ISSUED)

The 23-item checklist includes: syntactic correctness, semantic correctness, test coverage, commit message format, style consistency, no-regression verification, no-secret inclusion, proper imports, error handling, documentation updates, branch naming, dependency declarations, file placement, function naming, type safety, null handling, edge cases, performance impact, security impact, accessibility, license compliance, signed commit readiness, and cross-reference to prior patches.

### 2.4 Requirement 4: Polyrepo PR Coordination

Cross-repo coordination follows the Bureau's multi-jurisdiction model. When a code change spans multiple repositories, each repo requires its own license (approval). Coordination messages are posted as PR comments in the Bureau's citation format:

```
[NTLB-CITATION] License: NTLB-2026-0047 | Repo: backend
Status: PENDING INSPECTION | Depends: NTLB-2026-0046 (auth, LICENSED)
Inspector: Okafor | Due: next agent cycle
```

Forge-agnostic: the Bureau implements a `Jurisdiction` trait for GitHub/GitLab/Gitea.

### 2.5 Requirement 5: Agent Memory in Git Branches

Memory entries are stored in `refs/ntlb/licenses/` as license records:

```json
{
  "license_number": "NTLB-MEM-2026-0847",
  "issued": "2026-03-28",
  "expires": "2026-04-27",
  "status": "ACTIVE",
  "subject": "JWT validation uses RS256 with JWKS endpoint",
  "regulation_ref": "auth-conventions-v3",
  "inspector": "okafor",
  "renewal_count": 0
}
```

**The licensing memory model:** Every memory has an expiration date. Expired memories are not deleted — they are marked EXPIRED and retained. An agent retrieving memory receives both ACTIVE and EXPIRED entries for the same subject, enabling it to see what has changed. A memory that has been renewed multiple times (`renewal_count > 3`) is considered "established" and receives an extended TTL.

**Unique memory feature:** The Bureau tracks memory "violations" — instances where an agent acted on a memory that was later proven incorrect. Violated memories are marked REVOKED (analogous to a license revocation) and flagged in all future retrievals. This prevents agents from repeatedly acting on known-bad information.

### 2.6 Requirement 6: Signed Commits via OpenWallet

Director Liu signs all commits. The signing key is bound to her DID via OpenWallet. The signing ceremony is the final step in the licensing workflow — it occurs only after Inspector Okafor's inspection and Liu's review. An unsigned commit is an unlicensed performance: it may exist, but it carries no authority.

Key rotation: annually, with a 30-day overlap period. Emergency rotation: within 24 hours, with all commits signed during the suspected compromise window flagged for re-inspection.

---

## Section 3: Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Director Liu | 6,500 | 2,500 | 9,000 | Licensing, signing |
| Inspector Okafor | 6,500 | 3,000 | 9,500 | Inspection |
| Clerk Vasquez | 5,000 | 1,500 | 6,500 | Records, memory |
| Analyst Park | 9,000 | 6,500 | 15,500 | Patch generation |
| **Team Total** | **27,000** | **13,500** | **40,500** | |

Inspection overhead: ~3,500 tokens (23-item checklist, citation formatting).
**Total per task: ~44,000 tokens.**

---

## Section 4: Unique Insight

**Memory should expire, and expired memory should be visible.** Most memory systems treat expiration as garbage collection — expired entries are deleted or hidden. The Bureau treats expired memory as a signal. When an agent retrieves memory for a subject and finds both an ACTIVE entry and an EXPIRED entry, the delta between them reveals what has changed. An EXPIRED memory that says "auth uses bcrypt with cost factor 10" alongside an ACTIVE memory that says "auth uses argon2id" tells the agent not just the current state but the migration path. Deleting the expired entry deletes that context. The Bureau does not delete. The Bureau archives.

---

*CERTIFICATION: This proposal prepared per NTLB Proposal Standard 3.1. Filing reference: NTLB-RFP-2026-BUTAI-003. Status: LICENSED.*
