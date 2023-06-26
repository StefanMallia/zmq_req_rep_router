pub struct RequestClient
{
    socket: zmq::Socket,
}

impl RequestClient
{
    pub fn new(identity:&str, connection_string: &str) -> RequestClient
    {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::REQ).unwrap();
        socket.set_identity(&identity.as_bytes()).unwrap();
        socket.connect(connection_string).unwrap();
        RequestClient{socket}
    }

    pub fn send_request(&self, destination: &str, data: &str) -> Result<String, String>
    {
        return self.send_request_bytes(destination, data.as_bytes());
    }

    pub fn send_request_bytes(&self, destination: &str, data: &[u8]) -> Result<String, String>
    {
        let mut responder = zmq::Message::new();
        let mut response = zmq::Message::new();

        self.socket.send(&destination, zmq::SNDMORE).unwrap();
        self.socket.send("", zmq::SNDMORE).unwrap();
        self.socket.send(&data, 0).unwrap();

        self.socket.recv(&mut responder, zmq::SNDMORE).unwrap();
        self.socket.recv(&mut response, zmq::SNDMORE).unwrap();
        if response.as_str().unwrap()==""
            && responder.as_str().unwrap() == destination
        {
            self.socket.recv(&mut response, 0).unwrap();
            return Ok(response.as_str().unwrap().to_string())
        }
        return Err(format!("Incorrect response returned: {}", response.as_str().unwrap()))         
    }
}
