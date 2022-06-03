// use chela_query::runner::QueryRunner;
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, DeriveInput, Ident, Lit, LitStr, Meta, MetaNameValue,
    NestedMeta, Type,
};

#[proc_macro_derive(ToEntity, attributes(has_many, primary_key))]
pub fn derive_signature(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let mut keys: Vec<TokenStream> = Vec::new();

    let mut has_many_vec = Vec::new();
    let mut column_vec = Vec::new();
    let mut foreign_key = None;
    // let mut uuid = None;
    let mut table_name = None;
    for field in fields.named.iter() {
        parse_has_many(field, &mut foreign_key, &mut table_name, &mut has_many_vec);

        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();

        let literal_key_str = syn::LitStr::new(&name, field.span());
        let type_name = &field.ty;
        let mut type_is_vec = false;
        let key = quote! { #literal_key_str };
        let ty = type_name.to_token_stream();
        parse_type_is_vec(field, &mut type_is_vec);
        parse_primary_key(
            field,
            &mut column_vec,
            key.clone(),
            ty.clone(),
            &mut type_is_vec,
        );

        // if !type_is_vec {
        //     keys.push(key);

        //     idents.push(&field.ident);
        //     types.push(ty);
        // }
    }

    // let has_many_tokens = (0..has_many_vec.len()).map(syn::Index::from);
    let i = (0..keys.len()).map(syn::Index::from);
    let repository = format_ident!("{}{}", struct_name, "Repository");
    let mut table_name = struct_name.to_string().to_lowercase();
    table_name.push('s');
    let preloads = build_preloads();
    let has_many = build_vec(has_many_vec);
    let columns = build_vec(column_vec);

    let entity = build_entity(table_name, has_many, struct_name_str, columns);

    let expanded = quote! {
        impl ToEntity for #struct_name {
            fn to_entity()->Entity {

                #entity
                entity
            }
        }

        struct #repository {
            entity: Entity,
            pub preloads: HashMap<String,QueryBuilder>
        }

        impl Repository for #repository {
            fn entity(&self) -> Entity{
                self.entity.clone()
            }
            // fn as_any(&self) -> &dyn Any{
            //     self
            // }

        }

        impl #repository {
            pub fn new() -> #repository {
               #entity
               #preloads
                #repository {
                   entity: entity,
                   preloads: preloads
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

fn parse_type_is_vec(field: &syn::Field, type_is_vec: &mut bool) {
    if let Type::Path(ref p) = field.ty {
        *type_is_vec = p.path.segments.iter().next().unwrap().ident.to_string() == "Vec"
    } else {
        *type_is_vec = false;
    }
}

fn parse_primary_key(
    field: &syn::Field,
    columns: &mut Vec<TokenStream>,
    key: TokenStream,
    ty: TokenStream,
    type_is_vec: &mut bool,
) {
    let mut auto_increment = false;
    for attribute in field
        .attrs
        .iter()
        .filter(|attribute| attribute.path.is_ident("primary_key"))
    {
        let meta: Meta = attribute.parse_meta().unwrap(); //.unwrap_or_abort();

        const VALID_FORMAT: &str = r#"Expected `#[primary_key(auto_increment=true)]`"#;
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
                            ("auto_increment", Lit::Bool(lit)) => auto_increment = lit.value,
                            // ("uuid", Lit::Str(lit)) => uuid = Some(lit),
                            _ => abort_call_site!(VALID_FORMAT),
                        },

                        _ => abort_call_site!(VALID_FORMAT),
                    }
                } else {
                    abort_call_site!(VALID_FORMAT);
                }
            }
        }
    }
    if auto_increment {
        let column_primary_key_auto_increment =
            build_column_primary_key_auto_increment(key.clone());
        columns.push(column_primary_key_auto_increment);
    } else {
        if !*type_is_vec {
            let column = build_column_not_null(key.clone(), ty.clone());
            columns.push(column);
        }
    }
}

fn parse_has_many(
    field: &syn::Field,
    foreign_key: &mut Option<LitStr>,
    table_name: &mut Option<LitStr>,
    has_many_vec: &mut Vec<TokenStream>,
) {
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
                            ("foreign_key", Lit::Str(lit)) => *foreign_key = Some(lit),
                            ("table_name", Lit::Str(lit)) => *table_name = Some(lit),
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
            let struct_n = syn::LitStr::new(&struct_name, field.span());
            let has_many = build_has_many(foreign_key.clone(), struct_n, table_n);
            has_many_vec.push(has_many)
        }
    }
}

fn build_column_primary_key_auto_increment(key: TokenStream) -> TokenStream {
    let data_type = quote! { serial() };
    let options = quote! {primary_key_unique()};
    build_column(key, data_type, options)
}

fn build_column(key: TokenStream, data_type: TokenStream, options: TokenStream) -> TokenStream {
    quote! {
        Column {
            name: #key.to_string(),
            data_type: #data_type,
            options: #options
        }
    }
}

fn build_column_not_null(key: TokenStream, data_type: TokenStream) -> TokenStream {
    let d = quote! { DataType::from(stringify!(#data_type).to_string()) };
    let options = quote! {not_null()};
    build_column(key, d, options)
}

fn build_vec(has_many_vec: Vec<TokenStream>) -> TokenStream {
    quote! {vec![
        #(
            #has_many_vec
        ),*
    ]}
}

fn build_has_many(foreign_key: Option<LitStr>, struct_n: LitStr, table_n: LitStr) -> TokenStream {
    quote! {
            HasMany {
                foreign_key: #foreign_key.to_string(),
                struct_name: #struct_n.to_string(),
                table_name: #table_n.to_string(),
            }

    }
}

fn build_entity(
    table_name: String,
    has_many: TokenStream,
    struct_name_str: LitStr,
    columns: TokenStream,
) -> TokenStream {
    quote! {
            let entity = Entity {
            table_name: #table_name.to_string(),
            struct_name: #struct_name_str.to_string(),
            has_many: #has_many,
            columns: #columns,
            };
    }
}
fn build_preloads() -> TokenStream {
    quote! {
            let tuples : Vec<(String,QueryBuilder)>= entity.has_many.iter().map(|has_many| {
           (has_many.table_name.clone(),
           QueryBuilder::new()
           .select()
           .from(has_many.table_name.clone().to_string())
           .where_(has_many.foreign_key.to_string()) )
        }).collect();
        let preloads: HashMap<_, _> = tuples.into_iter().collect();
    }
}

fn table_to_struct_name<'a>(table_n: &'a String) -> String {
    let first_letter = table_n[0..1].to_uppercase().to_string().to_owned();
    let struct_name: &str = &table_n[1..table_n.len() - 1];
    first_letter + struct_name
}

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
