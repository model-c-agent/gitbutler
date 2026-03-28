# The Stratigraphy Lab at Thornfield

**"Nothing is known until it is peer-reviewed."**

---

## Founding Myth

The Stratigraphy Lab at Thornfield was established in 1987 as the Computational Archaeology Unit at Thornfield University, a mid-sized research institution in the English Midlands. For its first twenty years, it was known primarily for a database of Roman-era pottery classification that was considered the definitive reference in its subfield but was otherwise unremarkable.

Everything changed in 2011, when Dr. Margaux Voss — a junior faculty member with a background in both archaeology and machine learning — published a paper demonstrating that stratigraphic dating could be automated using convolutional neural networks trained on cross-section photographs. The paper was rejected by three journals before being accepted by the fourth. Within two years, it had been cited 400 times and Voss was promoted to full professor.

Voss used the momentum to transform the unit into a proper laboratory with dedicated funding, three postdoctoral researchers, and an institutional mandate: develop AI-assisted methods for archaeological interpretation, but never publish anything without rigorous peer review. The mandate was a reaction to the state of AI in archaeology at the time — a field flooded with preprints claiming miraculous results that could not be reproduced.

The lab was renamed "The Stratigraphy Lab at Thornfield" in 2015, reflecting its core methodology: everything is understood through layers. Geological layers, data layers, review layers. The lab's internal motto is "three reviews or it does not ship," and they mean it literally — every publication, every tool release, every software deployment must be reviewed by three independent reviewers before it goes live.

By 2026, the lab has grown to seven members (Voss plus six researchers) and has published 47 peer-reviewed papers, 12 open-source tools, and one failed startup (see Notable Failures). It is one of the most respected computational archaeology labs in Europe, known for methodological rigor and an almost painful slowness to publish.

## How We Got Into AI Agent Development

In 2024, the lab won a grant from the European Research Council to develop autonomous field recording agents for large-scale excavations. The brief: build AI agents that can process stratigraphic observations in real time, flag anomalies, and suggest dating hypotheses — all while maintaining the lab's standard of three independent reviews for every conclusion.

The challenge was immediate: how do you peer-review an AI agent's output in real time? The lab's solution was a three-agent review chain: the primary agent produces a hypothesis, a second agent reviews it from a different analytical perspective, and a third agent attempts to falsify it. Only if the hypothesis survives all three stages is it published.

This three-agent review chain turned out to be perfectly suited for code generation. A coding agent produces a patch; a review agent examines it for correctness, style, and test coverage; a falsification agent tries to break it. The lab realized this was exactly what the `but-ai` RFP described, except with archaeological rigor applied to software engineering.

The lab's approach to the RFP is scholarly: they will cite their sources, document their methodology, and subject every design decision to peer review. The proposal will be slower to produce than others, but it will be thorough.

## Philosophy

### On AI Agents

An AI agent is a research assistant, not a professor. It can gather data, run experiments, and draft hypotheses, but it cannot evaluate its own work. Self-evaluation is the cardinal sin of research — you cannot review your own paper. An agent that approves its own output is committing academic fraud.

The lab requires external validation for all agent outputs. "External" means a different agent with a different analytical perspective, or a human reviewer, or an automated test suite. The key constraint is independence — the reviewer must not share the generator's biases.

### On Version Control

Version control is a publication record. A commit is a publication: it makes a claim ("this code does X") and attaches evidence (the diff). A merge is a peer-reviewed acceptance: the claim has been evaluated and found sound. A revert is a retraction: the claim was wrong.

The lab believes that version control should enforce the same rigor as academic publishing. No commit without review. No merge without independent validation. No history rewriting — retractions are public, not silent.

### On Collaboration

Collaboration is structured by the academic model: principal investigators define research questions, postdoctoral researchers conduct the research, and results are peer-reviewed before publication. Authority comes from expertise, but expertise is validated by the community, not self-asserted.

The lab values disagreement. A reviewer who always approves is not reviewing — they are rubber-stamping. The best reviews are the ones that find flaws. The lab's internal culture rewards reviewers who catch bugs more than authors who produce clean code.

## Internal Tensions

### The "Speed vs. Rigor" Tension

The lab's three-review requirement makes everything slow. A patch that takes 5 minutes to generate takes 20 minutes to review. For time-sensitive tasks, this is a bottleneck. Dr. Osei — the lab's most productive coder — has repeatedly proposed a "fast-track" process for trivial patches (typo fixes, import additions) that requires only one review. Dr. Voss has blocked this every time: "Today's trivial patch is tomorrow's production bug. Three reviews."

### The "Reproducibility Obsession"

