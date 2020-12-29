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
    let generics = &ast.generics;
    let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics ToClasses for #name #ty_generics {
            fn to_classes(self) -> Option<Vec<String>> {
                Some(vec![String::from(&self)])
            }
        }
    };
    gen.into()
}
