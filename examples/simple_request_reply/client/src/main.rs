use req_client::RequestClient;

pub fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");

    let request_client = RequestClient::new("Client", &config_loader.get_string("message_router_address").unwrap());

    loop
    {
        println!("Sending message: '{}'", "Test request");
        let reply = request_client.send_request("Server", "Test request").unwrap(); 
        println!("Received reply: '{}'", reply);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
