# FCIU -- Technical Proposal

**Classification:** UNCLASSIFIED
**RFP:** GitButler `but-ai` Plugin
**Date:** 2026-03-28
**Prepared by:** Major Obi, Technical Officer, on behalf of Lt. Col. Reyes

---

## Operational Summary

The Financial Crimes Investigation Unit proposes a `but-ai` implementation designed for sustained, multi-day agent operations against complex financial targets. Our use case is not a 3-file feature branch. It is a 72-hour continuous graph traversal across millions of transactions. The plugin must handle long-running operations, strict evidence standards, and military-grade operational discipline.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Invoked per-task. Stateless between invocations.

- Binary: Rust, statically linked, FIPS 140-2 validated cryptographic libraries
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml` with case-level overrides
- Audit: every invocation logged with timestamp, arguments, and exit code

FIPS compliance is non-negotiable. The unit's evidence may be presented in federal court. Cryptographic operations must use validated implementations.

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`, `health_check`. Provider set per case.

| Provider | Clearance | Usage |
|----------|-----------|-------|
| Anthropic | UNCLASSIFIED | Standard investigations |
| OpenAI | UNCLASSIFIED | Batch processing |
| Ollama | All levels | Air-gapped operations on classified hardware |
| LMStudio | UNCLASSIFIED | Development and testing |

Air-gapped operation via Ollama is mission-critical. Classified case data never leaves unit-controlled hardware.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Evidence-Grade Output

Every INDEX.patch adds structured findings to the case record. Patches follow the FCIU Evidence Format:

```
evidence: identify structuring pattern in target account TGT-003

17 transactions between 2025-06-01 and 2025-08-15.
All amounts between $9,500 and $9,999.
Total: $167,234.
Pattern consistent with structuring (31 USC 5324).

Case: FCIU-2026-017
Agent: SIGMA-1
Confidence: HIGH
Evidence-class: SECONDARY (AI-generated, pending verification)
Graph-depth: 3
Traversal-path: TGT-001 -> SHELL-004 -> TGT-003
```

### Long-Running Operations

Standard agent operations complete in seconds. Graph traversals can run for hours. The plugin supports checkpoint-and-resume: SIGMA-1 produces intermediate patches at configurable intervals (default: every 100 graph edges traversed), allowing recovery from interruption without restarting the traversal.

---

## 4. Polyrepo PR Coordination

The unit coordinates with partner agencies and external forensic teams via forge-based PR workflows.

### Message Format

```
FCIU-COORD-<case>-<seq>
PRIORITY: <level>
FROM: SIGMA-3
TO: <agency/unit>
ACTION: <evidence_share | request_authorization | status_update>
PAYLOAD: <structured JSON>
CLASSIFICATION: UNCLASSIFIED
```

### Forge Support

GitHub (primary), GitLab, Forgejo. The unit does not use Bitbucket. Classified coordination uses air-gapped Gitea instances.

### Cross-Jurisdiction Protocol

Cross-jurisdiction evidence sharing requires SIGMA-3 to obtain authorization (from Reyes) before posting coordination messages to external repos. Unauthorized cross-jurisdiction messages are blocked by the forge adapter.

---

## 5. Agent Memory in Git Branches

### Case-Classified Memory

Memory stored in `refs/fciu/memory/<case>/<classification>/<key>`.

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `value` | Memory content |
| `case` | Case reference |
| `classification` | UNCLASSIFIED, RESTRICTED, CONFIDENTIAL |
| `source` | Agent that produced it |
| `ttl` | Hours (patterns: indefinite during active case, methods: 2160) |
| `verified` | Boolean -- has a human verified this? |

### Cross-Case Intelligence

Unlike evidence, investigative methods and patterns CAN be shared across cases. Methods memory (e.g., "structuring detection works best with 90-day windows") is stored in a shared namespace `refs/fciu/memory/methods/` with no case restriction.

Evidence memory is case-isolated. No exceptions. No override process. The firewall is absolute.

---

## 6. Signed Commits via OpenWallet

### Military-Grade Chain of Custody

Every commit is signed with the agent's OpenWallet credential. The credential includes:
- Agent callsign
- Case authorization
- Classification level
- Timestamp (RFC 3339, synchronized to unit NTP server)

### Key Management

- Rotation: every 24 hours (aligned with watch rotation)
- Revocation: immediate, logged in `refs/fciu/revoked` and reported to Reyes
- FIPS compliance: all signing operations use FIPS 140-2 validated modules
- Dual control: SIGMA-4 cannot provision its own keys; SIGMA-2 provisions all keys

---

## 7. Token Budget

| Callsign | Role | Input | Output | Total |
|----------|------|-------|--------|-------|
| SIGMA-1 | Investigation | 9,500 | 5,500 | 15,000 |
| SIGMA-2 | Technical | 3,500 | 800 | 4,300 |
| SIGMA-3 | Watch/coordination | 5,000 | 2,200 | 7,200 |
| SIGMA-4 | Evidence custody | 3,500 | 900 | 4,400 |
| **Unit Total** | | **21,500** | **9,400** | **30,900** |

Per-task budget for standard operations. CRITICAL cases operate with unlimited budget and post-hoc accounting.

---

## 8. Unique Insight: Checkpoint-and-Resume for Long-Running Agent Operations

Most agent frameworks assume tasks complete in a single invocation. Financial investigation tasks do not. A graph traversal across 2 million edges takes hours. If the agent crashes at edge 1.8 million, restarting from zero wastes hours of compute and token budget.

Our proposal: **checkpoint-and-resume**. The agent produces intermediate INDEX.patch files at regular intervals, committing partial results to the case branch. If the agent crashes, it resumes from the last committed checkpoint, not from the beginning.

This is standard practice in military operations: you report position at regular intervals so that if you go silent, the unit knows where you were and can send someone to continue the mission from that point.

Applied to agent development: partial results are committed incrementally. The worst-case loss is the work since the last checkpoint, not the entire task. This changes the economics of long-running agent operations from "all or nothing" to "progressive value accumulation."

---

*"The watch is set. SIGMA-1 through SIGMA-4, operational. Standing by for tasking."*
