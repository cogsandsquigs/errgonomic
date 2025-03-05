# `errgonomic`

A simple, powerful, and fun combinator parsing library for Rust.

## Why `errgonomic`?

This is essentially a hobby crate for me, but it was designed to solve a few issues:

1. Parsers should, by default, produce nice and readable errors. Nice enough that they can be used as a stand-in for
   custom parser errors!
2. Error-handling in `nom` (the major Rust parser-combinator crate) is kind-of lacking.
3. I wanted to build a parser-combinator that I can trust (because I wrote it!)

So, I built `errgonomic` with these core tenets in mind:

- **Usable and fun**: I went into this project wanting to use it. Therefore, the primary goal is to be as usable as
  possible and as easy to write parsers with as can be.
- **Errors are first-class**: This library was built with errors in mind. Therefore, custom error types and other things
  are supported outright. In the near (!) future, this library will have support for native error-recovery, including
  panic- and statement-mode recoveries, making handling complex error cases a breeze!
- **Dependency-Free**: By default, there are no dependencies in `errgonomic`. In the future, there will be a
  feature-flag to link in `thiserror` and `miette` for better, more featureful results, but that will, of course, be
  optional.
- **Fast**: While this isn't really that optimized, `errgonomic` uses immutable state and has minimal amounts
  of `clone`-s, therefore making it faster. Plus, it's built in Rust ;).

### Pitfalls

- **Built over binary**: I developed this library primarily to parse strings, but underneath the hood it is all
  essentially parsing binary. ~Unicode, at the moment, is effectively second-class. However, there is work underway to
  make Unicode supported (through a `unicode` feature-flag)~ The `unicode` feature-flag is ready for testing! It should
  enable unicode processing for all things.
- **Types are annoying**: Due to the way certain things work, you may have to specify types outright for the parsers
  (especially the custom-error type). This doesn't actually hinder parsing in any way, it just may make your source code
  uglier in certain scenarios. See `errgonomic::combinators` for more information.

## Feature-flags

- `unicode`: Enables Unicode support, and processes the bytes essentially as characters.

> [WARN!]
> Will decrease performance! Using a custom allocator, such as [mimalloc](https://github.com/purpleprotocol/mimalloc_rust) or [jemalloc](https://github.com/tikv/jemallocator) may improve performance.

> [WARN!]
> This is an experimental flag! It may not enable unicode support for _all_ parser combinators yet, that is still in
> progress.
> TODO: Convert all things that use bytes (e.g. numeric stuff) to unicode (parse chars instead)

  <!-- - `fancy`: Enables support for `miette`, and enables `miette::Diagnostic` for `Error` and `Errors`. NOTE: Requires
  anything implementing `CustomError` to implement `miette::Diagnostic` and `core::error::Error`. This also disables
  support for parsing bytes, i.e. `[u8]`. WARN: This feature is not stable yet!
  -->

## Contributing

Right now, there really isn't any spots available for contribution. However, you can always write an issue for any
feature/parser combinator/request you have, and I'll gladly take a look! Just remember that this is a side-project for
me, and thus I may not accept or even respond to your issue.

## TODO:

- [ ] Add unicode `char` buffer in input for unicode parsing.
- [ ] Add panic- and statement-mode recoveries.
- [ ] Add more unicode support to parsers that need it (if it accesses raw binary).
