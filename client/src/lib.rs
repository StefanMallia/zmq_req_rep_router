use std::sync::Arc;

pub struct RequestClient
{
    socket: Arc<futures::lock::Mutex<zmq::Socket>>,
}

impl RequestClient
{
    pub fn new(identity:&str, connection_string: &str) -> RequestClient
    {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::REQ).unwrap();
        socket.set_identity(&identity.as_bytes()).unwrap();
        socket.connect(connection_string).unwrap();
        let socket = Arc::new(futures::lock::Mutex::new(socket));
        RequestClient{socket}
    }

    pub async fn send_request(&self, destination: &str, data: &str) -> Result<String, String>
    {
        let mut responder = zmq::Message::new();
        let mut response = zmq::Message::new();

        let socket = self.socket.lock().await;
        
        socket.send(&destination, zmq::SNDMORE).unwrap();
        socket.send("", zmq::SNDMORE).unwrap();
        socket.send(&data, 0).unwrap();

        socket.recv(&mut responder, zmq::SNDMORE).unwrap();
        socket.recv(&mut response, zmq::SNDMORE).unwrap();
        if response.as_str().unwrap()==""
            && responder.as_str().unwrap() == destination
        {
            socket.recv(&mut response, 0).unwrap();
            return Ok(response.as_str().unwrap().to_string())
        }
        return Err(format!("Incorrect response returned: {}", response.as_str().unwrap()))         
    }
}
