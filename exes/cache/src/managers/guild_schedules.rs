use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;


#[derive(Default)]
pub struct GuildSchedules {}
impl CacheManager for GuildSchedules {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::GuildScheduledEventCreate(_) => {}
                DispatchEvent::GuildScheduledEventDelete(_) => {}
                DispatchEvent::GuildScheduledEventUpdate(_) => {}
                DispatchEvent::GuildScheduledEventUserAdd(_) => {}
                DispatchEvent::GuildScheduledEventUserRemove(_) => {}
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
