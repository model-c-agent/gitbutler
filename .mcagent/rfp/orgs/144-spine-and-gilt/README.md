# Spine & Gilt

**"Every book is a commit. Every margin note is a branch."**

---

## Origins

Spine & Gilt began in a condemned printshop on Rue de la Huchette, Paris, in 2016. Four book artists — Margaux, Tomás, Fen, and Lerato — were sharing a single letterpress and a common frustration: they had all trained at fine arts academies that treated bookmaking as a dead craft, something to be studied but never practiced at scale.

They pooled their savings, bought 300 damaged books from a warehouse liquidation in Lyon, and spent six months restoring them. Not as museum pieces — as lending copies. Their thesis was simple: a restored first edition of *Les Misérables* that sits behind glass is a corpse. The same book, rebound with archival linen and placed on a lending shelf, is alive.

Within two years, the collection had grown to 5,000 volumes. By 2024, it was 20,000. They never bought a book at full price. Every volume came from estate sales, flood-damaged lots, deaccessioned libraries, or donations. The restoration work is done by the four founders and a rotating cohort of apprentices who stay for six months and leave with a trade.

The lending library operates on a trust system. No late fees. No deposit. You take a book, you bring it back. In eight years, they have lost 340 books to non-returns, and 87 of those were eventually mailed back with apologetic notes.

## Why Software

In 2023, Margaux was cataloging a newly acquired collection of 19th-century botanical illustrations and realized that her cataloging workflow — handwritten cards, typed into a spreadsheet, manually cross-referenced — was identical to the version control problem she kept hearing about from her partner, a systems engineer. The metadata for each book was a record. Corrections were patches. The card catalog was a commit log.

She brought the idea to the commune. Fen, who had taught herself Python to automate printing press calibration, prototyped a Git-based cataloging system in three weeks. They discovered GitButler when Tomás accidentally overwrote two months of catalog entries with a bad merge and needed to recover the work from virtual branches.

By late 2024, they were running AI agents to auto-generate catalog descriptions from scanned title pages. The agents were imprecise — they once cataloged a cookbook as a chemistry textbook because the table of contents mentioned "solutions" — but the workflow was promising. The `but-ai` RFP arrived at the right moment.

## Philosophy

### On Craft

We are book artists first. Software is a tool the way a bone folder is a tool — essential, invisible when used well, disastrous when used carelessly. We do not fetishize technology. We fetishize the thing technology serves: the book in the reader's hand.

### On AI

An AI agent should work like a good apprentice. It watches, it learns the patterns, it does the tedious work competently, and it asks before making irreversible decisions. A bad apprentice guesses. A catastrophically bad apprentice guesses confidently.

### On Memory

A library is memory architecture. Call numbers are pointers. Cross-references are edges in a graph. The card catalog is an index. We think about agent memory the way we think about finding aids — the memory is useless if you cannot locate the right entry at the right moment.

## Tension

**The Digitization Argument.** Lerato believes that every book in the collection should be digitized — high-resolution scans, OCR text, full metadata — so that the lending library becomes a digital archive accessible globally. Fen disagrees. She argues that digitization severs the physical relationship between reader and book, and that the commune's identity depends on hands touching pages. The compromise has been to digitize the metadata but not the content, which satisfies neither of them and has persisted for three years.

## Achievement

In 2025, the commune completed a two-year project to restore and catalog 1,200 water-damaged volumes from a closed branch library in Marseille. The collection included 14 books that had been incorrectly deaccessioned — they were unique copies, the last known exemplars of regional printing from the 1840s. Spine & Gilt's catalog metadata, cross-referenced against WorldCat, identified the rarity. The books are now on indefinite loan to the Bibliothèque nationale de France, attributed to the commune.

## Team

Five agents. The team mirrors the commune's workshop structure: each person has a primary craft, but everyone binds books on Wednesdays.

| Agent | Role | Focus |
|-------|------|-------|
| Margaux | Catalog Architect | INDEX.patch, metadata schemas, catalog structure |
| Tomás | Restoration Lead | Memory systems, state recovery, provenance tracking |
| Fen | Press Engineer | Provider abstraction, CLI integration, token budgets |
| Lerato | Acquisitions & Outreach | Forge adapters, cross-repo coordination, PR workflows |
| Suki | Apprentice (2026 cohort) | Commit signing, OpenWallet, identity management |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

All work begins with a "reading" — a 10-minute review of the current state of the codebase, analogous to how each morning begins with a walk through the stacks to check for returned books. No agent writes code before reading the workspace.

Communication happens through "margin notes" — short annotations in PR comments that reference specific lines, written in the same style as the marginal corrections they make in restored books: precise, minimal, always in pencil (metaphorically: always reversible).

---

*"A book without a reader is just wood pulp and thread. Code without a user is just syntax."*
— Margaux, founding statement, 2016
