use crate::{Context, PoiseResult};
use rmini_core::freqs::*;

#[poise::command(slash_command, prefix_command)]
pub async fn freqs(
    ctx: Context<'_>,
    #[description = "Up to 6 ngrams"]
    #[rest]
    ngrams: Option<String>,
) -> PoiseResult<()> {
    let user_id = ctx.author().id;

    let response = match ngrams {
        Some(n) => {
            let split = n.split_whitespace();
            let ngrams = split.filter(|s| s.len() == 3).collect::<Vec<_>>();

            let pool = &ctx.data().pool;

            let settings = get_user_settings(pool, user_id.get()).await?;
            let trigrams = get_trigrams(pool, &settings.selected_corpus, &ngrams).await?;

            let content = trigrams
                .iter()
                .map(|t| format!("  {}: {:.2}%", t.trigram, t.frequency))
                .collect::<Vec<_>>();

            let freqs = content.join("\n");

            format!(
                concat!("```\n", "corpus: {}\n", "{}\n", "```"),
                settings.selected_corpus, freqs
            )
        }
        None => "Usage: freqs <up to 6 ngrams>".to_owned(),
    };

    ctx.say(response).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use sqlx::sqlite::SqlitePoolOptions;

    #[test]
    fn test_trigrams_sql() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let database_url = "./test.db";

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

        let freqs = rt.block_on(get_trigrams(&pool, corpus, trigrams));

        println!("{freqs:?}");
    }

    #[test]
    fn what() {
        let u1 = std::hint::black_box(i64::MAX as u64);
        let too_big = u1 + 1;
        let i1 = too_big as i64;
        let i2 = unsafe { std::mem::transmute::<u64, i64>(too_big) };

        let u2 = i1 as u64;

        println!("u1: {u1}\ni1: {i1}\ni2: {i2}\nu2: {u2}");
    }
}
