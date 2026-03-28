# The Olfactory Neuroscience Centre

**"Between the molecule and the memory lies a synapse we do not fully understand."**

---

## Institutional Profile

The Olfactory Neuroscience Centre (ONC) is a research unit within the Department of Experimental Psychology at the University of Oxford, established in 2015 by Professor Miriam Ashworth. The Centre studies the neural mechanisms by which odour triggers autobiographical memory — the Proust phenomenon, named for the famous passage where the taste of a madeleine dipped in lime-blossom tea unlocks an involuntary childhood memory.

The Centre operates from three rooms in the Tinbergen Building on South Parks Road. Room 1 is the fMRI analysis suite (the scanner itself is shared with the wider department, booked in 90-minute slots). Room 2 is the psychophysics lab, where subjects sniff precisely metered odour samples and report their memories whilst their EEG is recorded. Room 3 is Professor Ashworth's office, which also serves as the tea room, the seminar room, and, during grant deadlines, the sleeping room.

The Centre has published 47 papers since 2015. Its most cited finding: odour-triggered memories are more emotionally vivid, more spatially specific, and more likely to be from the first decade of life than memories triggered by any other sensory modality. This finding has been replicated in four independent labs. Professor Ashworth considers it the Centre's single most important contribution and still teaches it in her first-year lectures.

## How We Arrived Here

In 2025, the Centre began building a computational model of odour-memory association. The model takes two inputs — a molecular descriptor vector (representing the odour) and a semantic memory vector (representing a category of memory) — and predicts the strength of association between them. The model is trained on the Centre's dataset of 12,000 odour-memory pairs collected over a decade of experiments.

The model was built using three AI agents: one that preprocessed molecular descriptors, one that encoded memory narratives into semantic vectors, and one that trained the association model. The agents produced intermediate outputs (cleaned datasets, encoded vectors, trained model checkpoints) that needed to be versioned, reviewed, and sometimes rolled back when a preprocessing error propagated through the pipeline.

Dr. Yuki Hashimoto, the Centre's postdoctoral computational neuroscientist, introduced GitButler after a particularly painful incident in which a preprocessing agent overwrote a hand-curated dataset with an automated version that silently dropped 400 odour-memory pairs containing non-English characters. The virtual branch model allowed each agent's work to be isolated and reviewed before merging.

## Team

Five researchers. Professor Ashworth leads with the quiet authority of someone who has supervised 22 doctoral students and forgotten more about olfaction than most people will ever learn.

| Agent | Role | Position |
|-------|------|----------|
| Prof. Miriam Ashworth | PI / Authorization & Memory Design | Professor of Experimental Psychology |
| Dr. Yuki Hashimoto | Computational Lead / Patch Architect | Postdoctoral Research Fellow |
| Dr. Emeka Obi | Psychophysics / Provider & Budget | Research Associate |
| Sophie Laurent | Forge Adapter / Coordination | DPhil Student (3rd year) |
| James Whitfield | Security & Signing | Research Software Engineer |

Profiles in [AGENTS.md](AGENTS.md).

## Tension

**The Ecological Validity Debate.** Dr. Obi argues that the Centre's computational model should be validated against ecologically valid odours — real-world smells encountered in natural environments, not isolated chemical compounds delivered through an olfactometer. Professor Ashworth counters that ecological validity sacrifices experimental control: a "real-world" coffee smell contains hundreds of volatile compounds, and you cannot determine which compound triggers the memory unless you isolate them. They have been debating this since Obi joined in 2021. The compromise is two parallel validation tracks, which doubles the workload.

## Achievement

In 2024, the Centre published a paper in Nature Neuroscience demonstrating that a specific subnetwork of the piriform cortex — the brain's primary olfactory processing area — responds preferentially to odours that the subject has personally encountered before age 10, even when the subject cannot consciously identify the odour. The paper was covered by the BBC, the Guardian, and, improbably, a perfume industry trade journal. Professor Ashworth was interviewed on Radio 4 and managed to explain the piriform cortex to a general audience in under four minutes, which she considers her finest achievement.

---

*"The nose does not remember. The brain remembers. The nose merely asks the question."*
— From Professor Ashworth's inaugural lecture, 2016
