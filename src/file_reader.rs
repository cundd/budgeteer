use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use error::Error;

pub type LineParts = Vec<String>;

pub struct ReadFileResult {
    pub lines: Vec<LineParts>,
    pub errors: Vec<Error>,
}

pub struct FileReader {}

impl FileReader {
    pub fn read(input_file: &str) -> Result<ReadFileResult, Error> {
        let f = File::open(input_file)?;
        let file = BufReader::new(&f);
        let raw_lines = file.lines();
        let mut lines = vec![];
        let mut errors = vec![];

        let mut number_of_relevant_lines = 0;
        for line in raw_lines {
            let result = FileReader::read_line(line);

            match result {
                Ok(i) => {
                    // The first match will be the header, collect only the remaining ones
                    if number_of_relevant_lines > 1 {
                        lines.push(i);
                    }
                    number_of_relevant_lines += 1;
                }
                Err(e) => errors.push(e),
            }
        }

        Ok(ReadFileResult { lines, errors })
    }

    fn read_line(line: Result<String, io::Error>) -> Result<LineParts, Error> {
        match line {
            Ok(ref l) => {
                let trimmed = l.trim().trim_matches('|').to_string();
                if trimmed.starts_with(">") || trimmed.starts_with("<!--") {
                    Err(Error::LineComment)
                } else if trimmed.starts_with("--") {
                    Err(Error::LineSeparator)
                } else if trimmed == "" {
                    Err(Error::LineEmpty)
                } else {
                    FileReader::check_line_parts(trimmed)
                }
            }
            Err(ref e) => Err(Error::from(e)),
        }
    }

    fn check_line_parts(trimmed: String) -> Result<LineParts, Error> {
        let parts = trimmed.split('|').map(|part| part.trim().to_owned()).collect::<LineParts>();

        if parts.iter().any(|part| part.starts_with("--")) {
            Err(Error::LineSeparator)
        } else {
            Ok(parts)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_line() {
        assert!(FileReader::read_line(Ok("| a | b | c |".to_string())).is_ok());
        assert_eq!(FileReader::read_line(Ok("| a | b | c |".to_string())), Ok(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string()
        ]));
    }

    #[test]
    fn test_read_line_empty() {
        assert!(FileReader::read_line(Ok("".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("".to_string())), Err(Error::LineEmpty));
    }

    #[test]
    fn test_read_line_separator() {
        assert!(FileReader::read_line(Ok("----".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("----".to_string())), Err(Error::LineSeparator));

        assert!(FileReader::read_line(Ok("--".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("--".to_string())), Err(Error::LineSeparator));

        assert!(FileReader::read_line(Ok("| ------ | --- |".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("| ------ | --- |".to_string())), Err(Error::LineSeparator));

        assert!(FileReader::read_line(Ok("|------|---|".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("|------|---|".to_string())), Err(Error::LineSeparator));

        assert!(FileReader::read_line(Ok("|--|---|".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("|--|---|".to_string())), Err(Error::LineSeparator));
    }

    #[test]
    fn test_read_line_comment() {
        assert!(FileReader::read_line(Ok("> Something".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("> Something".to_string())), Err(Error::LineComment));

        assert!(FileReader::read_line(Ok(">".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok(">".to_string())), Err(Error::LineComment));

        assert!(FileReader::read_line(Ok("> ".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("> ".to_string())), Err(Error::LineComment));

        assert!(FileReader::read_line(Ok("<!-- some text".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("<!-- some text".to_string())), Err(Error::LineComment));

        assert!(FileReader::read_line(Ok("<!--".to_string())).is_err());
        assert_eq!(FileReader::read_line(Ok("<!--".to_string())), Err(Error::LineComment));
    }
}
