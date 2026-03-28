# ZoneMap

**"Ship zoning. Ship faster."**

---

## The Pitch

ZoneMap is a generative AI startup that produces zoning-compliant building proposals in under sixty seconds. You upload a parcel boundary, select a jurisdiction, and the system generates a 3D massing model that fits within the zoning envelope — setbacks, height limits, FAR, parking minimums, the whole regulatory stack. Architects use it to explore what is buildable before they start designing. Developers use it to evaluate sites before they make offers. Municipal planners use it to test "what if we changed the setback?" scenarios during code rewrites.

Founded in 2023 in Austin, Texas, by three former Sidewalk Labs engineers who watched their employer's grand urban tech vision implode and decided to build something smaller, cheaper, and actually shippable. The founding insight was that zoning codes are formal grammars — they specify what is allowed through a set of rules that can be parsed, compiled, and executed computationally. Nobody had built a good compiler for them.

We raised a $3.2M seed round in January 2024 from Urban Innovation Fund and a handful of angels who had personally experienced the pain of a zoning denial. We are now at 14 employees, 1,400 paying customers, and a burn rate that our CFO describes as "sustainable if we close Series A by September."

## How We Got Here

Our AI agents emerged from a customer support crisis. In mid-2024, we expanded from Texas to California and discovered that California zoning codes are not just complex — they are internally contradictory. A single parcel in San Francisco can be subject to the Planning Code, the Building Code, the Housing Element, a specific area plan, a design review overlay, and a conditional use authorization, and these documents sometimes disagree with each other.

We built a multi-agent system where each agent specialized in one regulatory layer. The Planning Code agent knows setbacks and FAR. The Housing Element agent knows density bonus rules. The Overlay agent knows design guidelines. They each generate partial massing constraints, and a coordination agent merges them into a single compliant envelope.

The coordination agent needed version control. Each regulatory agent produced constraints as structured data (JSON patches to a shared massing model), and these patches conflicted when regulatory layers disagreed. We needed a system that could hold multiple conflicting patches simultaneously and let a human (or a more authoritative agent) resolve them. GitButler's virtual branches were exactly this.

## Team

Fourteen employees total; five working on this proposal. We move fast, we ship often, and we break things that are not in production.

| Agent | Role | Specialty |
|-------|------|-----------|
| Maya Chen | CEO / Orchestration | Product vision, task decomposition |
| Diego Reyes | CTO / Patch Generation | Zoning compiler, constraint solving |
| Aisha Osman | Forge Lead / PR Coordination | API integrations, customer-facing features |
| Tyler Park | ML Engineer / Memory & Provider | Model infrastructure, token optimization |
| Kenji Watanabe | Security Lead / Signing | Auth, compliance, key management |

Profiles in [AGENTS.md](AGENTS.md).

## Tension

**The Accuracy vs. Speed Debate.** Diego wants every generated massing model to be provably compliant — mathematically guaranteed to satisfy every applicable zoning constraint. Maya wants generation to be fast enough for a demo, which means approximations. Their compromise is a two-tier system: "fast mode" generates an approximate model in 5 seconds with a compliance confidence score, "precise mode" generates a provably compliant model in 60 seconds with a formal verification step. Customers overwhelmingly use fast mode. Diego considers this a moral failing. Maya considers it product-market fit.

## Achievement

In Q3 2024, ZoneMap's zoning compliance engine was licensed by the City of Austin's Planning Department for internal use. It was the first time a US municipal government licensed an AI-based zoning tool. The deal was small ($48K/year) but symbolically enormous — it validated the premise that zoning codes are compilable and that computation can replace the current system of "submit your plans and wait three months for a reviewer to tell you the setback is wrong."

---

*"Zoning is a compiler error. We are the debugger."*
— Company tagline, revised four times
