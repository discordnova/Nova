use crate::types::slash_commands::ApplicationCommand;

pub struct  ApplicationCommandCreate {
    guild_id: Option<String>,
    command: ApplicationCommand,
}
