# Global Logger for Simplefi Engine

## Introduction

The main goal of these utility is to set a log level and a output file just once and I get an easy logger across all the project.

It requires to setup a couple of variables in the settings logger.

## `log_level`
 Variable to set the filtering. Possible values:
 - INFO
 - DEBUG
 - WARN
 - ERROR

## `log_file`
Output file to save the logging.

## How To
The way to use in your workspace is:

1. Add shared utils into your workspace cargo
2. Initialize the logger handler in your main execution program

```rust
use shared_utils::logger::init_logging;

// Add this call after your main function
fn main() {
    init_logging();
    
    // ...
}
```
3. Import log helpers and use them across your needs
```rust
use log::{ info, debug, error };

info!("Usually info message");

debug!("This is just for debugging. It should not visible in production");

error!("ouch! this is bad. something is broken");
```



 
