- ✓ rename &str converter function
- ✓ make PathBuf converter function
- ✓ get that CLI working
- ✓ make rename function that takes separate from and to
- ✓ add --rename command line argument
- ✓ add usage reporting
- convert underscore to dash
- Try a ToSlug trait with to_slug function mirroring ToString
  - implement for String and PathBuf
- add CLI test with fake stdin iterator
- consider https://github.com/rust-cli/paw for command line arg parsing and printing
  - Need custom error type for printing error messages

## Testable Main Subproject

- map Result from main to zero/non-zero exit code
- Custom error type with user-presentable message
- Custom exit code associated with an error
- stdio function argument struct
- stdio return struct
