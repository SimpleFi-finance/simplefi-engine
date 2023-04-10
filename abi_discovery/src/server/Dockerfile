# First we need to create an image with rust image to generate the settings file from the workspace /settings
# Then we need to create an image with the settings file to generate the abi files from the workspace /abi_discovery

# We need to create an image with rust image to generate the settings file from the workspace /settings
FROM rust:latest as settings

WORKDIR /settings

COPY ../../../settings/Cargo.toml settings/Cargo.lock ./

COPY settings/src ./src

# We don't need production, just to generate the .toml file
RUN cargo build

CMD ["target/debug/settings"]

RUN ls -la ~/
