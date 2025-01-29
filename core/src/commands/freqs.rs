use crate::Result;
use sqlx::{prelude::*, SqlitePool};

#[derive(Debug, Clone)]
pub enum NgramType {
    Char,
    Bigram,
    Skipgram,
    Trigram,
}

#[derive(Debug, Clone)]
pub struct Ngrams {
    pub chars: Vec<Ngram>,
    pub bigrams: Vec<Ngram>,
    pub skipgrams: Vec<Ngram>,
    pub trigrams: Vec<Ngram>,
}

impl Ngrams {
    pub fn has_chars(&self) -> bool {
        !self.chars.is_empty()
    }

    pub fn has_bigrams(&self) -> bool {
        !self.bigrams.is_empty()
    }

    pub fn has_skipgrams(&self) -> bool {
        !self.skipgrams.is_empty()
    }

    pub fn has_trigrams(&self) -> bool {
        !self.trigrams.is_empty()
    }
}

impl NgramType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Char => "char",
            Self::Bigram => "bigram",
            Self::Skipgram => "skipgram",
            Self::Trigram => "trigram",
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Char => 1,
            Self::Bigram => 2,
            Self::Skipgram => 2,
            Self::Trigram => 3,
        }
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct Ngram {
    pub ngram: String,
    pub frequency: f64,
}

#[derive(Clone, Debug, FromRow)]
pub struct UserSettings {
    pub user_id: i64,
    pub selected_corpus: String,
}

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

async fn get_particular_ngrams(
    pool: &SqlitePool,
    corpus: &str,
    ngram_type: NgramType,
    ngrams: &[&str],
) -> Result<Vec<Ngram>> {
    let ngrams = ngrams
        .iter()
        .filter(|n| n.len() == ngram_type.len())
        .collect::<Vec<_>>();

    if ngrams.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = ngrams.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query_str = format!(
        r#"
            SELECT ngram, frequency
            FROM {}s
            WHERE corpus = ?
                AND ngram IN ({})
        "#,
        ngram_type.name(),
        placeholders,
    );

    let mut query = sqlx::query_as::<_, Ngram>(&query_str).bind(&corpus);
    for ngram in ngrams {
        query = query.bind(ngram);
    }

    let mut ngrams = query.fetch_all(pool).await?;
    ngrams.reverse();

    Ok(ngrams)
}

pub async fn get_ngrams(pool: &SqlitePool, corpus: &str, ngrams: &[&str]) -> Result<Ngrams> {
    let chars = get_particular_ngrams(pool, corpus, NgramType::Char, ngrams).await?;
    let bigrams = get_particular_ngrams(pool, corpus, NgramType::Bigram, ngrams).await?;
    let skipgrams = get_particular_ngrams(pool, corpus, NgramType::Skipgram, ngrams).await?;
    let trigrams = get_particular_ngrams(pool, corpus, NgramType::Trigram, ngrams).await?;
    
    let ngrams = Ngrams {
        chars, bigrams, skipgrams, trigrams
    };

    Ok(ngrams)
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
            .block_on(get_ngrams(&pool, corpus, trigrams))
            .expect("Couldn't fetch freqs: ");

        assert_eq!(freqs.trigrams.len(), trigrams.len());

        println!("{freqs:?}");
    }
}
