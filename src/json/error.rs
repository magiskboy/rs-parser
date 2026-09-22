#[derive(Debug, Clone)]
pub enum JsonParserError {
    LexicalError(String),
    ParserError(String),
    SerializeError(String),
    DeserializeError(String),
}
