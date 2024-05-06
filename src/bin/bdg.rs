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
        filter: Vec<String>,
    },
}

fn main() {
    env_logger::init();

    match Bdg::parse().command {
        Commands::Show {
            budget,
            months,
            all,
            filter,
        } => {
            match budget
                .extension()
                .expect("File with not extension, can't detect filetype")
                .to_str()
                .expect("Invalid filename")
            {
                "toml" => Budget::from_file(budget).show(months, filter, all),
                "csv" => Budget::from_export(budget).show(months, filter, all),
                _ => panic!("Extension not supported: toml,csv"),
            };
        }
    }
}
