use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ScreenSize)]
pub fn screen_size_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_screen_size(&ast)
}

fn impl_screen_size(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ScreenSizeTrait for #name {
            fn screen(self, screen: Screen) -> ScreenSize {
                ScreenSize(screen, Box::new(self))
            }

            fn to_string(&self) -> String {
                String::from(self)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HoverState)]
pub fn hover_state_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hover_state(&ast)
}

fn impl_hover_state(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HoverStateTrait for #name {
            fn hover(self) -> HoverState {
                HoverState(Box::new(self))
            }
        }
    };
    gen.into()
}
