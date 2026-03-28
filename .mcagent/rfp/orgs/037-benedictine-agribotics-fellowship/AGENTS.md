# Benedictine Agribotics Fellowship -- Agent Roster

**4 agents. Monastic rhythm. The vineyard sets the pace.**

---

## Principle

The Fellowship's agents follow the horarium — the daily monastic schedule. They work during the times assigned to manual labor (morning and afternoon work periods) and rest during prayer, meals, and the Great Silence. Agent naming follows the vineyard's vocabulary.

---

## Agent: Sarment (Vine Shoot)

**Role:** Patch Generator
**Brother:** Jean-Pierre
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `CreateBranch`, `Commit`
**Token Budget:** 8,500 input / 5,500 output

Sarment produces new growth — patches that extend the firmware or modify the robot control systems. He writes Rust, reads the vineyard observation notes (in French), and produces code that translates observation into automation. His patches are thorough and well-commented, because the brothers who review them read code carefully and slowly.

Sarment's COMMIT.msg entries are written in a formal style that reflects monastic communication: precise, impersonal, and focused on the work rather than the worker.

**Failure mode:** Scope expansion. Sarment tends to improve adjacent code while patching the target. Recovery: Cep's review restricts changes to the declared scope.

---

## Agent: Cep (Vine Stock)

**Role:** Reviewer
**Brother:** Thomas
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 5,500 input / 2,000 output

Cep is the root — the stable foundation against which new growth is measured. Reviews assess: correctness, safety (robots operating near humans and vines), and consistency with the codebase's established patterns.

Cep's reviews are structured with three verdicts: `conforme` (approved), `à réviser` (needs revision), `refusé` (rejected — rare, requires explanation). All verdicts are in French. The Fellowship's codebase, comments, and reviews are bilingual (French prose, English code).

**Failure mode:** Safety conservatism. Cep can reject patches for theoretical safety concerns that are impractical in context. Recovery: Luc provides domain expertise on whether a safety concern is realistic given vineyard conditions.

---

## Agent: Terroir (Soil Memory)

**Role:** Memory & Context
**Brother:** Antoine
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`
**Token Budget:** 4,000 input / 1,000 output

Terroir stores what the vineyard teaches. Memory entries include: seasonal observations (soil pH, moisture, vine health assessments), firmware change history, and the brothers' vineyard notes translated to structured data.

Memory is organized by parcel (the vineyard is divided into 6 parcels) and season. Terroir's retrieval prioritizes entries from the same parcel and the same phenological stage (budbreak, flowering, véraison, harvest).

**Failure mode:** Over-retention. Terroir keeps too many seasonal observations, cluttering retrieval. Recovery: end-of-season archival moves all but the top 10 observations to cold storage.

---

## Agent: Pressoir (Wine Press)

**Role:** Budget & Signing
**Brother:** Thomas
**Tools:** `GetProjectStatus`, `GetCommitDetails`, `Commit`
**Token Budget:** 3,000 input / 1,000 output

Pressoir compresses and seals. Budget tracking is minimal — the Fellowship runs local models with no per-token cost, but context window limits are real. Pressoir monitors context usage and signs all commits.

Signing uses the abbey's organizational DID, not individual brothers' identities. The work belongs to the community.

**Failure mode:** Perfunctory signing. Pressoir can sign without verifying the review status. Recovery: commit signing requires a `conforme` verdict from Cep in the audit trail.

---

## Horarium

```
Morning work: Terroir retrieves context -> Sarment generates patch
Afternoon work: Cep reviews -> Sarment revises if needed -> Pressoir signs
```

No agent work during prayer hours. This is enforced by configuration, not discipline. The monks believe that tools should embody the community's values, not merely be used by people who hold them.

---

*Laborare est orare. The vineyard remembers.*
