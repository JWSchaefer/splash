mod attribute_field;
mod attribute_info;
mod quantity;
mod struct_field;
mod struct_info;

use attribute_info::AttributeInfo;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use struct_info::StructInfo;
use syn::parse_macro_input;

/// Parses the following syntax
///
/// struct $STRUCT_NAME {
///     #[$ATTR($FIELD : $TY)]
///     $VISABILITY $NAME : $TYPE,
/// }
///
/// #[lazy_nd(dim = D)]
/// impl Test<const D : usize> {
///     #[scalar(position : f64)]
///     #[vector(velocity : f64)]
///     #[vector(acceleration : f64)]
///     #[scalar(mass: f64)]
///     #[scalar(id : u32)]
///     name : &str
/// }

#[proc_macro_attribute]
pub fn lazy_nd(
    attr: TokenStream,
    item: TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as AttributeInfo);
    let parsed = parse_macro_input!(item as StructInfo);

    let (name, generics, struct_fields, attributed_fields) = parsed.unpack();

    let (impl_generics, type_generics, where_clause) =
        generics.split_for_impl();

    let gen = quote! { struct Test {}};

    gen.into()
}
