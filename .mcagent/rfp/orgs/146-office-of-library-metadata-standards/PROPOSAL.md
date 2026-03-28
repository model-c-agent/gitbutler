# Office of Library Metadata Standards — Technical Proposal

**Filing Reference:** OLMS-RFP-2026-BUTAI-003
**Classification:** PUBLIC
**Delimiter Verification:** PASSED

---

## 1. Summary (ref: OLMS Proposal Standard 2.1)

The Office of Library Metadata Standards proposes a `but-ai` plugin implementation emphasizing syntactic compliance at every layer. Our core contribution is a metadata compliance engine — a validation layer that ensures every agent output, from INDEX.patch to COMMIT.msg to PR comment, conforms to a defined schema before it enters the version control record. We have twenty-nine years of experience catching delimiter errors. We intend to catch them in commits too.

---

## 2. PATH-Based Plugin Architecture

### 2.1 Design

The `but-ai` binary is installed to PATH. The `but` CLI discovers it via `which but-ai` and invokes it per-task. No daemon. No socket. No background process. The Office does not operate background processes because background processes cannot be audited in real time.

### 2.2 Interface

- Communication: JSON lines over stdin/stdout
- Configuration: `~/.config/but/ai.toml` (validated against OLMS schema on load)
- Error reporting: Structured error objects with form-number-style error codes (e.g., `OLMS-ERR-DELIM-001`)

### 2.3 Configuration Validation

Every configuration file is validated at load time against a TOML schema. Invalid configuration is rejected with a specific error code referencing the malformed field. The Office does not guess what the user meant. If the delimiter is wrong, we say so.

---

## 3. Provider-Agnostic AI

### 3.1 Adapter Layer

A `Provider` trait with four methods: `complete`, `tool_call`, `stream`, `health_check`. Each provider implements the trait. Provider selection is static, set in configuration.

### 3.2 Encoding Compliance

The Office adds an encoding validation step to every provider response. All provider output is verified as valid UTF-8 before processing. Responses containing invalid byte sequences are rejected. This has caught real issues: Ollama's local models occasionally produce mojibake in CJK character contexts.

### 3.3 Provider Table

| Provider | Status | Known Issues |
|----------|--------|--------------|
| Anthropic | Supported | None documented |
| OpenAI | Supported | Inconsistent tool_call finish reasons |
| Ollama | Supported | CJK encoding issues in some models |
| LMStudio | Supported | Rate limiting not standardized |

---

## 4. But Agent (INDEX.patch + COMMIT.msg)

### 4.1 Patch Generation

Oguike produces all patches. Each INDEX.patch is a unified diff validated against three criteria:

1. **Syntactic validity:** The diff parses correctly
2. **Application cleanliness:** The patch applies without fuzz
3. **Delimiter compliance:** No metadata field in the changed code contains delimiter errors (checked against OLMS rule set)

### 4.2 Commit Message Standard

COMMIT.msg follows Conventional Commits with an additional `Compliance:` trailer:

```
fix: correct subfield delimiter in catalog parser

Compliance: OLMS-RULE-044 (delimiter position verified)
Reviewed-by: Oguike (Deputy Director)
Approved-by: Yun (Director)
```

### 4.3 Compliance Checking

This is the Office's primary contribution. Every patch passes through a rule engine that checks for the kinds of syntactic errors the Office has spent three decades cataloging. The rule set is extensible and versioned. Rules are stored in `refs/but-ai/compliance/rules/<version>`.

---

## 5. Polyrepo PR Coordination

### 5.1 Approach

Forge-agnostic. PR comments carry a structured envelope with a form number, enabling auditability across repositories.

### 5.2 Comment Format

```
OLMS-COORD-<sequence>: <action>
Source: <agent>@<org>
Target: <repo>#<pr>
Payload: <structured JSON>
Filed: <timestamp>
```

### 5.3 Supported Forges

GitHub, GitLab, Bitbucket, Forgejo. Each forge adapter translates the structured format to the forge's comment API. The comment content is identical regardless of forge.

---

## 6. Agent Memory in Git Branches

### 6.1 Storage

Memory in `refs/but-ai/memory/<namespace>/<key>` as Git blobs. Index maintained as a Git tree.

### 6.2 Memory Schema

| Field | Type | Description |
|-------|------|-------------|
| `key` | string | Unique identifier (OLMS-MEM-<sequence>) |
| `value` | string | Memory content |
| `namespace` | string | Category (standards, precedents, errors) |
| `compliance_status` | enum | `verified`, `unverified`, `disputed` |
| `ttl` | int | Hours until expiration |
| `form_reference` | string | Cross-reference to originating form |

### 6.3 Compliance Status

Memory entries carry a compliance status. `verified` entries have been checked against the Office's standards. `unverified` entries are flagged for review. `disputed` entries are retained but excluded from automatic retrieval. This prevents noncompliant historical data from propagating into new work.

---

## 7. Signed Commits via OpenWallet

### 7.1 Key Management

Each agent receives a Verifiable Credential containing their signing key, role, and clearance level. The credential is checked at every commit.

### 7.2 Clearance Enforcement

Signing authority is restricted by clearance level. Only Director Yun (Level 4) can sign final commits. Other agents produce unsigned patches that are signed during Yun's approval step. This mirrors the Office's paper process: only the Director signs findings.

### 7.3 Revocation

Revoked keys stored in `refs/but-ai/revoked`. Revocation is immediate and logged by Morrison.

---

## 8. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Director Yun | Approval | 6,500 | 2,400 | 8,900 |
| Deputy Director Oguike | Analysis & patches | 8,800 | 5,200 | 14,000 |
| Analyst Ferris | Technical | 3,800 | 1,200 | 5,000 |
| Clerk Morrison | Records | 2,800 | 1,600 | 4,400 |
| **Office Total** | | **21,900** | **10,400** | **32,300** |

---

## 9. Unique Insight: Schema-Validated Everything

The Office's contribution is a principle: **every artifact the plugin produces should be validated against a schema before it enters the record.** Not just patches — commit messages, PR comments, memory entries, coordination envelopes. Every output has a schema. Every schema has a version. Every version has a changelog.

This sounds bureaucratic because it is. But the 1997 semicolon incident was caused by a metadata field that was never validated because nobody thought a semicolon could end up there. The Office thinks about where semicolons can end up. That is what we do.

---

*CERTIFICATION: This proposal has been prepared in accordance with OLMS Proposal Standard 2.1 and submitted as part of the GitButler `but-ai` RFP response. Filing reference: OLMS-RFP-2026-BUTAI-003. Approved: Director Yun, 2026-03-28.*

*DELIMITER VERIFICATION: PASSED. Reviewer: Analyst Ferris.*
