extern crate rsfs;
extern crate slugger;
use rsfs::{GenFS, Metadata, Permissions};
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, io};
/*
fn slugger_main<
    P: Permissions,
    M: Metadata<Permissions = P>,
    F: GenFS<Permissions = P, Metadata = M>,
>(
    fs: &mut F,
    args: &[String],
) -> io::Result<()> {
    if args.len() < 1 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Usage: slugger <path1> [path2] [...path3]",
        ));
    }
    slugger::slugger(fs, &args)
}

fn main1() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut fs = rsfs::disk::FS;
    if let Err(message) = slugger_main(&mut fs, &args) {
        eprintln!("{}", message);
        std::process::exit(10);
    }
}
*/

fn main() {
    let stdin = io::stdin();
    for result in stdin.lock().lines() {
        // println!("{}", slugger::slug_line(line));
        // let from_path_buf = PathBuf::from(result.unwrap());
        let slug = slugger::Slug2::from(result.unwrap());
        println!("{}", slug);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slugger_zero_args_error() {
        use std::error::Error;
        let mut fs = rsfs::mem::FS::new();
        match slugger_main(&mut fs, &vec![]) {
            Ok(()) => panic!("should return Err with zero args"),
            Err(io_error) => assert!(io_error.description().starts_with("Usage")),
        }
    }
}
