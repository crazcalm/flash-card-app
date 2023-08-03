use std::fs::File;

use flash_cards::{Cards, Card};
use flash_cards::traits::{FlashCard, FlashCards, FlipFlashCard, Loader};
use flash_cards::enums::FlashCardState;
use flash_cards::loader::Csv;

fn main() {
    let _test_data = "\
front,back,hint,
front_1,back_1,hint_1,
front_2,back_2,hint_2,
front_3,back_3,,
";

    // TODO: create a config the sets where the default directory path for csv files.
    // Figure out path issuses so that it can be called from anywhere.
    let test_file = File::open("./src/data/test_data.csv").unwrap();

    let mut cards = Csv::load(test_file).unwrap();

    let mut card: Card = cards.draw().unwrap();
    card.set_state(FlashCardState::Hint);
    println!("{}", card);
    println!("{}", cards);
}
