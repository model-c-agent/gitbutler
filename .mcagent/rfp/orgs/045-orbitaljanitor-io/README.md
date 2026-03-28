# OrbitalJanitor.io

**"Someone has to clean up. Might as well get paid."**

---

## Origin

OrbitalJanitor.io was founded in 2023 by Priya Nair, a former satellite operations engineer at ISRO, and Marcus Cho, an aerospace MBA who spent five years at a NewSpace launch provider watching payload fairings become permanent orbital litter. They met at the 2022 Inter-Agency Space Debris Coordination Committee meeting in Darmstadt, where Priya presented a paper on autonomous rendezvous maneuvers and Marcus was trying to sell carbon credits to anyone who would listen.

Marcus's pitch was simple: satellite operators already pay insurance against collision. What if they could pay to reduce the risk instead? Remove a piece of debris, earn a "debris removal credit" tradeable against insurance premiums. Priya's contribution was the technical side: a software platform that identified which debris objects posed the highest actuarial risk and computed optimal removal sequences.

They raised a pre-seed round in Singapore, hired three engineers, and built their MVP in a co-working space above a hawker center in Geylang. The product is a risk-scoring engine that ingests TLE data from CelesTrak, commercial radar tracking from LeoLabs, and proprietary conjunction assessments to rank every catalogued object by removal value. Satellite operators buy credits. Debris removal missions redeem them.

## Philosophy

Space is a commons. If nobody cleans it, everyone loses access. OrbitalJanitor treats orbit management the way a janitor treats a building: the work is unglamorous, essential, and chronically undervalued. Their software reflects this ethos — practical, no-frills, obsessively reliable.

They believe AI agents should operate like orbital maintenance drones: autonomous within strict constraints, always reversible, never exceeding authority. An agent that force-pushes is an agent that creates debris.

## Internal Tension

Priya and Marcus disagree about transparency. Marcus wants the risk-scoring algorithm to be proprietary — it is their competitive moat. Priya wants to open-source the core model so that regulatory agencies can audit it. The compromise is a "verifiable scoring" protocol where results can be independently validated without revealing the model weights. Neither founder is satisfied.

## Achievement

In Q3 2025, OrbitalJanitor's risk model correctly predicted the fragmentation event of a defunct Russian ELINT satellite 72 hours before it occurred, allowing three commercial operators to execute avoidance maneuvers. The event generated 340 new trackable debris objects. Without the early warning, at least one active satellite would have been hit. This single prediction justified the company's entire first year of operating costs.

## Team

| Name | Role | Background |
|------|------|------------|
| Priya Nair | CEO / Technical Lead | 9 years ISRO satellite ops, orbital mechanics PhD |
| Marcus Cho | COO / Business Lead | Aerospace MBA, ex-Rocket Lab biz dev |
| Suki Tanaka | Data Engineer | TLE processing, ex-LeoLabs |
| Ren Osei | ML Engineer | Collision probability models, ex-ESA/ESOC |
| Davi Almeida | Full-Stack / DevOps | Platform engineering, ex-Grab |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message includes the current number of trackable objects in LEO (sourced from Space-Track.org at commit time). Example: `fix: rate limiter on TLE ingest [LEO: 28,441]`. They say it is a reminder that the problem is always growing.

---

*"Orbits are free. Collisions are expensive."*
