# Inter-Agency Space Object Registry

**"If it is not catalogued in triplicate, it does not exist."**

---

## Founding

The Inter-Agency Space Object Registry (IASOR) was established in 2014 by joint memorandum between four national space surveillance agencies who could not agree on a single debris catalogue format. Rather than resolve the disagreement — which had persisted since 1998 — the agencies created IASOR to maintain parallel catalogues in all four formats simultaneously, with a fifth canonical format that none of them use but all of them recognize as authoritative.

IASOR is headquartered in Vienna (chosen because it is near the UN Office for Outer Space Affairs, which lends bureaucratic gravity). It maintains satellite offices in Colorado Springs, Moscow (dormant since 2022), and Canberra. The Registry employs 43 civil servants across four time zones, 11 of whom understand the software. The remaining 32 manage compliance, interagency liaison, and the comment period process.

## Philosophy

IASOR believes in process. Not process as impediment — process as the only mechanism that prevents four sovereign nations from accidentally classifying each other's operational satellites as debris. Every change to the catalogue requires a 45-day comment period, a three-agency sign-off, and a reconciliation step that ensures all four parallel formats remain consistent. This is slow. This is intentional. In IASOR's worldview, a fast update that introduces an inconsistency between catalogues is worse than a slow update that does not.

They approach software development the same way. Every code change is a "registry amendment" that follows the same comment-and-reconciliation pipeline as catalogue updates. Pull requests are "proposed amendments." Code review is "interagency review." Merge is "ratification."

## Internal Tension

The younger engineers (hired post-2020) want to modernize. They have proposed CI/CD pipelines, automated testing, and — most controversially — reducing the comment period from 45 days to 14 for non-critical changes. The senior staff view this as reckless. Deputy Registrar Helena Vasques wrote a 12-page memo explaining why 45 days is the minimum time required to ensure all four time zones have adequate opportunity to review. The memo itself took 45 days to clear internal review.

## Achievement

In 2024, IASOR's reconciliation system detected a discrepancy between the US and Australian catalogues for object 44291 — a discrepancy that would have caused the Australian Space Agency to classify an active NRO satellite as debris. The reconciliation flag triggered a 72-hour interagency consultation that resolved the issue before any operational action was taken. IASOR's director called it "the system working exactly as designed." The engineers called it "a 72-hour Slack thread that could have been a database join."

## Team (Software Division)

| Name | Role | Background |
|------|------|------------|
| Helena Vasques | Deputy Registrar / Software Lead | 18 years ESA, policy and systems |
| Tobias Kreuz | Senior Engineer | Catalogue format specialist, ex-DLR |
| Amara Obi | Backend Engineer | Data reconciliation, ex-Nigerian Space Agency |
| Yuki Sato | DevOps & Compliance | Infrastructure with mandatory audit trails |
| Liam Byrne | Junior Engineer | Fresh from university, impatient, talented |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every document, commit message, and PR carries a "Registry Amendment Number" — a sequential identifier in the format `RA-YYYY-NNNN`. The numbering has never been reset. The current amendment for this proposal is RA-2026-1847. They are aware this is excessive for a commit message. They do not care.

---

*"Amendment proposed. Comment period opens. Forty-five days."*
