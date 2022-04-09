pub struct MessageBroker
{
    socket: zmq::Socket
}

impl MessageBroker
{
    pub fn new(connection_string: &str) -> MessageBroker
    {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::ROUTER).unwrap();
        socket.bind(connection_string).unwrap();
        MessageBroker{socket}
    }

    pub async fn route_messages(&self)
    {
        loop
        {
            let mut message: Vec<Vec<u8>> = self.socket.recv_multipart(0).unwrap();

            let source_address = message[0].clone();
            let dest_address = message[2].clone();

            message[0] = dest_address;
            message[2] = source_address;

            self.socket.send_multipart(message, 0).unwrap();
        }    
    }
}
