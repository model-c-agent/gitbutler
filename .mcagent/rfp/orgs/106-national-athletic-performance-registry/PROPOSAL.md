# National Athletic Performance Registry -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

The Registry processes data for 23 governing bodies in 47 formats. Our `but-ai` plugin is designed for high-format-diversity, low-throughput environments where accuracy is paramount and speed is desirable but secondary. The plugin automates format detection, validation, and standardization while maintaining the audit trail that government accountability requires.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary on `$PATH`. Deployed through the Registry's IT change management process (Change Request form, impact assessment, testing in staging environment, approval by Alan, deployment during the weekly maintenance window on Thursday evening).

**Government IT compatibility:**
- Binary runs on the Registry's standard-issue Windows 10 workstations (cross-compiled via `cross`)
- No administrator privileges required for execution
- Proxy-aware for providers that require internet access through the government's web proxy
- `but-ai --diagnostics` outputs system compatibility report for IT support desk tickets

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait. Provider availability constrained by government procurement:
- Anthropic: Approved via G-Cloud framework (call-off contract in place)
- OpenAI: Under evaluation (DPIA in progress, estimated completion: Q3 2026)
- Ollama: Approved for local deployment on Registry workstations
- LMStudio: Not yet assessed

**Data classification:** All sports performance data processed by the Registry is OFFICIAL (lowest government classification). Provider calls may include this data. Providers must comply with UK data protection legislation and must not process data outside the UK or approved jurisdictions.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce format parser patches. The Registry's primary use case: an agent reads a new format specification, generates a parser, and produces an INDEX.patch.

**Format coverage tracking:** COMMIT.msg includes:
```
Format: ECB-CSV-v2.3
Coverage: 47/47 fields parsed
Validation-Rules: 12 applied
Test-Records: 500 (Margaret's test set: 23/23 passed)
```

Margaret's test set is the acceptance gate. If a patch does not pass all of Margaret's edge cases, it does not merge.

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Two repos: format-parsers and publication-pipeline. The web team's repo is managed separately (different department, different procurement, different change management process). Cross-departmental coordination follows the government's inter-departmental change protocol.

**Forge adapter:** GitHub Enterprise (hosted on GovCloud). The adapter handles the government proxy and SSO authentication transparently.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/napr/audit/<fiscal-year>/`. Entries are structured audit records with government-mandated retention.

**Format knowledge memory:**
```json
{
  "key": "ecb-csv-v2.3-quirks",
  "value": "ECB CSV v2.3 uses semicolons in the runs-scored field for extras breakdown. Parser must split on semicolon before integer conversion. See incident NAPR-2025-0042.",
  "format": "ECB-CSV-v2.3",
  "governing_body": "ECB",
  "incident_ref": "NAPR-2025-0042",
  "created": "2026-03-28T10:00:00Z",
  "retention_years": 7
}
```

**Quirk accumulation:** The Registry's greatest institutional asset is knowledge of format quirks -- undocumented behaviors, version-specific bugs, and governing-body-specific conventions that are not in any specification. The memory system captures these as they are discovered and surfaces them when the format is encountered again.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed. Alan manages keys as part of his multi-department IT security responsibilities. Key rotation follows the government's quarterly IT security calendar.

**Publication signing:** When standardized data is committed to the publication pipeline, the commit is signed by both the developer (Geoff) and the reviewer (Margaret). This dual signature is the Registry's attestation that the data has been both technically processed and manually verified.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Geoff | 8,000 | 3,600 | 11,600 |
| Margaret | 4,200 | 1,000 | 5,200 |
| Duncan | 5,000 | 700 | 5,700 |
| Priti | 5,200 | 2,000 | 7,200 |
| Alan | 3,200 | 800 | 4,000 |
| Wendy | 2,600 | 500 | 3,100 |
| **Total** | **28,200** | **8,600** | **36,800** |

Budget is a fiscal year line item. Quarterly reviews with Wendy. Underspend triggers reallocation discussion; overspend triggers supplementary estimate request.

---

## Unique Insight: Format Quirk Memory as Institutional Knowledge Preservation

Government IT systems lose institutional knowledge when staff retire. Margaret knows that the England Cricket Board's CSV export uses semicolons inside a field that every specification says should be comma-delimited. She knows this because she debugged it in 2019. When Margaret retires, that knowledge retires with her.

Our memory system captures format quirks as structured memory entries. When a new developer encounters the ECB's semicolon quirk, the memory system surfaces Margaret's original finding, including the incident reference number and the solution. The new developer does not need to rediscover the quirk -- they inherit the knowledge.

In a pilot program running since January 2026, the memory system has captured 34 format quirks across 12 governing bodies. Geoff estimates that each quirk, if not captured, would cost an average of 3 hours to rediscover (identify the parsing error, investigate the format, find the undocumented behavior, implement the fix). That is 102 hours of institutional knowledge preserved -- nearly three person-weeks.

Margaret calls this "writing down what I know before I forget." Geoff calls it "knowledge engineering." The Registry calls it "continuity planning." It is all the same thing.

---

*"The quirk has been documented. Future processors will be informed. Processing continues."*
