extern crate slugger;
use std::path::PathBuf;
use slugger::slug;

fn main() {
    println!("Hello, world!");
    println!(
        "slug: {}",
        slug(&PathBuf::from("/hey/foo bar")).to_string_lossy()
    );
}
