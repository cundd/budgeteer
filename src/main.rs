use crate::currency::Currency;
use crate::printer::Printer;
use crate::transaction::transaction_type::TransactionType;
use crate::verbosity::Verbosity;
use clap::{arg, command, Parser, Subcommand};
use std::path::PathBuf;

mod calculator;
mod commands;
mod currency;
mod duplicate_check;
mod error;
mod file;
mod filter;
mod import;
mod month;
mod persistence;
mod printer;
mod transaction;
mod verbosity;
mod wizard;

/// Manage information about paid transactions
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show information about paid transactions
    #[command(alias("a"))]
    Analyze {
        /// Budget file to use
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Show entries from this date
        #[arg(short, long)]
        from: Option<String>,

        /// Show entries up to and including this date
        #[arg(short('x'), long)]
        to: Option<String>,

        /// Filter by type
        #[arg(short, long)]
        r#type: Option<TransactionType>,

        /// Search-term to find in notes
        #[arg(short, long)]
        search: Option<String>,

        /// Level of verbosity
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbosity: u8,
    },

    /// Interactive wizard to create new rows
    #[command(alias("w"))]
    Wizard {
        /// Budget file to use
        #[arg(value_name = "FILE")]
        output: PathBuf,

        /// Level of verbosity
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbosity: u8,
    },

    /// Import data from Markdown files into the database
    Import {
        /// Markdown or JSON file to import
        #[arg(value_name = "IMPORT-FILE")]
        input: PathBuf,

        /// Budget file to use
        #[arg(value_name = "DATABASE")]
        output: PathBuf,

        /// Require no user input during import
        #[arg(long)]
        no_interaction: bool,

        /// Level of verbosity
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbosity: u8,
    },

    /// Display the available types
    ShowTypes {},
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut printer = Printer::new();
    let base_currency = Currency::base();

    match cli.command {
        Some(Commands::Analyze {
            input,
            from,
            to,
            search,
            r#type,
            verbosity,
        }) => {
            commands::analyze::analyze(
                &mut printer,
                base_currency,
                &input,
                from,
                to,
                search,
                r#type,
                Verbosity::from_int(verbosity),
            )
            .await?
        }

        Some(Commands::Import {
            input,
            output,
            no_interaction,
            verbosity,
        }) => {
            commands::import::import(
                &mut printer,
                base_currency,
                &input,
                &output,
                no_interaction,
                Verbosity::from_int(verbosity),
            )
            .await?
        }

        Some(Commands::Wizard {
            output,
            verbosity: _,
        }) => commands::wizard::wizard(&mut printer, base_currency, &output).await?,

        Some(Commands::ShowTypes {}) => commands::show_types::show_types(&mut printer),
        None => {}
    }

    Ok(())
}
