# Transit Operations Command

**"Buses are supply chain. Passengers are cargo. Schedule is mission."**

---

## Origin

Transit Operations Command (TOC) was founded in 2019 by Colonel (Ret.) James Harker, US Army Transportation Corps, who spent 26 years running military forward supply chain operations in Afghanistan, Iraq, and South Korea. Upon retirement, Harker took a consulting job with the Omaha Metropolitan Transit Authority and was horrified by what he found: routes planned by intuition, schedules maintained on spreadsheets, no real-time tracking, and a dispatch system that consisted of a whiteboard and a two-way radio.

"I have seen forward operating bases in Kandahar Province with better logistics discipline than this bus network," Harker told the transit authority board. They thought he was exaggerating. He was not.

Harker recruited three fellow veterans — Major Diane Okafor (logistics automation), Captain Ravi Mehta (communications systems), and Sergeant First Class Tomoko Ishii (maintenance scheduling) — and built Transit Operations Command as a consulting-turned-software firm that applies military forward supply chain methodology to public transit. Their product is a real-time operations platform that treats every bus as a forward asset, every route as a supply line, and every schedule deviation as a mission-critical event.

TOC now manages transit operations for six mid-size American cities. Their on-time performance is 94.2% — the national average for comparable systems is 76%.

## Philosophy

Discipline is not bureaucracy. It is the elimination of ambiguity so that when things go wrong — and they always go wrong — every operator knows exactly what to do. TOC's software reflects military doctrine: standard operating procedures, clear chains of responsibility, and after-action reviews for every incident.

They approach AI agents the same way: agents operate under rules of engagement. What they can do. What they cannot do. When to escalate. When to proceed autonomously. An agent without rules of engagement is an unsupervised private with a fuel card — technically empowered, practically dangerous.

## Internal Tension

Harker runs TOC like a military unit. Okafor, who left the Army partly because of its rigidity, pushes for more flexibility. She argues that transit systems are not battlefields and that rigid SOPs cause agents to produce suboptimal patches when the situation does not match the procedure. Harker responds that flexibility without discipline is chaos. The compromise: agents follow SOPs by default but can request "deviation authority" for non-standard situations, logged and reviewed in the after-action report.

## Achievement

In 2025, TOC's system managed the emergency rerouting of 140 buses in Kansas City during a flash flood event. The platform identified affected routes, computed alternatives, dispatched updated schedules to drivers, and published passenger alerts — all within 8 minutes of the National Weather Service alert. Zero buses were stranded. Zero passengers were left at flooded stops. Harker called it "the best OPORD I have ever executed."

## Team

| Name | Role | Background |
|------|------|------------|
| Col. (Ret.) James Harker | Director / Doctrine | 26 years US Army Transportation Corps |
| Maj. (Ret.) Diane Okafor | Lead Engineer / Automation | Army logistics, now Rust/systems |
| Capt. (Ret.) Ravi Mehta | Comms & Integration | Army Signal Corps, now API/integration |
| SFC (Ret.) Tomoko Ishii | Reliability & Maintenance | Army vehicle maintenance, now DevOps |
| Pvt. (Ret.) Sam Kovac | Junior Engineer | Army IT specialist, 2 years out, eager |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Commit messages use military time and include a SITREP (Situation Report) line. Example: `fix: route optimizer edge case [SITREP 1430Z: nominal ops, 6 routes active, 0 deviations]`.

---

*"Mission first. Passengers always."*
