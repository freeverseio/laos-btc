# 🚀 Bitcoin + Polkadot: Integrating BRC721 for Mass NFT/RWA Tokenization with Dedicated Coretime

This proposal requests funding to complete the integration of the BRC721 protocol on the LAOS Parachain, enabling **trustless, bridgeless minting of NFTs and Real-World Assets (RWA) directly on the Bitcoin network, leveraging Polkadot's scalable consensus.**

BRC721 introduces a novel model that utilizes Polkadot's dedicated coretime as an efficient, parallel consensus layer to manage the creation, evolution, and security of millions of digital assets on Bitcoin at scale.

**Our Core Value Proposition for Polkadot:**
* **Massive Liquidity Bridge:** Connect Polkadot's ecosystem to Bitcoin's unparalleled liquidity and global recognition in a seamless, trustless manner.
* **Coretime Utility:** Provide a high-value, high-demand use case for Polkadot's underutilized coretime by offering an indispensable scaling solution for the world's largest blockchain.
* **Ecosystem Growth:** Attract new developers, projects (especially in RWA/Gaming), and users from the Bitcoin ecosystem to Polkadot's superior smart contract environment and interoperability.

---

## 🧐 Addressing Previous Community Feedback

We have carefully reviewed and incorporated the community's critical feedback from the prior submission to strengthen this proposal.

| Previous Concern | Action Taken in New Proposal |
| :--- | :--- |
| **BRC721 utility vs. Bitcoin Core v30/other protocols (Runes, Taproot Assets, Ordinals, RGB).** | Added a new section **"BRC721 Competitive Advantage"** to explicitly detail our unique value proposition, especially for RWA/Gaming, compared to alternatives. |
| **Lack of clear value/returns for DOT Treasury and ecosystem.** | Added a **"Tangible Benefits for Polkadot & KPIs"** section with clear, measurable Key Performance Indicators and a strategy for attracting users/liquidity. |
| **High ask and insufficient detail in expense breakdown.** | Reduced the total ask from **40,000 DOT to 32,000 DOT** and provided a **granular, staged expenditure breakdown** tied to milestones. |
| **Concerns about cross-chain indexing, reorgs, and security.** | Emphasized that BRC721's architecture, leveraging Polkadot's finality and LAOS's specialization in bridgeless minting, is designed to **mitigate these exact risks**, and added security milestones. |
| **No mechanism for the Treasury to recoup funds/commercial success sharing.** | Explicitly added a proposal for a **conditional clawback or token swap** mechanism linked to future commercial success milestones. |
| **Vague overall plan and lack of clear KPIs.** | Introduced a **Milestone-Based Deliverables** plan and specific, measurable KPIs. |
| **Lack of named collaborators/adoption pipelines and attracting BTC holders.** | Identified target *types* of initial partners and outlined a **focused user outreach strategy** aimed at Bitcoin developers and RWA/Gaming projects. |

---

## 📊 BRC721 Competitive Advantage

While Bitcoin Core v30 has increased the `OP_RETURN` limit, and protocols like Ordinals, Runes, Taproot Assets, and RGB exist, BRC721 offers unique, critical advantages for Polkadot and specific use cases:

* **Scalable Complex State & Evolution:** BRC721 uses Polkadot's coretime to manage complex, evolving NFT state (like leveling up a game character or asset collateral changes) *off-Bitcoin but secured by Bitcoin*. Alternatives are primarily static.
* **Bridgeless & Trustless:** Unlike traditional bridges, BRC721 ensures the NFT's ownership remains tied to a specific Bitcoin UTXO, with Polkadot acting as a permissionless, parallel validator for state changes. This is far more secure than cross-chain bridges.
* **Cost Efficiency for Mass Tokenization:** For RWA or gaming applications needing to mint millions of assets, writing all state data on Bitcoin (even with a 100kb `OP_RETURN` limit) remains prohibitively expensive and prone to fee spikes. BRC721 provides an **almost negligible cost** solution by utilizing Polkadot's consensus for the majority of the data/computation.
* **Direct Comparison:**
    * **Ordinals/Runes:** Primarily for fungible/static collections. Lack the ability for complex, evolving, and permissionlessly-managed state.
    * **Taproot Assets/RGB:** Excellent for fungible assets and simple transfers, but client-side validation makes them unsuitable for permissionless, large-scale, open ecosystems (like a public NFT marketplace or complex RWA platform) where a full set of validators (Polkadot) is required for shared, public consensus. BRC721 offers *shared, verifiable validation* leveraging Polkadot.

---

## 🎯 Tangible Benefits for Polkadot & KPIs

The successful integration of BRC721 will bring immediate and measurable value to the Polkadot ecosystem:

| Benefit | Key Performance Indicator (KPI) | Target by Q2 2026 |
| :--- | :--- | :--- |
| **Coretime Utilization** | Average Daily Transactions on LAOS Parachain via BRC721 minting/updates. | 10,000+ daily transactions |
| **Ecosystem TVL Growth** | Value of BTC-denominated assets secured by BRC721/LAOS architecture. | $5M+ equivalent secured value |
| **Adoption & Developers** | Number of unique Bitcoin UTXOs registered/linked to LAOS via BRC721. | 5,000+ unique UTXO holders |
| **Community Engagement** | Number of developers/projects actively building on BRC721/LAOS. | 5+ identified project integrations |
| **Treasury Recoupment** | *See Treasury Recoupment Section* | N/A (Based on commercial success) |

---

## 🛠️ Current State & Implementation Plan

