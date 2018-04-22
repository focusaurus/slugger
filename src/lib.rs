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
}
