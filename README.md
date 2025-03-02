# `errgonomic`

A simple, powerful, and fun combinator parsing library for Rust.

## Why `errgonomic`?

This is essentially a hobby crate for me, but it was designed to solve a few issues:

1. Parsers should, by default, produce nice and readable errors. Nice enough that they can be used as a stand-in for
   custom parser errors!
2. Error-handling in `nom` (the major Rust parser-combinator crate) is kind-of lacking.
3. I wanted to build a parser-combinator that I can trust (because I wrote it!)

## Features

- **Usable and Fun**: I went into this project wanting to use it. Therefore, the primary goal is to be as usable as
  possible and as easy to write parsers with as can be.
- **Dependency-Free**: By default, there are no dependencies in `errgonomic`. In the future, there will be a
  feature-flag to link in `thiserror` and `miette` for better, more featureful results.
- **Fast**: While this isn't really that optimized, `errgonomic` uses immutable state and has minimal amounts
  of `clone`-s, therefore making it faster. Plus, it is built in Rust ;).

## Contributing

Right now, there really isn't any spots available for contribution. However, you can always write an issue for any
feature/parser combinator/request you have, and I'll gladly take a look! Just remember that this is a side-project for
me, and thus I may not accept or even respond to your issue.
