# Bitcoin Integration of Bridgeless Minting using Polkadot

This proposal requests funding to complete the integration of the BRC721 protocol, enabling bridgeless minting of NFTs directly on the Bitcoin network.

BRC721 introduces a new model that uses Polkadot as a parallel consensus layer to create, secure, and manage NFTs on Bitcoin at scale. As published in [1], unlike earlier approaches, such as Ordinals/Inscription, some of which gained attention despite technical limitations, BRC721 is capable of dealing with Real-World Asset (RWA) or gaming applications, allowing users to tokenize millions of assets on Bitcoin at an almost negligible cost, while enabling users to trade those assets natively on Bitcoin. While doing so, BRC721 keeps all data permanently on-chain and eliminates reliance on bridges or third-party infrastructure.

BRC721 will connect Polkadot to the world’s blockchain with the largest liquidity, in a manner that is almost transparent to users. This integration could re-establish Polkadot as a key player in the blockchain ecosystem by combining its underutilized coretime and scalable consensus model with Bitcoin’s global recognition and security.


## Current State

1. Polkadot’s LAOS Parachain [2] specializes in Bridgeless Minting. Since June 2024, it has offered Polkadot’s coretime to every EVM-compatible blockchain, including Ethereum and its Layer 2s such as Base, Arbitrum, Polygon, etc. Games have begun to use this pattern to externalize the minting and evolution of NFTs across various chains, while allowing users to trade in their native currencies and existing dApps [3].

2. As of Q3 2024, R&D efforts have begun to extend the protocol to Bitcoin, the world’s most important non-programmable chain. This has resulted in:

    * The research paper recently published in the Cryptology Archive [1],
    * And the implementation of the first of its primitives [4], written in Rust and fully open source.

## What is Asked

The core team behind the development of the BRC721 protocol requests funding to complete its integration during Q4'25 and Q1'26, including a small portion allocated to marketing and user outreach. The team has over six years of full-time blockchain development experience, including the creation of one of the earliest Layer-2s, as well as the LAOS Parachain.

The lead developer will continue to be Alessandro Siniscalchi [5], supported by Toni Mateos [6] (co-author of the protocol paper), an additional developer to be hired, and Alun Evans [7] on the business and marketing side.

The total amount requested is 40,000 DOT, to be allocated approximately as follows: 30K for salaries, 3K for infrastructure (nodes, indexers), and 7K for marketing. We are accordingly applying under the Treasury → Medium Spender Track.

**Timeline:** Based on the effort estimates derived from the work already completed, the team estimates project completion by the end of Q1 2026.

## The Outcome

The outcome will be a fully open-source infrastructure, containing all components required to maintain the protocol in a permissionless manner. This includes a critical piece which is at about 25% completion: an indexer that continuously parses both the Bitcoin and LAOS chains using a BRC721-compliant parser. This enables applications to efficiently create NFTs at scale, trade them, query user inventories, retrieve NFT metadata, and more.

## References

[1] [Scalable Non-Fungible Tokens on Bitcoin](https://eprint.iacr.org/2025/641)

[2] About LAOS:
* [Main site](https://laosnetwork.io)
* [Resources](https://docs.laosnetwork.io/learn/resources), including whitepaper, developer docs, etc.
* [LAOS Parachain Repository](https://github.com/freeverseio/laos)

[3] [LAOS Medium](https://medium.com/laosnetwork), [Post About Gaming using LAOS](https://medium.com/laosnetwork/laos-network-lists-token-forges-partnership-with-sequence-to-bring-scalable-free-2-play-gaming-to-d49e56f7770f)

[4] [BRC721 Repository](https://github.com/freeverseio/laos-btc)

[5] https://www.linkedin.com/in/asiniscalchi/, https://github.com/asiniscalchi

[6] https://www.linkedin.com/in/toni--mateos/

[7] https://www.linkedin.com/in/alun-evans/
