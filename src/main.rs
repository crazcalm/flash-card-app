use std::fs::File;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use flash_cards::enums::FlashCardState;
use flash_cards::loader::Csv;
use flash_cards::traits::{FlashCard, FlashCards, FlipFlashCard, Loader};
use flash_cards::{Card, Cards};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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
enum Commands {
    /// Loader commands
    Loader {
        /// lists test values
        #[arg(long)]
        csv: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut cards: Box<dyn FlashCards<Card>>;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Loader { csv }) => {
            let test_file = File::open(csv).unwrap();

            cards = Csv::load(test_file).unwrap();
            let mut card: Card = cards.draw().unwrap();
            card.set_state(FlashCardState::Hint);
            println!("{}", card);
            println!("{}", cards);
        }
        None => {}
    }
}
