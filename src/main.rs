use popper_compiler::*;
use popper_cpu::{interpret_file, interpret_string};
use clap::{Parser, Subcommand};
use rustyline::history::FileHistory;

#[derive(Parser, Debug)]
/// Popper is a programming language
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// parse a file
    Parse {
        /// The name of the file to parse
        filename: String,
    },
    /// check a file
    Check {
        /// The name of the file to check
        filename: String,
    },
    /// compile a file
    Compile {
        /// The name of the file to compile
        filename: String,
    },
    /// interpret a file
    Interpret {
        /// The name of the file to interpret
        filename: String,
    },
    /// execute a file
    Execute {
        /// The name of the file to execute
        filename: String,
    },
    /// the repl
    Repl,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Parse { filename } => {
            let ast = get_ast(&std::fs::read_to_string(filename.clone()).unwrap(), &filename.clone());
            match ast {
                Some(ast) => {
                    println!("{:#?}", ast);
                }
                None => {
                    println!("Error parsing file");
                }
            }
        },
        Commands::Check { filename } => {
            let ast = get_ast(&std::fs::read_to_string(filename.clone()).unwrap(), &filename.clone());
            match ast {
                Some(ast) => {
                    let res =
                        check_program(ast, &std::fs::read_to_string(filename.clone()).unwrap(), &filename.clone());
                    if res {
                        println!("No errors found");
                    }
                }
                None => {
                    println!("Error parsing file");
                }
            }
        },
        Commands::Compile { filename } => {
            let body = std::fs::read_to_string(filename.clone()).unwrap();
            let ir = popper_compile(body.as_str(), &filename);
            println!("{}", ir);
        },
        Commands::Execute { filename } => {
            let body = std::fs::read_to_string(filename.clone()).unwrap();
            let ir = popper_compile(body.as_str(), &filename);
            interpret_string(ir.trim());
        },
        Commands::Interpret { filename } => {
            interpret_file(filename.as_str());
        },
        Commands::Repl => {
            use rustyline::error::ReadlineError;
            use rustyline::Editor;
            let mut rl: Editor<(), FileHistory> = Editor::new().unwrap();
            if rl.load_history("history.txt").is_err() {
                println!("No previous history.");
            }
            loop {
                let readline = rl.readline(">> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str()).unwrap();
                        let ir = popper_compile(line.as_str(), "repl");
                        interpret_string(ir.as_str());
                    }
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
            rl.save_history("history.txt").unwrap();
        },
    }
}
