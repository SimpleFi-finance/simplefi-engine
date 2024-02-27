# SimpleFi DeFi Analytics Engine V0

This is HOWTO documentation to configure and run this engine. The main programming language is Rust and it uses workspaces to generate multiples binaries to run all different processes while sharing libraries and settings.

*The project is not completed and to get it up and running more work outlined in the READMEs is required.*

## What is the SimpleFi DeFi Analytics Engine?

The engine was meant to be a data processor specialised in DeFi data. It works as a state machine, node-agnostic, where data produced by a given blockchain is digested and stored in a local rocksDB database ready for analysis.


## Status

The project is **not complete**.

The SimpleFi DeFi Analytics Engine is partially capable of syncing basic blockchain data, however, several essential features are still some missing.


## Installation

Clone the repository `git clone ...`

For development just build it like `cargo build`

## For Developers

The main crates to look at are: Utils, Stages and Storage.

Storage crate contains all the logic to use RocksDB as a database provider in the engine. It has been tested and runs on its own.

Stages crate contains all the logic to run the state engine and some of the code has been tested and can run when connected to an external node, downloading basic blockchain data such as transactions and logs.

### Utils

The utils crate contains all the logic to query nodes, select which node to use and decode incoming logs before storing them in the local database.

### Stages

#### Basic blockchain data

The logic to store basic blockchain data has been included in the main branch of the repo and can be found in crates/stages/*.rs
These methods can be added to a pipeline to be concurrently run. The pipeline methods can be found in crates/pipeline.

Goal of the blockchain data:
- backfill and track blockchain data served by a given node
- transform the data received in a unified data structure, allowing for a multi-chain environment
- make the data available for subsequent stages (ie. advanced DeFi analytics, ML models)

#### DeFi Specific Data

The logic to store DeFi specific data can be found in a separate branch of this repo, since it needs migrating from a previous version of the engine to the current state machine version.
The types and methods can be found in crates/silver of branch "refactor/gold-silver".

Goal of the DeFi Data:
- use the unified blockchain data
- select the correct methods to record volumentric and market snapshot data in a programmatic and automatic methodology
- create 5 min, 1hr, and daily timeseries documentation of the market activity
- backfill and track the history/progress of such markets

### Types

The structs related to blockchain, DeFi, storage and relevant others can be found in crates/primitives. The structs saved in the database already implement the serialize and deserialize methods to be properly write and read into RocksDB.


### Settings

Package to generate general settings to be used across the full application.

The settings are passed via arguments and they get stored in a file relative to the user's home directory.

Check the output file to find your settings destination.

Example:

`.\settings.exe --help`


# Major features missing

To run a basic full version of this engine the following features are missing:

- build and run a pipeline (state machine) within the bin folder to allow calling the process from the terminal
- import and convert the silver pipeline to match the current version of the engine
- various tests and performance checks

To run the engine in a meaningful way, the user needs to have access to either a local node or private cloud node.

### Building and testing

The Minimum Supported Rust Version (MSRV) of this project is [1.70.0](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html).

First, clone the repository:

```sh
git clone https://
cd 
```

Next, run the tests:

We recommend using [`cargo nextest`](https://nexte.st/) to speed up testing. With nextest installed, simply substitute `cargo test` with `cargo nextest run`.

## Acknowledgements

The SimpleFi DeFi Analytics Engine aims to generate and store reliable cross-chain DeFi data. In the process of developing the engine we were inspired by the design decisions of various node development teams. None of this would have been possible without them, so big shoutout to the teams below:

* [Geth](https://github.com/ethereum/go-ethereum/): We would like to express our heartfelt gratitude to the go-ethereum team for their outstanding contributions to Ethereum over the years. Their tireless efforts and dedication have helped to shape the Ethereum ecosystem and make it the vibrant and innovative community it is today. Thank you for your hard work and commitment to the project.
* [Erigon](https://github.com/ledgerwatch/erigon) (fka Turbo-Geth): Erigon pioneered the ["Staged Sync" architecture](https://erigon.substack.com/p/erigon-stage-sync-and-control-flows) that Reth is using, as well as [introduced MDBX](https://github.com/ledgerwatch/erigon/wiki/Choice-of-storage-engine) as the database of choice. We thank Erigon for pushing the state of the art in research on the performance limits of Ethereum nodes.
* [Reth](https://github.com/paradigmxyz/reth): Reth engineered a lightening fast node based in Rust. We thank Reth for pushing the state of the art in research on the performance limits of Ethereum nodes using the Rust language.
