use std::net::TcpListener;
use std::io::prelude::*;
use super::Request;
use super::Response;

pub trait RequestHandler {
    fn handle(&self, req: Request) -> Response;
}
pub struct Server {
    port: u16,
}
impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            port: port
        }
    }
    pub fn run<T: RequestHandler>(&mut self, handler: T) {
        let l = TcpListener::bind(
            format!("127.0.0.1:{}",self.port).as_str()
        ).unwrap();
        for s in l.incoming() {
            println!("expecting...");
            let mut s = s.unwrap();
            let mut buf = [0; 2*1024*1024];
            loop {
                s.read(&mut buf).unwrap(); 
                let req = Request::from(buf.to_vec());
                let mut res: Response = handler.handle(req);
                s.write(res.as_bytes()).unwrap();
                buf = [0; 2*1024*1024];
            }
        }
    }
}
