# Phantom Stats

**"The data is already public. The broadcast just forgot to mention it."**

---

## Origin

Phantom Stats does not exist, legally. There is no incorporated entity, no registered domain, no LinkedIn page. What exists is an IRC channel (yes, IRC — they consider Discord "a surveillance platform with emoji"), a shared GPG keyring, and six people who have never met in person but collectively operate the most accurate publicly available pitch-tracking dataset outside of MLB's proprietary Statcast system.

They started in 2020, during COVID, when live baseball was replaced by Korean Baseball Organization broadcasts available on ESPN. A computer vision researcher (handle: `nullswing`) noticed that the KBO broadcast overlay included pitch speed but not spin rate, location, or movement — data that Statcast captures with Doppler radar at every MLB stadium. nullswing wrote a neural network that estimated spin rate from high-framerate broadcast video by tracking the ball's seam rotation across consecutive frames.

The accuracy was surprising: within 8% of radar-measured spin rate for fastballs, 12% for breaking balls. nullswing posted the model and a dataset on an anonymous Gitea instance. Within two months, five other researchers had forked it, improved it, and extended it to extract pitch location, release point, and horizontal break — all from standard broadcast video.

The collective formed around this shared project. No one proposed it. No one named it. The Gitea instance already had a placeholder name: `phantom-stats`. It stuck.

Their dataset now covers MLB, NPB, KBO, and the Mexican League. They extract data from publicly available broadcast feeds, process it through open-source models, and publish the results on their Gitea instance. MLB's legal department has sent two cease-and-desist letters to the Gitea hosting provider, both ignored because the server is in Iceland and the provider does not recognize US intellectual property claims over mathematical transformations of publicly broadcast images.

## Philosophy

Information asymmetry is the enemy. When one party has proprietary data and the other does not, the informed party extracts rent. Phantom Stats exists to eliminate information asymmetry in sports analytics by making all trackable data publicly available.

They extend this to AI agents: an agent should never operate on hidden state. All agent state — memory, configuration, decision history — must be inspectable by any participant in the system. They consider opaque AI systems to be the computational equivalent of proprietary tracking data: power hoarded through information control.

## The Tension

`nullswing` and `deadball` disagree about attribution. nullswing believes all contributions should be pseudonymous — the work speaks for itself, names are irrelevant. deadball argues that pseudonymity is fine for the collective's public-facing work but that *internal* contributions should be attributed to enable accountability. "If someone pushes a bad model update, we need to know who to ask about it." nullswing counters: "If someone pushes a bad model update, the tests catch it. Attribution is social, not technical." The current compromise: internal Git commits are signed with pseudonymous GPG keys, providing accountability without identity.

## Notable Achievement

In February 2026, Phantom Stats published a dataset showing that a specific MLB pitcher's spin rate had declined 14% over three starts — a pattern consistent with fatigue-related injury risk. The pitcher's team had not publicly acknowledged any issue. Three days after the dataset was published, the team placed the pitcher on the injured list with "forearm tightness." Phantom Stats did not claim credit. The dataset spoke for itself.

## Team

Six pseudonymous contributors. No leader. Decisions by rough consensus on IRC.

| Handle | Role | Focus |
|--------|------|-------|
| nullswing | CV/ML Lead | Computer vision models, patch generation |
| deadball | Systems | Infrastructure, provider abstraction |
| velo | Data Ops | Data pipeline, memory architecture |
| ribbons | Security | Key management, signing, anonymization |
| shortporch | Forge Work | Cross-repo coordination, PR automation |
| phantom (the bot) | Budget | Token tracking, automated cost alerts |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

Asynchronous, pseudonymous, IRC-coordinated. No meetings. No video calls. No voice. All communication is text, logged, and searchable. Contributions are reviewed by minimum two members before merge. The Gitea instance runs on a server that `deadball` administers; no one else knows its physical location. Backups are distributed across three encrypted remotes maintained by different members.

---

*No signature. No attribution. The data is the statement.*
