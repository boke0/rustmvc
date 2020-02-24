extern crate sha1;
extern crate base64;
extern crate reftree;
#[macro_use] extern crate codegen;
mod request;
mod requests;
mod response;
mod uri;
mod value;
mod observer;
mod server;
mod client;
mod dataframe;
pub use request::Request;
pub use response::Response;
pub use observer::Observer;
pub use server::{Server,RequestHandler};
pub use client::Client;
pub use dataframe::DataFrame;

trait Migrator{}

struct DefaultRequestHandler;
impl DefaultRequestHandler {
    pub fn new() -> DefaultRequestHandler {
        DefaultRequestHandler
    }
}
impl RequestHandler for DefaultRequestHandler {
    fn handle(&self, req: Request) -> Response {
        let mut res = Response::new(200, String::from("1.1"));
        res
    }
}
pub fn run(port: u16){
    let mut server = Server::new(port);
    let handler = DefaultRequestHandler::new();
    server.run(handler);
}

struct User;
#[migrator]
impl User {
    fn get(req: Request) -> Response{
        let mut res = Response::new(205,String::from("1.1"));
    }
    fn post(req:Request) -> Response{
        let mut res = Response::new(300,String::from("1.1"));
    }
}
#[test]
fn migrate_test() {
    let req = Request::new();
    let user = User;
    assert_eq!(user.call("get",req), Response::new(205,String::from("1.1")));
}
