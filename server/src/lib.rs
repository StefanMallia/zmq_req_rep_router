use std::sync::Arc;

pub struct ReplyServer<T>
where T: ProcessRequest
{
    socket: Arc::<futures::lock::Mutex<zmq::Socket>>,
    message_processor: T
}

pub trait ProcessRequest
{
    fn process_message(&self, message: &str) -> String;
}

impl<T: ProcessRequest> ReplyServer<T>
{
    pub fn new(identity:&str, message_processor: T, 
               connection_string: &str) -> ReplyServer<T>
    {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::DEALER).unwrap();
        socket.set_identity(&identity.as_bytes()).unwrap();
        socket.connect(connection_string).unwrap();
        let socket = Arc::new(futures::lock::Mutex::new(socket));
        ReplyServer{socket, message_processor}
    }

    pub async fn receive_requests(&self)
    {
        loop
        {
            let socket = self.socket.lock().await;
            let message_multi: Vec<Vec<u8>> = socket.recv_multipart(0).unwrap();
            let requester = zmq::Message::from(&message_multi[1]);
            let request_msg = zmq::Message::from(&message_multi[3]);

            let response_msg = self.message_processor.process_message(request_msg.as_str().unwrap());

            let response: Vec<String> = 
              vec!["".to_string(),
                   String::from_utf8_lossy(&requester).to_string(),
                   "".to_string(),
                   response_msg];
        
            socket.send_multipart(&response, 0).unwrap();
        }
    }
}
