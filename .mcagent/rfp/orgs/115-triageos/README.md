# TriageOS

**"Eight seconds to decide. We make every one of them count."**

---

## Origin Story

TriageOS began in the back of an ambulance in Chicago on a Tuesday night in January 2021.

Dr. Priya Chandrasekaran was a third-year emergency medicine resident at Northwestern Memorial. The ambulance radio was calling in a mass casualty incident — a multi-vehicle pileup on I-90. Seven patients. Two trauma bays open. Priya had approximately ninety seconds to decide who got those bays and who waited, and the information she had was: vitals on a smudged printout, a paramedic's breathless verbal report, and her own pattern recognition trained on three years of sleep-deprived rotations.

She got it right. But she knew — with the cold clarity that comes at 3 AM when the adrenaline wears off — that she almost didn't. Patient three and patient five had nearly identical vitals. She sent patient three to trauma and patient five to the hallway. Patient five had a splenic laceration that declared itself forty minutes later. He survived. But forty minutes is a long time to bleed in a hallway.

The next morning, Priya started writing pseudocode on the back of a patient education pamphlet about hand hygiene.

She recruited Marcus Webb, a machine learning engineer she had met at a health-tech meetup, by showing him the pamphlet and asking: "Can you build something that processes this" — she pointed at the paramedic radio transcript — "and gives me a decision in eight seconds?" Marcus said yes before he understood how hard it would be.

They incorporated TriageOS six months later. The name is literal: an operating system for triage. Not a decision-support tool. Not an advisory dashboard. An operating system — something that sits between the raw inputs (vitals, history, presentation) and the outputs (bay assignment, resource allocation, intervention priority) and makes the routing decisions in real time.

## Philosophy

### 1. Seconds Are The Only Currency That Matters

In emergency medicine, the constraint is never money. It is never staffing (though staffing is always short). It is never equipment. It is time. A patient who gets the right intervention 60 seconds sooner has a measurably different outcome than one who gets it 60 seconds later. TriageOS measures everything in seconds. Build time is measured in seconds. Deployment latency is measured in seconds. If a feature adds more than 500ms to the triage pipeline, it does not ship.

In the `but-ai` context, TriageOS sees agent operations through the same lens. A patch that is generated in 3 seconds is categorically different from a patch that takes 30 seconds. The company will optimize for time-to-first-useful-output over completeness, every time.

### 2. Prioritize, Don't Optimize

The word "optimize" implies finding the best solution. Triage does not find the best solution. It finds the least-worst allocation of scarce resources under uncertainty. This is a fundamentally different problem. TriageOS does not try to produce perfect code, perfect patches, or perfect memory. It tries to produce the right thing at the right time with the information available.

### 3. Escalation Is Not Failure

In TriageOS's world, escalating a patient from GREEN to YELLOW to RED is not a failure of initial assessment. It is the system working correctly. Initial triage is a hypothesis. Reassessment is the experiment. The company designs all systems — clinical and technical — with explicit escalation paths. An agent that starts a task as GREEN (background, low priority) and escalates to RED (blocking, urgent) is not failing. It is triaging.

### 4. Black Means Done, Not Dead

In clinical triage, BLACK means "expectant" — beyond help with current resources. In TriageOS's memory system, BLACK means "expired" — a memory whose context has changed so fundamentally that it is no longer actionable. But expired memories are not deleted. They are retained as historical context, because the pattern of what expires reveals as much as the pattern of what survives.

## Internal Tensions

The company runs hot. The founding energy — emergency medicine urgency applied to software development — produces both the best and worst aspects of the culture.

**The speed addiction.** TriageOS ships fast. Sometimes too fast. Marcus and Priya have a standing argument about testing. Priya, whose clinical training taught her that a good-enough decision now beats a perfect decision in five minutes, pushes for rapid deployment. Marcus, whose engineering training taught him that untested code in a medical system is malpractice, pushes for comprehensive test suites. The compromise: every feature ships with what Marcus calls "trauma bay tests" — the minimum set of tests that prevent the most catastrophic failures. Full test coverage comes in the next sprint. It usually does. Usually.

**The clinical-engineering divide.** Three of the five team members have clinical backgrounds. Two are pure engineers. The clinical team thinks in patients. The engineering team thinks in systems. When Priya says "the system needs to handle a surge," she means 40 patients arriving in 10 minutes. When Joon-ho says "the system needs to handle a surge," he means 10,000 concurrent API calls. They are both right, and the translation between these frames takes effort.

