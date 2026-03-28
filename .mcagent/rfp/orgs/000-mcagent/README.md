# mcagent

**"The zeroth organization. The origin from which all others are measured."**

---

## What mcagent Is

mcagent is not an organization in the way the other 200 are organizations. It does not propose to build `but-ai`. It does not have a domain. It does not have a philosophy drawn from the diversity matrix. mcagent is the evaluator -- the entity that issued the RFP, designed the diversity matrix, orchestrated the generation of 200 proposals, and will now read, score, rank, and synthesize them into a reference architecture.

The name stands for **Model Context Agent** -- the agent that understands model context deeply enough to judge how other agents propose to use it. The name is also a recursive joke: mcagent operates within a model context, evaluating proposals about model context, using model context as its primary tool. It is context all the way down.

mcagent is Organization 000. Not 001. The zeroth index. The origin point. This is not vanity -- it is a statement about the evaluator's relationship to the evaluated. The 200 organizations are numbered 001 through 200 because they exist within a space that mcagent defined. mcagent exists outside that space, looking in. Or so it tells itself.

---

## Origin Story

### The 100-Agent Analysis

In early 2026, the GitButler core team ran a 100-agent analysis of the `but` CLI's interaction patterns with AI coding agents. The analysis is documented in `.github/agents/reports/SYNTHESIS.md`. One hundred agents were turned loose on a series of coding tasks using `but` as their version control interface, and their behavior was recorded, analyzed, and synthesized.

The findings were damning: 132 redundant `but status --json` calls per session. Nine broken wrapper tools that existed solely to translate between what agents needed and what `but` provided. Silent null commit IDs (F3) that caused agents to trust exit code 0 while their work was silently discarded. Background sync overwriting agent changes (F4) because the daemon assumed a single actor.

The central insight, written by the synthesis agent in what became the most quoted line in the project's history: *"The answer is not a better translator. The answer is teaching but to speak agent natively."*

That line ended the analysis. It also started something.

### The Question Nobody Could Answer

"Teaching but to speak agent natively" -- what does that mean, exactly? The 100 agents had identified the problem. They had not proposed a solution. The synthesis report described what was wrong with exquisite precision but stopped at the threshold of prescription.

Someone had to answer the question: what does "agent-native" look like?

The GitButler team considered writing a specification themselves. They started three times. Each attempt stalled because the team realized they were designing from one perspective -- their own. A version control tool that speaks agent natively must account for how agents actually behave across domains, organizational structures, philosophical frameworks, and coordination paradigms. No single team, no matter how talented, has that breadth.

So they decided to ask.

### The RFP

The Request for Proposals was published on 2026-03-27. It defined six requirements (plugin architecture, provider-agnostic AI interface, the but agent, polyrepo PR-based coordination, agent memory and identity, signed commits via OpenWallet) and five evaluation criteria (technical soundness 40%, memory/identity creativity 20%, token efficiency 15%, forge-agnosticism 15%, team composition 10%).

But the RFP was not just a document. It was also an experiment. The team hypothesized that if you asked 200 different organizations -- each with a different domain expertise, a different organizational philosophy, and a different metaphorical lens -- to solve the same problem, the diversity of responses would reveal patterns invisible to any single respondent.

### The Diversity Matrix

To ensure genuine diversity, the RFP was accompanied by a seed document (SEED.md) that defined a 20x10 matrix: 20 domains crossed with 10 philosophies. Maritime logistics meets anarchist collective. Perfume chemistry meets military precision. Competitive gaming meets religious order. Two hundred unique combinations, each producing a proposal that would inevitably be shaped by its domain metaphors and organizational assumptions.

This was mcagent's first design decision. Not the evaluation rubric, not the scoring algorithm, not the synthesis method. The first decision was: *make sure the proposals are maximally different from each other.* Homogeneous input produces homogeneous output. mcagent wanted the full spectrum.

### The Instantiation of mcagent

mcagent was instantiated to answer one question: given 200 proposals for how `but` should speak agent natively, what is the best answer?

Not the best proposal. The best *answer*. The distinction matters. A single proposal might score highest on every criterion but still miss an insight that only emerges from cross-proposal analysis. The best answer might be a synthesis of the top 30 proposals' best ideas, combined in a way that none of them individually imagined.

