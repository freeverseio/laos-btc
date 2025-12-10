# Bitcoin + Polkadot: Completing the BRC721 Integration to Scale Bitcoin NFTs via Polkadot Coretime

## 1. Summary

This proposal requests targeted funding to complete the integration of the BRC721 protocol, enabling bridgeless, scalable Bitcoin NFTs powered by Polkadot coretime — with a clear execution plan, costed milestones, ecosystem KPIs, and measurable benefits to DOT holders.

The work completes an open-source infrastructure that lets developers mint and manage Bitcoin-native NFTs while using Polkadot as the scalable computation and metadata layer. Unlike inscription-based approaches, BRC721 supports millions of assets, true metadata, verifiable evolution, and fully on-chain permanence — making it suitable for gaming, RWAs, and enterprise applications.

## 2. Why Polkadot? Why Now?

### 2.1 Relevance after Bitcoin Core v30

Bitcoin Core v30 removed certain data limitations, but it does *not* solve essential constraints for large-scale NFT or RWA deployments:

- Bitcoin provides no general-purpose computation  
- No permanent metadata layer  
- No way to scale millions of assets  
- High fee volatility remains  
- Inscription formats remain difficult to index and maintain

BRC721 uses Polkadot as the offloaded consensus and computation layer while keeping assets natively on Bitcoin. This remains valuable independent of Core v30.

### 2.2 Comparison with Alternatives (addressing previous criticism)

| Protocol | Pros | Cons | Why BRC721 + Polkadot? |
|---------|------|------|-------------------------|
| Ordinals | Simple, popular | No metadata, no structure, no scale | BRC721 solves structure + scalability |
| Runes | Great for fungible tokens | Not for NFTs | Not applicable |
| RGB | Powerful design | High complexity, low adoption | BRC721 is simpler and more accessible |
| Taproot Assets | Best for L2 issuance | Not NFT-oriented | Cannot scale NFT applications |
| BRC721 | L1 native, scalable via Polkadot | Needs indexing + consensus layer | Polkadot provides the missing layer |

BRC721 also shares wallet compatibility with Taproot key-path outputs, reducing migration friction.

### 2.3 Value for DOT / Treasury

This integration brings direct benefits:

- Higher demand for Polkadot coretime from BTC-native developers  
- Polkadot becomes Bitcoin’s computation layer  
- New revenue streams for parachains that integrate BRC721 SDK  
- Increased visibility and developer inflow from the largest liquidity network in crypto  
- All tooling open-sourced, enabling ecosystem reuse  

A treasury value-capture mechanism is included (Section 7).

## 3. What Already Exists

1. **LAOS Parachain:** In production since mid-2024, already used by EVM chains and early gaming partners.  
2. **Research:** Formal BRC721 paper published in the Cryptology ePrint Archive.  
3. **Implementation:** Core primitives (UTXO parsing, taproot validation, collection logic) are open-source.  

The remaining work is well-scoped and de-risked.

## 4. What This Proposal Delivers

### 4.1 Core Deliverables (fully open-source by Q1 2026)

1. Bitcoin/LAOS dual-chain indexer  
2. Complete BRC721 validation logic + wallet libraries  
3. Reference implementation + developer documentation  
4. Polkadot SDK for parachains integrating BRC721  
5. Outreach: workshops, documentation, tutorials  

All components are designed to be permissionless, forkable, and independently maintainable.

## 5. Adoption Pipeline (addressing lack of partners)

Early discussions already underway with:
- Three gaming studios using LAOS on EVM chains  
- Two Bitcoin infra providers exploring indexing and wallet support  
- One marketplace testing Bitcoin-native NFT listings  
- Two Polkadot parachains evaluating SDK integration  

Names can be provided privately to relevant Collectives due to NDA constraints.

## 6. KPIs (addressing “vagueness”)

### Technical KPIs
- Fully functioning dual indexer with reorg handling up to 200 blocks  
- Indexing latency under 2 seconds for BRC721 events  
- Complete SDK and developer documentation  
- Reference dApp demonstrating mint/trade flow  
- Testnet + mainnet validation with reorg simulations  

### Ecosystem KPIs
- 3 workshops targeting Bitcoin developers  
- 5+ projects supported in integration talks  
- 2 parachains prototyping SDK usage  
- 2 external users of the indexer within 60 days  

### Outreach KPIs
- 4 technical blog posts  
- 2 podcasts or AMAs  
- Tutorial series on BRC721 + Polkadot  

## 7. Updated Budget (addressing “excessive cost”)

**Total Request:** 28,000 DOT (reduced from 40,000 DOT)  
Fits in the Small Spender Track depending on DOT valuation.

### Breakdown

| Category | Amount | Justification |
|----------|---------|---------------|
| Salaries | 19,000 DOT | 6 months part-time for 3 engineers |
| Infrastructure | 2,000 DOT | BTC/LAOS nodes, storage, indexing infra |
| External audit | 3,000 DOT | Security review requested by prior feedback |
| Marketing & outreach | 2,000 DOT | Workshops, explainers, community engagement |
| Treasury value-capture reserve | 2,000 DOT | To address moral hazard concerns |

### Treasury Value-Capture Mechanism

- Treasury receives priority rights to any optional commercial service fees from future indexing APIs (if adopted).  
- Treasury receives free access to all future optional LAOS enterprise modules for 24 months.  
- Code remains MIT/Apache and fully open-source, avoiding vendor lock-in.

## 8. Risk Mitigation Plan (addressing concerns about cross-chain indexing)

### Chain Reorganization Handling
- Configurable reorg depth up to 200 blocks  
- Deterministic rollback logic  
- Snapshot and incremental catch-up system  

### Fee Spike Mitigation
- Adaptive fee estimation  
- Batched mint commitments  
- Optional delayed submission for fee-sensitive applications  

### Scalability Measures
- Parallelized parsing  
- Elastic use of coretime for heavy workloads  

## 9. Why This Benefits Polkadot

This work positions Polkadot as **the go-to computation layer for Bitcoin NFTs and RWAs**, a narrative with strong market pull.

Key benefits:

- Strengthens Polkadot 2.0 economics through increased coretime demand  
- Brings BTC-native developers into the Polkadot ecosystem  
- Expands Polkadot’s role beyond EVM interoperability into Bitcoin interoperability  
- Provides reusable infrastructure for parachains  
- Aligns with Polkadot’s technical strengths: parallelism, modularity, determinism  

## 10. Conclusion

This revised proposal directly incorporates criticisms from the prior attempt:

- Reduced budget  
- Detailed expense breakdown  
- Clear KPIs and execution milestones  
- Treasury value-capture plan  
- Comparisons with alternative Bitcoin protocols  
- Addressed reorg/security/indexing concerns  
- Identified adoption pipeline and partners  
- Clarified benefits to DOT holders  
- Explained strategic role despite Bitcoin Core v30  

Funding this project enables Polkadot to become Bitcoin’s scalable computation and metadata layer — a powerful and underexplored ecosystem opportunity.

