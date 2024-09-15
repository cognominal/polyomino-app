// This is a sample Rust program that demonstrates different ways to run tasks in Visual Studio Code.
// need to hook it function in gen_polyominoes.rs
use std::env;
mod gen_polyominos;
use gen_polyominos::generate_pentominoes;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("generate-pentominoes") => {
            let pentominoes = generate_pentominoes(5);
            println!("Generated {} pentominoes:", pentominoes.len());
        },
        Some("run-tests") => {
            // Run tests here
        },
        _ => {
            println!("Default main function");
        }
    }
}
