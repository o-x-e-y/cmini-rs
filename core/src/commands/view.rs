// use crate::{Context, PoiseResult};

// #[poise::command(prefix_command)]
// pub async fn view(ctx: Context<'_>, #[rest] layout: Option<String>) -> PoiseResult<()> {
//     let response = match layout {
//         Some(name) => format!("Stats of layout '{name}'"),
//         None => format!("Please specify a layout."),
//     };

//     ctx.say(response).await?;

//     Ok(())
// }
