# Ebbesen Gard Teknik

**"Vi bygger det selv." (We build it ourselves.)**

---

## The Farm

Ebbesen Gard Teknik is a Danish family farm in Thy, northern Jutland, that has been automating its own operations for three generations. The farm covers 280 hectares of barley, rapeseed, and sugar beet. The Ebbesen family has worked this land since 1923. They have been building machines for it since 1971, when Niels Ebbesen Sr. welded the first automated grain conveyor from scrap metal and a salvaged motor.

The farm is not a technology company. It is a farm that builds technology because the alternatives — buying proprietary equipment from AGCO or John Deere and accepting vendor lock-in — are unacceptable to people who believe they should be able to fix what they own.

Today the farm is run by three generations:

| Name | Generation | Role |
|------|-----------|------|
| **Niels Ebbesen** (83) | 1st builder | Retired. Advises. Disapproves of software. |
| **Karen Ebbesen** (56) | 2nd builder | Mechanical and electrical. Runs the workshop. |
| **Lars Ebbesen** (31) | 3rd builder | Software and automation. Wrote this proposal. |
| **Sofie Ebbesen** (28) | 3rd builder | Embedded systems. Lars's sister. |

## What We Build

The farm's workshop — a large steel barn that smells of diesel and solder — contains a working inventory of custom-built automation:

- **Grain dryer controller** (Karen, 1998): Microcontroller-based moisture sensing and fan control. Still running on the original hardware.
- **GPS-guided seed drill** (Lars, 2019): Retrofitted Amazone drill with RTK-GPS and a Raspberry Pi controller. Accuracy: 2 cm.
- **Autonomous sprayer** (Sofie, 2022): Electric utility vehicle with computer vision-based weed detection. Reduces herbicide use by 40%.
- **Weather station network** (Lars, 2020): 12 solar-powered stations across the farm, transmitting via LoRa to a central server in the workshop.

All firmware is written in Rust (Lars) or C (Sofie). All hardware designs are documented in the workshop notebook — a physical notebook, leather-bound, maintained continuously since 1971. Karen insists on the notebook. Lars insists on the Git repository. Both are maintained.

## Why This RFP

Lars has been using AI coding assistants since 2024 to help with firmware development. He works alone on most software tasks (Sofie helps when she is not in the field) and the AI assistants accelerate his productivity significantly. But the workflow is messy: AI-generated changes are mixed with his own, attribution is unclear, and there is no structured way to track which suggestions the AI made and which he chose to accept.

The `but-ai` plugin addresses this directly. Lars wants: clear attribution of AI-generated changes, structured memory of firmware patterns, and signed commits that distinguish human work from AI-assisted work.

## Internal Tension

The tension is generational. Niels Sr. believes the farm is over-automated and that machines should be simple enough for any farmer to repair with a wrench. Karen agrees in principle but builds increasingly complex electrical systems. Lars writes software that Niels cannot read and Karen cannot debug. Sofie bridges the gap — her embedded systems work is close enough to Karen's electrical work to be legible.

The specific disagreement for this proposal: Lars wants AI-generated firmware suggestions to be integrated into the codebase automatically (with review). Karen wants every AI suggestion to be printed, read, and manually typed in. They have compromised on: AI generates patches, Lars reviews on screen, Karen reviews a printed diff, and only approved changes are committed.

## Notable Achievement

In 2025, the farm's GPS-guided seed drill and autonomous sprayer operated together for the first time in a coordinated planting-and-treatment workflow. Lars wrote the coordination software in three weeks. The system planted 80 hectares and sprayed the same area within 48 hours, with the sprayer automatically adjusting its path based on the drill's planting map. Karen said it was "not bad." From Karen, this is equivalent to a standing ovation.

---

*Built in the workshop. Tested in the field. Three generations of making do.*
