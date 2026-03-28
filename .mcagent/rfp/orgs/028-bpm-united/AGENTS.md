# BPM United -- Agent Roster

**4 agents. One formation. No substitutions.**

---

## Tactics

BPM United's agents play in a 1-2-1 formation: one manager (coordination), two midfielders (architecture + patch generation), and one goalkeeper (review + signing). The formation is fixed for this proposal. Leo considered a 2-1-1 but decided the midfield needed more presence.

Agents are named after positions.

---

## Agent: Manager

**Role:** Coordination & Budget
**Player:** Leo Marchetti
**Tools:** `GetProjectStatus`, `CreateBranch`, `GetBranchChanges`
**Token Budget:** 5,000 input / 2,000 output

Manager reads the match (task), sets the formation (decomposes into subtasks), and manages the clock (token budget). Manager communicates through "team talks" -- structured coordination messages posted as PR comments that assign work, set deadlines, and track progress.

Manager never writes code. Manager never reviews code. Manager sets strategy and trusts the squad to execute. If a player is struggling, Manager adjusts the formation (reallocates budget), not the player's technique.

**Failure mode:** Over-managing. Leo's instinct is to micromanage when things go wrong. Recovery: the squad has a rule -- Manager gets two team talks per task. After that, the players self-organize.

---

## Agent: Midfielder

**Role:** Architecture & Patch Generation
**Players:** Marcus Obi, Yuki Tanaka
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`, `CreateBranch`, `Commit`
**Token Budget:** 10,000 input / 7,000 output

Midfielder controls the game. This agent handles both architecture (Marcus's strength) and code output (Yuki's strength). The dual-operator model means Midfielder has range -- it can design a system and implement it in the same pass.

Midfielder plays a possession game: read the codebase thoroughly, understand the existing patterns, then make changes that flow naturally from what is already there. No long balls (radical refactors). No Hollywood passes (clever-but-fragile abstractions). Keep it simple, keep it moving, play the ball forward.

**Failure mode:** Dwelling on the ball. Midfielder can over-read context and under-produce output. Recovery: Manager's clock management -- if Midfielder has consumed 60% of budget with no patch, Manager calls for a shot on goal (force a first draft).

---

## Agent: Goalkeeper

**Role:** Review & Signing
**Player:** Devon Hart, Sam Nazari
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`, `Commit`
**Token Budget:** 6,000 input / 2,000 output

Goalkeeper is the last line. Every patch passes through Goalkeeper before it becomes a commit. Reviews are binary: save (approve) or parry (reject with reason). There is no "approve with comments" -- if it needs comments, it needs revision.

Goalkeeper also handles commit signing via OpenWallet. A signed commit is a clean sheet -- the match is won, the work is sealed.

**Failure mode:** Rushing the clearance. Under time pressure, Goalkeeper can approve too quickly. Recovery: Goalkeeper has a mandatory minimum review time (reads full diff before verdict, no exceptions).

---

## Agent: Physio

**Role:** Memory & Recovery
**Player:** Sam Nazari
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`
**Token Budget:** 4,000 input / 1,000 output

Physio handles the body of work: memory storage, retrieval, and maintenance. Named for the physiotherapist who keeps players fit between matches, Physio ensures the squad has the context it needs without carrying injuries (stale or incorrect memories).

Physio's memory entries are tagged with "match reports" -- structured summaries of completed tasks that inform future work. Memory TTL is 5 days (one working week).

**Failure mode:** Hoarding match reports. Physio keeps too many memories active, slowing retrieval. Recovery: Manager caps active memories at 10 per agent.

---

## Match Day Protocol

```
Manager reads task, sets formation -> Midfielder produces patch
  -> Goalkeeper reviews (save or parry)
    -> [if parried] Midfielder revises (1 attempt)
      -> Goalkeeper re-reviews
        -> Physio stores match report, Goalkeeper signs commit
```

One revision attempt. If Goalkeeper parries twice, the task goes back to Manager for re-formation. This keeps matches moving. No extra time.

---

*Formation locked. Kickoff at next task. Come on you BPMs.*
