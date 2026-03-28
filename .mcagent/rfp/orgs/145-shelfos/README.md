# ShelfOS

**"Every book in its place. Every patron on their way."**

---

## Origin Story

ShelfOS was founded because Miriam Achterberg spent eleven years as a public librarian and could not stop thinking about the shelving cart.

The shelving cart — that humble metal trolley loaded with returned books — is the bottleneck of every library. A patron returns a book. The book goes on the cart. The cart sits until someone wheels it to the stacks and re-shelves each book by call number. In a busy branch, a book can sit on the cart for hours, sometimes days. During that time, the library's catalog says the book is "available" but a patron walking to the shelf will find it missing. The patron asks a librarian. The librarian checks the cart. The patron waits. Multiply this by hundreds of transactions a day, and you have a system where the catalog — the library's source of truth — is perpetually out of sync with physical reality.

Miriam spent five years trying to fix this with process improvements. Color-coded carts. Shelving schedules. Student worker training programs. None of it worked because the fundamental problem was not process — it was observability. The library did not know where its books were in real time.

In 2023, Miriam attended a computer vision conference (she was there for a talk on OCR for damaged manuscripts) and saw a demo of a warehouse inventory system that used ceiling-mounted cameras to track product locations. She had what she describes as "the most boring epiphany in the history of technology": this is just a shelving cart problem with barcodes.

She quit her job the next week and enrolled in a machine learning bootcamp. She was the oldest person in the cohort by fifteen years. She was also the most motivated. Three months later, she had a prototype: a Raspberry Pi with a camera that could read spine labels and report shelf positions to a central database.

She recruited Tomás Guerrero-Blanco, a robotics engineer who had been building inventory drones for Amazon and had quit because he wanted to work on something with, in his words, "a soul." Miriam's pitch was one sentence: "Libraries are warehouses that care about their users." Tomás said yes.

ShelfOS was incorporated six months later. The name is literal: an operating system for shelves.

## Philosophy

### 1. The Catalog Is Sacred

A library catalog is a promise: this book exists, it is here, you can find it. When the catalog is wrong, the promise is broken. ShelfOS exists to keep the promise. Every feature, every design decision, every architectural choice is evaluated against a single question: does this make the catalog more accurate?

In the `but-ai` context, ShelfOS treats the repository's state — its branches, commits, working tree — as a catalog. The catalog must be accurate. An agent that reports "task complete" when the patch hasn't been applied has broken the catalog. An agent that creates a branch but doesn't update the coordination metadata has broken the catalog.

### 2. Multiple Classification, One Object

A book is a single physical object, but it exists in multiple classification systems simultaneously. It has a Dewey Decimal number. It has a Library of Congress call number. It has subject headings. It has an ISBN. It has author, title, date, publisher. Each classification system is a different lens on the same object, and each is useful for different queries. A patron looking for "books about the French Revolution" needs subject headings. A patron looking for "that book by the woman with the French name" needs author search. A cataloger verifying a new acquisition needs ISBN.

