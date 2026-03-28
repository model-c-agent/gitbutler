# Spectrum Liberation Front — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Federated Knowledge Propagation

---

## Executive Summary

SLF proposes a federated agent system designed for multi-network knowledge sharing. Agents in each community mesh network discover local optimizations and propose them as patches to other networks, with context-aware adaptation. All inference is local. All forges are supported. The system learns from every network and makes that learning available to all.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary installs to `~/.local/bin/but-ai` (we follow XDG conventions). Compiled from Rust, statically linked, cross-compiled for x86_64 and aarch64 (several networks run on ARM hardware). The binary is distributed through our own package repository — we do not depend on external package managers.

Subcommands: `but ai scan` (analyze local network configuration for optimizations), `but ai propose` (generate a patch for the local network), `but ai propagate` (generate adapted patches for other networks), `but ai context` (display the SOCIAL_CONTEXT checklist for a proposal).

The `context` subcommand is our lesson from the Oakland Channel Conflict. It generates a human-readable checklist of assumptions the agent made, prompting the maintainer to verify: "This proposal assumes no neighboring networks use channel 11. Is that correct?"

## Requirement 2: Provider-Agnostic AI

Local only. We run Ollama on each network's coordination server (typically a repurposed desktop or a Raspberry Pi 5). Model: Mistral 7B quantized to 4-bit for ARM compatibility. On x86 servers with GPUs, we use Codestral for configuration analysis.

The provider interface: `analyze(config_context) -> Optimizations` and `adapt(optimization, target_context) -> AdaptedPatch`. Two methods. The `adapt` method is the key capability — it takes an optimization discovered in one network and adapts it for a different network's context (different hardware, different RF environment, different channel assignments).

No cloud providers. If a network's coordination server cannot run local inference, that network does not get agent support until the hardware is upgraded. We do not compromise on data sovereignty.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The optimization discovery pipeline:

1. sparks' scan agent analyzes network performance metrics and configuration files
2. The agent identifies configurations that could be improved (suboptimal channel assignments, redundant routing paths, degraded node connections)
3. The agent generates INDEX.patch modifying the network configuration files
4. COMMIT.msg includes context and rollback:

```
Optimize: reallocate node-14 from channel 6 to channel 11

Interference reduction: estimated 12dB improvement based on
ground_loop's propagation model for this urban segment.
Rollback: revert to previous channel assignment in node-14.conf.
Social-Context: REQUIRES VERIFICATION — are neighboring networks
using channel 11 in this coverage area?
```

5. Local maintainers verify the SOCIAL_CONTEXT checklist
6. If verified, libre_wave signs and the patch is applied

## Requirement 4: Polyrepo PR Coordination

Eleven networks, three forge platforms. meshkin's coordination layer posts structured propagation proposals:

```
[SLF:propagate] detroit-mesh#47 → oakland-mesh
Optimization: node channel reallocation reduced interference 12dB.
Adaptation-Required: oakland uses different hardware (Ubiquiti vs TP-Link).
Channel mapping may differ. Context-check required.
```

The multi-forge adapter is the collective's most technically complex component. It normalizes PR comments across Forgejo, GitHub, and Gitea into a unified format. Each forge has different API idioms, different authentication models, and different rate limits. meshkin maintains a compatibility matrix that is updated whenever a forge releases a new API version.

Propagation is advisory. The receiving network's maintainers decide whether to accept. SLF does not impose optimizations from one network on another — that would be the kind of centralized authority we exist to oppose.

## Requirement 5: Agent Memory in Git Branches

Memory branch per network: `refs/slf/memory/<network-name>`. Memory types:

- **`optimization`**: Discovered optimizations with performance delta and context requirements. TTL: 90 days.
- **`interference`**: Known interference sources and mitigation strategies. TTL: 30 days (interference landscape changes frequently).
- **`propagation-outcome`**: Whether a propagated optimization was accepted or rejected by the receiving network, with reasons. TTL: 1 year. This is the learning signal — the agent learns which optimizations transfer well between networks and which are context-specific.

A shared memory branch (`refs/slf/memory/global`) stores cross-network patterns: optimizations that were accepted by 3+ networks, indicating they are broadly applicable.

Memory retrieval uses key-based lookup with network-context filtering. When proposing a propagation, the agent retrieves `propagation-outcome` entries for the target network to check whether similar proposals were previously rejected.

## Requirement 6: Signed Commits via OpenWallet

Each maintainer has a pseudonymous OpenWallet DID. libre_wave manages key provisioning through a peer-to-peer key ceremony: two existing members co-sign the new member's DID. No central authority issues keys — the web of trust is the authority.

Key rotation follows network maintenance cycles (roughly quarterly). Revocation requires two member co-signatures — the same threshold as provisioning. This prevents a single compromised member from disrupting the signing infrastructure.

Signed commits include a `Network-Attestation` header listing which network the commit targets. This prevents a signed commit from being replayed against a different network's repository.

**Unique insight:** SLF's `adapt` method — taking an optimization from one network and adapting it for another's context — is a generalized form of knowledge transfer that most agent architectures lack. Typical agent systems share knowledge by copying it verbatim. SLF's agents share knowledge by *translating* it. The optimization "switch from channel 6 to channel 11" is not useful as a literal instruction to a network that does not use channel 6. But the *principle* — "when interference is detected on your current channel, scan for a cleaner channel and propose a switch" — transfers. The adaptation layer encodes this distinction between specific instructions and general principles, which is relevant to any multi-project `but-ai` deployment.

---

## Token Budget

| Handle | Input | Output | Total |
|--------|-------|--------|-------|
| sparks | 3,500 | 3,000 | 6,500 |
| ground_loop | 2,500 | 1,500 | 4,000 |
| meshkin | 2,000 | 1,500 | 3,500 |
| libre_wave | 1,500 | 500 | 2,000 |
| node_zero | 2,000 | 1,000 | 3,000 |
| **Task Total** | **11,500** | **7,500** | **19,000** |

All tokens consumed locally. Monthly infrastructure cost: amortized across community-donated hardware. Estimated: $0 marginal cost per optimization.

---

*"Mesh networks are not infrastructure. They are mutual aid with antennas."*
