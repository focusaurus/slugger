extern crate rsfs;
extern crate slugger;
use std::{env, io};
use std::error::Error;
use std::path::PathBuf;

fn slugger(args: Vec<String>) -> io::Result<()> {
    match args.len() {
        0 => Err(io::Error::new(
            io::ErrorKind::Other,
            "Usage: slugger <path1> [path2] [...path3]",
        )),
        _ => {
            let from = PathBuf::from(args.first().unwrap());
            let slug = slugger::get_slug(&from)?;
            let mut fs = rsfs::disk::FS;
            slugger::rename(&mut fs, &slug)?;

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
