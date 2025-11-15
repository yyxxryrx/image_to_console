use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Options)]
pub fn derive_options(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("Options can only be derived for structs"),
    };
    let (f_names, f_types, f_vises) = fields.iter().fold(
        (vec![], vec![], vec![]),
        |(mut names, mut types, mut vises), field| match &field.vis {
            syn::Visibility::Inherited => (names, types, vises),
            _ => {
                if let Some(ident) = field.ident.as_ref() {
                    names.push(ident);
                    types.push(&field.ty);
                    vises.push(&field.vis);
                }
                (names, types, vises)
            }
        },
    );
    let expanded = quote! {
        impl #name {
            #(
                    #f_vises fn #f_names(&mut self, #f_names: #f_types) -> &mut Self {
                    self.#f_names = #f_names;
                    self
                }
            )*
            pub fn get_options(&self) -> #name {
                self.clone()
            }
        }
    };
    TokenStream::from(expanded)
}
