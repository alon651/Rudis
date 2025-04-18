use crate::{
    commands::command::{
        errors, parse_bulk_string_arg, send_error, send_resp, Command, CommandContext,
    },
    resp::Resp,
};

pub struct GetCommand;

impl Command for GetCommand {
    fn name(&self) -> &'static str {
        "GET"
    }

    fn execute(&self, args: &[Resp], ctx: &mut CommandContext) {
        if args.len() != 1 {
            send_error(&mut ctx.stream, "GET takes exactly one argument");
            return;
        }

        match parse_bulk_string_arg(&args[0]) {
            Ok(key) => {
                let mut expiry_manager = ctx.state.expiry_manager.lock().unwrap();
                let mut memory = ctx.state.memory.lock().unwrap();

                // Check if the key is expired
                if expiry_manager.is_expired(&key) {
                    expiry_manager.remove_expiry(&key);
                    memory.delete(&key);
                    send_resp(&mut ctx.stream, Resp::BulkString(None));
                    return;
                }

                // Retrieve the value if it exists
                if let Some(value) = memory.get(&key) {
                    send_resp(&mut ctx.stream, Resp::BulkString(Some(value)));
                } else {
                    send_resp(&mut ctx.stream, Resp::BulkString(None));
                }
            }
            Err(_) => send_error(&mut ctx.stream, errors::INVALID_GET_KEY),
        }
    }
}
