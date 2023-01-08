use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;

#[derive(Default)]
pub struct Members {}
impl CacheManager for Members {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::MemberAdd(_) => {}
                DispatchEvent::MemberRemove(_) => {}
                DispatchEvent::MemberUpdate(_) => {}
                DispatchEvent::MemberChunk(_) => {}
                DispatchEvent::UserUpdate(_) => {}
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
