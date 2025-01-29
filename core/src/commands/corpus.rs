use crate::Result;
use sqlx::{prelude::*, SqlitePool};

#[derive(Clone, Debug, FromRow)]
pub struct Corpus {
    pub corpus: String,
    pub char_total: i64,
    pub bigram_total: i64,
    pub skipgram_total: i64,
    pub trigram_total: i64,
}

#[derive(Clone, Debug, FromRow)]
pub struct CorpusName {
    pub corpus: String,
}

pub async fn get_all_corpora(pool: &SqlitePool) -> Result<Vec<CorpusName>> {
    let query = sqlx::query_as!(
        CorpusName,
        r#"
            SELECT corpus
            FROM corpus
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(query)
}

pub async fn get_corpus(pool: &SqlitePool, corpus: &str) -> Result<Option<Corpus>> {
    let query = sqlx::query_as!(
        Corpus,
        r#"
            SELECT *
            FROM corpus
            WHERE corpus = ?
        "#,
        corpus
    )
    .fetch_one(pool)
    .await;

    match query {
        Ok(q) => Ok(Some(q)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn set_user_corpus(pool: &SqlitePool, corpus: &str, user_id: u64) -> Result<bool> {
    let id = user_id as i64;

    let rows_affected = sqlx::query!(
        r#"
            UPDATE settings
            SET selected_corpus = ?
            WHERE user_id = ?
        "#,
        corpus,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected > 0 {
        Ok(true)
    } else {
        let rows_affected = sqlx::query!(
            r#"
                INSERT INTO settings
                VALUES (?, ?)
            "#,
            id,
            corpus
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
}
