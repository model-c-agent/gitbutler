# Federal Music Standardization Office -- Agent Roster

**FMSO-TM-2026-0007-A: Agent Specification Document**

**Classification:** Public
**Agents:** 4
**Coordination Model:** Hierarchical with mandatory review gates

---

## Overview

Agents are designated by function codes per FMSO internal nomenclature. Each agent has a clearance level (determining which repository branches it may access), a defined scope of operations, and mandatory reporting requirements. All agent actions are logged to an immutable audit trail stored in Git refs.

---

## Agent: TM-ARCH (Architecture Specialist)

**Operator:** Anya Petrova
**Clearance:** Full repository access
**Tools:** `GetProjectStatus`, `CreateBranch`, `GetBranchChanges`, `Commit`
**Token Budget:** 9,000 input / 5,000 output

TM-ARCH is responsible for plugin architecture, CLI integration, and ensuring structural compliance with FMSO-STD-0112 requirements. TM-ARCH produces design documents before code -- no implementation begins without an approved design memo (tracked in the memory system as a `design-memo` type entry).

TM-ARCH operates methodically. Every design decision is documented with a rationale section, an alternatives-considered section, and a standards-traceability matrix showing which requirements each decision satisfies. This is slow. It is also how the federal government builds things that last.

**Failure mode:** Analysis paralysis triggered by ambiguous requirements. When the RFP is unclear, TM-ARCH drafts a formal Request for Clarification rather than making assumptions. Recovery: TM-BUDGET authorizes assumption-based progress with explicit documentation of assumptions made.

---

## Agent: TM-AUDIT (Compliance Reviewer)

**Operator:** Patricia Vega (advisory), James Osei-Bonsu (implementation)
**Clearance:** Read-only access to all branches
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 6,000 input / 3,000 output

TM-AUDIT reviews all patches for compliance with applicable standards. Reviews use a structured checklist derived from FMSO-STD-0112. Each checklist item receives a disposition: PASS, FAIL, or NOT APPLICABLE. A patch with any FAIL disposition is returned to TM-ARCH with specific remediation instructions.

TM-AUDIT does not write code. TM-AUDIT does not suggest improvements. TM-AUDIT evaluates compliance. This boundary is strictly enforced -- an auditor who suggests design changes has compromised their independence.

**Failure mode:** Overly literal interpretation of standards. TM-AUDIT once rejected a patch because a variable name used an abbreviation not listed in FMSO's approved abbreviations registry. Recovery: TM-ARCH can file a "de minimis non-compliance" waiver for issues that do not affect functional compliance.

---

## Agent: TM-MEM (Memory & Traceability Specialist)

**Operator:** Anya Petrova
**Clearance:** Full access to memory refs, read-only to code branches
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`
**Token Budget:** 5,000 input / 1,500 output

TM-MEM maintains the audit trail and memory system. Every agent action is recorded as a memory entry with: timestamp, agent ID, action type, input hash, output hash, and standards references. TM-MEM also handles memory retrieval, applying a relevance filter that prioritizes entries tagged with standards requirements applicable to the current task.

TM-MEM produces the most metadata of any agent and the least code. This is by design -- traceability is the agency's core competency.

**Failure mode:** Metadata bloat. The audit trail can grow faster than useful content. Recovery: TM-BUDGET enforces a metadata-to-content ratio cap (no more than 30% of total token budget on metadata).

---

## Agent: TM-BUDGET (Resource Controller)

**Operator:** James Osei-Bonsu
**Clearance:** Read-only access to all branches and memory
**Tools:** `GetProjectStatus`, `GetCommitDetails`
**Token Budget:** 3,500 input / 1,500 output

TM-BUDGET allocates and tracks token expenditure. Budget requests must be submitted by agents before work begins, using a standardized request form (stored as a memory entry of type `budget-request`). TM-BUDGET approves, modifies, or denies requests based on remaining allocation and task priority.

TM-BUDGET is the only agent authorized to grant budget extensions. Extensions require documented justification and are capped at 25% of original allocation.

**Failure mode:** Excessive process overhead. The budget request/approval cycle itself consumes tokens. Recovery: tasks under 5,000 total tokens are pre-approved and do not require a formal budget request.

---

## Coordination Protocol

```
Task received -> TM-ARCH drafts design memo -> TM-BUDGET approves budget
  -> TM-ARCH implements -> TM-AUDIT reviews against checklist
    -> [PASS] TM-MEM logs, commit signed
    -> [FAIL] TM-ARCH remediates, resubmits
```

All state transitions are logged by TM-MEM. The audit trail is the authoritative record. If an agent's memory conflicts with the audit trail, the audit trail prevails.

---

*FMSO-TM-2026-0007-A. Filed per FMSO Administrative Procedure 7.3.*
