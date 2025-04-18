use super::generate_rep_id;

#[derive(Clone)]
pub enum Role {
    Master(MasterProperties),
    Slave(String),
}

#[derive(Clone)]
pub struct MasterProperties {
    pub replid: String,
    pub repl_offset: i32,
}

impl Role {
    pub fn new(role_str: Option<String>) -> Role {
        match role_str {
            Some(master_url) => Role::Slave(master_url.replace(' ', ":")),
            None => Role::Master(MasterProperties {
                replid: generate_rep_id(),
                repl_offset: 0,
            }),
        }
    }
}
