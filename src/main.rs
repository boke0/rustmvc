#[macro_use] extern crate somei_yo4no;
use somei_yo4no::*;

struct User;
#[map_logics]
impl User {
    fn get(&self, id: u8) -> Result<DataFrame, MapErr>{
        println!("get");
        let mut df = DataFrame::new();
        df.push(Value::from_u8(10));
        Ok(df)
    }
}

fn main() {
    run(8080);
}
