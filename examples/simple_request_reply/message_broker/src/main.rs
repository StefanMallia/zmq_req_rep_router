use zmq_message_broker::MessageBroker;

#[tokio::main]
pub async fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");

    let message_broker = MessageBroker::new(&config_loader.get_value("message_broker_address").unwrap());

    message_broker.route_messages().await;
}
