# The Provident Order

**"We calculate risk so that the vulnerable are never abandoned to chance."**

---

## Vocation

The Provident Order is a lay religious community founded in 1987 in Cork, Ireland, by Brother Declan Riordan, a Franciscan friar and qualified actuary. The Order operates at the intersection of faith and finance: its members are trained actuaries who calculate risk for charitable mutual aid pools serving communities that commercial insurers refuse to cover.

Sixteen members live communally in a converted schoolhouse outside Cork. They take vows of simplicity (not poverty -- actuarial software licenses are expensive), service, and transparency. Their work funds itself: the mutual aid pools they administer generate modest fees that cover the Order's operating costs. Any surplus goes to the pools' beneficiaries.

The populations they serve include smallholder farmers in sub-Saharan Africa, informal-sector workers in Southeast Asia, and migrant communities in Europe. These groups are "uninsurable" by commercial standards -- not because their risk is unmanageable, but because the commercial industry's overhead makes small-premium policies unprofitable. The Order's stripped-down administration (no shareholders, no marketing, no executive bonuses) makes micro-insurance viable.

## The Software Dimension

In 2023, Brother Declan (now 71, still coding in R) realized the Order's actuarial models were running on spreadsheets maintained by two people. If those two people were unavailable, the mutual aid pools would have no way to calculate premiums, process claims, or assess reserves. The Order's mission depended on software that no one else could maintain.

A visiting volunteer (a former software engineer on sabbatical) introduced version control, code review, and documentation practices. GitButler was adopted in 2025 because the Order's workflow -- multiple actuaries working on different aspects of the same risk model -- mapped naturally to virtual branches.

The `but-ai` RFP represents an opportunity to automate the mechanical parts of actuarial model development, freeing the Order's limited human capacity for the pastoral work that computers cannot do: visiting communities, understanding local conditions, and making the judgment calls that no algorithm should make alone.

## Philosophy

Risk is a burden shared. Insurance, in its original form, was mutual aid: a community pooling resources so that no individual bears catastrophic loss alone. The commercial insurance industry inverted this, turning mutual aid into profit extraction. The Order seeks to restore the original purpose.

They view AI agents as fellow workers in this mission. An agent that automates premium calculation is performing an act of service. But an agent that makes coverage decisions without human oversight is performing an act of presumption. The boundary is clear: calculation is delegated; judgment is not.

## Internal Tension

**The Automation Boundary.** Sister Aoife (32, youngest member, data scientist background) wants agents to handle initial claim assessments -- flagging straightforward claims for automatic approval. Brother Declan insists that every claim be reviewed by a human, because "behind every claim is a person, and persons deserve to be seen by persons." The Order has not resolved this. Sister Aoife's prototype sits in a branch, unmerged.

## Notable Achievement

In 2025, the Order's micro-insurance pool for smallholder farmers in Kenya covered 4,200 families against drought-related crop failure. When a drought struck in October, the pool paid out 93% of valid claims within 14 days -- compared to the commercial industry average of 60 days. Brother Declan attributes the speed to their simplified claims process. Sister Aoife attributes it to the data pipeline she built. Both are right.

## Team Overview

| Agent | Role | Status |
|-------|------|--------|
| Br. Declan | Memory / Wisdom Keeper | Founder, semi-retired |
| Sr. Aoife | Patch Generation | Data scientist |
| Br. Thomas | Review / Actuarial QA | Fellow of SOA |
| Sr. Maria | Forge Coordination | Former NGO logistics |
| Br. Liam | Security & Signing | IT background |
| Sr. Grace | Budget & Provider | Accountant |

Details in [AGENTS.md](AGENTS.md).

---

*"Providence is not passivity. It is preparation meeting faith."*
