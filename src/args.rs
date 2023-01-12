use std::env::args;

pub struct Args {
    pub recipient_id: String,
    pub web_driver_port: String,
}

pub fn get_args() -> Args {
    let args: Vec<String> = args().collect();

    let web_driver_port = match args.get(1) {
        Some(id) => id.to_string(),
        None => "4444".to_string(),
    };

    let recipient_id = match args.get(2) {
        Some(id) => id.to_string(),
        None => "100004518426760".to_string(),
    };

    return Args {
        recipient_id,
        web_driver_port,
    };
}
