use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
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

#[proc_macro_attribute]
pub fn hello_macro_attr(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    assert!(input.is_empty()); // We dont want anything

    println!("Where does this go?");

    let item: syn::Item = syn::parse(annotated_item).unwrap();
    let mut ast: syn::ItemFn = match item {
        syn::Item::Fn(x) => x,
        _ => panic!("Not a fn!"),
    };

    let hello: syn::Stmt = syn::parse(
        quote! {
            println!("Hello from hello_macro attribute!");
        }
        .into(),
    )
    .unwrap();
    ast.block.stmts.insert(0, hello.into());

    let bye: syn::Stmt = syn::parse(
        quote! {
            println!("Goodbye from hello_macro attribute!");
        }
        .into(),
    )
    .unwrap();
    ast.block.stmts.push(bye.into());

    use quote::ToTokens;
    ast.into_token_stream().into()
}
