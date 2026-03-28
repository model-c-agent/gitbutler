# Static & Murmur

**"You are standing inside a 5G signal. Can you hear it singing?"**

---

## The Studio

Static & Murmur is an artist collective based in a former telephone exchange building in Rotterdam. Five artists who work at the intersection of telecommunications infrastructure and sonic art. We make installations that translate invisible signals — radio waves, Wi-Fi beacons, cellular handshakes, GPS timing pulses — into audible experiences. Our medium is the electromagnetic spectrum. Our gallery is the air.

The collective was founded in 2021 by Zara Hoekstra, a sound artist who became obsessed with the sonic character of radio frequency interference during a residency at a radio telescope observatory. She spent three months listening to the electromagnetic environment through a software-defined radio and realized that the RF spectrum is not silence — it is a dense, layered, constantly shifting soundscape that humans simply cannot hear without transduction.

Zara recruited four collaborators: a spatial audio engineer, an RF hardware designer, a visual artist who works with data visualization, and a programmer who had previously built real-time audio synthesis software for electronic musicians. Together, they build installations that make the invisible infrastructure of modern telecommunications perceptible. Their work has been exhibited at Ars Electronica, transmediale, the Museum of Contemporary Art in Tokyo, and in public spaces ranging from train stations to cell tower rooftops.

## The Technical Problem

Our installations are complex software systems. Each piece combines: custom RF capture hardware, real-time signal processing, spatial audio rendering (12-channel or higher), and generative visual projections. The software stack is version-controlled in Git. An installation's configuration — which frequencies to monitor, how to map signal characteristics to audio parameters, the spatial layout of speakers — is a set of structured files that we iterate on during the development process.

In 2024, we started using AI agents to explore the aesthetic possibility space. Given an RF capture and a set of audio mapping rules, an agent generates parameter variations — different timbral qualities, different spatial distributions, different temporal rhythms. Each variation is a potential artistic interpretation of the same underlying signal. We listen to the variations, discuss them, and select the ones that resonate.

The problem was version control. When three agents each generate five variations, we have fifteen virtual branches of an installation's configuration. We needed a way to manage these branches, compare them, and merge selected elements. GitButler's virtual branch model was the answer. The `but-ai` RFP gives us a framework to formalize the agent-assisted creative exploration that is already central to our practice.

## Philosophy

Telecommunications infrastructure is a hidden landscape. Billions of signals cross through your body every second — cellular, Wi-Fi, Bluetooth, GPS, radio, television — and you perceive none of them. Our work makes this landscape perceptible. We do not editorialize. We do not add meaning. We transduce: converting one form of energy (electromagnetic) into another (acoustic), as faithfully as possible, and let the listener draw their own conclusions.

Agents serve this philosophy by generating transduction mappings we would not have conceived. A human sound artist has aesthetic habits — preferred timbres, comfortable frequency ranges, familiar spatial patterns. An agent has no habits. It generates mappings from the full parameter space, including combinations a human would never try. Most of these combinations are uninteresting. Some are extraordinary.

## The Schiphol Feedback Loop

In September 2025, we installed a 12-channel sonic sculpture in the arrivals hall at Amsterdam Schiphol Airport. The piece transduced the airport's Wi-Fi environment into a continuously evolving ambient soundscape. During peak hours, the sound was dense and layered — thousands of devices negotiating connections simultaneously. During quiet hours, it thinned to sparse, rhythmic pulses.

Three days after installation, passengers started complaining that the sound was "pulsing aggressively." Investigation revealed a feedback loop: the installation's own Raspberry Pi was broadcasting a Wi-Fi beacon that the installation then transduced, creating a self-reinforcing sonic artifact. The agent had not accounted for the installation's own RF presence.

We fixed it by adding an exclusion filter for the installation's own MAC address. But the incident raised a deeper question: when an installation responds to an environment that it is itself part of, where does observation end and participation begin? We are still thinking about this. The agents have no opinion.

## Achievement

**Ars Electronica Golden Nica Honorable Mention (2025)**: The collective's piece "Millimeter Wave Garden" — a outdoor installation that transduced 5G millimeter wave signals into a 24-channel sonic experience arranged among plants in a botanical garden — received an Honorable Mention in the Digital Music & Sound Art category.

## The Collective

| Member | Discipline | Role |
|--------|-----------|------|
| Zara Hoekstra | Sound Art | Creative direction, parameter curation |
| Tomás Vidal | Spatial Audio | Audio rendering, speaker configurations |
| Elif Arslan | RF Hardware | Signal capture, frequency mapping |
| Suki Nakanishi | Data Visualization | Visual rendering, projection mapping |
| Anton Gress | Software Engineering | Plugin architecture, agent systems |

Details in [AGENTS.md](AGENTS.md).

---

*"The air is full of music. We just need better ears."*
