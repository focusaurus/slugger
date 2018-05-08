extern crate rsfs;
extern crate slugger;
use std::{env, io};
use std::error::Error;
use std::path::PathBuf;
use slugger::sort_depth_then_directories;

fn slugger(args: Vec<String>) -> io::Result<()> {
    match args.len() {
        0 => Err(io::Error::new(
            io::ErrorKind::Other,
            "Usage: slugger <path1> [path2] [...path3]",
        )),
        _ => {
            let mut paths: Vec<PathBuf> = args.iter().map(|arg| PathBuf::from(arg)).collect();
            paths.sort_by(|path_a, path_b| sort_depth_then_directories(&path_a, &path_b));
            let mut fs = rsfs::disk::FS;
            for path in paths.iter() {
                let slug = slugger::get_slug(&path)?;
                slugger::rename(&mut fs, &slug)?;
            }
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

#[test]
fn slugger_zero_args_error() {
    match slugger(vec![]) {
        Ok(()) => panic!("should return Err with zero args"),
        Err(io_error) => assert!(io_error.description().starts_with("Usage")),
    }
}
