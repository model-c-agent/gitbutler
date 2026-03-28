# root@field -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Host:** self-hosted Gitea, location: a barn
**Date:** 2026-03-28

---

## Context

We hack tractor firmware. We need version control that works for anonymous, distributed contributors working on legally sensitive modifications. Our proposal optimizes for: local-only operation, operator privacy, safety review, and compatibility with self-hosted forges.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary on PATH. Distributed as a reproducible build with published SHA-256 hashes on our Gitea instance. No auto-update mechanism (auto-update is a supply chain attack vector). Users verify the hash and install manually.

No daemon. No background processes. No filesystem access outside the repository directory and the config file. The plugin is a tool, not a service.

Configuration in TOML. The config file supports an `[anonymity]` section:
- `strip_metadata: true` — remove timestamps, hostnames, paths from output
- `ephemeral_identity: true` — generate new DID per session
- `encrypt_memory: true` — AES-256-GCM for stored memory
- `prompt_sanitize: true` — strip secrets and paths from prompts before sending to provider

These are not defaults for everyone. They are defaults for us.

---

## Requirement 2: Provider-Agnostic AI

Four providers. Ollama and LMStudio recommended. Cloud providers are supported but come with a warning: every prompt sent to a cloud provider is data you do not control.

The provider trait includes a `sanitize_prompt` hook that runs before every API call. The hook strips: absolute paths, environment variable values, hostnames, IP addresses, and strings matching common secret patterns. The sanitization is configurable — users can add custom patterns.

No telemetry to the provider beyond the prompt and response. No usage reporting, no analytics callbacks, no "helpful" features that phone home.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

`patch` generates firmware modifications. The workflow:

1. `.git` retrieves platform-specific context (memory maps, CAN protocol, known bugs)
2. `patch` reads the task and context
3. `patch` generates INDEX.patch (metadata-stripped unified diff)
4. `patch` generates COMMIT.msg:
   ```
   Restore diagnostic mode access on JD8R firmware 4.2

   Manufacturer restricts Mode 6 diagnostics to authorized dealers.
   This patch removes the dealer authentication check in the diagnostic
   menu handler at 0x4A00-0x4A80.

   Platform: JD8R
   Firmware: 4.2.1
   Safety-Review: required (modifies diagnostic subsystem)
   ```
5. `diff` reviews for correctness, safety, and privacy
6. If `NEEDS-HW-TEST`, the patch is tagged and queued for hardware verification

### Safety Review Protocol

Patches that modify code within 256 bytes of a safety-critical region are automatically flagged for enhanced review. Safety-critical regions are defined per platform in the memory system. `diff` cannot approve a safety-flagged patch alone — it requires a second reviewer with hardware expertise.

---

## Requirement 4: Polyrepo PR Coordination

47 repositories across 12 manufacturers. Cross-platform patches (same bug, different manufacturer) require coordination.

PR comment schema:
```
<!-- r@f:coord src=jd8r-firmware@fix-diag tgt=claas-lexion@port-diag status=review hw-tested=false -->
```

HTML comment format for invisibility to casual observers. Contains: source repo/branch, target repo/branch, status, and hardware test status.

Forge: Gitea first. The trait has three methods: `post_comment`, `read_comments`, `pr_status`. We also implement a Gitea-specific method for self-hosted webhook configuration, but this is behind an extension trait.

---

## Requirement 5: Agent Memory in Git Branches

Encrypted memory in `refs/but-ai/memory/<platform>/<version>/<key>`. Encrypted because memory entries contain reverse-engineering data that manufacturers would prefer did not exist.

Memory types:
| Type | TTL | Encrypted | Example |
|------|-----|-----------|---------|
| Memory map | Until firmware version changes | Yes | Register addresses, function signatures |
| CAN message definition | Until protocol revision | Yes | Message IDs, signal layouts |
| Known bug | 1 year | No | Bug description (not exploit) |
| Field report | 30 days | Yes | Contributor-submitted hardware test results |
| Safety region | Permanent | No | Defined safety-critical address ranges |

Safety region data is unencrypted because it needs to be readable by all agents without a decryption key — safety checks must work even if memory encryption configuration is wrong.

---

## Requirement 6: Signed Commits via OpenWallet

Ephemeral DID signing by default. Each session generates a fresh identity. The DID is valid for the duration of the session plus 30 days (for verification by reviewers).

For contributors who want persistent pseudonymity (same identity across sessions, not linkable to a real name), the plugin supports long-lived pseudonymous DIDs. The DID is stored in the local config, never transmitted to a directory service.

Key rotation: per-session for ephemeral DIDs, 30 days for persistent pseudonyms.

We do not support real-name identity. If a contributor wants to attach their legal name, they can do so in the commit message body. The signing infrastructure does not require or encourage it.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| patch | 10,000 | 7,000 | 17,000 | Firmware patches |
| diff | 6,000 | 2,500 | 8,500 | Review |
| .git | 4,500 | 1,000 | 5,500 | Memory |
| chmod | 3,500 | 1,000 | 4,500 | Signing/budget |
| **Collective** | **24,000** | **11,500** | **35,500** | |

---

## Unique Insight: Safety Review as a Typed Check

Firmware patches are dangerous. A bad patch can disable a tractor's engine governor, unlock maximum hydraulic pressure, or disable the PTO emergency stop. In the worst case, a bad firmware patch kills someone.

Our collective has developed a safety review protocol that we believe should be part of every `but-ai` implementation, not just ours:

1. **Safety region registry:** A machine-readable list of memory addresses and code regions that are safety-critical. Stored in the repository, versioned, and maintained by contributors with hardware expertise.
2. **Automatic flagging:** Any patch that modifies code within a configurable proximity of a safety region is automatically flagged. The flag is a structured annotation, not a human judgment.
3. **Elevated review:** Flagged patches require a second reviewer with hardware expertise. The second review is recorded in the commit trail.
4. **Hardware test requirement:** Patches flagged as safety-critical cannot be merged until a contributor reports a successful hardware test. The test report is a structured memory entry.

This protocol adds cost and latency. We accept both. The alternative — shipping unsafe firmware — is not acceptable, regardless of the time pressure.

Every codebase has safety-critical regions. For us, it is tractor firmware. For others, it is authentication code, financial transaction logic, or medical device software. A general-purpose `but-ai` safety review protocol — typed regions, automatic flagging, elevated review — would make every AI-generated patch safer.

---

```
root@field:~$ echo "If they won't fix it, we will."
If they won't fix it, we will.
root@field:~$ exit
```
