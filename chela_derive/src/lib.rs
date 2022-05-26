// use chela_query::runner::QueryRunner;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, DeriveInput};

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

    // let attr: syn::Attribute = parse_quote!(
    //     #[async_trait]
    // );
    let i = (0..keys.len()).map(syn::Index::from);
    let repository = format_ident!("{}{}", struct_name, "Repository");
    let mut table_name = struct_name.to_string().to_lowercase();
    table_name.push('s');
    let expanded = quote! {
        impl ToEntity for #struct_name {
            fn to_entity(&self)->Entity {

                Entity {
                    table_name: #table_name.to_string(),
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

        struct #repository {
            table_name: &'static str,
        }

        impl Repository for #repository {
            fn table_name(&self) -> &'static str{
                self.table_name
            }
            fn as_any(&self) -> &dyn Any{
                self
            }
            
        }

        impl #repository {
            pub fn new() -> #repository {
                PointRepository {
                   table_name: #table_name,
               }
            }


        }

        // impl Point {
        //     pub fn repo(chela:Chela)->&PointRepository{
        //         let repo_trait = chela.get_repo::<PointRepository>().unwrap();
        //         let repo = repo_trait
        //         .as_any()
        //         .downcast_ref::<PointRepository>()
        //         .unwrap();
        //         repo
        //     }
        // }


        #[async_trait]
        impl QueryRunner for #repository

        where  #struct_name : ToEntity
        {
            type Output = #struct_name;
            async fn first(&self, client: &Client)->Self::Output {
                let first_query = QueryBuilder::new()
                    .select()
                    .from(self.table_name.to_string())
                    .order_by(Some("id".to_string()))
                    .limit(Some(1))
                    .build();
                let row = client
                    .query_one(&first_query.to_string(), &[])
                    .await
                    .unwrap();
                    let x:  #struct_name =  #struct_name::from(row);
                    x
            }
        }

        impl From<tokio_postgres::row::Row> for #struct_name {
            fn from(row: tokio_postgres::row::Row) -> Self {
                Self {
                    #(
                        #idents: row.get(#i),
                    )*
                }
            }
        }






    };
    expanded.into()
}

// #[proc_macro_derive(Repository)]
// pub fn derive_signature2(_: TokenStream) -> TokenStream {
//     let expanded = quote! {
//         impl Repository for PointRepository {
//             fn table_name(&self) -> &'static str{
//                 self.table_name
//             }
//             fn as_any(&self) -> &dyn Any{
//                 self
//             }
//         }

//         impl PointRepository {
//             pub fn new(table_name:&'static str) -> PointRepository {
//                 PointRepository {
//                    table_name: table_name,
//                }
//             }

//         }
//     };

//     expanded.into()
// }
