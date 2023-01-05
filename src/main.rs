use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to input file to use.
    input: String,

    /// Flag to compile rust file from brainfuck.
    #[arg(short, long, default_value_t = false)]
    compile: bool,

    /// Flag to run Rust file.
    #[arg(short, long, default_value_t = false)]
    run: bool,
}

fn compile(input: String) -> String {
    let mut output = String::new();
    output.push_str("#![allow(unused)]");
    output.push_str("use std::io::{stdin, stdout, Read, Write};");
    output.push_str("fn main() {");
    output.push_str("let mut pointer = 0usize;");
    output.push_str("let mut data = [0u8; 30000];");
    for command in fs::read_to_string(input).unwrap().chars() {
        match command {
            '>' => output.push_str("pointer += 1;"),
            '<' => output.push_str("pointer -= 1;"),
            '+' => output.push_str("data[pointer] = data[pointer].wrapping_add(1);"),
            '-' => output.push_str("data[pointer] = data[pointer].wrapping_sub(1);"),
            '.' => output.push_str("print!(\"{}\", data[pointer] as char);"),
            ',' => {
                output.push_str("stdout().flush().unwrap();");
                output.push_str("data[pointer] = stdin().bytes().next().unwrap().unwrap();")
            }
            '[' => output.push_str("while data[pointer] > 0 {"),
            ']' => output.push('}'),
            _ => {}
        }
    }
    output.push('}');
    output
}

fn main() {
    let args = Args::parse();

    if args.compile {
        // Delete old Cargo project.
        fs::remove_dir_all("output").ok();

        // Initialize new Cargo project for output.
        Command::new("cargo")
            .arg("init")
            .arg("output")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        // Write main.rs file.
        let mut file = File::create("output/src/main.rs").unwrap();
        file.write_all(compile(args.input).as_bytes()).unwrap();

        // Format the file.
        Command::new("cargo")
            .arg("fmt")
            .arg("--manifest-path")
            .arg("output/Cargo.toml")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    if args.run {
        // Run Cargo project.
        Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg("--manifest-path")
            .arg("output/Cargo.toml")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
