use nanorand::{Rng, WyRand};

use crate::{Context, PoiseResult};
use rmini_core::eightball::*;

#[poise::command(aliases("8ball"), prefix_command)]
pub async fn eightball(ctx: Context<'_>, #[rest] _rest: Option<String>) -> PoiseResult<()> {
    let mut rng = WyRand::new();
    let idx = rng.generate_range(0..RESPONSES.len());

    let response = RESPONSES[idx];

    ctx.say(response).await?;

    Ok(())
}
