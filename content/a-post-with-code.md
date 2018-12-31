+++
title = "A post with code"
date = 2018-12-31

[taxonomies]
tags = ["rust"]
+++

```rust
use quicli::prelude::*;
use structopt::StructOpt;

use clover::{Cli, Plugin};
use clover::make::{Controller, Migration};

fn main() -> CliResult {
    match Cli::from_args() {
        Cli::New { name } => Plugin::new(name).init(),
        Cli::MakeController { name } => Controller::new(name).make(),
        Cli::MakeMigration { name } => Migration::new(name).make(),
    };

    Ok(())
}
```
