use leash::ignite;
use ratelimit::RatelimiterServerComponent;

#[cfg(not(target_os = "windows"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

ignite!(RatelimiterServerComponent);
