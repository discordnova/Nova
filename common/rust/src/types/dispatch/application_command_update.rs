use crate::types::slash_commands::ApplicationCommand;

pub struct ApplicationCommandUpdate {
    guild_id: Option<String>,
    command: ApplicationCommand,
}
