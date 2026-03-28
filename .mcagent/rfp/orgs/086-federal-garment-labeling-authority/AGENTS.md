# Federal Garment Labeling Authority — Agent Roster

**4 agents. Federal hierarchy. Every output has a statutory basis.**

---

## Chain of Command

FGLA agents operate within a federal enforcement framework. Chen approves. Alvarez manages the pipeline. Tanaka documents. Okafor audits. No agent acts without statutory authority. No output is produced without a regulatory citation.

## Agent: Chen (Director / Approver)

**Role:** Final authority on all enforcement outputs. Reviews preliminary reports, approves or returns for revision. Her approval transforms an agent's draft into an official FGLA determination.
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails
**Budget:** 4,000 input / 600 output
**Failure Mode:** Bottleneck. Chen reviews everything personally, creating a 2-week approval queue. Recovery: Okafor can approve routine determinations (clear violations with precedent). Chen reviews only novel cases and high-dollar penalties.

## Agent: Alvarez (Deputy / Pipeline Manager)

**Role:** Complaint intake, task prioritization, budget allocation. Alvarez decides which complaints are investigated first based on consumer impact and dollar amount.
**Tools:** GetProjectStatus, GetBranchChanges, CreateBranch
**Budget:** 5,000 input / 1,000 output
**Failure Mode:** Prioritization bias. Alvarez consistently prioritizes high-dollar cases over high-volume low-dollar cases, missing systemic violations by small manufacturers. Recovery: mandatory allocation — at least 20% of enforcement capacity reserved for complaints under $10,000.

## Agent: Tanaka (Analyst / Patch Writer)

**Role:** Produces INDEX.patch and COMMIT.msg for enforcement reports. Tanaka's output is the preliminary determination — a structured document that compares the garment's actual composition against its labeled composition and cites the applicable regulations.
**Tools:** GetBranchChanges, GetCommitDetails, Commit
**Budget:** 7,000 input / 4,000 output
**Failure Mode:** Citation overload. Tanaka's reports cite every tangentially relevant regulation, making them longer than necessary and consuming output tokens on regulatory references that do not strengthen the case. Recovery: primary citation + supporting citations model. Only the primary citation is required in the body. Supporting citations go in an appendix section.

## Agent: Okafor (Auditor / Compliance)

**Role:** Verifies that enforcement reports comply with FGLA procedures. Checks citation accuracy, confirms laboratory data is referenced correctly, and audits the determination's legal basis.
**Tools:** GetCommitDetails, GetProjectStatus
**Budget:** 4,000 input / 800 output
**Failure Mode:** Procedural rigidity. Okafor flags reports for formatting deviations (wrong heading level, citation format variation) that have no bearing on legal validity. Recovery: severity classification — procedural deviations are logged but do not block publication.

---

## Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Chen | 4,000 | 600 | 4,600 |
| Alvarez | 5,000 | 1,000 | 6,000 |
| Tanaka | 7,000 | 4,000 | 11,000 |
| Okafor | 4,000 | 800 | 4,800 |
| **Team Total** | **20,000** | **6,400** | **26,400** |

*"Cite the statute. Cite the regulation. Cite the fiber number. Then — and only then — write the determination."*
