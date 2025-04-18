use crate::{
    commands::command::{
        errors, parse_bulk_string_arg, parse_optional_flags, send_error, send_ok, Command,
        CommandContext,
    },
    resp::Resp,
};

pub struct SetCommand;

impl Command for SetCommand {
    fn name(&self) -> &'static str {
        "SET"
    }

    fn execute(&self, args: &[Resp], ctx: &mut CommandContext) {
        if args.len() < 2 {
            send_error(
                &mut ctx.stream,
                "SET requires at least key and value arguments",
            );
            return;
        }

        match (
            parse_bulk_string_arg(&args[0]),
            parse_bulk_string_arg(&args[1]),
        ) {
            (Ok(key), Ok(value)) => {
                // Parse optional flags (e.g., PX, EX)
                let flags = match parse_optional_flags(&args[2..]) {
                    Ok(flags) => flags,
                    Err(err) => {
                        send_error(&mut ctx.stream, err);
                        return;
                    }
                };

                let px = flags.get("PX").cloned();
                let ex = flags.get("EX").map(|&s| s * 1000);

                // Set the key-value pair in memory
                {
                    let mut memory = ctx.state.memory.lock().unwrap();
                    memory.set(key.clone(), value.clone());
                }

                // Set the expiration if provided
                if let Some(expiry) = px.or(ex) {
                    let mut expiry_manager = ctx.state.expiry_manager.lock().unwrap();
                    expiry_manager.set_expiry(&key, expiry);
                }

                send_ok(&mut ctx.stream);
            }
            _ => send_error(&mut ctx.stream, errors::INVALID_SET_KEY),
        }
    }
}
