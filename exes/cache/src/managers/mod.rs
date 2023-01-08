use std::future::Future;
use std::pin::Pin;
use twilight_model::gateway::event::DispatchEvent;

use crate::CacheSourcedEvents;

pub mod automoderation;
pub mod bans;
pub mod channels;
pub mod guild_schedules;
pub mod guilds;
pub mod integrations;
pub mod invites;
pub mod members;
pub mod messages;
pub mod reactions;
pub mod roles;
pub mod stage_instances;
pub mod threads;

pub trait CacheManager {
    fn handle(&self, event: DispatchEvent) -> Pin<Box<dyn Future<Output = CacheSourcedEvents>>>;
}
