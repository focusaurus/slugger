extern crate rsfs;
extern crate slugger;
use rsfs::{GenFS, Metadata, Permissions};
use std::{env, io};

fn main_result<
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
    let args = vec!(env::args().skip(1).collect());
    let mut fs = rsfs::disk::FS;
    if let Err(message) = main_result(&mut fs, &args) {
        eprintln!("{}", message);
        std::process::exit(10);
    }
}

#[test]
fn slugger_zero_args_error() {
    use std::error::Error;
    let mut fs = rsfs::mem::FS::new();
    match main_result(&mut fs, &vec![]) {
        Ok(()) => panic!("should return Err with zero args"),
        Err(io_error) => assert!(io_error.description().starts_with("Usage")),
    }
}
