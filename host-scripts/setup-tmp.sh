#!/usr/bin/env bash

# Please Use Google Shell Style: https://google.github.io/styleguide/shell.xml

# ---- Start unofficial bash strict mode boilerplate
# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -o errexit    # always exit on error
set -o errtrace   # trap errors in functions as well
set -o pipefail   # don't ignore exit codes when piping output
set -o posix      # more strict failures in subshells
# set -x          # enable debugging

IFS="$(printf "\n\t")"
# ---- End unofficial bash strict mode boilerplate

mkdir -p /tmp/slugger-tests
cd /tmp/slugger-tests
mkdir -p "./dir a/dir aa/dir bb"
touch "./dir a/dir aa/dir bb/FILE 1.txt"
touch "./dir a/dir aa/dir bb/FILE 2.txt"
touch "./dir a/dir aa/dir bb/FILE 3$.txt"
