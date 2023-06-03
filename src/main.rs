mod io;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 3 {
        todo!();
    }

    let source = &args[1];
    let dest = &args[2];

    if source.eq(dest) {
        todo!();
    }

    io::swap_file_content(&source, &dest).unwrap_or_else(|err| {
        eprintln!("Could not execute swapfc: {:?}", err);
        std::process::exit(1);
    });

    std::process::exit(0);
}
