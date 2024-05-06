use budget::Budget;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Bdg {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Shows the budget group by month
    Show {
        /// Sets config file
        budget: PathBuf,
        /// Sets the number of months to show
        #[arg(short, long, default_value = "12")]
        months: u32,
        /// Show the detail of payments
        #[arg(short, long)]
        all: bool,
        /// Filter
        #[arg(short, long)]
        filter: Option<String>,
    },
}

fn main() {
    let bdg = Bdg::parse();
    match bdg.command {
        Commands::Show {
            budget,
            months,
            all,
            filter,
        } => {
            let budget = Budget::from_file(budget);
            budget.show(months, filter, all);
        }
    }
}
