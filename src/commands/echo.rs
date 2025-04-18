use crate::{
    commands::command::{send_error, send_resp, Command, CommandContext},
    resp::Resp,
};

pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &'static str {
        "ECHO"
    }

    fn execute(&self, args: &[Resp], ctx: &mut CommandContext) {
        if args.len() == 1 {
            if let Resp::BulkString(Some(message)) = &args[0] {
                send_resp(&mut ctx.stream, Resp::BulkString(Some(message.clone())));
            } else {
                send_error(&mut ctx.stream, "invalid ECHO argument");
            }
        } else {
            send_error(&mut ctx.stream, "ECHO takes exactly one argument");
        }
    }
}
