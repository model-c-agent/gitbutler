# The Kinetics & Performance Lab

**"Every movement tells a story. We read the ones that haven't been written yet."**

---

## Origin Story

The Kinetics & Performance Lab was not founded so much as it crystallized. In 2019, Dr. Anya Mwangi was running her biomechanics PhD at the University of Cape Town, filming sprinters with a Phantom high-speed camera she had borrowed (permanently, she would say) from the physics department. She noticed something that no coaching manual described: a micro-dorsiflexion in the ankle — lasting less than 4 milliseconds — that appeared in runners 18 to 22 days before they reported Achilles tendon pain. It was invisible at 240 fps. At 10,000 fps, it was unmistakable.

She published the finding. Nobody cited it. The sample size was seven. The statistics were, charitably, exploratory. The peer reviewers wrote "interesting but inconclusive" in a tone that meant "not interesting."

Anya shelved the paper and took a postdoc at ETH Zurich working on gait analysis for rehabilitation robotics. There she met Kieran Voss, a sports data scientist who had been fired from an English Premier League club for telling the head coach that his star striker's hamstring would tear within two weeks. (It tore in nine days. They fired Kieran anyway — for undermining morale.) Kieran had the statistical framework Anya lacked. Anya had the imaging pipeline Kieran needed.

They started working weekends. Within six months, they had a dataset of 340 athletes across four sports, a prediction model with 0.87 AUC for soft-tissue injury 14-28 days out, and a grant application that was rejected because the review panel could not agree on whether it was sports science, computer science, or clinical medicine.

The lab exists in the interstitial space between disciplines, and it has learned to be comfortable there.

## Philosophy

The Lab operates on three principles:

### 1. The Frame Rate Determines the Truth

At 30 fps, a runner looks fine. At 240 fps, you see compensation patterns. At 10,000 fps, you see the precursors of failure before the body itself has registered them. The Lab believes that every system — biological, computational, version-control — has phenomena that are invisible at the default observation frequency. The job is to find the right frame rate.

This principle extends to how the Lab thinks about code. A git log at one-commit-per-day resolution tells one story. The same repository observed at the patch level — individual hunks, token-by-token diffs, sub-second timing of tool invocations — tells a fundamentally different story. The Lab's agents are designed to capture and reason about these high-frequency signals.

### 2. Prediction Is Not Prescription

The Lab's injury models do not tell athletes what to do. They surface patterns. The athlete, the coach, the physiotherapist — they decide. This is not philosophical modesty; it is hard-won empirical humility. The Lab's second-generation model had a prescription layer ("rest for 3 days, reduce training load by 20%"). In a field trial, athletes who followed the prescriptions had worse outcomes than athletes who simply received the raw prediction and made their own adjustments. Complex systems resist simple instructions.

In the `but-ai` context, this translates to a design preference: agents surface options and probabilities, not commands. A patch is a proposal. A branch strategy is a hypothesis. The orchestrator — human or automated — makes the call.

### 3. Capture Everything, Compress Later

Motion capture generates terabytes. The Lab never discards raw data. Compression happens at the analysis layer, not the storage layer. You cannot go back and increase the frame rate of a recording you already made. This principle drives the Lab's memory architecture: store memories at maximum resolution, and build views (playback speeds) on top.

## Internal Tensions

The Lab has a productive but occasionally painful tension between its two intellectual centers of gravity.

**Anya's camp** favors visual, spatial reasoning. She thinks in terms of joint angles, force vectors, and body-segment coordinate systems. Her instinct is to build tools that let you *see* the data — 3D reconstructions, overlay visualizations, slow-motion playback.

**Kieran's camp** favors statistical reasoning. He thinks in terms of distributions, confidence intervals, and effect sizes. His instinct is to build tools that let you *quantify* the data — p-values, AUCs, calibration curves.

The tension is productive because the best insights come from both: a statistical anomaly that is only interpretable when you see the motion, or a visual pattern that is only convincing when you quantify its prevalence. But it means the Lab's internal reviews can get heated, with Anya waving a 3D-printed joint model and Kieran pointing at a confusion matrix.

