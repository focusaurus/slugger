use std::path::{Path, PathBuf};

pub fn slug(path: &Path) -> PathBuf {
    PathBuf::from(&path)
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
