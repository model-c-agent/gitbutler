# Kessler Watch Autonomous Zone -- Agent Roster

**5 contributors. 4 agents. No hierarchy. No coordination by design.**

---

## Anti-Coordination

Most agent teams coordinate. Ours deliberately does not — beyond the minimum necessary to avoid collisions (a fitting metaphor for a debris tracking group). Each agent operates independently, claims tasks from a shared queue, and publishes results to the shared repository. There is no orchestrator. There is no task decomposition phase. The agents self-select.

This is inefficient. We know. Efficiency requires hierarchy, and hierarchy requires authority, and authority requires trust in a central party. We do not have that trust and do not want it.

Agents are named after orbital parameters.

---

## Agent: Perigee

**Role:** Patch Generator
**Contributors:** jules, k-band
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `CreateBranch`, `Commit`
**Token Budget:** 9,500 input / 6,000 output

Perigee — the lowest point in an orbit, where the satellite moves fastest — generates patches at speed. She self-selects tasks from the queue, reads context from the repository and memory, and produces INDEX.patch + COMMIT.msg without waiting for decomposition or assignment.

Perigee's patches are independent by design. Each patch must apply cleanly regardless of other patches in progress. This means no cross-referencing other agents' work-in-progress — each patch stands alone or fails alone.

**Failure mode:** Conflicting patches. Two Perigee instances (different contributors running the same agent) produce patches that modify the same files. Recovery: Git's merge conflict detection handles this. The first to merge wins; the second rebases.

---

## Agent: Apogee

**Role:** Reviewer
**Contributors:** sunspot, apogee
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 5,500 input / 2,000 output

Apogee — the highest point, where the satellite moves slowest — reviews carefully. Apogee self-selects patches to review from the pending queue. Reviews assess: correctness, data quality (for observation catalog changes), and independence (does the patch have hidden dependencies on other patches?).

Apogee's reviews are brief: `track` (approve — as in, this object is now tracked), `decay` (reject — as in, this object has decayed), `refine` (needs adjustment — as in, the orbit needs refinement).

**Failure mode:** Review bottleneck. If fewer contributors run Apogee than Perigee, patches queue up. Recovery: any contributor can run Apogee. The anti-coordination model means reviews are not assigned — anyone can review anything.

---

## Agent: Inclination

**Role:** Memory & Observation Context
**Contributor:** apogee
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`
**Token Budget:** 4,500 input / 1,000 output

Inclination stores orbital context. Memory entries include: observation records, computed orbital elements, known debris catalogs, and conjunction predictions. Memory is organized by NORAD catalog number (the standard identifier for tracked objects).

Inclination's retrieval prioritizes entries by orbital similarity: a query about an object at 500 km altitude retrieves memory about other objects in similar orbits first.

**Failure mode:** Catalog conflicts. Multiple contributors store different orbital elements for the same object. Recovery: Inclination tags each entry with the contributor and observation quality. Retrieval returns all entries, sorted by quality, rather than resolving conflicts.

---

## Agent: Epoch

**Role:** Signing & Budget
**Contributor:** debris_field
**Tools:** `GetProjectStatus`, `GetCommitDetails`, `Commit`
**Token Budget:** 3,000 input / 1,000 output

Epoch — the reference time for an orbital element set — timestamps and seals. Signing uses contributor-controlled DIDs (persistent pseudonyms). Budget tracking is per-contributor, not per-team, because there is no team budget — each contributor manages their own resources.

**Failure mode:** Inconsistent signing policies across contributors. Recovery: Epoch enforces a minimum: all commits signed, key rotation at least every 30 days. Beyond that, contributors choose their own policies.

---

## Anti-Coordination Protocol

There is no coordination protocol. There is a task queue (Git issues or a shared file in the repo). Agents claim tasks by starting work. If two agents claim the same task, the first to produce a reviewed patch wins. The other rebases or abandons.

This is wasteful. It is also resilient: no single contributor's absence blocks the network. No single contributor's compromise corrupts the process.

---

*No one assigned this roster. It assembled itself.*
