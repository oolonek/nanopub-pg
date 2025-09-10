use nanopub::{NpError, Nanopub};
use std::fs;

pub fn is_uri(input: &str) -> bool {
    input.starts_with("http://") || input.starts_with("https://")
}

pub fn load_nanopub<'a>(input: &str, rt: &tokio::runtime::Runtime) -> Result<Nanopub, NpError> {
    if is_uri(input) {
        rt.block_on(async { Nanopub::fetch(input).await })
    } else {
        let rdf = fs::read_to_string(input)
            .map_err(|e| NpError(format!("Failed to read file '{}': {}", input, e)))?;
        Nanopub::new(rdf.as_str())
    }
}

