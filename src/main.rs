use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use htils::constant::{BANNER, BIN_NAME};

#[derive(Parser)]
#[command(name = BIN_NAME, author, version, about, long_about = None, before_help= BANNER)]
struct Args {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Just say hi!
    Hi,
    /// Add two digit numbers
    Sum {
        num1: f64,
        num2: f64,
    },
    /// Generate auto complete for any shell.
    Completions {
        shell: Shell,
    },
}

fn main() {
    let args = Args::parse();
    if let Some(command) = args.command {
        match command {
            Command::Hi => println!("Hi, have a good day!"),
            Command::Sum { num1, num2 } => println!("{}", num1 + num2),
            Command::Completions { shell } => {
                generate(
                    shell,
                    &mut Args::command(),
                    BIN_NAME,
                    &mut std::io::stdout(),
                );
            }
        }
    } else {
        let _ = Args::command().print_help();
    }
}