1.  **Polkadot’s LAOS Parachain:** Specializes in Bridgeless Minting, currently providing coretime utility for EVM chains (Ethereum, L2s). This is the proven foundation for the BRC721 extension.
2.  **R&D Completed:**
    * Research paper published in the Cryptology Archive: [1]
    * Implementation of the first primitives in Rust: [4]

### Milestone-Based Deliverables (Q4 '25 - Q1 '26)

| Milestone | Deliverable | DOT Allocation | Completion Date |
| :--- | :--- | :--- | :--- |
| **M1: Indexer Alpha (Core)** | Completion of the BRC721-compliant dual-chain indexer (from 25% to 75% complete). Secure parsing of both Bitcoin and LAOS chains. Core logic for UTXO-to-NFT mapping. | 8,000 DOT | End of Q4 2025 |
| **M2: Core Protocol & Tooling** | BRC721 Protocol final implementation. Open-source Rust library for developers. Basic CLI tooling for testing minting/transfer. | 10,000 DOT | End of Jan 2026 |
| **M3: Full Indexer & Testnet Launch** | 100% completion of the indexer. Full Testnet deployment with a user-facing block explorer/UI showcasing minting/transfers. Comprehensive technical documentation. | 9,000 DOT | End of Feb 2026 |
| **M4: Mainnet Launch & Outreach** | BRC721 Mainnet Deployment on LAOS. Targeted outreach to 5+ specific Bitcoin/Gaming/RWA projects for pilot integration. | 5,000 DOT | End of Q1 2026 |
| **Total Request** | | **32,000 DOT** | **End of Q1 2026** |

---

## 💰 Funding Request & Detailed Breakdown

The total amount requested is **32,000 DOT** (a 20% reduction from the previous proposal). We are applying under the **Treasury $\rightarrow$ Medium Spender Track** as the amount remains within the typical range for significant ecosystem development. We will be active on Polkadot's community channels (e.g., Discord, Forum) to answer questions throughout the process.

| Category | DOT Allocation | Percentage | Detail |
| :--- | :--- | :--- | :--- |
| **Salaries (M1-M4)** | 26,000 DOT | 81.25% | Lead Dev (Alessandro Siniscalchi), Co-Author (Toni Mateos), 1x Hired Developer. (4 months @ $\approx 6,500$ DOT/month for 3 people). |
| **Infrastructure** | 3,000 DOT | 9.375% | Running dedicated Bitcoin Full Node, LAOS Parachain Indexer/Node costs, cloud hosting for indexer/API endpoint for pilot programs. |
| **Marketing & Outreach** | 3,000 DOT | 9.375% | Targeted outreach to Bitcoin developer/RWA/Gaming communities, technical content creation, and grants for early pilot projects using BRC721. |
| **TOTAL** | **32,000 DOT** | **100%** | |

### Treasury Recoupment Mechanism (Moral Hazard Mitigation)

To address the concern about poor fiscal precedent and a lack of return on investment, we propose the following **conditional commercial success share:**

* **Condition:** If, by Q4 2026, the cumulative total value of fees generated by the BRC721 protocol on LAOS (and thus contributed to the Polkadot ecosystem/treasury) exceeds **10,000 DOT**, we will initiate a proposal to **reimburse 10% of the original grant amount (3,200 DOT)** to the Polkadot Treasury.
* **Mechanism:** This reimbursement will be conducted via a subsequent Treasury proposal or a direct transfer, demonstrating a commitment to Polkadot's long-term financial health.

---

## 👥 The Team and Target Partners

The core team has a proven track record, including the creation of one of the earliest Layer-2s and the LAOS Parachain itself.

* **Alessandro Siniscalchi** (Lead Dev): [5]
* **Toni Mateos** (Co-Author): [6]
* **Alun Evans** (Business/Marketing): [7]

**Initial Target Partner Profile (Adoption Pipeline):**
We will focus our outreach on projects that *cannot* be served efficiently by other Bitcoin protocols:
1.  **Gaming Projects:** Studios requiring millions of NFTs with complex, mutable metadata (e.g., inventory, stats).
2.  **Real-World Asset (RWA) Tokenization:** Platforms needing to tokenize non-fungible, high-volume assets (e.g., property deeds, supply chain tracking) where security on Bitcoin is essential, but operational costs must be minimized.
3.  **Existing LAOS Partners:** Encouraging current LAOS users (EVM-compatible games) to expand their asset issuance to Bitcoin via BRC721.

## 🔗 References

[1] [Scalable Non-Fungible Tokens on Bitcoin](https://eprint.iacr.org/2025/641)

[2] About LAOS:
* [Main site](https://laosnetwork.io)
* [Resources](https://docs.laosnetwork.io/learn/resources), including whitepaper, developer docs, etc.
* [LAOS Parachain Repository](https://github.com/freeverseio/laos)

[3] [LAOS Medium](https://medium.com/laosnetwork), [Post About Gaming using LAOS](https://medium.com/laosnetwork/laos-network-lists-token-forges-partnership-with-sequence-to-bring-scalable-free-2-play-gaming-to-d49e56f7770f)

[4] [BRC721 Repository](https://github.com/freeverseio/laos-btc)

[5] [https://www.linkedin.com/in/asiniscalchi/](https://www.linkedin.com/in/asiniscalchi/), [https://github.com/asiniscalchi](https://github.com/asiniscalchi)

[6] [https://www.linkedin.com/in/toni--mateos/](https://www.linkedin.com/in/toni--mateos/)

[7] [https://www.linkedin.com/in/alun-evans/](https://www.linkedin.com/in/alun-evans/)