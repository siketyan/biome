use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, parse_macro_input};

struct DocLines(Vec<LitStr>);

impl Parse for DocLines {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut lines = Vec::new();
        while !input.is_empty() {
            lines.push(input.parse()?);
        }
        Ok(Self(lines))
    }
}

pub fn compress_rule_docs_impl(input: TokenStream) -> TokenStream {
    let DocLines(lines) = parse_macro_input!(input as DocLines);
    let mut text = String::new();
    for line in lines {
        text.push_str(&line.value());
        text.push('\n');
    }
    // Level 10 is the strongest level of `miniz_oxide`.
    let compressed = miniz_oxide::deflate::compress_to_vec(text.as_bytes(), 10);
    let literal = Literal::byte_string(&compressed);
    quote!(#literal).into()
}
