use std::env;

pub struct Options {
    pub mermaid: bool,
    pub inputs: Vec<String>,
}

pub fn parse_args() -> Options {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    let mut mermaid = false;

    if !args.is_empty() && args[0] == "--mermaid" {
        mermaid = true;
        args.remove(0);
    }

    Options {
        mermaid,
        inputs: args,
    }
}

