pub const PONG: &str = "PONG";

pub const SYNC_PORT: &str = "8888";

pub fn get_port(args: Vec<String>) -> String {
    let server_port = match args.get(1) {
        None => SYNC_PORT.to_string(),
        Some(port) => port.to_string(),
    };

    server_port
}
