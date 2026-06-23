use crate::error::{ParseError, Result};
use crate::value::Skill;

pub struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Skill> {
        self.skip_whitespace();
        self.expect_delimiter()?;

        let name = self.parse_field("name")?;
        let description = self.parse_field("description")?;

        self.expect_delimiter()?;
        self.skip_whitespace();
        let content = self.read_to_end().trim().to_string();

        Ok(Skill {
            name,
            description,
            content,
        })
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn skip_newline(&mut self) {
        if let Some(ch) = self.peek_char()
            && ch == '\n'
        {
            self.pos += 1;
            if let Some('\r') = self.peek_char() {
                self.pos += 1;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn next_char(&mut self) -> Option<char> {
        if self.pos < self.input.len() {
            let ch = self.input[self.pos];
            self.pos += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn expect_delimiter(&mut self) -> Result<()> {
        self.skip_whitespace();

        if !self.peek_delimiter() {
            return Err(ParseError::InvalidFormat(format!(
                "Expected '---', found at position {}",
                self.pos
            )));
        }

        for _ in 0..3 {
            self.next_char();
        }

        self.skip_whitespace();
        if let Some('\n') = self.peek_char() {
            self.pos += 1;
        }

        Ok(())
    }

    fn peek_delimiter(&self) -> bool {
        if self.pos + 2 >= self.input.len() {
            return false;
        }
        self.input[self.pos] == '-'
            && self.input[self.pos + 1] == '-'
            && self.input[self.pos + 2] == '-'
    }

    fn parse_field(&mut self, field_name: &str) -> Result<String> {
        self.skip_whitespace();

        let expected = format!("{}:", field_name);
        let expected_chars: Vec<char> = expected.chars().collect();

        for i in 0..expected_chars.len() {
            if self.pos + i >= self.input.len() {
                return Err(ParseError::InvalidFormat(format!(
                    "Expected '{}', got end of input",
                    expected
                )));
            }
            if self.input[self.pos + i] != expected_chars[i] {
                return Err(ParseError::InvalidFormat(format!(
                    "Expected '{}', found '{}' at position {}",
                    expected,
                    self.input[self.pos..self.pos + expected_chars.len().min(10)]
                        .iter()
                        .collect::<String>(),
                    self.pos
                )));
            }
        }

        self.pos += expected_chars.len();
        self.skip_whitespace();

        let value_start = self.pos;
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            }
            self.pos += 1;
        }

        let value: String = self.input[value_start..self.pos].iter().collect();
        let value = value.trim().to_string();

        self.skip_newline();

        if value.is_empty() {
            return Err(ParseError::InvalidFormat(format!(
                "Field '{}' cannot be empty",
                field_name
            )));
        }

        Ok(value)
    }

    fn read_to_end(&mut self) -> String {
        let start = self.pos;
        self.pos = self.input.len();
        self.input[start..].iter().collect()
    }
}
