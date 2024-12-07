DAPPER -- Data Access Protocol Provisions Exciting Raccoons
======

... okay, you can see I got a little carried away with the attempt at an acronym there.
Nevertheless!

Dapper is a system of:

- Schemas
- For rapid application and protocol development
- Aimed at config and business logic, and wire formats and datawarehousing alike.

At the core are Dapper Schemas and the Dapper Data Model, and serialization systems using them.

Dapper serialization is _schema free_, and closely analogous to JSON (but with a binary form, so it's a good bit faster).
Being schema-free means Dapper serialized data can always be read (and hopefully, understood) even without any schema in hand.

But application and protocol development often is easier with some kind of schemas as a design language, so we have those too.


Status
------

Pre-alpha, early R&D.

The schema language is the current target.  Experiments are underway using tree-sitter as the core.

The data model and codecs can be assumed to be roughly the same as the IPLD Data Model and DAG-CBOR, respectively.

Expect development to prioritize things that are on the path to protocol design facilitations moreso than the low level bits.
(Codecs are a SMOP.  An important one, but a relatively well-known space.  We can do those later without a serious risk of problematic unanticipated design feedback cycle.)


Building
--------

This project uses a LOT of tooling from various sources.
The setup may be nontrivial.

### for tree-sitter

There's a tree-sitter CLI tool; you'll need that to compile the grammar
and generate the parser (which comes out as a C file).

And to use that parser, you'll probably need `clang` on your path,
since it's a C file we're talking about.

### for the wasm and web stuff

For this, you'll need a **rust nightly**.
(This is because we need the `-Zwasm-c-abi=spec` flag;
without it, wasm'ify'd rust can't call wasm'ify'd C correctly.
To the best of my knowledge, there's simply no way to address this
aside from going to a nightly rust.)

And you'll need the wasm-pack tool.  That's fairly easy to get:

```
cargo install wasm-pack
```

And you'll need your rust install to have the materials for wasm targets.
The `wasm-pack` tool will either get that for you automatically (if you used `rustup`),
or offer you instructions to follow if you need to do it manually.

And you'll also still need clang.
And probably a fairly recent version: it needs to have wasm32 support,
and needs to be in reasonable alignment to what rust's wasm stuff is expecting.


License
-------

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
