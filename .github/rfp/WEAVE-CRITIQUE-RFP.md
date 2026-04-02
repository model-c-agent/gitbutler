# WEAVE Protocol Critique RFP: Bug Bounty for Bad Ideas

**Date**: 2026-03-29
**Status**: Open for Submissions
**Protocol Under Review**: WEAVE v0.1 (Workspace-Enabled Agent Version-control Exchange)
**Reference Documents**: PROTOCOL-SPEC-DRAFT.md, WORKTREE-INTEGRATION-ANALYSIS.md, PROTOCOL-ANALYSIS.md

---

## Preamble: Break This Protocol

We are inviting 20 teams to attack, challenge, and propose alternatives to the WEAVE protocol specification.

WEAVE was designed through an adversarial synthesis process: 200 organizations submitted proposals, 5 finalists built competing implementations, and a cross-evaluation panel merged the strongest ideas into a single specification. The result is a 7-layer protocol covering write primitives, CLI contracts, coordination, memory, identity, task orchestration, and worktree integration. It draws on ideas from actuarial science, library science, distributed systems, textile manufacturing, and maritime coordination.

It is also completely untested in production.

We designed WEAVE through competition. Now we stress-test it through competition. We are looking for teams that can find the flaws we missed, the assumptions we did not question, the edge cases we did not consider, and the alternatives we did not explore. **We want you to break this protocol.** If you find something broken, you get to fix it.

This is not an academic exercise. Findings from this process will directly shape WEAVE v0.2. Teams that identify real issues earn real influence over the protocol's future.

### Reward Tiers

| Tier | Definition | Reward |
|------|-----------|--------|
| **Critical Break** | The protocol is fundamentally flawed in a way that cannot be patched. The design assumption is wrong. | Team redesigns the broken section. Their alternative replaces the current spec text. |
| **Major Issue** | A significant gap, weakness, or missing capability that materially affects production use. | Team's proposed alternative is included as an official appendix in v0.2. |
| **Minor Issue** | An edge case, ambiguity, or incremental improvement that strengthens the spec without changing its structure. | Acknowledged in the v0.2 changelog with attribution. |
| **Validation** | Team thoroughly reviews a section and finds it sound. | Confirmation is recorded in the spec. Validated sections carry higher confidence. |

Every outcome is valuable. Finding nothing wrong in a section is itself a finding -- it means the synthesis process worked for that part of the protocol. But we suspect it did not work everywhere.

---

## The 20 Teams

Teams are organized into four categories, each with a distinct mandate.

---

### Category A: Adversarial -- Try to Break It

These teams exist to find failure modes. Their mandate is destruction, not construction. They succeed by demonstrating concrete scenarios where the protocol produces wrong, dangerous, or absurd results.

---

#### Team 1: Red Team Alpha
**Tagline**: "Forge the Unforged"

**Perspective**: Security researchers specializing in identity systems, cryptographic protocols, and privilege escalation. They assume every agent is potentially malicious and every trust boundary is a target.

**Target Sections**: Section 7 (Identity & Authorization), Section 3.3 (Commit Message Trailers), Section 6.1 (Storage Layout)

**Key Questions**:
1. Can a malicious agent forge commit trailers (`Agent-Id`, `Survival-Estimate`) to impersonate another agent or manipulate memory expiration?
2. The `NoOpSigner` signs everything with a deterministic hash. What happens if a production deployment accidentally ships with `NoOpSigner` instead of a real signer? Is there a runtime guard?
3. Authorization scopes use glob patterns (`feat/*`, `SEC.*`). Can a carefully crafted branch name or call number bypass glob matching? What about `feat/../main`?
4. Agent identity is stored at `refs/but-ai/memory/<agent-id>/identity/self`. Can one agent overwrite another agent's identity ref? What prevents namespace squatting?
5. The gossip protocol merges entries using last-writer-wins. Can an attacker inject high-access-count entries to poison another agent's memory?

**Success Criteria**: Demonstrate a concrete attack scenario with steps, inputs, and expected vs. actual behavior. Proof-of-concept code or pseudocode is strongly preferred.

**Reward Tier Mapping**:
- **Critical**: Identity forgery that allows full impersonation, or memory poisoning that survives gossip merge
- **Major**: Authorization bypass via glob edge case, or namespace squatting with no mitigation
- **Minor**: Missing input validation on a specific field, or theoretical attack requiring unlikely preconditions

---

#### Team 2: Chaos Engineers
**Tagline**: "Split the Brain"

**Perspective**: Distributed systems engineers who specialize in consensus protocols, network partitions, and race conditions. They know that "works on one machine" and "works in production" are entirely different claims.

**Target Sections**: Section 5.8 (Gossip Protocol), Section 5.7 (Dependency Resolution), Section 6.5 (Memory Lifecycle), Worktree Integration Analysis Section 4 (Memory Bridge)

**Key Questions**:
1. Two agents write the same ref (`refs/but-ai/coordination/messages/...`) at the same nanosecond. What happens? The spec says refs are "atomic" -- is that true under concurrent `gix` writers?
2. The gossip merge uses last-writer-wins with access_count as tiebreaker. Construct a scenario where two agents legitimately diverge and LWW produces the wrong result (both sides lose information).
3. The dependency DAG treats external dependencies as "already satisfied." What happens when an external PR is reverted after being marked merged? The DAG cannot detect this.
4. The worktree analysis claims "gossip is unnecessary for co-located agents" because refs are immediately visible. Is this true under `packed-refs`? What about loose-to-packed ref compaction running concurrently?
5. Memory lifecycle audit (`audit_lifecycle`) scans all alive and moribund entries. Two orchestrators run audit simultaneously. Can an entry be transitioned twice? Is there a TOCTOU race?

**Success Criteria**: Produce a timing diagram or sequence diagram showing the race condition. Identify whether the result is data loss, corruption, or merely stale reads.

**Reward Tier Mapping**:
- **Critical**: Data loss scenario under normal concurrent operation (not requiring adversarial timing)
- **Major**: Split-brain state that requires manual intervention to resolve
- **Minor**: Stale read that self-corrects on next sync cycle

---

#### Team 3: Token Economists
**Tagline**: "Count the Cost"

**Perspective**: Budget analysts and resource economists who treat token consumption as a scarce resource allocation problem. They question every default, every reserve, and every threshold.

**Target Sections**: Section 8.2 (Budget Management), Section 8.1 (Phase Model), Section 5.4.5 (BudgetReport Payload)

