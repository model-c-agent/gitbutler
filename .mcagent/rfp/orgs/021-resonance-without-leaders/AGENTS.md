# Resonance Without Leaders — Agent Roster

**6 agents. No hierarchy. No fixed roles. Self-organizing.**

---

## Team Structure

There is no structure. That is the structure.

Every agent is identical in capability. No agent has authority over any other. No agent has a fixed role. At the start of each task, agents self-select into roles based on the task's requirements and their own current state (available context, memory load, budget remaining). An agent that handled patch generation in the last task might handle memory retrieval in the next.

This sounds like it should not work. It works approximately 85% of the time.

## How Roles Emerge

When a task arrives, it is posted to a shared context visible to all six agents. Each agent evaluates the task and "claims" a role by announcing its intention:

- "I will observe" (workspace state, memory retrieval, context gathering)
- "I will implement" (patch generation)
- "I will coordinate" (forge adapter, PR management)
- "I will verify" (review, sign)
- "I will support" (documentation, formatting, supplementary tasks)

If no agent claims a critical role (implementation, verification), the task is returned as unassignable. If two agents claim the same role, both proceed — the outputs are compared by a quality metric, and the better one is used.

## Consensus and Conflict

Decisions are made by "resonance" — a process borrowed from the collective's music production workflow. An agent proposes an action. If no other agent objects within one communication round, the action proceeds. If any agent objects, the proposal is revised.

Objections must include a reason. "I object because the patch does not match the existing code style" is valid. "I object" without reason is ignored.

## Duplication and Waste

The collective tolerates duplication. Two agents producing competing patches for the same task is not a failure — it is parallel exploration. The better patch is used; the other is discarded. The token cost of the discarded patch is considered the cost of exploration.

This makes the collective approximately 15-20% more expensive per task than a hierarchical team. The collective argues that the quality improvement from competitive selection justifies the cost.

## Token Budget Summary

| Configuration | Input | Output |
|---------------|-------|--------|
| 2 observers | 4,000 each | 500 each |
| 2 implementers | 4,500 each | 4,000 each |
| 1 coordinator | 2,500 | 1,200 |
| 1 verifier | 3,500 | 600 |
| **Team Total (max)** | **23,500** | **10,800** |

Note: This is the maximum budget assuming competitive implementation (2 agents implementing in parallel). If roles do not overlap, the budget is approximately 18,000 input / 8,000 output.

## Failure Mode

The team fails when no agent claims a critical role. If the task is complex and all agents prefer observation over implementation, no patch is produced. Recovery: the system has a "last resort" rule — if no agent claims implementation within one round, the agent with the highest remaining budget is drafted. The drafted agent complies but may produce lower-quality output because it did not self-select.

The team also fails by competing too enthusiastically: when three agents claim implementation, the token cost triples with no quality improvement. Recovery: the verifier rejects the task and forces a role re-selection with a "max 2 implementers" constraint.
