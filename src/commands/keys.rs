use crate::{
    commands::command::{send_error, send_resp, Command, CommandContext},
    resp::Resp,
};
use glob::Pattern;

pub struct KeysCommand;

impl Command for KeysCommand {
    fn name(&self) -> &'static str {
        "KEYS"
    }


    fn execute(&self, args: &[Resp], ctx: &mut CommandContext) {
        if args.len() != 1 {
            send_error(&mut ctx.stream, "KEYS requires exactly one argument");
            return;
        }

        let pattern = match &args[0] {
            Resp::BulkString(Some(p)) => p,
            _ => {
                send_error(&mut ctx.stream, "invalid pattern format");
                return;
            }
        };

        let memory = ctx.state.memory.lock().unwrap();
        let keys: Vec<String> = memory
            .data
            .keys()
            .filter(|key| Pattern::new(pattern).map_or(false, |p| p.matches(key)))
            .cloned()
            .collect();

        let response = Resp::Array(
            keys.into_iter()
                .map(|k| Resp::BulkString(Some(k)))
                .collect(),
        );
        send_resp(&mut ctx.stream, response);
    }
}
