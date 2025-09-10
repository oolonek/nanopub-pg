nanopub-pg
===========

Small CLI to play with the nanopub-rs library: fetch and check Nanopublications from a URI or a local TriG/TriX file.

Usage
-----

- Build and run with a sample URI:
  - `cargo run`

- Check one or more Nanopubs by URI:
  - `cargo run -- https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw https://w3id.org/np/RA...`

- Check a local file (TriG/TriX) containing a single Nanopublication:
  - `cargo run -- ./path/to/np.trig`

What it does
------------

- For URIs: asynchronously fetches the Nanopub via `Nanopub::fetch`, then validates it with `check()`.
- For files: loads the RDF into `Nanopub::new`, then validates with `check()`.
- Prints a concise `NpInfo` summary and the RDF size.

References
----------

- Docs: https://vemonet.github.io/nanopub-rs/packages/#check-nanopubs
- Crate: https://crates.io/crates/nanopub
