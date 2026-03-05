use std::{
    env,
    io,
    fs,
    path::{Path, PathBuf},
    process,
    ffi::OsStr,
};

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: You must provide a directory path!");
        eprintln!("Usage: cargo run -- .");
        process::exit(1);
    };

    let directory_path = &args[1];

    let entries = fs::read_dir(&directory_path).unwrap_or_else(|err| {
        eprintln!("Error reading directory!: {} {}", directory_path, err);
        process::exit(1);
    });

    println!("Scanning directory: {}", directory_path);

    for entry in entries {
        let entry = entry.unwrap_or_else(|error| {
            eprintln!("Error reading the file: {}", error);
            process::exit(1);
        });

        let path = entry.path();
        println!("Found!: {}", path.display());
    }
}