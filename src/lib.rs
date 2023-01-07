use proc_macro2::TokenStream as TokenStream2;

use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result as SynResult},
    parse2, Attribute,
};

struct Ts(TokenStream2);

impl Parse for Ts {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let content;
        parenthesized!(content in input);
        Ok(Self(content.parse()?))
    }
}

/// Parse a slice of [`Attribute`] into [`TokenStream2`] with the name of the
/// attribute specified.
pub fn attrs_to_ts(attrs: &[Attribute], name: &str) -> Option<TokenStream2> {
    attrs
        .iter()
        .filter(|attr| attr.path.segments.len() == 1)
        .filter(|attr| attr.path.segments[0].ident == name)
        .last()
        .map(|attr| {
            parse2::<Ts>(attr.tokens.clone())
                .expect("invalid attributes")
                .0
        })
}
