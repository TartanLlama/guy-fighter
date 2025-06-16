mod visualization;
mod game;
mod names;

fn main() -> wasmtime::Result<()> {
    game::run_game()
}
