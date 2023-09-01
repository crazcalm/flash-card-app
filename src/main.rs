use std::cell::RefCell;
use std::rc::Rc;

use flash_cards::{Card, FlashCard, FlipFlashCard};

mod cli;
mod command;
mod input;
mod ui;

use command::{parse_command, Command};

fn main() {
    let mut card_manager = cli::setup();

    let card_weak_ref = card_manager.next_card();
    if card_weak_ref.is_none() {
        println!("No cards were loaded");
        return;
    }

    let total_cards = card_manager.num_of_cards_in_deck() + card_manager.num_of_cards_seen();

    let mut command = Command::Unknown;
    while command != Command::Quit {
        // Clearing the screen
        let _ = clearscreen::clear();

        let card_ref: Rc<RefCell<Card>> = card_manager.current_card().unwrap().upgrade().unwrap();
        let card = card_ref.borrow_mut();
        let has_next = card_manager.num_of_cards_in_deck() != 0;
        let has_previous = card_manager.num_of_cards_seen() > 1;
        let has_hint = card.get_hint().is_some();

        let commands = ui::get_availiable_commands(&card, &card_manager);

        println!(
            "\n{}",
            ui::flashcard_v2(
                card.get_state().to_string().as_str(),
                card.to_string().as_str(),
                card_manager.num_of_cards_seen(),
                total_cards,
                commands.as_str(),
            )
        );

        // Dropping so that I can borrow it again later in the different context.
        drop(card);

        // Testing out
        let stdin = std::io::stdin();
        let handle = stdin.lock();
        let user_input = input::get_command(handle).unwrap();

        command = parse_command(&user_input, true, true, true);
        match command {
            Command::Unknown => println!("Not a valid command"),
            Command::Next => match has_next {
                true => {
                    let _ = card_manager.next_card().unwrap();
                    card_manager.reset_current_card_state();
                }
                false => {}
            },
            Command::Previous => match has_previous {
                true => {
                    let _ = card_manager.previous_card().unwrap();
                    card_manager.reset_current_card_state();
                }
                false => {}
            },
            Command::Flip => {
                card_manager.flip_current_card();
            }
            Command::Hint => match has_hint {
                true => {
                    card_manager.try_to_flip_current_card_to_hint();
                }
                false => {}
            },
            Command::Shuffle => {
                match has_next {
                    true => {
                        // Include the currently shown card in the shuffle
                        card_manager.previous_card();
                        card_manager.shuffle();

                        // set new shown card
                        let _ = card_manager.next_card().unwrap();
                    }
                    false => {}
                }
            }
            Command::Restart => {
                card_manager.add_previous_cards_to_deck();

                // Ensuring that there is a current card
                let _ = card_manager.next_card();
            }
            Command::Quit => {}
        }
    }
}
