use std::collections::HashMap;

use crate::json::{
    core::JsonValue,
    error::JsonParserError,
    lexer::{JsonToken, JsonTokenKind},
};

#[derive(Debug, Clone)]
pub struct JsonParser<'a> {
    pub current_token_idx: usize,
    pub source: &'a str,
    pub tokens: &'a [JsonToken],
}

impl<'a> JsonParser<'a> {
    pub fn parse(&mut self) -> Result<JsonValue, JsonParserError> {
        if self.tokens.is_empty() {
            return Err(JsonParserError::ParserError(String::from("json is empty")));
        }

        self.parse_value()
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonParserError> {
        self.parse_token(JsonTokenKind::LBrace)?;
        self.next_token()?;
        let members = self.parse_members()?;
        self.next_token()?;
        self.parse_token(JsonTokenKind::RBrace)?;
        Ok(JsonValue::Object(HashMap::<String, JsonValue>::from_iter(
            members.into_iter(),
        )))
    }

    fn parse_members(&mut self) -> Result<Vec<(String, JsonValue)>, JsonParserError> {
        let mut members: Vec<(String, JsonValue)> = vec![];
        loop {
            if let Ok(pair) = self.parse_pair() {
                members.push(pair);
                self.next_token()?;
                if let Ok(_) = self.parse_token(JsonTokenKind::Comma) {
                    self.next_token()?;
                    continue;
                } else {
                    break;
                }
            } else {
                // if parse_pair is fail, token was back by them so we don't need back at here
                // self.back_token()?;
                break;
            }
        }
        Ok(members)
    }

    fn parse_pair(&mut self) -> Result<(String, JsonValue), JsonParserError> {
        let key_token = self.parse_token(JsonTokenKind::String)?;
        let key = self.get_string_content(&key_token)?;
        self.next_token()?;
        self.parse_token(JsonTokenKind::Colon)?;
        self.next_token()?;
        let value = self.parse_value()?;
        Ok((key.to_string(), value))
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonParserError> {
        self.parse_token(JsonTokenKind::LBracket)?;
        self.next_token()?;
        let elements = self.parse_elements()?;
        self.next_token()?;
        self.parse_token(JsonTokenKind::RBracket)?;
        Ok(JsonValue::Array(elements))
    }

    fn parse_elements(&mut self) -> Result<Vec<JsonValue>, JsonParserError> {
        let mut items: Vec<JsonValue> = vec![];
        loop {
            if let Ok(item) = self.parse_value() {
                items.push(item);
                self.next_token()?;
                if let Ok(_) = self.parse_token(JsonTokenKind::Comma) {
                    self.next_token()?;
                    continue;
                } else {
                    break;
                }
            } else {
                self.back_token()?;
                break;
            }
        }
        Ok(items)
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonParserError> {
        let first = self.get_token();

        match first.kind {
            JsonTokenKind::Null => Ok(JsonValue::Null),
            JsonTokenKind::True => Ok(JsonValue::True),
            JsonTokenKind::False => Ok(JsonValue::False),
            JsonTokenKind::String => Ok(JsonValue::String(self.get_string_content(first)?)),
            JsonTokenKind::Number => Ok(JsonValue::Number(self.get_number_content(first)?)),
            JsonTokenKind::LBracket => self.parse_array(),
            JsonTokenKind::LBrace => self.parse_object(),
            _ => Err(JsonParserError::ParserError(String::from(
                "Unexpected token, position is 0",
            ))),
        }
    }

    fn parse_token(&mut self, kind: JsonTokenKind) -> Result<JsonToken, JsonParserError> {
        let token = self.get_token();
        if token.kind != kind {
            let mut msg = String::new();
            msg.push_str("Unexpected token, expect ");
            msg.push_str(kind.to_string().as_str());
            msg.push_str(" but ");
            msg.push_str(token.kind.to_string().as_str());
            self.back_token()?;

            return Err(JsonParserError::ParserError(msg));
        }

        Ok(token.clone())
    }

    pub fn new(tokens: &'a [JsonToken], source: &'a str) -> Self {
        Self {
            current_token_idx: 0,
            source,
            tokens,
        }
    }

    fn next_token(&mut self) -> Result<usize, JsonParserError> {
        if self.current_token_idx == self.tokens.len() - 1 {
            return Err(JsonParserError::ParserError(String::from(
                "[JsonParser.next_token] Index is out of range",
            )));
        }

        self.current_token_idx = self.current_token_idx + 1;
        Ok(self.current_token_idx)
    }

    fn back_token(&mut self) -> Result<usize, JsonParserError> {
        if self.current_token_idx == 0 {
            return Err(JsonParserError::ParserError(String::from(
                "[JsonParser.back_token] Index is out of range",
            )));
        }
        self.current_token_idx = self.current_token_idx - 1;
        Ok(self.current_token_idx)
    }

    fn get_token(&self) -> &'a JsonToken {
        self.tokens.get(self.current_token_idx).unwrap()
    }

    fn get_string_content(&self, token: &JsonToken) -> Result<String, JsonParserError> {
        if token.kind == JsonTokenKind::String {
            let content = self
                .source
                .get(token.span.start + 1..token.span.end - 1) // ignore wrapper \" and \"
                .ok_or(JsonParserError::ParserError(String::from(
                    "Can't get content of string",
                )))?;
            return Ok(String::from(content));
        } else {
            return Err(JsonParserError::ParserError(String::from(
                "Can't get content of string",
            )));
        }
    }

    fn get_number_content(&self, token: &JsonToken) -> Result<f32, JsonParserError> {
        if token.kind == JsonTokenKind::Number {
            let content = self.source.get(token.span.start..token.span.end).ok_or(
                JsonParserError::ParserError(String::from("Can't get content of number")),
            )?;
            let number = String::from(content).parse::<f32>().map_err(|_| {
                JsonParserError::ParserError(String::from("Can't parse number content to float32"))
            })?;
            return Ok(number);
        }

        Err(JsonParserError::ParserError(String::from(
            "Can't get content of number",
        )))
    }
}

#[cfg(test)]
mod test {
    use crate::json::{core::JsonValue, error::JsonParserError, lexer::Lexer, parser::JsonParser};
    use std::collections::HashMap;

    fn parse(source: &str) -> Result<JsonValue, JsonParserError> {
        let mut lexer = Lexer::new();
        let tokens = lexer.parse(source)?;
        for t in tokens.clone() {
            Lexer::print_token_with_value(t, source);
        }
        let mut parser = JsonParser::new(&tokens, source);
        parser.parse()
    }

    #[test]
    fn parse_singlular_value() {
        let input: [&str; 5] = ["10", "true", "false", "null", "\"hello world\""];
        let values: [Result<JsonValue, _>; 5] = [
            JsonValue::Number(10.),
            JsonValue::True,
            JsonValue::False,
            JsonValue::Null,
            JsonValue::String(String::from("hello world")),
        ]
        .map(|item| Ok(item));
        for (s, expected) in input.iter().zip(values.iter()) {
            assert_eq!(&parse(s), expected);
        }
    }

    #[test]
    fn parse_empty_values() {
        let input: [&str; 3] = ["\"\"", "[]", "{}"];
        let expected: [Result<JsonValue, _>; 3] = [
            JsonValue::String(String::new()),
            JsonValue::Array(vec![]),
            JsonValue::Object(HashMap::<String, JsonValue>::new()),
        ]
        .map(|item| Ok(item));
        for (s, expected) in input.iter().zip(expected.iter()) {
            assert_eq!(&parse(s), expected);
        }
    }
}
