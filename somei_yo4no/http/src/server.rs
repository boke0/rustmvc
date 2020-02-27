use std::net::TcpListener;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::time::Duration;
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
    pub fn run<T: RequestHandler>(&mut self, handler: &T) {
        let l = TcpListener::bind(
            format!("127.0.0.1:{}",self.port).as_str()
        ).unwrap();
        for s in l.incoming() {
            let s = s.unwrap();
            match s.set_read_timeout(Some(Duration::from_secs(15))) {
                Ok(_) => {
                    let mut buf_r = BufReader::new(&s);
                    let mut buf_w = BufWriter::new(&s);
                    let mut buf = [0; 2*1024*1024];
                    buf_r.read(&mut buf).unwrap();
                    let req = Request::from(buf.to_vec());
                    let mut res: Response = handler.handle(req);
                    let bytes = res.as_bytes();
                    buf_w.write(bytes).unwrap();
                },
                Err(_) => {
                    continue;
                }
            }
        }
    }
}
