use flash_cards::enums::FlashCardState;
use flash_cards::traits::FlipFlashCard;
use flash_cards::Card;

mod cli;
mod ui;

fn main() {
    let mut cards = cli::setup();
    println!("{}", ui::helloworld("Marcus"));

    if cards.deck_size() == 0 {
        // Early exit (will replace later)
        return;
    }

    let mut card: Card = cards.draw().unwrap();
    card.set_state(FlashCardState::Hint);
    println!("{}", card);
    println!("{}", cards);
}