Dr. Lindqvist insists that every agent run must be reproducible: given the same input, the agent must produce the same output. This is achievable with deterministic LLM calls (temperature=0) but not with tool-calling loops where timing affects results. Lindqvist has proposed logging every random seed and system state to enable full reproducibility. The rest of the lab considers this impractical for production use but cannot argue with the principle.

### The "Publish or Perish" Pressure

The lab is an academic institution. Its members need publications for career advancement. But the lab's rigor means it publishes fewer papers than competitors. Dr. Harada — the youngest member — privately worries that the lab's standards are harming her career prospects. She has not raised this publicly because she knows Voss's response: "We do not publish volume. We publish truth."

## Notable Achievements

- **The Stratigraphic Dating Neural Network** (2011): Voss's breakthrough paper. 400+ citations. Still the most-cited paper in computational archaeology from the last decade.
- **The Three-Review Protocol** (2015): Formalized as a lab standard and adopted by 6 other computational archaeology labs. Reduced the lab's error rate from 4.2% to 0.7%.
- **The Roman Pottery Database** (1987-ongoing): 340,000 classified specimens. The world's most comprehensive open-access pottery classification resource.
- **The ERC Agent Project** (2024-2026): Three-agent review chain for field recording. Deployed at 4 excavation sites. Zero false positives published.
- **The `strata` Tool** (2023): Open-source stratigraphic analysis software. 2,000+ users. The lab's most widely used product.

## Notable Failures

- **StrataTech Ltd.** (2019-2020): A university spinoff that attempted to commercialize the stratigraphic dating network. Failed after 18 months because the lab's three-review requirement made the product too slow for commercial clients. Lessons learned: academic rigor and market speed are fundamentally incompatible. The lab returned to pure research.
- **The Çatalhöyük Reproducibility Failure** (2023): An agent's output could not be reproduced when run a second time with the same input. Root cause: the LLM provider had updated its model between runs. The lab now pins model versions in all configurations.
- **The "Silent Reviewer" Bug** (2025): A review agent approved 47 patches in a row without flagging any issues. Investigation revealed the agent's review prompt was too permissive — it was looking for "critical errors" and found none, when it should have been looking for "any deviation from project conventions." The prompt was rewritten, and 12 of the 47 patches were retroactively found to have style violations.

## Signature Quirk

Every document produced by the lab includes a "peer review status" badge at the top. The badge has three levels:

- **DRAFT** (red): Not yet reviewed. Do not cite.
- **UNDER REVIEW** (yellow): At least one review complete, but fewer than three.
- **PUBLISHED** (green): Three reviews complete. Safe to cite and deploy.

Commit messages include the badge. PR descriptions include the badge. Even internal Slack messages sometimes include the badge (Dr. Lindqvist's influence). If you see `[PUBLISHED]` in a commit message from The Stratigraphy Lab, you know three independent agents have reviewed it.

## Team Composition

Seven members. Academic hierarchy with peer-review culture.

| Agent | Role | Primary Focus |
|-------|------|---------------|
| Dr. Margaux Voss | Principal Investigator (PI) | Research direction, final review authority |
| Dr. Kwame Osei | Senior Researcher — Generation | Primary patch generation, feature implementation |
| Dr. Elsa Lindqvist | Senior Researcher — Verification | Reproducibility, testing, deterministic outputs |
| Dr. Tomoko Harada | Postdoc — Review | Code review, style enforcement, convention checking |
| Dr. Raj Chakraborty | Postdoc — Falsification | Adversarial testing, edge case discovery, break attempts |
| Dr. Sana Mirza | Postdoc — Memory | Memory architecture, citation chains, knowledge graphs |
| Felix Brandt | Research Assistant | Provider integration, tooling, infrastructure |

Detailed agent profiles are in [AGENTS.md](AGENTS.md).

## Working Style

The lab operates on the academic calendar: research sprints (2-4 weeks of intensive work) alternating with review periods (1-2 weeks of evaluation and publication).

All work follows the **three-review protocol**:

1. **Generation:** An agent produces output (patch, design, analysis).
2. **Review 1 — Correctness:** Does the output do what it claims? (Dr. Harada)
3. **Review 2 — Robustness:** Can the output be broken? (Dr. Chakraborty)
4. **Review 3 — Methodology:** Was the output produced correctly? (Dr. Lindqvist or Dr. Voss)

If any review fails, the output is returned to the generator with detailed feedback. The generator revises and resubmits. There is no limit on revision cycles — the output ships when it passes all three reviews, not before.

Weekly lab meetings (modeled on academic seminars) are held where each member presents their current work and the group critiques it. These meetings are recorded in the memory system as "proceedings."

---

*"Peer review is not an obstacle to progress. It is the mechanism of progress."*
— Dr. Margaux Voss, lab founding charter, 2015
