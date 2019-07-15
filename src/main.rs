extern crate rsfs;
extern crate slugger;
extern crate structopt;
use rsfs::{GenFS, Metadata, Permissions};
use std::ffi::OsString;
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

*/

fn main() {
    if let Err(message) = slugger_main(None) {
        eprintln!("{}", message);
        std::process::exit(10);
    }
}

fn slugger_main(override_args: Option<Vec<OsString>>) -> io::Result<()> {
    let opt = match override_args {
        Some(args) => Opt::from_iter(args.iter()),
        None => Opt::from_args(),
    };
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
    fn slugger_help() {
        use std::error::Error;
        let mut fs = rsfs::mem::FS::new();
        match slugger_main(&mut fs, &vec![]) {
            Ok(()) => panic!("should return Err with zero args"),
            Err(io_error) => assert!(io_error.description().starts_with("Usage")),
        }
    }
}
*/
