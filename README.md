# SimpleFi Finance Engine

This is HOWTO documentation to configure and run this engine. The main programming language is Rust and it uses workspaces to generate multiples binaries to run all different processes while sharing libraries and settings.

## Installation

Clone the repository `git clone ...`

For development just build it like `cargo build`

On production, build using release parameter `cargo build --release`

## Executioners

### Settings

Package to generate general settings to be used across the full application.

The settings are passed via arguments and they get stored in a file relative to the user's home directory.

Check the output file to find your settings destination.

Example:

`.\settings.exe --help`


