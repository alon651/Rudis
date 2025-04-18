pub(crate) mod command;
mod del;
mod echo;
mod get;
mod keys;
mod ping;
mod set;

pub use command::{send_error, Command, CommandContext};
use del::DelCommand;
use echo::EchoCommand;
use get::GetCommand;
use keys::KeysCommand;
use ping::PingCommand;
use set::SetCommand;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command + Send + Sync>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = CommandRegistry {
            commands: HashMap::new(),
        };

        // Register built-in commands
        registry.register(Box::new(PingCommand));
        registry.register(Box::new(EchoCommand));
        registry.register(Box::new(GetCommand));
        registry.register(Box::new(SetCommand));
        registry.register(Box::new(KeysCommand)); // Register KEYS command
        registry.register(Box::new(DelCommand));

        registry
    }

    pub fn register(&mut self, command: Box<dyn Command + Send + Sync>) {
        let name = command.name().to_uppercase();
        self.commands.insert(name, command);
    }

    pub fn get_command(&self, name: &str) -> Option<&(dyn Command + Send + Sync)> {
        self.commands
            .get(&name.to_uppercase())
            .map(|cmd| cmd.as_ref())
    }
}

// Make the registry thread-safe and shareable
pub type SharedRegistry = Arc<Mutex<CommandRegistry>>;

pub fn create_registry() -> SharedRegistry {
    Arc::new(Mutex::new(CommandRegistry::new()))
}
