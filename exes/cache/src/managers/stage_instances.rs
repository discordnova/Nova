use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;

#[derive(Default)]
pub struct StageInstances {}
impl CacheManager for StageInstances {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::StageInstanceCreate(_) => {}
                DispatchEvent::StageInstanceDelete(_) => {}
                DispatchEvent::StageInstanceUpdate(_) => {}
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
