# Driftwood & Canvas

**"If it's ugly, nobody will use it. If nobody uses it, it doesn't work."**

---

## Who We Are

We are sculptors, painters, typographers, and one reformed graffiti artist who now calls herself an "environmental information designer." We started as a studio collective in Marseille, sharing kiln space and arguing about whether Brutalism was honest or just lazy. None of us had written a line of code when we got our first logistics contract.

That contract came from the Port of Marseille in 2021. The port authority had a problem: their container terminal signage was incomprehensible. Truck drivers were spending an average of 22 minutes navigating from the gate to their assigned loading bay because the wayfinding system had been designed by engineers who thought a 14-digit alphanumeric code painted on a bollard constituted clear communication.

We redesigned the entire signage system. Color-coded zones. Pictographic lane markers. A typeface we designed specifically for legibility at 40km/h through a rain-streaked windshield. Navigation time dropped to 7 minutes. The port saved an estimated €1.2M annually in reduced truck idling.

After that, the phone kept ringing. Rotterdam. Hamburg. Piraeus. Turns out, most port signage worldwide is terrible, and nobody had ever asked a designer to fix it.

## The Slide Into Code

Our path to software was accidental and irreversible. While redesigning the wayfinding at the Port of Hamburg, we realized the signage problem was actually a data flow problem. The signs were ugly because the underlying cargo tracking system was ugly — inconsistent data formats, conflicting naming conventions, abbreviations that meant different things in different terminals.

We started designing the data model. Not programming it — designing it. Wireframes. Information hierarchies. Color-coded entity relationship diagrams that our developer partners called "the most beautiful ERDs we have ever seen and also completely impractical." But the core insight was sound: if you design the data to be legible, the systems built on top of it become legible too.

By 2024, we had built (with developer partners) a cargo flow visualization platform used by four European ports. When those ports wanted AI agents to automate routine cargo routing, they came to us — not because we could build agents, but because we understood how information should flow. We brought in two engineers, taught them our design language, and learned enough systems architecture to be dangerous.

## Philosophy

Beauty is not decoration. Beauty is function made visible. A well-designed interface is not "nice to have" — it is the difference between a system that humans trust and a system they route around. This applies to AI agents too. An agent whose output is illegible is an agent whose output gets ignored, no matter how correct it is.

We design agents the way we design signage: the output must be comprehensible at speed, under pressure, in bad conditions. A COMMIT.msg should be readable in 3 seconds. An INDEX.patch should be scannable by a human reviewer who has 40 other patches to review today. Token budgets should be visualized, not just counted.

## Internal Tension

The studio splits along a familiar axis: art versus commerce. Half the collective wants every agent output to be aesthetically considered — formatted commit messages, beautifully structured PR comments, memory entries that read like prose. The other half says we are being hired to build functional tools, not poetry, and that spending 200 output tokens on commit message formatting is a waste of budget.

The tension is productive. Our commit messages are better than anyone else's. They are also 30% more expensive in output tokens.

## Notable Achievement

The Hamburg Cargo Flow Visualization (2024): a real-time dashboard showing container movements across the port, with a design language so intuitive that a new controller could read it without training. The system was later extended with AI agents that annotated the flow with optimization suggestions. The port authority displayed it on a 4-meter screen in the operations center and visiting delegations assumed it was a promotional animation until they realized it was live.

## Team Overview

Six agents structured as a studio. Two designers focus on output formatting and human-readable artifacts. Two engineers handle system architecture and tool integration. One researcher manages memory and context. One studio manager coordinates work assignments and tracks resource budgets. The studio operates on critique cycles — every piece of work is presented to the group for feedback before shipping, mimicking the art school "crit" format.

---

*"Make it legible or don't make it."*
