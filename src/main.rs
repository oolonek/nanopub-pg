use nanopub::Nanopub;
use sophia::api::dataset::Dataset; // trait for iterating quads
use sophia::api::quad::Quad; // quad accessor methods s/p/o/g
use sophia::api::term::Term;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    // Collect inputs: URIs or local TriG/TriX files containing a single Nanopub
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    // Default example URI if nothing provided
    if args.is_empty() {
        args.push("https://w3id.org/np/RATGmPlZuuhgKAcqSICT4Qg_J9z5N9rVQbdGt4hJ7yMJM".to_string());
        eprintln!("No input provided. Using example URI.\n");
    }

    // Flags
    let mut output_mermaid = false;
    if !args.is_empty() && args[0] == "--mermaid" {
        output_mermaid = true;
        args.remove(0);
    }

    // Use a Tokio runtime to support async fetch
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    for (idx, input) in args.iter().enumerate() {
        if !output_mermaid {
            println!("----\nInput: {}", input);
        }

        // Decide whether this is a URI or a file path
        let is_uri = input.starts_with("http://") || input.starts_with("https://");

        // Load or fetch the Nanopub
        let np_result = if is_uri {
            rt.block_on(async { Nanopub::fetch(input).await })
        } else {
            match fs::read_to_string(input) {
                Ok(rdf) => Nanopub::new(rdf.as_str()),
                Err(e) => Err(nanopub::NpError(format!(
                    "Failed to read file '{}': {}",
                    input, e
                ))),
            }
        };

        match np_result {
            Ok(np) => {
                if output_mermaid {
                    // In graph mode, avoid extra stdout from check(); just render the dataset.
                    let mmd = mermaid_from_nanopub(&np);
                    if args.len() == 1 {
                        println!("{}", mmd);
                    } else {
                        let fname = format!("np_{idx}.mmd");
                        if let Err(e) = fs::write(&fname, mmd) {
                            eprintln!("Failed to write {}: {}", fname, e);
                        } else if !output_mermaid {
                            println!("Mermaid saved to {}", fname);
                        }
                    }
                } else {
                    // Validate the nanopub (checks trusty hash and signature when present)
                    match np.check() {
                        Ok(checked) => {
                            println!("{}", checked.info);
                            match checked.rdf() {
                                Ok(rdf) => println!("RDF length: {} bytes", rdf.len()),
                                Err(e) => eprintln!("Could not serialize RDF: {}", e.0),
                            }
                        }
                        Err(e) => eprintln!("❌ Check failed: {}", e.0),
                    }
                }
            }
            Err(e) => eprintln!("❌ Could not load Nanopub: {}", e.0),
        }
    }
}

fn mermaid_from_nanopub(np: &Nanopub) -> String {
    // Build ids for nodes and edges grouped by graph
    let mut id_map: HashMap<String, String> = HashMap::new();
    let mut next_id: usize = 0;
    let mut node_defs: HashMap<&'static str, Vec<String>> = HashMap::new();
    let mut edges: HashMap<&'static str, Vec<String>> = HashMap::new();

    // iterate quads and group by graph

    let mut get_node_id = |label: &str| -> String {
        if let Some(id) = id_map.get(label) {
            return id.clone();
        }
        let id = format!("N{}", next_id);
        next_id += 1;
        id_map.insert(label.to_string(), id.clone());
        id
    };

    for res in np.dataset.quads() {
        if let Ok(q) = res {
            let gname = {
                let g = q.g();
                if let Some(iri) = g.and_then(|gn| gn.iri()) {
                    let gstr = iri.as_str();
                    if gstr == np.info.assertion.as_str() {
                        "assertion"
                    } else if gstr == np.info.prov.as_str() {
                        "provenance"
                    } else if gstr == np.info.pubinfo.as_str() {
                        "pubinfo"
                    } else if gstr == np.info.head.as_str() {
                        "head"
                    } else {
                        "other"
                    }
                } else {
                    "other"
                }
            };

            let s_label = term_label(q.s());
            let p_iri_str = q
                .p()
                .iri()
                .map(|i| i.as_str().to_owned())
                .unwrap_or_default();
            let p_label = pred_label(&p_iri_str);
            let o_label = term_label(q.o());

            let sid = get_node_id(&s_label);
            let oid = get_node_id(&o_label);

            // Define nodes with readable labels
            node_defs
                .entry(gname)
                .or_default()
                .push(format!("{}[\"{}\"]", sid, escape_mermaid(&s_label)));
            node_defs
                .entry(gname)
                .or_default()
                .push(format!("{}[\"{}\"]", oid, escape_mermaid(&o_label)));

            edges
                .entry(gname)
                .or_default()
                .push(format!("{} -- {} --> {}", sid, escape_mermaid(&p_label), oid));
        }
    }

    // Deduplicate node defs per graph
    for defs in node_defs.values_mut() {
        defs.sort();
        defs.dedup();
    }

    let mut out = String::new();
    out.push_str("graph LR\n");

    for gname in ["head", "assertion", "provenance", "pubinfo", "other"] {
        if let Some(defs) = node_defs.get(gname) {
            if !defs.is_empty() {
                out.push_str(&format!("  subgraph {}\n", gname));
                for n in defs {
                    out.push_str("    ");
                    out.push_str(n);
                    out.push_str("\n");
                }
                if let Some(es) = edges.get(gname) {
                    for e in es {
                        out.push_str("    ");
                        out.push_str(e);
                        out.push_str("\n");
                    }
                }
                out.push_str("  end\n");
            }
        }
    }

    out
}

fn term_label<T: Term>(t: T) -> String {
    if let Some(iri) = t.iri() {
        compact_iri(iri.as_str())
    } else if let Some(lit) = t.lexical_form() {
        let mut s = lit.to_string();
        if s.trim().is_empty() {
            return "(empty)".to_string();
        }
        if s.len() > 64 {
            s.truncate(61);
            s.push_str("...");
        }
        s
    } else {
        "_:bnode".to_string()
    }
}

fn pred_label(p: &str) -> String {
    compact_iri(p)
}

fn compact_iri(s: &str) -> String {
    let mut label = s;
    if let Some(pos) = s.rfind('#') {
        label = &s[pos + 1..];
    } else if let Some(pos) = s.rfind('/') {
        label = &s[pos + 1..];
    }
    label.to_string()
}

fn escape_mermaid(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return "(empty)".to_string();
    }
    trimmed.replace('"', "\\\"")
}
