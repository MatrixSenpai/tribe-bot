mod handle_intro_message;
mod handle_new_member;
mod handle_user_actions;

use serenity::prelude::*;
use serenity::all::{ButtonStyle, ChannelId, CreateButton, CreateInteractionResponse, CreateMessage, Interaction, Member, Message, MessageId, Ready, UserId};
use serenity::model::channel::ReactionType;
use crate::Env;

pub struct GatewayHandler;

#[serenity::async_trait]
impl EventHandler for GatewayHandler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        self.handle_new_member(ctx, new_member).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        self.handle_intro_message(ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        info!("tribe bot is online: {} (guilds: {:?})", data_about_bot.user.name, data_about_bot.guilds);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        self.handle_user_actions(ctx, interaction).await;
    }
}