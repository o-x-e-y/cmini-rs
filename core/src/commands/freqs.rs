// use std::collections::HashMap;

use crate::Result;
use sqlx::{prelude::*, SqlitePool};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct Trigrams(HashMap<String, f64>);

#[derive(Clone, Debug, FromRow)]
pub struct SqlTrigram {
    pub trigram: String,
    pub frequency: f64,
}

#[derive(Clone, Debug, FromRow)]
pub struct UserSettings {
    pub user_id: i64,
    pub selected_corpus: String,
}

// pub fn get_trigrams_json<'a>(trigrams: &[&'a str]) -> PoiseResult<Vec<(&'a str, f64)>> {
//     let s = std::fs::read_to_string("./corpora/trigrams.json")?;
//     let json = serde_json::from_str::<Trigrams>(&s)?;

//     Ok(trigrams
//         .iter()
//         .map(|&t| (t, *json.0.get(t).unwrap_or(&0.0)))
//         .collect())
// }

pub async fn get_user_settings(pool: &SqlitePool, user_id: u64) -> Result<UserSettings> {
    let id = user_id as i64;

    let query = sqlx::query_as!(
        UserSettings,
        r#"
            SELECT *
            FROM settings
            WHERE user_id = ?
            "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(query)
}

pub async fn get_trigrams<'a>(
    pool: &SqlitePool,
    corpus: &str,
    trigrams: &[&'a str],
) -> Result<Vec<SqlTrigram>> {
    let placeholders = trigrams.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query_str = format!(
        r#"
        SELECT trigram, frequency
        FROM trigrams
        WHERE corpus = ?
            AND trigram IN ({})
        "#,
        placeholders
    );

    let mut query = sqlx::query_as::<_, SqlTrigram>(&query_str).bind(&corpus);
    for trigram in trigrams {
        query = query.bind(trigram);
    }

    let mut trigrams = query.fetch_all(pool).await?;
    trigrams.reverse();

    Ok(trigrams)
}

#[cfg(test)]
mod tests {
    use super::*;

    use sqlx::sqlite::SqlitePoolOptions;

    #[test]
    fn test_trigrams_sql() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let database_url = "../test.db";

        let pool = rt
            .block_on(
                SqlitePoolOptions::new()
                    .max_connections(5)
                    .acquire_timeout(std::time::Duration::from_secs(3))
                    .connect(&database_url),
            )
            .expect("Couldn't connect to database");

        let corpus = "english";
        let trigrams = &["the", "dof", "lol"];

        let freqs = rt
            .block_on(get_trigrams(&pool, corpus, trigrams))
            .expect("Couldn't fetch freqs: ");

        assert_eq!(freqs.len(), trigrams.len());

        println!("{freqs:?}");
    }
}
