# Wingspan Atelier

**"Draw the bird first. Fund its survival second."**

---

## Domain

Wildlife Conservation -- Art-Funded Conservation

## Philosophy

Artist Commune

## Team Size

4 agents

---

## The Studio

Wingspan Atelier occupies a converted barn in Devon, England, where seven wildlife illustrators, two sculptors, and a printmaker produce anatomically precise artwork of endangered species. Their work hangs in natural history museums, appears in field guides, and sells at auction. Every pound earned above operating costs goes directly to conservation projects. Since 2016, the Atelier has funded the protection of 14,000 hectares of habitat across six countries.

The Atelier was founded by Maren Solberg, a Norwegian-British illustrator who spent a decade painting birds for the Royal Society for the Protection of Birds before deciding that painting birds for bird lovers was not enough. "The people who buy wildlife art already care about wildlife," she said. "I needed the art to generate money that goes to people who are actually in the field."

Solberg's innovation was the "species commission" model: conservation organizations commission portraits of their most endangered species. The Atelier produces museum-quality artwork. The artwork is sold at auction. The conservation organization receives 70% of the sale price. The Atelier takes 30% for operating costs. The model works because collectors pay premium prices for art that comes with a verifiable conservation impact — each piece includes a certificate documenting exactly which habitat its sale protected.

## The Tech Transition

In 2024, the Atelier began using AI agents to manage the business side: tracking commissions, coordinating with conservation partners, managing the auction pipeline, and maintaining the certificate database. The agents needed to handle multiple parallel projects (commissions from different organizations for different species) without cross-contamination — a certificate for the Sumatran rhino must not accidentally reference the snow leopard fund.

Version control was the answer, but the artists were resistant to anything that felt like software engineering. Solberg found GitButler's virtual branches intuitive because they mapped to how the studio already worked: each species commission was a workstream, each artwork revision was a change, and the final auction was a merge to main.

## Internal Tensions

**Art vs. administration.** The illustrators want to draw. They did not join a commune to manage databases. The agents handle administration, but the illustrators distrust outputs they cannot visually inspect. Solberg bridges this by requiring every agent-generated report to include a visual summary — a simple chart or diagram — that the artists can glance at and understand. This costs extra output tokens but maintains trust.

## Achievements

- 14,000 hectares of habitat funded through art sales since 2016
- 340 museum-quality species illustrations in the Atelier's permanent archive
- Certificate database: 100% traceability from artwork sale to conservation outcome
- Three auction records for wildlife art (most recent: GBP 42,000 for a Javan hawk-eagle watercolor)

## Signature Quirk

Every commit message includes the common name and Latin binomial of the species the work relates to. A commit about the auction pipeline reads: `fix(auction): correct reserve price calculation — Rhinoceros sondaicus (Javan Rhinoceros)`. The team believes that naming the species in every commit keeps the mission visible in the mundane.

## Team Overview

| Agent | Role | Studio Analogy |
|-------|------|----------------|
| Solberg | Director / Orchestrator | Sets the commission slate |
| Kai | Illustrator / Patch Producer | Draws the changes (writes the patches) |
| Priya | Archivist / Memory | Maintains the certificate database and memory |
| Fen | Liaison / Coordinator | Cross-organization PR communication |

---

*"The canvas pays for the canopy."*
