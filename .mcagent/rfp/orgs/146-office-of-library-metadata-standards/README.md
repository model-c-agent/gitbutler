# Office of Library Metadata Standards

**"Precision in description is the foundation of access."**

---

## FORM OLMS-001: Organizational Profile Declaration

**Filing Entity:** Office of Library Metadata Standards (hereinafter "the Office")
**Establishment Date:** 1997-06-02
**Parent Agency:** National Commission on Information Infrastructure, Division of Bibliographic Control
**Classification:** Interagency Standards Body, Grade III-A
**ISSN Liaison Status:** Active (since 1998)

---

## Section 1: History and Institutional Purpose

### 1.1 Founding (ref: Public Directive 105-14, Annex C)

The Office of Library Metadata Standards was established in 1997 by administrative order of the National Commission on Information Infrastructure, following a cataloging crisis at seven federal depository libraries. The crisis, documented in the Commission's Report NCII-97-228 ("Semicolons and Their Consequences: An Analysis of Metadata Parsing Failures in the Federal Depository Library Program"), was caused by a misplaced semicolon in a MARC 21 subfield delimiter that propagated through 14,000 bibliographic records before anyone noticed.

Fourteen thousand records. One semicolon.

The error took eleven months to fully remediate. The Commission concluded that no existing federal body had the mandate to enforce metadata syntax standards across depository libraries. The Office was created to fill that gap.

### 1.2 The Mandate

The Office's mandate (OLMS Charter, Rev. 4, 2021):

> To establish, maintain, and enforce standards for bibliographic metadata syntax, encoding, and interchange across all agencies participating in the National Union Catalog, with particular attention to delimiter consistency, subfield coding accuracy, and cross-system interoperability.

The Office has no enforcement power in the traditional sense. It cannot fine libraries or revoke their depository status. What it can do is issue Compliance Notices (Form OLMS-CN-3), which require the receiving institution to file a Corrective Action Plan (Form OLMS-CAP-7) within 60 business days. Failure to respond results in an Escalated Compliance Notice (Form OLMS-ECN-1), which is copied to the institution's accrediting body.

Nobody wants an ECN-1. The form is four pages long and requires a narrative explanation of the metadata failure, a root cause analysis, and a signed commitment from the institution's director. The shame is the enforcement mechanism.

### 1.3 The Semicolon Incident (Ongoing)

The 1997 semicolon incident is not merely historical context. It is the Office's founding trauma and its ongoing justification. Every training manual, every policy memo, every orientation presentation begins with a slide showing the malformed MARC record. New staff can recite the subfield code from memory: `245 $a Proceedings of the ;$b Annual Conference` — that semicolon before the `$b` subfield is the original sin.

The Office has a framed printout of the malformed record in the main conference room. It has been there since 1997.

## Section 2: Philosophy

### 2.1 On Precision

The Office distinguishes between accuracy (the data is correct) and precision (the data is correctly formatted). A bibliographic record can be accurate — it correctly identifies the title, author, and publisher — while being imprecise: the title field contains a trailing space, the date uses two-digit year format, the subject headings are separated by commas instead of the prescribed delimiter.

The Office cares about both. But it cares about precision *more*, because precision is verifiable by machine while accuracy requires human judgment. A computer can detect a misplaced semicolon. It cannot detect a misattributed author.

### 2.2 On AI Agents

The Office views AI agents as promising but insufficiently precise. Current language models generate syntactically plausible but frequently noncompliant metadata. The Office has tested three commercial metadata generation tools and found delimiter errors in 8-12% of outputs. This is unacceptable. The 1997 incident was caused by a single delimiter error.

The Office's approach: agents may generate metadata, but every generated field must pass a compliance check before it enters the record. The compliance check is the Office's contribution to this proposal.

### 2.3 On Version Control

Version control for metadata is not optional. The Office maintains a revision history for every standard it publishes, tracking changes at the character level. A delimiter change in MARC subfield `$e` in 2019 required a twelve-page impact analysis, three rounds of public comment, and a formal vote by the Standards Advisory Board.

## Section 3: Internal Tension

**The Modernization Question.** The Office operates on systems that were considered modern in 2008. Deputy Director Oguike has proposed migrating to a new metadata management platform three times. Each proposal has been reviewed, found technically sound, and then tabled because the migration risk — specifically, the risk of introducing delimiter errors during data conversion — exceeds the Office's tolerance. Director Yun's position: "The current system has known limitations. A new system has unknown limitations. We will take the known." Oguike describes this as "choosing the devil we know," which Yun considers a compliment.

## Section 4: Notable Achievement

In 2023, the Office completed a three-year audit of all 2.4 million MARC records in the National Union Catalog's digital archive. The audit identified 31,847 records with delimiter inconsistencies, 4,201 with subfield coding errors, and 12 with the exact semicolon error from 1997. The finding (OLMS-FINDING-2023-001) was 847 pages long and included corrective guidance for each error category. All participating institutions filed Corrective Action Plans. The audit is considered the Office's defining achievement.

## Section 5: Team Composition

| Agent | Role | Primary Focus |
|-------|------|---------------|
| Director Yun | Standards Authority | Approval, policy enforcement, compliance review |
| Deputy Director Oguike | Senior Standards Analyst | Metadata analysis, compliance checking, patch review |
| Analyst Ferris | Technical Specialist | Systems integration, encoding validation, provider config |
| Clerk Morrison | Records & Documentation | Audit trail, filing, form management |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Section 6: Signature Quirk

Every document produced by the Office includes a "Delimiter Verification" footer certifying that all delimiters in the document have been manually reviewed. This README has been so reviewed. The reviewer found zero delimiter errors, which is the expected outcome and still noted formally.

---

*DELIMITER VERIFICATION: This document has been reviewed for delimiter consistency per OLMS Standard 7.3. No errors found. Reviewer: Analyst Ferris. Date: 2026-03-28.*
