use punjabi_lang::error::PunjabiError;
use punjabi_lang::interpreter::Interpreter;
use punjabi_lang::{parse, run_source, tokenize};
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    if let Err(error) = run_cli() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run_cli() -> Result<(), PunjabiError> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [] => print_help(),
        [flag, path] if flag.as_str() == "--tokens" => show_tokens(path),
        [flag, path] if flag.as_str() == "--ast" => show_ast(path),
        [command] if command.as_str() == "repl" => repl(),
        [command, path] if command.as_str() == "run" => run_file(path),
        _ => {
            print_help()?;
            Err(PunjabiError::usage(
                "Command samajh nai aayi. Upar wali examples vicho koi command chalao.",
            ))
        }
    }
}

fn print_help() -> Result<(), PunjabiError> {
    println!("Punjabi Programming Language");
    println!();
    println!("Commands:");
    println!("  cargo run -- run examples/hello.pun");
    println!("  cargo run -- repl");
    println!("  cargo run -- --tokens examples/hello.pun");
    println!("  cargo run -- --ast examples/hello.pun");
    Ok(())
}

fn read_file(path: &str) -> Result<String, PunjabiError> {
    fs::read_to_string(path).map_err(|error| {
        PunjabiError::usage(format!("File '{path}' read nai hoi: {error}"))
    })
}

fn run_file(path: &str) -> Result<(), PunjabiError> {
    let source = read_file(path)?;
    for line in run_source(&source)? {
        println!("{line}");
    }
    Ok(())
}

fn show_tokens(path: &str) -> Result<(), PunjabiError> {
    let source = read_file(path)?;
    for token in tokenize(&source)? {
        println!("{token:?}");
    }
    Ok(())
}

fn show_ast(path: &str) -> Result<(), PunjabiError> {
    let source = read_file(path)?;
    for statement in parse(&source)? {
        println!("{statement:#?}");
    }
    Ok(())
}

fn repl() -> Result<(), PunjabiError> {
    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();

    println!("Punjabi REPL. Exit layi 'bahar' likho.");
    loop {
        print!("punjabi> ");
        io::stdout()
            .flush()
            .map_err(|error| PunjabiError::usage(format!("Prompt print nai hoya: {error}")))?;

        let mut line = String::new();
        stdin
            .read_line(&mut line)
            .map_err(|error| PunjabiError::usage(format!("Input read nai hoya: {error}")))?;

        if line.trim() == "bahar" {
            break;
        }

        match parse(&line).and_then(|statements| interpreter.run(&statements)) {
            Ok(output) => {
                for value in output {
                    println!("{value}");
                }
            }
            Err(error) => eprintln!("{error}"),
        }
    }

    Ok(())
}