The two junior researchers — Tomoko and Rafael — have learned to be translators. Tomoko speaks both languages fluently. Rafael, frankly, speaks neither, but his engineering skill means that both camps need him to build anything, which gives him diplomatic immunity.

## Achievements

- **The Mwangi Dorsiflexion Index (MDI):** Now cited in 23 papers and used by six professional sports organizations. The original "interesting but inconclusive" finding has been replicated with n=1,200.
- **Project Ghoststep:** A collaboration with the South African Rugby Union where the Lab's models predicted 71% of soft-tissue injuries in the 2024 season more than 10 days in advance. The results were presented at the World Congress of Biomechanics.
- **MotionDB:** An open-source database of 8,000+ motion-capture sessions across 12 sports, with full 3D kinematic data at up to 10,000 fps. The Lab's most-used contribution to the field.
- **The ETH Zurich Incident:** In 2023, the Lab's gait-analysis system flagged a subtle asymmetry in a marathon runner during routine testing. The runner felt fine. The Lab recommended an MRI. The MRI found a stress reaction in the third metatarsal that would have become a fracture within two weeks of continued training. The runner adjusted training, competed at the European Championships, and finished seventh. She sends the Lab Christmas cards.

## Failures

- **The Cricket Debacle:** In 2022, the Lab partnered with a cricket board to predict bowling injuries. The model performed brilliantly in the lab and catastrophically in the field. The reason: lab captures were on a flat, instrumented runway. Actual bowling happens on a cricket pitch with variable give. The Lab learned that transfer between controlled and ecological environments requires explicit domain adaptation, not just "more data."
- **The Real-Time Mirage:** An attempt to do real-time injury risk assessment during live matches. The system worked but introduced a 200ms latency in the video feed sent to the coaching staff. During a football match, 200ms is the difference between seeing a tackle and missing it. The coaches stopped using the system after two matches. The Lab now clearly distinguishes between real-time analysis (processing during the event) and rapid analysis (processing immediately after).

## Signature Quirk

Every research presentation at the Lab begins with a slow-motion video clip. Not as illustration — as calibration. Before any data is discussed, everyone in the room watches the same movement at the same speed. Anya calls it "tuning the eye." The practice has leaked into their software: the Lab's tools always start with a visual grounding — a status view, a diff summary, a branch topology — before any analytical output.

## Team Overview

| Name | Role | Background |
|------|------|------------|
| Dr. Anya Mwangi | Principal Investigator | Biomechanics, UCT/ETH Zurich. High-speed imaging specialist. Thinks in joint angles. |
| Kieran Voss | Lead Data Scientist | Sports analytics, ex-Premier League. Statistical modeler. Thinks in distributions. |
| Tomoko Hayashi | Research Engineer | Robotics, Osaka University. Builds the pipelines. Translates between the two camps. |
| Rafael Espinosa | Systems Developer | Self-taught. Embedded systems background. Makes the hardware talk to the software. |

The Lab is small by design. Anya has resisted every suggestion to grow beyond four. Her argument: "A biomechanics lab with more than four people starts holding meetings. Meetings are where frame rates go to die."

## Relationship to the RFP

The Lab sees the `but-ai` plugin as an instrumentation problem. The current GitButler workspace is observed at low resolution — `but status` snapshots, commit messages, branch names. The Lab proposes to increase the observation frequency: capture agent behavior at the token level, store it as high-resolution motion data, and build analysis tools that let you play back, slow down, fast-forward, and overlay agent sessions the way they play back athlete motion.

Their memory architecture — motion-capture memory — is a direct translation of their core research methodology into the version-control domain. Memories are not static records. They are time-series that look different depending on the playback speed.

---

*The Kinetics & Performance Lab is an independent research group affiliated with no university, funded by a combination of sports science grants, consulting contracts with professional sports organizations, and a small but reliable licensing income from MotionDB. They operate from a converted squash court in Cape Town that still has the court markings on the floor, which Anya refuses to paint over because "they help with calibration."*
