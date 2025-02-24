use ariadne::{Color, Label, Report, ReportKind, Source};
use clap::{self, Parser};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

mod error;
mod interpreter;
mod lexer;
mod node;
mod parser;
mod scope;
mod token;
mod value;

pub use error::*;
pub use interpreter::*;
pub use lexer::*;
pub use node::*;
pub use scope::*;
pub use token::*;
pub use value::*;

#[derive(clap::Parser)]
struct Arguments {
    /// The file to run
    file: Option<String>,
    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Arguments::parse();

    match args.file {
        Some(file) => {
            let path = Path::new(&file);
            let input = fs::read_to_string(path).expect("Could not read file");
            run(input, args.verbose, &mut Interpreter::default());
        }
        None => {
            let stdin = io::stdin();
            let mut stdout = io::stdout();

            let mut interpreter = Interpreter::default();

            loop {
                write!(&stdout, "> ").expect("Could not show prompt");

                stdout.flush().expect("Could not flush stdout");

                let mut input = String::new();

                if let Err(e) = stdin.read_line(&mut input) {
                    writeln!(&stdout, "Error: {e}").expect("Could not read from stdin");
                    return;
                }

                run(input, args.verbose, &mut interpreter);
            }
        }
    }
}

fn run(input: String, verbose: bool, interpreter: &mut Interpreter) {
    let mut lexer = Lexer::new(input.clone());
    match lexer.lex() {
        Ok(tokens) => {
            if verbose {
                println!(
                    "tokens: {}",
                    tokens
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }

            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    if verbose {
                        println!("AST: {}", ast);
                    }

                    let value = interpreter.run(ast);
                    match value {
                        Ok(value) => {
                            println!("{}", value);
                        }
                        Err(e) => {
                            print_error(e, &input);
                        }
                    }
                }
                Err(e) => {
                    print_error(e, &input);
                }
            }
        }
        Err(e) => {
            print_error(e, &input);
        }
    }
}

fn print_error(error: AmiError, input: &str) {
    Report::build(ReportKind::Error, error.range.clone())
        .with_message(&error.msg)
        .with_label(
            Label::new(error.range)
                .with_color(Color::Red)
                .with_message(&error.reason),
        )
        .finish()
        .eprint(Source::from(input))
        .unwrap();
}