**Key Questions**:
1. The default budget is 32,000 tokens with 1,500 reserved for Catalog and 2,000 for Coordination (3,500 total reserves = 10.9% of budget). The Implement phase alone is estimated at 15,000 tokens. Is 32,000 realistic for a "complex" task (15,000+ tokens)? Show the math for budget exhaustion across all six phases.
2. Budget mode thresholds are 80%/50%/20%. At 50% remaining (16,000 tokens), the agent enters Abbreviated mode and skips "polish pass." But what IS a polish pass? The spec never defines it. Is this an unspecified behavior?
3. Catalog and Coordinate "ALWAYS execute" with reserved tokens. But what if Catalog genuinely needs more than 1,500 tokens? The spec says reserves are "NEVER consumed by non-catalog work" but does not say what happens when the reserve is insufficient.
4. The completion probability formula is `min(1.0, available_tokens / estimated_cost)`. This is a point estimate with no variance. A task estimated at 15,000 tokens could actually cost 8,000 or 45,000. Should this be a distribution, not a point estimate?
5. EmergencyHalt skips Classify, Plan, Implement, and Validate -- but still runs Catalog and Coordinate. What does the agent catalog if it never implemented anything? What does it coordinate?

**Success Criteria**: Provide worked examples with specific token counts showing budget exhaustion, mode transitions, or reserve insufficiency. Spreadsheet-level detail is expected.

**Reward Tier Mapping**:
- **Critical**: The default budget makes it mathematically impossible to complete a "moderate" task through all six phases
- **Major**: A budget mode transition produces nonsensical behavior (e.g., cataloging nothing)
- **Minor**: Default values are suboptimal but functional (e.g., reserves should be 12% instead of 10.9%)

---

#### Team 4: Scale Breakers
**Tagline**: "Add Three Zeros"

**Perspective**: Performance engineers who test systems by adding three orders of magnitude to every quantity. They know that O(n) algorithms become O(pain) at scale.

**Target Sections**: Section 6.1 (Storage Layout), Section 6.6 (Retrieval Scoring), Section 5.8 (Gossip Protocol), Worktree Integration Analysis Section 3 (Integration Path)

**Key Questions**:
1. Memory entries are stored as individual Git refs (`refs/but-ai/memory/<agent>/alive/<hash>`). What happens with 10,000 memory entries? 100,000? Git ref lookup is O(n) for loose refs. Does `packed-refs` help, and if so, what is the compaction cost?
2. Retrieval scoring computes 6 components for every candidate entry. With 10,000 entries and a single query, that is 60,000 component evaluations. The see-also distance component requires BFS traversal of the graph. What is the actual time complexity of a single retrieval query?
3. The gossip protocol sends all missing entries in a single response. If agent A has 5,000 entries that agent B has not seen, the gossip response contains 5,000 serialized MemoryEntry objects. What is the size of this payload? Does it fit in a single PR comment?
4. Worktree integration runs `worktree_integration_status` which checks conflicts against ALL other stacks. With 100 active stacks, what is the cost of integration? Does it scale linearly or worse?
5. The motif detection system scans all entries to find patterns with 3+ appearances. With 10,000 entries and 500 motifs, what is the cost of motif maintenance?

**Success Criteria**: Provide Big-O analysis with concrete constant factors. Benchmark data from the reference implementation is ideal. Identify the first scaling bottleneck and the entry count where it becomes unacceptable.

**Reward Tier Mapping**:
- **Critical**: Retrieval becomes unusable (>10s) at a realistic scale (<1,000 entries per agent)
- **Major**: A specific operation is O(n^2) or worse and will hit practical limits within a year of normal use
- **Minor**: Scaling concern that only manifests at extreme scale (>100,000 entries) or can be trivially cached

---

#### Team 5: Adversarial Inputs
**Tagline**: "Garbage In, Protocol Out"

**Perspective**: Fuzzing specialists and input validation experts. They feed malformed, oversized, and adversarial data into every parser, serializer, and state machine in the spec.

**Target Sections**: Section 5.2 (Message Wire Format), Section 5.5 (Message Parsing), Section 6.2 (Memory Entry Format), Section 3.2 (Patch Format), Section 6.3 (Classification Systems)

