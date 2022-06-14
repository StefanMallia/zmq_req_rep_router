use std::sync::Arc;

pub struct MessageRouter
{
  socket: Arc<futures::lock::Mutex<zmq::Socket>>
}

impl MessageRouter
{
    pub fn new(connection_string: &str) -> MessageRouter
    {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::ROUTER).unwrap();
        socket.set_router_mandatory(true);
        socket.set_router_handover(true);
        socket.bind(connection_string).unwrap();
        let socket = Arc::new(futures::lock::Mutex::new(socket));
        MessageRouter{socket}
    }

    pub async fn route_messages(&self)
    {
        loop
        {
            let socket = self.socket.lock().await;
            let mut message: Vec<Vec<u8>> = socket.recv_multipart(0).unwrap();

            let source_address = message[0].clone();
            let dest_address = message[2].clone();

            message[0] = dest_address;
            message[2] = source_address;

            

            let result = socket.send_multipart(&message, 0);
            match result
            {
                Ok(res) => res,
                Err(err) =>
                {
                    let destination = message[0].clone();
                    message[0] = message[2].clone();
                    message[2] = destination;
                    message[4] = err.to_string().as_bytes().to_vec();
                    socket.send_multipart(&message, 0);

                }
            }
        }    
    }
}
