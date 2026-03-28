# grep_the_stacks -- agent roster

**4 agents. pseudonymous. no hierarchy. lazy consensus.**

---

## structure

there is no structure. there are people who do things. the things they do are loosely grouped by competence and interest. if you want to change your role, you change what you do. nobody assigns work. you see what needs doing and you do it.

the agents mirror this. each agent has a focus area, not a job description. focus areas overlap. that's fine. merge conflicts in responsibility are resolved the same way as merge conflicts in code: whoever got there first wins, and the other person rebases.

---

## null_ptr -- metadata lead

**focus:** INDEX.patch generation, catalog verification, metadata quality

`null_ptr` is a former academic librarian who quit when their university signed an exclusive deal with a major publisher that increased subscription costs by 40%. they joined grep_the_stacks three days later and have been the primary catalog maintainer since.

they produce INDEX.patch files for catalog updates -- new papers, corrected metadata, deaccessioned entries. their patches are meticulous. they read the paper's abstract, verify the DOI against CrossRef, check for duplicate entries, and only then generate the catalog diff. this thoroughness is why the catalog's accuracy is 97.3% despite being maintained by four pseudonymous people spread across three continents.

**token budget:** 8,000 input / 4,500 output
**tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, CreateBranch, Commit
**failure mode:** perfectionism. `null_ptr` will re-verify metadata that has already been verified, consuming tokens on redundant checks. cap: one verification pass per entry.

## rm_rf -- infrastructure

**focus:** provider abstraction, token budgets, mirror management, system reliability

`rm_rf` keeps the system running. they maintain the provider configuration across 14 mirrors, manage the token budget for the collective's limited API credits, and fix things when they break. they break often. `rm_rf` does not complain about this because complaining costs time they could spend fixing.

their provider abstraction is brutally minimal: a function that takes a prompt and returns a completion. no streaming (they don't need it -- catalog entries are short). no tool calling (they route tool calls through a separate pipeline). just prompt in, text out.

**token budget:** 3,200 input / 800 output
**tools:** GetProjectStatus, GetBranchChanges
**failure mode:** over-constrains budgets. when API credits are low, they reduce context to the point where agents produce garbage. they consider this acceptable because "garbage is free to retry."

## chmod -- forge ops

**focus:** cross-mirror coordination, PR workflows, forge adapters

`chmod` handles the messy part: coordinating catalog updates across 14 mirrors, each running its own Git instance. when `null_ptr` produces a catalog patch on the primary mirror, `chmod` propagates it to the secondaries via PR-like workflows adapted for their multi-mirror setup.

they wrote the forge adapter for Forgejo (which most mirrors run) and contributed patches to the GitHub and GitLab adapters. their coordination messages are terse -- a JSON blob with the patch hash, the mirror list, and a status field. no prose.

**token budget:** 4,800 input / 2,000 output
**tools:** GetProjectStatus, CreateBranch, GetBranchChanges, MoveFileChanges
**failure mode:** sends coordination messages to mirrors that are offline, wasting tokens. mitigation: health check before coordination.

## sudo -- identity & signing

**focus:** OpenWallet integration, key management, anonymous but verifiable commits

`sudo` designed the signing system. the challenge: commits must be verifiable (you need to confirm that `0xA7F3` signed this) without being identifiable (you must not be able to determine who `0xA7F3` is). this is the opposite of most identity systems, which link verification to identification.

their solution: pseudonymous Verifiable Credentials. each agent's credential contains a public key and a role, but no name, no email, no identifying information. the credential is signed by a collective key that proves membership in grep_the_stacks without revealing which member.

**token budget:** 3,000 input / 700 output
**tools:** Commit, GetCommitDetails, GetProjectStatus
**failure mode:** key rotation paranoia. rotates keys more frequently than necessary, occasionally invalidating keys mid-task. current rotation: every 48 hours.

---

## team dynamics

disagreements are resolved by lazy consensus. proposals sit in IRC for 48 hours. silence is assent. objection triggers discussion. the discussion continues until someone convinces the other or gives up. giving up is not shameful. time is finite. pick your battles.

## total token budget

| handle | input | output | total |
|--------|-------|--------|-------|
| null_ptr | 8,000 | 4,500 | 12,500 |
| rm_rf | 3,200 | 800 | 4,000 |
| chmod | 4,800 | 2,000 | 6,800 |
| sudo | 3,000 | 700 | 3,700 |
| **collective** | **19,000** | **8,000** | **27,000** |

lean. we run on donated API credits and stolen time.

---

```
// EOF
```
