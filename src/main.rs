use anyhow::Result;
use clap::Parser;
use std::{fs, path, process};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'f', long = "file")]
    file: path::PathBuf,
}

const DG_ERROR: &str = "//~^ ERROR ";

fn main() {
    if let Err(error) = try_parse() {
        eprintln!("Error: {:?}", error);
        process::exit(1);
    }
}

fn try_parse() -> Result<()> {
    let args = Cli::parse();

    if !args.file.exists() {
        panic!("File {:?} not found", args.file);
    }

    let code = fs::read_to_string(&args.file).expect("Failed to read file");

    let new_code = transform_code(&code, DG_ERROR);

    fs::remove_file(&args.file).expect("Failed to delete file");
    fs::write(&args.file, new_code.join("\n")).expect("Failed to write to file");

    Ok(())
}

fn transform_code(code: &String, dg_directive: &str) -> Vec<String> {
    let mut new_code = Vec::new();

    for line in code.lines() {
        if line.contains(dg_directive) {
            // replace the rust directive to dejagnu directive
            // TODO: Add more directive relative to rustc
            let new_line = line.replace(dg_directive, "// { dg-error \"");

            // format the line according to dejagnu format
            let new_line = format!("{}\" }}", new_line);
            new_code.push(new_line);
        } else {
            new_code.push(line.to_string());
        }
    }
    new_code
}
}
