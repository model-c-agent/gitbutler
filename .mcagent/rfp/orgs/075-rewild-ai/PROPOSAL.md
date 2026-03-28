# ReWild.ai Proposal — `but-ai` Plugin

**Submitted by:** ReWild.ai
**Date:** 2026-03-28

---

## Core Thesis

The plugin is a deployment pipeline. Every patch targets a device. Every commit is a firmware candidate. Speed matters, but false negatives kill animals.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary. CLI mode: `but ai deploy` (produce device-targeted patch), `but ai validate` (run canary checks), `but ai status` (device fleet health). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `DeviceTarget` (tag patch with hardware revision and reserve) and `CanaryDeploy` (mark patch for limited rollout).

WASI: Validation-only mode. Camera traps could theoretically run the validator locally (they are ARM devices running Linux). Partial WASI support is a long-term goal.

Config: `[but-ai]` in Git config.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` only. Provider via `gitbutler.aiModelProvider`. ReWild.ai adds latency-aware provider selection: for time-critical patches (poacher alert response), the system automatically selects the lowest-latency provider. For routine patches, it selects the most cost-effective provider.

```
[but-ai "provider-latency"]
    ollama = 0.5s
    lmstudio = 0.6s
    openai = 1.5s
    anthropic = 2.0s
```

Latency profiles are updated weekly from actual measurements. New providers: config + adapter.

---

## 3.3 The But Agent

Device-targeted workflow:

1. **Scope:** Krishnamurthy defines the task and target devices (reserve, hardware revision)
2. **Produce:** Deshmukh (edge) or Okafor (platform) generates INDEX.patch + COMMIT.msg
3. **Validate:** Hendriks reviews. If approved, signs an approval tag.
4. **Canary:** Sato deploys to 10% of target devices
5. **Full deploy:** After 24h with no anomalies, Sato deploys to remaining devices

Branch naming: `rewild/<reserve>/<hw-rev>/<task-id>` — e.g., `rewild/tadoba-s7/rev3/nocturnal-fix`.

Budget: Krishnamurthy allocates. Patch producers get 55%, validation 20%, coordination 15%, oversight 10%. At 80% budget, produce current best and enter canary-only mode.

---

## 3.4 Polyrepo PR-Based Coordination

Sato coordinates across repos (edge firmware repo, platform repo, model repo). Schema:

```json
{
  "rewild_schema": "1.0",
  "type": "patch|validation|canary|full-deploy|alert",
  "target": { "reserve": "tadoba-s7", "hw_rev": "3" },
  "from": "Sato",
  "body": "...",
  "canary_status": "pending|passed|failed",
  "budget_remaining_pct": 55
}
```

The `alert` type is for time-critical patches triggered by poaching events. Alert PRs skip the normal review queue and go directly to Hendriks for expedited validation.

Forge adapter: `ForgeAdapter` trait with `submit` (PR), `annotate` (comment), `fleet_status` (list PRs by reserve/device), `deploy` (merge). GitHub implementation.

Cross-repo: `depends-on:` in PR descriptions. Platform patches may depend on model repo changes; edge patches may depend on platform API changes.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/rewild/memory/<reserve>/<species>/`. Organized by deployment target (reserve) and species.

**Structure:**
```json
{
  "key": "tadoba-s7-tiger-nocturnal",
  "reserve": "tadoba-s7",
  "species": "Panthera tigris",
  "value": "Nocturnal detection false-negative rate 4.2% on hw-rev-3",
  "device_class": "hw-rev-3",
  "ttl": "60d",
  "tags": ["detection", "nocturnal", "false-negative"]
}
```

**Relevance scoring:** Reserve match + species match as primary filters. Within matched entries, BM25 on tags. Device-class match provides a 1.3x boost. Maximum 5 entries per retrieval.

**Expiration:** Device-specific memories expire when the device firmware is updated (TTL tied to firmware version, not calendar date). Species observation memories follow calendar TTL (60 days default).

**Compaction survival:** Memories tagged `alert` (related to poaching events) always survive. Performance metrics survive if their firmware version is still deployed. Everything else follows TTL.

**Identity:** At `refs/rewild/identity/<name>`:
```json
{
  "name": "Deshmukh",
  "role": "edge-systems",
  "target_expertise": ["arm-linux", "raspberry-pi", "camera-firmware"],
  "signing_key_id": "rewild:deshmukh:2026"
}
```

**Unique insight:** Firmware-versioned memory expiration. Memories about device behavior are tied to the firmware version, not calendar dates. When firmware is updated, the old memories become irrelevant. This prevents agents from acting on performance data from obsolete firmware.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed. Hendriks' approval is a countersignature — patches that affect detection models require two signatures (producer + validator).

**Authorization:**
```json
{
  "Deshmukh": { "allow": ["rewild/*/edge/*"], "deny": ["main", "rewild/*/platform/*"] },
  "Okafor": { "allow": ["rewild/*/platform/*"], "deny": ["main", "rewild/*/edge/*"] },
  "Hendriks": { "allow": ["rewild/*/validation/*", "main"], "role": "validator" },
  "Sato": { "allow": ["rewild/*/deploy/*"], "deny": ["rewild/*/edge/*", "rewild/*/platform/*"] }
}
```

Model-affecting patches require Hendriks' countersignature. The system rejects deployment if only one signature is present.

**Key lifecycle:**
- Provisioning: at hire, via OpenWallet API
- Rotation: every 6 months (startup moves fast, key hygiene must keep up)
- Compromise: immediate revocation. All canary deployments from the compromised key are rolled back automatically.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Identity, tools, active deployments |
| Task ingestion | 2,000 | 400 | Once/task | Target devices, scope |
| Planning | 1,200 | 700 | Once/task | Device selection, approach |
| Tool call (per call) | 1,000 | 500 | 4/task avg | Branch ops, device targeting |
| Patch generation | 3,500 | 4,500 | Once/task | INDEX.patch |
| Commit message | 500 | 400 | Once/task | With device target metadata |
| Memory retrieval | 1,500 | 200 | 2/task avg | Reserve/species lookup |
| Validation | 1,500 | 800 | Once/task | Hendriks review |
| Coordination event | 1,500 | 600 | 1/task avg | Cross-repo deploy comms |
| **TOTAL (typical)** | **19,200** | **9,600** | -- | 200-line patch, multi-device target |

---

## Testing Strategy

1. **Multi-device:** Verify patches target correct hardware revisions. Cross-compile tests for ARM.
2. **Canary simulation:** Mock 10% deployment, inject anomaly, verify rollback triggers.
3. **Countersignature:** Verify model patches fail deployment without Hendriks' approval.
4. **Memory expiration:** Bump firmware version, verify old device memories expire.
5. **Alert path:** Verify alert-type PRs bypass normal queue and reach validator within SLA.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 50000 | Per-task budget |
| `but-ai.agent.memoryRef` | `refs/rewild/memory` | Memory prefix |
| `but-ai.agent.canaryPct` | 10 | Percentage of devices for canary deploy |
| `but-ai.agent.canaryWaitHours` | 24 | Hours before full deploy |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.alertBypass` | true | Skip queue for alert-type patches |

---

*"400 cameras. 1.2 million images per day. Every one of them matters."*
