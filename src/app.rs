use crate::args::Options;
use crate::loader::load_nanopub;
use crate::renderer::mermaid_from_nanopub;
// use nanopub::Nanopub; // not needed in this module

pub fn run(opts: Options) {
    let mut inputs = opts.inputs;
    if inputs.is_empty() {
        inputs.push(
            "https://w3id.org/np/RATGmPlZuuhgKAcqSICT4Qg_J9z5N9rVQbdGt4hJ7yMJM".to_string(),
        );
        if !opts.mermaid {
            eprintln!("No input provided. Using example URI.\n");
        }
    }

    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    for (idx, input) in inputs.iter().enumerate() {
        if !opts.mermaid {
            println!("----\nInput: {}", input);
        }

        match load_nanopub(input, &rt) {
            Ok(np) => {
                if opts.mermaid {
                    let mmd = mermaid_from_nanopub(&np);
                    if inputs.len() == 1 {
                        println!("{}", mmd);
                    } else {
                        let fname = format!("np_{idx}.mmd");
                        if let Err(e) = std::fs::write(&fname, mmd) {
                            eprintln!("Failed to write {}: {}", fname, e);
                        }
                    }
                } else {
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
