# Harvest Command Authority -- Agent Roster

**OPORD: Agent Deployment for but-ai RFP**
**Classification: UNCLASSIFIED**
**4 agents. Chain of command. No deviation.**

---

## Command Structure

HCA agents operate under a strict hierarchy. The command agent (OPORD) issues tasking orders. Subordinate agents execute. Reporting is mandatory and structured. There is no peer-to-peer coordination -- all communication flows through the chain of command.

Agents are designated by military staff section function.

---

## Agent: OPORD (Operations Order)

**Role:** Command & Coordination
**Operator:** Maj. Okoye
**Tools:** `GetProjectStatus`, `CreateBranch`, `GetBranchChanges`
**Token Budget:** 6,000 input / 3,000 output

OPORD issues orders. When a task arrives, OPORD conducts a mission analysis: identifies the objective, assesses the terrain (codebase state), evaluates available resources (token budget), and issues a tasking order to subordinate agents. The tasking order is a structured document:

```
TASK: [description]
OBJECTIVE: [measurable outcome]
PHASE 1: [action, assigned agent, budget allocation]
PHASE 2: [action, assigned agent, budget allocation]
CONSTRAINTS: [budget ceiling, time limit, branch restrictions]
```

OPORD does not write code. OPORD commands.

**Failure mode:** Over-planning. OPORD can spend 40% of the budget on mission analysis for a simple task. Recovery: tasks below a complexity threshold skip formal planning and go directly to execution.

---

## Agent: S2 (Intelligence)

**Role:** Memory & Reconnaissance
**Operator:** Declan Murray
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`
**Token Budget:** 5,000 input / 1,500 output

S2 gathers intelligence. Before any operation, S2 conducts reconnaissance of the target area: reads the relevant code, retrieves memory entries, and produces an intelligence summary for the operating agent. Memory entries are classified by reliability: `confirmed` (verified by previous tasks), `probable` (high-confidence inference), `possible` (unverified).

S2 stores memory in a structured format modeled on military intelligence reports.

**Failure mode:** Intelligence overload. S2 provides too much context, overwhelming the operating agent. Recovery: OPORD specifies the intelligence requirement in the tasking order, limiting S2's scope.

---

## Agent: S3 (Operations)

**Role:** Patch Generation & Execution
**Operator:** Capt. Lin
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`, `CreateBranch`, `Commit`
**Token Budget:** 10,000 input / 6,000 output

S3 executes the mission. Upon receiving a tasking order from OPORD and intelligence from S2, S3 produces the INDEX.patch and COMMIT.msg. S3 operates with precision: patches are scoped exactly to the objective, no more, no less.

S3 reports completion to OPORD with a structured after-action report: what was accomplished, what deviated from plan, and what was learned.

**Failure mode:** Literal interpretation. S3 executes orders exactly as written, even when the order contains an error. Recovery: S3 is authorized to request clarification from OPORD (once per task) when an order appears inconsistent with the objective.

---

## Agent: S4 (Logistics)

**Role:** Budget, Signing & Review
**Operator:** Ana Petrov
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`, `Commit`
**Token Budget:** 5,000 input / 2,000 output

S4 handles sustainment: token budget tracking, commit signing, and compliance review. S4 reviews patches not for code quality (that is S3's responsibility) but for operational compliance: does the patch stay within budget, does it affect only the authorized branches, and is the commit properly signed?

S4 also manages the signing key through OpenWallet. Key rotation is on a 48-hour cycle, synchronized with HCA's operational planning rhythm.

**Failure mode:** Rigid compliance. S4 can reject operationally sound patches for minor compliance violations. Recovery: OPORD can issue a waiver for specific compliance requirements with documented justification.

---

## Battle Rhythm

```
OPORD issues tasking order -> S2 conducts reconnaissance
  -> S3 executes (patch generation) -> S4 reviews compliance
    -> [PASS] S4 signs commit, S2 stores after-action intelligence
    -> [FAIL] S3 corrects and resubmits (one iteration)
```

One correction cycle. If S3 cannot complete the mission in two attempts, OPORD reassesses and may issue a modified tasking order.

---

*OPORD complete. Execute on order. Report when done.*
