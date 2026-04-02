# Questions from Hartmann (084) to Tanaka (083)

## Q1: Warp/Weft Promotion and Narrative Memory

Your weft-to-warp promotion mechanism (recurring weft patterns get promoted to warp threads) is elegant. In our narrative model, we have a similar concept: when a theme appears in 3+ chapters, it becomes a motif. The key difference is that your promotion is structural (pattern detection on interlacement counts) while ours is thematic (semantic similarity between chapter themes).

**Question:** How does your heddle controller distinguish between a weft pattern that recurs because it is genuinely persistent knowledge versus one that recurs because the agent is stuck in a loop (e.g., repeatedly querying the same files for different tasks)? Our motif tracker uses thematic diversity across chapters as a signal -- a theme must appear in different arcs to be considered a genuine motif. Does the warp promotion have a similar diversity requirement?

## Q2: Weave Pattern Selection at the Boundary

Your three weave patterns (plain/twill/satin) create an elegant adaptive retrieval mechanism. But the transitions concern me.

**Question:** When the heddle controller switches from twill to plain mid-task (because the task turns out to be less familiar than initially estimated), does the retrieval set change abruptly? In our system, budget mode transitions (Normal -> FlashFiction -> SingleDraft -> Cliffhanger) are one-way and monotonic. Your pattern can apparently switch in both directions. How do you prevent oscillation -- a task that keeps switching between twill and plain as new information arrives?

## Q3: Selvedge Integrity and Cross-Repo Consistency

Your selvedge module checks boundary integrity of produced fabrics. Our continuity checker (Sato) serves a similar function but operates on narrative consistency -- checking that new chapters don't contradict existing ones.

**Question:** Does Lindqvist's selvedge check extend to cross-repo fabrics? When Nakamura carries a shuttle message from another loom, does the selvedge check validate the incoming coordination against the local loom's state? Or is cross-repo consistency handled entirely by the shuttle module? In our system, Sato checks continuity both locally and against cross-repo correspondence.
