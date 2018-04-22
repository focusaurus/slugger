#[macro_use(slugify)]
extern crate slugify;
extern crate slugger;

use std::path::PathBuf;
use slugger::slug;
use slugify::slugify;

fn main() {
    let hey = "/foo/bar baz/bux .txt ";
    println!("{}", slugify!("Hello, world!"));
    println!("{}", slugify!(hey));
    println!(
        "slug: {}",
        slug(&PathBuf::from("/hey/foo bar")).to_string_lossy()
    );
}
