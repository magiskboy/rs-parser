use crate::json::{error::JsonParserError, lexer::LexerState::InTrue};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub current_state: LexerState,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonToken {
    pub kind: TokenKind,
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

impl Lexer {
    pub fn new() -> Self {
        Self {
            current_state: LexerState::NewToken,
        }
    }

    pub fn parse(&mut self, source: &str) -> Result<Vec<JsonToken>, JsonParserError> {
        let true_string = String::from("true");
        let false_string = String::from("false");
        let null_string = String::from("null");

        let mut tokens: Vec<JsonToken> = Vec::new();
        let mut state = LexerState::NewToken;

        for (pos, c) in source.char_indices() {
            state = match state {
                LexerState::NewToken => match c {
                    '{' => {
                        tokens.push(JsonToken {
                            kind: TokenKind::LBrace,
                            span: Span {
                                start: pos,
                                end: pos + 1,
                            },
                        });
                        LexerState::NewToken
                    }
                    '}' => {
                        tokens.push(JsonToken {
                            kind: TokenKind::RBrace,
                            span: Span {
                                start: pos,
                                end: pos + 1,
                            },
                        });
                        LexerState::NewToken
                    }
                    '[' => {
                        tokens.push(JsonToken {
                            kind: TokenKind::LBracket,
                            span: Span {
                                start: pos,
                                end: pos + 1,
                            },
                        });
                        LexerState::NewToken
                    }
                    ']' => {
                        tokens.push(JsonToken {
                            kind: TokenKind::LBracket,
                            span: Span {
                                start: pos,
                                end: pos + 1,
                            },
                        });
                        LexerState::NewToken
                    }
                    ':' => {
                        tokens.push(JsonToken {
                            kind: TokenKind::Colon,
                            span: Span {
                                start: pos,
                                end: pos + 1,
                            },
                        });
                        LexerState::NewToken
                    }
                    ',' => {
                        tokens.push(JsonToken {
                            kind: TokenKind::Comma,
                            span: Span {
                                start: pos,
                                end: pos + 1,
                            },
                        });
                        LexerState::NewToken
                    }
                    '"' => LexerState::InString(Span {
                        start: pos + 1,
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
                    '1'..='9' | '-' => LexerState::InNumber(Span {
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
                },
                LexerState::InString(span) => match c {
                    '"' => {
                        let apart = source.get(span.start..span.end + 1).unwrap();
                        if apart.ends_with('\\') {
                            LexerState::InString(Span {
                                start: span.start,
                                end: pos,
                            })
                        } else {
                            tokens.push(JsonToken {
                                kind: TokenKind::String,
                                span: Span {
                                    start: span.start,
                                    end: pos,
                                },
                            });
                            LexerState::NewToken
                        }
                    }
                    _ => LexerState::InString(Span {
                        start: span.start,
                        end: pos,
                    }),
                },
                LexerState::InNumber(span) => {
                    let apart = source.get(span.start..span.end).unwrap();

                    match c {
                        '0'..'9' => LexerState::InNumber(Span {
                            start: span.start,
                            end: pos,
                        }),
                        '-' => {
                            if !apart.ends_with('.') && !apart.ends_with('-') {
                                LexerState::InNumber(Span {
                                    start: span.start,
                                    end: pos,
                                })
                            } else {
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
                                LexerState::InNumber(Span {
                                    start: span.start,
                                    end: pos,
                                })
                            } else {
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
                                LexerState::InNumber(Span {
                                    start: span.start,
                                    end: pos,
                                })
                            } else {
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
                                kind: TokenKind::Number,
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
                    let t = source.get(span.start..pos).unwrap();
                    if t == true_string {
                        tokens.push(JsonToken {
                            kind: TokenKind::True,
                            span: Span {
                                start: span.start,
                                end: pos + 1,
                            },
                        });
                        LexerState::NewToken
                    } else if true_string.starts_with(&t) {
                        InTrue(Span {
                            start: span.start,
                            end: pos,
                        })
                    } else {
                        LexerState::Invalid((
                            String::new(),
                            Span {
                                start: pos,
                                end: pos + 1,
                            },
                        ))
                    }
                }
                LexerState::InFalse(span) => {
                    let t = source.get(span.start..pos).unwrap();
                    if t == false_string {
                        tokens.push(JsonToken {
                            kind: TokenKind::False,
                            span: Span {
                                start: span.start,
                                end: pos + 1,
                            },
                        });
                        LexerState::NewToken
                    } else if false_string.starts_with(&t) {
                        LexerState::InFalse(Span {
                            start: span.start,
                            end: pos,
                        })
                    } else {
                        LexerState::Invalid((
                            String::new(),
                            Span {
                                start: pos,
                                end: pos + 1,
                            },
                        ))
                    }
                }
                LexerState::InNull(span) => {
                    let t = source.get(span.start..pos).unwrap();
                    if t == null_string {
                        tokens.push(JsonToken {
                            kind: TokenKind::Null,
                            span: Span {
                                start: span.start,
                                end: pos + 1,
                            },
                        });
                        LexerState::NewToken
                    } else if null_string.starts_with(&t) {
                        LexerState::InNull(Span {
                            start: span.start,
                            end: pos,
                        })
                    } else {
                        LexerState::Invalid((
                            String::new(),
                            Span {
                                start: pos,
                                end: pos + 1,
                            },
                        ))
                    }
                }
                LexerState::Invalid((message, span)) => {
                    let msg = format!("{} at {:?}", message, span);
                    return Err(JsonParserError::LexicalError(msg));
                }
            };
        }

        Ok(tokens)
    }

    pub fn print_token(token: JsonToken, source: &str) {
        let output: String = match token.kind {
            TokenKind::LBrace => format!(
                "<kind=LBrace start={}, end={} />",
                token.span.start, token.span.end
            ),
            TokenKind::RBrace => format!(
                "<kind=RBrace start={}, end={} />",
                token.span.start, token.span.end
            ),
            TokenKind::LBracket => format!(
                "<kind=LBracket start={}, end={} />",
                token.span.start, token.span.end
            ),
            TokenKind::RBracket => format!(
                "<kind=RBracket start={}, end={} />",
                token.span.start, token.span.end
            ),
            TokenKind::String => format!(
                "<kind=String start={}, end={}, content={}",
                token.span.start,
                token.span.end,
                source.get(token.span.start..token.span.end).unwrap()
            ),
            TokenKind::Number => format!(
                "<kind=Number start={}, end={}, content={}",
                token.span.start,
                token.span.end,
                source.get(token.span.start..token.span.end).unwrap()
            ),
            TokenKind::True => format!(
                "<kind=True start={}, end={} />",
                token.span.start, token.span.end
            ),
            TokenKind::False => format!(
                "<kind=False start={}, end={} />",
                token.span.start, token.span.end
            ),
            TokenKind::Null => format!(
                "<kind=Null start={}, end={} />",
                token.span.start, token.span.end
            ),
            TokenKind::Colon => format!(
                "<kind=Colon start={}, end={} />",
                token.span.start, token.span.end
            ),
            TokenKind::Comma => format!(
                "<kind=Comma start={}, end={} />",
                token.span.start, token.span.end
            ),
        };
        println!("{}", output);
    }
}
