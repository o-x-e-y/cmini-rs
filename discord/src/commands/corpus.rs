use crate::{Context, PoiseResult};
use sqlx::SqlitePool;

use rmini_core::corpus::*;

#[allow(unused)]
async fn fmt_all_corpora(pool: &SqlitePool) -> PoiseResult<String> {
    let corpora = get_all_corpora(pool).await?;

    let corpora_str = corpora
        .into_iter()
        .map(|c| format!("- {}\n", c.corpus))
        .collect::<String>();

    let msg = format!("```\nList of Corpora:\n{corpora_str}```");

    Ok(msg)
}

#[poise::command(prefix_command)]
pub async fn corpus(ctx: Context<'_>, #[rest] corpus: Option<String>) -> PoiseResult<()> {
    let pool = &ctx.data().pool;

    let response = match corpus {
        Some(c) => {
            let corpus = get_corpus(pool, &c).await?;

            match corpus {
                Some(c) => {
                    let added = set_user_corpus(pool, &c.corpus, ctx.author().id.get()).await?;

                    if !added {
                        format!("Couldn't set corpus. Please try again!")
                    } else {
                        format!("Set corpus to `{}`.", c.corpus)
                    }
                }
                None => fmt_all_corpora(&pool).await?,
            }
        }
        None => fmt_all_corpora(&pool).await?,
    };

    ctx.say(response).await?;

    Ok(())
}
