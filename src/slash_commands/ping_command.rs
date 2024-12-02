use serenity::all::{CommandInteraction, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage, Timestamp};

pub fn run_command(command: &CommandInteraction) -> CreateInteractionResponse {
    let diff = command.id.created_at().to_utc() - Timestamp::now().to_utc();

    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content(
            format!(":ping_pong: Pong! ({} ms)", diff.num_milliseconds())
        )
    )
}

pub fn create_command() -> CreateCommand {
    CreateCommand::new("ping")
        .description("Pings the bot")
}