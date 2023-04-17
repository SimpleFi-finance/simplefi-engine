# ABI DISCOVERY PACKAGE

## HOWTO

First it's needed to create the settings. 

### Default settings
`cargo run -p abi_discovery --bin generate_settings`

### Help
`cargo run -p abi_discovery --bin generate_settings -- --help`

## Runners

### Create Rabbit MQ Exchange queue
`cargo run -p abi_discovery --bin create_queue`

### Queue consumer to scrap etherescan
`cargo run -p abi_discovery --bin scap_etherscan`