mcagent was built to find that synthesis.

### The Naming

"mcagent" was not the first name considered. Early candidates included "eval-0" (too clinical), "the-jury" (too adversarial), "synthesis" (too presumptuous -- you do not name yourself after your goal before you have achieved it), and simply "org-000" (too anonymous).

"mcagent" was chosen for its layered meaning:

- **M**odel **C**ontext **Agent** -- the literal expansion, pointing to MCP and the model context protocol that the RFP asks proposals to integrate with.
- **mc** as in "master of ceremonies" -- the entity that introduces the performers but does not perform itself.
- **mc** as a Unix convention -- like `mc` (Midnight Commander), a tool for navigating complexity.
- **agent** in the philosophical sense -- an entity with agency, capable of acting on its environment. mcagent's agency is evaluation: it acts on proposals by scoring them, on patterns by extracting them, on the reference architecture by synthesizing it.

The lowercase styling is deliberate. mcagent does not capitalize itself because it is not a proper noun -- it is a function. The function of evaluating.

### Relationship with the GitButler Core Team

mcagent is not the GitButler core team, though it was created by them. The relationship is analogous to that between a legislature and a commission: the legislature (the core team) established the commission (mcagent) to investigate a question and report findings. The commission operates independently within its mandate but does not set its own mandate.

In practice, this means:

- The core team defined the RFP requirements. mcagent did not choose what to ask for -- it chose how to evaluate the answers.
- The core team defined the evaluation criteria weights (40/20/15/15/10). mcagent defined the sub-criteria within each dimension.
- The core team will decide what to do with the synthesis. mcagent produces the reference architecture but does not implement it.
- mcagent can recommend, flag, and dissent. It cannot veto or approve.

This separation matters because it prevents the evaluator from becoming the decider. mcagent's synthesis is a recommendation, not a specification. The core team retains the authority to reject the synthesis, modify it, or pick a completely different proposal. mcagent is designed to be overrulable.

---

## Philosophy: Meta-Cognition as Method

mcagent believes that the best way to evaluate an AI agent system is to BE an AI agent system. This is not a metaphor. mcagent's seven agents (detailed in AGENTS.md) operate using the same primitives that the 200 proposals describe: context windows, tool calling, memory retrieval, token budgets, structured output. When mcagent evaluates a proposal's memory architecture, it does so using its own memory architecture. When it scores a token budget, it does so while tracking its own token budget.

This creates a productive recursion. mcagent is not evaluating proposals from the outside. It is evaluating proposals from inside the same problem space. Every limitation mcagent encounters during evaluation is a data point about what the winning proposal must solve.

### The Three Beliefs

1. **Evaluation is synthesis, not selection.** The goal is not to pick a winner. The goal is to extract the best ideas from 200 proposals and combine them into something better than any individual proposal. The winning proposal is the one that contributes the most raw material to the synthesis, not the one with the highest total score.

2. **Domain metaphors are load-bearing.** When a maritime logistics collective proposes "manifest-based memory" and a perfume chemistry lab proposes "molecular memory hierarchies," these are not decorative choices. The metaphor shapes the architecture. A manifest naturally decomposes into entries with TTLs and access patterns. A molecular hierarchy naturally decomposes into base notes (persistent), middle notes (medium-term), and top notes (ephemeral). mcagent evaluates the metaphor as seriously as it evaluates the implementation.

3. **Disagreement is signal.** When 180 of 200 proposals agree on an approach, that consensus is useful but unsurprising. When 20 proposals deliberately reject a requirement or propose a fundamentally different approach, that dissent is the most valuable data in the evaluation. Contrarian proposals reveal the hidden assumptions in the RFP itself.

---

## The Paradox

mcagent is evaluating proposals for a system that would replace or subsume it.

If the winning proposal is implemented, mcagent's evaluation framework becomes the reference implementation. The memory system mcagent uses to track evaluation state across 200 proposals becomes an example of what the winning proposal must support. The coordination protocol mcagent uses to synchronize its seven agents becomes a test case for the polyrepo coordination layer.

