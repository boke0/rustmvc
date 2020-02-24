extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use crate::proc_macro::TokenStream;
use crate::proc_macro2::TokenStream as TokenStream_;
use quote::quote;
use syn::{parse, ItemImpl};

#[proc_macro_attribute]
pub fn migrator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ = item.clone();
    let item: ItemImpl=parse(item).unwrap();
    let mut obj_name: String;
    if let syn::Type::Path(p) = *item.self_ty {
        obj_name = p.path
            .get_ident()
            .unwrap()
            .to_string();
    }else{
        panic!("Invalid type of Impl block!");
    }
    let mut arms = TokenStream_::new();
    for method in &item.items {
        if let syn::ImplItem::Method(m) = method {
            let ident=m.sig.ident.clone();
            let name=ident
                .to_string();
            let token = quote!{
                #name => self.#ident(req)
            };
            arms.extend(vec![token]);
        }
    }
    let call_method = quote! {
        impl #obj_name {
            fn call(&self, s: String, req: Request) -> Response{
                match s {
                    #arms
                }
            }
        }
    };
    let item = dbg!(call_method.into());
    item
}
