#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    Unknown,
    Next,
    Previous,
    Flip,
    Hint,
    Shuffle,
    Restart,
    Quit,
}

pub fn parse_command(input: &str, has_next: bool, has_previous: bool, has_hint: bool) -> Command {
    match input.to_lowercase().as_str() {
        "n" | "next" => match has_next {
            true => Command::Next,
            false => Command::Unknown,
        },
        "p" | "previous" => match has_previous {
            true => Command::Previous,
            false => Command::Unknown,
        },
        "f" | "flip" => Command::Flip,
        "h" | "hint" => match has_hint {
            true => Command::Hint,
            false => Command::Unknown,
        },
        "s" | "shuffle" => Command::Shuffle,
        "r" | "restart" => Command::Restart,
        "q" | "quit" => Command::Quit,
        _ => Command::Unknown,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_command_has_x_is_false() {
        let cases = vec!["n", "p", "h"];

        for input in cases {
            assert_eq!(parse_command(input, false, false, false), Command::Unknown);
        }
    }

    #[test]
    fn test_parse_command() {
        let cases = vec![
            ("n", Command::Next),
            ("N", Command::Next),
            ("next", Command::Next),
            ("NeXt", Command::Next),
            ("p", Command::Previous),
            ("P", Command::Previous),
            ("previous", Command::Previous),
            ("PreVious", Command::Previous),
            ("PreVious", Command::Previous),
            ("f", Command::Flip),
            ("F", Command::Flip),
            ("flip", Command::Flip),
            ("FliP", Command::Flip),
            ("h", Command::Hint),
            ("H", Command::Hint),
            ("hint", Command::Hint),
            ("HinT", Command::Hint),
            ("s", Command::Shuffle),
            ("S", Command::Shuffle),
            ("shuffle", Command::Shuffle),
            ("ShufflE", Command::Shuffle),
            ("r", Command::Restart),
            ("R", Command::Restart),
            ("restart", Command::Restart),
            ("ReStaRt", Command::Restart),
            ("q", Command::Quit),
            ("Q", Command::Quit),
            ("quit", Command::Quit),
            ("QuiT", Command::Quit),
            ("anything", Command::Unknown),
        ];

        for (input, expected) in cases {
            assert_eq!(parse_command(input, true, true, true), expected);
        }
    }
}
