use crate::{
    commands::{command::send_resp, send_error},
    resp::Resp,
    utils::role::Role,
};

use super::Command;

pub struct InfoCommand;

impl Command for InfoCommand {
    fn name(&self) -> &'static str {
        "INFO"
    }

    fn execute(&self, args: &[crate::resp::Resp], ctx: &mut super::CommandContext) {
        if args.len() != 1 {
            send_error(&mut ctx.stream, "info can have only one argument");
        }

        let role_str = match ctx.server_meta_data.role {
            crate::utils::role::Role::Master(_) => "master",
            crate::utils::role::Role::Slave(_) => "slave",
        };

        let mut info_string = format!("# Replication\nrole:{}\n", role_str).to_string();

        match &ctx.server_meta_data.role {
            Role::Master(master_properties) => {
                info_string.push_str(&format!(
                    "master_replid:{}\nmaster_repl_offset:{}\n",
                    master_properties.replid, master_properties.repl_offset
                ));
            }
            Role::Slave(_) => {}
        }

        send_resp(&mut ctx.stream, Resp::BulkString(Some(info_string)));
    }
}
