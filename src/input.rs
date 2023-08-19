use std::io::BufRead;

pub fn get_command(mut reader: impl BufRead) -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

#[cfg(test)]
mod test {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_get_command() {
        let data = "  testing  ".to_string();

        let reader = BufReader::new(data.as_bytes());

        let result = get_command(reader).unwrap();

        assert_eq!(result, data.trim().to_string());
    }
}
