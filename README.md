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


License
-------

MIT or Apache2, at your convenience.
