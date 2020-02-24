extern crate sha1;
extern crate base64;
mod request;
mod response;
mod uri;
mod reftree;
mod value;
mod observer;
mod client;
mod dataframe;
use std::net::TcpListener;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use request::Request;
use response::Response;
use observer::Observer;
use client::Client;
use dataframe::DataFrame;
use sha1::Sha1;

pub struct Server {
    port: i16,
}
impl Server {
    pub fn new(port: i16) -> Server {
        Server {
            port: port
        }
    }
    pub fn run(&mut self) {
        let l = TcpListener::bind(
            format!("127.0.0.1:{}",self.port).as_str()
        ).unwrap();
        for s in l.incoming() {
            println!("expecting...");
            let mut s = s.unwrap();
            let mut buf = [0; 2*1024*1024];
            s.read(&mut buf).unwrap(); 
            let req_str = String::from_utf8_lossy(&mut buf).into_owned();
            let mut res: Response = Response::new(200, "1.1".to_string());
            match Request::from(req_str) {
                Ok(req) => {
                    if let Option::Some(val) = req.headers.get("Connection") {
                        //必要であればkeep-aliveについてかく
                    }
                    let (mime, bytes) = self.get_static(req.uri.path);
                    res.headers.insert("Content-Type".to_string(),mime);
                    res.body=bytes;        
                },
                Err(code) => res.status = code
            };
            s.write(res.to_vec().as_slice()).unwrap();
        }
    }
    pub fn get_static(&self, path: String) -> (String, Vec<u8>){
        let mut path_buf=Path::new("./static").to_path_buf();
        path_buf=path_buf.join(path);
        let path = path_buf.as_path();
        let mut buf=Vec::new();
        let mime: String;
        let mut file = if path.is_file() {
            mime = match path.extension() {
                Some(ext) => {
                    ext.to_str()
                        .unwrap()
                        .to_string()
                },
                None => "text/plain".to_string()
            };
            BufReader::new(File::open(path).unwrap())
        }else{
            mime = "text/html".to_string();
            BufReader::new(File::open("./static/index.html").unwrap())
        };
        file.read_to_end(&mut buf).unwrap();
        (mime, buf)
    }
}



