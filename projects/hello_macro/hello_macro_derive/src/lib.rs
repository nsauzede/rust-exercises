extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

fn fmt_ast(ast : &syn::DeriveInput) -> String {
    format!("{} attrs, ident={}", ast.attrs.len(), ast.ident)
//    + format!("data={:?}", match ast.data {syn::DataStruct{} => "DataStruct"})
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    println!("Wow, seen this ast : '{}'", fmt_ast(&ast));
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Yay proc_macro_attr attr={} item={}", attr.to_string(), item.to_string());
    let (a, i) = (attr.to_string(), item.to_string());
    let gen = quote! {
//        println!("HELLO");
        fn hello() { println!("hello quote attr={:#?} item={:#?}", #a, #i); }
//        #i
    };
    gen.into()
//    item
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let i = input.to_string();
    let gen = quote! {
//        println!("HELLO");
//        fn hello() { println!("hello quote attr={:#?} item={:#?}", #a, #i); }
//        #i
        format!("input={:#?}", #i)
    };
    gen.into()
}
