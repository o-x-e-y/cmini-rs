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
