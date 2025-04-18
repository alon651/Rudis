use crate::{
    commands::command::{send_error, send_resp, Command, CommandContext},
    resp::Resp,
};

pub struct PingCommand;

impl Command for PingCommand {
    fn name(&self) -> &'static str {
        "PING"
    }

    fn execute(&self, args: &[Resp], ctx: &mut CommandContext) {
        // PING should have no additional arguments
        if args.is_empty() {
            send_resp(&mut ctx.stream, Resp::SimpleString("PONG".to_owned()));
        } else {
            send_error(&mut ctx.stream, "PING takes no arguments");
        }
    }
}
