use anyhow::*;
use budget::Budget;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Bdg {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    /// Command to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Shows the budget group by month
    Show {
        /// Sets config files
        budgets: Vec<PathBuf>,
        /// Sets the number of months to show
        #[arg(short, long, default_value = "12")]
        months: u32,
        /// Show the detail of payments
        #[arg(short, long)]
        all: bool,
        /// Filter
        #[arg(short, long)]
        filter: Vec<String>,
    },
}

fn main() -> Result<()> {
    env_logger::init();

    match Bdg::parse().command {
        Commands::Show {
            budgets,
            months,
            all,
            filter,
        } => Budget::load(budgets)?.show(months, filter, all),
    }
    Ok(())
}
