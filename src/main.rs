mod commands;
mod expiry_manager;
mod memory;
mod resp;
mod server;

use std::{
    env,
    sync::{Arc, Mutex},
};

fn main() {
    // Create shared memory
    let memory = Arc::new(Mutex::new(memory::Memory::new()));

    // Create command registry
    let registry = commands::create_registry();

    // Create expiry manager
    let expiry_manager = Arc::new(Mutex::new(expiry_manager::ExpiryManager::new()));

    let port = env::args().nth(2).unwrap_or("6379".to_string());

    let address = format!("127.0.0.1:{}", port);

    // Create and run server
    let server = server::Server::new(&address, memory, registry, expiry_manager)
        .expect("Failed to create server");

    server.run();
}
