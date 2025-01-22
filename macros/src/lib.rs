mod attribute_field;
mod quantity;
mod state;
mod struct_field;

use quote::quote;
use state::State;

use syn::{parse_macro_input, Type};

use std::collections::HashSet;

/// Parses the following syntax
///
/// struct $STRUCT_NAME {
///     #[$ATTR($FIELD : $TY)]
///     $VISABILITY $NAME : $TYPE,
/// }
///
/// #[derive(State)]
/// impl Test {
///     #[scalar(position : f64)]
///     #[vector(velocity : f64)]
///     #[vector(acceleration : f64)]
///     #[scalar(mass: f64)]
///     #[scalar(id : u32)]
///     name : &str
/// }

#[proc_macro_derive(State, attributes(vector, scalar))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as State);

    let (name, generics, _fields, vectorised_fields) = parsed.unpack();

    let vectorisd_types: Vec<Type> = vectorised_fields
        .iter()
        .map(|field| field.ty.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    for ty in vectorisd_types {
        println!("{}", quote! {#ty});
    }

    let (impl_generics, type_generics, where_clause) =
        generics.split_for_impl();

    let gen = quote! {

        impl #impl_generics #name #type_generics #where_clause {
            pub fn new()  {
                println!("Hello, world!");
            }
        }
    };

    gen.into()
}
