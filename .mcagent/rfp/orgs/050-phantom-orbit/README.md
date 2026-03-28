# Phantom Orbit

**"The data wants to be free. We just open the cage."**

---

## Origin

Phantom Orbit has no founding date because it has no founders. It coalesced between 2020 and 2021 on an encrypted Matrix channel called `#tle-leak`, where amateur satellite trackers shared observations of objects that did not appear in any public catalogue. Someone noticed that certain classified military satellites — objects whose orbital parameters were withheld from the public Space-Track catalogue — were still visible to anyone with a decent telescope and a clear sky. The satellites existed. The data about them was simply... withheld.

The channel grew. By late 2021, thirty-odd contributors across fourteen countries were pooling observations, computing orbital elements, and publishing the results on a Tor-hosted site under the handle "Phantom Orbit." The collective had no membership list, no dues, no governance structure. You contributed data or you did not. Identity was a gpg key. Reputation was the accuracy of your orbital elements.

The US Department of Defense sent a cease-and-desist to a domain registrar in 2022. Phantom Orbit moved to IPFS. The site has been unreachable via clearnet ever since, but the data is mirrored on seventeen nodes across nine countries. The orbital elements are updated daily. The accuracy rivals Space-Track's public catalogue.

The collective's five most active contributors now form the core team submitting this RFP response. They use handles, not names. They do not attend conferences. They communicate exclusively via encrypted channels. Three of them have never met in person.

## Philosophy

Information asymmetry is the root of systemic risk. When military agencies hoard orbital data, commercial operators cannot avoid collisions with objects they do not know exist. Phantom Orbit believes that debris tracking data is a public safety necessity, not a national security asset. Their software reflects this: everything is open, auditable, and cryptographically verifiable. Trust is established through signatures, not institutions.

They apply the same philosophy to AI agents: an agent's work must be transparent, signed, and verifiable by anyone. No black boxes. No hidden state. If an agent makes a decision, the reasoning and the data that informed it must be inspectable.

## Internal Tension

The collective argues about engagement with institutions. `null_vec` (the de facto coordinator) wants to submit this RFP response and engage with commercial open-source projects like GitButler. `sat_ghost` considers any engagement with corporate entities a form of capture and voted against submitting. The vote was 3-2 in favor of submitting. `sat_ghost` abstained from the proposal but did not leave the collective. The tension is ongoing.

## Achievement

In March 2024, Phantom Orbit published the orbital elements of a previously classified signals intelligence satellite 48 hours before it performed an unannounced maneuver that brought it within 200 meters of a European commercial Earth observation satellite. The commercial operator had no warning from any official source. Phantom Orbit's publication was the only reason the operator knew to look. The operator confirmed the close approach and filed a formal protest with the UN Committee on the Peaceful Uses of Outer Space.

## Team

| Handle | Role | Known Details |
|--------|------|---------------|
| null_vec | Coordinator / Architect | Eastern Europe timezone, Rust expert |
| sat_ghost | OpSec / Infrastructure | Unknown timezone, infrastructure hardening |
| kep_flux | Data & Memory | UTC+8, orbital mechanics, data engineering |
| sig_epoch | Signing & Identity | Western Europe timezone, cryptography focus |
| drift_null | Provider & Budget | North America timezone, ML engineering |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit is signed with a gpg key whose comment field contains a haiku about orbital mechanics. The haikus are unique per key. When Phantom Orbit rotates keys, they write a new haiku. The collection currently has 23 haikus. They refuse to explain why.

---

*"You cannot classify physics. Orbits do not have security clearances."*
