use rep_server::{ProcessRequest, ReplyServer};

struct RequestProcessor
{
}

impl ProcessRequest for RequestProcessor
{
    fn process_message(&self, message: &str) -> String
    {
        println!("Received message: '{}'", message);
        println!("Sending message response: '{}'", "Message response");
        "Message response".to_string()
    }
}

fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");

    let reply_server = ReplyServer::new("Server", &config_loader.get_value("message_broker_address").unwrap());
    let request_processor = RequestProcessor{};

    loop
    {
        request_processor.receive_request(&reply_server);
    }
}
