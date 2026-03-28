# Schola Cantorum Machina

**"Ora et labora et computa."**
*(Pray, work, and compute.)*

---

## The Order

The Schola Cantorum Machina is a community of twelve Benedictine oblates living in a converted telecommunications relay station outside Maastricht, Netherlands. They are not monks in the traditional sense -- oblates take vows of association rather than full monastic vows -- but they follow the Rule of Saint Benedict with adaptations for digital life. They rise at 4:30 AM for Lauds. They observe the Great Silence from Compline (9 PM) until Lauds. They produce sacred electronic music using synthesizer pipe organs of their own construction.

The community was founded in 2016 by Brother Anselm (born Marcus de Groot), a former Philips audio engineer who experienced a conversion during a particularly grueling product launch. He left the company, took oblate vows, and spent two years converting the relay station into a combined chapel-studio-datacenter. The chapel contains a pipe organ augmented with MIDI-controlled analog synthesizers -- each rank of pipes paired with a corresponding oscillator bank. The result is an instrument that produces both acoustic and electronic sound simultaneously, blended in the chapel's natural reverb.

The community's music is distributed freely under Creative Commons. They have released eleven albums of sacred electronic compositions. Their most popular work, *Vespers for Voltage*, has been streamed 2.3 million times. They do not track metrics. They learned this number from a journalist.

## The Turn to Software

In 2023, the community received a donation of server hardware from a retiring Dutch hosting company. Brother Anselm, whose engineering instincts had never fully yielded to monastic contemplation, proposed using the servers to run local AI models for composition assistance. The community debated this for three weeks during chapter meetings. The concern was not theological -- the brothers had no objection to AI in principle. The concern was about dependency: would relying on external AI providers create an attachment that conflicted with monastic simplicity?

The compromise was self-hosting. The community runs Ollama on their donated servers, using open-weight models exclusively. They do not use cloud AI providers. They do not send data outside the relay station. Their entire AI workflow -- from composition assistance to the code that controls their synthesizer organ -- runs on hardware they can see from the chapel window.

When the `but-ai` RFP appeared, Brother Matthias (the community's most technically adept member) argued that their self-hosted, privacy-first approach to AI was exactly what the plugin's provider-agnostic design was meant to support. The Abbot agreed. The community assigned four brothers to the proposal during their daily work period (9 AM to 12 PM, 2 PM to 5 PM).

## Philosophy

1. **Simplicity is not poverty.** A simple system is one with few dependencies, not one with few capabilities.
2. **Silence before speech.** An agent should observe before acting. The Great Silence is not absence -- it is attention.
3. **Community over individual.** No agent operates alone. Every action is witnessed by the community.

## Internal Tension

Brother Matthias and Brother Anselm disagree about the role of memory in agent systems. Anselm, drawing on Benedictine tradition, believes agents should practice detachment -- each task should be approached fresh, without the weight of accumulated memory. Matthias, the pragmatist, argues that memory is experience, and an agent without experience is an agent that repeats mistakes.

The compromise: short-lived memory with mandatory review. Memory entries expire after 48 hours unless a brother explicitly renews them during the daily chapter meeting. This ensures that only memories the community has consciously chosen to retain persist.

## Notable Achievement

In 2025, *Compline in C Minor* -- a 40-minute composition created collaboratively between three brothers and a locally-hosted AI model -- was performed at the Concertgebouw in Amsterdam. The performance used a portable version of their synthesizer organ and a live AI accompanist running on a laptop. The AI's contributions were signed with the community's GPG key. It was the first concert in the venue's history where an AI performer had cryptographic attribution.

---

*Composed during the work period. Reviewed during chapter. Approved by the Abbot.*
