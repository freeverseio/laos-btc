# Bitcoin + Polkadot: Completing the BRC-721 Integration to Scale Bitcoin NFTs via Polkadot Coretime

Referendum type: Treasury Spend (Medium Spender)

Requested amount: **140,000 USDC** (from Polkadot Treasury via Asset Hub)

Purpose: Complete the open-source BRC-721 integration stack so Bitcoin-native NFTs can scale using Polkadot(via LAOS + coretime) as the deterministic computation, validation, and metadata layer.

## 1) Summary

This proposal funds the final integration work required to make **BRC-721 production-ready** for the Polkadot ecosystem.

BRC-721 enables **Bitcoin-native NFTs** that remain on Bitcoin (no wrapped assets, no custodial bridge) while using Polkadot as the scalable layer for deterministic validation, metadata evolution, indexing primitives, and developer tooling.

The result is an ecosystem primitive that makes Bitcoin NFT and RWA-style assets viable at scale for applications that require:
- Millions of assets
- True metadata with evolution
- Verifiable rules and provenance
- Application-grade indexing and developer UX

All deliverables will continue to be **fully open-source**.

**Current Status**

The technical implementation of BRC-721 has been started by the LAOS team and is currently at 70% completion. 
We have prepared several videos, documents, and AI-assistants to demonstrate the current status of development:

- [Introduction to BRC-721 video]()
- [Current technical implementation video]()
- [Text walkthough/demo]()
- [Open ChatGPT Explainer]()

## 2) Benefits for the Polkadot ecosystem

Polkadot Treasury spending should create durable, compounding value for DOT holders. This proposal does that in four concrete ways:

### 2.1 Increased demand for Coretime (Polkadot 2.0 economics)

BRC-721 uses Polkadot as the computation and metadata layer. As adoption grows, the indexer/validation workload and protocol usage create **ongoing demand for coretime** (directly or via chains/services that procure it), supporting the long-term economics of Polkadot.

**Measurable signal:** post-launch, we will track coretime usage attributable to BRC-721 workloads and publish a  usages report (where measurable).

### 2.2 A credible “Bitcoin interoperability” narrative with real builders

Bitcoin remains the largest liquidity network in crypto. BRC-721 gives Polkadot a *specific*, developer-usable wedge into Bitcoin:

- Not “bridging marketing” but actual tooling and infrastructure developers can use.
- A clear value proposition: Bitcoin-native assets + Polkadot-grade compute/metadata.

This helps Polkadot compete in the interoperability landscape with a differentiated story: **Polkadot as the computation layer for Bitcoin-native assets**.

**Measurable signal:** number of external Bitcoin ecosystem teams running the indexer and integrating wallet libraries within 120 days of release.

### 2.3 Reusable infrastructure for the wider Polkadot ecosystem

The deliverables are designed to be reusable beyond BRC-721:

- Reorg-safe event indexing patterns
- Deterministic validation pipelines
- SDK packaging for parachains/dApps
- Documentation templates and reference flows

This reduces time-to-market for other cross-chain or UTXO-adjacent projects and creates common tooling the ecosystem can build on.

**Measurable signal:** number of independent forks/adoptions of the libraries and indexer repos.

### 2.4 Open-source public good (no vendor lock-in)

All work is released under permissive licenses with reproducible builds and public CI. That means:
- Any team can fork, run, or maintain the stack.
- Treasury funding creates a public good, not a dependency.

**Measurable signal:** third-party deployments of the indexer and SDK.

### 2.5 Why Treasury (vs private funding)

This is infrastructure that benefits the ecosystem broadly, not just one product. Treasury is the appropriate mechanism to fund:
- Open-source protocol integrations
- Developer tooling and reference implementations

## 3) Why Polkadot? Why now?

### 3.1 Bitcoin policy changes do not remove NFT/RWA constraints

Recent Bitcoin relay policy changes have increased default limits for certain data-carrying outputs, but Bitcoin still does not provide:
- General-purpose computation for NFT/RWA logic
- A rich, canonical metadata layer
- Application-grade scalability for millions of evolving assets
- Deterministic indexing primitives required by developers