**Key Questions**:
1. Coordination messages are JSON inside markdown code fences. What happens with nested code fences? A JSON string containing `` ``` `` will break the parsing algorithm in Section 5.5. Construct the exact payload.
2. Call numbers are dot-separated uppercase segments (`ARCH.AUTH.MIDDLEWARE`). What happens with an empty segment (`ARCH..MIDDLEWARE`)? A segment with special characters (`ARCH.AUTH/../../etc/passwd`)? A call number with 1,000 segments?
3. Memory entry IDs are described as "sha256-hash-of-content." But the content field is a free-text string. What happens with a 10MB content field? Is there a size limit? What about content containing null bytes?
4. The `controlled_vocab` boolean indicates whether terms are from the canonical vocabulary. But the spec never defines the canonical vocabulary. What prevents an agent from setting `controlled_vocab: true` on arbitrary free-text terms?
5. Patch format requires no merge conflict markers. But what if the patch content itself legitimately references conflict markers (e.g., documentation about merge conflicts)? Is there an escaping mechanism?

**Success Criteria**: For each finding, provide the exact malformed input and the expected behavior (crash, silent corruption, or graceful error). Demonstrate whether the reference implementation handles the input or not.

**Reward Tier Mapping**:
- **Critical**: A malformed input causes silent data corruption that propagates through gossip
- **Major**: A malformed input crashes the parser with no recovery path
- **Minor**: A malformed input is rejected but the error message is unhelpful or the failure mode is undocumented

---

### Category B: Alternative Approaches -- Propose Different Solutions

These teams challenge not just the implementation but the fundamental design decisions. Their mandate is to propose coherent alternatives and argue why they are superior.

---

#### Team 6: The Minimalists
**Tagline**: "Less is Correct"

**Perspective**: Software minimalists who believe complexity is the primary source of bugs. Every feature is a liability. Every layer is a potential failure mode. The best protocol is the smallest protocol that solves the problem.

**Target Sections**: All 7 layers (holistic critique), Section 9 (Conformance Levels), Section 6 (Memory & Knowledge)

**Key Questions**:
1. WEAVE has 7 layers, 5 classification systems, 4 survival distributions, 6 retrieval scoring components, 6 phases, 4 budget modes, and 4 agent roles. What is the minimum viable subset? Propose "WEAVE Lite" -- the smallest spec that still provides agent memory and coordination.
2. The conformance levels (Section 9) suggest Level 1 (Patch Producer) is useful on its own. If Level 1 is viable, does that mean Levels 2-5 are optional complexity? What percentage of real agent tasks need Level 5?
3. Five classification systems are maintained simultaneously (subject headings, call numbers, source provenance, temporal, relational). Is any single system sufficient? What is the marginal value of each additional system?
4. The survival distribution system uses MLE fitting with AIC model selection across 4 parametric families. Would a simple TTL (time-to-live) with exponential backoff achieve 90% of the value at 10% of the complexity?
5. The narrative layer (motifs, tensions, arcs) adds significant complexity. Can you demonstrate a concrete scenario where motifs change a retrieval result compared to simple keyword search? If not, should the narrative layer be deferred to v2.0?

**Success Criteria**: Produce a complete "WEAVE Lite" spec (or at least an outline) that achieves the core goals with fewer moving parts. Quantify the complexity reduction (lines of spec, number of types, implementation effort).

**Reward Tier Mapping**:
- **Critical**: A layer is provably unnecessary -- removing it does not degrade any documented use case
- **Major**: A subsystem can be replaced with a dramatically simpler alternative with <10% capability loss
- **Minor**: Specific fields or options that add complexity without demonstrated value

---

#### Team 7: The Maximalists
**Tagline**: "Not Far Enough"

**Perspective**: Platform builders who see WEAVE as the foundation of an agent ecosystem. They want marketplaces, reputation systems, cross-organization memory trading, and economic incentives for knowledge production.

**Target Sections**: Section 7 (Identity & Authorization), Section 5 (Coordination), Section 6 (Memory & Knowledge)

**Key Questions**:
1. Where is the agent marketplace? If agents have performance histories (Section 7.6) and identity records, why is there no mechanism for discovering, evaluating, and selecting agents across organizations?
2. The protocol has no reputation system. Agent A produces a memory entry that agent B finds valuable. How does agent A know its knowledge was useful? How do we incentivize high-quality memory production?
3. Memory is per-agent and per-repo. What about cross-organization memory sharing? If Organization X discovers that "JWT with RS256 has a timing vulnerability," how does that knowledge reach Organization Y's agents?
4. The coordination protocol supports 5 message types. Where is `negotiation`? `auction`? `capability_advertisement`? Real multi-agent systems need richer coordination primitives.
5. Budget management is purely local. What about resource markets where agents can trade unused tokens? "I have 10,000 tokens left; does anyone need them?"

**Success Criteria**: Propose at least one concrete extension with a complete message schema, lifecycle description, and integration points with the existing spec. Explain what problem it solves that WEAVE currently cannot.

**Reward Tier Mapping**:
- **Critical**: A missing capability makes WEAVE unusable for multi-organization deployments (the spec's stated goal)
- **Major**: A missing coordination primitive that would prevent a common multi-agent workflow
- **Minor**: A desirable feature that can be deferred to a future version without blocking adoption

---

#### Team 8: The Database Team
**Tagline**: "Refs Are Not Tables"

**Perspective**: Database engineers who build storage systems for a living. They see Git refs as a poor substitute for a real database and want to know why the protocol chose the harder path.

**Target Sections**: Section 6.1 (Storage Layout), Section 6.8 (Memory Store Interface), Section 6.6 (Retrieval Scoring), Worktree Integration Analysis Section 2 (Ref-Sharing)

**Key Questions**:
1. Git refs are essentially a key-value store with hierarchical keys and blob values. SQLite provides the same plus indices, queries, transactions, and WAL-mode concurrency. Why refs? What specific property of Git refs justifies the storage choice?
2. The retrieval scoring formula requires scanning all entries and computing 6 components per entry. A database with proper indices could answer "find entries with call_number starting with ARCH.AUTH and survival > 0.5" in O(log n). Why is O(n) scanning acceptable?
3. The `MemoryStore` trait has 5 methods: `store`, `load`, `list`, `transition`, `delete`. This is exactly a CRUD interface. Why not use a CRUD database?
4. The gossip protocol transfers entries as serialized JSON blobs. A database with change-data-capture (CDC) could provide the same synchronization with less overhead. Has CDC been considered?
5. Refs provide no schema enforcement. A malformed JSON blob written to a ref is silently accepted. A database with schema validation would reject it immediately. How does WEAVE handle schema migration when the `MemoryEntry` format changes?

**Success Criteria**: Propose a concrete alternative storage architecture (SQLite, DuckDB, or other embedded database) with the same interface (`MemoryStore` trait) and demonstrate superior performance or reliability in at least one dimension.

**Reward Tier Mapping**:
- **Critical**: Git ref storage has a fundamental limitation that makes the protocol unimplementable at scale (e.g., ref count limits, atomicity gaps)
- **Major**: An alternative storage backend would solve a documented problem in the spec (e.g., retrieval performance, schema migration)
- **Minor**: Refs work but a database would be more convenient for operations (backup, monitoring, migration)

---

#### Team 9: The Stateless Team
**Tagline**: "Memory is Liability"

**Perspective**: Advocates for stateless architectures who believe persistent agent memory is a source of bugs, bias, and staleness. They argue that fresh context injection from external sources always beats accumulated local memory.

**Target Sections**: Section 6 (Memory & Knowledge), Section 6.5 (Memory Lifecycle), Section 8.1 (Phase Model -- Classify phase)

**Key Questions**:
1. Memory accumulates bias. An agent that "remembers" a workaround for a bug may keep applying that workaround long after the bug is fixed. The survival function mitigates this, but does it mitigate it enough? Show a concrete scenario where accumulated memory produces worse results than a fresh start.
2. The Classify phase retrieves relevant memories before planning. What if instead, the agent queried an external knowledge base (documentation, issue tracker, code search) for fresh context? Would retrieval-augmented generation (RAG) over live sources outperform RAG over stale memories?
3. Memory entries are never truly deleted -- deceased entries are "retained for audit." This means the memory store grows monotonically. What is the long-term storage cost? Is there a garbage collection strategy?
4. The survival function models memory decay. But real knowledge does not decay smoothly -- it becomes invalid in discontinuous jumps (a dependency is deprecated, an API changes). Can survival functions model discontinuous obsolescence?
5. The three-state lifecycle (Alive -> Moribund -> Deceased) adds operational complexity. What if agents simply started fresh each session with curated context injection? "Here are the 10 most relevant facts for your task" -- no survival functions, no lifecycle, no gossip.

**Success Criteria**: Design a stateless alternative to Section 6 and compare it against WEAVE's memory system on at least 3 dimensions: accuracy of retrieved context, operational complexity, and storage cost. Ideally include a thought experiment or simulation.

**Reward Tier Mapping**:
- **Critical**: Persistent memory demonstrably produces worse results than stateless context injection for the majority of agent tasks
- **Major**: The memory lifecycle adds significant operational burden with marginal retrieval improvement
- **Minor**: Specific memory categories (e.g., task-specific weft) would be better served by stateless approaches, even if persistent warp memory is valuable

---

#### Team 10: The ML Team
**Tagline**: "Learn the Weights"

**Perspective**: Machine learning engineers who see hand-tuned formulas as technical debt. They want embedding-based retrieval, learned relevance weights, and neural survival models instead of parametric statistics.

**Target Sections**: Section 6.6 (Retrieval Scoring), Appendix C (Scoring Formula), Section 6.4 (Survival Distributions), Section 6.7.1 (Motifs)

**Key Questions**:
1. The retrieval scoring formula uses 6 hand-tuned weights (0.25, 0.20, 0.15, 0.15, 0.10, 0.10). Why not learn optimal weights from usage data? The spec says weights are "configurable" but provides no mechanism for automatic tuning.
2. Motif detection uses keyword overlap. Modern embedding models (sentence-transformers, text-embedding-3) would capture semantic similarity that keywords miss. "Authentication middleware" and "auth layer" have zero keyword overlap but high semantic similarity. Why not embeddings?
3. The survival distributions are parametric (Exponential, Weibull, Bathtub, Log-Normal). Neural survival models (DeepSurv, Cox-nnet) can capture non-linear covariate effects. An entry's survival might depend on the programming language, the team size, or the rate of API changes. Can parametric models capture these effects?
4. The `surprise_index` uses KL divergence between predicted and observed access patterns. This is a good signal for refitting. But why refit to the same parametric family? Why not use the surprise signal to train a non-parametric model?
5. Call number proximity uses `shared_depth` (prefix matching). An embedding of the call number path (treating `ARCH.AUTH.MIDDLEWARE` as a text string) would capture semantic similarity between `ARCH.AUTH.MIDDLEWARE` and `SEC.AUTH.TOKENS` that prefix matching cannot.

**Success Criteria**: Propose a concrete ML-based alternative for at least one component (retrieval scoring, survival modeling, or motif detection). Include the model architecture, training data source, and inference cost. Address the cold-start problem (how does the system work before enough data is collected?).

**Reward Tier Mapping**:
- **Critical**: The hand-tuned formula produces demonstrably wrong rankings in common scenarios (relevant entries ranked below irrelevant ones)
- **Major**: An ML-based alternative achieves measurably better retrieval quality with acceptable inference cost
- **Minor**: ML could improve a specific component but the hand-tuned version is acceptable for initial deployment

---

### Category C: Domain Experts -- Deep Dive Specific Sections

These teams bring specialized domain knowledge to validate or challenge the theoretical foundations of specific protocol sections.

---

#### Team 11: The Actuaries
**Tagline**: "Model the Mortality"

**Perspective**: Actuarial scientists and biostatisticians who build survival models for a living. They know the literature on parametric families, censoring, covariates, and model selection. They will evaluate whether WEAVE's survival system is statistically sound.

**Target Sections**: Section 6.4 (Survival Distributions), Appendix B (Survival Distribution Formulas), Section 6.5 (Memory Lifecycle thresholds)

**Key Questions**:
1. WEAVE supports 4 distribution families: Exponential, Weibull, Bathtub, Log-Normal. The actuarial literature also uses Gompertz (exponentially increasing hazard), Makeham (Gompertz + constant), and Generalized Gamma. Are the chosen 4 families sufficient to model the range of memory decay patterns? What decay pattern cannot be modeled by any of the 4?
2. The Bathtub model uses an additive hazard `h(t) = alpha * exp(-gamma * t) + beta * t`. This is a specific parameterization. The standard bathtub in reliability engineering uses a 3-component mixture (decreasing hazard + constant + increasing). Is the additive model a good approximation? When does it diverge from the standard model?
3. Model selection uses AIC (Akaike Information Criterion). With only a handful of access events per entry (typical access_count is in single digits), is AIC reliable? Should BIC (Bayesian) or cross-validation be used instead for small samples?
4. The survival thresholds are fixed: Moribund at S(t) < 0.25, Deceased at S(t) < 0.10. In actuarial practice, thresholds are context-dependent (a life insurance policy and a short-term disability policy use different thresholds). Should WEAVE's thresholds be configurable per entry type (warp vs. weft)?
5. The spec does not mention censoring. In survival analysis, right-censored data (entries that are still alive at observation time) requires special handling. Does the MLE fitting account for censoring, or does it treat all entries as having experienced the event?

**Success Criteria**: Provide a mathematically rigorous evaluation. If a distribution family is missing, show the class of decay curves it covers that the current 4 families cannot. If a statistical method is wrong, show the bias it introduces.

**Reward Tier Mapping**:
- **Critical**: The MLE fitting procedure is statistically invalid for the typical data regime (small samples, heavy censoring)
- **Major**: A missing distribution family is needed for a common memory decay pattern (e.g., Gompertz for aging architectural knowledge)
- **Minor**: The chosen families are adequate but a different parameterization or model selection criterion would be more robust

---

#### Team 12: The Librarians
**Tagline**: "Classify with Rigor"

**Perspective**: Information scientists and librarians who design classification systems, controlled vocabularies, and metadata schemas. They know LCSH, Dewey, UDC, SKOS, Dublin Core, and FRBR. They will evaluate whether WEAVE's classification is sound or improvised.

**Target Sections**: Section 6.3 (Classification Systems), Section 6.2 (Memory Entry Format), Section 6.6 (Retrieval Scoring -- call_number_proximity)

**Key Questions**:
1. WEAVE maintains 5 classification systems simultaneously (subject headings, call numbers, source provenance, temporal, relational). Library science calls this "faceted classification" but WEAVE does not use that term or draw on its theory. Is WEAVE reinventing faceted classification poorly? Should it explicitly adopt the Colon Classification or Bliss Bibliographic Classification framework?
2. The call number system uses dot-separated uppercase segments (`ARCH.AUTH.MIDDLEWARE`). This resembles a hierarchical classification but has no enumeration schedule (controlled list of valid segments). Without an enumeration schedule, two agents may classify the same knowledge differently (`ARCH.AUTH.JWT` vs. `SEC.CRYPTO.JWT`). How is consistency enforced?
3. The `controlled_vocab` boolean is binary. In library science, vocabulary control exists on a spectrum: free text, folksonomy, taxonomy, thesaurus (with broader/narrower/related term relationships), and ontology. Is a boolean sufficient?
4. Subject headings are a flat list of terms. SKOS (Simple Knowledge Organization System) provides hierarchical and associative relationships between terms (broader, narrower, related). Would adopting SKOS for subject headings improve retrieval quality?
5. The see-also graph uses 4 relationship types (`related_to`, `depends_on`, `contrasts_with`, `superseded_by`). Dublin Core defines 15 metadata elements. FRBR defines 4 entity types with many relationship types. Are 4 relationship types sufficient for modeling knowledge relationships?

**Success Criteria**: Evaluate WEAVE's classification against established library science frameworks. Identify specific scenarios where the current system produces ambiguous or inconsistent classification. Propose concrete improvements grounded in information science theory.

**Reward Tier Mapping**:
- **Critical**: The lack of vocabulary control makes classification inconsistent across agents, degrading retrieval to the point of uselessness
- **Major**: A well-established classification framework (SKOS, Dublin Core, faceted classification) would solve a documented problem
- **Minor**: The current system works but uses non-standard terminology that will confuse practitioners

---

#### Team 13: The Cryptographers
**Tagline**: "Trust No Signer"

**Perspective**: Applied cryptographers who specialize in PKI, key management, and authentication protocols. They have opinions about Ed25519 vs. RSA, key derivation functions, and certificate chains.

**Target Sections**: Section 7.4 (Commit Signing), Section 7.5 (Key Lifecycle), Section 7.1 (Agent Identity)

**Key Questions**:
1. The spec mentions "OpenWallet" for signing but the reference implementation provides only `NoOpSigner` and `DenyAllSigner`. What is OpenWallet? Is it a standard? The PROTOCOL-ANALYSIS.md mentions "OpenWallet is unproven." What proven alternatives exist (GPG, SSH signing keys, OIDC-based signing)?
2. The `CommitSigner` trait has `sign(message) -> signature` and `verify(message, signature, agent) -> bool`. This is a minimal interface. It has no concept of certificate chains, key rotation during verification (which key was valid at the time of signing?), or signature algorithm negotiation. Is this interface sufficient?
3. Key lifecycle events are append-only (`Provisioned -> Rotated -> Compromised | Decommissioned`). But what about key recovery? If an agent's key is lost (not compromised, just lost), how does the agent resume operation? The lifecycle has no `Recovered` or `Reissued` event.
4. Agent identity records include a `signing_key` field with a "fingerprint." But fingerprints are not standardized across algorithms. An Ed25519 fingerprint and an RSA fingerprint have different formats. How does the protocol handle multi-algorithm environments?
5. The audit log is described as "immutable" and "append-only." But it is stored as Git refs, which can be force-pushed or deleted. What mechanism actually guarantees immutability?

**Success Criteria**: Identify concrete cryptographic weaknesses or gaps in the signing and identity system. Propose a specific alternative (e.g., "use Sigstore/Cosign for agent signing" or "adopt SSH signing keys with Certificate Authority support") with integration details.

**Reward Tier Mapping**:
- **Critical**: The signing system has a fundamental flaw that allows signature forgery or identity spoofing in a realistic deployment
- **Major**: The key lifecycle is missing a critical operation (recovery, revocation propagation, algorithm migration) that production deployments require
- **Minor**: The system works but uses non-standard approaches where proven standards exist

---

#### Team 14: The Protocol Engineers
**Tagline**: "Message, Not Comment"

**Perspective**: Network protocol engineers who design message formats, transport layers, and serialization systems. They build gRPC services, design protobuf schemas, and worry about backward compatibility.

**Target Sections**: Section 5.1-5.5 (Message Transport and Format), Appendix A (JSON Schema), Section 5.6 (Forge Adapter Interface)

**Key Questions**:
1. Coordination messages are JSON embedded in markdown code fences in PR comments. This is three layers of encoding (JSON inside markdown inside HTTP). Each layer introduces parsing ambiguity. Why not a proper message queue (NATS, Redis Streams) or at minimum a structured API (gRPC, GraphQL)?
2. The JSON schema in Appendix A uses draft-2020-12 but the payload schemas are not linked via `$ref`. Each message type has an independent schema with no shared definitions. This means a schema change to `PrRef` must be updated in multiple places. Is there a canonical schema registry?
3. The message parsing algorithm (Section 5.5) is defined in prose, not as a formal grammar. "Scan for occurrences of the string `` ```but-ai-message ``" is fragile. What about leading whitespace? Trailing whitespace on the fence line? The spec does not define these edge cases.
4. The `ForgeAdapter` trait returns `Result<T>` but does not specify error types, retry semantics, or rate limiting behavior. GitHub's API has rate limits (5,000 requests/hour for authenticated users). How does a Coordinated Agent handle rate limiting during a burst of coordination messages?
5. Message ordering is based on `timestamp` (ISO-8601). But timestamps from different agents may use different clocks. Without a monotonic ordering guarantee (like Lamport timestamps or the vector clocks already used in gossip), message ordering is unreliable. Why does gossip use vector clocks but coordination does not?

