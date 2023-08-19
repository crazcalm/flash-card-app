use askama::Template; // bring trait in scope

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
