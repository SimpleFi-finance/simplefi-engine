*The project is still work in progress, see the [disclaimer below](#status).*

## Status

The project is **not ready for production use**.

The SimpleFi DeFi Analytics Engine is fully capable of syncing, however, there are still some missing features, and we are still working on performance and stability. Because of this, we are still introducing breaking changes.

It has **not been audited for security purposes** and should not be used in production yet.

We will be updating the documentation with the completion status of each component, as well as include more contributing guidelines (design docs, architecture diagrams, repository layouts) and "good first issues".

## Installation

Clone the repository `git clone ...`

For development just build it like `cargo build`

On production, build using release parameter `cargo build --release`

### Settings

Package to generate general settings to be used across the full application.

The settings are passed via arguments and they get stored in a file relative to the user's home directory.

Check the output file to find your settings destination.

Example:

`.\settings.exe --help`

### Using the engine as a library

You can use individual crates of the engine in your project.

### Contributing

- Our contributor guidelines can be found in [`CONTRIBUTING.md`](./CONTRIBUTING.md).

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

- Open a discussion with your question

## Security

See [`SECURITY.md`](./SECURITY.md).

