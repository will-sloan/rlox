use crate::Scanner;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use text_io::read;

static mut HADERROR: bool = false;

pub fn run_lox(args: Vec<String>) -> std::io::Result<()> {
    // if args.len() > 2 {
    //     panic!("Too many arguments, usage 'jlox [script]");
    // } else if args.len() == 2 {
    //     let _ = run_file(args[1].clone());
    // } else {
    //     run_prompt();
    // }
    let _ = match args.len() {
        x if x > 2 => panic!("Too many arguments, usage 'jlox [script]"),
        2 => run_file(args[1].clone()),
        _ => run_prompt(),
    };
    Ok(())
}
fn run_prompt() -> std::io::Result<()> {
    println!("Run prompt");
    loop {
        let line: String = read!("{}\n");
        if line.is_empty() {
            break;
        }
        run(line);
        unsafe {
            HADERROR = false;
        }
    }
    Ok(())
}

fn run_file(path: String) -> std::io::Result<()> {
    println!("Run file, {}", path);
    let mut file = File::open(path.as_str())?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    println!("Contents of file: {:?}", contents);
    run(contents);

    unsafe {
        if HADERROR {
            process::exit(1);
        }
    }
    Ok(())
}

fn run(input: String) {
    let mut scanner = Scanner::new(input);
    scanner.scan_tokens();

    for token in scanner.get_tokens().iter() {
        println!("Got Token: {}", token);
    }
}

pub fn error(line: u32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: u32, loc: String, message: String) {
    println!("[line {}] Error {}: {}", line, loc, message);
    unsafe {
        HADERROR = true;
    }
}
