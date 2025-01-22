use crate::attribute_field::AttributeField;
use crate::struct_field::StructField;

use syn::{
    braced,
    parse::{Parse, ParseStream},
    Generics, Ident, Result, Token,
};

pub struct State {
    name: Ident,
    generics: Generics,
    struct_fields: Vec<StructField>,
    attribute_fields: Vec<AttributeField>,
}

impl State {
    pub fn unpack(
        self,
    ) -> (Ident, Generics, Vec<StructField>, Vec<AttributeField>) {
        (
            self.name,
            self.generics,
            self.struct_fields,
            self.attribute_fields,
        )
    }
}

impl Parse for State {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![struct]>()?;

        let name: Ident = input.parse()?;

        let generics: Generics = input.parse()?;

        let content;
        braced!(content in input);

        let attrs = AttributeField::parse_inner(&content)?;
        let fields = StructField::parse_inner(&content)?;

        // if !content.is_empty() {
        //     eprintln!("Unconsumed tokens: {}", content);
        //     return Err(content.error("Unexpected tokens remaining"));
        // }

        Ok(Self {
            name,
            generics,
            struct_fields: fields,
            attribute_fields: attrs,
        })
    }
}
