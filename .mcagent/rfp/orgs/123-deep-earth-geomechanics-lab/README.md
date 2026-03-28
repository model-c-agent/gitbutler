# The Deep Earth Geomechanics Lab

**"The rock remembers every stress it has ever felt. Our models should too."**

---

## Origin

The Deep Earth Geomechanics Lab operates out of the basement of the Earth Sciences building at the University of the Witwatersrand in Johannesburg — an appropriate location for a group that studies what happens to rock under the pressures found three kilometers below the surface.

The lab was founded in 2015 by Professor Naledi Mokoena, a geomechanics researcher who had spent fifteen years studying rock bursts in South Africa's deep gold mines. Rock bursts — sudden, violent fractures in highly stressed rock — are the most dangerous phenomenon in deep mining. They kill miners and destroy equipment without warning. Predicting them requires understanding stress distributions in rock at a molecular level, which requires computational models that no single researcher can build alone.

Professor Mokoena assembled a team of five researchers spanning geomechanics, computational physics, numerical methods, and machine learning. Their research program models rock stress using finite element methods at an unprecedented resolution: individual mineral grains within the rock matrix, each with its own stress-strain relationship, interacting across millions of grain boundaries. The models are computationally expensive — a single simulation of a 10-meter rock face takes 72 hours on the university's HPC cluster — but they predict micro-fracture propagation patterns that lower-resolution models miss entirely.

In 2023, the lab began using AI agents to manage their simulation pipelines. The simulations require complex workflows: mesh generation, boundary condition specification, solver configuration, result extraction, and visualization. Each step produces artifacts that the next step consumes. The lab built agents to manage this pipeline, using Git to track simulation configurations and results.

## Philosophy

**High fidelity at every scale.** Low-resolution models are fast but miss the grain-boundary interactions that cause rock bursts. High-resolution models are slow but accurate. The lab always chooses accuracy, then works to make the accurate model faster. Never the reverse.

They apply this to AI agents: an agent should produce the most accurate output possible within its budget, not the fastest output. Speed is a secondary concern. Correctness at the grain level — the individual line of code — is primary.

## The Tension

Dr. Sipho Ndlovu (numerical methods) and Dr. Amara Diagne (machine learning) disagree about model interpretability in agent decisions. Ndlovu builds deterministic numerical models where every output can be traced to specific input conditions. He wants the same traceability for AI agents. Diagne argues that neural networks are inherently opaque and that demanding full interpretability limits what agents can do: "You don't demand that a seismometer explain why the needle moved. You read the output." Ndlovu's counter: "A seismometer obeys known physics. A neural network obeys learned statistics. I trust the physics."

## Notable Achievement

In 2025, the lab's molecular-scale rock stress model predicted a rock burst event in a deep gold mine 48 hours before it occurred. The prediction was specific enough to identify the section of tunnel at risk and the approximate time window. The mine evacuated the section. The rock burst destroyed the tunnel face 36 hours later. No injuries. The prediction saved an estimated twelve lives, based on the shift schedule for that section.

## Team

Five researchers. Professor Mokoena leads. Academic consensus for research directions; Professor Mokoena decides funding allocation and publication priorities.

| Agent | Role | Focus |
|-------|------|-------|
| Prof. Naledi Mokoena | PI / Architecture | Research direction, system architecture |
| Dr. Sipho Ndlovu | Numerical Methods | Patch generation, deterministic validation |
| Dr. Amara Diagne | Machine Learning | Memory systems, pattern recognition |
| Dr. Yuki Tanaka | Computational Physics | Provider abstraction, simulation pipeline |
| Dr. Gabriel Santos | Visualization & Comms | Cross-repo coordination, result communication |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The lab operates on academic rhythms: research sprints between teaching obligations, conference deadline-driven intensity, and summer slowdowns. Code reviews happen at weekly lab meetings — whiteboards, projectors, and vigorous argument. Professor Mokoena insists on understanding every line of code that enters the production pipeline, which slows development but has prevented three bugs that would have invalidated months of simulation results.

---

*"Resolution is truth."*
— Prof. Mokoena, opening every conference talk
