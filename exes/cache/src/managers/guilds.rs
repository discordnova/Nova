use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;

#[derive(Default)]
pub struct Guilds {}
impl CacheManager for Guilds {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::GuildCreate(_) => {}
                DispatchEvent::GuildDelete(_) => {}
                DispatchEvent::UnavailableGuild(_) => {}
                DispatchEvent::GuildUpdate(_) => {}
                DispatchEvent::WebhooksUpdate(_) => {}
                DispatchEvent::GuildStickersUpdate(_) => {}
                DispatchEvent::GuildEmojisUpdate(_) => {}
                DispatchEvent::VoiceServerUpdate(_) => {}
                DispatchEvent::GuildIntegrationsUpdate(_) => {}
                DispatchEvent::CommandPermissionsUpdate(_) => {}
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
