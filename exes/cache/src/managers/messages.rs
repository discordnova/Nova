use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;


#[derive(Default)]
pub struct Messages {}
impl CacheManager for Messages {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::MessageCreate(_) => {},
                DispatchEvent::MessageDelete(_) => {},
                DispatchEvent::MessageDeleteBulk(_) => {},
                DispatchEvent::MessageUpdate(_) => {},
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
