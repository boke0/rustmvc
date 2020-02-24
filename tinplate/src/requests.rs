use super::request::{Request, Method};
use super::uri::Uri;

enum State {
    Method,
    Uri,
    Version,
    HeaderKey,
    HeaderValue
}
pub struct Requests {
    vec: Vec<Request>,
}
impl Requests {
    pub fn new() -> Requests {
        Requests {
            vec: Vec::new()
        }
    }
    pub fn from(base: Vec<u8>) -> Requests {
        let mut reqs = Requests::new();
        let mut state = State::Method;
        let mut key_buf = Vec::new();
        let mut val_buf = Vec::new();
        let mut body_buf = Vec::new();
        let mut req_buf = Request::new();
        let mut base_iter = base.into_iter();
        'outer: loop {
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
                                    req_buf.method = Method::from(String::from_utf8_lossy(&key_buf).into_owned());
                                    state = State::Uri;
                                    key_buf = Vec::new();
                                },
                                State::Uri => {
                                    req_buf.uri = Uri::from(String::from_utf8_lossy(&key_buf).into_owned());
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
                                    let len = req_buf.headers
                                        .get("Content-Length")
                                        .unwrap()
                                        .parse()
                                        .unwrap();
                                    for i in 0..len {
                                        match base_iter.next() {
                                            Some(s) => {
                                                body_buf.push(s);
                                            }
                                            None => {
                                                req_buf.body = body_buf;
                                                println!("Ended");
                                                break 'outer;
                                            }
                                        }
                                    }
                                    req_buf.body = body_buf;
                                    let con = req_buf.get_header("Connection");
                                    match con {
                                        Some(con) => {
                                            println!("Connection is:{}",con);
                                            reqs.push(req_buf);
                                            if con == "keep-alive" {
                                                req_buf = Request::new();
                                                body_buf = Vec::new();
                                                state = State::Method;
                                            }else{
                                                println!("Ended");
                                                break;
                                            }
                                        }
                                        None => {
                                            reqs.push(req_buf);
                                            println!("Ended");
                                            break;
                                        },
                                    };
                                },
                                State::HeaderValue => {
                                    let k = String::from_utf8_lossy(&key_buf).into_owned();
                                    let v = String::from_utf8_lossy(&val_buf).into_owned();
                                    req_buf.headers.insert(k,v);
                                    key_buf=Vec::new();
                                    val_buf=Vec::new();
                                    state = State::HeaderKey;
                                },
                                State::Version => {
                                    req_buf.version = String::from_utf8_lossy(&key_buf).into_owned();
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
        reqs
    }
    pub fn push(&mut self, req: Request) {
        self.vec.push(req);
    }
    pub fn as_slice(&self) -> &[Request] {
        self.vec.as_slice()
    }
}

