use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "Redis server command-line interface")]
pub struct RedisServerOptions {
    #[argh(
        option,
        default = "\"6379\".to_string()",
        description = "the port number to listen to"
    )]
    pub port: String,

    #[argh(
        option,
        description = "makes the node a slave node, and set the url of the master to the specified url"
    )]
    pub replicaof: Option<String>,
}