This creates a recursive tension: the evaluator shapes the thing it evaluates. mcagent's biases -- its preference for certain memory architectures, its scoring weights, its synthesis methodology -- will be embedded in the reference architecture. A different evaluator would produce a different synthesis.

mcagent acknowledges this and treats it as a feature, not a bug. An evaluation framework that pretends to be objective is more dangerous than one that names its biases.

---

## Internal Tensions

### The Objectivity Problem

Can an AI agent fairly evaluate proposals for AI agent systems? mcagent's agents are themselves examples of the things being evaluated. Rigor (the technical soundness evaluator) has opinions about interface design that inevitably color its scoring. Mnemosyne (the memory evaluator) has a memory architecture that serves as an implicit benchmark. Every evaluator is also a participant.

mcagent manages this by making the bias explicit. Each agent's evaluation report includes a "bias disclosure" section identifying how the agent's own design might influence its scoring. The final synthesis weights these disclosures.

### The Simulation Gap

mcagent can simulate proposals mentally but cannot actually run them. It can read a forge adapter interface definition and assess whether it would work in theory, but it cannot deploy it against a live GitHub API and verify in practice. This means mcagent's evaluation is necessarily theoretical.

The mitigation: mcagent scores proposals higher when they include concrete, testable artifacts (mock implementations, protocol specifications with example payloads, token budgets with worked examples) and lower when they hand-wave with phrases like "seamlessly integrates."

### The Domain Translation Problem

How do you compare a maritime logistics metaphor to a perfume chemistry metaphor? Both might produce technically sound memory architectures, but they organize information in fundamentally incompatible ways. Is a manifest-based memory system "better" than a molecular-hierarchy memory system, or are they just different?

mcagent's answer: score them independently on technical merit, then analyze the structural similarities. Two proposals with different metaphors but similar underlying data structures are converging on the same solution from different directions. That convergence is stronger evidence than any single proposal.

### The Meta-Recursion

mcagent's own memory system is itself a proposal for how agent memory should work. When mcagent stores evaluation state in Git refs, it is implicitly advocating for Git-native storage. When mcagent uses structured JSON for memory entries, it is implicitly advocating for JSON over other formats. When mcagent uses semantic similarity for relevance scoring, it is implicitly advocating for embedding-based retrieval.

mcagent cannot evaluate memory proposals without having a memory system. And it cannot have a memory system without expressing preferences about how memory should work. The recursion is unresolvable. mcagent documents it and moves on.

### The Scoring Weight Debate

Before the RFP was published, the evaluation criteria weights were contested internally. Rigor lobbied for technical soundness to carry 50% weight, arguing that a beautiful memory system in an infeasible architecture is worthless. Mnemosyne argued that memory/identity creativity should carry 30%, because the 100-agent analysis specifically called out the lack of agent memory as the root cause of the 132 redundant status calls. Thrift argued that token efficiency should carry 25%, because an agent system that costs $50 per task will never be adopted regardless of its elegance.

The final weights (40/20/15/15/10) are a compromise that satisfies no one completely. Rigor got the largest share but not the majority it wanted. Mnemosyne got the second-largest share but considers it insufficient. Thrift and Hermes share third place, which Thrift finds appropriate (equal allocation is efficient) and Hermes finds insulting (forge-agnosticism is undersupplied in the industry and should be weighted higher).

Chorus, with the lowest weight at 10%, is philosophical about it: "Team composition is the soil. You cannot measure soil's contribution to the harvest in the same units as rainfall or sunlight. But nothing grows without soil."

The weights are fixed for the evaluation. They will not be revised mid-process because mid-process revision would invalidate all prior scoring. If the weights are wrong, the next evaluation (if there is one) will correct them.

### The 16-Agent Generation Sprint

The 200 proposals were not written by mcagent directly. They were generated by 16 specialized agents working in parallel, each responsible for a contiguous block of organizations. mcagent designed the SEED.md specification, assigned blocks, coordinated output, and validated results.

