# The Probability Garden

**"We paint what the actuaries calculate."**

---

## What Grows Here

The Probability Garden is an artist commune in a converted greenhouse outside Eindhoven. Eight artists and three actuaries share the space, producing immersive visual installations that translate actuarial data into sensory experience. Mortality curves become topiary. Loss distributions become light shows. Confidence intervals become sound.

Founded in 2022 by Noor Bakkali, a data visualization researcher who left her post at TU Eindhoven after concluding that academic papers were the wrong medium for actuarial communication. "A mortality table is the most important document most people will never read," she said. "We make it impossible to look away."

The commune's first installation, *Your Remaining Tuesdays* (2023), projected a personalized countdown of remaining Tuesdays in a visitor's expected lifetime onto a garden wall, using age and country data entered at the door. It was exhibited at the Dutch Design Week and made three visitors cry. Noor considers this a success.

## The Software Turn

The installations run on software. Every light cue, sound trigger, and projection mapping coordinate is generated from actuarial models. The codebase grew organically -- Python scripts, Arduino sketches, Processing visuals -- until 2024, when a thunderstorm shorted the greenhouse's electrical panel and they lost a month of uncommitted work.

After that, everything went into Git. After GitButler, the artists could work on parallel visual variants of the same installation without merge conflicts. After the `but-ai` RFP, they saw the chance to have AI agents handle the data pipeline (fetch mortality tables, fit distributions, generate the numbers) while artists focus on the translation to visual experience.

## Philosophy

Numbers without aesthetics are ignored. Aesthetics without numbers are decoration. The commune believes actuarial data becomes meaningful only when it is felt, not just understood. Their installations aim for visceral comprehension: you should *feel* what a 2% mortality rate increase means, not just know it.

They apply this to code as well. Noor insists that the codebase itself be beautiful -- not in the subjective "clean code" sense, but in the visual sense. Variable names are chosen for rhythm. Functions are organized by the narrative of the installation, not by technical module boundaries.

## Internal Tension

**The Accuracy Debate.** Jonas (sculptor, no math background) wants to exaggerate actuarial data for emotional impact -- stretch a mortality curve to make the peak more dramatic. Elif (actuary) insists on strict fidelity: "If we distort the data, we are no better than the insurers who obscure it." The current rule: data transformations must be documented and reversible. Artists can reshape, but never fabricate.

## Notable Achievement

*Confidence Interval Garden* (2025), an outdoor installation where hedges were trimmed to the shape of confidence intervals around Netherlands life expectancy data. The 95% interval was a wide, sprawling hedge. The 50% interval was a tight, manicured row. Visitors walked through the intervals, physically experiencing the difference between certainty and uncertainty. The installation was acquired by the Kroller-Muller Museum.

## Team Overview

| Agent | Role | Medium |
|-------|------|--------|
| Noor | Lead / Memory | Data visualization |
| Elif | Patch Generation | Actuarial modeling |
| Jonas | Review / Aesthetics | Sculpture |
| Aiko | Forge Coordination | Sound design |
| Tobias | Security & Signing | Light engineering |
| Preet | Provider & Budget | Installation tech |

Details in [AGENTS.md](AGENTS.md).

---

*"Walk through the data. Feel the confidence interval."*
