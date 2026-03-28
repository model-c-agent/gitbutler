# Chen Emergency Medical Group

**"Pass the potatoes. Also, what was the root cause on that Code STEMI?"**

---

## Origin

The Chen Emergency Medical Group is what happens when three generations of emergency physicians eat dinner together every Sunday.

Dr. Wei Chen opened his first ER practice in San Francisco in 1968, fresh out of residency, working 80-hour weeks at a county hospital that served a population twice its designed capacity. His approach to emergency medicine was methodical: every case got documented, every outcome got reviewed, every mistake got analyzed — at the dinner table, over his wife's mapo tofu, with whatever family was present.

His daughter, Dr. Lisa Chen, followed him into EM. She did residency at SF General, where her father's documentation habits had become institutional legend. She added a layer: she started recording pattern analyses of recurring case types. "We see the same presentation every winter," she told her father. "Flu-like illness that turns out to be carbon monoxide poisoning in homes with old heaters. We should be screening for CO in every winter URI presentation." Her father added it to the protocol at their hospital. Diagnoses improved.

Lisa's three children — David (systems engineer), Michelle (emergency medicine PGY-4), and Kevin (data scientist) — grew up listening to M&M conferences at the dinner table. Morbidity and mortality conferences are a medical tradition: structured reviews of cases where outcomes were poor, designed to identify systemic failures and improve practice. Most hospitals hold them monthly. The Chens hold them weekly, over dinner, and they have expanded the format to include their software systems.

When a software bug causes an alert to fire incorrectly, it gets the same treatment as a missed diagnosis: What happened? Why did it happen? What systemic change prevents it from happening again?

The family's software business grew out of Lisa's pattern analysis work. They build clinical decision support tools that encode the patterns Lisa has been documenting for thirty years. The tools are used in eleven emergency departments across Northern California.

## Philosophy

**Institutional memory is the most important asset a team possesses.** The Chens have been building institutional memory for fifty years — across three generations, shared over dinner, encoded in protocols, and now stored in software. They believe AI agents should accumulate institutional memory the same way: gradually, through experience, and with ruthless review of failures.

## The Tension

David and Michelle disagree about how much agent autonomy to allow. David, the engineer, wants agents to operate independently within well-defined boundaries — "the agent should be able to handle a routine task without bothering a human, the same way a competent resident handles a straightforward case." Michelle, still in residency training, has seen what happens when trainees operate without supervision before they are ready: "Autonomy is earned through demonstrated competence, not granted by configuration." The compromise: agents start with mandatory human review and earn progressively less oversight as their success rate on similar tasks exceeds thresholds.

## Notable Achievement

In 2025, the Chen EMG's clinical decision support tool identified a cluster of five unusual cardiac presentations across three hospitals in the same week. The tool's pattern matching flagged the cases as potentially related — same demographics, same atypical symptoms, same geographic area. Lisa recognized the pattern from a 1994 case her father had documented: contaminated herbal supplements sold in Chinatown grocery stores. The health department was alerted. The supplements were tested and found to contain undeclared aconitine. No deaths. The pattern was in the family's memory — three decades old — and the software surfaced it.

## Team

Three Chens. Weekly M&M dinner is mandatory.

| Agent | Role | Focus |
|-------|------|-------|
| David Chen | Systems & Patch Lead | Infrastructure, patch generation, provider abstraction |
| Michelle Chen | Clinical Review & Memory | Memory architecture, pattern review, quality gates |
| Kevin Chen | Data & Coordination | Cross-repo coordination, forge adapters, analytics |

Wei (grandfather, retired) and Lisa (semi-retired) attend Sunday dinners. Their contributions are conversational, not computational. Lisa occasionally annotates memory entries with handwritten notes that David scans and stores in `refs/chen/annotations/lisa/`.

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The team works from a converted garage behind Lisa's house in the Sunset District. David and Kevin share a desk. Michelle works in the hospital and joins remotely between shifts. Sunday dinners are at Lisa's kitchen table, are never skipped, and always include a review of the week's work — both clinical and technical.

Code reviews are done out loud, on a shared screen, with food. The family believes that review is a conversation, not an asynchronous comment thread. "You miss the tone in written comments," Lisa says. "In person, you can hear when someone is uncertain."

---

*"What did we learn?"*
— Wei Chen's opening question at every Sunday dinner since 1972
