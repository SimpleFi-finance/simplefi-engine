# ABI DISCOVERY PACKAGE

## HOWTO

First it's needed to create the settings. 

### Default settings
`cargo run -p abi_discovery --bin generate_settings`

### Help
`cargo run -p abi_discovery --bin generate_settings -- --help`

## Runners
For the scraper to run correctly, it's needed to have rabbit mq running and the settings correctly pointing to it with username and password. 

Then we run the Create Exchange queue where the producer should add 1 address per message.

Then we should keep running the consumer `scrap_etherscan`


### Create Rabbit MQ Exchange queue
`cargo run -p abi_discovery --bin create_queue`

### Queue consumer to scrap etherescan
`cargo run -p abi_discovery --bin scrap_etherscan`



