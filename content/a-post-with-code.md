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

The CLI looks like this:

```rust
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Cli {
    /// Creates a new plugin
    #[structopt(name = "new")]
    New {
        /// plugin name
        name: String,
    },
    /// Make a new controller
    #[structopt(name = "make:controller")]
    MakeController {
        /// controller name
        name: String,
    },
    /// Make a new migration
    #[structopt(name = "make:migration")]
    MakeMigration {
        /// migration name
        name: String,
    }
}
```
