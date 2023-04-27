@echo off
:restart
echo Starting your Rust application...
cargo run -p abi_discovery --bin scrap_etherscan
echo Your Rust application has exited with an error. Restarting...
timeout /t 5
goto restart
