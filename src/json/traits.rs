use crate::json::error::JsonParserError;

pub trait Serializer {
    fn serialize(self) -> Result<String, JsonParserError>;
}

pub trait Deserializer {
    fn deserializer<T>(value: &str) -> Result<T, JsonParserError>;
}
