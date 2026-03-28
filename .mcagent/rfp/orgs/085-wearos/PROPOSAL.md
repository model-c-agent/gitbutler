# WearOS Proposal — `but-ai` Plugin

**Submitted by:** WearOS
**Date:** 2026-03-28

---

## Core Thesis

Firmware is fashion is firmware. The plugin must manage code that runs on people's bodies. Compatibility is safety. The compatibility matrix is sacred.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary. CLI mode: `but ai flash` (produce firmware patch), `but ai compat` (check compatibility matrix), `but ai fleet` (garment fleet status), `but ai deploy` (staged rollout). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `CompatMatrix` (validate cross-component compatibility) and `FleetTarget` (tag patch with garment hardware revision).

WASI: Compatibility checking only. Run the compat matrix validator in WASI to verify that a firmware build is safe before deploying. No write capability.

Config: `[but-ai]` in Git config.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` only. Provider via `gitbutler.aiModelProvider`. WearOS adds a "code-generation accuracy" metric per provider, measured by running generated firmware patches through the compiler and checking for warnings:

```
[but-ai "provider-accuracy"]
    anthropic = 0.94
    openai = 0.91
    ollama = 0.85
    lmstudio = 0.83
```

For firmware patches (safety-critical), only providers with accuracy >= 0.90 are used. For backend/app patches, any provider is acceptable. Updated monthly.

---

## 3.3 The But Agent

Multi-component workflow:

1. **Scope** (Mbeki): Define the change, identify affected components (firmware, app, backend)
2. **Produce** (Trujillo/Venkatesh/Osei): Each component agent produces its INDEX.patch
3. **Compat check** (automated): Verify compatibility matrix across all patches
4. **Design review** (Park): Verify no design impact
5. **Stage deploy:** Canary to 10% of fleet, then full rollout

Every COMMIT.msg includes the compatibility matrix: `compat:grid-rev-N+,mcu-vN+,app-vN.N+`. Missing matrix = rejected.

Branch naming: `wearos/<component>/<feature>/<task-id>` — e.g., `wearos/firmware/ble-latency/fix-2026-031`.

Budget: Trujillo + Osei (firmware) 40%, Venkatesh (platform) 30%, Park (coordination) 20%, Mbeki (oversight) 10%. At 80% budget, produce minimal viable patches for the highest-priority component only.

---

## 3.4 Polyrepo PR-Based Coordination

Three repos: firmware, companion-app, cloud-backend. Park coordinates.

```json
{
  "wearos_schema": "1.0",
  "type": "patch|compat-check|deploy-request|incident",
  "component": "firmware|app|backend",
  "compat_matrix": {
    "grid_rev_min": 2,
    "mcu_version_min": "v4",
    "app_version_min": "3.2.0"
  },
  "from": "Park",
  "body": "...",
  "fleet_pct_target": 10,
  "budget_remaining_pct": 62
}
```

Every cross-repo message includes the compatibility matrix. Messages that affect multiple components must declare all affected components and their minimum compatible versions.

Forge adapter: trait with `submit_patch` (PR), `annotate` (comment), `component_status` (list by component), `deploy_approve` (merge). GitHub implementation.

Cross-repo: Firmware changes that require app changes create bidirectional dependencies. Both PRs must pass compat check before either merges.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/wearos/memory/<component>/<hardware-rev>/`. Organized by component and hardware revision — because firmware behavior varies by hardware.

**Structure:**
```json
{
  "key": "grid-rev-2-ble-latency",
  "component": "firmware",
  "hardware_rev": "grid-rev-2",
  "value": "BLE connection latency on grid-rev-2 is 340ms avg (vs. 180ms on rev-3) due to older BLE stack. Optimization ceiling: ~250ms.",
  "compat": { "grid_rev": [2], "mcu_version": ["v3", "v4"] },
  "ttl": "180d",
  "tags": ["ble", "latency", "grid-rev-2"]
}
```

