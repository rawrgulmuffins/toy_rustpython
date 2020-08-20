# toy_rustpython
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

## Other Features
* Python REPL for all keywords.
* Run Programs from files.

# Resources
[CPython Tokens](https://github.com/python/cpython/blob/master/Grammar/Tokens)
[CPython Grammar](https://github.com/python/cpython/blob/master/Grammar/python.gram)
