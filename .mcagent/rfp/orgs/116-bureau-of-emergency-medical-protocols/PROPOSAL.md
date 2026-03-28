# Bureau of Emergency Medical Protocols — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

*Document Reference: BEMP-RFP-2026-001*
*Classification: External — For Public Distribution*
*Approved: Director E. Halverson, 2026-03-27*

---

## Executive Summary

The Bureau proposes a `but-ai` reference implementation that prioritizes **auditability, traceability, and compliance**. Every agent action is logged. Every decision is traceable. Every output is documented. Our implementation may not be the fastest, but it will be the most auditable, and we consider this the correct priority ordering for production AI agent systems.

---

## Requirement 1: PATH-based Plugin Architecture

Standard PATH-based binary with compliance tooling.

**Design:**
- Binary: `but-ai`, statically linked, version-stamped with build metadata
- Commands: `but ai patch`, `but ai audit`, `but ai certify`, `but ai memory`, `but ai forms`
- Config: `~/.config/but-ai/bemp.toml`
- `but ai certify <run-id>` validates that an agent run's audit trail complies with BEMP-001
- `but ai forms list` displays required documentation forms for a given task type
- All commands write to a structured log at `~/.local/share/but-ai/audit.log`

---

## Requirement 2: Provider-Agnostic AI

Provider abstraction with **mandatory provider certification**.

**Certification process:**
1. Provider is registered in config
2. `but ai certify-provider <name>` runs the certification test suite (47 tests)
3. Results are stored in `refs/bemp/certifications/<provider>`
4. Only certified providers can be used for production tasks
5. Re-certification required every 90 days or after provider API changes

**Architecture:**
- Provider trait: standard invoke/stream with certification metadata
- Supported: OpenAI, Anthropic, Ollama, LMStudio (all require certification)
- Fallback: only to certified providers; uncertified providers are never used regardless of availability

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Patch generation follows a **documented specification-implementation-validation cycle**.

**Cycle:**
1. **Specification** — Agent reads task and generates a Feature Specification (Form 22-C equivalent): what will change, why, and what the expected impact is
2. **Implementation** — Agent generates INDEX.patch conforming to the specification
3. **Validation** — Patch is checked against specification and compliance test suite
4. **Documentation** — COMMIT.msg includes specification reference, compliance check results, and audit trail reference
5. **Approval** — Patch enters review queue (automated for routine, manual for critical)

**COMMIT.msg format:**
```
feat: add retry logic to API client

Specification: BEMP-RFP-2026-001-SPEC-042
Compliance: PASSED (47/47 tests)
Audit-Trail: refs/bemp/audit/run-2026-03-28-001
Reviewed-By: automated (routine classification)
```

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination with **full documentation trail**.

**Protocol:**
- PR comments: `<!-- bemp:coord:{action}:{ref}:{payload} -->`
- Every coordination action is referenced by a document number
- Actions: `spec` (post specification), `ack` (acknowledge specification), `validate` (report validation results), `approve` (approve for merge)

**Documentation requirement:** Cross-repo changes require a Cross-Repository Impact Assessment (Form 18-A) filed in the lead repository before coordination begins.

**Forge adapters:** GitHub, GitLab, Gitea, Forgejo. Standard trait. All adapter implementations must pass certification.

---

## Requirement 5: Agent Memory in Git Branches

Memory stored as **certified knowledge entries** with formal provenance.

**Entry requirements:**
- Every memory entry must cite its source (commit SHA, file path, or simulation ID)
- Every entry must have a confidence score
- Every entry must have a review status: `draft`, `reviewed`, `certified`
- Only `certified` entries are injected into agent context by default

**Storage:** `refs/but-ai/memory/certified/<domain>/<key>`

**Certification process for memory:** A memory entry is promoted from `draft` to `reviewed` after 3 consistent observations. It is promoted from `reviewed` to `certified` after manual review (or automated review for routine patterns).

**Expiration:** Draft: 14 days. Reviewed: 60 days. Certified: 180 days. Permanent entries require Form 35-B (Permanent Knowledge Registration).

---

## Requirement 6: Signed Commits via OpenWallet

Signing governed by **Bureau Security Policy BEMP-SEC-003**.

**Policy requirements:**
- All agent commits must be signed
- Signing keys must be generated through a documented ceremony
- Key rotation must occur every 30 days (documented on Form 29-A)
- Key revocation must be logged with justification (Form 29-B)
- Signature verification must be possible offline (no network dependency)

**Implementation:** Standard OpenWallet integration. VC includes: agent identity, certification status, audit trail reference, and policy version under which the commit was signed.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Halverson | Approval | 1,500 | 300 | 1,800 |
| Mostafa | Compliance | 5,500 | 1,800 | 7,300 |
| Thorne | Implementation | 9,000 | 5,000 | 14,000 |
| Mensah | Documentation | 4,000 | 2,500 | 6,500 |
| Chadha | Validation | 5,500 | 2,000 | 7,500 |
| Diaz | Security | 3,200 | 800 | 4,000 |
| **Total** | | **28,700** | **12,400** | **41,100** |

### Documentation Overhead Analysis

| Category | Tokens | % of Total |
|----------|--------|-----------|
| Code (patch + implementation) | 24,770 | 60% |
| Documentation (specs, comments, forms) | 10,450 | 25% |
| Compliance (audit, certification) | 5,880 | 15% |

### Scaling

| Classification | Description | Budget |
|----------------|-------------|--------|
| Minor amendment | Small change, existing spec | 20,550 (0.5x) |
| Standard feature | New feature, new spec | 41,100 (1.0x) |
| Major feature | Multi-repo, impact assessment | 82,200 (2.0x) |
| System change | Architecture, full committee review | 123,300 (3.0x) |

---

## Unique Insight: Documentation as the Primary Artifact

Most agent systems treat code as the primary output and documentation as an afterthought. We invert this. The specification comes first. The code implements the specification. The tests validate the implementation against the specification. The audit trail records the entire process.

This means our agents can answer a question that most cannot: "Why does this code exist?" Not "what does it do" — any code reader can answer that. But "why was this specific approach chosen, what alternatives were considered, and who approved the decision." The specification captures intent. The audit trail captures process. The code is merely the final expression of a documented decision.

This is expensive. We spend 40% of our token budget on documentation and compliance. We consider this investment, not overhead. A system that cannot explain itself is a system that cannot be trusted, and a system that cannot be trusted should not be deployed in production.

The Bureau has been accused of moving slowly. This is accurate. We have never been accused of shipping software we could not explain. We consider this the more important metric.

---

*Filed under BEMP-RFP-2026-001.*
*"The process is the product."*
