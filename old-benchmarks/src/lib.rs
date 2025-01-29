mod get_trigrams_json;

pub use get_trigrams_json::*;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, Error>;
