extern crate rsfs;
extern crate unidecode;
use rsfs::{GenFS, Metadata, Permissions};
use std::io;
use std::path::{Path, PathBuf};
use unidecode::unidecode;

/// Sluggify a raw string.
///
/// This is the base implementation of the slug conversion rules.
/// Manipulation of paths is built upon this.
///
/// # Examples
///
/// ```
/// assert_eq!(slugger::convert_str("a b"), "a-b", "whitespace to dash");
/// ```
pub fn convert_str(input: &str) -> String {
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

pub fn convert_path(from: &Path) -> io::Result<PathBuf>
// where
//     P: Into<PathBuf>,
{
    // let from :PathBuf = from.into();
    // get the last component
    let last = from.components().last();
    // FIXME error handling
    let last = last.unwrap().as_os_str().to_string_lossy();
    let mut to = PathBuf::from(convert_str(&last));
    if let Some(dir) = from.parent() {
        let mut dir = dir.to_path_buf();
        dir.push(to);
        to = dir;
    }

    Ok(to)
}

pub fn rename<
    P: Permissions,
    M: Metadata<Permissions = P>,
    F: GenFS<Permissions = P, Metadata = M>,
>(
    fs: &mut F,
    from: &Path,
    to: &Path,
) -> io::Result<()> {
    if to == from {
        return Ok(());
    }
    if let Err(_err) = fs.metadata(&from) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Slug source file not found: {}", &from.display()),
        ));
    }
    if fs.metadata(&to).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Slug destination already exists: {}", &to.display()),
        ));
    }
    fs.rename(&from, &to)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;
    use std::path::PathBuf;
    #[test]
    fn test_convert_slug() {
        assert_eq!(convert_str("A"), "a", "uppercase to lowercase");
        assert_eq!(convert_str("Z"), "z", "uppercase to lowercase");
        assert_eq!(convert_str("a b"), "a-b", "whitespace to dash");
        assert_eq!(convert_str("a\nb"), "a-b", "whitespace to dash");
        assert_eq!(convert_str("a\tb"), "a-b", "whitespace to dash");
        assert_eq!(convert_str(" a"), "a", "trim whitespace");
        assert_eq!(convert_str("a "), "a", "trim whitespace");
        assert_eq!(convert_str("\ta\t"), "a", "trim whitespace");
        assert_eq!(convert_str("Á"), "a", "transliterate");
        assert_eq!(convert_str("a-b"), "a-b", "preserve dashes");
        assert_eq!(convert_str("a-"), "a", "trim dashes");
        assert_eq!(convert_str("-a"), "a", "trim dashes");
        assert_eq!(convert_str("-a-"), "a", "trim dashes");
        assert_eq!(convert_str("--a"), "a", "trim dashes");
        assert_eq!(convert_str("a--"), "a", "trim dashes");
        assert_eq!(convert_str("foo.txt"), "foo.txt", "preserve periods");
    }

    #[test]
    fn test_rename_base() -> Result<(), io::Error> {
        let mut fs = rsfs::mem::FS::new();
        let from = PathBuf::from("/A");
        let to = convert_path(&from)?;
        fs.create_file(from.as_path())?;
        match rename(&mut fs, &from, &to) {
            Err(_) => panic!("base rename should Ok"),
            Ok(_) => {
                match fs.metadata(&to) {
                    Ok(metadata) => {
                        assert!(metadata.is_file());
                    }
                    Err(_) => {
                        panic!("to path should exist after rename");
                    }
                }
                match fs.metadata(&from) {
                    Ok(_) => {
                        panic!("from path should not exist after rename");
                    }
                    Err(io_error) => {
                        assert_eq!(io_error.kind(), io::ErrorKind::NotFound);
                        Ok(())
                    }
                }
            }
        }
    }

    #[test]
    fn test_rename_no_clobber() -> Result<(), io::Error> {
        let mut fs = rsfs::mem::FS::new();
        let from = PathBuf::from("/A");
        let to = convert_path(&from)?;
        fs.create_file(&from)?;
        // TODO would like to know why I can't do this
        fs.create_file(&to)?;
        // fs.create_file(PathBuf::from("/a"))?;
        match rename(&mut fs, &from, &to) {
            Ok(_) => panic!("rename should not succeed if destination already exists"),
            Err(io_error) => {
                assert_eq!(io_error.kind(), std::io::ErrorKind::AlreadyExists);
                Ok(())
            }
        }
    }

    #[test]
    fn test_rename_no_op() -> Result<(), io::Error> {
        let mut fs = rsfs::mem::FS::new();
        let from = PathBuf::from("/a");
        let to = convert_path(&from)?;
        fs.create_file(&from)?;
        rename(&mut fs, &from, &to)
    }

    #[test]
    fn test_nested_file() -> Result<(), io::Error> {
        let mut fs = rsfs::mem::FS::new();
        let mut from = PathBuf::from("/Dir1");
        fs.create_dir(&from)?;
        from.push("Dir Two");
        fs.create_dir(&from)?;
        from.push("file one");
        fs.create_file(&from)?;
        let to = convert_path(&from)?;
        rename(&mut fs, &from, &to)?;
        // let to_clone = slug.to?.clone();
        fs.metadata(&to).expect("to path should exist");
        assert_eq!(to, PathBuf::from("/Dir1/Dir Two/file-one"));
        assert!(fs.metadata(&from).is_err(), "from path should not exist");
        Ok(())
    }

}

/*
        #[test]
        fn sort_by_name() {
            let p1 = PathBuf::from("a");
            let p2 = PathBuf::from("b");
            assert_eq!(sort_depth_then_directories(&p1, &p2), Ordering::Less);
        }

        #[test]
        fn sort_by_depth() {
            let p1 = PathBuf::from("b/b");
            let p2 = PathBuf::from("a");
            assert_eq!(sort_depth_then_directories(&p1, &p2), Ordering::Less);
        }

        #[test]
        fn sort_directories_first() {
            let src_dir = PathBuf::from(file!());
            let src_dir = src_dir.parent().unwrap();
            let src_path = PathBuf::from("s"); // earlier same name but file not dir
            println!("true.cmp(false) {:?}", true.cmp(&false));
            println!("src_dir components {:?}", src_dir.components().count());
            println!("src_dir is_dir {:?}", src_dir.is_dir());
            println!("src_path components {:?}", src_path.components().count());
            println!("src_path is_dir {:?}", src_path.is_dir());
            assert_eq!(
                sort_depth_then_directories(&src_dir, &src_path),
                Ordering::Less
            );
        }





    #[test]
    fn test_slugger_depth_first() {
        let mut fs = rsfs::mem::FS::new();
        fs.create_dir("/dir a").unwrap();
        fs.create_file("/dir a/file 1").unwrap();
        fs.create_file("/dir a/file 2").unwrap();
        let paths: Vec<String> = vec![
            "/dir a".into(),
            "/dir a/file 1".into(),
            "/dir a/file 2".into(),
        ];
        // Ensure the deep files get renamed before their containing directory
        slugger(&mut fs, &paths).unwrap();
        fs.metadata("/dir-a").unwrap();
        fs.metadata("/dir-a/file-1").unwrap();
        fs.metadata("/dir-a/file-2").unwrap();
    }

    #[test]
    fn test_slugger_from_not_found() {
        let mut fs = rsfs::mem::FS::new();
        let paths: Vec<String> = vec!["/from not found".into()];
        // Ensure the deep files get renamed before their containing directory
        let result = slugger(&mut fs, &paths);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let description = err.description();
        assert!(description.starts_with("Slug source file not found"));
        assert!(description.contains("/from not found"));
    }
*/
