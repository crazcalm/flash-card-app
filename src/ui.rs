use askama::Template;
use flash_cards::{Card, CardsManager, traits::FlashCardsManager, FlashCard, FlipFlashCard, enums::FlashCardState};

#[derive(Template)] // this will generate the code...
#[template(path = "flash_card.txt")] // using the template in this path, relative
struct FlashCardTemplate<'a> {
    side: &'a str,
    text: &'a str,
    seen: usize,
    total: usize,
    has_next: bool,
    has_previous: bool,
    has_hint: bool,
}

pub fn flashcard(
    side: &str,
    text: &str,
    seen: usize,
    total: usize,
    has_next: bool,
    has_previous: bool,
    has_hint: bool,
) -> String {
    let flashcard_template = FlashCardTemplate {
        side,
        text,
        seen,
        total,
        has_next,
        has_previous,
        has_hint,
    };
    flashcard_template.render().unwrap()
}

#[derive(Template)] // this will generate the code...
#[template(path = "flash_card_v2.txt")] // using the template in this path, relative
struct FlashCardTemplateV2<'a> {
    side: &'a str,
    text: &'a str,
    seen: usize,
    total: usize,
    commands: &'a str,
}

pub fn flashcard_v2(
    side: &str,
    text: &str,
    seen: usize,
    total: usize,
    commands: &str,
) -> String {
    let flashcard_template = FlashCardTemplateV2 {
        side,
        text,
        seen,
        total,
        commands,
    };
    flashcard_template.render().unwrap()
}


pub fn get_availiable_commands(card: &Card, card_manager: &Box<dyn FlashCardsManager<Card>>) -> String {
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
        },
        None => {},
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
    use super::*;

    #[test]
    fn test_flashcard_data() {
        let result = flashcard("side", "text", 1, 10, true, true, true);

        let expected = vec![
            "Card side:",
            "text",
            "Count 1/10",
            "(n)ext (p)revious (f)lip (h)int (s)huffle (r)estart (q)uit",
        ];

        for item in expected {
            assert!(result.contains(item), "{} Not Found in {}", item, result);
        }
    }

    #[test]
    fn test_flashcard_variations() {
        let cases = vec![
            (true, true, true, vec!["(n)ext", "(p)revious", "(h)int"]),
            (false, true, true, vec!["(p)revious", "(h)int"]),
            (true, true, false, vec!["(n)ext", "(p)revious"]),
            (true, false, true, vec!["(n)ext", "(h)int"]),
            (true, false, false, vec!["(n)ext"]),
            (false, true, false, vec!["(p)revious"]),
        ];

        for (has_next, has_previous, has_hint, expected) in cases {
            let result = flashcard("side", "text", 5, 10, has_next, has_previous, has_hint);

            for item in expected {
                assert!(result.contains(item));
            }
        }
    }
}
