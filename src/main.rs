extern crate rsfs;
extern crate slugger;
extern crate structopt;
use std::env;
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
    let stdin = io::stdin();
    let lock = stdin.lock();
    if let Err(message) = slugger_main(env::args(), lock) {
        eprintln!("{}", message);
        std::process::exit(10);
    }
}

fn slugger_main<I>(args: I, input: impl BufRead) -> io::Result<()>
where
    I: IntoIterator<Item = String>,
{
    let opt = Opt::from_iter(args);
    for result in input.lines() {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slugger_help() {
        use std::io;
        assert!(slugger_main(vec!["--help".into()], io::empty()).is_ok());
    }
}
