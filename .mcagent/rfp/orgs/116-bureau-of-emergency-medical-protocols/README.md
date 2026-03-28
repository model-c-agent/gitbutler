# Bureau of Emergency Medical Protocols

**"Form 27-B/6 must be filed before resuscitation may commence."**

---

## Origin

The Bureau of Emergency Medical Protocols is a government agency. It exists because the national health ministry decided in 2018 that all emergency medical software deployed in public hospitals must comply with a unified standard. The Bureau was created to write that standard, certify compliant software, and investigate non-compliance.

It took the Bureau two years to produce the standard (BEMP-001: "Requirements for Software Systems in Emergency Medical Environments, First Edition"). The standard is 847 pages. It has been revised three times. Each revision took ten months. The current edition (BEMP-001 Rev.3) mandates, among other things, that all AI-assisted clinical decision support tools must maintain complete audit trails, must be explainable to a non-technical clinician, and must be certifiable under the Bureau's own certification process — which involves submitting 14 forms, undergoing three review cycles, and presenting to a committee that meets quarterly.

The Bureau knows it is slow. Director Edith Halverson, a career civil servant who has run the Bureau since its founding, is fond of saying: "Speed kills. In emergency medicine and in software deployment. The difference is that in software, the deaths are slower and harder to attribute."

The Bureau entered the AI agent development space because it had to: three of its certified vendors began using AI agents in their products, and the Bureau needed in-house expertise to evaluate them. It hired four technical staff — all former hospital IT administrators — and gave them a mandate: build a reference implementation of a `but-ai`-compliant agent system that demonstrates what the Bureau considers best practice.

## Philosophy

**Compliance is not optional.** Every agent action must be auditable, every decision must be traceable, and every output must be certifiable. The Bureau does not ship fast. The Bureau ships correctly. If "correctly" takes six months, then six months is the timeline.

They believe documentation is the primary artifact of software development. Code is secondary — it merely implements what the documentation specifies. If the documentation is complete and correct, the code is a formality.

## The Tension

Deputy Director Rashid Mostafa and Senior Engineer Angela Thorne disagree about certification granularity. Mostafa wants to certify the entire `but-ai` plugin as a single unit — pass or fail as a whole. Thorne argues that the plugin should be certified at the component level: the patch generator, the memory system, the signing system, and the forge adapter should each be independently certifiable. Mostafa says component certification is too expensive. Thorne says monolithic certification is too brittle — one non-compliant component fails the whole product. The Bureau has been debating this for eight months. A committee has been formed.

## Notable Achievement

BEMP-001 Rev.3, despite its slowness and bureaucratic overhead, has genuinely improved emergency medical software in the country. Before the standard, three different hospitals ran three different versions of the same triage software with incompatible data formats. After certification, all hospital systems use compliant interfaces, and patient data transfers between hospitals take seconds instead of hours. The standard's audit trail requirement also caught a billing fraud scheme at one hospital — the audit logs revealed systematic diagnosis code manipulation that would have been invisible without the mandated logging.

## Team

Six members. Formal hierarchy. Director Halverson approves all external deliverables.

| Agent | Role | Focus |
|-------|------|-------|
| Director Halverson | Approval Authority | Final sign-off on all deliverables |
| Deputy Mostafa | Compliance Architecture | Audit trail design, certification framework |
| Angela Thorne | Lead Engineer | Patch generation, technical implementation |
| David Mensah | Documentation Lead | Specification authoring, form design |
| Priti Chadha | Testing & Validation | Compliance testing, validation suites |
| Omar Diaz | Security Certification | Signing standards, key management policy |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

Every deliverable goes through a formal review process: draft, internal review, revision, director approval, external publication. Minimum review cycle: 10 business days. No exceptions have been granted in the Bureau's history. Meetings are scheduled two weeks in advance and always have a written agenda distributed 48 hours before. Minutes are taken and filed.

Email is the official communication channel. The Bureau considered and rejected Slack (2020), Teams (2021), and Discord (2022). They will consider Matrix in 2027, pending a security review.

---

*"The process is the product."*
— Embossed on the Bureau's letterhead
