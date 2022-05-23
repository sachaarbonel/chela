use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToEntity)]
pub fn derive_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    let struct_name_str = syn::LitStr::new(&struct_name.to_string(), struct_name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let mut keys = Vec::new();
    let mut idents = Vec::new();
    let mut types = Vec::new();

    for field in fields.named.iter() {
        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();
        let literal_key_str = syn::LitStr::new(&name, field.span());
        let type_name = &field.ty;
        keys.push(quote! { #literal_key_str });
        idents.push(&field.ident);
        types.push(type_name.to_token_stream());
    }

    let expanded = quote! {
        impl ToEntity for #struct_name {
            fn to_entity(&self)->Entity {
                let mut table_name = #struct_name_str.to_lowercase();
                table_name.push('s');
                Entity {
                    table_name: table_name.to_string(),
                    struct_name: #struct_name_str.to_string(),
                    columns:
                    vec![
                        #( Column {
                            column_name: #keys.to_string(),
                            column_type: stringify!(#types).to_string(),
                        }
                        ),*
                    ]
                }
            }
        }
    };
    expanded.into()
}
