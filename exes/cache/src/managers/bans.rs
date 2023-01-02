use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;

#[derive(Default)]
pub struct Bans {}
impl CacheManager for Bans {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::BanAdd(_) => {}
                DispatchEvent::BanRemove(_) => {}
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
