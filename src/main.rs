mod cli;
mod commands;
mod expiry_manager;
mod memory;
mod resp;
mod server;
mod utils;

use std::sync::{Arc, Mutex};

use cli::RedisServerOptions;
use utils::role::Role;

fn main() {
    // Create shared memory
    let memory = Arc::new(Mutex::new(memory::Memory::new()));

    // Create command registry
    let registry = commands::create_registry();

    // Create expiry manager
    let expiry_manager = Arc::new(Mutex::new(expiry_manager::ExpiryManager::new()));

    let options: RedisServerOptions = argh::from_env();

    let address = format!("127.0.0.1:{}", options.port);

    let role = Role::new(options.replicaof);

    // Create and run server
    let server = server::Server::new(&address, memory, registry, expiry_manager, role)
        .expect("Failed to create server");

    server.run();
}
