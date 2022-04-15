use zmq_message_router::MessageRouter;

#[tokio::main]
pub async fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");

    let message_router = MessageRouter::new(&config_loader.get_value("message_router_address").unwrap());

    message_router.route_messages().await;
}
