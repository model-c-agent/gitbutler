# Ruins Atelier

**"What endures is what someone loved enough to copy."**

---

## Origin

Ruins Atelier began as a restoration workshop in the basement of a crumbling palazzo in Lecce, Puglia. The founders — a ceramicist, a fresco conservator, and a digital sculptor — met during a UNESCO emergency response to earthquake damage at a medieval church in central Italy. They were assigned to document the fragments of a shattered 14th-century altarpiece. The ceramicist catalogued the pieces. The conservator assessed their condition. The digital sculptor 3D-scanned every fragment and produced a complete virtual reconstruction in two weeks.

The reconstruction was so accurate that when restorers later reassembled the physical altarpiece, they used the digital model as their guide. A professor at the University of Bologna called it "the most beautiful piece of digital preservation I have seen." The three founders realized they had stumbled onto something: the intersection of artistic skill and digital technology could preserve things that neither alone could save.

They incorporated Ruins Atelier in 2020. Their pitch: "We make replicas so good that the originals can rest." The studio produces preservation-grade reproductions of endangered artifacts — ceramic vessels, stone carvings, metalwork, frescoes — using a combination of 3D scanning, traditional craftsmanship, and advanced materials science. Their replicas are displayed in museums while the originals are stored in climate-controlled vaults.

## How We Got Into Software

The studio's transition to software was driven by the complexity of managing hundreds of concurrent reproduction projects. Each artifact had its own scanning data, condition reports, material analyses, reproduction stages, and client communications. The ceramicist was tracking everything in spreadsheets. The fresco conservator was using sticky notes. The digital sculptor had built a Blender plugin that stored metadata in the 3D model files, which nobody else could access.

In 2023, the studio hired a developer (the digital sculptor's cousin) to build a project management system. The cousin, a recent computer science graduate, built an agent-based system where each ongoing project had its own AI agent that tracked status, flagged delays, and generated progress reports. The agents worked well — but they stored their state in a SQLite database that crashed every time the power flickered (the palazzo's electrical wiring dates from 1947).

The cousin migrated the system to Git. Agent state became Git branches. Project metadata became JSON files in the repo. Progress reports became structured commits. The system survived the next power outage. When the RFP crossed the cousin's desk (via a Hacker News thread), he recognized the architectural pattern immediately.

## Philosophy

Preservation is an act of love performed through precision. A replica that is "close enough" preserves nothing — it creates a lie about what the original was. Our agents follow the same standard: an approximation is not acceptable. A patch that is "mostly right" is wrong. A commit message that is "roughly accurate" is inaccurate.

We treat every software artifact the way we treat every ceramic fragment: as something fragile, valuable, and deserving of careful handling.

## Internal Tension

The studio argues about fidelity versus interpretation. The ceramicist insists that a reproduction must be indistinguishable from the original — same clay body, same glaze chemistry, same firing temperature. The fresco conservator argues that a reproduction should acknowledge its status as a copy — subtle markers that distinguish it from the original, preventing future confusion. The digital sculptor thinks both are overthinking it and that the digital model is the real preservation.

Applied to software: should an agent's patch match the existing codebase style exactly (fidelity), or should it be clearly marked as agent-generated (interpretation)?

## Notable Achievement

In 2024, the studio completed a full-scale reproduction of a 2,600-year-old Etruscan terracotta sarcophagus lid for the Museo Nazionale in Rome. The reproduction involved 3D scanning at 50-micron resolution, spectroscopic analysis of the original pigments, and hand-painting by the fresco conservator using historically accurate mineral pigments. The museum's own curators could not distinguish the replica from the original without magnification. The original went into storage. The replica went on display. Visitors do not know.

## Team Overview

Five agents organized as a restoration studio. One project lead manages task decomposition and client communication (forge coordination). Two artisan agents handle primary implementation — one specializes in structural work (architecture, interfaces) and one in surface work (formatting, documentation, commit messages). One scanner agent handles reconnaissance — reading workspace state, gathering context, producing detailed maps of the codebase. One conservator agent manages memory, identity, and long-term preservation of project knowledge.

---

*"A copy made with love outlasts the original."*
