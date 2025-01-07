use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Type};

#[proc_macro_derive(State, attributes(vector, scalar))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let name = &input.ident;

    let (impl_generics, type_generics, where_clause) =
        input.generics.split_for_impl();

    let builder_name = format!("{name}Expanded");

    let builder = syn::Ident::new(&builder_name, name.span());

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);

    for (n, t) in field_name.zip(field_type) {
        let n = match n {
            Some(n) => n,
            None => panic!("Fuck"),
        };

        let t = match t {
            Type::Path(type_path) => type_path.path.clone(),
            _ => panic!("Fuck Fuck"),
        };

        let (t, att) = match t.segments.first() {
            Some(t) => (t.ident.to_string(), t.ident.attrs),
            None => panic!("Fuck Fuck Fuck"),
        };
        eprintln!("{:?} : {:?}", n.to_string(), t);
    }

    let gen = quote! {

        // Define the built struct
        struct #builder #impl_generics #where_clause {}

        // Implement the `new()` method for the original struct
        impl #impl_generics #name #type_generics #where_clause {
            pub fn new() -> #builder #type_generics {
                println!("Hello, world!");
                #builder {}
            }
        }
    };

    gen.into()
}
