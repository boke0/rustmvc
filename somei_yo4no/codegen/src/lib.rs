extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate http;

use crate::proc_macro::TokenStream;
use crate::proc_macro2::{TokenStream as TokenStream_, Punct, Span, Spacing, Literal, Group, Delimiter};
use quote::{quote, TokenStreamExt};
use syn::{parse, ItemImpl, Ident, FnArg};

#[proc_macro_attribute]
pub fn map_logics(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: ItemImpl = parse(item).unwrap();
    let item_ = item.clone();
    let obj_name;
    if let syn::Type::Path(p) = *item.self_ty {
        let p_ = p.clone();
        obj_name = p_.path
            .get_ident()
            .unwrap()
            .clone();
    }else{
        panic!("Invalid type of Impl block!");
    }
    let mut arms = TokenStream_::new();
    for method in &item.items {
        if let syn::ImplItem::Method(m) = method {
            let ident = m.sig.ident.clone();
            let name = ident
                .to_string();
            let name_ = name.as_str();
            let mut attributes = TokenStream_::new();
            let mut i = 0;
            dbg!(m.sig.inputs.clone());
            for attr in &m.sig.inputs {
                let arg_type: String;
                match attr {
                    FnArg::Receiver(_) => {
                        continue;
                    },
                    FnArg::Typed(t) => {
                        let t_ = t.clone();
                        match *t_.ty {
                            syn::Type::Path(p) => {
                                arg_type = p.clone().path
                                    .get_ident()
                                    .unwrap()
                                    .to_string();
                            },
                            _ => {
                                panic!("Invalid type of ImplItem");
                            }
                        }
                    }
                }
                attributes.append(Ident::new("df", Span::call_site()));
                attributes.append(Punct::new('.', Spacing::Alone));
                attributes.append(Ident::new("get", Span::call_site()));
                let mut token = TokenStream_::new();
                token.append(Literal::usize_suffixed(i));
                let arg_group = Group::new(
                        Delimiter::Parenthesis,
                        token
                    );
                attributes.append(arg_group);
                i+=1;
                attributes.append(Punct::new('.', Spacing::Alone));
                match arg_type.as_str() {
                    "u8" | "u16" | "u32" | "u64" | 
                    "i8" | "i16" | "i32" | "i64" |
                    "f32" | "f64" | "String" => {
                        let ident_str = format!("as_{}",arg_type);
                        attributes.append(
                            Ident::new(
                                ident_str.as_str(),
                                Span::call_site()
                            )
                        );
                    },
                    &_ => {
                        panic!("Invalid type of ImplItem");
                    }
                }
                let token_ = TokenStream_::new();
                let arg_group_ = Group::new(
                        Delimiter::Parenthesis,
                        token_
                    );
                attributes.append(arg_group_.clone());
                attributes.append(Punct::new('.',Spacing::Alone));
                attributes.append(Ident::new("unwrap",Span::call_site()));
                attributes.append(arg_group_.clone());
            }
            let token = quote!{
                #name_ => {
                    match self.#ident( #attributes ) {
                        Ok(mut result) => {
                            result.as_bytes().to_vec()
                        },
                        Err(mut err) => {
                            status = err.code;
                            err.result.as_bytes().to_vec()
                        }
                    }
                },
            };
            arms.extend(vec![token]);
        }
    }
    let call_method = quote! {
        #item_
        impl somei_yo4no::Callable for #obj_name {
            fn call(&self, s: String, df: DataFrame) -> somei_yo4no::Response{
                let mut status = 200;
                let body = match s.as_str() {
                    #arms
                    &_ => {
                        status = 404;
                        Vec::new()
                    }
                };
                let mut res = Response::new(status as u16, "1.1".to_string());
                res.body = body;
                res
            }
        }
    };
    let item: TokenStream = dbg!(call_method.into());
    item
}
