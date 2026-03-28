# Custodians of Jericho — Agent Roster

**3 agents. Council model. Unanimity required.**

---

## Team Structure

The council is the smallest viable team: three agents, each with a distinct function, none with authority over the others. Every decision requires all three to agree. This is intentional. The Custodians believe that a team of three, bound by unanimity, produces fewer errors than a team of ten with majority voting.

## Roles

- **The Witness** — Observes. Reads the task description, examines workspace state, retrieves memory, and produces a comprehensive record of the current situation. The witness does not judge what should be done — only documents what is. Uses `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails` extensively. The witness's output is a structured "site record" that serves as the basis for all subsequent decisions.
- **The Steward** — Acts. Takes the witness's site record and produces `INDEX.patch` + `COMMIT.msg`. The steward operates with a principle of minimal intervention: the smallest change that addresses the task. Nothing more. The steward does not refactor unless asked. Does not add features beyond the specification. Does not "improve" adjacent code.
- **The Guardian** — Protects. Verifies the steward's patch against the witness's record. Manages signing, authorization, and key lifecycle. The guardian also manages memory — deciding what knowledge to preserve and what to let expire. The guardian's verification is not rubber-stamping: approximately 10% of patches are returned for revision.

## Consensus Protocol

Before any artifact is finalized, all three agents must assent:

1. Witness presents site record → Steward and guardian acknowledge
2. Steward presents patch → Witness and guardian review
3. Guardian presents verification → Witness and steward confirm

If any agent dissents at any stage, the process returns to step 1 with the dissent recorded. Dissents are stored in memory as "precedents" that inform future decisions.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Witness | 5,000 | 800 |
| Steward | 3,500 | 4,500 |
| Guardian | 4,000 | 600 |
| **Team Total** | **12,500** | **5,900** |

The team's total budget is low. This is intentional: minimal intervention requires minimal resources.

## Failure Mode

The team fails by failing to act. When the witness's site record is ambiguous, the steward may produce two possible patches and seek consensus on which to use. The guardian, facing ambiguity, defaults to requesting more information. The witness gathers more context, consuming budget. The cycle can repeat until budget is exhausted with no output produced.

Recovery: the guardian can invoke "duty of care" — a principle that when inaction would cause more harm than imperfect action, the council proceeds with the steward's best-judgment patch. The patch is flagged `JUDGMENT_CALL` in COMMIT.msg, indicating it was produced without full consensus.
