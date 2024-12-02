use serenity::all::{ButtonStyle, ChannelId, Context, CreateButton, CreateInteractionResponse, CreateMessage, Interaction, Message, MessageId, Ready, UserId};
use serenity::model::channel::ReactionType;
use crate::Env;
use crate::event_commands::GatewayHandler;

impl GatewayHandler {
    pub async fn handle_intro_message(&self, ctx: Context, msg: Message) {
        let env_binding = ctx.data.read().await;
        let env = env_binding.get::<Env>().expect("Environment is not in context!");

        let current_user_id = ctx.cache.current_user().id;
        let guild_id = msg.guild_id.map(|g| g.get()).unwrap_or_default();
        let should_handle = env.guild_id_list.contains(&guild_id)
                && msg.channel_id.get().eq(&env.intro_channel)
                && current_user_id.ne(&msg.author.id);
        if !should_handle { return }
        info!("New message in {} by {} > {}", msg.channel_id, msg.author.name, msg.content);

        let asl_regex = regex::Regex::new(r"^(?<age>\d{2})[/\s]?(?<sex>\w+)[/\s](?<loc>\w+\s?\w+?|\d{3})$").unwrap();
        if let Some(matches) = asl_regex.captures(&msg.content) {
            let admin_channel = ChannelId::new(env.admin_channel);

            let allow_button = CreateButton::new(format!("approve:{}", msg.author.id))
                .label("Approve")
                .style(ButtonStyle::Success)
                .emoji(ReactionType::from('✅'));
            let more_info = CreateButton::new(format!("identify:{}", msg.author.id))
                .label("Request Info")
                .style(ButtonStyle::Primary)
                .disabled(true)
                .emoji(ReactionType::from('🪪'));
            let deny_button = CreateButton::new(format!("deny:{}", msg.author.id))
                .label("Deny")
                .style(ButtonStyle::Danger)
                .emoji(ReactionType::from('❎'));

            let content = format!(
                "A new ASL has been posted from {}\nDetected stats: {} / {} / {}\nSee more here: {}",
                msg.author.name,
                &matches["age"],
                &matches["sex"],
                &matches["loc"],
                msg.link(),
            );

            let add_message = CreateMessage::new()
                .content(content)
                .button(allow_button)
                .button(more_info)
                .button(deny_button);

            if let Err(e) = admin_channel.send_message(&ctx.http, add_message).await {
                error!("Could not post new user to admin channel! {e:?}");
            }

            if let Err(e) = msg.reply_ping(&ctx.http, "Your entry is being reviewed").await {
                error!("Could not respond to entry! {e:?}");
            }
        } else {
            if let Err(e) = msg.reply_ping(&ctx.http, "Please post a recognizable asl format only!").await {
                error!("Could not reprimand user {} for message {}\n{e:?}", msg.author.name, msg.content);
            }

            if let Err(e) = msg.delete(&ctx.http).await {
                error!("Could not remove improperly formatted message! {e:?}");
            }
        }
    }
}