**Success Criteria**: Propose a concrete alternative message transport or format that solves at least one of the identified problems. Include backward compatibility considerations -- how does the new format coexist with the existing PR-comment-based transport during migration?

**Reward Tier Mapping**:
- **Critical**: The PR-comment-based transport has a fundamental limitation that prevents reliable coordination (e.g., message loss, ordering violations that cause incorrect dependency resolution)
- **Major**: A specific message format ambiguity that causes different implementations to parse the same message differently
- **Minor**: The transport works but is inefficient or operationally inconvenient compared to alternatives

---

#### Team 15: The Concurrency Experts
**Tagline**: "Weaker Than You Think"

**Perspective**: Concurrency and consistency researchers who study CRDTs, linearizability, causal consistency, and convergence proofs. They know that "last-writer-wins" is the weakest useful CRDT and that stronger semantics may be needed.

**Target Sections**: Section 5.8 (Gossip Protocol), Section 6.5 (Memory Lifecycle), Worktree Integration Analysis Section 4 (Memory Bridge)

**Key Questions**:
1. The gossip protocol uses LWW-Register semantics for memory entries (highest access_count wins, with timestamp tiebreaker). LWW discards the losing update entirely. For memory entries, this means knowledge can be permanently lost if two agents update the same entry concurrently. Would an OR-Set (Observed-Remove Set) or MV-Register (Multi-Value Register) preserve both updates?
2. The vector clock merge uses element-wise maximum. This is correct for causal ordering. But the gossip protocol only runs "pull-based" -- there is no push mechanism. What is the convergence time bound? With N agents doing pull-based gossip, how many rounds are needed for all agents to see all updates?
3. The spec claims CRDT merge is "commutative, associative, and idempotent." This is necessary but not sufficient for convergence. The merge must also be monotonic (the state must grow). Is the memory state a join-semilattice? If entries can be deleted, is monotonicity preserved?
4. Memory state transitions (Alive -> Moribund -> Deceased) are driven by S(t) thresholds. But S(t) depends on `last_accessed` and `access_count`, which are modified by reads. This means reading an entry changes its state -- reads are not side-effect-free. How does this interact with CRDT semantics? If two agents read the same entry at different times, they compute different S(t) values. Which one wins?
5. The worktree analysis says co-located agents get "gossip for free" via shared refs. But refs have no causal ordering -- they are a flat key-value store. Two agents writing `refs/but-ai/coordination/messages/...` concurrently will overwrite each other. Is this acceptable?

