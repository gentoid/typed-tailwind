use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ToSeedClass)]
pub fn to_seed_class_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_to_seed_class(&ast)
}

fn impl_to_seed_class(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ToClasses for #name {
            fn to_classes(self) -> Option<Vec<String>> {
                Some(vec![String::from(&self)])
            }
        }
    };
    gen.into()
}
