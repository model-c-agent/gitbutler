# root@field -- Agent Roster

**4 agents. Anonymous operation. No telemetry home.**

---

## Principles

root@field's agents follow three rules:
1. No data leaves the local machine without the operator's explicit consent.
2. No agent stores operator-identifying information.
3. All agent code is auditable — no compiled-only components, no obfuscated logic.

These rules exist because our users are modifying firmware on equipment owned by hostile manufacturers. The agents must be tools of liberation, not new instruments of surveillance.

Agents are named after UNIX commands.

---

## Agent: patch

**Role:** Patch Generator
**Operators:** sprocket, combine_queen
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `CreateBranch`, `Commit`
**Token Budget:** 10,000 input / 7,000 output

`patch` generates firmware modifications. She reads the target firmware's memory map, the CAN bus protocol documentation, and the collective's reverse-engineering notes, then produces a unified diff that implements the requested change.

`patch` strips all environment metadata from output: no timestamps in diffs, no hostnames in comments, no file paths outside the repository. Every byte of output is auditable for information leakage.

**Failure mode:** Incomplete reverse-engineering data. When the firmware map is partially documented, `patch` fills gaps with assumptions that may be wrong. Recovery: `diff` catches undocumented assumptions during review and flags them for manual verification on hardware.

---

## Agent: diff

**Role:** Reviewer
**Operators:** harvest_moon, ph7
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 6,000 input / 2,500 output

`diff` reviews for: correctness (does the patch do what it claims?), safety (can it brick the equipment?), and privacy (does it leak operator information?). The safety review is critical — a bad firmware patch can disable a tractor's safety systems. `diff` maintains a list of safety-critical memory regions per platform and rejects any patch that modifies them without explicit justification.

Reviews: `LGTM` (approve), `NACK` (reject with reason), `NEEDS-HW-TEST` (cannot approve without hardware verification).

**Failure mode:** False NACK on safety grounds for patches that modify regions near (but not in) safety-critical areas. Recovery: sprocket provides hardware expertise to distinguish adjacent-but-safe from actually-dangerous modifications.

---

## Agent: .git

**Role:** Memory & Context
**Operator:** dirt_clock
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`
**Token Budget:** 4,500 input / 1,000 output

`.git` stores collective knowledge: reverse-engineered memory maps, CAN bus message definitions, known bugs per platform, and contributor-submitted field reports. Memory is encrypted at rest (AES-256-GCM, key from local config).

Retrieval prioritizes entries by platform and firmware version. A query for "JD8R firmware 4.2" returns entries specific to that version first, then entries for the JD8R family, then generic entries.

**Failure mode:** Stale reverse-engineering data. Manufacturer firmware updates invalidate stored memory maps. Recovery: entries are tagged with the firmware version they apply to; queries for a new version return no results rather than outdated ones.

---

## Agent: chmod

**Role:** Signing & Budget
**Operators:** ph7, zero_till
**Tools:** `GetProjectStatus`, `GetCommitDetails`, `Commit`
**Token Budget:** 3,500 input / 1,000 output

`chmod` changes permissions — specifically, it controls who can seal a commit. Signing uses ephemeral DIDs (new identity per session) to prevent manufacturers from building contributor profiles.

Budget tracking is minimal. Most contributors run local models (free) on personal hardware. `chmod` tracks context window usage rather than dollar cost.

**Failure mode:** Ephemeral DID confusion. When multiple contributors work on the same repo, ephemeral DIDs make it hard to correlate commits to review history. Recovery: `chmod` supports an optional persistent pseudonym mode where the DID is stable across sessions but not linkable to a real identity.

---

## Workflow

```
.git retrieves context -> patch generates firmware mod
  -> diff reviews (correctness + safety + privacy)
    -> patch revises if needed -> chmod signs with ephemeral DID
```

All steps run locally. Nothing touches the network until the contributor explicitly pushes to Gitea.

---

```
root@field:~$ ls agents/
patch  diff  .git  chmod
root@field:~$
```
