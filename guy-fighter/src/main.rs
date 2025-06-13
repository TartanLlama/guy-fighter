use clap::Parser;
use std::path::PathBuf;

mod visualization;
mod game;
mod names;

#[derive(Parser)]
#[command(name = "guy-fighter")]
#[command(about = "A guy fighting game with plugin support")]
struct Args {
    #[arg(long, help = "Plugin directory")]
    plugins: Option<PathBuf>,
}

fn main() -> wasmtime::Result<()> {
    let args = Args::parse();

    if args.plugins.is_none() {
        eprintln!("Usage: guy-fighter --plugins <path_to_plugins>");
        std::process::exit(1);
    }

    game::run_game(&args.plugins.unwrap())
}
