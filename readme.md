# slugger

A unix filesystem utility to rename files using a restricted url-safe character subset.

## Why slug your file paths?

* Avoid bugs and vulnerabilities due to poor handling of problematic characters including spaces, quotes, etc
* Simplify things by sticking with lowercase

## Development Plans and Notes to Self

These are for me not you. No need to read them.

1. Given a path, generate a sorted list of paths in depth-first order safe to sluggify
1. Given a list of paths, generate a list of from/to slugs
1.

```
unit-test/sub 1/z z
unit-test/sub 1/a a
unit-test/sub 2/b b
unit-test/sub 2/y y
unit-test
unit-test/a a
unit-test/b b.txt

```