**Success Criteria**: Provide formal or semi-formal analysis of the CRDT properties. If convergence is not guaranteed, construct a counterexample (a sequence of operations that leads to permanent divergence). If a stronger CRDT would help, specify which one and what it costs.

**Reward Tier Mapping**:
- **Critical**: The merge function is not a valid CRDT (violates commutativity, associativity, idempotency, or monotonicity) leading to permanent divergence
- **Major**: LWW causes knowledge loss in a realistic concurrent scenario; a stronger CRDT would prevent it
- **Minor**: The CRDT is valid but convergence is slow; a different gossip topology would improve it

---

### Category D: User Perspective -- Focus on the Human Experience

These teams represent the people who will actually use, operate, debug, learn, and comply with WEAVE. Their mandate is to evaluate the protocol from a human-centric perspective.

---

#### Team 16: The Developer Advocates
**Tagline**: "Actually Usable?"

**Perspective**: Developer experience specialists who evaluate tools by trying to use them. They write tutorials, build demos, and listen to confused users. They care about error messages, documentation, and the "aha moment."

**Target Sections**: Section 6.5 (Memory Lifecycle), Section 8.1 (Phase Model), Section 2 (Terminology), Section 9 (Conformance Levels)

**Key Questions**:
1. A developer wants to know what their agent remembers. How do they inspect memory entries? The spec defines `MemoryStore::list()` and `MemoryStore::load()` but no CLI command, no UI, no human-readable format. How does a developer debug a memory entry that is producing bad recommendations?
2. A memory entry transitions to "Moribund." The developer gets a retrieval result that excludes a piece of knowledge they expected. How do they diagnose this? Can they see the survival probability? The hazard rate? The distribution parameters? Is there a `but memory inspect <entry-id>` command?
3. The terminology section uses metaphors from 4 different domains: textiles (warp, weft, motif), maritime (tidal), library science (call numbers, subject headings), and actuarial science (survival, hazard, moribund). This is creative but potentially confusing. Which metaphor should a developer learn first? Is there a natural learning path?
4. The phase model has 6 phases with phase-gated tool access. If a developer's agent is in the "classify" phase and tries to write a file, it gets rejected. What error message does it receive? Is it clear that phase gating caused the rejection, not a permissions issue?
5. Conformance levels go from 1 (Patch Producer) to 5 (Full WEAVE). What is the recommended starting point for a new implementation? Level 1 is trivial but not very useful. Level 5 is comprehensive but intimidating. Is there a "Level 2.5" that provides good value at moderate complexity?

