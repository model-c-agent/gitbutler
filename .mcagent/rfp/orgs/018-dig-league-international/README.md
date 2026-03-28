# Dig League International

**"Fastest trowel wins."**

---

## What We Are

Dig League International is the world's only competitive archaeological excavation organization. Yes, you read that correctly. We run timed excavation competitions where teams race to survey, excavate, and document archaeological features against the clock. Our annual championship in Anatolia draws 32 teams from 19 countries. ESPN3 streamed it once. The ratings were modest but the comment section was electric.

Founded in 2021 by Dr. Amir Farooqi, a field archaeologist who spent fifteen years watching excavation teams work at wildly different speeds and wondered: what if we measured it? What if we created incentives for speed without sacrificing quality? What if we made archaeology competitive?

The first tournament was held at a training site in Turkey — a purpose-built archaeological simulation where known artifacts were seeded at known depths in controlled stratigraphy. Teams were scored on three metrics: speed (time to complete the survey unit), accuracy (percentage of features correctly identified), and documentation quality (completeness and clarity of the site record). The scoring formula weights all three equally, so a team that finishes fast but documents poorly scores the same as a team that documents perfectly but takes forever.

## How We Got Into Software

The league's scoring system started as a Google Sheet. By the third tournament, it had become a sprawling, macro-laden monstrosity that crashed every time more than two judges entered scores simultaneously. We needed a real system.

Farooqi's cousin (there is always a cousin) was a software developer in Istanbul. He built a scoring platform: mobile apps for field judges, a real-time leaderboard, and an API for streaming integration. The platform worked well, but managing the data — scores, site records, photogrammetry, team rosters, tournament brackets — required constant human attention.

In 2024, we added AI agents to automate scoring validation (checking that judges' scores were consistent), leaderboard updates, and documentation quality assessment. The agents compared submitted site records against a rubric and flagged teams whose documentation fell below threshold. This freed human judges to focus on field observation rather than paperwork.

The agents needed version control because each tournament generates hundreds of scoring events and documentation submissions that must be tracked, versioned, and auditable (teams protest scores, and we need to show exactly what was submitted when). GitButler's virtual branches let us isolate each team's submissions while maintaining a unified tournament timeline.

## Philosophy

Competition reveals capacity. A team that has never been timed does not know how fast it can work. A team that has never been scored against a rubric does not know where its weaknesses are. We believe the same applies to AI agents: benchmarking, scoring, and competition produce better agents than unsupervised development.

Our agents are built to be measured. Every output includes metrics: time to completion, token cost, patch accuracy, documentation score. We do not hide our numbers.

## Internal Tension

The league argues about whether speed or quality should be weighted more heavily in the scoring formula. The "sprinters" (teams that prioritize speed) lobby for higher speed weights. The "documentarians" (teams that prioritize meticulous records) lobby for higher quality weights. The current formula weights them equally, and both factions consider this unsatisfactory. The debate has been ongoing since Tournament 2 and shows no sign of resolution.

## Notable Achievement

The 2025 Anatolia Championship featured a new category: AI-assisted excavation. Teams could deploy up to three AI agents to assist with documentation and feature identification during the dig. The winning team — PastStack (see org 015) funded two of its interns to compete — used agents that generated site records in real-time from tablet-entered observations. Their documentation score was 98.7%, the highest in league history. Their speed was middling. They won on documentation alone.

## Team Overview

Five agents structured as a competition team. One team captain calls strategy and manages the clock (task decomposition and budget). Two field agents handle primary implementation (patch generation). One documentation agent ensures all outputs meet the league's quality rubric. One judge agent validates outputs against scoring criteria before submission. The team is optimized for speed-quality balance: fast enough to finish, thorough enough to score well.

---

*"The clock is running. Dig."*
