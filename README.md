# toy_rustpython

[![Build Status](https://travis-ci.com/rawrgulmuffins/toy_rustpython.svg?branch=master)](https://travis-ci.com/rawrgulmuffins/toy_rustpython)
[![Crate](https://img.shields.io/crates/v/toy_rustpython.svg)](https://crates.io/crates/toy_rustpython)
[![API](https://docs.rs/toy_rustpython/badge.svg)](https://docs.rs/toy_rustpython)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/rawrgulmuffins/toy_rustpython.svg)](http://isitmaintained.com/project/rawrgulmuffins/toy_rustpython)
[![Percentage of issues still open](http://isitmaintained.com/badge/open/rawrgulmuffins/toy_rustpython.svg)](http://isitmaintained.com/project/rawrgulmuffins/toy_rustpython)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.32.0+-lightgray.svg)](https://github.com/rawrgulmuffins/toy_rustpython#minimum-supported-rust-version-msrv)

This is a learning project where the point is to learn the Rust programming language by making a toy Python compiler

# Goals
* Be a small enough project that I'll actually finish it.
* Learn Rust
* Learn How Python ByteCode works (maybe?).
* Learn how to implement a REPL.
* Be expandable if I want to continue working on this.
* Learn how to use Docker for local testing and how CircleCI or Github Actions
can also use the same Docker Images.
* Build a test suite that can be re-used if I want to use this to learn other languages.

# Specific Features
## Operators
* `+`
* `-`
* `*`
* `**`
* `%`
* `/`
* `//`
* `=`
## Statements
* `if`
* `elif`
* `else`
* `print`
* `def`
* `return`

## Other In Scope Features
* Python REPL for all keywords.
* Run Programs from files.
* CircleCI or github Actions running fmt, tests, and crates upload.

### Maybe Later
* `for`
* `while`
* `globals`
* `locals`
* `del`
* Dict types and `[` `]`
* `(` `)`
* `==` and other logical comparisons
* all of the operator =s (`*=`, `-=`, etc.)
* indent syntax error
* objects.
* dot traversal (object.method, object.object.attribute)


# Resources
[CPython Tokens](https://github.com/python/cpython/blob/master/Grammar/Tokens)
[CPython Grammar](https://github.com/python/cpython/blob/master/Grammar/python.gram)
