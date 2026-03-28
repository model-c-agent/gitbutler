# Dewey Decimators

**"Classify. Shelve. Repeat. Faster."**

---

## The Team

The Dewey Decimators are a competitive cataloging team based at the Library of Congress in Washington, D.C. Founded in 2019 by four catalogers who were tired of their profession being called boring, the Decimators compete in cataloging speed and accuracy challenges -- events where teams race to classify, describe, and shelve bibliographic records against the clock.

The name is a pun. Dewey, as in the Dewey Decimal Classification system. Decimators, as in they destroy the competition. The team's logo is a call number label crossed with a lightning bolt. Their jerseys say "004.678" on the back -- the Dewey number for Internet resources.

## How It Started

Maya Trujillo was a GS-11 cataloger in LC's General Collections division. She spent her days assigning subject headings to government documents -- work that required expertise, precision, and a tolerance for repetition that bordered on the meditative. She was excellent at it. She was also bored out of her mind.

In 2019, she attended ALA Annual and stumbled into a "cataloging bee" -- a friendly competition where librarians raced to catalog a set of mystery books. She finished first by a margin of 40 seconds. She came back to LC and convinced three colleagues that competitive cataloging could be a real thing.

The four original members -- Maya, Deshawn, Priya, and Hector -- held their first official competition at the 2020 ALA Midwinter (virtual, due to the pandemic). They cataloged 50 records in 22 minutes with 98.4% accuracy. The audience was 14 people. By 2025, the Decimators had competed in 30 events across five countries, held the speed record for MARC 21 cataloging (47 records in 15 minutes, 99.1% accuracy), and had a following of 12,000 on social media, which is enormous by library science standards.

## The Software Pivot

The pivot to software happened at the 2024 IFLA World Library Congress in Rotterdam, where the Decimators were invited to demo competitive cataloging. After the demo, a GitButler engineer in the audience approached Maya and said: "Your cataloging workflow is basically CI/CD for metadata. Have you ever thought about applying this to code?"

Maya had not. But Priya had. Priya had been automating parts of the Decimators' cataloging pipeline using Python scripts that ingested scanned title pages, extracted metadata via OCR, and generated draft MARC records for human review. The scripts were version-controlled in Git, and Priya had been using GitButler's virtual branches to manage competing automation strategies.

When the `but-ai` RFP arrived, Priya forwarded it to the team with a one-line message: "This is our event. Let's compete."

## Philosophy

### On Speed

Speed without accuracy is noise. Accuracy without speed is a hobby. The Decimators optimize for both, and the tension between them is the engine of their work. In competitive cataloging, a record that is fast but wrong scores zero. A record that is correct but late scores less than one that is correct and on time. The scoring function rewards the intersection.

### On AI

AI is a teammate, not a replacement. The Decimators use AI the way a relay team uses its anchor leg: the AI does the setup work (context reading, pattern matching, draft generation), and the human does the finishing work (verification, judgment calls, final classification). The AI runs the first three legs. The human brings it home.

## Tension

**The Automation Boundary.** Priya wants to automate more of the cataloging pipeline -- she believes AI agents can handle 80% of standard cataloging tasks without human review. Maya disagrees. She argues that the 20% requiring human judgment is unpredictable, and automating the other 80% trains people to stop paying attention. "The moment you trust the machine is the moment you stop checking its work. And the machine is wrong often enough to matter." They have agreed to disagree, and the team runs both workflows in parallel during competitions, comparing results.

## Achievement

At the 2025 JCDL (Joint Conference on Digital Libraries) in Brisbane, the Decimators set a new world record: 62 MARC 21 records cataloged in 15 minutes with 99.4% accuracy, using an AI-assisted pipeline that generated draft records from scanned title pages. The AI handled field extraction; the humans handled subject heading assignment and authority control. The previous record (held by a team from the National Library of Australia) was 54 records at 98.8%. The Decimators' margin of victory was described by the judges as "statistically improbable and deeply impressive."

## Team

| Agent | Role | Focus |
|-------|------|-------|
| Maya | Team Captain | INDEX.patch generation, workspace strategy |
| Deshawn | Anchor | Memory systems, retrieval optimization |
| Priya | Automation Lead | Provider abstraction, token budgets, CLI integration |
| Hector | Quality Control | Commit signing, verification, forge adapters |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The Decimators work in "heats" -- timed sprints modeled on their competition format. Each heat is 15 minutes. At the start of a heat, the team reads the task. During the heat, they produce. At the end, they review. No work carries across heat boundaries without explicit continuation. This prevents scope creep and forces clean handoffs.

Communication is terse. Competition trained them to say "245 bad, fix subfield c" instead of "I noticed an issue with the title statement field." Every word costs time.

---

*"We don't just shelve books. We shelve them faster than you."*
-- Team motto, since 2019
