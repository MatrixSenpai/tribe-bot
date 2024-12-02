use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, Interaction, Member, RoleId, User, UserId};
use crate::Env;
use crate::event_commands::GatewayHandler;

impl GatewayHandler {
    pub async fn handle_user_actions(&self, ctx: Context, interaction: Interaction) {
        // info!("New interaction: {:?}", interaction);

        if let Some(command) = interaction.into_message_component() {
            if let Err(e) = command.create_response(&ctx.http, CreateInteractionResponse::Acknowledge).await {
                error!("Could not send acknowledge: {e:?}");
                return
            }

            info!("Custom id: {}", command.data.custom_id);
            let (status, user_id) = command.data.custom_id.split_once(":").unwrap();
            let user_id = UserId::new(user_id.parse().unwrap());

            let user = match user_id.to_user(&ctx.http).await {
                Ok(user) => user,
                Err(e) => {
                    error!("Could not retrieve user by id: {e:?}");
                    return
                }
            };
            info!("User retrieved from message. Action: {status}, user: {}", user.name);

            match status {
                "approve" => self.approve_new_user(&ctx, &command, user).await,
                "deny" => self.deny_new_user(&ctx, &command, user).await,

                _ => error!("Unknown interaction requested!"),
            }
        }
    }

    async fn approve_new_user(&self, ctx: &Context, interaction: &ComponentInteraction, user: User) {
        let env_binding = ctx.data.read().await;
        let env = env_binding.get::<Env>().expect("Environment not in context!");

        let guild = interaction.guild_id.expect("Could not get guild id!");
        let member = guild.member(&ctx.http, user.id).await.expect("Could not fetch member details!");

        let intro_role = RoleId::new(env.new_member_role);
        let regular_role = RoleId::new(env.regular_member_role);

        if let Err(e) = member.remove_role(&ctx.http, intro_role).await {
            error!("Could not remove intro role! {e:?}");
        }
        if let Err(e) = member.add_role(&ctx.http, regular_role).await {
            error!("Could not add regular role! {e:?}");
        }
    }

    async fn deny_new_user(&self, ctx: &Context, interaction: &ComponentInteraction, user: User) {
        let env_binding = ctx.data.read().await;
        let env = env_binding.get::<Env>().expect("Environment not in context!");

        let guild = interaction.guild_id.expect("Could not get guild id!");
        let member = guild.member(&ctx.http, user.id).await.expect("Could not fetch member details!");

        member.kick_with_reason(&ctx.http, "Your application was not approved by mods!").await.expect("Could not kick user!");
    }
}