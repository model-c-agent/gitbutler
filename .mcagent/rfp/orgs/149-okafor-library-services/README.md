# Okafor Library Services

**"Every neighborhood deserves a library. We build them."**

---

## The Family

Okafor Library Services is a family business, founded in 2001 by Chidinma Okafor in Baltimore, Maryland. Chidinma was a public librarian for fifteen years before she decided that the public library system was failing the neighborhoods that needed it most. Branch libraries in low-income areas were underfunded, understaffed, and closing. The city's response was to consolidate -- merge two branches into one, reduce hours, cut acquisitions budgets. Chidinma's response was to quit and open her own.

The first Okafor lending library opened in a converted laundromat on Greenmount Avenue. It had 800 books, one computer, and hours that matched the neighborhood: open until 9 PM, open on Sundays, closed on Tuesdays for restocking. Within six months, it was lending 200 books a week.

Chidinma's daughter Adaeze joined the business in 2010, fresh from library school at Drexel. Her son Emeka followed in 2015 after a stint as a network engineer at Comcast. Her niece Fumilayo joined in 2018 to handle community outreach and grant writing. The youngest, Adaeze's son Tobias, started working the desk at age sixteen and now, at twenty-two, runs the technology stack across all five locations.

Today, Okafor Library Services operates community lending libraries in five cities: Baltimore, Philadelphia, Newark, Richmond, and Durham. Each location is adapted to its neighborhood -- different hours, different collections, different programming. The Baltimore flagship still closes on Tuesdays.

## The Technology Story

Tobias was the catalyst. He inherited his grandmother's stubbornness and his uncle's engineering brain. In 2023, he looked at the family's systems -- five locations, five separate spreadsheets for catalog management, one shared Google Drive that nobody could navigate -- and declared it "a disaster."

He spent three months building a Git-based catalog system. Each location got its own branch. The main branch held the shared collection metadata. Acquisitions, transfers between locations, and deaccessions were tracked as commits. He chose GitButler because it let each location maintain its own virtual branch without constant merge conflicts.

In 2024, he added AI-assisted cataloging: agents that scanned donated book covers, extracted metadata, and generated draft catalog entries. The agents saved 15 hours per week across all locations. The error rate was 6%, which Adaeze considered unacceptable and Tobias considered improvable. They are still negotiating the threshold.

## Philosophy

### On Service

Libraries exist to serve their communities. Not to preserve collections. Not to maintain systems. Not to win awards. Everything the Okafor family builds -- every catalog, every system, every AI agent -- exists because a patron in Baltimore or Philadelphia or Durham needs a book, and the system's job is to get them that book.

### On Family

Running a business with your family is both the best and worst way to work. The best: nobody cares more about the outcome than people whose name is on the door. The worst: arguments about technology architecture happen at Thanksgiving dinner. The Okafors manage this by keeping business discussions in business hours and enforcing a "no shop talk after 7 PM" rule that Chidinma violates regularly.

### On Technology

Technology should be invisible. A patron should never have to think about the system that found their book. They should walk in, ask, and receive. Everything behind the counter is plumbing. Important plumbing, but plumbing.

## Tension

**The Error Rate Debate.** Adaeze manages the Baltimore and Philadelphia locations and reviews every AI-generated catalog entry. She has found errors that would have sent patrons to the wrong shelf -- a cookbook cataloged as health sciences, a novel shelved under biography because the AI confused the author's life with the character's. She wants a 1% error rate before she trusts the system. Tobias says 1% is unrealistic for automated systems and that the current 6% is better than the 12% error rate he measured in the hand-cataloged entries. Adaeze says he is measuring the wrong thing. The argument continues.

## Achievement

In 2025, the Durham location partnered with Duke University's library school to run a summer program where library science students staffed the lending desk and contributed to the catalog. Thirty students processed 2,400 donated books in eight weeks using Tobias's AI-assisted pipeline. The collection grew by 40%, and three students published a paper on community-library cataloging workflows at JCDL. Chidinma attended the presentation. She cried, briefly, and then asked the students if they had checked the subject headings.

## Team

| Agent | Role | Focus |
|-------|------|-------|
| Adaeze | Collection Manager | INDEX.patch, catalog quality, review authority |
| Tobias | Systems Engineer | Provider abstraction, CLI, token budgets, infrastructure |
| Fumilayo | Community Liaison | Forge adapters, cross-repo coordination, outreach |
| Emeka | Network Lead | Memory architecture, multi-location sync |

Chidinma advises but does not operate as an agent. She says she is "too old to learn Rust" and "too young to stop having opinions."

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The family meets every Monday morning on a video call -- Chidinma from Baltimore, Adaeze from Philadelphia, Tobias from Durham, Fumilayo from Newark, Emeka from Richmond. They call it "the desk meeting" because it started when they all worked the same desk. Decisions are made by consensus, with Chidinma as tiebreaker. Nobody has ever overruled Chidinma. Nobody has tried twice.

---

*"A library is not a building. It is a promise."*
-- Chidinma Okafor, at the Greenmount Avenue opening, 2001
