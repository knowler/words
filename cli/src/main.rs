use log::error;
use pretty_env_logger;
use structopt::StructOpt;

use std::path::Path;

mod post;

use self::post::Post;

#[derive(StructOpt, Debug)]
enum Cli {
    /// The name of the post. This will be converted to TitleCase.
    #[structopt(name = "new")]
    New { title: String },
}

fn main() {
    let input = Cli::from_args();
    pretty_env_logger::init();

    if Path::new("content").exists() {
        match input {
            Cli::New { title } => Post::new(title).make(),
        }
    } else {
        error!("Doesn’t seem to be a Zola site: ./content not found.");
    }
}
