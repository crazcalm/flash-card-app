use askama::Template;
use flash_cards::{
    enums::FlashCardState, traits::FlashCardsManager, Card, CardsManager, FlashCard, FlipFlashCard,
};

#[derive(Template)] // this will generate the code...
#[template(path = "flash_card.txt")] // using the template in this path, relative
struct FlashCardTemplate<'a> {
    side: &'a str,
    text: &'a str,
    seen: usize,
    total: usize,
    commands: &'a str,
}

pub fn flashcard_v2(side: &str, text: &str, seen: usize, total: usize, commands: &str) -> String {
    let flashcard_template = FlashCardTemplate {
        side,
        text,
        seen,
        total,
        commands,
    };
    flashcard_template.render().unwrap()
}

pub fn get_availiable_commands(
    card: &Card,
    card_manager: &Box<dyn FlashCardsManager<Card>>,
) -> String {
    let mut command_list = vec![];

    if card_manager.num_of_cards_in_deck() > 0 {
        command_list.push("(n)ext");
    }

    if card_manager.num_of_cards_seen() > 1 {
        command_list.push("(p)revious");
    }

    // Add flip now to appease my wanted order
    command_list.push("(f)lip");

    match card.get_hint() {
        Some(_hint) => {
            if card.get_state() != &FlashCardState::Hint {
                command_list.push("(h)int");
            }
        }
        None => {}
    }

    // Should not be able to shuffle when you are looking at the last card
    if card_manager.num_of_cards_in_deck() > 0 {
        command_list.push("(s)huffle");
    }

    // Can only restart after reaching card number 2
    if card_manager.num_of_cards_seen() > 1 {
        command_list.push("(r)estart");
    }

    // Quit
    command_list.push("(q)uit");

    command_list.join(" ")
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use flash_cards::loader::Csv;
    use flash_cards::traits::{FlashCards, FlashCardsManager, FlipFlashCard, Loader};
    use flash_cards::{Cards, CardsManager};

    use super::*;

    #[test]
    fn test_available_commands() {
        let card_data = "\
front,back,hint,
front_text,back_text,hint_text,
front_text,back_text,,
front_text,back_text,,
front_text,back_text,hint_text,
";
        let expected = HashMap::from([
            (1, "(n)ext (f)lip (h)int (s)huffle (q)uit".to_string()),
            (
                2,
                "(n)ext (p)revious (f)lip (s)huffle (r)estart (q)uit".to_string(),
            ),
            (
                3,
                "(n)ext (p)revious (f)lip (s)huffle (r)estart (q)uit".to_string(),
            ),
            (4, "(p)revious (f)lip (r)estart (q)uit".to_string()),
        ]);

        let mut cards = Cards::new();

        let csv_cards: Box<dyn FlashCards<Card>> =
            Csv::load(card_data.as_bytes()).expect("load csv error");
        cards.add_deck(csv_cards);

        let mut card_manager: Box<dyn FlashCardsManager<Card>> =
            Box::new(CardsManager::create_from_deck(cards));
        while card_manager.num_of_cards_in_deck() > 0 {
            let binding = card_manager.next_card().unwrap().upgrade().unwrap();
            let mut card = binding.borrow_mut();

            // Case 4: showing the hint side should remove the hint command from the command_list
            if card_manager.num_of_cards_seen() == 4 {
                card.set_state(FlashCardState::Hint);
            }

            let command_text = get_availiable_commands(&card, &card_manager);
            assert_eq!(
                expected.get(&card_manager.num_of_cards_seen()).unwrap(),
                command_text.as_str(),
                "Case {} failed",
                card_manager.num_of_cards_seen()
            );
        }
    }
}
