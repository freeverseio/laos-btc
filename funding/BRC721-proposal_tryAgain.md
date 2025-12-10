# Bitcoin + Polkadot: Integration of the BRC-721 protocol to scale Bitcoin NFT using Polkadot coretime

## Summary

The LAOS Network Parachain[1] specializes in Bridgeless Minting - creating dynamic NFTs in other chains, while spending gas only in LAOS.

This proposal requests funding to complete the integration, and incentivize usage, of the BRC-721 protocol[2], permitting bridgeless minting of NFTs directly on the Bitcoin network, spending gas in LAOS.

For **Bitcoin users**, it would permit minting of large numbers of decentralized NFTs on Bitcoin, without congesting transactions and without needing to spend bitcoin. 

For **Polkadot**, it represents first link, without using bridges, between Polkadot and Bitcoin, incentivizing the use of Polkadot for the creation and evolution of dynamic NFTs.

LAOS Network has previously demonstrated[1] the ability to mint NFTs on all EVM networks, spending gas in LAOS to do so (as opposed to spending gas in the ownership EVM chain). Several applications have successfully used this feature in production, saving them thousand of dollars in gas fees. 

The BRC-721 protocol extends this feature to Bitcoin. It is in active development, and a beta version is already working, as demonstrated in:

* [a demonstration article](http://link) 
* [an open-source repository](https://github.com/laosfoundation/BRC-721)
* [a published research paper](https://eprint.iacr.org/2025/641)

This proposal is an update from [Referendum 1751](https://polkadot.polkassembly.io/referenda/1757), addressing some of the concerns of the community, and more importantly demonstrating the progress that has been made in the implementation of BRC-721 since that time. The LAOS team hopes that by submitting this updated proposal, we are demonstrating our strong desire to continue working and publicising the protocol, raising the profile of Polkadot and benefitting the community. 

## Introduction to BRC-721

BRC-721 introduces a new model that uses Polkadot as a parallel consensus layer to create, secure, and manage NFTs on Bitcoin at scale. As published in [1], unlike earlier approaches, such as Ordinals/Inscription, some of which gained attention despite technical limitations, BRC-721 is capable of dealing with Real-World Asset (RWA) or gaming applications, allowing users to tokenize millions of assets on Bitcoin at an almost negligible cost, while enabling users to trade those assets natively on Bitcoin. While doing so, BRC-721 keeps all data permanently on-chain and eliminates reliance on bridges or third-party infrastructure.

BRC-721 will connect Polkadot to the blockchain with the largest liquidity in the world, in a manner that is almost transparent to users. This integration could re-establish Polkadot as a key player in the blockchain ecosystem by combining its underutilized coretime and scalable consensus model with Bitcoin’s global recognition and security.

## Current State

1. Since June 2024, LAOS has offered Polkadot’s coretime to every EVM-compatible blockchain, including Ethereum and its Layer 2s such as Base, Arbitrum, Polygon, etc. Several applications have begun to use this pattern to externalize the minting and evolution of NFTs across various chains, while allowing users to trade in their native currencies and existing dApps [3].

2. In Q3 2024, R&D efforts began to extend the protocol to Bitcoin, the world’s most important non-programmable chain. This resulted in the research paper recently published in the Cryptology Archive [2], and the implementation of the first of its primitives [2], written in Rust and fully open source.

3. In Q4 2025, the LAOS team published the first working demo of the BRC-721 protocol[1]. The current version integrates:
    * a BRC-721 Indexer on Bitcoin
    * the creation and management of NFT metadata on LAOS
    * an Indexer Logic module to query created assets and metadata

## What is Asked

The core team behind the development of the BRC-721 protocol requests funding to complete its tecnical development during Q1'26, and then incentivizing its use via marketing and user outreach. The team has over six years of full-time blockchain development experience, including the creation of one of the earliest EVM Layer-2s, as well as the LAOS Parachain.

The lead developer will continue to be Alessandro Siniscalchi [4], supported by Toni Mateos [5] (co-author of the protocol paper), an additional developer to be hired, and Alun Evans [6] on the business and marketing side.

The total amount requested is 90,000 USDT, to be allocated approximately as follows: 40,000 for salaries, 10,000 for infrastructure (nodes, indexers), and 40,000 for marketing and user outreach. We are accordingly applying under the Treasury → Medium Spender Track.

**Timeline:** Based on the effort estimates derived from the work already completed, the team estimates completion of the technical work by the end of Q1 2026, and user-outreach to occur during Q2 and Q3 2026.

## The Outcome

The outcome will be a fully open-source infrastructure, containing all components required to maintain the protocol in a permissionless manner. This includes a critical piece which is at about 75% completion: an indexer that continuously parses both the Bitcoin and LAOS chains using a BRC-721-compliant parser. This enables applications to efficiently create NFTs at scale, trade them, query user inventories, retrieve NFT metadata, and more.

## References

[1] About LAOS:
* [Main site](https://laosnetwork.io)
* [Resources](https://docs.laosnetwork.io/learn/resources), including whitepaper, developer docs, etc.
* [LAOS Parachain Repository](https://github.com/freeverseio/laos)

[2] BRC-721 Current Status:
* [Scalable Non-Fungible Tokens on Bitcoin](https://eprint.iacr.org/2025/641)
* [Demonstration of current functionality](http://linky)
* [BRC-721 Repository](https://github.com/freeverseio/laos-btc)

[3] [LAOS Medium](https://medium.com/laosnetwork), [Post About Gaming using LAOS](https://medium.com/laosnetwork/laos-network-lists-token-forges-partnership-with-sequence-to-bring-scalable-free-2-play-gaming-to-d49e56f7770f)

[4] https://www.linkedin.com/in/asiniscalchi/, https://github.com/asiniscalchi

[5] https://www.linkedin.com/in/toni--mateos/

[6] https://www.linkedin.com/in/alun-evans/
