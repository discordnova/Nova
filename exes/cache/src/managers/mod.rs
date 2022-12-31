use std::pin::Pin;
use twilight_model::gateway::event::DispatchEvent;
use std::future::Future;

use crate::CacheSourcedEvents;

pub mod channels;
pub mod guilds;
pub mod guild_schedules;
pub mod stage_instances;
pub mod integrations;
pub mod members;
pub mod bans;
pub mod reactions;
pub mod messages;
pub mod threads;
pub mod invites;
pub mod roles;
pub mod automoderation;

pub trait CacheManager {
    fn handle(&self, event: DispatchEvent) -> Pin<Box<dyn Future<Output = CacheSourcedEvents>>>;
}
