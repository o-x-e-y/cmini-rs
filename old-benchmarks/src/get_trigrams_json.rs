use std::collections::HashMap;

use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Trigrams(HashMap<String, f64>);

pub fn get_trigrams_json<'a>(trigrams: &[&'a str]) -> Result<Vec<(&'a str, f64)>> {
    let s = std::fs::read_to_string("./corpora/trigrams.json")?;
    let json = serde_json::from_str::<Trigrams>(&s)?;

    Ok(trigrams
        .iter()
        .map(|&t| (t, *json.0.get(t).unwrap_or(&0.0)))
        .collect())
}
