# Dominguez Orbital Services

**"Three generations of watching the sky. We are not stopping now."**

---

## Origin

The Dominguez family has been tracking objects in orbit from the Canary Islands since 1989, when Abuelo Esteban Dominguez — a retired Spanish Navy radar technician — pointed a surplus military tracking antenna at the sky from his rooftop in La Laguna, Tenerife. Esteban had spent twenty years tracking ships for the Navy and saw no reason the same principles would not work for satellites. He was right. His hand-built tracking station, powered by a Commodore 64 running custom BASIC programs, produced orbital observations accurate enough to contribute to the amateur satellite tracking community within its first year.

Esteban's daughter, Carmen Dominguez-Reyes, took over in 2003. Carmen was an electrical engineer who had worked for Telefonica before returning to Tenerife to care for her aging father. She modernized the station: replaced the Commodore with a Linux box, added a second antenna for optical tracking, and wrote the first version of the tracking software that the family still uses (heavily modified) today. Carmen's contribution was automation — she built a system that could track, catalogue, and report debris without human intervention for 72 hours at a time, allowing the station to operate while she slept.

Now it is 2026 and Carmen's son, Diego Dominguez, runs the operation. Diego is 29, has a degree in aerospace engineering from UPM in Madrid, and worked briefly at GMV before returning to Tenerife because he missed the station and because his grandmother's paella is better than anything in Madrid. Diego employs his cousin Lucía (software engineer, returned from Barcelona), and two childhood friends: Mateo (hardware and RF) and Sofía (data science, remote from Las Palmas).

The station occupies the rooftop and the third floor of the family's building in La Laguna. The second floor is the family apartment. The ground floor is a bodega that Carmen still runs on weekends.

## Philosophy

Family businesses survive by being reliable, not revolutionary. Dominguez Orbital Services does not chase the latest technology. They upgrade carefully, test thoroughly, and maintain backward compatibility with observation formats going back to Esteban's original Commodore logs. Their software reflects this: conservative, well-documented, and built to last decades.

They believe AI agents should behave like family employees: trustworthy, accountable, and present for the long term. An agent that produces brilliant work for one task and crashes on the next is not reliable. Reliability is the metric that matters.

## Internal Tension

Diego wants to expand — more antennas, more clients, maybe a second station on La Palma. Carmen thinks the station is the right size and that growth means debt, which means risk. The argument happens every Sunday over paella. Diego always loses because Carmen controls the bodega, and the bodega subsidizes the station.

## Achievement

In 2024, the Dominguez station achieved 99.7% uptime over 365 days — the highest of any single-site amateur tracking station in the IAU catalogue. The 0.3% downtime was caused by a power outage during a Calima (Saharan dust storm) that also knocked out half of Tenerife's grid. Mateo had the backup generator running within 40 minutes. Diego considers the Calima incident a failure. Carmen considers 99.7% a miracle and wishes Diego would relax.

## Team

| Name | Role | Background |
|------|------|------------|
| Diego Dominguez | Director / Aerospace Eng | UPM, ex-GMV, runs the station |
| Lucía Dominguez-Vega | Lead Software Engineer | Cousin, ex-Barcelona fintech |
| Mateo Ruiz | Hardware & RF | Childhood friend, self-taught RF wizard |
| Sofía Herrera | Data Science | Childhood friend, remote from Las Palmas |
| Carmen Dominguez-Reyes | Advisor / Bodega Owner | Mother, built the automation layer |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message includes the local weather at the station in La Laguna, because atmospheric conditions affect tracking accuracy and Carmen insists on recording them. Example: `fix: TLE parser edge case [La Laguna: 22C, clear, seeing 3/5]`.

---

*"Abuelo pointed the antenna. We keep it aimed."*
