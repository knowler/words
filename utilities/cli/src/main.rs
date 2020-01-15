use log::error;
use pretty_env_logger;
use structopt::StructOpt;

use std::path::Path;

mod post;

use self::post::Post;
use crate::command;

#[derive(StructOpt, Debug)]
struct Words {
    #[structopt(subcommand)]
    command: Option<Subcommand>,
}

impl Words {
    pub fn run(self) {
        if let Some(command) = self.command {
            command.run();
        }
    }
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    /// The name of the post. This will be converted to TitleCase.
    #[structopt(name = "new")]
    New(command::New),
}

impl Subcommand {
    pub(crate) fn run(self) {
        match self {
            Subcommand::New(new) => new.run(),
        }
    }
}

fn main() {
    let words = Words::from_args();

    words.run();

    pretty_env_logger::init();

    //if Path::new("content").exists() {
    //    println!("{:?}", words);

    ////match input {
    ////    New { title } => Post::new(title).make(),
    ////}
    //} else {
    //    error!("Doesn’t seem to be a Zola site: ./content not found.");
    //}
}
