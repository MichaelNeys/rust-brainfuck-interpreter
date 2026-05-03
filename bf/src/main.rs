use clap::Parser as ClapParser;
use std::fs;
use std::io::{stdin};
use crate::executor::Executor;
use crate::parser::Parser;

pub mod instruction;
pub mod memory_tape;
pub mod executor;
pub mod parser;

#[derive(ClapParser, Debug)]
#[command(version, about = "Brainfuck Interpreter")]
struct Args {
    program_path: String,

    input_path: Option<String>,

    #[arg(short, long)]
    debug_dump: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let code = fs::read_to_string(&args.program_path)
        .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", args.program_path, e))?;

    let instruction_list = Parser::parse(&code);

    if args.debug_dump {
        let mut count = 0;
        for inst in &instruction_list.instructions {
            eprintln!("{:?}", inst);
            count += 1;
        }
        eprintln!("Total number of instructions: {}", count);
    }

    if let Some(path) = args.input_path {
        let input_file = fs::File::open(path)?;
        let mut executor = Executor::new(instruction_list, input_file);
        executor.run();
    } else {
        let mut executor = Executor::new(instruction_list, stdin().lock());
        executor.run();
    }

    Ok(())
}