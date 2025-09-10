use nanopub::Nanopub;
use std::env;
use std::fs;

fn main() {
    // Collect inputs: URIs or local TriG/TriX files containing a single Nanopub
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    // Default example URI if nothing provided
    if args.is_empty() {
        args.push("https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw".to_string());
        eprintln!("No input provided. Using example URI.\n");
    }

    // Use a Tokio runtime to support async fetch
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    for input in args {
        println!("----\nInput: {}", input);

        // Decide whether this is a URI or a file path
        let is_uri = input.starts_with("http://") || input.starts_with("https://");

        // Load or fetch the Nanopub
        let np_result = if is_uri {
            rt.block_on(async { Nanopub::fetch(&input).await })
        } else {
            match fs::read_to_string(&input) {
                Ok(rdf) => Nanopub::new(rdf.as_str()),
                Err(e) => Err(nanopub::NpError(format!(
                    "Failed to read file '{}': {}",
                    input, e
                ))),
            }
        };

        match np_result {
            Ok(np) => {
                // Validate the nanopub (checks trusty hash and signature when present)
                match np.check() {
                    Ok(checked) => {
                        // Print a concise info summary
                        println!("{}", checked.info);
                        match checked.rdf() {
                            Ok(rdf) => println!("RDF length: {} bytes", rdf.len()),
                            Err(e) => eprintln!("Could not serialize RDF: {}", e.0),
                        }
                    }
                    Err(e) => eprintln!("❌ Check failed: {}", e.0),
                }
            }
            Err(e) => eprintln!("❌ Could not load Nanopub: {}", e.0),
        }
    }
}
