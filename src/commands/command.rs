use crate::{expiry_manager::ExpiryManager, memory::Memory, resp::Resp, server::ServerMetaData};
use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

// Centralized shared state
pub struct SharedState {
    pub memory: Arc<Mutex<Memory>>,
    pub expiry_manager: Arc<Mutex<ExpiryManager>>,
}

pub struct CommandContext {
    pub stream: TcpStream,
    pub state: Arc<SharedState>, // Use Arc to share the state
    pub server_meta_data: ServerMetaData,
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[Resp], ctx: &mut CommandContext);
}

// Centralized error messages
pub mod errors {
    pub const INVALID_GET_KEY: &str = "invalid GET key";
    pub const INVALID_DEL_KEY: &str = "invalid DEL key";
    pub const INVALID_SET_KEY: &str = "invalid SET key";
}

// Helper function for parsing BulkString arguments
pub fn parse_bulk_string_arg(arg: &Resp) -> Result<String, &'static str> {
    if let Resp::BulkString(Some(value)) = arg {
        Ok(value.clone())
    } else {
        Err("invalid argument format")
    }
}

// Utility function to parse optional flags with generic value type
pub fn parse_optional_flags<T: std::str::FromStr>(
    args: &[Resp],
) -> Result<HashMap<String, T>, &'static str> {
    let mut flags = HashMap::new();

    for chunk in args.chunks(2) {
        if let [Resp::BulkString(Some(flag)), Resp::BulkString(Some(value))] = chunk {
            match value.parse::<T>() {
                Ok(parsed_value) => {
                    flags.insert(flag.to_ascii_uppercase(), parsed_value);
                }
                Err(_) => return Err("invalid flag value"),
            }
        } else {
            return Err("invalid flag format");
        }
    }

    Ok(flags)
}

pub fn send_resp(stream: &mut TcpStream, response: Resp) {
    let encoded_response = response.to_string();
    let _ = stream.write_all(encoded_response.as_bytes());
}

pub fn send_error(stream: &mut TcpStream, message: &str) {
    let _ = stream.write_all(format!("-ERR {}\r\n", message).as_bytes());
}

pub fn send_ok(stream: &mut TcpStream) {
    send_resp(stream, Resp::SimpleString("OK".to_owned()));
}