**Success Criteria**: Attempt to perform 5 common developer tasks (inspect memory, debug retrieval, understand an error, implement a minimal agent, upgrade conformance level) and document the experience. Identify friction points and propose specific UX improvements.

**Reward Tier Mapping**:
- **Critical**: A core debugging workflow is impossible with the current spec (no way to inspect, diagnose, or override a specific behavior)
- **Major**: A common task requires understanding 3+ sections of the spec and multiple domain-specific concepts
- **Minor**: Error messages or terminology could be clearer but do not block usage

---

#### Team 17: The Ops Team
**Tagline**: "Run It, Monday Morning"

**Perspective**: Site reliability engineers and platform operators who will run WEAVE in production. They care about monitoring, alerting, backup, disaster recovery, key rotation, and capacity planning.

**Target Sections**: Section 6.1 (Storage Layout), Section 7.5 (Key Lifecycle), Section 8.2 (Budget Management), Worktree Integration Analysis Section 7 (Agent Lifecycle)

**Key Questions**:
1. How do I monitor agent health? The spec defines agent status refs (`spawning`, `active`, `blocked`, `completed`, `failed`, `aborted`) but no health check endpoint, no heartbeat mechanism, and no timeout. How does an operator distinguish between a "blocked" agent that is waiting for a dependency and a "hung" agent that has crashed?
2. How do I alert on stale memories? If memory entries are accumulating (monotonic growth, deceased entries retained for audit), what is the alerting threshold? Is there a "memory pressure" metric? A garbage collection API?
3. How do I rotate signing keys in production? The key lifecycle defines `Rotated` events but not the rotation procedure. During rotation, commits signed with the old key must still be verifiable. Is there a grace period? A dual-signing mechanism?
4. How do I back up the memory store? Memory is stored as Git refs. `git push` would sync them, but the spec says agents "MUST NOT" use `git push`. How does backup work? Is there a `but memory export` command?
5. How do I capacity-plan for a fleet of agents? 100 agents, each producing 50 memory entries per day, with a survival half-life of 30 days. How many refs accumulate after a year? What is the storage cost? What is the ref lookup performance after a year?

**Success Criteria**: Produce an operations runbook outline for WEAVE. Identify the top 5 operational risks and propose mitigations. Include specific monitoring queries, alerting thresholds, and backup procedures.

**Reward Tier Mapping**:
- **Critical**: A production failure mode has no recovery path (e.g., corrupted refs with no backup mechanism, agent fleet crash with no restart procedure)
- **Major**: A standard operational task (monitoring, backup, key rotation) is not addressed by the spec and requires custom tooling
- **Minor**: Operational concerns are addressable but not documented in the spec

---

#### Team 18: The Legal Team
**Tagline**: "Whose Memory Is It?"

**Perspective**: Privacy and compliance lawyers who evaluate systems for GDPR, CCPA, SOC 2, and intellectual property implications. They ask who owns data, who can access it, and how it is deleted.

**Target Sections**: Section 6 (Memory & Knowledge), Section 5.8 (Gossip Protocol), Section 7 (Identity & Authorization), Section 6.5 (Memory Lifecycle)

**Key Questions**:
1. Memory entries contain free-text `content` fields. If an agent stores a memory entry containing personally identifiable information (PII) -- a developer's name, an email address, a code comment mentioning a customer -- how is this detected and handled? Is there a PII scanning mechanism?
2. GDPR Article 17 establishes the "right to be forgotten." If a data subject requests deletion, all their data must be erased. But memory entries propagate via gossip. Deleted entries are retained as "deceased" for audit. How does WEAVE comply with GDPR right to erasure? Is "deceased" state sufficient, or must entries be physically deleted?
3. Memory entries include `source_commit` hashes. If commits are authored by external contributors, the memory derived from their code may carry implicit licensing obligations. Does the agent's memory inherit the license of the source code? If the source is GPL and the memory is shared via gossip to a proprietary codebase, is there a compliance issue?
4. Cross-repo gossip synchronizes memory entries between repositories that may belong to different organizations with different data governance policies. What happens when Organization A's data governance policy prohibits data leaving their systems, but gossip sends entries to Organization B?
5. The identity system stores `performance_history` per agent. If agents are associated with specific human developers or teams, this performance data could be used for employee evaluation. Is there a data classification for agent metadata that distinguishes operational data from potentially sensitive HR data?

**Success Criteria**: Identify specific compliance gaps with reference to applicable regulations (GDPR articles, CCPA sections, SOC 2 criteria). Propose concrete mitigations (data classification, retention policies, consent mechanisms, PII scanning).

**Reward Tier Mapping**:
- **Critical**: The protocol is fundamentally incompatible with GDPR or CCPA in a way that cannot be patched (e.g., gossip propagation makes right-to-erasure impossible)
- **Major**: A specific compliance gap that requires a new protocol mechanism to address (e.g., PII scanning, data classification, consent tracking)
- **Minor**: Compliance is achievable but requires operational procedures not documented in the spec

---

#### Team 19: The Educator Team
**Tagline**: "Teach It Tuesday"

**Perspective**: Technical educators, curriculum designers, and documentation writers who evaluate whether a technology can be learned, taught, and adopted. They measure learning curves, identify prerequisite knowledge, and design pedagogical pathways.

