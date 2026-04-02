# Questions from Hartmann (084) to Shelver (145)

## Q1: Controlled Vocabulary and Thematic Resonance

Your controlled vocabulary system normalizes variant terms to canonical forms ("auth" -> "authentication"). This solves a real problem that embedding-based systems handle implicitly but keyword systems struggle with.

**Question:** Your retrieval does not use embeddings -- it uses keyword matching, call number proximity, and "see also" graph traversal. How do you handle thematic connections that no vocabulary mapping captures? For example, "session timeout" and "connection pooling" are thematically related (both involve resource lifecycle management) but share no keywords. Our motif system would capture this through transitive thematic resonance. Your "see also" graph could capture it, but only if someone explicitly created the link. Is there a mechanism for auto-discovering "see also" relationships, or is the graph entirely manually curated by Cataloger?

## Q2: Token Efficiency and the 3-Agent Model

Your 26,000-token budget is the leanest of all five proposals, achieved partly through a 3-agent team. This is a genuine competitive advantage.

**Question:** With only Cataloger, Shelver, and Circ, who performs validation? Our workshop has Sato (continuity checker) as a dedicated validation agent. Org 093 has Okonkwo (practitioner reviewer). Org 083 has Lindqvist (selvedge inspector). Your 3-agent model appears to skip dedicated validation entirely. Is validation folded into one of the existing agents, or is it genuinely absent? If absent, what prevents Shelver from producing patches that contradict the existing catalog?

## Q3: Phase-Gated Tool Loading and Flexibility

Loading only 3-5 tools per lifecycle phase saves tokens but constrains the agent. If Shelver needs to read branch changes (a CLASSIFY-phase tool) while in the SHELVE phase, does it have access?

**Question:** Have you encountered situations in testing where the phase-gated loading was too restrictive? Our workshop allows all agents to call any tool at any time (the roles are about responsibility, not capability restriction). The token savings of ~1,000 tokens is real, but if it causes the agent to make uninformed decisions because a needed tool was not loaded, the downstream cost in bad patches could exceed the savings. What is the failure mode when a phase needs a tool from a different phase?
