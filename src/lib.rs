extern crate rsfs;
extern crate unidecode;
use rsfs::{GenFS, Metadata, Permissions};
use std::path::{Path, PathBuf};
use unidecode::unidecode;

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
            'a'...'z' | '0'...'9' | '.' | '-' => slug.push(symbol),
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

pub fn rename<
    P: Permissions,
    M: Metadata<Permissions = P>,
    F: GenFS<Permissions = P, Metadata = M>,
>(
    fs: &mut F,
    slug: Slug,
) -> Result<(), String> {
    Err("Not Impletement".into())
}
