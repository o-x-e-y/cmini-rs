mod commands;

use crate::commands::*;

use std::{collections::HashMap, sync::Mutex};

use poise::{
    serenity_prelude::{self as serenity, GatewayIntents},
    PrefixFrameworkOptions,
};
use sqlx::sqlite::SqlitePoolOptions;

pub struct Data {
    pub dof_cache: Mutex<HashMap<serenity::GuildId, Vec<String>>>,
    pub pool: sqlx::SqlitePool,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type PoiseResult<T> = Result<T, Error>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> PoiseResult<()> {
    env_logger::init();
    dotenvy::dotenv()?;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Couldn't connect to database");

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("rmini".to_owned()),
                additional_prefixes: vec![],
                ..Default::default()
            },
            commands: vec![
                add(),
                age(),
                corpus(),
                dofball(),
                eightball(),
                view(),
                freqs(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    dof_cache: Default::default(),
                    pool: pool,
                })
            })
        })
        .build();

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?
        .start()
        .await?;

    Ok(())
}
