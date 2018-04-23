extern crate walkdir;
use std::path::{Path, PathBuf};
use std::io;
use walkdir::WalkDir;
#[macro_use(slugify)]
extern crate slugify;
use slugify::slugify;

#[derive(Debug)]
pub struct Slug<'a> {
    pub from: &'a Path,
    pub to: PathBuf,
}

pub fn slug(path: &Path) -> io::Result<()> {
    let mut entries: Vec<walkdir::DirEntry> = WalkDir::new(path)
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();
    entries.sort_by(|a, b| {
        b.path()
            .components()
            .count()
            .cmp(&a.path().components().count())
    });
    for entry in entries.iter() {
        println!("{:?}", entry.path());
        if entry.path().is_file() {
            match entry.path().file_name() {
                Some(name) => {
                    println!("{:?}", slugify!(&name.to_string_lossy()));
                }
                None => {
                    println!("NONE");
                }
            }
        }
    }
    /*    for result in WalkDir::new(path).into_iter() {
        match result {
            Ok(entry) => {
                if entry.path().is_file() {
                    println!("{:?}", entry.path().file_name()?);
                }
                let slug = Slug {
                    from: &entry.path(),
                    to: PathBuf::from(slugify!(&entry.path().to_string_lossy())),
                };
                println!("{:?}", slug);
                // println!("{}", slugify!(&entry.path().to_string_lossy()));
            }
            Err(message) => {
                eprintln!("ERR: {}", message);
            }
        }
    }
    */
    Ok(())
}

#[test]
fn identity() {
    let input = PathBuf::from("/");
    assert!(slug(input.as_path()) == input);
    let input = PathBuf::from("/foo");
    assert!(slug(input.as_path()) == input);
    let input = PathBuf::from("/foo/bar/baz-dash-bux.txt");
    assert!(slug(input.as_path()) == input);
}

#[test]
fn space_in_word() {
    let input = PathBuf::from("/space here.txt");
    assert!(slug(input.as_path()) == PathBuf::from("/space-here.txt"));
}
