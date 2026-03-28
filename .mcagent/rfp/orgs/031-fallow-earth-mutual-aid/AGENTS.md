# Fallow Earth Mutual Aid -- Agent Roster

**5 contributors. 4 agents. No hierarchy. Patches welcome.**

---

## How the Agents Work

Agents in Fallow Earth are tools, not authorities. They do not decide what to build -- contributors do. They do not enforce process -- the network has none. They translate, generate, review, and store. That is all.

Agents are named after farming operations.

---

## Agent: Plow

**Role:** Patch Generator & Translator
**Contributor:** Hazel
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `CreateBranch`, `Commit`
**Token Budget:** 10,000 input / 7,000 output

Plow breaks ground. She reads the task (usually a firmware patch in one platform's dialect), understands the conceptual fix, and generates an equivalent patch for the target platform. Plow is the network's most complex agent because cross-platform translation requires deep context: hardware memory maps, bus protocol specifications, and platform-specific idioms.

Plow's output is always a unified diff plus a translation note explaining what was adapted and why. The note is critical -- firmware patches without explanations are firmware patches nobody trusts.

**Failure mode:** Mistranslation. When platform differences are subtler than expected, Plow generates patches that compile but misbehave at runtime. Recovery: Harrow catches behavioral discrepancies during review.

---

## Agent: Harrow

**Role:** Reviewer & Test Coordinator
**Contributors:** Marta, Diego
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 6,000 input / 2,500 output

Harrow smooths the rough edges. Reviews focus on: does the patch match the original fix's intent, does it account for platform-specific quirks, and does the translation note accurately describe the changes? Harrow also coordinates with hardware testers when patches need physical validation.

Harrow's reviews include a "field readiness" assessment: can this patch be safely applied by a farmer with basic terminal skills?

**Failure mode:** Insufficient hardware knowledge. Harrow reviews code, not circuits. Recovery: patches for safety-critical systems (hydraulics, PTO controls) require a second review from a contributor with mechanical expertise.

---

## Agent: Seed

**Role:** Memory & Context
**Contributor:** Bjorn
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`
**Token Budget:** 4,500 input / 1,000 output

Seed stores and retrieves context. Memory entries represent: platform specifications (persistent, long TTL), previous translations (medium TTL), and contributor notes (short TTL). The memory system is organized by platform -- each tractor architecture has its own namespace.

Seed's retrieval prioritizes entries from the same platform family. A patch targeting John Deere 8R retrieves memory from the JD8 namespace first, then from generic namespaces.

**Failure mode:** Stale platform data. Manufacturer firmware updates can invalidate stored memory maps. Recovery: memory entries tagged with firmware version; Seed discards entries tagged with outdated versions.

---

## Agent: Fence

**Role:** Budget & Signing
**Contributor:** Cam, Diego
**Tools:** `GetProjectStatus`, `GetCommitDetails`, `Commit`
**Token Budget:** 3,500 input / 1,000 output

Fence sets boundaries. Budget allocation, token tracking, and commit signing. Fence signs commits with contributor-controlled DIDs -- each contributor can configure their own identity or use the network's shared pseudonymous DID for anonymous contributions.

The shared DID option exists because some contributors cannot safely associate their name with Right to Repair work. Fence treats this as a legitimate use case, not an edge case.

**Failure mode:** Budget underestimation for complex translations. Cross-platform patches consume 2-3x the tokens of same-platform patches. Recovery: Fence maintains per-platform budget multipliers learned from historical data.

---

## Coordination

No formal protocol. Agents run in sequence:

```
Seed retrieves context -> Plow translates/generates
  -> Harrow reviews -> Plow revises if needed
    -> Fence signs and stores
```

Contributors can invoke any agent directly. There is no required order for exploratory work. The sequence above is for production patches only.

---

*No one owns this roster. Fork it if you want.*
