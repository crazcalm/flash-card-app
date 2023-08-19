use flash_cards::enums::FlashCardState;
use flash_cards::traits::FlipFlashCard;
use flash_cards::{Card, FlashCard};

mod cli;
mod command;
mod input;
mod ui;

use command::{parse_command, Command};

fn main() {
    let mut cards = cli::setup();

    if cards.deck_size() == 0 {
        // Early exit (will replace later)
        return;
    }

    let mut card: Card = cards.draw().unwrap();
    card.set_state(FlashCardState::Hint);
    println!("{}", card);
    println!("{}", cards);

    print!(
        "{}",
        ui::flashcard(
            "front",
            card.get_front().as_str(),
            1,
            cards.deck_size(),
            true,
            false,
            false
        )
    );

    let mut command = Command::Unknown;
    while command != Command::Quit {
        // Testing out
        let stdin = std::io::stdin();
        let handle = stdin.lock();
        let user_input = input::get_command(handle).unwrap();

        command = parse_command(&user_input, true, true, true);

        println!("command: {:?}", command);
    }

    println!("You have exited");
}
