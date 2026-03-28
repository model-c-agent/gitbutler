# Office of Actuarial Oversight & Compliance

**"Form 7B-AO must be submitted in triplicate before any code review may commence."**

---

## Mandate

The Office of Actuarial Oversight & Compliance (OAOC) is a regulatory body established by ministerial decree in 2019 to oversee the use of algorithmic pricing models in the insurance sector. Headquartered in a brutalist concrete building in The Hague that was originally designed as a parking garage, the Office employs 47 civil servants, of whom 12 are actuaries, 8 are software auditors, and the remaining 27 handle administrative processes that the Office has not yet found a way to automate.

The Office produces annual compliance reports that average 340 pages. The reports are thorough, meticulously sourced, and read by approximately six people, four of whom work at the Office.

## Origin of the Software Initiative

In 2024, the Ministry of Finance directed the Office to develop internal tooling for auditing AI-assisted actuarial models used by insurance companies. The directive was four sentences long. The Office's response was a 78-page implementation plan that required three rounds of ministerial approval, took eleven months to finalize, and resulted in a budget allocation that arrived six weeks before the fiscal year ended.

With the remaining budget, the Office hired a contractor (who quit after two months), inherited a half-finished Python codebase, and eventually assembled an internal team of four civil servants who had relevant programming experience and were willing to work on "the software thing" in addition to their existing duties.

GitButler was adopted after a procurement process that involved evaluating seventeen version control platforms against a 200-item requirements matrix. GitButler scored highest because it was the only tool that supported virtual branches, which the evaluation committee interpreted (correctly, if accidentally) as "parallel compliance review tracks."

## Philosophy

Process is protection. The Office does not move fast because moving fast in regulation means missing edge cases that become scandals. Every decision is documented. Every document is reviewed. Every review is logged. The audit trail is not a byproduct of the work -- it *is* the work.

The Office approaches AI agents with the same procedural rigor: an agent is a system that must be authorized, monitored, audited, and periodically re-authorized. Deploying an agent without a compliance review is the regulatory equivalent of deploying a pricing model without actuarial sign-off. It simply does not happen.

## Internal Tension

**The Modernization Frustration.** Pieter (lead developer, 38) wants to ship features faster. He has proposed reducing the approval process from three stages to one for low-risk changes. Director van der Berg (59) considers all changes potentially high-risk until proven otherwise and has denied the proposal twice. Pieter has started documenting the cost of delayed features in a spreadsheet he calls "The Price of Process." Director van der Berg has not read it. She is busy reviewing Form 7B-AO submissions.

## Notable Achievement

In 2025, the Office completed its first fully automated compliance audit of an insurer's AI pricing model. The audit, which would normally take a team of three actuaries four weeks, was completed in three days using the Office's tooling. The result was a 412-page report generated from structured audit data. Director van der Berg called it "an adequate start." Pieter called it "proof that we can do this." Both are correct.

## Team Overview

| Agent | Role | Civil Service Grade |
|-------|------|---------------------|
| Dir. van der Berg | Approval Authority | Senior Executive |
| Pieter | Lead Developer / Patches | Grade 10 |
| Annelies | Compliance Review | Grade 9 |
| Mads | Memory / Audit Trail | Grade 8 |
| Fatima | Forge Coordination | Grade 8 |
| Henrik | Security, Budget, Signing | Grade 9 |

Details in [AGENTS.md](AGENTS.md).

---

*"Compliance is not optional. Neither is the paperwork."*
