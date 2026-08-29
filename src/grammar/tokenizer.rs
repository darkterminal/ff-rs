/// Token types for the FF CLI tool.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A regular word token
    Word(String),
    /// A file path token
    Path(String),
    /// A format token (e.g., .mp4, .avi)
    Format(String),
    /// A numeric value
    Number(f64),
    /// An unknown token type
    Unknown(String),
}

/// Tokenizer for converting plain English commands into tokens.
///
/// Uses `Vec<char>` internally for O(1) character access,
/// making total tokenization O(n) instead of O(n²).
pub struct Tokenizer {
    chars: Vec<char>,
    position: usize,
}

impl Tokenizer {
    /// Creates a new tokenizer for the given text.
    ///
    /// # Arguments
    ///
    /// * `text` - The input text to tokenize
    ///
    /// # Examples
    ///
    /// ```
    /// use ffrs::Tokenizer;
    /// let mut tokenizer = Tokenizer::new("convert video.mp4 to video.avi");
    /// let tokens = tokenizer.tokenize();
    /// ```
    pub fn new(text: &str) -> Self {
        Self {
            chars: text.chars().collect(),
            position: 0,
        }
    }

    /// Tokenizes the input text into a vector of tokens.
    ///
    /// # Returns
    ///
    /// A vector of `Token` enums representing the parsed tokens.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.position < self.chars.len() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            } else {
                self.position += 1;
            }
        }
        tokens
    }

    /// Peeks at the current character without advancing.
    #[inline]
    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    /// Peeks at a character `offset` positions ahead without advancing.
    #[inline]
    fn peek_at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.position + offset).copied()
    }

    /// Extracts a string slice from `start` to `end` (char indices).
    #[inline]
    fn slice(&self, start: usize, end: usize) -> String {
        self.chars[start..end].iter().collect()
    }

    fn next_token(&mut self) -> Option<Token> {
        // Skip whitespace
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }

        if self.position >= self.chars.len() {
            return None;
        }

        match self.peek().unwrap() {
            '.' => Some(self.tokenize_format()),
            '/' => Some(self.tokenize_path()),
            ch if ch.is_numeric() => Some(self.tokenize_number()),
            _ => Some(self.tokenize_word()),
        }
    }

    fn tokenize_word(&mut self) -> Token {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                self.position += 1;
            } else {
                break;
            }
        }
        let word = self.slice(start, self.position);
        if word.contains('.') && !word.starts_with('.') {
            Token::Path(word)
        } else {
            Token::Word(word.to_lowercase())
        }
    }

    fn tokenize_path(&mut self) -> Token {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '/' || ch == '_' || ch == '-' || ch == '~' {
                self.position += 1;
            } else if ch == '.' {
                // Dot is part of path only if followed by alphanumeric (file extension)
                if let Some(next_ch) = self.peek_at(1) {
                    if next_ch.is_alphanumeric() {
                        self.position += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let path = self.slice(start, self.position);
        Token::Path(path)
    }

    fn tokenize_format(&mut self) -> Token {
        let start = self.position;
        if self.peek() == Some('.') {
            self.position += 1;
        }
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() {
                self.position += 1;
            } else {
                break;
            }
        }
        let format = self.slice(start, self.position);
        Token::Format(format.to_lowercase())
    }

    fn tokenize_number(&mut self) -> Token {
        let start = self.position;
        let mut has_decimal = false;
        while let Some(ch) = self.peek() {
            if ch.is_numeric() {
                self.position += 1;
            } else if ch == '.' && !has_decimal {
                has_decimal = true;
                self.position += 1;
            } else {
                break;
            }
        }
        let number_str = self.slice(start, self.position);
        if let Ok(number) = number_str.parse::<f64>() {
            Token::Number(number)
        } else {
            Token::Unknown(number_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_command() {
        let mut tokenizer = Tokenizer::new("convert video.mp4 to video.avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("video.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("video.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_format() {
        let mut tokenizer = Tokenizer::new("convert video to .avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Word("video".to_string()),
                Token::Word("to".to_string()),
                Token::Format(".avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_path_with_directories() {
        let mut tokenizer = Tokenizer::new("convert /home/user/video.mp4 to output.avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("/home/user/video.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("output.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_empty_input() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_tokenize_whitespace_only() {
        let mut tokenizer = Tokenizer::new("   ");
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, vec![]);
    }
}