### 3.2 Polkadot’s unique value proposition

Polkadot (and coretime) is built for parallel execution and modular infrastructure. BRC-721 leverages this to:
- Offload computation and deterministic validation to Polkadot
- Maintain an on-chain canonical metadata/evolution layer
- Provide developer-grade UX via SDKs, indexers, and reference integrations

## 4) What already exists

The technical implementation of BRC-721 has been started by the LAOS team and is currently at 70% completion. 

- [LAOS parachain](https://laosnetwork.io) is live, stable, and in production. LAOS's Bridgeless Minting allows NFTs to minted on any EVM chain, while spending only gas in LAOS. The solution greatly scales NFT minting and is already being used by several web3 games.
- [BRC-721 research/spec](https://eprint.iacr.org/2025/641) exists publicly.
- Core primitives already implemented:
  - UTXO parsing
  - Taproot-related logic (as required by the protocol)
  - Collection logic
  - Early tooling
- [Current technical implementation video](https://youtu.be/1Yt_LHAr8z0)
- [Text walkthough/demo](https://medium.com/laosnetwork/bitcoin-demo-of-the-new-brc-721-protocol-87e33e26321d)
- [Open ChatGPT Explainer](https://chatgpt.com/g/g-6935d265a9bc8191b00c390933e5d73c-brc721-guru)

## 5) What this proposal delivers

All deliverables will be released under **MIT or Apache-2.0** (final list in Appendix A), with public repositories and reproducible builds.

### 5.1 Core Deliverables

1) **Bitcoin ↔ LAOS dual-chain indexer (production-grade)**
- Reorg handling up to configurable depth
- Deterministic rollback and replay
- Exportable event stream / database schema

2) **Complete BRC-721 validation logic**
- Full protocol rules coverage
- Published test vectors and adversarial cases
- “Known-good” reference implementation

3) **Wallet + developer libraries**
- Minimal, auditable libraries for constructing/validating BRC-721 transactions
- Reference integration guidance for wallet providers

4) **Polkadot SDK package for parachain / dApp teams**
- Clear integration path for Polkadot builders
- Templates + example flows

5) **Reference dApp + documentation**
- Demonstrates mint → index → verify → trade/list (as applicable)
- End-to-end developer documentation

### 5.2 Outreach (developer adoption)

- 3 workshops targeting Bitcoin developers
- Tutorials and technical write-ups
- Office hours for early integrators

## 6) Execution plan and milestones

**Target completion:** end of **Q2 2026**

### Milestone 1 (Weeks 1–8): Indexer foundation + reorg engine

**Criteria**
- Reorg simulation harness passes up to **200 blocks**
- Deterministic rollback verified (repeatable state from the same inputs)
- Indexing latency target: **< 2 seconds** for new events under normal conditions
- Public repository with:
  - CI tests
  - Release instructions
  - Operator documentation

### Milestone 2 (Weeks 9–14): Full validation + SDK packaging

**Criteria**
- Protocol rule coverage documented and complete
- Comprehensive test vectors published (including malformed/adversarial cases)
- SDK packaged and released with example integration project
- Reference dApp alpha available on a public test environment

### Milestone 3 (Weeks 16–20): Mainnet readiness

**Acceptance criteria**
- Mainnet readiness checklist completed and published
- Adoption outputs delivered:
  - 3 workshops (recordings published)
  - Tutorial series published
  - Presentation at relevant conference/event

## 7) Budget request (122,000 USDC) with itemization

**Total requested:** **122,000 USDC**  

Breakdown:

- **Engineering (1 full-time engineer, 1 part-time engineer, 20 weeks): 78,000 USDC**
  - Indexer/reorg + validation + SDK + CI/release engineering
- **Infrastructure (nodes, storage, monitoring, CI runners): 8,000 USDC**
- **Documentation + developer relations + workshops + events: 30,000 USDC**
- **Contingency (unspent returned to Treasury): 6,000 USDC**

## 8) Accountability, reporting, and controls

### 8.1 Public reporting

