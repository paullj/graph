use std::env;

use graph_core;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filename as a command line argument.");
        return;
    }

    let filename = &args[1];

    match graph_core::generate_graph(filename) {
        Ok(document) => {
            println!("{document}")
        }
        Err(_) => println!("Error!"),
    }
}
