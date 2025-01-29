use crate::{Context, PoiseResult};

#[poise::command(prefix_command)]
pub async fn server(ctx: Context<'_>, #[rest] _rest: Option<String>) -> PoiseResult<()> {
    let response = match ctx.guild() {
        Some(guild) => {
            let roles = guild
                .roles
                .iter()
                .filter(|(_, r)| r.colour.0 == 1752220)
                .map(|(_, r)| format!("{}: {}", r.name, r.colour.0))
                .collect::<Vec<_>>();

            format!(
                "----- {} stats -----\n```{}```",
                guild.name,
                roles.join("\n")
            )
        }
        None => "No guild found".to_owned(),
    };

    ctx.say(response).await?;

    Ok(())
}
