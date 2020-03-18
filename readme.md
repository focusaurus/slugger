# slugger

A unix filesystem utility to rename files using a restricted url-safe character subset.

## Why slug your file paths?

* Avoid bugs and vulnerabilities due to poor handling of problematic characters including spaces, quotes, etc
* Simplify things by sticking with lowercase
* Allow paths to be typed without keyboard gymnastics
* Allow easy tab completion

## Usage
 
Slugger reads filesystem paths from standard input, one per line. In the default mode, it just prints out the slugged versions. If you pass the `--rename` command line argument, slugger will do the file renames.
Typically the input comes from commands that output lists of files like `find`, `ls`, etc.

- Preview slugged names without doing anything: `find . -type f | slugger`
- Do the actual renaming: `find . -type f | slugger --rename`

## How files are renamed

- Convert to lowercase
- Trim leading and trailing whitespace
- Remove repeated dashes
- Do a "unidecode" to translate mostly to ASCII
- Convert all whitespaces to dash
- Delete anything else

## Behavior notes

- slugger will not clobber existing files. If the slugged file already exists, it will abort and exit with an error code.
- slugger is safe to run on already-slugged files. It will no-op and exit success (it's idempotent).

## How correct and stable is it?

Uh, it has some decent unit tests but it's brand new and should be considered unreliable for now.
