use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;

#[derive(Default)]
pub struct Channels {}
impl CacheManager for Channels {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::ChannelCreate(_) => {}
                DispatchEvent::ChannelDelete(_) => {}
                DispatchEvent::ChannelPinsUpdate(_) => {}
                DispatchEvent::ChannelUpdate(_) => {}
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
