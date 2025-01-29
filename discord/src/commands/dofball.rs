use crate::{Context, PoiseResult};
use nanorand::{Rng, WyRand};

#[poise::command(prefix_command)]
pub async fn dofball(ctx: Context<'_>, #[rest] _rest: Option<String>) -> PoiseResult<()> {
    let response = match ctx.guild() {
        Some(guild) => {
            let mut rng = WyRand::new();

            match ctx.data().dof_cache.lock() {
                Ok(mut dofs) => match dofs.get(&guild.id) {
                    Some(mojis) => {
                        println!("Using cached dofs!");

                        let idx = rng.generate_range(0..(mojis.len()));
                        mojis[idx].clone()
                    }
                    None => {
                        println!("No cache yet. Caching...");

                        let mojis = guild
                            .emojis
                            .values()
                            .filter(|moji| moji.available && moji.name.contains("dof"))
                            .map(|moji| format!("{moji}"))
                            .collect::<Vec<_>>();

                        let idx = rng.generate_range(0..(mojis.len()));
                        let moji = mojis[idx].clone();

                        dofs.insert(guild.id, mojis);

                        moji
                    }
                },
                Err(_) => {
                    println!("Mutex error. Using fallback:");

                    let mojis = guild
                        .emojis
                        .values()
                        .cloned()
                        .filter(|moji| moji.available && moji.name.contains("dof"))
                        .map(|moji| format!("{moji}"))
                        .collect::<Vec<_>>();

                    let idx = rng.generate_range(0..(mojis.len()));
                    mojis[idx].clone()
                }
            }
        }

        None => "No dofs here :(".to_owned(),
    };

    ctx.say(response).await?;

    Ok(())
}
