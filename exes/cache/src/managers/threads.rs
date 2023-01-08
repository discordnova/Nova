use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;

#[derive(Default)]
pub struct Threads {}
impl CacheManager for Threads {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::ThreadCreate(_) => {}
                DispatchEvent::ThreadDelete(_) => {}
                DispatchEvent::ThreadListSync(_) => {}
                DispatchEvent::ThreadMemberUpdate(_) => {}
                DispatchEvent::ThreadMembersUpdate(_) => {}
                DispatchEvent::ThreadUpdate(_) => {}
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
