use std::fmt::Display;

use crate::json::{error::JsonParserError, lexer::LexerState::InTrue};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub state: LexerState,
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

impl Lexer {
    pub fn new() -> Self {
        Self {
            state: LexerState::NewToken,
        }
    }

    pub fn parse(&mut self, source: &str) -> Result<Vec<JsonToken>, JsonParserError> {
        let mut tokens: Vec<JsonToken> = Vec::new();
        let mut state = LexerState::NewToken;
        let mut iter = source.char_indices();
        let mut token: Option<(usize, char)> = iter.next();

        while token.is_some() {
            let (pos, c) = token.unwrap();
            state = match state {
                LexerState::NewToken => {
                    token = iter.next();
                    match c {
                        '{' => {
                            tokens.push(JsonToken {
                                kind: JsonTokenKind::LBrace,
                                span: Span {
                                    start: pos,
                                    end: pos + 1,
                                },
                            });
                            LexerState::NewToken
                        }
                        '}' => {
                            tokens.push(JsonToken {
                                kind: JsonTokenKind::RBrace,
                                span: Span {
                                    start: pos,
                                    end: pos + 1,
                                },
                            });
                            LexerState::NewToken
                        }
                        '[' => {
                            tokens.push(JsonToken {
                                kind: JsonTokenKind::LBracket,
                                span: Span {
                                    start: pos,
                                    end: pos + 1,
                                },
                            });
                            LexerState::NewToken
                        }
                        ']' => {
                            tokens.push(JsonToken {
                                kind: JsonTokenKind::RBracket,
                                span: Span {
                                    start: pos,
                                    end: pos + 1,
                                },
                            });
                            LexerState::NewToken
                        }
                        ':' => {
                            tokens.push(JsonToken {
                                kind: JsonTokenKind::Colon,
                                span: Span {
                                    start: pos,
                                    end: pos + 1,
                                },
                            });
                            LexerState::NewToken
                        }
                        ',' => {
                            tokens.push(JsonToken {
                                kind: JsonTokenKind::Comma,
                                span: Span {
                                    start: pos,
                                    end: pos + 1,
                                },
                            });
                            LexerState::NewToken
                        }
                        '"' => LexerState::InString(Span {
                            start: pos,
                            end: pos + 1,
                        }),
                        't' => LexerState::InTrue(Span {
                            start: pos,
                            end: pos,
                        }),
                        'f' => LexerState::InFalse(Span {
                            start: pos,
                            end: pos,
                        }),
                        'n' => LexerState::InNull(Span {
                            start: pos,
                            end: pos,
                        }),
                        '0'..='9' | '-' => LexerState::InNumber(Span {
                            start: pos,
                            end: pos,
                        }),
                        ' ' | '\n' => continue, // Skip whitespace
                        _ => LexerState::Invalid((
                            String::new(),
                            Span {
                                start: pos,
                                end: pos + 1,
                            },
                        )),
                    }
                }
                LexerState::InString(span) => {
                    let new_span = Span {
                        start: span.start,
                        end: pos + 1,
                    };
                    match c {
                        '"' => {
                            let apart = source.get(new_span.start..new_span.end).unwrap();
                            if apart.ends_with('\\') {
                                token = iter.next();
                                LexerState::InString(new_span)
                            } else {
                                tokens.push(JsonToken {
                                    kind: JsonTokenKind::String,
                                    span: new_span,
                                });
                                token = iter.next();
                                LexerState::NewToken
                            }
                        }
                        _ => {
                            token = iter.next();
                            LexerState::InString(new_span)
                        }
                    }
                }
                LexerState::InNumber(span) => {
                    let apart = source.get(span.start..span.end).unwrap();

                    match c {
                        '0'..'9' => {
                            //TODO: handle leading zero
                            token = iter.next();
                            LexerState::InNumber(Span {
                                start: span.start,
                                end: pos,
                            })
                        }
                        '-' => {
                            if !apart.ends_with('.') && !apart.ends_with('-') {
                                token = iter.next();
                                LexerState::InNumber(Span {
                                    start: span.start,
                                    end: pos,
                                })
                            } else {
                                token = iter.next();
                                LexerState::Invalid((
                                    String::from(apart),
                                    Span {
                                        start: pos,
                                        end: pos + 1,
                                    },
                                ))
                            }
                        }
                        'e' => {
                            if !apart.ends_with('-') && !apart.contains('e') {
                                token = iter.next();
                                LexerState::InNumber(Span {
                                    start: span.start,
                                    end: pos,
                                })
                            } else {
                                token = iter.next();
                                LexerState::Invalid((
                                    String::from(""),
                                    Span {
                                        start: pos,
                                        end: pos + 1,
                                    },
                                ))
                            }
                        }
                        '.' => {
                            if !apart.ends_with('e')
                                && !apart.ends_with('.')
                                && !apart.ends_with('-')
                            {
                                token = iter.next();
                                LexerState::InNumber(Span {
                                    start: span.start,
                                    end: pos,
                                })
                            } else {
                                token = iter.next();
                                LexerState::Invalid((
                                    String::from(""),
                                    Span {
                                        start: pos,
                                        end: pos + 1,
                                    },
                                ))
                            }
                        }
                        _ => {
                            tokens.push(JsonToken {
                                kind: JsonTokenKind::Number,
                                span: Span {
                                    start: span.start,
                                    end: pos,
                                },
                            });
                            LexerState::NewToken
                        }
                    }
                }
                LexerState::InTrue(span) => {
                    let new_span = Span {
                        start: span.start,
                        end: pos,
                    };
                    let t = source.get(new_span.start..new_span.end).unwrap();
                    if t == TRUE_LITERAL {
                        tokens.push(JsonToken {
                            kind: JsonTokenKind::True,
                            span: new_span,
                        });
                        LexerState::NewToken
                    } else if TRUE_LITERAL.starts_with(&t) {
                        token = iter.next();
                        InTrue(new_span)
                    } else {
                        LexerState::Invalid((String::new(), new_span))
                    }
                }
                LexerState::InFalse(span) => {
                    let new_span = Span {
                        start: span.start,
                        end: pos,
                    };
                    let t = source.get(new_span.start..new_span.end).unwrap();
                    if t == FALSE_LITERAL {
                        tokens.push(JsonToken {
                            kind: JsonTokenKind::False,
                            span: new_span,
                        });
                        LexerState::NewToken
                    } else if FALSE_LITERAL.starts_with(&t) {
                        token = iter.next();
                        LexerState::InFalse(new_span)
                    } else {
                        LexerState::Invalid((String::new(), new_span))
                    }
                }
                LexerState::InNull(span) => {
                    let new_span = Span {
                        start: span.start,
                        end: pos,
                    };
                    let t = source.get(new_span.start..new_span.end).unwrap();
                    if t == NULL_LITERAL {
                        tokens.push(JsonToken {
                            kind: JsonTokenKind::Null,
                            span: new_span,
                        });
                        LexerState::NewToken
                    } else if NULL_LITERAL.starts_with(&t) {
                        token = iter.next();
                        LexerState::InNull(new_span)
                    } else {
                        LexerState::Invalid((String::new(), new_span))
                    }
                }
                LexerState::Invalid((message, span)) => {
                    let msg = format!("{} at {:?}", message, span);
                    return Err(JsonParserError::LexicalError(msg));
                }
            };
        }

        // drain
        match state {
            LexerState::InNumber(span) => {
                let content = source.get(span.start..span.end + 1).unwrap();
                if let Ok(_) = content.parse::<f32>() {
                    tokens.push(JsonToken {
                        span: Span {
                            start: span.start,
                            end: span.end + 1,
                        },
                        kind: JsonTokenKind::Number,
                    });
                }
            }
            LexerState::InNull(span) => {
                let content = source.get(span.start..span.end + 1).unwrap();
                if content == "null" {
                    tokens.push(JsonToken {
                        span: Span {
                            start: span.start,
                            end: span.end + 1,
                        },
                        kind: JsonTokenKind::Null,
                    });
                }
            }
            LexerState::InTrue(span) => {
                let content = source.get(span.start..span.end + 1).unwrap();
                if content == "true" {
                    tokens.push(JsonToken {
                        span: Span {
                            start: span.start,
                            end: span.end + 1,
                        },
                        kind: JsonTokenKind::True,
                    });
                }
            }
            LexerState::InFalse(span) => {
                let content = source.get(span.start..span.end + 1).unwrap();
                if content == "false" {
                    tokens.push(JsonToken {
                        span: Span {
                            start: span.start,
                            end: span.end + 1,
                        },
                        kind: JsonTokenKind::False,
                    });
                }
            }
            _ => {}
        }

        Ok(tokens)
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
        let mut lexer = Lexer::new();
        lexer.parse(source)
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
