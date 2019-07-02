extern crate rsfs;
extern crate unidecode;
use rsfs::{GenFS, Metadata, Permissions};
use std::cmp::Ordering;
use std::io;
use std::path::{Path, PathBuf};
use unidecode::unidecode;
// use std::str::FromStr;

#[derive(Debug)]
pub struct Slug<'a> {
    pub from: &'a Path,
    pub to: PathBuf,
}

#[derive(Debug)]
pub struct Slug2 {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl From<PathBuf> for Slug2 {
    fn from(from_path_buf: PathBuf) -> Self {
        let to_path_buf = from_path_buf.clone();
        Slug2 {
            from: from_path_buf,
            to: to_path_buf
        }
    }
}

impl From<String> for Slug2 {
    fn from(from_string: String) -> Self {
        Slug2::from(PathBuf::from(from_string))
    }
}

fn sort_depth_then_directories<'a>(path_a: &'a Path, path_b: &'a Path) -> Ordering {
    // deepest first
    path_a
        .components()
        .count()
        .cmp(&path_b.components().count())
        .reverse()
    // directories first (rust considers true>false)
        .then(path_a.is_dir().cmp(&path_b.is_dir()).reverse())
    // then files sorted by name
       .then(path_a.cmp(&path_b))
}

pub fn slug(input: &str) -> String {
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

pub fn get_slug(from: &Path) -> io::Result<Slug> {
    // get the last component
    let last = from.components().last();
    let last = last.unwrap();
    let last = last.as_os_str();
    let last = last.to_string_lossy(); // FIXME error handling
    let mut to = PathBuf::from(slug(&last));
    if let Some(dir) = from.parent() {
        let mut dir = dir.to_path_buf();
        dir.push(to);
        to = dir;
    }

    let slug = Slug { from, to };
    Ok(slug)
}

pub fn slugger<
    P: Permissions,
    M: Metadata<Permissions = P>,
    F: GenFS<Permissions = P, Metadata = M>,
>(
    fs: &mut F,
    args: &[String],
) -> io::Result<()> {
    let mut paths: Vec<PathBuf> = args.iter().map(PathBuf::from).collect();
    paths.sort_by(|path_a, path_b| sort_depth_then_directories(&path_a, &path_b));
    for path in &paths {
        let slug = get_slug(&path)?;
        rename(fs, &slug)?;
    }
    Ok(())
}

// pub fn slug_line<'a>(line: String<'a>) -> io::Result<Slug<'a>> {
//     get_slug(&PathBuf::from(line))
// }

pub fn rename<
    P: Permissions,
    M: Metadata<Permissions = P>,
    F: GenFS<Permissions = P, Metadata = M>,
>(
    fs: &mut F,
    slug: &Slug,
) -> io::Result<()> {
    if slug.to == slug.from {
        return Ok(());
    }
    if let Err(_err) = fs.metadata(&slug.from) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Slug source file not found: {}", &slug.from.display()),
        ));
    }
    if fs.metadata(&slug.to).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Slug destination already exists: {}", &slug.to.display()),
        ));
    }
    fs.rename(&slug.from, &slug.to)
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use super::*;
    use std::error::Error;

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
    fn test_slug() {
        assert_eq!(slug("A"), "a", "uppercase to lowercase");
        assert_eq!(slug("Z"), "z", "uppercase to lowercase");
        assert_eq!(slug("a b"), "a-b", "whitespace to dash");
        assert_eq!(slug("a\nb"), "a-b", "whitespace to dash");
        assert_eq!(slug("a\tb"), "a-b", "whitespace to dash");
        assert_eq!(slug(" a"), "a", "trim whitespace");
        assert_eq!(slug("a "), "a", "trim whitespace");
        assert_eq!(slug("\ta\t"), "a", "trim whitespace");
        assert_eq!(slug("Á"), "a", "transliterate");
        assert_eq!(slug("a-b"), "a-b", "preserve dashes");
        assert_eq!(slug("a-"), "a", "trim dashes");
        assert_eq!(slug("-a"), "a", "trim dashes");
        assert_eq!(slug("-a-"), "a", "trim dashes");
        assert_eq!(slug("--a"), "a", "trim dashes");
        assert_eq!(slug("a--"), "a", "trim dashes");
        assert_eq!(slug("foo.txt"), "foo.txt", "preserve periods");
    }

    #[test]
    fn test_rename_base() {
        let mut fs = rsfs::mem::FS::new();
        let from = PathBuf::from("/A");
        let slug = get_slug(&from).unwrap();
        fs.create_file(slug.from).unwrap();
        rename(&mut fs, &slug).unwrap();
        match fs.metadata(slug.to) {
            Ok(metadata) => {
                assert!(metadata.is_file());
            }
            Err(_) => {
                panic!("to path should not have errors after rename");
            }
        }
        match fs.metadata(slug.from) {
            Ok(_) => {
                panic!("from path should not exist after rename");
            }
            Err(io_error) => {
                assert_eq!(io_error.kind(), io::ErrorKind::NotFound);
            }
        }
    }

    #[test]
    fn test_rename_no_clobber() {
        let mut fs = rsfs::mem::FS::new();
        let from = PathBuf::from("/A");
        let slug = get_slug(&from).unwrap();
        fs.create_file(&slug.from).unwrap();
        fs.create_file(&slug.to).unwrap();
        let result = rename(&mut fs, &slug);
        match result {
            Ok(_) => {
                panic!("rename should not succeed if destination already exists");
            }
            Err(io_error) => {
                assert_eq!(io_error.kind(), std::io::ErrorKind::AlreadyExists);
            }
        }
    }

    #[test]
    fn test_rename_no_op() {
        let mut fs = rsfs::mem::FS::new();
        let from = PathBuf::from("/a");
        let slug = get_slug(&from).unwrap();
        fs.create_file(&slug.from).unwrap();
        if let Err(_) = rename(&mut fs, &slug) {
            panic!("should not error if existing file is already a slug");
        }
    }

    #[test]
    fn test_nested_file() {
        let mut fs = rsfs::mem::FS::new();
        let mut from = PathBuf::from("/Dir1");
        fs.create_dir(&from).unwrap();
        from.push("Dir Two");
        fs.create_dir(&from).unwrap();
        from.push("file one");
        fs.create_file(&from).unwrap();
        let slug = get_slug(&from).unwrap();
        assert_eq!(slug.to, PathBuf::from("/Dir1/Dir Two/file-one"));
        rename(&mut fs, &slug).unwrap();
        fs.metadata(&slug.to).expect("to path should exist");
        assert!(
            fs.metadata(&slug.from).is_err(),
            "from path should not exist"
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
}