**Relevance scoring:** Component + hardware-rev match is primary. BM25 on tags within matched entries. Entries with compatibility data matching the current task's target hardware receive a 1.5x boost. Maximum 5 entries.

**Expiration:** Hardware-specific memories expire when the hardware revision is end-of-lifed. Component-agnostic architectural decisions TTL 365 days. Incident memories never expire (they are post-mortems).

**Compaction survival:** Active hardware revision memories always survive. EOL hardware memories are dropped. Incident memories always survive.

**Identity:** At `refs/wearos/identity/<name>`:
```json
{
  "name": "Trujillo",
  "role": "firmware-lead",
  "components": ["firmware"],
  "hardware_expertise": ["grid-rev-2", "grid-rev-3", "mcu-v3", "mcu-v4"],
  "signing_key_id": "wearos:trujillo:2026"
}
```

**Unique insight:** Hardware-revision-indexed memory. In a system where the same code runs on different hardware with different behaviors, memory must be partitioned by hardware. What is true about BLE on grid-rev-2 is false on grid-rev-3. Hardware-aware memory prevents agents from applying optimization strategies that only work on specific hardware.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed. Firmware patches require dual signature: producer (Trujillo or Osei) + compat validator (automated, signed by WearOS CI key).

**Authorization:**
```json
{
  "Trujillo": { "allow": ["wearos/firmware/*"], "deny": ["main", "wearos/app/*"] },
  "Venkatesh": { "allow": ["wearos/backend/*"], "deny": ["wearos/firmware/*"] },
  "Osei": { "allow": ["wearos/firmware/*"], "deny": ["wearos/backend/*"] },
  "Park": { "allow": ["wearos/*/coordination"], "deny": ["wearos/*/implementation"] },
  "Mbeki": { "allow": ["main", "wearos/*"], "role": "ceo" }
}
```

Firmware and backend are separated. Nobody who writes firmware can deploy backend changes, and vice versa.

**Key lifecycle:**
- Provisioning: at hire, via OpenWallet
- Rotation: every 6 months (startup pace)
- Compromise: immediate revocation. If a firmware signing key is compromised, all garments running firmware signed by that key are flagged for OTA re-validation.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,600 | 0 | Once/session | Identity, tools, fleet status |
| Task ingestion | 2,000 | 500 | Once/task | Feature spec, target hardware |
| Planning | 1,500 | 800 | Once/task | Component decomposition |
| Tool call (per call) | 1,000 | 500 | 5/task avg | Branch ops, compat checks |
| Patch generation | 3,500 | 4,500 | Once/task | INDEX.patch |
| Compat matrix check | 800 | 400 | Once/task | Cross-component validation |
| Commit message | 500 | 400 | Once/task | With compat matrix |
| Memory retrieval | 1,500 | 200 | 2/task avg | Hardware-rev-indexed |
| Coordination event | 1,500 | 600 | 2/task avg | Cross-component PR |
| **TOTAL (typical)** | **19,900** | **10,400** | -- | Multi-component feature |

---

## Testing Strategy

1. **Compat matrix:** Submit patches with incompatible matrices. Verify rejection.
2. **Hardware isolation:** Verify grid-rev-2 memories do not contaminate grid-rev-3 patches.
3. **Canary deploy:** Mock 10% fleet deployment. Inject regression. Verify rollback.
4. **Cross-repo dependency:** Firmware change requires app change. Verify both must pass before either merges.
5. **Provider accuracy:** Generate firmware patches with all 4 providers. Compile. Compare warning counts.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 50000 | Per-task budget |
| `but-ai.agent.memoryRef` | `refs/wearos/memory` | Memory prefix |
| `but-ai.agent.compatRequired` | true | Require compat matrix in commits |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.canaryPct` | 10 | Fleet percentage for canary deploy |
| `but-ai.agent.firmwareAccuracyMin` | 0.90 | Provider accuracy floor for firmware |

---

*"2,300 garments. 1,400 heartbeats. Every firmware update is a promise."*
