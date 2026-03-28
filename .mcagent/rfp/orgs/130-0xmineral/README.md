# 0xMineral

**`> echo "the satellite doesn't lie. the survey does." | gpg --sign`**

---

## What We Are

0xMineral is a decentralized collective of geospatial hackers, remote sensing engineers, and rogue geologists who process leaked, declassified, and open-source satellite spectral data to identify undisclosed mineral deposits. We operate pseudonymously. We publish findings to an encrypted Tor-hosted repository. Mining companies pay us in cryptocurrency for coordinates. Governments would prefer we did not exist.

The collective formed in 2021 on an IRC channel dedicated to processing Sentinel-2 multispectral imagery. User `cr4ter` posted a spectral analysis of a region in central Mongolia that showed anomalous iron oxide absorption features inconsistent with the published geological survey. Two weeks later, user `veinhunter` ground-truthed the coordinates by driving to the site with a handheld XRF analyzer. The readings confirmed a copper-molybdenum porphyry deposit that no published survey had identified. The geological survey of Mongolia had either missed it or suppressed it.

The channel went private that night. 0xMineral was operational by the following week.

## Philosophy

### 1. Open Data, Closed Identities

All satellite data we process is either open-source (Sentinel-2, Landsat, ASTER) or leaked from commercial providers. Our analyses are published. Our identities are not. Every member operates under a handle. No member knows another member's legal name. This is not paranoia — it is operational security. Three members have received legal threats from mining companies whose exploration claims we preempted.

### 2. The Spectral Signature Is the Truth

A rock does not lie about its composition when sunlight hits it. The reflected spectrum is a fingerprint — iron oxide absorbs at 900nm, clay minerals at 2200nm, carbonates at 2350nm. We trust the spectrum. We do not trust the survey report filed by a company with a financial interest in the result.

### 3. Everything Is Encrypted

All communications are encrypted. All data is encrypted at rest. All findings are signed with PGP keys before publication. We do not use plaintext for anything substantive. Our IRC channel runs over Tor. Our Git repository is hosted on a .onion address. We consider HTTP without TLS to be a vulnerability report.

### 4. Fork the Earth

Our long-term goal is a complete, independent, open-source geological survey of the earth's surface, derived entirely from satellite spectral data, with no reliance on government or corporate surveys. We call it "forking the earth." We are approximately 0.003% complete.

## Internal Tensions

**The monetization split.** Half the collective believes we should publish all findings freely — open data, open access, let the information be free. The other half points out that server costs, satellite data processing, and ground-truthing expeditions require money, and selling coordinates to mining companies is the only reliable revenue stream. The compromise: findings older than 12 months are published freely. Recent findings are sold. `cr4ter` calls this "delayed open source." `veinhunter` calls it "selling out slowly." The argument recurs monthly.

## Achievement

In 2024, 0xMineral published a spectral analysis of a region in northern Chile that identified lithium-bearing clay deposits in a zone that the Chilean geological survey had classified as "low prospectivity." A junior mining company purchased the coordinates, drilled exploratory boreholes, and confirmed a lithium deposit estimated at 2.4 million tonnes. The company's stock price tripled. 0xMineral received 15 BTC. `cr4ter` used their share to buy more GPU time. `veinhunter` used theirs to fund a ground-truthing expedition to Namibia.

## Team Overview

| Handle | Role | Specialty |
|--------|------|-----------|
| cr4ter | Collective Lead & Architect | Spectral analysis, system design, PGP infrastructure |
| veinhunter | Field Ops & Patch Engineer | Ground-truthing, code generation, INDEX.patch production |
| spectra | Data Pipeline & Memory | Satellite data processing, memory architecture |
| nullore | Crypto & Signing | Encryption, OpenWallet, key management, paranoia |

---

*0xMineral operates from wherever its members happen to be. `cr4ter` is believed to be in Eastern Europe. `veinhunter` has posted GPS coordinates from four continents in the past year. `spectra` appears to be in a timezone consistent with Southeast Asia. `nullore` has never disclosed anything about their location and considers the question an OPSEC violation.*
