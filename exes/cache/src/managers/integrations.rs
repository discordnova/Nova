use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

use super::CacheManager;
use std::future::Future;

#[derive(Default)]
pub struct Integrations {}
impl CacheManager for Integrations {
    fn handle(
        &self,
        event: twilight_model::gateway::event::DispatchEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::CacheSourcedEvents>>> {
        Box::pin(async move {
            match event {
                DispatchEvent::IntegrationCreate(_) => {}
                DispatchEvent::IntegrationDelete(_) => {}
                DispatchEvent::IntegrationUpdate(_) => {}
                DispatchEvent::InteractionCreate(_) => {}
                _ => unreachable!(),
            };

            CacheSourcedEvents::None
        })
    }
}
