use crate::json::{
    error::JsonParserError,
    lexer::Lexer,
    parser::JsonParser,
    traits::{Deserializer, Serializer},
};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(f32),
    Null,
    True,
    False,
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::True => write!(f, "true"),
            JsonValue::False => write!(f, "false"),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(st) => write!(f, "\"{}\"", st),
            JsonValue::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",{}", item)?;
                    } else {
                        write!(f, "{}", item)?;
                    }
                }
                write!(f, "]")
            }
            JsonValue::Object(value) => {
                write!(f, "{{")?;
                for (i, (k, v)) in value.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",\"{}\":{}", k, v)?;
                    } else {
                        write!(f, "\"{}\":{}", k, v)?;
                    }
                }
                write!(f, "}}")
            }
        }?;
        Ok(())
    }
}

impl Serializer for JsonValue {
    fn serialize(self) -> Result<String, JsonParserError> {
        match self {
            Self::String(val) => Ok(format!("\"{}\"", val.clone())),
            Self::Number(val) => Ok(val.to_string()),
            Self::Null => Ok(String::from("null")),
            Self::True => Ok(String::from("true")),
            Self::False => Ok(String::from("false")),
            Self::Array(val) => {
                let items: Result<Vec<_>, _> =
                    val.iter().map(|item| item.clone().serialize()).collect();
                let items = items?.join(",");
                Ok(format!("[{}]", items))
            }
            Self::Object(val) => {
                let items: Result<Vec<_>, _> = val
                    .iter()
                    .map(|item| {
                        let str = JsonValue::serialize(item.1.clone()).unwrap();
                        return Ok(format!("\"{}\": {}", item.0, str));
                    })
                    .collect();
                let items = items?.join(",");
                Ok(format!("{{{}}}", items))
            }
        }
    }
}

impl Deserializer for JsonValue {
    fn deserializer(source: &str) -> Result<JsonValue, JsonParserError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer
            .parse()
            .map_err(|err| JsonParserError::ParserError(String::from(err.to_string())))?;

        Ok(JsonValue::Null)
        // let mut parser = JsonParser::new(&tokens, source);
        // let value = parser.parse()?;
        // Ok(value)
    }
}

mod test {
    #[allow(warnings)]
    use crate::json::traits::{Deserializer, Serializer};
    #[allow(warnings)]
    use std::collections::HashMap;

    #[test]
    fn test_serialize_single_value() {
        assert_eq!(
            super::JsonValue::Null.serialize().unwrap(),
            String::from("null")
        );
        assert_eq!(
            super::JsonValue::True.serialize().unwrap(),
            String::from("true")
        );
        assert_eq!(
            super::JsonValue::False.serialize().unwrap(),
            String::from("false")
        );
        assert_eq!(
            super::JsonValue::String(String::from("Hello world"))
                .serialize()
                .unwrap(),
            String::from("\"Hello world\"")
        );
        assert_eq!(
            super::JsonValue::Number(3.14).serialize().unwrap(),
            String::from("3.14")
        );
    }

    #[test]
    fn test_array_of_single_value() {
        assert_eq!(
            super::JsonValue::Array(vec![
                super::JsonValue::Number(1.),
                super::JsonValue::String(String::from("Hello world")),
                super::JsonValue::True,
                super::JsonValue::False,
                super::JsonValue::Null
            ])
            .serialize()
            .unwrap(),
            String::from("[1,\"Hello world\",true,false,null]")
        );
    }

    #[test]
    fn test_array_of_complex_value() {
        let item = super::JsonValue::Array(vec![
            super::JsonValue::Number(1.),
            super::JsonValue::String(String::from("Hello world")),
            super::JsonValue::True,
            super::JsonValue::False,
            super::JsonValue::Null,
        ]);

        assert_eq!(
            super::JsonValue::Array(vec![item.clone(), item.clone()])
                .serialize()
                .unwrap(),
            String::from(
                "[[1,\"Hello world\",true,false,null],[1,\"Hello world\",true,false,null]]"
            )
        );
    }

    #[test]
    fn test_object_of_complex_value() {
        let item = super::JsonValue::Array(vec![
            super::JsonValue::Number(1.),
            super::JsonValue::String(String::from("Hello world")),
            super::JsonValue::True,
            super::JsonValue::False,
            super::JsonValue::Null,
        ]);
        let mut hs = HashMap::<String, super::JsonValue>::new();
        hs.insert(String::from("key1"), item.clone());
        hs.insert(String::from("key2"), item.clone());
        let value = super::JsonValue::Object(hs);
        let serialized = value.clone().serialize().unwrap();
        assert_eq!(super::JsonValue::deserializer(&serialized).unwrap(), value);
    }
}
