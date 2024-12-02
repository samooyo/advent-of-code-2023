use anyhow::Result;
use clap::{Parser, Subcommand};
mod days;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(value_name = "day", help = "Day to run")]
        day: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { day } => match day {
            Some(day) => run_day(day.parse::<usize>().unwrap()),
            None => {
                println!("Please provide a day to run");
                Ok(())
            }
        },
    }
}

fn run_day(day: usize) -> Result<()> {
    match day {
        1 => days::day01::run(),

        wrong => panic!("Wrong day provided : {}", wrong),
    }
}
