use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;


#[derive(Default)]
pub struct Invites {}
impl CacheManager for Invites {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::InviteCreate(_) => {}
                DispatchEvent::InviteDelete(_) => {}
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
