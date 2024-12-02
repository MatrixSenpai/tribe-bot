#![allow(unused, dead_code)]

mod event_commands;
mod slash_commands;

use anyhow::{Context, Result};
use serenity::all::{GetMessages, Message, Ready, TypingStartEvent};
use serenity::prelude::*;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    setup_logger()?;

    let env = Env::new()?;

    let intents = GatewayIntents::non_privileged() |
        GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(env.discord_token.clone(), intents)
        .type_map_insert::<Env>(env)
        .event_handler(event_commands::GatewayHandler)
        .event_handler(slash_commands::SlashCommandHandler)
        .await
        .context("Could not initialize client")?;

    info!("Tribe bot coming online now...install here: https://discord.com/oauth2/authorize?client_id=1309908462322450546");
    client.start().await.context("Could not start client shard")
}

#[derive(Clone, Debug)]
pub struct Env {
    pub discord_token: String,
    pub guild_id_list: Vec<u64>,
    pub admin_channel: u64,
    pub intro_channel: u64,
    pub new_member_role: u64,
    pub regular_member_role: u64,
}

unsafe impl Send for Env {}
unsafe impl Sync for Env {}

impl TypeMapKey for Env { type Value = Self; }

impl Env {
    pub fn new() -> Result<Self> {
        let discord_token = std::env::var("DISCORD_TOKEN")?;
        let guild_id_list = std::env::var("DISCORD_GUILD_LIST")?.split(",")
            .map(|e| e.parse::<u64>().unwrap())
            .collect();
        let admin_channel = std::env::var("DISCORD_ADMIN_CHANNEL")?.parse::<u64>()?;
        let intro_channel = std::env::var("DISCORD_INTRO_CHANNEL")?.parse::<u64>()?;
        let new_member_role = std::env::var("DISCORD_NEW_MEMBER_ROLE")?.parse::<u64>()?;
        let regular_member_role = std::env::var("DISCORD_REGULAR_MEMBER_ROLE")?.parse::<u64>()?;

        Ok(
            Self {
                discord_token,
                guild_id_list,
                admin_channel,
                intro_channel,
                new_member_role,
                regular_member_role,
            }
        )
    }
}

#[cfg(target_os = "windows")]
fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .level(log::LevelFilter::Off)
                .level_for("tribe_bot", log::LevelFilter::Trace)
                .chain(std::io::stdout())
                .format(|out, message, record| {
                    let colors = fern::colors::ColoredLevelConfig::default();
                    out.finish(format_args!(
                        "[{}]\t {} | {} > {}",
                        colors.color(record.level()),
                        record.target(),
                        humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                        message,
                    ))
                })
        )
        .chain(
            fern::Dispatch::new()
                .level(log::LevelFilter::Debug)
                .chain(std::fs::File::create("output.log")?)
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}] ({} | {}) > {}",
                        record.level(),
                        record.target(),
                        humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                        message,
                    ))
                })
        )
        .apply()?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn setup_logger() -> Result<()> {
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_USER,
        hostname: None,
        process: "tribe-bot".to_string(),
        pid: 0
    };
    fern::Dispatch::new()
        .level(log::LevelFilter::Off)
        .level_for("tribe_bot", log::LevelFilter::Trace)
        .chain(syslog::unix(formatter).unwrap())
        .apply()?;

    Ok(())
}