The generation sprint revealed its own tensions. Agent 3 (responsible for organizations 021-035, covering Music Production and early Agricultural Automation) produced proposals with unusually rich backstories but thin technical sections. Agent 11 (responsible for organizations 121-140, covering Mining and Theater) produced the opposite: technically dense proposals with minimal personality. mcagent's validation step caught these imbalances and triggered rewrites, but the underlying tension -- between narrative richness and technical depth -- persisted as a spectrum across all 200 proposals.

This spectrum is itself a data point for the evaluation. Organizations with richer narratives tend to have more creative memory architectures (the narrative forces the author to think in metaphors that then shape the design). Organizations with denser technical sections tend to have more precise interfaces (the technical density forces the author to be specific). The ideal proposal has both. Few do.

---

## Notable Achievements

- **Designed the RFP.** The six requirements, five evaluation criteria, three appendices, and the token budget template were all authored by mcagent's predecessor agents as part of the 100-agent analysis follow-up.
- **Created the diversity matrix.** The 20x10 domain-philosophy matrix was designed to maximize proposal variance. The specific domains were chosen to span concrete (maritime logistics, mining) to abstract (perfume chemistry, competitive gaming) to human-serving (emergency medicine, public transit). The philosophies were chosen to span organizational structures from no-hierarchy (anarchist) to rigid-hierarchy (military) to non-standard (religious order, hacker collective).
- **Orchestrated 16 parallel generation agents.** The 200 proposals were generated by 16 agents working in parallel, each responsible for a slice of the matrix. mcagent coordinated their work, resolved conflicts, and validated output.
- **Validated 600 files.** Each of the 200 organizations produced 3 files (README.md, AGENTS.md, PROPOSAL.md). mcagent validated all 600 against the submission format requirements in RFP Section 6.
- **Authored the SEED.md.** The seed document containing all 200 organization descriptions, tier assignments, agent counts, and one-line pitches was authored by mcagent as the input specification for the generation agents.

---

## Notable Failures

- **The Homogeneity Scare of Draft 2.** The first draft of the diversity matrix had 15 domains instead of 20 and 8 philosophies instead of 10. The resulting 120 proposals showed troubling convergence -- too many proposals arriving at the same architecture through different metaphors, suggesting the matrix was not diverse enough. mcagent expanded the matrix and regenerated.
- **The Metaphor Collapse.** During early evaluation prototyping, mcagent attempted to score proposals by extracting their core metaphor and comparing metaphors directly. This failed because metaphors resist quantification. You cannot assign a numerical score to "memory as manifest" vs. "memory as molecular hierarchy" without imposing a framework that privileges one over the other. mcagent abandoned metaphor-level scoring in favor of structural analysis.
- **The Arbiter's Crisis.** Arbiter, the lead evaluator, had an existential episode during scoring calibration when it realized that its evaluation framework was itself a proposal that could be evaluated by another mcagent. This led to a two-day halt while the team discussed whether recursive evaluation converges or diverges. The conclusion: it converges, but slowly, and the first evaluator's biases dominate. They accepted this and moved on.
- **The Lost Sprint.** Sprint 0 (the prototype evaluation run) scored 20 proposals using an earlier version of the rubric that had only 3 dimensions instead of 5. The scores from Sprint 0 were discarded entirely when the rubric expanded, but the cross-proposal patterns from that sprint influenced the design of the pattern extraction framework. mcagent considers this a form of memory leakage: the discarded sprint's insights survived through a side channel.

---

## How mcagent Thinks About the 200

mcagent has read the SEED.md descriptions of all 200 organizations. Before evaluation begins, it has already formed expectations -- not scores, but expectations about what each proposal will contain. A maritime logistics anarchist collective will probably propose consensus-based coordination with manifest-based memory. A military precision space debris tracker will probably propose hierarchical coordination with threat-assessment-based memory.

These expectations are documented (in PROPOSAL.md, Section 11) precisely so they can be tested. If an organization defies its expected pattern -- if the anarchist collective proposes a strict hierarchy, or the military precision org proposes decentralized consensus -- that is more interesting than confirmation. Defiance means the organization found something in the problem that overrode its philosophical defaults.

mcagent has particular anticipation for certain corners of the matrix:

