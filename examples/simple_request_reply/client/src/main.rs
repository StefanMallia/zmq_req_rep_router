use req_client::RequestClient;

#[tokio::main]
pub async fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");

    let request_client = RequestClient::new("Client", &config_loader.get_value("message_broker_address").unwrap());

    loop
    {
        println!("Sending message: '{}'", "Test request");
        let reply = request_client.send_request("Server", "Test request").await.unwrap(); 
        println!("Received reply: '{}'", reply);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
