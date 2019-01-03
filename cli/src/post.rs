use chrono::{Local, SecondsFormat};
use heck::{KebabCase, TitleCase};
use serde_derive::Serialize;
use tera::Tera;

use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Debug)]
pub struct Post {
    filename: String,
    title: String,
    date: String,
}

impl Post {
    pub fn new(title: String) -> Self {
        Self {
            filename: format!("{}.md", title.to_kebab_case()),
            title: title.to_title_case(),
            date: Local::now().to_rfc3339_opts(SecondsFormat::Secs, false),
        }
    }

    pub fn make(&self) {
        let mut tera = Tera::default();
        tera.add_raw_template(
            "new-post.md",
            "+++\ntitle = \"{{title}}\"\ndate = {{date}}\n+++",
        )
        .expect("New post template failed to render");

        let path = Path::new("content").join(&self.filename);
        let mut file = File::create(&path).expect("Cannot create new post.");
        let contents = tera.render("new-post.md", &self);

        file.write_all(&contents.unwrap().as_bytes())
            .expect("Cannot write new post.");

        println!("Created: {}", path.display().to_string());
    }
}
