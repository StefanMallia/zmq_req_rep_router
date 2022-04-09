use std::sync::Arc;

pub struct ReplyServer
{
    socket: Arc<futures::lock::Mutex<zmq::Socket>>,
}



impl ReplyServer
{
    pub fn new(identity:&str, connection_string: &str) -> ReplyServer
    {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::DEALER).unwrap();
        socket.set_identity(&identity.as_bytes()).unwrap();
        socket.connect(connection_string).unwrap();
        let socket = Arc::new(futures::lock::Mutex::new(socket));
        ReplyServer{socket}
    }
}

trait RequestProcessor
{
    fn receive_request(&self, socket: &zmq::Socket)    {
        let message_multi: Vec<Vec<u8>> = socket.recv_multipart(0).unwrap();
        let requester = zmq::Message::from(&message_multi[1]);
        let request_msg = zmq::Message::from(&message_multi[3]);        

        let response_msg = self.process_message(request_msg.as_str().unwrap());

        let response: Vec<String> = 
          vec![//String::from_utf8_lossy(&message[1]).to_string(),
               "".to_string(),
               String::from_utf8_lossy(&requester).to_string(),
               "".to_string(),
               response_msg];
    
        socket.send_multipart(&response, 0).unwrap();
    }

    fn process_message(&self, message: &str) -> String;
}

