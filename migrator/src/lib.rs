extern crate quote;
use quote::quote;

mod test;

macro_rules! mig {
    ($st: block) => {
        let tokens = quote! {
            $st
        }
        println!("{:.}",tokens);
    }
}