- Weekly progress updates on Polkassembly, plus GitHub links and on the LAOS Medium blog
- Public dashboard: milestones, commits, releases, tests, open issues
- “Scope lock” snapshot at referendum start:
  - Git tag / commit hash
  - IPFS hash of the scope and budget document

### 8.2 Return-of-unspent commitment

- Any unspent funds will be returned to the Treasury within **30 days** of completion.
- A public final accounting report will be published.

## 10) KPIs (measurable, time-bound)

### 10.1 Technical KPIs (by end of Milestone 3)

- Dual-chain indexer runs continuously with reorg simulations passing
- Full validation rules implemented with test coverage and published vectors
- SDK released + reference dApp demonstrating end-to-end flow
- Mainnet readiness checklist completed and published

### 10.2 Ecosystem KPIs (within 120 days post-release)

- 2 parachain or Polkadot teams prototyping SDK usage
- 2 external users running the indexer (self-hosted)
- 3 workshops delivered; recordings published
- 4 technical posts + tutorial series published

## 11) Conclusion

This proposal funds a well-scoped, last-mile integration that positions Polkadot as the **computation + metadata layer for Bitcoin-native NFTs** without custodial bridging.

It is designed to be easy to evaluate:
- Clear milestones and acceptance criteria
- Stablecoin budgeting
- Transparent reporting and controls
- Fully open-source deliverables

## Appendices

#### Appendix A: References

- [LAOS parachain](https://laosnetwork.io)
- [BRC-721 research/spec](https://eprint.iacr.org/2025/641).
- [Introduction to BRC-721 video (AI)](https://youtu.be/vUzTmcARSQI)
- [Current technical implementation video](https://youtu.be/1Yt_LHAr8z0)
- [Text walkthough/demo](https://medium.com/laosnetwork/bitcoin-demo-of-the-new-brc-721-protocol-87e33e26321d)
- [Open ChatGPT Explainer](https://chatgpt.com/g/g-6935d265a9bc8191b00c390933e5d73c-brc721-guru)

#### Appendix B: Team Bios

#### Dr. Toni Mateos

[LinkedIn](https://www.linkedin.com/in/toni--mateos/)
Dr. Toni Mateos leads research at LAOS Network. He was recently awarded an Oscar for Scientific & Technological Achievements from the Academy of Motion Picture Arts and Sciences, as co-creator of Dolby Atmos, from research to product, a tech that has reached ~1Bn users in 90+ countries. 

He was Director of Research at Dolby Laboratories for 7 years. In 2019, he left Dolby to co-found Freeverse, a blockchain R&D company, with the vision of using blockchain technology to bring digital ownership to the mainstream.

Toni holds a degree in Quantum Gravity (U. Barcelona), a PhD in Mathematical Physics (String Theory, Imperial College London), a Postgrad in Blockchain Technologies (Polytechnic University of Catalonia), has 17 years of experience leading R&D teams in the entertainment tech industry, and is author of 30+ patents in various technological fields. 

#### Alessandro Siniscalchi

[LinkedIn](https://www.linkedin.com/in/asiniscalchi/)
Alessandro is the Lead Engineer at LAOS Network, bringing over 20 years of experience deploying mission-critical systems across domains including blockchain, industrial control for space observatories, military avionics, cinema sound, and audio signal processing. He is highly experienced in system design, software quality, and agile development practices.

Alessandro became a Polkadot protocol developer in 2022 after graduating from the inaugural Polkadot Blockchain Academy and has been a key contributor to the Polkadot ecosystem ever since.

#### Dr. Alun Evans

[LinkedIn](https://www.linkedin.com/in/alun-evans/)  
Dr. Alun Evans has over 20 years of experience in the tech industry for entertainment. Alun has a passion for building teams with a strong collaborative culture, that are focused on creating products that solve genuine problems. Previously, he was CEO of Shar3d.io (collaborative 3D applications on the web), CTO of Bodypal.com (virtual garment and fitting service), and Director of Barcelona World Race - THE GAME, the first-ever video game that allowed players to compete in a simultaneous real-world sporting event.  Alun has a Ph.D. in Medical Physics from University College London.



