use crate::types::slash_commands::ApplicationCommand;

pub struct ApplicationCommandDelete {
    guild_id: Option<String>,
    command: ApplicationCommand,
}