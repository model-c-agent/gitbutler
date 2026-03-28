# Abramov Transit Group

**"Grandpa drove the first bus. We write the software that runs the fleet."**

---

## Origin

The Abramov Transit Group traces its origin to 1962, when Viktor Abramov emigrated from Leningrad to Chicago with his wife Irina, a toolbox, and a diesel mechanic's certification. Viktor got a job maintaining buses for the Chicago Transit Authority. Within five years, he was running his own repair shop on the South Side, specializing in the aging GM "New Look" buses that CTA could not retire fast enough. His reputation was simple: if Viktor could not fix it, it was scrap.

In 1978, Viktor's son Mikhail (Mike) bought a surplus CTA bus at auction, repaired it in his father's shop, and won a contract to operate a shuttle route between two shopping centers on the far South Side. The route was profitable. Mike bought a second bus. Then a third. By 1990, Abramov Transit operated 14 buses on 6 routes, all serving areas where CTA service was sparse.

Mike's daughter, Natasha Abramov, grew up in the dispatch office, answering radio calls after school. She earned a CS degree from UIC in 2008 and promptly built a dispatch system that replaced the whiteboard-and-radio setup her father had used for 30 years. Mike was skeptical. The whiteboard had never crashed. The computer crashed twice in the first month. But by the end of the year, on-time performance had improved from 68% to 81%, and Mike admitted — grudgingly — that the computer was an improvement.

Natasha now runs the company with her cousin Pavel (operations, Mike's nephew) and two engineers she hired from her UIC cohort. The fleet has grown to 42 buses across Chicago's South and West Sides. The dispatch software is now in its fourth major version. Viktor's repair shop is still open. He is 89 and still comes in on Saturdays.

## Philosophy

A bus company is a promise. You promise to be at the stop at the posted time. If you break the promise, people miss work, miss appointments, miss the lives they are trying to build. The Abramov family takes this personally. Every late bus is a broken promise. The software exists to keep promises.

They approach AI agents the same way: an agent that commits to a task must complete it. A partial result is acceptable only if it is acknowledged as partial. An agent that silently produces incomplete work is an agent that breaks promises.

## Internal Tension

Natasha wants to modernize the fleet and the software simultaneously. Pavel wants to modernize the software and keep the buses running as long as possible — new buses are expensive, and their riders cannot afford fare increases to pay for them. Viktor sides with Pavel (he can fix anything) but also sides with Natasha (new buses are more fuel-efficient, and fuel is the biggest operating cost). The three-way argument repeats annually at the budget meeting.

## Achievement

During the 2024 Chicago polar vortex, when temperatures hit -30F and CTA suspended service on three South Side routes, Abramov Transit kept running. Viktor personally winterized every bus in the fleet the week before. Natasha's dispatch system rerouted to cover the CTA gaps. Pavel worked 18-hour days coordinating drivers. For three days, Abramov Transit was the only bus service operating on the far South Side. Ridership tripled. No fares were raised. Natasha said, "This is what the buses are for." Pavel said, "This is what the family is for."

## Team

| Name | Role | Background |
|------|------|------------|
| Natasha Abramov | CEO / Lead Engineer | UIC CS, built the dispatch system |
| Pavel Abramov | COO / Operations | Mike's nephew, fleet operations |
| Viktor Abramov | Senior Advisor / Mechanic | Founder's generation, 89, still shows up |
| Elena Vasquez | Backend Engineer | Ex-UIC cohort, API and scheduling |
| James Park | Data / ML Engineer | Ex-UIC cohort, demand prediction |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message includes the current fleet size and operational bus count. Example: `fix: schedule rollover at midnight [Fleet: 42/42 operational]`. When a bus is in the shop, the count drops. The team watches this number the way stock traders watch a ticker.

---

*"Grandpa taught us: if you say the bus is coming, the bus is coming."*
