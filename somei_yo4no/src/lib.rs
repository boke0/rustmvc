extern crate http;
#[allow(unused_imports)] #[macro_use] pub extern crate codegen;
mod dataframe;
pub use std::collections::HashMap;
pub use http::*;
pub use dataframe::{DataFrame, Value};
pub use codegen::*;

pub trait Callable{
    fn call(&self,s: String, df: DataFrame) -> Response;
}

#[derive(Debug)]
pub struct MapErr{
    pub code: u8,
    pub result: DataFrame
}

pub struct ObjectMap {
    pub map: HashMap<String, Box<dyn Callable>>,
    pub port: u16
}

impl ObjectMap {
    pub fn new(port: u16) -> ObjectMap {
        ObjectMap {
            map: HashMap::new(),
            port: port
        }
    }
    pub fn add(&mut self, k: &str, v: Box<dyn Callable>) {
        self.map.insert(String::from(k), v);
    }
    pub fn run(&mut self) {
        let mut server = Server::new(self.port);
        server.run(self);
    }
}

impl RequestHandler for ObjectMap {
    fn handle(&self, req: Request) -> Response {
        let path: Vec<&str> = req.uri.path.split("/").collect();
        let path_f = path[1];
        if path_f == "objmap_ep" {
            match self.map.get(path[2]) {
                Some(obj) => {
                    match DataFrame::from(req.body.as_slice()) {
                        Ok(df) => {
                            obj.call(path[3].to_string(),df)
                        },
                        Err(_) => {
                            Response::new(422, "1.1".to_string())
                        }
                    }
                },
                None => {
                    Response::new(404,String::from("1.1"))
                }
            }
        }else{
            Response::new(200,String::from("1.1"))
        }
    }
}

pub fn run(port: u16){
    let mut default = ObjectMap::new(port);
    default.run();
}

