use proc_macro::TokenStream;

use convert_case::Casing;
use quote::{quote, ToTokens};
use syn::{
    self,
    Expr,
    ExprLit,
    Lit,
    Meta,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Token,
};

mod codec_router;
mod solidity_router;
mod utils;

#[proc_macro]
pub fn derive_keccak256_id(token: TokenStream) -> TokenStream {
    let signature = token.to_string();
    let method_id = utils::calculate_keccak256_id(&signature);
    TokenStream::from(quote! {
        #method_id
    })
}

#[derive(Debug, PartialEq)]
enum RouterMode {
    Solidity,
    Codec,
}

impl std::str::FromStr for RouterMode {
    type Err = ();

    fn from_str(input: &str) -> Result<RouterMode, Self::Err> {
        match input {
            "solidity" => Ok(RouterMode::Solidity),
            "codec" => Ok(RouterMode::Codec),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct RouterArgs {
    mode: RouterMode,
}

impl Parse for RouterArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut mode = None;

        let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

        for meta in metas {
            if let Meta::NameValue(m) = meta {
                if m.path.is_ident("mode") {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    }) = &m.value
                    {
                        mode = Some(lit_str.value().parse::<RouterMode>().map_err(|_| {
                            syn::Error::new_spanned(&m.value, "Expected 'solidity' or 'codec'")
                        })?);
                    } else {
                        return Err(syn::Error::new_spanned(&m.value, "Expected a string value"));
                    }
                }
            }
        }

        let mode = mode.ok_or_else(|| syn::Error::new(input.span(), "mode is required"))?;

        Ok(Self { mode })
    }
}

#[proc_macro_attribute]
pub fn router(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as RouterArgs);

    let expanded = match args.mode {
        RouterMode::Solidity => solidity_router::derive_solidity_router(TokenStream::new(), item),
        RouterMode::Codec => codec_router::derive_codec_router(TokenStream::new(), item),
    };
    TokenStream::from(expanded)
}

// TODO: d1r1 Implement codec router

// Fake implementation of the attribute to avoid compiler and linter complaints
#[proc_macro_attribute]
pub fn signature(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_parse_solidity_mode() {
        let input: TokenStream = parse_quote!(mode = "solidity");
        let args: RouterArgs = syn::parse2(input).expect("Failed to parse");
        assert_eq!(args.mode, RouterMode::Solidity);
    }

    #[test]
    fn test_parse_codec_mode() {
        let input: TokenStream = parse_quote!(mode = "codec");
        let args: RouterArgs = syn::parse2(input).expect("Failed to parse");
        assert_eq!(args.mode, RouterMode::Codec);
    }

    #[test]
    fn test_parse_invalid_mode() {
        let input: TokenStream = parse_quote!(mode = "InvalidMode");
        let result = syn::parse2::<RouterArgs>(input);
        assert!(result.is_err());
    }
}
