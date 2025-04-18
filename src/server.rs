use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    commands::{command::SharedState, send_error, CommandContext, SharedRegistry},
    expiry_manager,
    memory::Memory,
    resp::{parse_resp, Resp},
    utils::role::Role,
};

#[derive(Clone)]
pub struct ServerMetaData {
    pub role: Role,
}

pub struct Server {
    listener: TcpListener,
    memory: Arc<Mutex<Memory>>,
    command_registry: SharedRegistry,
    expiry_manager: Arc<Mutex<expiry_manager::ExpiryManager>>,
    server_metadata: ServerMetaData,
}

impl Server {
    pub fn new(
        address: &str,
        memory: Arc<Mutex<Memory>>,
        command_registry: SharedRegistry,
        expiry_manager: Arc<Mutex<expiry_manager::ExpiryManager>>,
        role: Role,
    ) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(address)?;
        Ok(Server {
            listener,
            memory,
            command_registry,
            expiry_manager,
            server_metadata: ServerMetaData { role },
        })
    }

    pub fn run(&self) {
        println!("Server running on {}", self.listener.local_addr().unwrap());

        let memory_clone = Arc::clone(&self.memory);
        let expiry_manager_clone = Arc::clone(&self.expiry_manager);

        thread::spawn(move || {
            loop {
                {
                    let mut expiry_manager = expiry_manager_clone.lock().unwrap();
                    let mut memory = memory_clone.lock().unwrap();

                    // Cleanup expired keys and remove them from memory
                    expiry_manager.cleanup_expired_keys(|key| {
                        println!("deleted key: {}", key);
                        memory.delete(key);
                    });
                }
                thread::sleep(Duration::from_millis(100)); // Run cleanup every 100ms
            }
        });

        for stream in self.listener.incoming() {
            match stream {
                Ok(tcp_stream) => {
                    let memory = self.memory.clone();
                    let registry = self.command_registry.clone();
                    let expiry_manager = self.expiry_manager.clone();
                    let metadata = self.server_metadata.clone();
                    std::thread::spawn(move || {
                        handle_client(memory, registry, tcp_stream, expiry_manager, metadata)
                    });
                }
                Err(e) => eprintln!("Connection error: {}", e),
            }
        }
    }
}

fn handle_client(
    memory: Arc<Mutex<Memory>>,
    registry: SharedRegistry,
    mut stream: TcpStream,
    expiry_manager: Arc<Mutex<crate::expiry_manager::ExpiryManager>>,
    metadata: ServerMetaData,
) {
    let state = Arc::new(SharedState {
        memory,
        expiry_manager,
    });

    println!("Accepted new connection");

    let mut buf = [0; 1024];
    let client_addr = stream
        .peer_addr()
        .unwrap_or_else(|_| "unknown".parse().unwrap());
    println!("New connection from {}", client_addr);

    loop {
        let bytes_read = match stream.read(&mut buf) {
            Ok(0) => {
                println!("Client {} disconnected", client_addr);
                return;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read: {}", e);
                return;
            }
        };

        let input = String::from_utf8_lossy(&buf[..bytes_read]);
        let command = parse_resp(&input);

        match command {
            Ok((_, Resp::Array(arr))) => {
                if let Some(Resp::BulkString(Some(cmd))) = arr.first() {
                    let registry = registry.lock().unwrap();
                    if let Some(command) = registry.get_command(cmd) {
                        let mut context = CommandContext {
                            stream: stream.try_clone().unwrap(),
                            state: state.clone(), // Use Arc to share the state
                            server_meta_data: metadata.clone(),
                        };
                        command.execute(&arr[1..], &mut context);
                    } else {
                        send_error(&mut stream, &format!("unknown command '{}'", cmd));
                    }
                } else {
                    send_error(&mut stream, "invalid command format");
                }
            }
            Ok((remaining, _)) => {
                if !remaining.is_empty() {
                    eprintln!("Unprocessed input remaining: {:?}", remaining);
                    send_error(&mut stream, "unprocessed input remaining");
                }
            }
            Err(e) => {
                eprintln!("Error parsing input: {:?}", e);
                send_error(&mut stream, "invalid input");
            }
        }
    }
}
