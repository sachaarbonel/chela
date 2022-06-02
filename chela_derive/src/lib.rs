// use chela_query::runner::QueryRunner;
use proc_macro::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, DeriveInput, Lit, Meta, MetaNameValue, NestedMeta, Type,
};


#[proc_macro_derive(ToEntity, attributes(has_many))]
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
    let mut has_many_vec = Vec::new();
    let mut foreign_key = None;
    let mut table_name = None;
    for field in fields.named.iter() {
        for attribute in field
            .attrs
            .iter()
            .filter(|attribute| attribute.path.is_ident("has_many"))
        {
            let meta: Meta = attribute.parse_meta().unwrap(); //.unwrap_or_abort();

            const VALID_FORMAT: &str = r#"Expected `#[has_many(foreign_key="foreign_key_name", table_name="your table name")]`"#;
            if let Meta::List(meta) = meta {
                for meta in meta.nested {
                    if let NestedMeta::Meta(meta) = meta {
                        match meta {
                            Meta::NameValue(MetaNameValue { path, lit, .. }) => match (
                                path.get_ident()
                                    .unwrap_or_else(|| abort_call_site!(VALID_FORMAT))
                                    .to_string()
                                    .as_str(),
                                lit,
                            ) {
                                ("foreign_key", Lit::Str(lit)) => foreign_key = Some(lit),
                                ("table_name", Lit::Str(lit)) => table_name = Some(lit),
                                _ => abort_call_site!(VALID_FORMAT),
                            },

                            _ => abort_call_site!(VALID_FORMAT),
                        }
                    } else {
                        abort_call_site!(VALID_FORMAT);
                    }
                }
            }
            if let Some(table_n) = table_name.clone() {
                let table_n_value = table_n.value();
                let struct_name = table_to_struct_name(&table_n_value);
                let struct_n = syn::LitStr::new(&struct_name.to_uppercase(), field.span());
                has_many_vec.push(quote! {
                        HasMany {
                            foreign_key: #foreign_key.to_string(),
                            struct_name: #struct_n.to_string(),
                            table_name: #table_n.to_string(),
                        }

                })
            }
        }
        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();

        let literal_key_str = syn::LitStr::new(&name, field.span());
        let type_name = &field.ty;
        let type_is_vec = if let Type::Path(ref p) = field.ty {
            p.path.segments.iter().next().unwrap().ident.to_string() == "Vec"
        } else {
            false
        };
        if !type_is_vec {
            keys.push(quote! { #literal_key_str });

            idents.push(&field.ident);
            types.push(type_name.to_token_stream());
        }
    }

    // let has_many_tokens = (0..has_many_vec.len()).map(syn::Index::from);
    let i = (0..keys.len()).map(syn::Index::from);
    let repository = format_ident!("{}{}", struct_name, "Repository");
    let mut table_name = struct_name.to_string().to_lowercase();
    table_name.push('s');
    let entity = quote! {Entity {
        table_name: #table_name.to_string(),
        struct_name: #struct_name_str.to_string(),
        has_many: vec![
            #(
                #has_many_vec
            ),*
        ],
        columns:
        vec![
            #( Column {
                column_name: #keys.to_string(),
                column_type: ColumnType::from(stringify!(#types).to_string()),
            }
            ),*
        ]
    }};
    let expanded = quote! {
        impl ToEntity for #struct_name {
            fn to_entity()->Entity {

                #entity
            }
        }

        struct #repository {
            entity: Entity,
        }

        impl Repository for #repository {
            fn entity(self) -> Entity{
                self.entity
            }
            fn as_any(&self) -> &dyn Any{
                self
            }

        }

        impl #repository {
            pub fn new() -> #repository {
                #repository {
                   entity: #entity,
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


        // #[async_trait]
        // impl QueryRunner for #repository

        // where  #struct_name : ToEntity
        // {
        //     type Output = #struct_name;
        //     async fn first(self, client: &Client)->Self::Output {
        //         let first_query = QueryBuilder::new()
        //             .select()
        //             .from(self.entity().table_name.to_string())
        //             .order_by(Some("id".to_string()))
        //             .limit(Some(1))
        //             .build();
        //         let row = client
        //             .query_one(&first_query.to_string(), &[])
        //             .await
        //             .unwrap();
        //             let x:  #struct_name =  #struct_name::from(row);
        //             x
        //     }
        // }

        // impl From<tokio_postgres::row::Row> for #struct_name {
        //     fn from(row: tokio_postgres::row::Row) -> Self {
        //         Self {
        //             #(
        //                 #idents: row.get(#i),
        //             )*
        //         }
        //     }
        // }






    };
    expanded.into()
}

fn table_to_struct_name<'a>(table_n: &'a String) -> String {
    let first_letter = table_n[0..1].to_uppercase().to_string().to_owned();
    let struct_name: &str = &table_n[1..table_n.len() - 1];
    first_letter + struct_name
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

#[cfg(test)]
mod tests {
    use crate::table_to_struct_name;

    #[test]
    fn it_works() {
        let user_string = "users".to_string();
        let struct_name = table_to_struct_name(&user_string);
        assert_eq!(struct_name, "User");
    }
}
