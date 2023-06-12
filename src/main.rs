mod io;

use std::{env, process};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 3 {
        eprintln!("Usage: ./swapfc <source> <destination>");
        process::exit(1);
    }

    let source = &args[1];
    let dest = &args[2];

    let source_path = Path::new(&source);
    let dest_path = Path::new(&dest);

    if !source_path.try_exists().or(dest_path.try_exists()).unwrap() {
        eprintln!("Error: both paths should exist");
        process::exit(1);
    }

    if !source_path.is_file() || !dest_path.is_file() {
        eprintln!("Error: source and destination need to be files");
        process::exit(1);
    }

    if source.eq(dest) {
        eprintln!("Error: source has same destination");
        process::exit(1);
    }

    if !contains_same_extention(&source_path, &dest_path) {
        eprintln!("Error: source and destination does not have same extension");
        process::exit(1);
    }

    io::swap_file_content(&source, &dest).unwrap_or_else(|err| {
        eprintln!("Could not execute swapfc properly: {:?}", err);
        process::exit(1);
    });

    process::exit(0);
}

fn contains_same_extention(s1: &Path, s2: &Path) -> bool {
    let opt_ext1 = s1.extension();
    let opt_ext2 = s2.extension();

    opt_ext1.eq(&opt_ext2)
}
