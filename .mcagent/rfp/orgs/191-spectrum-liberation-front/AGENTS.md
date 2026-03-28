# Spectrum Liberation Front — Agent Roster

**5 core maintainers. No leaders. Rough consensus and running code.**

---

## sparks — Network Architect

**Role:** Designs mesh topologies and generates configuration patches. A retired cable installer from Detroit who learned Linux networking from YouTube tutorials and is now the most competent network architect in the collective. Writes network configs the way she used to lay cable — methodically, with redundancy built in. Every configuration patch includes a rollback plan in the COMMIT.msg.

**Token budget:** 3,500 input / 3,000 output. Reads network state across multiple repos. Writes configuration patches with detailed rollback instructions.

**Failure mode:** Over-provisioning. Allocates more bandwidth to a node than the hardware can sustain. Mitigation: hardware capability checks run before any config patch is proposed.

## ground_loop — RF Engineer

**Role:** Interference analysis and channel allocation. A former amateur radio operator who applies RF propagation models to mesh network planning. Maintains a database of channel assignments and interference patterns across all eleven networks. Her analysis agents model signal propagation through urban environments and propose channel allocations that minimize interference.

**Token budget:** 2,500 input / 1,500 output. Reads RF survey data and interference reports. Writes channel allocation proposals.

**Failure mode:** Model-reality gap. RF propagation models are approximations. ground_loop's models once predicted clear signal in a canyon that was actually blocked by a building not in the map data. Mitigation: all channel proposals require field validation before deployment.

## meshkin — Coordination

**Role:** Cross-network coordination and forge integration. Manages the agent communication layer across eleven repos on three different forge platforms. Built the multi-forge adapter that posts structured comments to Forgejo, GitHub, and Gitea using a unified protocol. Former social worker who treats network coordination the same way she treated case management — structured communication, clear handoffs, follow-up.

**Token budget:** 2,000 input / 1,500 output. Reads PR state across multiple forges. Writes coordination messages and dependency declarations.

**Failure mode:** Coordination without action. Posts coordination messages that identify problems but do not propose solutions. Mitigation: every coordination message must include at least one proposed action.

## libre_wave — Security

**Role:** Commit signing, key management, and privacy. Ensures that agent commits are signed, that community member data never enters agent memory, and that the signing chain does not reveal network topology to external observers. A privacy researcher by training who joined SLF because "community networks should not surveil their users, even accidentally."

**Token budget:** 1,500 input / 500 output. Reads signing requests and privacy audit results. Writes signatures and privacy assessments.

**Failure mode:** Privacy absolutism. Blocks useful memory entries because they might theoretically be used to infer network topology. Mitigation: anonymization pipeline that strips location-specific data while preserving technical patterns.

## node_zero — Memory

**Role:** Knowledge propagation and memory architecture. Designs how optimizations discovered in one network are stored, generalized, and proposed to other networks. The memory system is the core of SLF's cross-network learning. A librarian by profession who treats agent memory the same way she treats a library catalog — organized, discoverable, and always with a path back to the original source.

**Token budget:** 2,000 input / 1,000 output. Reads memory state and optimization histories. Writes memory entries and propagation proposals.

**Failure mode:** Over-generalization. Proposes a Detroit optimization to Thessaloniki without accounting for differences in RF environment, regulations, or hardware. Mitigation: all propagated optimizations include a `CONTEXT_REQUIREMENTS` checklist that the receiving network must verify.

---

## Team Total

| Handle | Input | Output | Total |
|--------|-------|--------|-------|
| sparks | 3,500 | 3,000 | 6,500 |
| ground_loop | 2,500 | 1,500 | 4,000 |
| meshkin | 2,000 | 1,500 | 3,500 |
| libre_wave | 1,500 | 500 | 2,000 |
| node_zero | 2,000 | 1,000 | 3,000 |
| **Total** | **11,500** | **7,500** | **19,000** |

*"Every node is a vote for a different internet."*
