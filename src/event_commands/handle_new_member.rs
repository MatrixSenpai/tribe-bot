use serenity::all::{ChannelId, Context, Member, Mentionable, RoleId};
use crate::Env;
use crate::event_commands::GatewayHandler;

impl GatewayHandler {
    pub async fn handle_new_member(&self, ctx: Context, member: Member) {
        let env_binding = ctx.data.read().await;
        let env = env_binding.get::<Env>().expect("Environment not loaded in context!");

        let new_member_role = RoleId::new(env.new_member_role);
        if let Err(e) = member.add_role(&ctx.http, new_member_role).await {
            error!("Could not assign new member role! {e:?}");
        }

        let intro_channel = ChannelId::new(env.intro_channel);
        if let Err(e) = intro_channel.say(
            &ctx.http,
            format!("{} Welcome to Texas Tribe! Please post your ASL here for entry and a mod will review your request", member.mention())
        ).await {
            error!("Could not send welcome message to new member! {e:?}");
        }
    }
}