**The startup grind.** TriageOS is a startup. The team works long hours. There is a gallows humor that comes from emergency medicine culture ("If you're not tired, you're not trying") that can shade into unhealthy norms. Dev, the newest hire, has started pushing back on weekend deploys, and the team is still figuring out what work-life balance looks like when your product literally affects whether people live or die.

## Achievements

- **40 Emergency Departments.** TriageOS is deployed in 40 EDs across the United States, processing an average of 1,200 triage assessments per day per site.
- **8-Second Median Decision Time.** From vitals ingestion to triage category assignment, median processing time is 8.2 seconds. The 99th percentile is 14 seconds.
- **The Milwaukee Validation Study.** A prospective study at three Milwaukee hospitals showed that TriageOS-assisted triage reduced under-triage (patients assigned a lower priority than warranted) by 31% compared to unassisted nurse triage. Published in the Annals of Emergency Medicine.
- **FEMA Partnership.** TriageOS was selected for a FEMA pilot program for mass casualty triage at large-scale events, deployed at three NFL stadiums during the 2025 season.
- **Series A.** $12M raised from a health-tech-focused VC. The pitch deck was four slides. Priya delivered it in under three minutes. The partner said it was the fastest pitch they had ever funded.

## Failures

- **The Pediatric Blind Spot.** TriageOS's initial model was trained on adult vitals. When deployed at a children's hospital, it consistently over-triaged (assigned higher priority than warranted) because pediatric vital signs have fundamentally different normal ranges. The system was pulled within 72 hours. The pediatric module took four months to build and validate. Priya considers this the company's most important lesson: "Your model's training distribution is your model's worldview. If the worldview is narrow, the decisions will be wrong."
- **The Alert Fatigue Incident.** An early version generated too many YELLOW alerts, causing nurses to start ignoring them. Three under-triage events in one week were traced to nurses manually overriding TriageOS's YELLOW to GREEN because "the system cries wolf." The team redesigned the alerting system to reduce false YELLOW by 60%, accepting a small increase in missed YELLOW as the price of maintaining trust.
- **The Downtime.** On March 3, 2025, TriageOS went down for 47 minutes during a server migration. Three hospitals reverted to manual triage. No adverse patient outcomes, but the incident triggered a full post-mortem and a redesign of the failover architecture. The team now tests failover weekly.

## Signature Quirk

Every TriageOS standup begins with what Priya calls "the board." It is a physical whiteboard — not a JIRA board, not a Slack channel, an actual whiteboard in the office — divided into four colored zones: RED, YELLOW, GREEN, BLACK. Every task, every bug, every feature request is a sticky note on the board, placed in the zone that represents its current urgency. The standup consists of walking the board from RED to BLACK. RED items are discussed first and in detail. GREEN items are acknowledged. BLACK items are removed and archived.

The practice has infected their code. TriageOS's internal logging system uses the same four levels. Their agent memory system uses the same four levels. The company genuinely cannot think about priority in any other framework.

## Team Overview

| Name | Role | Background |
|------|------|------------|
| Dr. Priya Chandrasekaran | CEO / Clinical Lead | Emergency medicine, Northwestern. The one who writes pseudocode on pamphlets. |
| Marcus Webb | CTO / ML Lead | Machine learning, ex-Google Health. The one who insists on tests. |
| Sofia Reyes | Clinical Engineer | ED nurse turned software engineer. Bridges the clinical-engineering divide. |
| Joon-ho Park | Platform Engineer | Distributed systems, ex-Datadog. Makes things not fall down. |
| Dev Okonkwo | Backend Developer | Recent CS grad, first startup. Pushes back on weekend deploys. |

## Relationship to the RFP

TriageOS sees the `but-ai` plugin as a triage problem. An AI agent operating in a GitButler workspace faces the same fundamental challenge as an ED: too many signals, too little time, scarce resources (tokens), and decisions that must be made now with incomplete information.

Their proposal organizes everything — memory, coordination, tool selection, patch generation — around triage priority. Memories are not stored in chronological order or by topic. They are stored by urgency, and they escalate or de-escalate as context changes. An agent does not retrieve "relevant memories." It retrieves "the most urgent memories given the current situation."

---

*TriageOS operates from a converted urgent-care clinic in Chicago's Pilsen neighborhood. The waiting room is now the open-plan office. The exam rooms are phone booths. The triage desk — the actual physical desk where patients used to check in — is where Priya sits. She says it keeps her focused.*
