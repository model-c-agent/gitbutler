# The Hermits of the Long Wave

**"The signal carries the word. The silence carries God."**

---

## The Brotherhood

The Hermits of the Long Wave are a small monastic community — twelve brothers — scattered across seven hermitages on four continents. They are connected to each other, and to nothing else, by shortwave radio. No internet. No telephone. No postal service faster than the signal propagating at the speed of light through the ionosphere.

The Order was founded in 1987 by Brother Ambrose, a Trappist monk and licensed amateur radio operator (callsign W1AMB) who believed that monastic enclosure need not mean monastic isolation. The original Rule of Saint Benedict assumed a single monastery. Brother Ambrose proposed a distributed model: hermits living alone or in pairs, observing the monastic hours independently, and connecting through scheduled radio transmissions. The Order calls these transmissions "the Office of the Air" — a liturgical hour conducted over shortwave.

The hermitages are located in: the Scottish Highlands, the New Mexico desert, a small island in the Philippine archipelago, the Namibian coast, the Australian outback, a forest in northern Finland, and a mountain village in Ecuador. Each hermitage has a shortwave transceiver, a solar power system, and a small computer. The computers are not connected to the internet. They run local software. Data enters and leaves only through the radio.

In 2022, Brother Columba — the youngest hermit, a former satellite communications engineer who took vows at age 31 — proposed using digital modes over shortwave to transmit structured data between hermitages. Not voice, not Morse code, but digital packets carrying text, configuration files, and software patches. The Order adopted FT8 and JS8Call for routine communication and a custom protocol (based on ARDOP) for bulk data transfer.

This digital link made version-controlled collaboration possible. The hermits began maintaining a shared Git repository, synchronized over shortwave. Commits are small (the channel bandwidth is measured in bytes per second, not megabytes). Synchronization happens during the evening transmission window (19:00-20:00 UTC, when ionospheric conditions favor long-distance propagation). A full sync takes 15-40 minutes depending on atmospheric conditions.

## Why This RFP

The hermits maintain several software projects: the shortwave synchronization protocol, the liturgical calendar engine (computes feast days and readings), and a contemplative journaling system. Development is slow — each hermit contributes according to their ability and availability, between prayer hours. Commits are rare and deliberate.

In 2025, Brother Columba proposed AI-assisted development. Not for speed — the hermits do not value speed — but for quality. An agent could review code for errors, suggest improvements, and verify that patches applied cleanly before they were transmitted over the narrow shortwave link. A bad patch that does not apply on the receiving end wastes 15 minutes of transmission time and cannot be retried until the next evening window.

The `but-ai` RFP appealed to the Order because of its focus on signed commits (provenance matters when you cannot verify in person), agent memory in Git (the only persistent storage that can be synchronized over shortwave), and local inference (cloud APIs are not accessible from a hermitage with no internet).

## Philosophy

Technology serves the contemplative life. It does not replace it. Every tool must justify its existence by answering: "Does this bring the brothers closer to God and to each other?" A tool that distracts is worse than a tool that does not exist. The shortwave link is justified because it enables the Office of the Air. The computer is justified because it runs the liturgical calendar. The AI agent is justified because it reduces wasted transmission time.

The hermits are deeply suspicious of abundance. Their lives are structured around sufficiency — enough food, enough warmth, enough light, enough signal. Their agent system reflects this: it uses the absolute minimum tokens, the smallest possible model, and generates the shortest possible patches. There is no surplus. Every byte transmitted over shortwave has a cost measured in battery power and propagation time.

## The Aurora Disruption

In May 2025, a geomagnetic storm disrupted shortwave propagation for six days. The hermitages were completely isolated from each other. When propagation returned, the first synchronization revealed that Brother Elias (New Mexico) and Brother Fiacre (Finland) had independently modified the same liturgical calendar function. Both patches were correct. Both were incompatible.

The merge conflict could not be resolved over shortwave — the discussion bandwidth was too narrow. Brother Columba proposed a resolution by email (the Finnish hermitage has occasional internet access through a nearby village). Brother Ambrose, the founder, refused. "If the solution requires the internet, the solution is not monastic." The conflict was resolved over three evenings of shortwave conversation, at approximately 40 words per minute.

The incident reinforced the Order's commitment to small, independent patches that minimize the probability of conflicts. The agents now check for potential conflicts before transmission.

## Achievement

**22 years of continuous operation**: The Office of the Air has been conducted without interruption since 2004, across geomagnetic storms, equipment failures, and one hurricane that destroyed the Philippine hermitage's antenna (Brother Paolo rebuilt it from salvaged aluminum within a week).

## The Brothers (Software Contributors)

| Brother | Hermitage | Role |
|---------|-----------|------|
| Columba | Scotland | Architect — protocol design, plugin development |
| Elias | New Mexico | Patch generation, liturgical calendar |
| Fiacre | Finland | Memory systems, synchronization |
| Benedict | Australia | Security, signing |
| Ambrose | Ecuador | Discernment, founder |

Details in [AGENTS.md](AGENTS.md).

---

*"The ionosphere is the cloister. The signal is the prayer."*
— Brother Ambrose, founding address, 1987
