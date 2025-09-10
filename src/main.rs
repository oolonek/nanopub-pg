mod app;
mod args;
mod loader;
mod renderer;

fn main() {
    let opts = args::parse_args();
    app::run(opts);
}
