
use std::fs::File;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use flash_cards::loader::Csv;
use flash_cards::traits::{FlashCard, FlashCards, Loader};
use flash_cards::Cards;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Loader commands
    Loader {
        /// lists test values
        #[arg(long)]
        csv: PathBuf,
    },
}

pub fn setup<T>() -> Box<dyn FlashCards<T>> where T: for <'de> FlashCard <'de> + 'static{
    let cli_app = Cli::parse();
    let mut cards = Cards::new();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli_app.command {
        Some(Commands::Loader { csv }) => {
            let test_file = File::open(csv).unwrap();

            let csv_cards = Csv::load(test_file).unwrap();
            cards.add_deck(csv_cards);
        }
        None => {}
    }

    Box::new(cards)
}
