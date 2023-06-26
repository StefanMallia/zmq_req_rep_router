pub struct MessageRouter
{
  socket: zmq::Socket
}

impl MessageRouter
{
    pub fn new(connection_string: &str) -> MessageRouter
    {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::ROUTER).unwrap();
        socket.set_router_mandatory(true).unwrap();
        socket.set_router_handover(true).unwrap();
        socket.bind(connection_string).unwrap();
        rust_log::info!("MessageRouter listening on: {}", connection_string);
        MessageRouter{socket}
    }

    pub fn route_messages(&self)
    {
        loop
        {
            let mut message: Vec<Vec<u8>> = self.socket.recv_multipart(0).unwrap();

            let source_address = message[0].clone();
            let dest_address = message[2].clone();

            message[0] = dest_address;
            message[2] = source_address;

            let result = self.socket.send_multipart(&message, 0);
            match result
            {
                Ok(res) => res,
                Err(err) =>
                {
                    let destination = message[0].clone();
                    message[0] = message[2].clone();
                    message[2] = destination;
                    message[4] = err.to_string().as_bytes().to_vec();
                    let result = self.socket.send_multipart(&message, 0);
                    match result
                    {
                        Ok(()) => {},
                        Err(err) => {rust_log::warn!("Failed to send message: {}.", err);}
                    }
                }
            }
        }    
    }
}