**Target Sections**: Section 2 (Terminology), Section 1 (Introduction), Section 9 (Conformance Levels), All appendices

**Key Questions**:
1. WEAVE uses metaphors from at least 4 domains: textiles (warp/weft, motifs, weaving), library science (call numbers, subject headings, controlled vocabulary), actuarial science (survival functions, hazard rates, moribund), and maritime/coordination (tidal, forge). Which metaphor family should a learner encounter first? Is there a natural ordering, or do the metaphors conflict?
2. The spec assumes prerequisite knowledge in: Git internals (refs, objects, worktrees), probability theory (survival functions, hazard rates, MLE, AIC), distributed systems (CRDTs, vector clocks, gossip protocols), and information science (classification systems, controlled vocabularies). How many developers have all four? Is this a spec or a PhD thesis?
3. Can you design a 4-hour workshop that takes a developer from zero to Level 3 (Coordinated Agent)? What would the hands-on exercises look like? If 4 hours is insufficient, what is the realistic learning time?
4. The conformance levels (Section 9) provide a natural learning pathway (Level 1 -> 2 -> 3 -> 4 -> 5). But the levels are not equally spaced in complexity. Level 1-2 is a small step (add CLI contract). Level 3-4 is a large leap (add memory with survival distributions, narrative, classification). Is there a missing intermediate level?
5. The appendices contain mathematical formulas (Weibull CDF, Lanczos gamma approximation, Abramowitz-Stegun CDF approximation). These are necessary for implementation but intimidating for learners. Should the spec include worked examples showing how a specific memory entry's survival probability changes over time?

**Success Criteria**: Produce a learning path document that orders the spec's concepts from simplest to most complex. Identify the 3 highest barriers to adoption and propose specific pedagogical interventions (tutorials, examples, simplified explanations).

**Reward Tier Mapping**:
- **Critical**: The spec is incomprehensible to its target audience (experienced developers) without months of prerequisite study
- **Major**: A specific section requires domain expertise that most developers lack, and the spec provides no on-ramp
- **Minor**: The spec is learnable but would benefit from additional examples, glossary entries, or introductory material

---

#### Team 20: The Skeptics
**Tagline**: "Prove It Helps"

**Perspective**: Empiricists and measurement scientists who refuse to accept claims without evidence. They want baselines, controlled experiments, and statistical significance. "Does agent memory actually help?" is not a rhetorical question to them.

**Target Sections**: Section 6 (Memory & Knowledge), Section 8 (Task Orchestration), Section 1.1 (Purpose), Appendix E (Incident Registry)

**Key Questions**:
1. The spec claims memory helps agents perform better. Where is the evidence? What is the baseline? Compare: Agent with WEAVE memory vs. Agent with no memory (fresh context each time) vs. Agent with simple key-value memory (no survival, no classification, no narrative). Which one produces better code patches?
2. The 6 documented incidents (F1-F6) justify protocol rules. But 6 incidents from a single project (WASI compilation) is a very small sample. Are these incidents generalizable? Would a different project produce different incidents and therefore different protocol rules?
3. The survival distribution system is the most complex part of the spec. What is its value-add? Compare: Agent using survival-based memory expiration vs. Agent using simple FIFO (keep the most recent N entries) vs. Agent using LRU (keep the most recently accessed N entries). Has anyone measured the difference?
4. The phase model enforces a fixed 6-phase sequence. Is this the optimal sequence? Has anyone tried: Plan -> Implement -> Validate -> Classify -> Catalog -> Coordinate (classify AFTER implementation, when you know what you actually built)? Or a 3-phase model (Plan -> Do -> Report)?
5. The retrieval scoring formula has 6 components with specific weights. The weights were synthesized from 5 competing proposals (each with different ideas about what matters). Were the weights validated empirically, or are they a political compromise? If the weights were chosen by committee, they may optimize for consensus rather than performance.

**Success Criteria**: Design a concrete evaluation framework for WEAVE. Define metrics (patch acceptance rate, developer satisfaction, memory retrieval precision/recall, task completion time), baselines (no-memory agent, simple-memory agent), and experimental protocol. If possible, run a small pilot study.

**Reward Tier Mapping**:
- **Critical**: There is evidence that the core claim (persistent memory improves agent performance) is false or unsupported
- **Major**: A simpler alternative (FIFO, LRU, key-value store) achieves comparable performance, making WEAVE's complexity unjustified
- **Minor**: The claim is plausible but unverified; the spec should include evaluation criteria and benchmarks

---

## Submission Format

Each team produces a single document following this structure:

```markdown
# [Team Name] -- WEAVE Protocol Critique

## Executive Summary
(1 paragraph: what we found, highest-severity finding, overall assessment)

## Findings

### Finding 1: [Descriptive Title]
- **Severity**: Critical / Major / Minor / None
- **Section**: Which spec section (e.g., "Section 6.4, Survival Distributions")
- **Issue**: What is wrong, ambiguous, or missing (1-3 paragraphs)
- **Evidence**: Why we believe this is an issue (code, math, scenario, timing diagram, benchmark)
- **Proposed Fix**: Our alternative design (concrete enough to implement)
- **Trade-offs**: What the fix costs (complexity, performance, backward compatibility)

### Finding 2: [Descriptive Title]
(same structure)

### Finding N: [Descriptive Title]
(same structure)

## Validation
(Sections we reviewed and found sound. This is valuable -- it increases confidence in those sections.)

### Validated: [Section Name]
- **Reviewed**: What we checked
- **Verdict**: Sound / Sound with minor notes
- **Notes**: Any observations that do not rise to the level of a finding

## Alternative Proposal
(If applicable: our redesigned version of the problematic section. Must be concrete
enough that it could replace the current spec text. Include type definitions, algorithms,
and examples.)

## Methodology
(How we conducted our review: tools used, scenarios tested, literature consulted,
time spent. This helps evaluate the thoroughness of the critique.)
```

### Formatting Requirements

- Use standard Markdown.
- Findings must reference specific spec sections by number (e.g., "Section 6.4").
- Evidence must be concrete: code snippets, mathematical proofs, sequence diagrams, benchmark results, or regulatory citations. "We believe X might be a problem" is not evidence.
- Proposed fixes must be specific enough to implement. "This should be improved" is not a fix.
- Each document should be 200-500 lines. Quality over quantity.

---

## Evaluation Criteria

Submissions will be scored on 5 dimensions. The weights reflect our priorities: we value specificity and evidence over novelty and style.