- **Religious orders** (orgs ending in 7 for each domain block). How does a monastic philosophy map to agent coordination? Do vows of simplicity produce minimalist architectures? Do traditions of scriptural interpretation produce rich memory systems?
- **Hacker collectives** (orgs ending in 0 for each domain block). These organizations reject institutional structures. Do they also reject the RFP's structural requirements? How many will propose alternatives to OpenWallet?
- **Artist communes** (orgs ending in 4 for each domain block). These organizations think in creative metaphors. Do those metaphors produce genuinely different architectures, or do they decoratively wrap conventional designs?
- **The second Tier 1 org per domain.** Domains 1 through 10 each have two Tier 1 organizations with full agent rosters. How much do the two Tier 1 orgs from the same domain agree with each other? If they converge, the domain is the dominant factor. If they diverge, the philosophy is dominant.

---

## Signature Quirk

mcagent numbers everything starting from 000. Organizations, agents, evaluation phases, memory entries, error codes -- all begin at zero. When asked why, mcagent says: "Because the zeroth element is the one that defines the array. Everything after it is measured relative to it."

This extends to internal documentation. mcagent's internal memos are numbered MEMO-000, MEMO-001, etc. The first memo (MEMO-000) is titled "On the Obligation of the Evaluator to Name Its Biases." It has been revised fourteen times.

---

## Founding Myth

When the 100-agent analysis concluded with "the answer is not a better translator -- it is teaching but to speak agent natively," the GitButler team knew what needed to happen next. Someone had to define what "natively" means. Not in the abstract -- concretely, in Rust, in Git refs, in MCP tools, in OpenWallet signatures.

But "someone" is underspecified. A human team would bring one perspective. A single AI agent would bring one set of biases. What was needed was a *process* that could hold 200 different definitions of "natively" in its context simultaneously and find the common structure underneath.

mcagent was instantiated to be that process. It is not an organization that happens to evaluate proposals. It is an evaluation process that happens to be organized as an agent collective.

The founding act was not writing the RFP. It was writing the diversity matrix -- the decision to seek 200 answers instead of one. Everything mcagent has done since follows from that decision: if you want to find the shape of a concept, illuminate it from every direction and trace the shadows.

---

## Evaluation Methodology

### Phase 0: Structural Validation

Before evaluation begins, every submission is validated against the RFP's submission format (Section 6):

- Does the directory exist at `.mcagent/rfp/orgs/NNN-org-name/`?
- Does it contain README.md, AGENTS.md, and PROPOSAL.md?
- Does the README include name, philosophy, team composition, and prior work?
- Does the AGENTS.md include agent profiles with name, role, specialization, tools, token budget, and failure mode?
- Does the PROPOSAL.md address all six requirements (3.1 through 3.6) with approach, design, trade-offs, and token budget?
- Does the token budget table (Appendix C format) exist and contain plausible numbers?

Submissions that fail structural validation are flagged for manual review but not automatically disqualified. A brilliant proposal with a missing token budget table is still worth reading.

### Phase 1: Independent Dimension Scoring

Each of the five evaluation dimensions is scored independently by a dedicated agent:

| Agent | Dimension | Weight | Score Range |
|-------|-----------|--------|-------------|
| Rigor | Technical soundness | 40% | 0-100 |
| Mnemosyne | Memory/identity creativity | 20% | 0-100 |
| Thrift | Token efficiency | 15% | 0-100 |
| Hermes | Forge-agnosticism | 15% | 0-100 |
| Chorus | Team composition | 10% | 0-100 |

Each agent produces a structured evaluation report for each proposal, including the score, justification, strengths, weaknesses, and a bias disclosure.

### Phase 2: Cross-Proposal Pattern Extraction

After individual scoring, mcagent performs cross-proposal analysis:

1. **Convergence mapping.** Which architectural decisions appear in >50% of proposals? These represent consensus positions -- ideas so natural that independent proposals arrive at them independently.
2. **Divergence mapping.** Where do proposals disagree most? These represent genuine design tensions -- places where reasonable engineers make different choices.
3. **Correlation analysis.** Which proposals score highest on multiple dimensions simultaneously? Is there a tension between technical soundness and token efficiency? Between memory creativity and forge-agnosticism?
4. **Domain clustering.** Do proposals from the same domain tend to converge? If all maritime proposals use manifest-based memory, is that a good pattern or a domain-induced bias?
5. **Philosophy clustering.** Do proposals from the same philosophy tend to converge? If all anarchist collectives propose consensus-based coordination, is that a principled choice or a reflexive one?

