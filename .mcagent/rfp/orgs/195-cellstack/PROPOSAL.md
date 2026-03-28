# CellStack — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Multi-Protocol Configuration Agents

---

## Executive Summary

CellStack proposes a configuration management agent system for software-defined small cells. Agents analyze site conditions and generate unit configurations as versioned patches. Cross-unit coordination ensures configuration changes at one site do not degrade performance at neighboring sites. The system is designed for field deployment where human operators may be unreachable during emergencies.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary installs to `~/.gitbutler/bin/`. Luis builds it in Rust, cross-compiled for x86_64 (office machines) and aarch64 (field laptops). The binary is self-contained — it includes the protocol constraint validator so that configurations can be checked offline in the field.

Subcommands: `but ai survey` (ingest site survey data), `but ai configure` (generate unit configuration patch), `but ai coordinate` (check cross-unit impact), `but ai deploy` (push approved configuration to field unit via CI/CD), `but ai emergency` (auto-approve pre-authorized emergency changes).

The `emergency` subcommand implements the lesson from the Montana Blizzard. It accepts only configuration changes that match a pre-approved template (e.g., "satellite failover with bandwidth prioritization") and applies them without human review. The template constraints are strict: only specific parameters may change, only within defined ranges.

## Requirement 2: Provider-Agnostic AI

Provider routing by task type:

| Task | Provider | Rationale |
|------|----------|-----------|
| Configuration generation | Anthropic | Best structured output for parameter sets |
| Cross-unit impact analysis | Ollama (local) | Sensitive deployment topology data stays local |
| Emergency templates | No AI — rule-based | Emergency changes must be deterministic |

The provider interface: `generate_config(site_data, constraints) -> RankedConfigs`. Returns a ranked list of configurations with expected performance scores. Raj reviews the top-ranked proposal; Priya reviews any that deviate from standard patterns.

Fallback: cloud provider failure falls back to Ollama. Ollama failure falls back to rule-based defaults (pre-computed conservative configurations). The system always has an answer — in the field, "I don't know" is not acceptable.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The configuration pipeline:

1. Priya's survey agent ingests site data (RF measurements, terrain, traffic forecasts)
2. The configuration agent generates a ranked list of parameter sets
3. Raj validates the top choice against protocol constraints
4. The agent generates INDEX.patch modifying the unit's configuration file:

```diff
- protocol_allocation = { lte = 0.5, nr = 0.5 }
+ protocol_allocation = { lte = 0.3, nr = 0.6, satellite_reserve = 0.1 }
- handover_threshold_dbm = -110
+ handover_threshold_dbm = -105
```

5. COMMIT.msg:

```
Config: unit-MT-007 peak hour optimization

Site: Montana grid section 7 (grain elevator)
Change: increase 5G allocation, add satellite reserve
Performance-Expected: +15% throughput during peak (18:00-22:00)
Robustness-Margin: 10% applied to all parameters
Cross-Unit-Impact: checked — no degradation to MT-005, MT-006, MT-008
Protocol-Compliance: 3GPP TS 38.331 validated
```

## Requirement 4: Polyrepo PR Coordination

One repo per deployment (Montana, Norway). Cross-deployment coordination is rare (deployments are geographically independent) but cross-unit coordination within a deployment is constant.

Within a deployment, units are branches. Configuration changes that affect multiple units are coordinated through PR comments:

```
[CS:coordinate] montana-deploy: unit-MT-007#23 impacts unit-MT-006
Increasing transmit power on MT-007 will raise interference floor
at MT-006 by estimated 2dB. Recommend compensating adjustment
to MT-006 handover threshold. Patch for MT-006 attached.
```

Luis's forge adapter supports GitHub (current). The Norway operator has requested GitLab. Luis will build it when the expansion contract is signed and funded.

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/cellstack/memory/<deployment>`. Memory types:

- **`site-profile`**: RF characteristics and traffic patterns per site. TTL: until next survey (typically quarterly).
- **`config-outcome`**: Performance measurements after configuration changes. TTL: 1 year. This is the feedback loop — the agent learns which configuration changes actually improved performance vs. which only looked good in simulation.
- **`failure-event`**: Documented failures (backhaul outages, interference events, hardware faults). TTL: permanent. Failure history informs emergency template design.
- **`cross-unit-impact`**: Measured impact of configuration changes on neighboring units. TTL: 6 months. Essential for coordination accuracy.

Memory retrieval is key-based: `<deployment>:<unit>:<parameter>`. Kira maintains the memory lifecycle and runs weekly compaction to keep the memory branch manageable.

## Requirement 6: Signed Commits via OpenWallet

Each team member has an OpenWallet DID. Configuration commits require dual signature: the generating agent (Priya's pipeline) and the approving human (Raj or Maya). Emergency commits require only the generating agent's signature but are tagged `EMERGENCY_AUTO_APPROVED` and queued for retroactive human review.

The signing chain is important for regulatory compliance — telecom operators must demonstrate that configuration changes to radio equipment are authorized and traceable. CellStack's signed commit history provides this traceability. The Montana electric cooperative's regulatory filing cited the commit log as evidence of compliant configuration management.

**Unique insight:** CellStack's emergency auto-approve pathway — pre-authorized templates that deploy without human review under defined conditions — addresses a gap in most agent architectures: the assumption that a human reviewer is always available. In field-deployed systems (rural infrastructure, maritime, disaster response), the human may be unreachable precisely when the agent's output is most needed. The template-constrained auto-approve pattern provides a principled middle ground: the human pre-authorizes a narrow set of actions, and the agent executes them autonomously only within those bounds. This is neither full autonomy nor full human-in-the-loop — it is *bounded delegation*, and it is the pattern that real-world deployments demand.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Maya | 1,500 | 800 | 2,300 |
| Raj | 3,500 | 2,500 | 6,000 |
| Priya | 4,000 | 4,000 | 8,000 |
| Luis | 3,000 | 2,500 | 5,500 |
| Kira | 2,000 | 1,000 | 3,000 |
| **Task Total** | **14,000** | **10,800** | **24,800** |

Cross-unit coordination overhead: 3,500 tokens. Grand total per configuration task: **28,300 tokens**.

---

*"Software-defined means never having to send a truck."*
— CellStack pitch deck, slide 1
