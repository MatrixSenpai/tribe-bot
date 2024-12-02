mod ping_command;

use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, Ready};
use serenity::prelude::*;

pub struct SlashCommandHandler;

#[serenity::async_trait]
impl EventHandler for SlashCommandHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        for guild in data_about_bot.guilds.iter() {
            guild.id.set_commands(&ctx.http, vec![
                ping_command::create_command(),
            ]).await.expect("Could not create command in guild");
        }

        info!("Slash command handler created all commands successfully");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            debug!("Command received: {command:?}");

            let command_data = match command.data.name.as_str() {
                "ping" => Some(ping_command::run_command(&command)),

                _ => {
                    error!("Somehow an unrecognized command was applied!");
                    None
                }
            };

            if let Some(content) = command_data {
                if let Err(context) = command.create_response(&ctx.http, content).await {
                    error!("Could not send response to interaction: {context:?}");
                }
            }
        }
    }
}