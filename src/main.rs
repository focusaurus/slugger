extern crate slugger;
extern crate rsfs;
use slugger::Slug;
use std::env;
use std::path::PathBuf;

fn slugger(args: Vec<String>) -> Result<(), String> {
    match args.len() {
        0 => Err("Usage: slugger <path1> [path2] [...path3]".into()),
        _ => {
            let from = PathBuf::from(args.first().unwrap());
            let slug = slugger::get_slug(&from)?;
            let mut fs = rsfs::disk::FS;
            slugger::rename(&mut fs, slug)?;

            Ok(())
        }
    }
}

fn main() {
    let args = env::args().skip(1).collect();
    if let Err(message) = slugger(args) {
        eprintln!("{}", message);
        std::process::exit(10);
    }
}