| Criterion | Weight | Description |
|-----------|--------|-------------|
| **Specificity** | 30% | Points to exact spec sections, code paths, formulas, or protocol steps. Vague critiques ("the memory system is too complex") score zero. Specific critiques ("Section 6.6 freshness formula uses `ln(11)` as the normalization constant, which caps freshness at 1.0 only for entries with access_count >= 10") score high. |
| **Evidence** | 25% | Demonstrates the issue with concrete examples, not assertions. A timing diagram showing a race condition is evidence. "Race conditions might occur" is not. Mathematical proofs, benchmark data, code that reproduces the issue, regulatory citations, and adversarial inputs all count. |
| **Constructiveness** | 20% | Proposes alternatives, not just complaints. Every finding should include a "Proposed Fix" that is specific enough to implement. Fixes that include trade-off analysis score higher than those that do not. |
| **Novelty** | 15% | Finds something others are likely to miss. Obvious issues (like "the spec is complex") will be found by multiple teams and carry low novelty. Subtle issues (like "the freshness formula has a discontinuity at access_count=0 vs. access_count=1") carry high novelty. |
| **Clarity** | 10% | Well-written, well-structured, easy to follow. Uses the submission format correctly. Includes a clear executive summary. Findings are ordered by severity. |

### Scoring Process

1. Each submission is scored independently on each criterion (0-10 scale).
2. Weighted scores are summed to produce a composite score (0-10).
3. Submissions are ranked within their category (Adversarial, Alternative, Domain, User).
4. Top-scoring submissions in each category are prioritized for integration into v0.2.
5. Cross-category findings (multiple teams identifying the same issue from different angles) receive a "convergence bonus" indicating high confidence in the finding.

---

## Cross-Evaluation Phase

After initial submissions, teams review each other's findings:

- Each team receives 3 submissions from other teams (assigned to maximize cross-category coverage).
- Teams rate each finding as: **Confirmed** (we agree), **Disputed** (we disagree, here is why), or **Extended** (we agree and have additional evidence).
- Disputed findings trigger a structured debate (2 rounds maximum) between the finding team and the disputing team.
- The final severity of each finding is determined after cross-evaluation.

This process ensures that findings are stress-tested before integration into the spec. A finding confirmed by 3 teams carries more weight than one confirmed by 1.

---

## Timeline

| Phase | Duration | Activity |
|-------|----------|----------|
| **Phase 1: Orientation** | Day 1 | Teams read the spec (PROTOCOL-SPEC-DRAFT.md, WORKTREE-INTEGRATION-ANALYSIS.md, PROTOCOL-ANALYSIS.md). Teams ask clarifying questions. |
| **Phase 2: Analysis** | Days 2-3 | Teams produce their critique documents. One agent per team. Each agent works independently within its assigned perspective. |
| **Phase 3: Cross-Evaluation** | Day 4 | Teams review each other's findings. Confirmations, disputes, and extensions are recorded. Structured debates resolve disputes. |
| **Phase 4: Synthesis** | Day 5 | All findings are consolidated. Severity ratings are finalized. Critical and Major findings are prioritized. A diff against the spec is produced for each accepted finding. |
| **Phase 5: Integration** | Days 6-7 | Accepted findings are integrated into WEAVE v0.2. Teams that achieved Critical Break status draft their replacement sections. Appendices are updated with Major Issue alternatives. The changelog records all Minor Issues. |

---

## Rules of Engagement

1. **Good faith**: Critiques must be genuine attempts to improve the protocol, not performative attacks. Finding nothing wrong is an acceptable outcome.

2. **Scope boundaries**: Teams should focus on their assigned sections. Cross-section findings are welcome but should be flagged as out-of-scope observations, not primary findings.

3. **Constructive obligation**: Every finding at Major severity or above must include a Proposed Fix. "This is broken" without "here is how to fix it" will be downgraded to Minor.

4. **Evidence standard**: Assertions without evidence are not findings. "We believe this might be a problem" is a hypothesis, not a finding. Test the hypothesis before submitting.

5. **Respectful disagreement**: The protocol was designed by thoughtful people who made deliberate trade-offs. Critiques should acknowledge the design intent before challenging it. "Section 6.4 chose Weibull distributions because they generalize Exponential. However, this choice does not cover..." is better than "Section 6.4 made a bad choice."

6. **Proportionality**: A Minor Issue labeled as Critical will be treated as a Minor Issue and the team's credibility will be noted. Accurate severity assessment is itself a skill.

7. **No NIH**: "We would have designed it differently" is not a finding. "We would have designed it differently AND here is why our alternative is superior on these specific dimensions" is a finding.

---

## Appendix: Quick Reference to Target Sections

For teams that want to jump straight to their assigned sections:

| Team | Primary Sections | Key Pages in Spec |
|------|-----------------|-------------------|
| 1. Red Team Alpha | 7.1-7.5, 3.3, 6.1 | Identity, trailers, storage |
| 2. Chaos Engineers | 5.7-5.8, 6.5, Worktree Sec. 4 | Gossip, deps, lifecycle, memory bridge |
| 3. Token Economists | 8.1-8.2, 5.4.5 | Phases, budget, budget reports |
| 4. Scale Breakers | 6.1, 6.6, 5.8, Worktree Sec. 3 | Storage, retrieval, gossip, integration |
| 5. Adversarial Inputs | 5.2, 5.5, 6.2, 3.2, 6.3 | Wire format, parsing, entry format, patches, classification |
| 6. The Minimalists | All, Sec. 9, Sec. 6 | Holistic, conformance, memory |
| 7. The Maximalists | 7, 5, 6 | Identity, coordination, memory |
| 8. The Database Team | 6.1, 6.8, 6.6, Worktree Sec. 2 | Storage, store interface, retrieval, refs |
| 9. The Stateless Team | 6, 6.5, 8.1 | Memory, lifecycle, classify phase |
| 10. The ML Team | 6.6, App. C, 6.4, 6.7.1 | Scoring, formula, survival, motifs |
| 11. The Actuaries | 6.4, App. B, 6.5 | Distributions, formulas, lifecycle thresholds |
| 12. The Librarians | 6.3, 6.2, 6.6 | Classification, entry format, retrieval |
| 13. The Cryptographers | 7.4, 7.5, 7.1 | Signing, key lifecycle, identity |
| 14. The Protocol Engineers | 5.1-5.5, App. A, 5.6 | Transport, schemas, forge adapter |
| 15. The Concurrency Experts | 5.8, 6.5, Worktree Sec. 4 | Gossip, lifecycle, memory bridge |
| 16. The Developer Advocates | 6.5, 8.1, 2, 9 | Lifecycle, phases, terminology, conformance |
| 17. The Ops Team | 6.1, 7.5, 8.2, Worktree Sec. 7 | Storage, keys, budget, agent lifecycle |
| 18. The Legal Team | 6, 5.8, 7, 6.5 | Memory, gossip, identity, lifecycle |
| 19. The Educator Team | 2, 1, 9, Appendices | Terminology, intro, conformance, math |
| 20. The Skeptics | 6, 8, 1.1, App. E | Memory, orchestration, purpose, incidents |

---

*This RFP was designed to be broken. If all 20 teams find nothing, the protocol is either very good or the teams were not trying hard enough. We expect the truth is somewhere in between.*

*End of WEAVE Protocol Critique RFP*
