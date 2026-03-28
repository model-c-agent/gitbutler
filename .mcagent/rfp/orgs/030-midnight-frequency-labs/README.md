# Midnight Frequency Labs

**"You have already heard us. You just did not know it."**

---

## What We Are

Midnight Frequency Labs is an anonymous collective of producers, signal processing engineers, and cryptographers who release albums encoded as steganographic data in publicly available satellite imagery. The music is there -- in the least significant bits of weather satellite photos, in the noise floor of publicly archived radar data, in the spectral margins of astronomical survey images. You need their decoder to hear it. They have released nine albums this way. Approximately 14,000 people have the decoder. Nobody knows who they are.

The collective communicates exclusively through encrypted channels. Members use pseudonyms derived from radio frequency bands. No member knows another member's legal name. Meetings happen in text. Voices are never shared. The collective has been active since 2021 and has never met in person.

This proposal was written by six members, reviewed by four others, and approved by consensus through a voting protocol that uses homomorphic tallying -- each member submits an encrypted vote, and the results are computed without decrypting individual ballots. The proposers' pseudonyms are listed below. Their identities are not.

## How We Got Here

The collective formed around a shared frustration: the music industry's surveillance infrastructure. Streaming platforms track every listen. Labels track every play. Metadata follows the music everywhere. MFL was founded on the principle that music should be heard, not surveilled.

The steganographic release format was not a gimmick -- it was a necessity. By embedding music in existing public data, the collective ensures that the music cannot be taken down, cannot be demonetized, and cannot be attributed to individuals who might face consequences. Several members live in countries where the content of their music (political, queer, dissident) puts them at risk.

In 2024, the collective began using AI models to generate the steganographic encoding parameters. The encoding is a cat-and-mouse game: detection algorithms improve, so encoding must evolve. Rather than manually tuning parameters, the collective trained a model to find encoding schemes that survive statistical analysis. This was their first encounter with AI agent workflows -- and with the version control problems that come with them.

The encoding model's configuration files, training data references, and output parameters needed versioning. Multiple members were iterating on the model simultaneously, across time zones, without being able to share screen recordings or pair program. They needed version control that worked for anonymous, asynchronous, distributed collaboration. They tried Git. They tried Mercurial. They tried a bespoke system built on IPFS. All were inadequate.

When the `but-ai` RFP appeared, five members independently posted links to it in the collective's channel within 24 hours. The consensus vote to respond passed 9-1.

## Pseudonyms

| Handle | Frequency Band | Focus |
|--------|---------------|-------|
| **VLF** | Very Low Frequency (3-30 kHz) | Cryptography, signing |
| **HF** | High Frequency (3-30 MHz) | Protocol design, forge abstraction |
| **UHF** | Ultra High Frequency (300-3000 MHz) | Patch generation, systems code |
| **SHF** | Super High Frequency (3-30 GHz) | Memory systems, storage |
| **EHF** | Extremely High Frequency (30-300 GHz) | Budget, coordination |
| **MW** | Medium Wave (300-3000 kHz) | Review, quality assurance |

## Philosophy

1. **Anonymity is a feature.** Attribution should be opt-in, not mandatory. The system must work when the operator cannot or will not identify themselves.
2. **Data should be deniable.** Memory stored in Git refs should be encrypted. An observer who can read the refs should not be able to read the memories.
3. **Trust is distributed.** No single party -- not the forge, not the provider, not the plugin -- should have complete knowledge of the agent's activities.

## Internal Tension

The collective's central disagreement is about OpenWallet. The RFP requires signed commits via a decentralized identity system. VLF supports this -- cryptographic signing proves authorship without revealing identity. HF opposes it -- any identity system, no matter how decentralized, creates a linkable trace. The compromise: agents sign commits with ephemeral DIDs that are discarded after each session. This satisfies the signing requirement while minimizing linkability. VLF considers it mathematically sound. HF considers it "a truce, not a solution."

## Notable Achievement

In 2025, a security researcher at MIT published a paper titled "Acoustic Steganography in Publicly Available Satellite Imagery" that analyzed MFL's encoding scheme. The researcher found the data but could not decode it, proving that the steganographic layer was detectable but the content was secure. The collective considered this a partial victory: the encoding needs to be undetectable, not just undecryptable. They updated their encoding parameters within 48 hours.

---

*This document was composed collaboratively over encrypted channels.*
*No member approved this document with their legal name.*
*If you are reading this, you already know why that matters.*
