use std::{error::Error, fmt::Display};

use crate::json::error::JsonParserError;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub state: LexerState,
    pub index: usize,
    pub source: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonTokenKind {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    String,
    Number,
    True,
    False,
    Null,
    Colon,
    Comma,
    Whitespace,
    InvalidToken,
    Stop,
}

impl Display for JsonTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonTokenKind::LBrace => write!(f, "LBrace"),
            JsonTokenKind::RBrace => write!(f, "RBrace"),
            JsonTokenKind::LBracket => write!(f, "LBracket"),
            JsonTokenKind::RBracket => write!(f, "RBracket"),
            JsonTokenKind::String => write!(f, "String"),
            JsonTokenKind::Number => write!(f, "Number"),
            JsonTokenKind::True => write!(f, "True"),
            JsonTokenKind::False => write!(f, "False"),
            JsonTokenKind::Null => write!(f, "Null"),
            JsonTokenKind::Colon => write!(f, "Colon"),
            JsonTokenKind::Comma => write!(f, "Comma"),
            JsonTokenKind::Whitespace => write!(f, "Whitespace"),
            JsonTokenKind::InvalidToken => write!(f, "InvalidToken"),
            JsonTokenKind::Stop => write!(f, "Stop"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonToken {
    pub kind: JsonTokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerState {
    NewToken,
    InString(Span),
    InNumber(Span),
    InTrue(Span),
    InFalse(Span),
    InNull(Span),
    Invalid((String, Span)),
}

const TRUE_LITERAL: &str = "true";
const FALSE_LITERAL: &str = "false";
const NULL_LITERAL: &str = "null";
const ESCAPE_TOKENS: [&str; 8] = ["\\\"", "\\\\", "\\/", "\\b", "\\f", "\\n", "\\r", "\\t"];
const SEPARATORS_LITERALS: [char; 11] = ['{', '}', ':', ',', ']', '[', ' ', ' ', '\n', '\t', '\r'];

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            state: LexerState::NewToken,
            source,
            index: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<JsonToken, JsonParserError> {
        if self.index >= self.source.len() {
            return Ok(JsonToken {
                kind: JsonTokenKind::Stop,
                span: Span {
                    start: self.source.len(),
                    end: self.source.len() + 1,
                },
            });
        }

        let c = self.source.chars().nth(self.index).unwrap();
        let pos = self.index;
        let (token, next_index) = match c {
            '{' => (
                JsonToken {
                    kind: JsonTokenKind::LBrace,
                    span: Span {
                        start: pos,
                        end: pos + 1,
                    },
                },
                pos + 1,
            ),
            '}' => (
                JsonToken {
                    kind: JsonTokenKind::RBrace,
                    span: Span {
                        start: pos,
                        end: pos + 1,
                    },
                },
                pos + 1,
            ),
            '[' => (
                JsonToken {
                    kind: JsonTokenKind::LBracket,
                    span: Span {
                        start: pos,
                        end: pos + 1,
                    },
                },
                pos + 1,
            ),
            ']' => (
                JsonToken {
                    kind: JsonTokenKind::RBracket,
                    span: Span {
                        start: pos,
                        end: pos + 1,
                    },
                },
                pos + 1,
            ),
            ':' => (
                JsonToken {
                    kind: JsonTokenKind::Colon,
                    span: Span {
                        start: pos,
                        end: pos + 1,
                    },
                },
                pos + 1,
            ),
            ',' => (
                JsonToken {
                    kind: JsonTokenKind::Comma,
                    span: Span {
                        start: pos,
                        end: pos + 1,
                    },
                },
                pos + 1,
            ),
            ' ' | '\n' | '\r' | '\t' => (
                JsonToken {
                    kind: JsonTokenKind::Whitespace,
                    span: Span {
                        start: pos,
                        end: pos + 1,
                    },
                },
                pos + 1,
            ),
            '"' => self.parse_string()?,
            't' => self.parse_true()?,
            'f' => self.parse_false()?,
            'n' => self.parse_null()?,
            '0'..='9' | '-' => self.parse_number()?,
            _ => (
                JsonToken {
                    kind: JsonTokenKind::InvalidToken,
                    span: Span {
                        start: pos,
                        end: pos + 1,
                    },
                },
                0,
            ),
        };

        if token.kind == JsonTokenKind::InvalidToken {
            return Err(JsonParserError::LexicalError(String::from(
                "invalid character",
            )));
        }

        self.index = next_index;
        Ok(token)
    }

    fn parse_true(&self) -> Result<(JsonToken, usize), JsonParserError> {
        let span = self.get_candicate_span(self.index);

        if self.get_str(&span) == Some(TRUE_LITERAL) {
            return Ok((
                JsonToken {
                    kind: JsonTokenKind::True,
                    span,
                },
                span.end,
            ));
        }
        Err(JsonParserError::LexicalError(String::from(
            "Invalid character",
        )))
    }

    fn parse_false(&self) -> Result<(JsonToken, usize), JsonParserError> {
        let span = self.get_candicate_span(self.index);

        if self.get_str(&span) == Some(FALSE_LITERAL) {
            return Ok((
                JsonToken {
                    kind: JsonTokenKind::False,
                    span,
                },
                span.end,
            ));
        }
        Err(JsonParserError::LexicalError(String::from(
            "Invalid character",
        )))
    }

    fn parse_null(&self) -> Result<(JsonToken, usize), JsonParserError> {
        let span = self.get_candicate_span(self.index);

        if self.get_str(&span) == Some(NULL_LITERAL) {
            return Ok((
                JsonToken {
                    kind: JsonTokenKind::Null,
                    span,
                },
                span.end,
            ));
        }
        Err(JsonParserError::LexicalError(String::from(
            "Invalid character",
        )))
    }

    fn parse_number(&self) -> Result<(JsonToken, usize), JsonParserError> {
        let span = self.get_candicate_span(self.index);

        match self.get_str(&span) {
            Some(value) => {
                if Self::validate_json_number(value) {
                    return Ok((
                        JsonToken {
                            kind: JsonTokenKind::Number,
                            span,
                        },
                        span.end,
                    ));
                }

                return Err(JsonParserError::LexicalError(String::from(
                    "invalid number",
                )));
            }

            None => {
                return Err(JsonParserError::LexicalError(String::from(
                    "Invalid number",
                )));
            }
        }
    }

    fn parse_string(&mut self) -> Result<(JsonToken, usize), JsonParserError> {
        let mut n_backslash = 0;
        for i in (self.index + 1)..self.source.len() {
            let c = self.source.chars().nth(i).unwrap();
            if c == '\\' {
                n_backslash += 1;
            }
            if c == '"'
                && (self.source.chars().nth(i - 1).unwrap() != '\\'
                    || (self.source.chars().nth(i - 1).unwrap() == '\\' && n_backslash % 2 == 0))
            {
                return Ok((
                    JsonToken {
                        kind: JsonTokenKind::String,
                        span: Span {
                            start: self.index,
                            end: i + 1,
                        },
                    },
                    i + 1,
                ));
            }
        }

        Err(JsonParserError::LexicalError(String::from(
            "invalid string",
        )))
    }

    pub fn parse(&mut self) -> Result<Vec<JsonToken>, JsonParserError> {
        let mut tokens: Vec<JsonToken> = vec![];

        loop {
            let token = self.next_token()?;
            if token.kind == JsonTokenKind::Whitespace {
                continue;
            }
            if token.kind == JsonTokenKind::Stop {
                break;
            }
            Lexer::print_token_with_value(token.clone(), self.source);
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn validate_json_number(value: &str) -> bool {
        return true;
    }

    fn get_str(&self, span: &Span) -> Option<&str> {
        self.source.get(span.start..span.end)
    }

    fn get_candicate_span(&self, start: usize) -> Span {
        for end in start..self.source.len() {
            if SEPARATORS_LITERALS.contains(&self.source.chars().nth(end).unwrap()) {
                return Span { start, end };
            }
        }
        Span {
            start,
            end: self.source.len(),
        }
    }

    pub fn print_token_with_value(token: JsonToken, source: &str) {
        let output: String = match token.kind {
            JsonTokenKind::LBrace => format!(
                "<kind={} start={}, end={} />",
                token.kind, token.span.start, token.span.end
            ),
            JsonTokenKind::RBrace => format!(
                "<kind={} start={}, end={} />",
                token.kind, token.span.start, token.span.end
            ),
            JsonTokenKind::LBracket => format!(
                "<kind={}, start={}, end={} />",
                token.kind, token.span.start, token.span.end
            ),
            JsonTokenKind::RBracket => format!(
                "<kind={}, start={}, end={} />",
                token.kind, token.span.start, token.span.end
            ),
            JsonTokenKind::String => format!(
                "<kind={}, start={}, end={}, content={}",
                token.kind,
                token.span.start,
                token.span.end,
                source.get(token.span.start..token.span.end).unwrap()
            ),
            JsonTokenKind::Number => format!(
                "<kind={}, start={}, end={}, content={}",
                token.kind,
                token.span.start,
                token.span.end,
                source.get(token.span.start..token.span.end).unwrap()
            ),
            JsonTokenKind::True => format!(
                "<kind={}, start={}, end={} />",
                token.kind, token.span.start, token.span.end
            ),
            JsonTokenKind::False => format!(
                "<kind={}, start={}, end={} />",
                token.kind, token.span.start, token.span.end
            ),
            JsonTokenKind::Null => format!(
                "<kind={}, start={}, end={} />",
                token.kind, token.span.start, token.span.end
            ),
            JsonTokenKind::Colon => format!(
                "<kind={}, start={}, end={} />",
                token.kind, token.span.start, token.span.end
            ),
            JsonTokenKind::Comma => format!(
                "<kind={}, start={}, end={} />",
                token.kind, token.span.start, token.span.end
            ),
            JsonTokenKind::Stop => format!("<kind = {} />", token.kind,),
            JsonTokenKind::InvalidToken => format!("<kind = {} />", token.kind,),
            JsonTokenKind::Whitespace => format!(
                "<kind {}, start={}, end={} />",
                token.kind, token.span.start, token.span.end,
            ),
        };
        println!("{}", output);
    }
}

#[cfg(test)]
mod test {
    use crate::json::{
        error::JsonParserError,
        lexer::{JsonToken, JsonTokenKind, Lexer, Span},
    };

    fn run(source: &str) -> Result<Vec<JsonToken>, JsonParserError> {
        let mut lexer = Lexer::new(source);
        lexer.parse()
    }

    fn tok(kind: JsonTokenKind, start: usize, end: usize) -> JsonToken {
        JsonToken {
            kind,
            span: Span { start, end },
        }
    }

    fn kinds(tokens: &[JsonToken]) -> Vec<JsonTokenKind> {
        tokens.iter().map(|t| t.kind.clone()).collect()
    }

    #[test]
    fn structural_characters() {
        let input: [&str; 6] = ["{", "}", "[", "]", ":", ","];
        let expected: [JsonTokenKind; 6] = [
            JsonTokenKind::LBrace,
            JsonTokenKind::RBrace,
            JsonTokenKind::LBracket,
            JsonTokenKind::RBracket,
            JsonTokenKind::Colon,
            JsonTokenKind::Comma,
        ];
        for (s, kind) in input.iter().zip(expected.iter()) {
            assert_eq!(
                run(s),
                Ok(vec![tok(kind.clone(), 0, s.len())]),
                "structural {:?}",
                s
            );
        }
    }

    #[test]
    fn whitespace_is_insignificant() {
        let input: [&str; 5] = ["", " ", "\t", "\n", "\r"];
        for s in input {
            assert_eq!(run(s), Ok(vec![]), "whitespace {:?}", s);
        }

        let padded: [&str; 4] = ["  true  ", "\tfalse\t", "\nnull\n", "\r1\r"];
        let expected_kinds: [JsonTokenKind; 4] = [
            JsonTokenKind::True,
            JsonTokenKind::False,
            JsonTokenKind::Null,
            JsonTokenKind::Number,
        ];
        for (s, kind) in padded.iter().zip(expected_kinds.iter()) {
            let tokens = run(s).expect("should lex");
            assert_eq!(kinds(&tokens), vec![kind.clone()], "padded {:?}", s);
        }
    }

    #[test]
    fn literals() {
        let input: [&str; 3] = ["true", "false", "null"];
        let expected: [JsonToken; 3] = [
            tok(JsonTokenKind::True, 0, 4),
            tok(JsonTokenKind::False, 0, 5),
            tok(JsonTokenKind::Null, 0, 4),
        ];
        for (s, expected) in input.iter().zip(expected.iter()) {
            assert_eq!(&run(s), &Ok(vec![expected.clone()]));
        }
    }

    #[test]
    fn incomplete_or_invalid_literals_are_errors() {
        let input: [&str; 9] = [
            "tru", "tr", "t", "fals", "fal", "nul", "nu", "TRUE", "False",
        ];
        for s in input {
            assert!(run(s).is_err(), "expected lexical error for {:?}", s);
        }
    }

    #[test]
    fn strings_valid() {
        let cases: [(&str, usize, usize); 6] = [
            ("\"\"", 0, 2),
            ("\"hello\"", 0, 7),
            ("\" \"", 0, 3),
            ("\"\\\"\"", 0, 4),
            ("\"\\\\\"", 0, 4),
            ("\"\\n\\t\\r\\b\\f\\/\"", 0, 14),
        ];
        for (s, start, end) in cases {
            assert_eq!(
                run(s),
                Ok(vec![tok(JsonTokenKind::String, start, end)]),
                "string {:?}",
                s
            );
        }
    }

    #[test]
    fn strings_unicode_escape() {
        let input = "\"\\u0041\"";
        assert_eq!(
            run(input),
            Ok(vec![tok(JsonTokenKind::String, 0, input.len())])
        );
        let input = "\"\\uD83D\\uDE00\"";
        assert_eq!(
            run(input),
            Ok(vec![tok(JsonTokenKind::String, 0, input.len())])
        );
    }

    #[test]
    fn strings_invalid_are_errors() {
        let input: [&str; 8] = [
            "\"",
            "\"abc",
            "\"\\x\"",
            "\"\\u\"",
            "\"\\u12\"",
            "\"\\uZZZZ\"",
            "\"\n\"",
            "\"\t\"",
        ];
        for s in input {
            assert!(run(s).is_err(), "expected lexical error for {:?}", s);
        }
    }

    #[test]
    fn numbers_valid() {
        let input: [&str; 14] = [
            "0", "1", "10", "123", "-0", "-1", "-10", "0.1", "3.14", "10.0", "1e2", "1E2", "1e+2",
            "1e-2",
        ];
        for s in input {
            assert_eq!(
                run(s),
                Ok(vec![tok(JsonTokenKind::Number, 0, s.len())]),
                "number {:?}",
                s
            );
        }
    }

    #[test]
    fn numbers_with_fraction_and_exponent() {
        let input: [&str; 4] = ["1.2e3", "1.2E+3", "-1.2e-3", "0.0e0"];
        for s in input {
            assert_eq!(
                run(s),
                Ok(vec![tok(JsonTokenKind::Number, 0, s.len())]),
                "number {:?}",
                s
            );
        }
    }

    #[test]
    fn numbers_invalid_are_errors() {
        let input: [&str; 14] = [
            "+", "+1", "01", "-01", "1.", ".1", "-.1", "1e", "1e+", "1e-", "--1", "1.2.3", "1ee2",
            "0x1",
        ];
        for s in input {
            assert!(run(s).is_err(), "expected lexical error for {:?}", s);
        }
    }

    #[test]
    fn number_then_structural() {
        let cases: [(&str, &[JsonTokenKind]); 3] = [
            ("10,", &[JsonTokenKind::Number, JsonTokenKind::Comma]),
            ("10]", &[JsonTokenKind::Number, JsonTokenKind::RBracket]),
            ("10}", &[JsonTokenKind::Number, JsonTokenKind::RBrace]),
        ];
        for (s, expected) in cases {
            let tokens = run(s).expect("should lex");
            assert_eq!(kinds(&tokens), expected, "source {:?}", s);
            assert_eq!(tokens[0].span, Span { start: 0, end: 2 });
        }
    }

    #[test]
    fn array_tokens() {
        let input = "[1,2,true,false,null,\"x\"]";
        let tokens = run(input).expect("should lex");
        assert_eq!(
            kinds(&tokens),
            vec![
                JsonTokenKind::LBracket,
                JsonTokenKind::Number,
                JsonTokenKind::Comma,
                JsonTokenKind::Number,
                JsonTokenKind::Comma,
                JsonTokenKind::True,
                JsonTokenKind::Comma,
                JsonTokenKind::False,
                JsonTokenKind::Comma,
                JsonTokenKind::Null,
                JsonTokenKind::Comma,
                JsonTokenKind::String,
                JsonTokenKind::RBracket,
            ]
        );
    }

    #[test]
    fn object_tokens() {
        let input = "{\"a\":1,\"b\":true}";
        let tokens = run(input).expect("should lex");
        assert_eq!(
            kinds(&tokens),
            vec![
                JsonTokenKind::LBrace,
                JsonTokenKind::String,
                JsonTokenKind::Colon,
                JsonTokenKind::Number,
                JsonTokenKind::Comma,
                JsonTokenKind::String,
                JsonTokenKind::Colon,
                JsonTokenKind::True,
                JsonTokenKind::RBrace,
            ]
        );
    }

    #[test]
    fn nested_structure_tokens() {
        let input = "{\"arr\":[1,{\"k\":null}]}";
        let tokens = run(input).expect("should lex");
        assert_eq!(
            kinds(&tokens),
            vec![
                JsonTokenKind::LBrace,
                JsonTokenKind::String,
                JsonTokenKind::Colon,
                JsonTokenKind::LBracket,
                JsonTokenKind::Number,
                JsonTokenKind::Comma,
                JsonTokenKind::LBrace,
                JsonTokenKind::String,
                JsonTokenKind::Colon,
                JsonTokenKind::Null,
                JsonTokenKind::RBrace,
                JsonTokenKind::RBracket,
                JsonTokenKind::RBrace,
            ]
        );
    }

    #[test]
    fn unknown_characters_are_errors() {
        let input: [&str; 6] = ["@", "#", "'abc'", "NaN", "Infinity", "-Infinity"];
        for s in input {
            assert!(run(s).is_err(), "expected lexical error for {:?}", s);
        }
    }

    #[test]
    fn literals_must_be_complete_tokens() {
        let input: [&str; 3] = ["truex", "falsey", "nullable"];
        for s in input {
            assert!(run(s).is_err(), "expected lexical error for {:?}", s);
        }
    }
}
