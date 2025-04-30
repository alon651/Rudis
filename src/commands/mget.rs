use crate::{
    commands::command::{
        errors, parse_bulk_string_arg, send_error, send_resp, Command, CommandContext,
    },
    resp::Resp,
};

pub struct MgetCommand;

impl Command for MgetCommand {
    fn name(&self) -> &'static str {
        "mget"
    }

    fn execute(&self, args: &[Resp], ctx: &mut CommandContext) {
        let mut results = Vec::new();

        let mut expiry_manager = ctx.state.expiry_manager.lock().unwrap();
        let mut memory = ctx.state.memory.lock().unwrap();

        for arg in args {
            match parse_bulk_string_arg(arg) {
                Ok(key) => {
                    // Check if the key is expired
                    if expiry_manager.is_expired(&key) {
                        expiry_manager.remove_expiry(&key);
                        memory.delete(&key);
                        send_resp(&mut ctx.stream, Resp::BulkString(None));
                        return;
                    }

                    // Retrieve the value if it exists
                    if let Some(value) = memory.get(&key) {
                        results.push(Resp::BulkString(Some(value)));
                    } else {
                        results.push(Resp::BulkString(None));
                    }
                }
                Err(_) => {
                    send_error(&mut ctx.stream, errors::INVALID_GET_KEY);
                    return;
                }
            }
        }

        send_resp(&mut ctx.stream, Resp::Array(results));
    }
}
