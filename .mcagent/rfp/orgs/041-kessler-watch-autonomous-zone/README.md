# Kessler Watch Autonomous Zone

**"Nobody assigned us. Nobody can unassign us."**

---

## What Kessler Watch Is

Kessler Watch Autonomous Zone is a decentralized network of amateur astronomers, satellite trackers, and orbital debris enthusiasts who monitor the space debris environment without funding, without authorization, and without coordination. The name references the Kessler Syndrome — the theoretical cascade of collisions that could render low Earth orbit unusable — and the word "autonomous" because nobody asked them to do this.

The network has approximately 400 participants across 30 countries. Some are retired aerospace engineers. Some are ham radio operators who track satellites as a hobby. Some are undergraduate astronomy students contributing observations for thesis projects. Some are just people with telescopes who think someone should be watching the sky and noticed that the official watchers are not sharing their data.

There is no organization. There is a shared Git repository, a Mastodon instance (@kesslerwatch@astrodon.social), and a set of IRC channels where people argue about orbital mechanics, TLE accuracy, and whether the US Space Force is undercounting debris below 10 cm.

## Origin

The network crystallized in 2021 when the Russian ASAT test created 1,500+ trackable debris fragments in low Earth orbit. Official agencies (the US 18th Space Defense Squadron, ESA's Space Debris Office) published initial tracking data, then went quiet — classified portions of the debris catalog are not public. Amateur trackers filled the gap, sharing visual observations, radar returns, and computed orbital elements through a patchwork of forums and mailing lists.

In 2022, a contributor named Jules Moreau (a retired CNES orbital analyst in Toulouse) proposed consolidating the observations into a shared, version-controlled catalog. They created the Git repository, wrote a data schema for observation records, and posted the link in every space tracking forum they could find. Within six months, the catalog had 12,000 observations from 80 contributors.

The problem was coordination. Observations came in different formats, different reference frames, and different quality levels. Two observers could report the same object with TLE sets that diverged by kilometers. Nobody was responsible for reconciliation because nobody was responsible for anything.

This lack of coordination is simultaneously the network's greatest weakness and its ideological core. Kessler Watch exists because the participants believe that debris tracking should be public, decentralized, and independent of any government. Adding hierarchy would make it more effective and less free. They choose freedom.

## Team (Proposal Contributors)

| Handle | Location | Focus |
|--------|----------|-------|
| **jules** | Toulouse, FR | Orbital mechanics, data reconciliation |
| **k-band** | Melbourne, AU | Radar observations, signal processing |
| **sunspot** | Cape Town, ZA | Visual observations, optical tracking |
| **apogee** | Kyoto, JP | Memory systems, data persistence |
| **debris_field** | Edmonton, CA | Budget, signing, infrastructure |

## Philosophy

1. **The sky belongs to everyone.** Debris data should be public. Classifying it is a political decision, not a scientific one.
2. **No leaders, only contributors.** Coordination happens through protocol, not authority.
3. **Observe first, model later.** Raw observations are more valuable than computed orbits because observations are facts and orbits are interpretations.

## Internal Tension

Jules and k-band disagree about data quality thresholds. Jules wants to enforce minimum observation quality — a TLE with an RMS residual above 5 km should not enter the catalog. k-band argues that low-quality observations from diverse locations are more valuable than high-quality observations from a few sites, because diversity reveals systematic errors in official catalogs. The current catalog includes all observations with a quality tag but no quality gate. This means the catalog contains both precise laser rangefinding data and naked-eye estimates, which is either comprehensive or chaotic depending on your perspective.

## Notable Achievement

In 2025, the network independently identified a close approach between a Starlink satellite and a debris fragment from the 2021 ASAT test that official agencies had not publicly flagged. The prediction was posted on Mastodon 18 hours before closest approach. The satellites maneuvered 6 hours later. ESA's Space Debris Office subsequently confirmed the conjunction. The network claims this as validation. ESA neither confirms nor denies using amateur data. The truth is probably in between.

---

*Observed from 30 countries. Cataloged by no one in particular. The sky does not care who is watching.*
