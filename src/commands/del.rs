use crate::{
    commands::command::{
        errors, parse_bulk_string_arg, send_error, send_resp, Command, CommandContext,
    },
    resp::Resp,
};

pub struct DelCommand;

impl Command for DelCommand {
    fn name(&self) -> &'static str {
        "DEL"
    }

    fn execute(&self, args: &[Resp], ctx: &mut CommandContext) {
        if args.is_empty() {
            send_error(&mut ctx.stream, "DEL requires at least one argument");
            return;
        }

        let mut deleted_count = 0;

        for arg in args {
            match parse_bulk_string_arg(arg) {
                Ok(key) => {
                    let mut memory = ctx.state.memory.lock().unwrap();
                    if memory.delete(&key).is_some() {
                        deleted_count += 1;
                    }
                    ctx.state.expiry_manager.lock().unwrap().remove_expiry(&key);
                }
                Err(_) => {
                    send_error(&mut ctx.stream, errors::INVALID_GET_KEY);
                    return;
                }
            }
        }

        send_resp(&mut ctx.stream, Resp::Integer(deleted_count));
    }
}
