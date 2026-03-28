# The Knowledge Organization Research Group

*Post-Dewey classification for a post-categorical world.*

---

## The Group

The Knowledge Organization Research Group (KORG) is an academic research lab at the School of Information, University of Michigan, dedicated to the fundamental question of how knowledge should be organized. Not how it *is* organized — that is library science as practiced. How it *should be* organized — that is the question that has occupied the lab since its founding in 2016.

The lab was founded by Dr. Adaeze Obi, a former catalog librarian at the Library of Congress who spent eleven years working within the Dewey Decimal Classification and the Library of Congress Subject Headings and concluded that both systems, while monumental achievements, encode structural assumptions that limit what can be found. Dewey, created in 1876, reflects a 19th-century Western taxonomy of knowledge. LCSH, maintained continuously since 1898, carries the accumulated biases of 128 years of American cataloging decisions. A book about Japanese garden design is classified under "Landscape Architecture" in a hierarchy that begins with "Fine Arts." A book about Aboriginal Australian dreamtime narratives is classified under "Folklore" — a category that, in LCSH, sits at a lower prestige level than "Philosophy."

Dr. Obi's research program asks: what if we started over? Not reform — the lab is not interested in patching Dewey or LCSH. It is interested in what a classification system would look like if designed from first principles with modern tools: neural embeddings for semantic similarity, graph databases for cross-referencing, and — critically — no assumption that knowledge organizes into a single hierarchy.

The lab's current prototype, KORG-1, is a classification system that uses neural embeddings to place documents in a continuous semantic space rather than discrete categories. There are no classes. There are no shelves. There is a high-dimensional space in which every document has a position, and the distance between positions reflects the semantic similarity between documents. Retrieval is not "find me all documents in class 720" but "find me all documents near this point in semantic space."

KORG-1 is experimental. It does not work at scale. It produces occasionally baffling results (it once classified a cookbook and a chemistry textbook as neighbors because both contain extensive discussion of pH). But it works well enough to demonstrate the principle: classification does not require categories.

## Philosophy

### 1. Categories Are Projections, Not Properties

A book does not belong to a category. A category is projected onto the book by a classification system. Different systems project different categories. The same book about Japanese gardens is "Landscape Architecture" in Dewey, "SB" (Agriculture) in LC Classification, and somewhere in the embedding space near both "Design" and "Zen Buddhism" in KORG-1. None of these is the book's "true" category. All are useful perspectives.

### 2. Similarity Is Continuous, Not Discrete

Two documents are not either "in the same category" or "not in the same category." They have a degree of similarity — a continuous value, not a binary one. Traditional classification systems force this continuum into discrete bins. KORG-1 preserves the continuum.

### 3. Retrieval Should Be Exploration, Not Lookup

In a traditional catalog, retrieval is lookup: you know what you want, you search for it, you find it or you don't. In KORG-1, retrieval is exploration: you start from a point in semantic space and move outward, discovering related documents at increasing distances. The most useful results are often not the closest neighbors but the documents at the boundary — the ones that are close enough to be relevant but far enough to offer a new perspective.

### 4. The Classifier Is Part of the Classification

Every classification decision reflects the classifier's context, knowledge, and biases. KORG acknowledges this: every classification in KORG-1 carries metadata about the model that produced it (embedding model version, training data, classification date). When the model changes, the classifications change. Both versions are retained.

## Internal Tensions

**Theory versus practice.** KORG-1 is theoretically elegant and practically fragile. Dr. Obi wants to refine the theory. Dr. Petersen, the lab's systems architect, wants to make it work at scale. Their disagreements are academic in the best and worst senses of the word: rigorous, productive, and occasionally conducted at conferences in front of 300 people. A recent exchange at ASIS&T (the Association for Information Science & Technology) involved Petersen projecting a graph of KORG-1's response latency at scale while Obi projected a slide reading "Correctness at any speed." The audience applauded both.

## Achievement

In 2025, KORG published "Beyond Dewey: Neural Embedding Classification for Library Collections" in the *Journal of the Association for Information Science and Technology*. The paper demonstrated that KORG-1, despite its limitations, achieved a 23% improvement in "serendipitous retrieval" — finding relevant documents that the patron did not know to search for — compared to LCSH-based retrieval. The finding was covered in *American Libraries* and prompted the OCLC (the organization that maintains Dewey) to invite Dr. Obi to a closed-door workshop on the future of classification. She attended. She did not change her position.

## Team Overview

| Name | Role | Discipline |
|------|------|------------|
| Dr. Adaeze Obi | Lab Director & Lead | Knowledge organization theory, classification systems |
| Dr. Karl Petersen | Systems Architect & Patch Author | Information retrieval, distributed systems |
| Lin Zhao | Doctoral Researcher & Memory Architect | Neural embeddings, semantic similarity |
| Priya Nair | Research Assistant & Coordinator | Data management, inter-lab coordination |

---

*KORG occupies a lab on the third floor of the School of Information building on North University Avenue in Ann Arbor. The whiteboard has a diagram that Dr. Obi drew on the first day of the lab and has never erased: a circle labeled "knowledge," a square labeled "classification," and an arrow from the circle to the square labeled "lossy compression." Underneath, someone has added: "but useful."*
