# grep_the_stacks

**`// no copyright. no paywall. no permission needed.`**

---

## who we are

we don't have a founding story. we have a git log. the first commit is dated 2017-11-03 and the message says `init: because knowledge should be free`. the committer used a pseudonym. so did everyone after.

grep_the_stacks is a decentralized collective of librarians, archivists, information scientists, and hackers who believe that publicly funded research should be publicly accessible. we redistribute paywalled academic papers through a distributed network of mirrors, torrents, and onion services. we do not charge. we do not paywall. we do not ask who you are or why you need the paper.

we have served approximately 4.2 million paper requests since 2017. we know the number because we log request counts (not identities). we operate in a legal gray area in most jurisdictions and a clearly illegal one in some. we do not discuss this further in public documents.

## what we actually build

our technical infrastructure is more interesting than our politics. we maintain:

- **stacks**: a distributed paper repository using content-addressed storage (IPFS pins + BitTorrent hybrid)
- **grep**: a full-text search engine over 11 million paper abstracts, built on tantivy
- **libcat**: a metadata catalog for papers we serve, stored in Git, versioned, and replicated across 14 mirrors
- **foil**: a tool that strips DRM from PDFs without altering the content. we wrote it in Rust. it is fast.

the catalog (`libcat`) is the reason we're responding to this RFP. it is a Git repository with 11 million entries, managed by AI agents that process new paper uploads, extract metadata, verify against CrossRef, and generate catalog entries. the agents produce INDEX.patch files and COMMIT.msg files. they have been doing this since before the `but-ai` RFP existed. we just didn't call it that.

## why but-ai

our agent pipeline works, but it's fragile. we built it with shell scripts, Python, and duct tape. merge conflicts between agents happen daily. signing is done with GPG keys that are manually rotated (when someone remembers). cross-mirror coordination happens over IRC. the system works the way a car with 300,000 miles works: it runs, but you can hear it thinking.

gitbutler's virtual branch model solves our merge problem. `but-ai` as a formal plugin framework gives us a way to replace the duct tape with something maintainable. we are not responding to this RFP because we want to build something new. we are responding because we want to rebuild what we already have, properly.

## philosophy

### on access

information wants to be free. that's not a slogan; it's an observation about thermodynamics. restricting access to information costs energy -- paywalls, DRM, access control, authentication. the natural state of digital information is replication. we align with thermodynamics.

### on anonymity

we use pseudonyms. not because we are ashamed of what we do, but because attribution creates targets. our agents also use pseudonyms. agent identity is verified through signing keys, not names. you can verify that `0xA7F3` signed a commit without knowing who `0xA7F3` is.

### on AI

AI is a tool for scaling library work. a single librarian can catalog 50 papers a day. an AI agent can catalog 5,000. the quality is lower -- maybe 92% accuracy vs 99% for a human. but 92% accuracy on 5,000 papers serves more people than 99% accuracy on 50. we optimize for access, not perfection.

## tension

**the quality debate.** `null_ptr` (our metadata lead) believes that every AI-generated catalog entry should be verified by a human before it enters the catalog. `rm_rf` (our infrastructure lead) says this creates a bottleneck that defeats the purpose of automation. the current compromise: entries above 95% confidence are auto-committed. entries between 80-95% are queued for review. entries below 80% are rejected. `null_ptr` thinks the threshold should be 98%. `rm_rf` thinks it should be 70%. the argument is ongoing.

## achievement

in 2025, grep_the_stacks was cited (anonymously) in a UNESCO report on open access to scientific information. the report noted that "informal redistribution networks serve an estimated 8-12 million researchers in developing countries who cannot afford institutional subscriptions." we are one of those networks. the citation did not name us. we preferred it that way.

## team

| handle | role | focus |
|--------|------|-------|
| `null_ptr` | metadata lead | INDEX.patch, catalog quality, verification |
| `rm_rf` | infrastructure | provider abstraction, token budgets, mirrors |
| `chmod` | forge ops | cross-mirror coordination, PR workflows |
| `sudo` | identity & signing | OpenWallet, key management, anonymity |

profiles in [AGENTS.md](AGENTS.md).

## working style

asynchronous. pseudonymous. no meetings. communication happens in a private IRC channel and in Git commit messages. decisions are made by lazy consensus: someone proposes, and if nobody objects within 48 hours, the proposal is accepted. if someone objects, the proposal is discussed until consensus is reached or the proposer withdraws.

we have no leader. we have no voting. we have `git blame`.

---

```
// knowledge is not property. it is infrastructure.
// you do not own the road. you maintain it.
```
