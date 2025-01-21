use proc_macro2::{Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use std::ops::Range;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Generics, Ident, Result, Token, Type, Visibility,
};

use ndarray::{Array2, ArrayView2, ArrayViewMut2};

use std::collections::HashSet;
/// Parses the following syntax
///
/// struct $STRUCT_NAME {
///     #[$ATTR]
///     $VISABILITY $NAME : $TYPE,
/// }
///  
///
/// For example:
/// #[derive(State)]
/// struct MyState<const D: usize> {
///     #[vector]
///     position: f64,
///     #[vector]
///     velocity: f64,
///     #[vector]
///     acceleration: f64,
///     #[scalar]
///     mass: f64,
///     #[scalar]
///     id: u32,
///     foo: str,
/// }
struct State {
    fields: Vec<StateField>,
    generics: Generics,
    name: Ident,
}

impl Parse for State {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![struct]>()?;

        let name: Ident = input.parse()?;

        let generics: Generics = input.parse()?;

        let content;
        syn::braced!(content in input);

        let fields: Punctuated<StateField, Token![,]> =
            Punctuated::parse_terminated_with(&content, StateField::parse)?;

        Ok(Self {
            name,
            generics,
            fields: fields.into_iter().collect(),
        })
    }
}

struct StateField {
    attribute: Option<Attribute>,
    visibility: Visibility,
    name: Ident,
    ty: Type,
}

impl Parse for StateField {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;

        let visibility: Visibility =
            input.parse().unwrap_or_else(|_| Visibility::Inherited);

        let name: Ident = input.parse()?;

        input.parse::<Token![:]>()?;

        let ty: Type = input.parse()?;

        let attribute = attrs.into_iter().next();

        Ok(Self {
            attribute,
            visibility,
            name,
            ty,
        })
    }
}

impl ToTokens for StateField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.attribute {
            Some(attribute) => attribute.to_tokens(tokens),
            None => {}
        };
        self.visibility.to_tokens(tokens);
        self.name.to_tokens(tokens);
        tokens.append(Punct::new(':', Spacing::Alone));
        self.ty.to_tokens(tokens);
    }
}

#[proc_macro_derive(State, attributes(vector, scalar))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as State);

    let (name, generics, fields) =
        (parsed.name, parsed.generics, parsed.fields);

    let (impl_generics, type_generics, where_clause) =
        generics.split_for_impl();

    let attributed_fields: Vec<&StateField> = fields
        .iter()
        .filter(|field| field.attribute.is_some())
        .collect();

    let non_attributed_fields: Vec<&StateField> = fields
        .iter()
        .filter(|field| field.attribute.is_none())
        .collect();

    let non_attributed_names: Vec<Ident> = non_attributed_fields
        .iter()
        .map(|field| field.name.clone())
        .collect();

    let non_attributed_types: Vec<Type> = non_attributed_fields
        .iter()
        .map(|field| field.ty.clone())
        .collect();

    let index_names: Vec<Ident> = fields
        .iter()
        .map(|field| field.name.clone())
        .map(|name| {
            Ident::new(
                &format!("{}_index", quote! {#name}.to_string()),
                name.span(),
            )
        })
        .collect();

    let attibuted_types: Vec<Type> = fields
        .iter()
        .map(|field| field.ty.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let array_names: Vec<Ident> = attibuted_types
        .iter()
        .map(|ty| {
            Ident::new(
                &format!("array_{}", quote! {#ty}.to_string()),
                ty.span(),
            )
        })
        .collect();

    let get_function_types: Vec<Type> = attributed_fields
        .iter()
        .map(|field| field.ty.clone())
        .collect();

    let get_function_names: Vec<Ident> = attributed_fields
        .iter()
        .map(|field| field.name.clone())
        .map(|name| {
            Ident::new(&format!("get_{}", name).to_string(), name.span())
        })
        .collect();

    let get_mut_function_names: Vec<Ident> = attributed_fields
        .iter()
        .map(|field| field.name.clone())
        .map(|name| {
            Ident::new(&format!("get_{}_mut", name).to_string(), name.span())
        })
        .collect();
    let state_name = syn::Ident::new(&format!("{name}State"), name.span());
    let inner_name = syn::Ident::new(&format!("{name}StateInner"), name.span());

    let gen = quote! {

        struct #inner_name #impl_generics #where_clause {
            #(
                #array_names : Array2<#attibuted_types>,
            )*
            #(
                #index_names : Range<usize>,
            )*
        }

        impl #impl_generics #inner_name #type_generics #where_clause {
            pub fn new() -> Self {

                Self {
                    #(
                        #array_names : Array2::<#attibuted_types>::new(),
                    )*

                }
            }

            #(
                pub fn #get_function_names(&self) -> ArrayView2<#get_function_types> {}
            )*
        }

        struct #state_name #impl_generics  #where_clause {
            inner : #inner_name,
            #(
                #non_attributed_fields,
            )*
        }

        impl #impl_generics #state_name #type_generics #where_clause {
            pub fn new(#(#non_attributed_names : #non_attributed_types, )*) -> Self {
                Self {
                    #(
                        #non_attributed_names,
                    )*
                    inner : #inner_name :: #type_generics ::new(),
                }
            }
        }

        impl #impl_generics #name #type_generics #where_clause {
            pub fn new() -> #state_name #type_generics{
                println!("Hello, world!");
                #state_name::#type_generics::new()
            }
        }
    };

    gen.into()
}
