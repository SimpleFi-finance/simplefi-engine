# SIP DeFi Engine

This is HOWTO documentation to configure and run this engine. The main programming language is Rust and it uses workspaces to generate multiples binaries to run all different processes while sharing libraries and settings.

| [Developer Docs](./docs)

*The project is still work in progress, see the [disclaimer below](#status).*

## What is SIP DeFi Engine?

## Goals

More concretely, our goals are:
-
-
-

## Status

The project is **not ready for production use**.

SIP DeFi Engine is fully capable of syncing, however, there are still some missing features, and we are still working on performance and stability. Because of this, we are still introducing breaking changes.

It has **not been audited for security purposes** and should not be used in production yet.

We will be updating the documentation with the completion status of each component, as well as include more contributing guidelines (design docs, architecture diagrams, repository layouts) and "good first issues".

We appreciate your patience until we get there. Until then, we are happy to answer all questions in the Telegram link above.

## For Users

See the [SIP Engine Book](https://) for instructions on how to install and run Reth.

## Installation

Clone the repository `git clone ...`

For development just build it like `cargo build`

On production, build using release parameter `cargo build --release`

## For Developers

### Settings

Package to generate general settings to be used across the full application.

The settings are passed via arguments and they get stored in a file relative to the user's home directory.

Check the output file to find your settings destination.

Example:

`.\settings.exe --help`

### Using SIP engine as a library

You can use individual crates of SIP in your project.

For a general overview of the crates, see [Project Layout](./docs/repo/layout.md).

### Contributing

If you want to contribute, or follow along with contributor discussion, you can use our [main discord](https://) to chat with us about the development of Reth!

- Our contributor guidelines can be found in [`CONTRIBUTING.md`](./CONTRIBUTING.md).
- See our [contributor docs](./docs) for more information on the project. A good starting point is [Project Layout](./docs/repo/layout.md).

### Building and testing

The Minimum Supported Rust Version (MSRV) of this project is [1.70.0](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html).

First, clone the repository:

```sh
git clone https://
cd 
```

Next, run the tests:

We recommend using [`cargo nextest`](https://nexte.st/) to speed up testing. With nextest installed, simply substitute `cargo test` with `cargo nextest run`.

## Getting Help

If you have any questions, first see if the answer to your question can be found in the [book][book].

If the answer is not there:

- Join the [Discord][discord-url] to get help, or
- Open a [discussion](https://github.com/) with your question, or
- Open an issue with [the bug](https://github.com/)

## Security

See [`SECURITY.md`](./SECURITY.md).

## Acknowledgements

SIP DeFi Engine is a new implementation towards reliable DeFi data cross-chain. In the process of developing the engine we investigated the design decisions some nodes have made to understand what is done well, what is not, and where we can improve the status quo.

None of this would have been possible without them, so big shoutout to the teams below:
* [Geth](https://github.com/ethereum/go-ethereum/): We would like to express our heartfelt gratitude to the go-ethereum team for their outstanding contributions to Ethereum over the years. Their tireless efforts and dedication have helped to shape the Ethereum ecosystem and make it the vibrant and innovative community it is today. Thank you for your hard work and commitment to the project.
* [Erigon](https://github.com/ledgerwatch/erigon) (fka Turbo-Geth): Erigon pioneered the ["Staged Sync" architecture](https://erigon.substack.com/p/erigon-stage-sync-and-control-flows) that Reth is using, as well as [introduced MDBX](https://github.com/ledgerwatch/erigon/wiki/Choice-of-storage-engine) as the database of choice. We thank Erigon for pushing the state of the art research on the performance limits of Ethereum nodes.
* [Akula](https://github.com/akula-bft/akula/): Reth uses forks of the Apache versions of Akula's [MDBX Bindings](https://github.com/paradigmxyz/reth/pull/132), [FastRLP](https://github.com/paradigmxyz/reth/pull/63) and [ECIES](https://github.com/paradigmxyz/reth/pull/80) . Given that these packages were already released under the Apache License, and they implement standardized solutions, we decided not to reimplement them to iterate faster. We thank the Akula team for their contributions to the Rust Ethereum ecosystem and for publishing these packages.
* [Reth](https://github.com/paradigmxyz/reth): Reth engineered a lightening fast node based in Rust. We thank Reth for pushing the state of the art research on the performance limits of Ethereum nodes using the Rust language.

[book]: https://
[discord-url]: https://
