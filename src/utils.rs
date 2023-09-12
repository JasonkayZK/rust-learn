pub fn get_port(args: Vec<String>) -> String {
    let server_port = match args.get(1) {
        None => "8888".to_string(),
        Some(port) => {
            port.to_string()
        }
    };

    server_port
}
