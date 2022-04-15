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

            socket.send_multipart(message, 0).unwrap();
        }    
    }
}
