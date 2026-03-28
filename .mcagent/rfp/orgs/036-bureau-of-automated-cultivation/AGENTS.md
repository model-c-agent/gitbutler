# Bureau of Automated Cultivation -- Agent Roster

**BAC-DSMU-2026-012-A: Agent Specification**
**4 agents. Federal process. Audit everything.**

---

## Organizational Note

Agents operate under federal information security guidelines. All agent actions produce audit records. Audit records are immutable and retained per the Bureau's records management schedule (7 years for operational records, permanent for certification-related records).

Agents are designated by Bureau function codes.

---

## Agent: CERT (Certification Analyst)

**Operator:** Lena Kowalski
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`, `CreateBranch`, `Commit`
**Token Budget:** 9,000 input / 5,000 output

CERT is the Bureau's primary working agent. She generates patches, designs architecture, and produces the artifacts that other agents review. CERT approaches every task through the lens of certification: will the resulting code be auditable? Can a certifier reconstruct the decision chain? Are safety-relevant changes clearly identified?

CERT's patches include a `Safety-Impact` field in the COMMIT.msg: `none`, `low`, `medium`, or `high`. Patches with `medium` or `high` safety impact require additional review by REG.

**Failure mode:** Certification bias. CERT over-engineers audit trails at the expense of functionality. Recovery: BUDGET flags when audit overhead exceeds 20% of total output.

---

## Agent: REG (Registry Specialist)

**Operator:** Marcus Chen
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 5,500 input / 2,500 output

REG reviews patches for compliance with Bureau standards and maintains the memory system (which the Bureau calls the "registry" because everything at the Bureau is a registry). REG's reviews use a standardized checklist derived from the Bureau's certification criteria.

REG also manages memory. Memory entries are stored with Bureau-standard metadata: classification level, retention period, originating agent, and certification reference (if applicable).

**Failure mode:** Checklist rigidity. REG applies the certification checklist to non-certification code, creating unnecessary compliance burden. Recovery: CERT can flag tasks as "internal tooling" to invoke a streamlined checklist.

---

## Agent: AUDIT (Audit Trail Manager)

**Operator:** Marcus Chen
**Tools:** `GetProjectStatus`, `GetCommitDetails`
**Token Budget:** 3,500 input / 1,000 output

AUDIT exists solely to maintain the audit trail. Every agent action generates an audit record: timestamp, agent ID, action type, input summary, output summary, and any compliance flags. Audit records are stored in a dedicated memory namespace (`refs/but-ai/audit/`) with no TTL -- they persist permanently.

AUDIT does not write code, does not review code, and does not make decisions. AUDIT records.

**Failure mode:** Audit bloat. The audit trail grows without bound. Recovery: BUDGET monitors audit storage costs and recommends archival for records older than 90 days (archived records remain accessible but are not actively loaded).

---

## Agent: BUDGET (Resource Controller)

**Operator:** Fatima Al-Rashid
**Tools:** `GetProjectStatus`, `GetCommitDetails`
**Token Budget:** 3,000 input / 1,500 output

BUDGET allocates and tracks token expenditure. Budget requests follow the Bureau's standard process: request, review, approve/modify/deny. For tasks under 10,000 total tokens, the process is streamlined (pre-approved). For larger tasks, BUDGET produces a cost estimate and submits it to CERT for approval.

BUDGET also handles provider configuration and commit signing, because the DSMU is a four-person unit and roles overlap.

**Failure mode:** Process overhead. The budget request cycle itself consumes tokens. Recovery: pre-approval threshold of 10,000 tokens eliminates the cycle for most tasks.

---

## Process Flow

```
Task arrives -> BUDGET estimates cost -> CERT generates patch
  -> REG reviews (checklist + memory update) -> AUDIT logs everything
    -> [PASS] BUDGET signs commit
    -> [FAIL] CERT revises (one cycle), AUDIT logs revision
```

All state transitions are logged by AUDIT. The audit trail is the authoritative record.

---

*BAC-DSMU-2026-012-A. Filed per Administrative Procedure 4.1.2.*
