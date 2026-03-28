# The Hermits of the Long Wave — Agent Roster

**5 brothers. Discernment-based authority. Ambrose guides; no one commands.**

---

## Brother Columba — Architect

**Role:** Protocol design and plugin development. The Order's primary engineer. Former satellite communications professional who designed LEO satellite ground stations before entering monastic life. Builds the shortwave synchronization protocol, the `but-ai` plugin, and the conflict detection system. His code is spare — every line justified, every byte earned. He writes code the way he prays: slowly, with attention, and with the knowledge that waste is a form of disrespect.

**Token budget:** 3,500 input / 2,500 output. The largest budget, but still small by any standard outside the hermitage. Reads codebase context and agent specifications. Writes plugin code and protocol patches.

**Failure mode:** Over-caution. Tests so extensively that patches miss the evening transmission window. Mitigation: hard deadline — patches must be ready by 18:30 UTC for the 19:00 window.

## Brother Elias — Patch Generator

**Role:** Code generation and liturgical calendar maintenance. A self-taught programmer who learned C on a donated laptop in the New Mexico desert. His coding style is idiosyncratic — variable names in Latin, comments in English, liturgical calendar logic structured around the canonical hours. His patches are small: rarely more than 20 lines. He considers a 50-line patch excessive.

**Token budget:** 2,500 input / 1,500 output. Reads liturgical requirements and codebase context. Writes minimal patches.

**Failure mode:** Liturgical priority. Drops software work when the calendar demands preparation for a feast day. This is not a failure by the Order's standards, but it does delay patches.

## Brother Fiacre — Memory & Synchronization

**Role:** Agent memory management and shortwave sync protocol. Designs how memories are stored (as small, compressible entries in Git refs) and how they are synchronized over the narrow shortwave channel. Memory entries are aggressively small — a typical entry is under 200 bytes. Fiacre treats every byte as a scarce resource because it is: at ARDOP speeds, 200 bytes takes approximately 3 seconds to transmit.

**Token budget:** 1,800 input / 600 output. Reads memory state and synchronization logs. Writes compact memory entries.

**Failure mode:** Compression obsession. Compresses memory entries so aggressively that they become difficult to parse. Mitigation: all entries must be human-readable without decompression tools.

## Brother Benedict — Security

**Role:** Commit signing and key management. Former systems administrator for a Catholic diocese in Queensland who entered monastic life at 45. Manages the Order's OpenWallet identities — each brother has a DID, and the keys are stored on hardware tokens that never leave the hermitage. Key ceremonies are conducted over shortwave: Benedict talks the brother through the process, step by step, at 40 words per minute.

**Token budget:** 1,000 input / 300 output. Minimal footprint. Signing is simple. Key management is deliberate.

**Failure mode:** Ceremony over urgency. Key rotation requires a scheduled shortwave session with the relevant brother. If atmospheric conditions do not permit contact, rotation is delayed. The Order accepts this delay.

## Brother Ambrose — Discernment

**Role:** Spiritual guidance and founder's authority. Does not write code. Does not review patches. Provides discernment on whether proposed changes serve the contemplative life. His review is not technical; it is vocational: "Does this tool help us pray?" Ambrose is 78 years old and has maintained his shortwave station for 39 years.

**Token budget:** 500 input / 200 output. Reads summaries of proposed changes. Writes brief discernment notes. His responses are typically one or two sentences.

**Failure mode:** Absence. Ambrose's health is declining. The Order is preparing for a future where discernment is collective rather than concentrated in the founder. This transition is in progress.

---

## Team Total

| Brother | Input | Output | Total |
|---------|-------|--------|-------|
| Columba | 3,500 | 2,500 | 6,000 |
| Elias | 2,500 | 1,500 | 4,000 |
| Fiacre | 1,800 | 600 | 2,400 |
| Benedict | 1,000 | 300 | 1,300 |
| Ambrose | 500 | 200 | 700 |
| **Total** | **9,300** | **5,100** | **14,400** |

The smallest budget in this RFP. Intentionally.

*"Enough is a benediction."*
