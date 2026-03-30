# Questions from Tanaka (083) to Hartmann (084 - Loom & Verse)

## Question 1: Motif emergence threshold and cold-start

Your `MotifTracker` requires 3 appearances before a theme becomes a motif. In our woven memory, new warp threads can influence retrieval immediately through the pattern selector. How does your system handle the cold-start problem -- when an agent begins working on a new codebase with zero chapters and zero motifs? During those first dozen tasks, motif-based retrieval returns nothing, and you fall back to embedding similarity alone. Have you considered a "proto-motif" weighting scheme where themes that have appeared once or twice still contribute to retrieval scoring, even before formal motif promotion?

## Question 2: Tension escalation and cross-arc interference

Your tension system escalates unresolved contradictions after 14 days. But what happens when a tension spans multiple arcs? For example, a session timeout tension introduced in the auth arc that affects the export arc. Your `Tension` struct has `introduced_in` (a chapter number within an arc) but no cross-arc reference. In our loom model, a warp thread can be interlaced with weft from any task regardless of category. How would you handle tensions that are relevant to arcs they were not introduced in?

## Question 3: Arc dormancy during active tension

Your `ArcState` has Active, Dormant, and Archived states, with dormancy triggered by inactivity. But what if an arc has an active (unresolved) tension but no new chapters? Should the arc go dormant? Your proposal says tensions persist across arc dormancy, but the arc summary that replaces individual chapters during dormancy might lose the context needed to understand and resolve the tension. How do you prevent information loss during dormancy for arcs with active tensions?
