extern crate walkdir;
use std::path::{Path, PathBuf};
use std::{cmp, io};
use std::cmp::Ordering;
use walkdir::{DirEntry, WalkDir};
#[macro_use(slugify)]
extern crate slugify;
use slugify::slugify;
// use std::fs::FileType;
extern crate unidecode;
use unidecode::unidecode;
// use std::borrow::Cow;
use rsfs::{Fs, GenFS, Metadata, Permissions};

#[derive(Debug)]
pub struct Slug<'a> {
    pub from: &'a Path,
    pub to: PathBuf,
}

pub fn slug3(input: &str) -> String {
    let separator = '-';
    let input = input.trim();
    let input = input.to_lowercase();
    let input = input.trim_matches(separator);
    let input = unidecode(&input);
    let mut slug = String::with_capacity(input.len());
    for symbol in input.chars() {
        if symbol.is_whitespace() {
            slug.push(separator);
            continue;
        }
        match symbol {
            'a'...'z' | '0'...'9' | '.'| '-' => slug.push(symbol),
            _ => (), // delete anything else
        }
    }
    slug
}

#[test]
fn test_slug3() {
    assert_eq!(slug3("A"), "a", "uppercase to lowercase");
    assert_eq!(slug3("Z"), "z", "uppercase to lowercase");
    assert_eq!(slug3("a b"), "a-b", "whitespace to dash");
    assert_eq!(slug3("a\nb"), "a-b", "whitespace to dash");
    assert_eq!(slug3("a\tb"), "a-b", "whitespace to dash");
    assert_eq!(slug3(" a"), "a", "trim whitespace");
    assert_eq!(slug3("a "), "a", "trim whitespace");
    assert_eq!(slug3("\ta\t"), "a", "trim whitespace");
    assert_eq!(slug3("Á"), "a", "transliterate");
    assert_eq!(slug3("a-b"), "a-b", "preserve dashes");
    assert_eq!(slug3("a-"), "a", "trim dashes");
    assert_eq!(slug3("-a"), "a", "trim dashes");
    assert_eq!(slug3("-a-"), "a", "trim dashes");
    assert_eq!(slug3("--a"), "a", "trim dashes");
    assert_eq!(slug3("a--"), "a", "trim dashes");
    assert_eq!(slug3("foo.txt"), "foo.txt", "preserve periods");
}
// Based on https://docs.rs/slugify/0.1.0/src/slugify/lib.rs.html#1-355
pub fn slug2(input: &str) -> String {
    let separator = '-';
    let input: String = unidecode(input.into())
        .to_lowercase()
        .trim()
        .trim_matches(separator)
        .replace(' ', &separator.to_string());

    let mut slug = Vec::with_capacity(input.len());

    let mut is_sep = true;

    for x in input.chars() {
        match x {
            'a'...'z' | '0'...'9' | '.' => {
                is_sep = false;
                slug.push(x as u8);
            }
            _ => {
                if !is_sep {
                    is_sep = true;
                    slug.push(separator as u8);
                } else {
                }
            }
        }
    }

    if slug.last() == Some(&(separator as u8)) {
        slug.pop();
    }

    String::from_utf8(slug).unwrap()
}

pub fn get_slug(from: &Path) -> Result<Slug, String> {
    // get the last component
    let last = from.components().last();
    let last = last.unwrap();
    let last = last.as_os_str();
    let last = last.to_string_lossy(); // FIXME error handling
    let mut to = PathBuf::from(slug3(&last));
    let parent = from.parent();
    match parent {
        Some(dir) => {
            let mut dir = dir.to_path_buf();
            dir.push(to);
            to = dir;
        }
        None => (),
    }

    let slug = Slug { from, to };
    Ok(slug)
}

pub fn compile() {
    println!("{:?}", get_slug(&PathBuf::from("/foo/✓ /a b c.txt")));
}

pub fn rename<
    P: Permissions,
    M: Metadata<Permissions = P>,
    F: GenFS<Permissions = P, Metadata = M>,
>(
    fs: &mut F,
    slug: Slug,
) -> Result<Slug, String> {
}
// #[test]
// fn slug1_base_case() {
//     assert_eq!(slug1(&PathBuf::from("a b")).unwrap(), PathBuf::from("a-b"));
// }

/// Scans depth first then by path name within
/// Scans depth first then by path name within
pub fn scan(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(|result| result.ok())
        .map(|ref entry| entry.path().to_path_buf())
        .collect();
    // let mut entries = entries.clone();
    paths.sort_by(|path_a, path_b| sort_depth_then_directories(&path_a, &path_b));
    // let slugs: Vec<Result<Slug, String>> = paths
    //     .clone()
    //     .into_iter()
    //     .map(|path_buf| get_slug(path_buf))
    //     .collect();
    // for slug in slugs {
    //     println!("{:?}", slug);
    // }
    compile();
    Ok(paths)
}
//
// #[test]
// fn scan_is_depth_first_sorted() {
//     let dir = PathBuf::from(file!());
//     let mut dir = PathBuf::from(dir.parent().unwrap().parent().unwrap()); // project root
//     dir.push(PathBuf::from("unit-test"));
//
//     let entries = scan(&dir).unwrap();
//     let bulk: Vec<String> = entries
//         .iter()
//         .map(|entry| entry.path().to_string_lossy().into_owned())
//         .collect();
//     let bulk: String = bulk.join("\n");
//     // Note you can rebuild this on the command line with `find unit-test -depth`
//     let expected = "unit-test/sub1/a?b
// unit-test/sub1/x y z
// unit-test/sub1
// unit-test/sub2
// unit-test";
//     assert_eq!(bulk, expected);
// }

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

/*
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
*/
