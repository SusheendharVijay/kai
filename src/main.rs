use std::{env, fs, io::Write, path::Path};

// mod scanner;
// mod token;

use ckai::parser::Parser;
use ckai::scanner::Scanner;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("usage kai [script]!")
    } else if args.len() == 2 {
        run_script(Path::new(args[1].as_str()))?;
    } else {
        run_prompt()?
    }

    Ok(())
}

fn run_script(script_path: &Path) -> Result<()> {
    let source_code = fs::read_to_string(script_path).unwrap();
    println!("source: {}", &source_code);
    run(&source_code)?;

    Ok(())
}

fn run(source_code: &str) -> Result<()> {
    let mut scanner = Scanner::new(&source_code);
    scanner.scan_tokens()?;
    scanner.print_tokens();
    let mut parser = Parser::new(scanner.tokens);
    let exp = parser.expression();
    println!("Expression {}", exp);
    Ok(())
}

fn run_prompt() -> Result<()> {
    println!("starting kai prompt");
    loop {
        let mut input = String::new();
        print!("kai> ");
        let _ = std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;

        if input == "exit\n" {
            break;
        }
        run(&input)?
    }
    Ok(())
}
