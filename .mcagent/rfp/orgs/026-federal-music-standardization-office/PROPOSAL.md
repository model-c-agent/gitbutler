# Federal Music Standardization Office -- Technical Proposal

**Document:** FMSO-TM-2026-0007-P
**RFP:** `but-ai` Plugin for GitButler
**Classification:** Public
**Standards Reference:** FMSO-STD-0112 (Draft), FMSO-STD-0098

---

## 1. Scope

This proposal describes the FMSO Technology Modernization Task Force's recommended implementation of the `but-ai` plugin. The design prioritizes auditability, standards compliance, and traceability. Performance optimization is a secondary objective. The Office's position is that a slow, auditable system is preferable to a fast, opaque one.

---

## 2. Requirement 1: PATH-Based Plugin Architecture

### 2.1 Design

The plugin binary SHALL be installed to a directory in the system PATH. Discovery SHALL use the standard `which`/`where` mechanism. The binary SHALL respond to `but-ai --version` with its version string, build date, and FMSO-STD-0112 compliance level (one of: `full`, `partial`, `none`).

### 2.2 Configuration

Configuration SHALL be stored in `$XDG_CONFIG_HOME/but-ai/config.toml` (Unix) or `%APPDATA%/but-ai/config.toml` (Windows). The configuration schema SHALL be documented in a machine-readable format (JSON Schema) and validated at startup. Invalid configurations SHALL produce a diagnostic message referencing the specific schema violation.

### 2.3 Standards Traceability

Each plugin command SHALL be traceable to at least one requirement in FMSO-STD-0112. Commands that do not map to a standard requirement SHALL be documented as "extension" commands and clearly marked in `--help` output.

---

## 3. Requirement 2: Provider-Agnostic AI

### 3.1 Design

Provider abstraction SHALL be implemented as a trait with methods: `initialize`, `complete`, `complete_with_tools`, `estimate_tokens`, `report_usage`. The `report_usage` method is not present in most proposals -- we require it because FMSO-STD-0112 Section 4.3 mandates that AI systems SHALL report resource consumption to the operator.

### 3.2 Provider Registration

Providers SHALL be registered via configuration, not auto-detected. Each provider entry SHALL include: endpoint, model identifier, API key reference (NOT the key itself -- the key SHALL be retrieved from an environment variable or secret store), and a compliance flag indicating whether the provider's terms of service permit use in regulated workflows.

### 3.3 Fallback Policy

Provider fallback SHALL be explicit. Automatic failover is prohibited because switching providers mid-task changes the model and therefore changes the decision-making characteristics of the system. If the configured provider fails, the task SHALL fail with a diagnostic message. The operator decides whether to retry with a different provider.

---

## 4. Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

### 4.1 Patch Production

The agent SHALL produce a unified diff (INDEX.patch) and a commit message (COMMIT.msg) as separate files. The patch SHALL apply cleanly against the declared base commit. The commit message SHALL include: a summary line, a body describing the change, a `Standards-Ref:` trailer listing applicable standards, and an `Audit-Id:` trailer linking to the audit trail entry.

### 4.2 Patch Validation

Before submission, the patch SHALL pass:
1. Syntactic validation (`git apply --check`)
2. Content validation (no secrets, no binary files unless explicitly permitted)
3. Compliance validation (TM-AUDIT checklist)

### 4.3 Reproducibility

The Office does not require bit-identical reproducibility (we are regulators, not physicists). We require traceability: given a patch, an auditor SHALL be able to determine which model produced it, what context was provided, what token budget was allocated, and what memory entries were consulted. This audit chain is stored in the memory system.

---

## 5. Requirement 4: Polyrepo PR Coordination

### 5.1 Design

Cross-repository coordination SHALL use structured PR comments conforming to a published schema. The schema SHALL be versioned and backwards-compatible. The schema document SHALL be stored in the repository as `docs/schemas/coord-comment.json`.

### 5.2 Forge Abstraction

Forge-specific operations SHALL be isolated behind a `Forge` trait with methods: `create_pr`, `comment`, `read_comments`, `get_pr_status`. The trait SHALL NOT include forge-specific features (draft PRs, review requests, labels). Forge-specific features MAY be exposed through optional extension traits.

### 5.3 Dependency Management

Cross-repo dependencies SHALL be declared in coordination comments. Circular dependencies SHALL be rejected at declaration time. The dependency graph SHALL be stored in the memory system and SHALL be queryable by any agent.

---

## 6. Requirement 5: Agent Memory in Git Branches

### 6.1 Storage

Memory entries SHALL be stored as blobs in `refs/but-ai/memory/<namespace>/<key>`. Each entry SHALL include: creation timestamp, TTL, author agent ID, content type, and content body. Expired entries SHALL be garbage-collected on a configurable schedule (default: daily).

### 6.2 Audit Trail

The audit trail is a special memory namespace (`refs/but-ai/audit/`) with entries that have no TTL -- they are permanent. Each audit entry records: agent ID, action type, timestamp, input hash (SHA-256 of the task description and context), output hash (SHA-256 of the produced patch), budget consumed, and standards references.

### 6.3 Retrieval

Memory retrieval SHALL use keyword matching with configurable relevance thresholds. Retrieval results SHALL be logged in the audit trail, including which entries were retrieved and which were filtered.

---

## 7. Requirement 6: Signed Commits via OpenWallet

### 7.1 Design

All agent commits SHALL be signed using keys provisioned by an OpenWallet-compatible identity provider. Unsigned agent commits SHALL be rejected by the plugin. There is no override. This is non-negotiable.

### 7.2 Key Management

Signing keys SHALL have a maximum validity of 24 hours. Key rotation SHALL occur automatically before expiry. Compromised keys SHALL be revoked via the wallet's revocation mechanism, and all commits signed with the compromised key SHALL be flagged in the audit trail.

### 7.3 Verification

Any party with access to the wallet's public DID document SHALL be able to verify commit signatures without contacting the signing agent. Offline verification is a requirement -- the Office has learned from experience that verification mechanisms that require network access fail at the worst possible time.

---

## 8. Token Budget

| Agent | Input | Output | Total | Function |
|-------|-------|--------|-------|----------|
| TM-ARCH | 9,000 | 5,000 | 14,000 | Architecture & implementation |
| TM-AUDIT | 6,000 | 3,000 | 9,000 | Compliance review |
| TM-MEM | 5,000 | 1,500 | 6,500 | Memory & audit trail |
| TM-BUDGET | 3,500 | 1,500 | 5,000 | Resource control |
| **Team** | **23,500** | **11,000** | **34,500** | |

Overhead for audit trail logging is approximately 12% of total budget. The Office considers this a cost of accountability, not waste.

---

## 9. Unique Insight: Standards as Test Suites

The Office's contribution to this RFP is not primarily technical. It is methodological.

Every standard the Office publishes includes a conformance test suite -- a set of specific, measurable criteria that an implementation must satisfy to be compliant. FMSO-STD-0112 will include a test suite for AI agent workflows: does the agent produce auditable logs? Does it respect budget constraints? Does it sign commits? Can an auditor reconstruct the decision chain from the audit trail?

We propose embedding this test suite into the `but-ai` plugin as a self-assessment tool: `but-ai compliance check`. The command runs the FMSO-STD-0112 conformance suite against the current configuration and reports pass/fail/not-applicable for each requirement. This transforms a regulatory standard from an external imposition into an internal quality tool.

Standards are not constraints. They are automated tests for organizational behavior.

---

*FMSO-TM-2026-0007-P. Submitted per Administrative Procedure 12.1.*
*Public comment period: 60 days from publication date.*
