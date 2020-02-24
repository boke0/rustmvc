use super::uri::Uri;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

enum State {
    Method,
    Uri,
    Version,
    HeaderKey,
    HeaderValue
}
#[derive(Debug,PartialEq)]
pub enum Method{
    Post,
    Get,
    Put,
    Delete,
    Head,
    Options,
    Connect
}
impl Method {
    pub fn from(str: String) -> Method{
        match str.as_str() {
            "POST" => Method::Post,
            "GET" => Method::Get,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "OPTIONS" => Method::Options,
            "HEAD" => Method::Head,
            "CONNECT" => Method::Connect,
            &_ => Method::Get
        }
    }
    pub fn to_string(&self) -> String {
        String::from(match self {
            Method::Post => "POST",
            Method::Get => "GET",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Options => "OPTIONS",
            Method::Head => "HEAD",
            Method::Connect => "CONNECT"
        })
    }
}
#[derive(Debug,PartialEq)]
pub struct Request {
    pub base: Vec<u8>,
    pub version: String,
    pub method: Method,
    pub uri: Uri,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}
impl Request {
    pub fn new() -> Request {
        Request {
            base: Vec::new(),
            uri: Uri::from(String::from("")),
            method: Method::Get,
            version: String::from("1.1"),
            headers: HashMap::new(),
            body: Vec::new()
        }
    }
    pub fn set_header(&mut self, k: String, v: String) {
        self.headers.insert(k,v);
    }
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }
    pub fn set_uri(&mut self, uri: Uri) {
        self.uri = uri;
    }
    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }
    pub fn as_bytes(&mut self) -> &[u8] {
        let url_bytes = self.uri.to_string();
        let mut t = format!(
            "{} {} {}",
            self.method.to_string(),
            url_bytes.as_str(),
            self.version
        ).as_bytes().to_vec();
        self.base.append(&mut t);
        for (k,v) in &self.headers {
            let mut t=format!("{}: {}\r\n",k,v)
                    .as_bytes()
                    .to_vec();
            self.base.append(&mut t);
        }
        self.base.append(&mut self.body);
        self.base.as_slice()
    }
    pub fn get_header(&mut self, k: &str) -> Option<String> {
        match self.headers.get(k) {
            Some(str) => Some(String::from(str)),
            None => None
        }
    }
    pub fn from(base: Vec<u8>) -> Request {
        let mut state = State::Method;
        let mut key_buf = Vec::new();
        let mut val_buf = Vec::new();
        let mut body_buf = Vec::new();
        let mut req = Request::new();
        let mut base_iter = base.into_iter();
        loop {
            let d = base_iter.next();
            match d {
                Some(c) => {
                    match c as char {
                        '\0' => {
                            println!("Ended");
                            break;
                        },
                        ' ' => {
                            match state {
                                State::Method => {
                                    println!("Method is:{}",String::from_utf8_lossy(&key_buf));
                                    req.method = Method::from(String::from_utf8_lossy(&key_buf).into_owned());
                                    state = State::Uri;
                                    key_buf = Vec::new();
                                },
                                State::Uri => {
                                    req.uri = Uri::from(String::from_utf8_lossy(&key_buf).into_owned());
                                    state = State::Version;
                                    key_buf = Vec::new();
                                },
                                _ => {
                                    continue;
                                }
                            }
                        },
                        ':' => {
                            state=State::HeaderValue;
                        },
                        '\r' => {
                            continue;
                        },
                        '\n' => {
                            match state {
                                State::HeaderKey => {
                                    let len = req.headers
                                        .get("Content-Length")
                                        .unwrap()
                                        .parse()
                                        .unwrap();
                                    for _i in 0..len {
                                        match base_iter.next() {
                                            Some(s) => {
                                                body_buf.push(s);
                                            }
                                            None => {
                                                break;
                                            }
                                        }
                                    }
                                    req.set_body(body_buf);
                                    break;
                                },
                                State::HeaderValue => {
                                    let k = String::from_utf8_lossy(&key_buf).into_owned();
                                    let v = String::from_utf8_lossy(&val_buf).into_owned();
                                    req.headers.insert(k,v);
                                    key_buf=Vec::new();
                                    val_buf=Vec::new();
                                    state = State::HeaderKey;
                                },
                                State::Version => {
                                    req.version = String::from_utf8_lossy(&key_buf).into_owned();
                                    state = State::HeaderKey;
                                    key_buf = Vec::new();
                                },
                                _ => break
                            }
                        },
                        _ => {
                            match state {
                                State::HeaderKey => key_buf.push(c),
                                State::HeaderValue => val_buf.push(c),
                                State::Version=> key_buf.push(c),
                                State::Method => key_buf.push(c),
                                State::Uri => key_buf.push(c),
                            };
                        }
                    }
                },
                None => break
            }
        }
        req
    }
}

#[test]
fn request_parse_test() {
    let mut f = File::open("./test/requests").unwrap();
    let mut buf = [0; 2*1024];
    f.read(&mut buf).unwrap();
    let req = Request::from(buf.to_vec());
    let mut req_ = Request::new();
    req_.set_method(Method::Post);
    req_.set_uri(Uri::from(String::from("/hello/world?i=m#testing")));
    req_.set_version(String::from("1.1"));
    req_.set_header(String::from("Content-Length"), String::from("12"));
    req_.set_header(String::from("Content-Type"), String::from("text/plain"));
    req_.set_header(String::from("Connection"), String::from("keep-alive"));
    req_.set_body("hello world!".as_bytes().to_vec());
    assert_eq!(req,req_);
}
