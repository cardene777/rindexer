use std::path::PathBuf;

use rindexer::event::callback_registry::EventCallbackRegistry;

use super::rindexer_playground::rocket_pool_eth::rocket_pool_eth_handlers;

pub async fn register_all_handlers(manifest_path: &PathBuf) -> EventCallbackRegistry {
    let mut registry = EventCallbackRegistry::new();
    rocket_pool_eth_handlers(manifest_path, &mut registry).await;
    registry
}
