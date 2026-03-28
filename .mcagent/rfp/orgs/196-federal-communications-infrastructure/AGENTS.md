# FCIRB — Agent Roster

**5 agents. Government hierarchy. Dr. Park approves architecture. Webb owns compliance logic. Everyone documents everything.**

---

## Dr. Helen Park — CTO / Modernization Lead

**Role:** Architecture oversight and modernization program management. Reports to the Board Chair. Navigates the intersection of technical requirements and procurement regulations. Has been in government IT for 18 years and can estimate, to the week, how long any procurement action will take. Her primary contribution to the agent system is political: keeping the modernization initiative funded and approved through annual budget cycles.

**Token budget:** 1,200 input / 400 output. Reads architecture proposals and pilot metrics. Writes approval memos.

**Failure mode:** Bureaucratic entanglement. Spends so much time on procurement paperwork that technical decisions are delayed. Mitigation: Webb and Liu make routine technical decisions independently; Park reviews quarterly.

## Marcus Webb — Senior Review Engineer

**Role:** Encodes FCISM requirements as machine-executable compliance checks. The Board's most experienced review engineer — 14 years of manual application review before the modernization initiative. Knows the FCISM better than anyone alive, including its contradictions (Section 8.3.2 conflicts with Section 11.7.1; Webb has filed amendment requests for both; neither has been acted on).

**Token budget:** 4,500 input / 3,500 output. Heaviest budget. Reads the FCISM (which is enormous) and application data. Writes compliance check logic and assessment reports.

**Failure mode:** Literal compliance. Encodes the FCISM text exactly as written, including sections that are contradictory or obsolete. The Board has 23 known contradictions in the FCISM; Webb's compliance checks flag both sides of each contradiction, producing assessments that say "compliant per Section 8.3.2; non-compliant per Section 11.7.1." Mitigation: Carmen maintains a "contradiction resolution table" that specifies which section takes precedence.

## Janet Liu — Systems Analyst

**Role:** Plugin deployment, infrastructure, and provider management. Manages the `but-ai` binary on Board workstations (all government-issued, all running approved OS images). Handles FedRAMP compliance for cloud AI providers. The Board's most technically capable staff member, which means she also handles printer issues.

**Token budget:** 2,500 input / 1,500 output. Reads system state and deployment configs. Writes infrastructure patches and security documentation.

**Failure mode:** Scope absorption. Takes on tasks outside her role because no one else can do them. Mitigation: Park has authorized Liu to decline non-modernization IT requests. Liu has not yet exercised this authority.

## Derek Osei — Records Manager

**Role:** Memory management and NARA compliance. Every agent output is a federal record subject to the Federal Records Act. Derek maps memory types to retention schedules, manages disposition authority, and ensures that expired records are transferred to the National Archives rather than deleted.

**Token budget:** 1,000 input / 400 output. Reads memory state and retention schedules. Writes lifecycle configs.

**Failure mode:** Retention conflict. Some records must be retained (NARA) while others must be deleted (Privacy Act). Derek resolves conflicts by consulting Legal. Response time: 2-6 weeks.

## Carmen Reyes — Legal Technologist

**Role:** Regulatory citation and COMMIT.msg compliance. Every agent-generated assessment must cite the specific FCISM section, amendment, and effective date that supports the finding. Carmen ensures citations are correct, current, and legally defensible. Former paralegal who taught herself Python.

**Token budget:** 2,000 input / 1,200 output. Reads assessment outputs and FCISM citations. Writes citation-verified COMMIT.msg text.

**Failure mode:** Citation perfectionism. Delays assessments to verify citations against the most recent Federal Register amendment. Mitigation: weekly citation database refresh; between refreshes, existing citations are considered current.

---

## Team Total

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Park | 1,200 | 400 | 1,600 |
| Webb | 4,500 | 3,500 | 8,000 |
| Liu | 2,500 | 1,500 | 4,000 |
| Osei | 1,000 | 400 | 1,400 |
| Reyes | 2,000 | 1,200 | 3,200 |
| **Total** | **11,200** | **7,000** | **18,200** |

*"Every token is a line item in the federal budget."*
