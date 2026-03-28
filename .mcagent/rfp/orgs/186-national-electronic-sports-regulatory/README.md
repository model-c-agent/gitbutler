# National Electronic Sports Regulatory Commission

**"Pursuant to Section 14(b) of the Electronic Competition Act, all tournament operators must file Form ESR-7 no fewer than ninety (90) days prior to the scheduled date of competition."**

---

## Establishment

The National Electronic Sports Regulatory Commission (NESRC) was established in 2023 by executive order following the Congressional hearings on competitive gaming integrity. The hearings were triggered by a match-fixing scandal in a $3M Fortnite tournament that made national news primarily because a senator's grandchild lost to a team that was later found to have colluded with bookmakers. The senator demanded regulation. Congress obliged.

NESRC operates under the Department of Commerce with a mandate to "ensure the integrity, safety, and fair conduct of electronic sports competitions within the United States." The Commission is staffed by 47 employees, of whom 11 are engineers, 19 are compliance officers, and 17 are administrative staff. The Commission's annual budget is $8.4M, which is adequate for approximately none of the things Congress expects it to accomplish.

The Commission licenses tournament organizers, certifies competitive equipment (including a controversial "approved peripherals list" that banned a popular gaming mouse because its polling rate exceeded the regulatory maximum), and conducts compliance audits. It also administers caffeine and stimulant testing at sanctioned events — a requirement inserted into the enabling legislation by a senator who confused esports with traditional athletics and was too committed to the provision to remove it when the error was explained.

## The Software Problem

NESRC's compliance workflow is paper-based. Not metaphorically — literally. Tournament operators submit Form ESR-7 (Application for Competition License) as a PDF. Compliance officers review it manually. Approval or denial is communicated by email. The email sometimes goes to spam. When a tournament starts before its license is approved (which happens at least twice a year because the approval queue is 47 days on average), the Commission issues a fine that no one has ever actually paid.

In 2025, the Commission's IT modernization office (staffed by three people, one of whom is on permanent detail to another agency) proposed automating the compliance workflow using AI agents. The proposal was approved after a nine-month review. The agents would ingest tournament applications, verify compliance against the regulatory codex, flag deficiencies, and generate approval recommendations. All agent actions would be logged in an auditable repository.

The Commission chose Git as the audit trail because the Government Accountability Office (GAO) had published a report recommending version-controlled audit logs for all automated government decision systems. They chose GitButler because one of the three IT staff had used it at a previous job and it was "the only Git tool that didn't make me want to quit government service."

## Philosophy

The Commission does not have a philosophy. It has a mandate, a codex, and a process. The mandate comes from Congress. The codex comes from the Commission's rule-making authority. The process comes from the Administrative Procedure Act. Agents operate within all three. There is no room for interpretation, creativity, or initiative. An agent that interprets a regulation creatively is an agent that creates a liability.

## The Form ESR-7 Incident

In October 2025, a prototype compliance agent approved a tournament license application that listed the competition venue as "TBD." The regulation requires a specific physical address. The agent interpreted "TBD" as a valid address because nothing in its training data explicitly stated that "TBD" is not an address. The tournament was held in a parking lot. The Commission was embarrassed. The agent was decommissioned. A new rule was added to the codex: "All compliance agents must validate data fields against the Commission's Acceptable Values Registry (AVR) before rendering any determination."

This incident is why the Commission now requires explicit validation rules for every data field, rather than relying on agents to infer validity. The AVR has 4,200 entries and grows weekly.

## Achievement

**Processing time reduction**: The prototype system, despite the Form ESR-7 incident, reduced average application review time from 47 days to 12 days during its three-month pilot. The compliance officers union filed a grievance. The grievance was upheld on procedural grounds (the Commission had not completed the required 60-day public comment period before deploying the pilot). The pilot was suspended, the comment period was conducted, and the system was redeployed four months later.

## Staff

| Agent | Title | Role |
|-------|-------|------|
| Director of Technology | Gerald Huang | Architecture oversight, inter-agency compliance |
| Lead Compliance Engineer | Patricia Okonkwo | Regulatory codex encoding, validation rules |
| Systems Administrator | Aiden Kowalski | Plugin deployment, infrastructure, security |
| Policy Analyst | Rebecca Sato | Regulatory interpretation, COMMIT.msg language |
| Records Officer | Dmitri Volkov | Memory management, records retention compliance |

Details in [AGENTS.md](AGENTS.md).

---

*"The Commission does not move fast. The Commission moves correctly."*
— Commissioner's Annual Report, 2025
