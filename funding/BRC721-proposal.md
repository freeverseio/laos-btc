# Bitcoin Integration of Bridgeless Minting using Polkadot 

This proposal asks for the funding required to complete the integration of the BRC721 protocol for Bridgeless Minting on Bitcoin. When completed, BRC721 will aim to become the new standard for the creation, management, and trading of NFTs on Bitcoin. It will vastly improve the capabilities of previous protocols, such as Ordinals/Inscriptions, by integrating Polkadot as a sibling consensus system for minting and optionally modifying token metadata. Via the BRC721 protocol, all data remains permanently on-chain, with no reliance on bridges or third-party operators.

## Benefit for Polkadot

BRC721 will connect Polkadot to the world's blockchain with the largest liquidity, in a manner almost transaparent to users. Real World Asset or Gaming applications will be able to tokenize in millions, at an almost negligible cost, while allowing their users to trade those assets completely natively on Bitcoin. Polkadot's huge capacity can become a sort of companion processor for Bitcoin, giving visibility to it, and become the de facto place where NFT data are stored and managed for Bitcoin's users.  


## Current State

1. Polkadot's LAOS Parachain [1] specializes on Bridgeless Minting. Since June 2024, it has put Polkadot's coretime at the disposal of every EVM compatible blockchain, including Ethereum and its Layer 2s, such as Base, Arbitrum, Polygon, etc. Games have started to use this pattern to externalize the minting and evolution of NFTs on various other chains, while letting their users trade as usual in those chains' currencies and existing DAPs [2].

2. As of Q3 2024, R&D efforts started to extend the protocol to Bitcoin, the world's most important non-programmable chain. This has resulted in:

* the recent reseach paper published in the Cryptology Archive [3],
* as well as in the implementation of the first of its primitives [4], written in Rust, and fully open source.

## What is asked

The core team that started the development for the BRC721 protocol asks for funding to complete its integration during 2025, including a small fraction to be devoted to marketing it and attracting users. The lead developer will continue to be Alessandro Siniscalchi [5], with the help of Toni Mateos, one extra developer to be hired, and one of the co-authors of the protocol paper, as well as Alun Evans, on the business and marketing side. 

The total amount asked is 40,000 DOT, planned to be spent rouhgly as follows: 28K in salaries, 4K for infrastructure (nodes, indexers) and 8K for marketing.

## References

[1] About LAOS
* [Main site](https://laosnetwork.io)
* [Resources](https://docs.laosnetwork.io/learn/resources), including whitepaper, developer docs, etc.
* [LAOS Parachain Repository](https://github.com/freeverseio/laos)

[2] [LAOS Medium](https://medium.com/laosnetwork), [Post About Gaming using LAOS](https://medium.com/laosnetwork/laos-network-lists-token-forges-partnership-with-sequence-to-bring-scalable-free-2-play-gaming-to-d49e56f7770f)
[BRC721 Repository](https://github.com/freeverseio/laos-btc)

[3] https://eprint.iacr.org/2025/641

[4] [BRC721 Repository](https://github.com/freeverseio/laos-btc)

