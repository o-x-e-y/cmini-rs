use crate::{Context, PoiseResult};

#[poise::command(prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    name: Option<String>,
    #[rest] _layout: Option<String>,
) -> PoiseResult<()> {
    let response = match name {
        Some(name) => format!("Added {name}!"),
        None => format!("Please specify a layout."),
    };

    ctx.say(response).await?;

    Ok(())
}
