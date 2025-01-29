use crate::{Context, PoiseResult};
use rmini_core::freqs::*;

fn fmt_ngrams(ngram_type: NgramType, ngrams: &[Ngram]) -> Option<String> {
    if ngrams.is_empty() {
        return None
    }

    let freqs = ngrams.iter().map(|t| format!("  {}: {:.2}%\n", t.ngram, t.frequency))
        .collect::<String>();

    let fmt = format!("\n{}s:\n{freqs}", ngram_type.name());

    Some(fmt)
}

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
            let ngrams = n.split_whitespace().collect::<Vec<_>>();

            let pool = &ctx.data().pool;

            let settings = get_user_settings(pool, user_id.get()).await?;
            let ngrams = get_ngrams(pool, &settings.selected_corpus, &ngrams).await?;

            let chars = fmt_ngrams(NgramType::Char, &ngrams.chars).unwrap_or_default();
            let bigrams = fmt_ngrams(NgramType::Bigram, &ngrams.bigrams).unwrap_or_default();
            let skipgrams = fmt_ngrams(NgramType::Skipgram, &ngrams.skipgrams).unwrap_or_default();
            let trigrams = fmt_ngrams(NgramType::Trigram, &ngrams.trigrams).unwrap_or_default();

            format!(
                "```\ncorpus: {}\n{}{}{}{}```",
                settings.selected_corpus, chars, bigrams, skipgrams, trigrams
            )
        }
        None => "Usage: freqs <up to 6 ngrams>".to_owned(),
    };

    ctx.say(response).await?;

    Ok(())
}
