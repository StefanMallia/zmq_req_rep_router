use zmq_message_router::MessageRouter;

pub fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");

    let message_router = MessageRouter::new(&config_loader.get_string("message_router_address").unwrap());

    message_router.route_messages();
}
