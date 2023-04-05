use proc_macro2::TokenTree;
use quote::{quote, ToTokens};

pub fn parse_attr_core(
    attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let input_parsed: syn::ItemFn = syn::parse2(input).unwrap();
    let attr = get_raw_args(attr);

    let fn_name = input_parsed.sig.ident;

    let expanded = quote! {
        fn #fn_name (){
            println!("{:?}", vec![#(#attr),*])
        }
    };
    expanded.into()
}

fn get_raw_args(attr: proc_macro2::TokenStream) -> Vec<Box<dyn ToTokens>> {
    let mut attrs = attr.into_iter().collect::<Vec<TokenTree>>();
    let mut raw_args: Vec<Box<dyn ToTokens>> = Vec::new();
    while !attrs.is_empty() {
        match attrs.remove(0) {
            TokenTree::Ident(id) => {
                let name = id.to_string();
                raw_args.push(Box::new(name));
            }
            TokenTree::Literal(literal) => {
                let string_literal = literal.to_string();
                if !string_literal.starts_with('\"') || !string_literal.ends_with('\"') {
                    panic!("Expected a string literal, got '{}'", string_literal);
                }
                // Hacky way of getting a string without the enclosing quotes
                raw_args.push(Box::new(
                    string_literal[1..string_literal.len() - 1].to_string(),
                ));
            }
            x => {
                panic!("Expected either strings or literals as args, not {}", x);
            }
        }
        if !attrs.is_empty() {
            match attrs.remove(0) {
                TokenTree::Punct(p) if p.as_char() == ',' => {}
                x => {
                    panic!("Expected , between args, not {}", x);
                }
            }
        }
    }
    raw_args
}
