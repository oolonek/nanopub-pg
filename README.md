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

- Output a Mermaid graph of the Nanopub (to stdout or files):
  - `cargo run -- --mermaid https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw > np.mmd`
  - For multiple inputs, files `np_0.mmd`, `np_1.mmd`, … are created.
  - View with VS Code Mermaid preview or render via `mmdc` (Mermaid CLI).

Example
-------

Mermaid example (trimmed) showing ~~colored~~ subgraphs for Head, Assertion, Provenance and PubInfo. Nodes are colored per subgraph for readability.

```mermaid
graph LR
  subgraph head
    N0["RATGmPlZuuhgKAcqSICT4Qg_J9z5N9rVQbdGt4hJ7yMJM"]
    N1["Nanopublication"]
    N2["assertion"]
    N3["provenance"]
    N4["pubinfo"]
    N0 -- type --> N1
    N0 -- hasAssertion --> N2
    N0 -- hasProvenance --> N3
    N0 -- hasPublicationInfo --> N4
  end
  subgraph assertion
    N5["association"]
    N6["OrganismTaxonToChemicalAssociation"]
    N7["Q3006048"]
    N8["RO_0002162"]
    N9["Q105674316"]
    N5 -- type --> N6
    N5 -- object --> N7
    N5 -- predicate --> N8
    N5 -- subject --> N9
  end
  subgraph provenance
    N10["research-activity"]
    N11["ol2030907"]
    N12["ScholarlyWork"]
    N2["assertion"]
    N2 -- wasGeneratedBy --> N10
    N11 -- type --> N12
    N10 -- description --> N11
  end
  subgraph pubinfo
    N0["RATGmPlZuuhgKAcqSICT4Qg_J9z5N9rVQbdGt4hJ7yMJM"]
    N13["2024-02-07T10:09:11.310Z"]
    N14["0000-0003-3389-2191"]
    N15["(empty)"]
    N16["BiodivNanopub"]
    N17["trigocherrin A in taxon Trigonostemon cherrieri"]
    N18["RA_bMeVDu1KMzSAdjjedJfMFgcyGZ-G3FbELL6J1vFZ_o"]
    N19["RAA2MfqdBCzmz9yVWjKLXNbyfBNcwsMmOqcNUxkk1maIM"]
    N20["RAh1gm83JiG5M6kDxXhaYT1l49nCzyrckMvTzcPn-iv90"]
    N21["RABTEyo9MqchL1TXRTnaND6ISSV-FDd5IY1Wl1g4E7BOw"]
    N22["Trigonostemon cherrieri - species of plant"]
    N23["trigocherrin A - chemical compound"]
    N24["sig"]
    N25["RSA"]
    N26["MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCNgV7KvEdXGXu+GLDP17lXL..."]
    N27["JGXsSD55f/hTJtyVxldEda0icW/iz+7SiaiZpRqCOoQ8VeQE/x+8UJ4QwcUjP..."]
    N5["association"]
    N7["Q3006048"]
    N9["Q105674316"]
    N0 -- created --> N13
    N0 -- creator --> N14
    N0 -- license --> N15
    N0 -- hasNanopubType --> N16
    N0 -- introduces --> N5
    N0 -- label --> N17
    N0 -- wasCreatedFromProvenanceTemplate --> N18
    N0 -- wasCreatedFromPubinfoTemplate --> N19
    N0 -- wasCreatedFromPubinfoTemplate --> N20
    N0 -- wasCreatedFromTemplate --> N21
    N7 -- hasLabelFromApi --> N22
    N9 -- hasLabelFromApi --> N23
    N24 -- hasAlgorithm --> N25
    N24 -- hasPublicKey --> N26
    N24 -- hasSignature --> N27
    N24 -- hasSignatureTarget --> N0
  end
```

What it does
------------

- For URIs: asynchronously fetches the Nanopub via `Nanopub::fetch`, then validates it with `check()`.
- For files: loads the RDF into `Nanopub::new`, then validates with `check()`.
- Prints a concise `NpInfo` summary and the RDF size.

References
----------

- Docs: https://vemonet.github.io/nanopub-rs/packages/#check-nanopubs
- Crate: https://crates.io/crates/nanopub

Project Structure
-----------------

- `src/main.rs` — thin entry point wiring modules together.
- `src/args.rs` — minimal CLI parsing (`--mermaid`, inputs).
- `src/app.rs` — app orchestration: runtime, per-input processing, calling render/validate.
- `src/loader.rs` — load a `Nanopub` from URI (async fetch) or local file.
- `src/renderer.rs` — Mermaid graph rendering (grouped subgraphs and colors).

Contributing
------------

- Keep modules focused (I/O, rendering, orchestration).
- Avoid printing in library-like modules (renderer/loader), return strings/results instead. The app layer decides what to print.
- Prefer small functions with clear inputs/outputs; no global state.