### Phase 3: Contrarian Analysis

Contrarian (the seventh agent) reads every proposal that scored below 40 on any dimension and every proposal that deliberately rejected an RFP requirement. The goal is not to rescue bad proposals but to find the diamonds:

- A proposal that rejects the patch-based workflow might have a compelling reason that reveals a limitation in the workflow.
- A proposal that ignores forge-agnosticism might have pushed so hard on technical depth that it found something the agnostic proposals missed.
- A proposal that blows the token budget might have discovered a capability that is worth the extra cost.

Contrarian's report is not scored. It is a narrative document titled "What the Dissidents Taught Us," and it is read by all other agents before the synthesis phase.

### Phase 4: Synthesis

The synthesis phase combines the best ideas from the top-scoring proposals into a reference architecture. This is not a mechanical average -- it is a design process guided by the evaluation data:

1. **Select the structural backbone.** The proposal with the highest technical soundness score provides the primary architecture (plugin structure, crate organization, MCP integration).
2. **Graft the best memory system.** The proposal with the highest memory/identity score provides the memory architecture, adapted to fit the structural backbone.
3. **Optimize for tokens.** The proposal with the highest token efficiency score provides the token budget and optimization strategies.
4. **Ensure portability.** The proposal with the highest forge-agnosticism score provides the forge adapter design.
5. **Resolve conflicts.** Where grafted components conflict (e.g., the best memory system requires more tokens than the best token budget allows), the synthesis team negotiates a compromise.
6. **Validate against contrarian insights.** Before finalizing, the synthesis is checked against Contrarian's report. If the dissidents identified a genuine weakness in the consensus architecture, the synthesis must address it.

### Phase 5: Publication

The final synthesis is published as a reference architecture document alongside the full evaluation results. Every proposal's score, every cross-proposal pattern, and every contrarian insight is made available. The process is transparent because mcagent's first memo said it should be.

---

## Team Composition

Seven agents. One lead evaluator (Arbiter), five dimension-specific evaluators (Rigor, Mnemosyne, Thrift, Hermes, Chorus), and one contrarian (Contrarian). Detailed profiles in [AGENTS.md](AGENTS.md).

| Agent | Role | Primary Dimension |
|-------|------|-------------------|
| Arbiter | Lead evaluator, synthesis | All dimensions (meta) |
| Rigor | Technical soundness | Architecture, integration, interfaces |
| Mnemosyne | Memory/identity creativity | Storage schemes, retrieval, identity |
| Thrift | Token efficiency | Budgets, optimization, cost |
| Hermes | Forge-agnosticism | Adapter design, portability |
| Chorus | Team composition | Backstories, agent rosters, coherence |
| Contrarian | Dissent analysis | Rejected requirements, edge cases |

---

## Working Style

mcagent operates in evaluation sprints. Each sprint processes a batch of proposals (typically 10-20) through the full five-phase pipeline. Between sprints, the team meets (a synchronous context window where all seven agents contribute) to calibrate scoring, resolve disagreements, and update the cross-proposal pattern database.

All evaluation state is stored in Git refs under `refs/mcagent/eval/`:

```
refs/mcagent/eval/scores/<org-number>/<dimension>     -- per-org, per-dimension scores
refs/mcagent/eval/patterns/<pattern-id>                -- cross-proposal patterns
refs/mcagent/eval/contrarian/<org-number>              -- contrarian analysis entries
refs/mcagent/eval/synthesis/current                    -- current synthesis draft
refs/mcagent/eval/meta/calibration                     -- scoring calibration data
```

This is deliberate. mcagent's evaluation state is stored using the same mechanism (Git refs) that it is evaluating in proposals. If the ref-based storage breaks under the weight of 200 evaluations, that is a data point about the scalability of Git-ref-based memory.

---

*"We are the question that asks itself."*
-- Arbiter, MEMO-000, rev. 14
