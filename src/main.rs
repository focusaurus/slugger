extern crate slugger;
#[macro_use(slugify)]
extern crate slugify;
extern crate walkdir;

use slugify::slugify;
use std::env;
// use walkdir::WalkDir;
use slugger::Slug;
use std::path::PathBuf;

fn main() {
    if let Some(from) = env::args().nth(1) {
        let from = PathBuf::from(from);
        if let Ok(slug) = slugger::get_slug(&from) {
            println!("{:?}", slug.to.display());
        }
    }
}
