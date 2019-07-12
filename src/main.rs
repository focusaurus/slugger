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

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut fs = rsfs::disk::FS;
    if let Err(message) = slugger_main(&mut fs, &args) {
        eprintln!("{}", message);
        std::process::exit(10);
    }
}
*/

fn main() {
    if let Err(message) = slugger_main() {
        eprintln!("{}", message);
        std::process::exit(10);
    }
}

fn slugger_main() -> io::Result<()> {
    let mut rename = false;
    if let Some(arg) = env::args().nth(1) {
        rename = arg == "--rename";
    }
    let stdin = io::stdin();
    for result in stdin.lock().lines() {
        let from = PathBuf::from(result?);
        let to = slugger::convert_path(&from)?;
        if rename {
            let mut fs = rsfs::disk::FS;
            slugger::rename(&mut fs, &from, &to)?;
        }
        println!("{}", to.display());
    }
    Ok(())
}
/*
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
*/
