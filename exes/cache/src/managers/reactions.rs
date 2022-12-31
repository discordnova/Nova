use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;


#[derive(Default)]
pub struct Reactions {}
impl CacheManager for Reactions {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::ReactionAdd(_) => {},
                DispatchEvent::ReactionRemove(_) => {},
                DispatchEvent::ReactionRemoveAll(_) => {},
                DispatchEvent::ReactionRemoveEmoji(_) => {},
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
