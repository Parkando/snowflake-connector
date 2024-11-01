extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(SnowflakeDeserialize)]
pub fn snowflake_deserialize_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    impl_snowflake_deserialize(&ast)
}

fn impl_snowflake_deserialize(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let (t_name, t_index, t_ty) = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(data) => {
                let count = data.named.len();
                let mut t_name = Vec::with_capacity(count);
                let mut t_index = Vec::with_capacity(count);
                let mut t_ty = Vec::with_capacity(count);
                for (i, field) in data.named.iter().enumerate() {
                    let name = field.ident.as_ref().unwrap();
                    let ty = &field.ty;
                    t_name.push(name);
                    t_index.push(i);
                    t_ty.push(ty);
                }
                (t_name, t_index, t_ty)
            }
            _ => panic!("Named fields only!"),
        },
        Data::Enum(_) => panic!("This macro can only be derived in a struct, not enum."),
        Data::Union(_) => panic!("This macro can only be derived in a struct, not union."),
    };

    #[rustfmt::skip]
    let gen = quote! {


        impl #impl_generics snowflake_connector::Selectable for #name #ty_generics #where_clause {
	    const SELECT: &'static str = stringify!(#(#t_name),*);
	}


        impl #impl_generics snowflake_connector::SnowflakeDeserialize for #name #ty_generics #where_clause {
            fn snowflake_deserialize(
                response: snowflake_connector::SnowflakeSqlResponse,
            ) -> snowflake_connector::DeserializeResult<snowflake_connector::SnowflakeSqlResult<Self>> {

		use snowflake_connector::DeserializeFromStr;

                let count = response.result_set_meta_data.num_rows;
                let mut results = Vec::with_capacity(count);
                for data in response.data {
                    results.push(#name #ty_generics {
                        #(#t_name: <#t_ty>::deserialize_from_str(data[#t_index].as_deref())
			  .map_err(|err| snowflake_connector::DeserializeError::Field {
			      field: stringify!(#t_name),
			      err: Box::new(err)
			  })?),*
                    });
                }
                Ok(snowflake_connector::SnowflakeSqlResult {
                    data: results,
                })
            }
        }
    };
    gen.into()
}
