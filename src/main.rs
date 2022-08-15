use std::{env, fs, io::Write, path::Path};

// mod scanner;
// mod token;

use ckai::scanner::{Scanner, ScannerError};

fn main() -> Result<(), ScannerError> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("usage kai [script]!")
    } else if args.len() == 2 {
        run_script(Path::new(args[1].as_str()))?;
    } else {
        run_prompt()
    }

    Ok(())
}

fn run_script(script_path: &Path) -> Result<(), ScannerError> {
    let source_code = fs::read_to_string(script_path).unwrap();
    println!("source: {}", &source_code);

    let mut scanner = Scanner::new(&source_code);
    scanner.scan_tokens()?;
    scanner.print_tokens();
    Ok(())
}

fn run_prompt() {
    println!("starting kai prompt");
    loop {
        let mut input = String::new();
        print!("kai> ");
        let _ = std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        if input == "exit\n" {
            break;
        }
        run(&input)
    }
}

fn run(source_code: &String) {
    println!("Running with code: {}", source_code)
}
