use rep_server::{ProcessRequest, ReplyServer};

struct RequestProcessor {}

impl ProcessRequest for RequestProcessor
{
    fn process_message(&self, message: &str) -> String
    {
        println!("Received message: '{}'", message);
        println!("Sending message response: '{}'", "Message response");
        "Message response".to_string()
    }
}

pub fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");

    let request_processor = RequestProcessor{};
    let reply_server = ReplyServer::new("Server", request_processor,
                                        &config_loader.get_string("message_router_address").unwrap());

    reply_server.receive_requests();
}