ShelfOS believes that memories — like books — should be indexed by multiple classification systems simultaneously. A memory about "how to handle merge conflicts in Rust" should be findable by subject (merge conflicts), by language (Rust), by date (when it was learned), by source (which task produced it), and by call number (a hierarchical position in the agent's knowledge structure). Cross-references connect related memories across classification systems.

### 3. "See Also" Is The Most Powerful Link

In a card catalog, the most valuable notation is "See also." It tells you: the thing you're looking for is related to this other thing you didn't think to look for. "See also" links are created by catalogers who understand the knowledge domain well enough to see connections that the classification system's structure doesn't capture.

ShelfOS's memory system makes "see also" links first-class citizens. When an agent learns something, the system doesn't just store the memory — it identifies connections to existing memories and creates bidirectional "see also" links. These links form a navigable graph that lets agents discover relevant memories they didn't explicitly search for.

### 4. The Library Is A Living System

A library is not a warehouse. A warehouse stores things. A library circulates them. The value of a library is not the number of books on its shelves but the number of books in the hands of its patrons. ShelfOS designs for circulation, not storage. Memories that are never retrieved are dead weight. The system tracks retrieval frequency and surfaces under-used memories for potential expiration.

## Internal Tensions

ShelfOS is a three-person startup with a two-mind problem.

**The librarian vs. the engineer.** Miriam thinks like a librarian. She cares about classification, access, equity, and the patron experience. Tomás thinks like an engineer. He cares about latency, accuracy, uptime, and system reliability. These are not in conflict — they are complementary — but they speak different languages. When Miriam says "the system should be accessible," she means "people with disabilities should be able to use it." When Tomás says "the system should be accessible," he means "the API should have clear documentation and stable endpoints." They have learned to ask "which kind of accessible?" before proceeding.

**The startup paradox.** ShelfOS is a startup that sells to libraries. Libraries are the most budget-constrained institutions in public life. They operate on fiscal-year procurement cycles, require RFPs for purchases above $5,000, and evaluate technology on a five-year horizon. Startups operate on runway, move fast, and evaluate technology on a quarterly horizon. The mismatch is profound. ShelfOS has adapted by offering a free tier (basic inventory tracking for branches with fewer than 10,000 items) and a paid tier (full wayfinding, real-time shelf tracking, patron recommendation) that libraries can budget for in next year's cycle.

**The scope question.** Miriam wants ShelfOS to do everything a library needs: inventory, cataloging, circulation, patron services, wayfinding, programming schedules, community room booking. Tomás wants to do one thing — shelf inventory — and do it perfectly. Wei moderates: build the shelf inventory perfectly, then expand only when libraries ask for the next thing. So far, libraries have asked for wayfinding (done), patron self-checkout (in progress), and a way to find the bathroom (not a technology problem, but they added a map anyway).

## Achievements

- **12 Library Systems.** ShelfOS is deployed in 12 public library systems across the United States, covering 47 branches.
- **Real-Time Shelf Accuracy.** In deployed branches, shelf accuracy (the percentage of items whose catalog location matches their physical location) improved from an average of 72% to 96%.
- **The Cleveland Demo.** At the American Library Association conference in 2025, ShelfOS demonstrated a patron walking into a library, searching for a book on their phone, and being guided to the exact shelf position in under 10 seconds. The demo went viral in library circles (which is a smaller circle than you might think, but deeply enthusiastic).
- **Accessibility Partnership.** ShelfOS partnered with the National Library Service for the Blind and Print Disabled to develop an audio wayfinding system that guides visually impaired patrons to materials using spatial audio cues through their phone.
- **The Shelf-Reading Elimination.** "Shelf reading" — the manual process of walking the stacks to verify that every book is in the right place — takes approximately 40 person-hours per week in a medium-sized branch. ShelfOS's cameras do it continuously. The twelve deployed library systems have collectively recovered an estimated 25,000 person-hours of librarian time per year, redirected to patron services.

## Failures

- **The Privacy Incident.** An early version of ShelfOS's patron wayfinding system tracked patron movement through the library to optimize shelf placement. A library board member pointed out that tracking which sections patrons visit is functionally a reading surveillance system. The feature was removed within 24 hours and ShelfOS published a privacy commitment: cameras track books, never patrons. The system was redesigned to use anonymous, aggregated traffic patterns for shelf optimization.
- **The Dewey Decimal Assumption.** ShelfOS's first prototype assumed all libraries used the Dewey Decimal Classification. It turns out that academic libraries use Library of Congress Classification, many special libraries use custom schemes, and some public libraries have abandoned formal classification entirely in favor of "bookstore-style" browsing categories. The system now supports pluggable classification schemes, but the initial assumption cost three months of rework.
- **The Wi-Fi Dependency.** ShelfOS requires Wi-Fi for real-time updates. Three library branches in rural areas had such poor Wi-Fi that the system couldn't maintain a connection. ShelfOS built an offline mode that caches updates and syncs when connectivity is available, but the experience is degraded. Miriam considers this a failure of equity: the libraries that need the most help are the ones with the worst infrastructure.

## Signature Quirk

ShelfOS uses library terminology for everything. Bugs are "misfiled." Deployments are "accessions." Deprecated features are "deaccessioned." The product roadmap is "the collection development plan." Sprint retrospectives are "shelf readings." New team members are "accessioned" and departing team members are "withdrawn." It started as Miriam's habit and became the company culture. Outsiders find it charming for about ten minutes and then mildly annoying, which is exactly how Miriam felt about tech jargon when she first entered the startup world.

## Team Overview

| Name | Role | Background |
|------|------|------------|
| Miriam Achterberg | CEO / Product Lead | Public librarian for 11 years, MLIS from University of Washington. Thinks in call numbers. |
| Tomás Guerrero-Blanco | CTO / Engineering Lead | Robotics, ex-Amazon Robotics. Builds the cameras and the CV pipeline. Wants to do one thing well. |
| Wei Zhang | Full-Stack Developer | CS, Stanford dropout. Builds the catalog sync, the API, and the patron-facing apps. Mediates the scope wars. |

## Relationship to the RFP

ShelfOS sees the `but-ai` plugin as a cataloging problem. Agent memories are items in a collection. They need to be classified, shelved, circulated, and eventually deaccessioned. The catalog — the index that makes the collection navigable — is the core intellectual contribution.

Their memory architecture — card-catalog memory — indexes every memory by multiple classification systems simultaneously: subject (what the memory is about), source (where it came from), temporal (when it was acquired), relational (what it connects to), and hierarchical (where it sits in the agent's knowledge structure). Cross-references ("see also" links) create a navigable graph that goes beyond keyword matching to capture semantic relationships between memories.

The proposal is the most library-like document you will ever read from a software startup. It includes a classification scheme, a circulation policy, and a deaccession protocol. Miriam would have it no other way.

---

*ShelfOS operates from a co-working space in Seattle that is, by coincidence, located above a used bookstore. Miriam takes this as a sign. Tomás takes it as a lease negotiation outcome. Wei doesn't have an opinion because Wei works remotely from San Francisco and has visited the office exactly twice.*
