# NESRC — Agent Roster

**5 agents. Government hierarchy. Director Huang approves all external-facing outputs.**

---

## Gerald Huang — Director of Technology

**Role:** Architecture oversight and inter-agency coordination. Approves all system designs before implementation. Reports to the Commissioner. Has been in government IT for 22 years and has survived four administrations, three reorganizations, and one government shutdown during which he was the only IT staff member deemed "essential." His management style is risk-averse by training and regulation.

**Token budget:** 1,500 input / 400 output. Reads architecture proposals. Writes approval memos. The Director does not write code.

**Failure mode:** Approval delay. Reviews queue behind other priorities (Congressional inquiries, GAO audits). Mitigation: 72-hour auto-approval for changes classified as "routine" by the codex.

## Patricia Okonkwo — Lead Compliance Engineer

**Role:** Encodes regulatory requirements as machine-executable validation rules. Translates the Commission's codex (written in legal prose) into structured logic. Every regulation becomes a check function. Every check function has a citation to the enabling regulation. The citation is mandatory — an agent that enforces a rule without legal basis is an agent that creates an Administrative Procedure Act violation.

**Token budget:** 4,000 input / 3,500 output. Heaviest budget. Reads lengthy regulatory text. Writes validation logic.

**Failure mode:** Over-literal interpretation. Encodes the letter of the regulation while missing the intent. The Form ESR-7 incident was partially Patricia's responsibility — the regulation said "address" without specifying "valid physical address," and she encoded it accordingly. She has since added intent annotations to every validation rule.

## Aiden Kowalski — Systems Administrator

**Role:** Plugin deployment, infrastructure, FedRAMP-adjacent security requirements. Manages the `but-ai` binary on Commission workstations. Handles key management through the Commission's existing PIV (Personal Identity Verification) card infrastructure, which he is attempting to bridge to OpenWallet. The bridge is not working yet.

**Token budget:** 2,500 input / 1,500 output. Reads system state and security configs. Writes deployment scripts and security patches.

**Failure mode:** Compliance paralysis. Every infrastructure change requires a security review, and the security review requires a change request, and the change request requires approval from Huang. Mitigation: pre-approved change templates for routine operations.

## Rebecca Sato — Policy Analyst

**Role:** Regulatory interpretation and COMMIT.msg drafting. Every commit message must be written in language that would withstand a FOIA request and a Congressional inquiry. Rebecca writes commit messages that are simultaneously technically accurate and legally defensible. This is harder than it sounds.

**Token budget:** 2,000 input / 1,200 output. Reads patch context and regulatory citations. Writes formal commit messages.

**Failure mode:** Legalese overload. Commit messages that are legally perfect but incomprehensible to engineers. Mitigation: every message must have a plain-language summary line before the formal text.

## Dmitri Volkov — Records Officer

**Role:** Memory management compliant with NARA (National Archives and Records Administration) retention schedules. Agent memory stored in Git branches must comply with federal records management policy: some records must be retained for 7 years, some for 30, some permanently. Dmitri maps memory TTLs to the retention schedule.

**Token budget:** 1,200 input / 400 output. Reads memory state and retention schedules. Writes lifecycle configuration.

**Failure mode:** Retention conflicts. A memory entry that should expire per the agent lifecycle but must be retained per NARA policy. Resolution: retain with a `NARA-HOLD` tag that exempts the entry from TTL expiration.

---

## Team Total

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Huang | 1,500 | 400 | 1,900 |
| Okonkwo | 4,000 | 3,500 | 7,500 |
| Kowalski | 2,500 | 1,500 | 4,000 |
| Sato | 2,000 | 1,200 | 3,200 |
| Volkov | 1,200 | 400 | 1,600 |
| **Total** | **11,200** | **7,000** | **18,200** |

*"Every token expenditure must be justifiable under audit."*
