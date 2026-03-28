# Harvest Command Authority -- Technical Proposal

**OPORD: but-ai Plugin Implementation**
**Classification: UNCLASSIFIED**
**RFP:** `but-ai` Plugin for GitButler
**Date:** 2026-03-28

---

## Commander's Intent

Deliver a disciplined, auditable implementation of the `but-ai` plugin that treats every agent operation as a military logistics operation: planned, executed, documented, and reviewed. The system shall be reliable under adverse conditions (poor connectivity, constrained resources, high operational tempo) because agricultural operations do not pause for software failures.

---

## Requirement 1: PATH-Based Plugin Architecture

Binary deployed to PATH via controlled distribution (checksum-verified download, no auto-update). The binary reports its version, build hash, and configuration status on invocation.

Configuration in a single TOML file. Configuration changes are treated as operational orders: each change is logged with a timestamp and justification. The plugin validates its configuration at startup and refuses to operate with an invalid configuration -- a misconfigured system is worse than a non-functioning one.

No daemon process. HCA systems operate in field environments where background processes are a liability (power constraints, unattended operation). The plugin starts cold, executes, and exits.

---

## Requirement 2: Provider-Agnostic AI

Four providers. Provider selection is a command decision: specified in configuration, not auto-detected or auto-switched. If the provider fails, the operation fails. The operator (or OPORD) decides the next action.

Automatic fallback is explicitly prohibited. In military operations, switching supply chains mid-mission without authorization causes confusion. The same applies here: switching providers changes the model's behavior, and the operator must authorize that change.

Provider health checks run at startup. If the provider is degraded (high latency, rate limiting), the plugin reports the degradation and the operator decides whether to proceed.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

S3 generates patches per the tasking order. The process follows a military decision-making sequence:

1. **Receive mission:** Read task description and OPORD's tasking order
2. **Conduct reconnaissance:** Read intelligence summary from S2
3. **Develop courses of action:** Identify possible patch approaches (typically 1-2)
4. **Select course of action:** Choose based on OPORD's constraints
5. **Execute:** Generate INDEX.patch and COMMIT.msg
6. **Report:** Submit after-action report with patch

Patches include an operational trailer in COMMIT.msg:

```
OPORD-Ref: <tasking order ID>
Phase: <which phase of the tasking order this fulfills>
Budget-Consumed: <tokens used>/<tokens allocated>
Intel-Used: <memory entries referenced>
```

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination follows the military concept of "coordination lines" -- boundaries between units that require synchronization when crossed.

Structured PR comments:

```
[HCA:COORD] source=repo-a@branch target=repo-b@branch phase=2 status=awaiting-sync
```

Dependencies are modeled as phase dependencies in the operational plan: Phase 2 in Repo B cannot begin until Phase 1 in Repo A is complete. OPORD tracks phase status across repos and issues "go" orders when dependencies are met.

Forge abstraction: minimal trait (`post_comment`, `read_comments`, `pr_status`). GitHub implementation. Additional forges on operational requirement.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/but-ai/memory/<section>/<key>`. Memory is classified by reliability:

| Classification | Description | TTL | Example |
|---------------|-------------|-----|---------|
| CONFIRMED | Verified by multiple tasks | 30 days | "Module X uses pattern Y consistently" |
| PROBABLE | High-confidence single observation | 14 days | "This API returns 404 for missing resources" |
| POSSIBLE | Unverified inference | 3 days | "This function may have a race condition" |

S2 stamps every memory entry with a reliability classification. Retrieval prioritizes CONFIRMED entries, then PROBABLE, then POSSIBLE. When context budget is tight, POSSIBLE entries are excluded.

### After-Action Intelligence

After every task, S2 writes an after-action report to memory: what intelligence was accurate, what was wrong, and what new intelligence was gathered. Over time, POSSIBLE entries get upgraded to PROBABLE or CONFIRMED based on repeated observation, or they expire.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed by S4 using DID-based credentials from OpenWallet. The signing flow:

1. S4 verifies the patch passed compliance review
2. S4 requests a signing credential (24-hour validity)
3. Commit is signed with Ed25519
4. Credential ID, expiry, and OPORD reference are recorded in commit trailers

Key compromise triggers an immediate operational halt. All commits signed with the compromised key are flagged. Operations resume only after key replacement and re-verification.

---

## Token Budget

| Agent | Input | Output | Total | Section |
|-------|-------|--------|-------|---------|
| OPORD | 6,000 | 3,000 | 9,000 | Command |
| S2 | 5,000 | 1,500 | 6,500 | Intelligence |
| S3 | 10,000 | 6,000 | 16,000 | Operations |
| S4 | 5,000 | 2,000 | 7,000 | Logistics |
| **HCA** | **26,000** | **12,500** | **38,500** | |

Budget includes operational overhead (tasking orders, after-action reports). This overhead is 15-20% of total budget and is non-negotiable -- you do not cut planning to save fuel.

---

## Unique Insight: Operational Planning as Architecture

Software architecture discussions tend toward the abstract: layers, modules, interfaces, patterns. Military operational planning is concrete: what happens first, what depends on what, what do you do when the plan fails, and who decides?

Our insight is that agent coordination is an operational planning problem, not an architectural one. The questions that matter are not "what is the right abstraction?" but "what is the sequence of actions, what are the dependencies between them, what are the decision points, and what is the fallback when an action fails?"

Our tasking order format answers these questions explicitly for every task. Phase lines define sequence. Dependency declarations define synchronization points. Constraint specifications define boundaries. Contingency branches define fallback actions.

This is not software engineering methodology applied to farming. It is military planning methodology applied to software. The distinction matters because military planning has a 3,000-year track record of coordinating autonomous agents (soldiers) under uncertainty, with imperfect communication, on a deadline. The problems are not new. The solutions are proven.

---

*OPORD COMPLETE. EXECUTE ON ORDER.*
*HARVEST COMMAND AUTHORITY -- Mission First, Standards Always.*
