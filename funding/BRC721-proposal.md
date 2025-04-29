# Bitcoin Integration of Bridgeless Minting using Polkadot

This proposal requests funding to complete the integration of the BRC721 protocol for Bridgeless Minting on Bitcoin. Once completed, BRC721 will aim to become the new standard for the creation, management, and trading of NFTs on Bitcoin. It will significantly improve upon previous protocols, such as Ordinals/Inscriptions, by integrating Polkadot as a sibling consensus system for minting and optionally modifying token metadata. Through the BRC721 protocol, all data remains permanently on-chain, with no reliance on bridges or third-party operators.

## Benefit for Polkadot

BRC721 will connect Polkadot to the world’s blockchain with the largest liquidity, in a manner that is almost transparent to users. Real-world asset or gaming applications will be able to tokenize millions of assets on Bitcoin at an almost negligible cost, while enabling users to trade those assets natively on Bitcoin. Polkadot’s vast capacity can act as a companion processor for Bitcoin, enhancing Polkadot's visibility and becoming the de facto platform for storing and managing NFT data for Bitcoin users.

## Current State

1. Polkadot’s LAOS Parachain [1] specializes in Bridgeless Minting. Since June 2024, it has offered Polkadot’s coretime to every EVM-compatible blockchain, including Ethereum and its Layer 2s such as Base, Arbitrum, Polygon, etc. Games have begun to use this pattern to externalize the minting and evolution of NFTs across various chains, while allowing users to trade in their native currencies and existing dApps [2].

2. As of Q3 2024, R&D efforts have begun to extend the protocol to Bitcoin, the world’s most important non-programmable chain. This has resulted in:

    * The research paper recently published in the Cryptology Archive [3],
    * And the implementation of the first of its primitives [4], written in Rust and fully open source.

## What is Asked

The core team behind the development of the BRC721 protocol requests funding to complete its integration during 2025, including a small portion allocated to marketing and user outreach. The team has over six years of full-time blockchain development experience, including the creation of one of the earliest Layer-2s, as well as the LAOS Parachain.

The lead developer will continue to be Alessandro Siniscalchi [5], supported by Toni Mateos (one of the co-authors of the protocol paper), an additional developer to be hired, and Alun Evans on the business and marketing side.

The total amount requested is 40,000 DOT, to be allocated approximately as follows: 30K for salaries, 3K for infrastructure (nodes, indexers), and 7K for marketing. We are accordingly applying under the Treasury → Medium Spender Track.

**Timeline:** Based on the effort estimates derived from the work already completed, the team estimates project completion by the end of 2025.

## References

[1] About LAOS:
* [Main site](https://laosnetwork.io)
* [Resources](https://docs.laosnetwork.io/learn/resources), including whitepaper, developer docs, etc.
* [LAOS Parachain Repository](https://github.com/freeverseio/laos)

[2] [LAOS Medium](https://medium.com/laosnetwork), [Post About Gaming using LAOS](https://medium.com/laosnetwork/laos-network-lists-token-forges-partnership-with-sequence-to-bring-scalable-free-2-play-gaming-to-d49e56f7770f)

[3] [Scalable Non-Fungible Tokens on Bitcoin](https://eprint.iacr.org/2025/641)

[4] [BRC721 Repository](https://github.com/freeverseio/laos-btc)

