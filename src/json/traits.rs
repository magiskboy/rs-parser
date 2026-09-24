use crate::json::{core::JsonValue, error::JsonParserError};

pub trait Serializer {
    fn serialize(self) -> Result<String, JsonParserError>;
}

pub trait Deserializer {
    fn deserializer(value: &str) -> Result<JsonValue, JsonParserError>;
}
