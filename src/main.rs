extern crate rsfs;
extern crate slugger;
extern crate structopt;
use rsfs::{GenFS, Metadata, Permissions};
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// Transform filenames to eliminate special characters. Optionally rename.
#[derive(StructOpt, Debug)]
#[structopt(name = "slugger")]
struct Opt {
    /// Actually perform renames on the filesystem.
    /// By default will print slugs to stdout but not access the filesystem.
    #[structopt(short = "r", long = "rename")]
    rename: bool,
}

/*
fn slugger_main<
    P: Permissions,
    M: Metadata<Permissions = P>,
    F: GenFS<Permissions = P, Metadata = M>,
>(
    fs: &mut F,
    args: &[String],
) -> io::Result<()> {
    if args.is_empty() {
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
    let opt = Opt::from_args();
    let stdin = io::stdin();
    for result in stdin.lock().lines() {
        let from = PathBuf::from(result?);
        let to = slugger::convert_path(&from)?;
        if opt.rename {
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
