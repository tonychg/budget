use budget::Budget;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Bdg {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Show {
        /// Sets config file
        budget: PathBuf,
        /// Sets the number of months to show
        #[arg(short, long, default_value = "12")]
        months: u32,
    },
}

fn main() {
    let bdg = Bdg::parse();
    if let Some(name) = bdg.name {
        println!("Name: {}", name);
    }
    match bdg.command {
        Some(Commands::Show { budget, months }) => {
            let budget = Budget::from_file(budget);
            let mut total = 0.0;
            budget.group_by_date(months).iter().for_each(|(date, group)| {
                println!("{} total={} month={}", date, total, group.sum());
                total += group.sum();
            });
        }
        None => println!("No command provided"),
    }
}
