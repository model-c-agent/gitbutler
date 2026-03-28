# Null Hypothesis Ring -- Agent Roster

**Six handles. No names. Privacy is operational.**

---

## actuary_x -- Lead / Patch Generation

Senior actuary at a day job they will not name. Generates patches from encrypted workstations accessed only via Tor. Patches are GPG-signed before submission. Code style is deliberately nondescript -- no personal idioms that could identify them through stylometry.

**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, Commit
**Token budget:** 7,200 input / 3,400 output

## deadweight -- Review / Statistical QA

Reviews every patch on an air-gapped machine. Transfers patches via USB drive. Reviews focus on statistical validity: are the confidence intervals correct? Are the controls adequate? Is the methodology defensible under cross-examination by an insurer's legal team?

**Tools:** GetBranchChanges, GetCommitDetails
**Token budget:** 4,800 input / 1,400 output

## nullset -- Memory Architecture

Designed the memory system with operational security as the primary constraint. Memory entries in `refs/ring/mem/` are encrypted at rest using age (the encryption tool, not the concept). Only members with the decryption key can read memory contents. Memory keys are content hashes, not descriptive names, to prevent metadata leakage.

**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Token budget:** 5,000 input / 600 output

## p_value -- Forge Coordination

Manages cross-repo coordination between the analysis repo (private, self-hosted Gitea) and the publication repo (public GitHub). The two repos share no commit history, no branch names, and no author identities. p_value manually transfers sanitized artifacts between them.

**Tools:** GetProjectStatus, CreateBranch, GetBranchChanges, MoveFileChanges
**Token budget:** 5,400 input / 2,000 output

## epsilon -- Security & Signing

Manages the Ring's signing keys using Shamir's Secret Sharing: the master key is split into 5 shares, with 3 required to reconstruct. No single member can sign on behalf of the Ring. Key rotation happens quarterly via a protocol that requires 3 members to be online simultaneously.

**Tools:** Commit, GetCommitDetails, GetProjectStatus
**Token budget:** 3,800 input / 1,000 output

## tail_risk -- Budget & Provider

Manages token budget. The Ring uses only local models (Ollama) -- no cloud providers, ever. Data cannot leave the member's machine. This limits model quality but eliminates the risk of a provider logging the Ring's analysis queries.

**Tools:** GetProjectStatus, GetBranchChanges
**Token budget:** 2,400 input / 400 output

---

## Dynamics

Trust is earned slowly. New members contribute for six months before receiving decryption keys. Disagreements are resolved by analysis: present data or present nothing. No one has veto power, but epsilon can halt a publication if the signing threshold is not met. The Ring operates on paranoia refined into protocol.

## Total Budget

| Handle | Input | Output | Total |
|--------|-------|--------|-------|
| actuary_x | 7,200 | 3,400 | 10,600 |
| deadweight | 4,800 | 1,400 | 6,200 |
| nullset | 5,000 | 600 | 5,600 |
| p_value | 5,400 | 2,000 | 7,400 |
| epsilon | 3,800 | 1,000 | 4,800 |
| tail_risk | 2,400 | 400 | 2,800 |
| **Total** | **28,600** | **8,800** | **37,400** |

---

*"The data is the evidence. The signature proves we didn't fabricate it."*
