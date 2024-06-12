use proc_macro::TokenStream;

use convert_case::{Case, Casing};
use quote::{quote, ToTokens};
use syn::{
    self,
    Expr,
    FnArg,
    Ident,
    ImplItem,
    ImplItemFn,
    ItemImpl,
    Lit,
    LitStr,
    parse::Parse,
    parse_macro_input,
    punctuated::Punctuated,
    Token,
    Type,
    TypePath,
};

use crate::utils::{get_all_methods, get_public_methods};

// #[proc_macro_attribute]
pub fn derive_solidity_router(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemImpl = parse_macro_input!(item as ItemImpl);
    let struct_name = &ast.self_ty;

    let all_methods = get_all_methods(&ast);
    let public_methods = get_public_methods(&ast);

    // Dispatch all methods (public and private) if implementing a trait
    let methods_to_dispatch = if ast.trait_.is_some() {
        all_methods
            .clone()
            .into_iter()
            .filter(|func| func.sig.ident != "deploy")
            .collect()
    } else {
        public_methods.clone()
    };

    // Generate Solidity function signatures or use provided ones from #[signature]
    let signatures = get_signatures(&methods_to_dispatch);

    // Derive route method that dispatches Solidity function calls
    let router_impl = derive_route_method(&methods_to_dispatch);

    let expanded = quote! {
        use alloy_sol_types::{sol, SolCall, SolValue};
        use fluentbase_sdk::{
            Address,
            Bytes,
            U256,
        };

        impl #struct_name {
            #( #all_methods )*
            #router_impl
        }
        #signatures
    };

    TokenStream::from(expanded)
}

fn get_signatures(methods: &[&ImplItemFn]) -> proc_macro2::TokenStream {
    let mut signatures: Vec<proc_macro2::TokenStream> = vec![];
    for func in methods {
        let sig: Option<LitStr> = func.attrs.iter().find_map(|attr| {
            if attr.path().is_ident("signature") {
                attr.parse_args().ok()
            } else {
                None
            }
        });

        if let Some(fn_signature) = sig {
            let signature_value = fn_signature.value();
            let full_signature = if signature_value.starts_with("function ") {
                signature_value + "; "
            } else {
                let method_name = &func.sig.ident;
                let sol_method_name = rust_name_to_sol(method_name);

                let inputs = parse_function_inputs(&func.sig.inputs);
                let output = if let syn::ReturnType::Type(_, ty) = &func.sig.output {
                    rust_type_to_sol(ty)
                } else {
                    quote! { void }
                };

                format!(
                    "function {}({}) external returns ({});",
                    sol_method_name,
                    inputs
                        .into_iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    output.to_string()
                )
            };

            let fn_signature = syn::parse_str::<proc_macro2::TokenStream>(&full_signature)
                .expect("Failed to parse signature");
            signatures.push(fn_signature);
        } else {
            let method_name = &func.sig.ident;
            let sol_method_name = rust_name_to_sol(method_name);

            let inputs = parse_function_inputs(&func.sig.inputs);
            let output = if let syn::ReturnType::Type(_, ty) = &func.sig.output {
                rust_type_to_sol(ty)
            } else {
                quote! { void }
            };
            // Generate function signature in Solidity syntax
            signatures.push(quote! {
                function #sol_method_name(#(#inputs),*) external returns (#output);
            });
        }
    }
    quote! {
        sol! {
            #(#signatures)*
        }
    }
}

fn derive_route_method(methods: &Vec<&ImplItemFn>) -> proc_macro2::TokenStream {
    let selectors: Vec<proc_macro2::TokenStream> = methods
        .iter()
        .filter_map(|method| {
            let selector = derive_route_selector_arm(method);
            Some(selector)
        })
        .collect();

    let match_arms = if selectors.is_empty() {
        quote! {
            _ => panic!("No methods to route"),
        }
    } else {
        quote! {
            #(#selectors),*,
            _ => panic!("unknown method"),
        }
    };

    quote! {
        pub fn main<SDK: SharedAPI>(&self) {
            let input_size = SDK::input_size();
            if input_size < 4 {
                panic!("input too short, cannot extract selector");
            }
            let mut selector: [u8; 4] = [0; 4];
            SDK::read(&mut selector, 0);
            let input = fluentbase_sdk::alloc_slice(input_size as usize);
            SDK::read(input, 0);
            match selector {
                #match_arms
            }
        }
    }
}

fn derive_route_selector_arm(func: &ImplItemFn) -> proc_macro2::TokenStream {
    let method_name = &func.sig.ident;
    let method_name_call = sol_call_fn_name(method_name);
    let selector_name = quote! { #method_name_call::SELECTOR };
    let abi_decode = quote! { #method_name_call::abi_decode };

    let args: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    Some(&pat_ident.ident)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let args_expr = derive_route_selector_args(&args, &abi_decode);

    quote! {
        #selector_name => {
            #args_expr
            let output = self.#method_name(#(#args),*).abi_encode();
            SDK::write(&output);
        }
    }
}

fn derive_route_selector_args(
    args: &[&Ident],
    abi_decode_fn: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if args.len() == 1 {
        let arg = args[0];
        quote! {
            let #arg = match #abi_decode_fn(&input, true) {
                Ok(decoded) => decoded.#arg,
                Err(e) => {
                    panic!("Failed to decode input {:?}", e);
                }
            };
        }
    } else {
        let fields: Vec<proc_macro2::TokenStream> =
            args.iter().map(|arg| quote! { decoded.#arg }).collect();
        quote! {
            let (#(#args),*) = match #abi_decode_fn(&input, true) {
                Ok(decoded) => (#(#fields),*),
                Err(e) => {
                    panic!("Failed to decode input {:?}", e);
                }
            };
        }
    }
}

fn parse_function_inputs(inputs: &Punctuated<FnArg, Token![,]>) -> Vec<proc_macro2::TokenStream> {
    inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let param_name = &pat_ident.ident;
                    let sol_type = rust_type_to_sol(&*pat_type.ty);
                    Some(quote! { #sol_type #param_name })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn sol_call_fn_name(method_name: &Ident) -> Ident {
    let method_name_sol = rust_name_to_sol(method_name);
    Ident::new(&(method_name_sol.to_string() + "Call"), method_name.span())
}

fn rust_name_to_sol(ident: &Ident) -> Ident {
    let span = ident.span();
    let camel_name = ident.to_string().to_case(Case::Camel);
    Ident::new(&camel_name, span)
}

fn rust_type_to_sol(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Array(ty) => convert_array_type(ty),
        Type::Paren(ty) => convert_paren_type(ty),
        Type::Slice(ty) => convert_slice_type(ty),
        Type::Tuple(ty) => convert_tuple_type(ty),
        Type::Path(type_path) => convert_path_type(type_path),
        Type::Reference(type_ref) => rust_type_to_sol(&type_ref.elem),
        _ => panic!("Unsupported type: {}", ty.to_token_stream().to_string()),
    }
}

fn convert_array_type(ty: &syn::TypeArray) -> proc_macro2::TokenStream {
    if let Expr::Lit(expr_lit) = &ty.len {
        if let Lit::Int(lit_int) = &expr_lit.lit {
            let len = lit_int.base10_digits();
            let len_token: proc_macro2::TokenStream = len.parse().expect("Invalid token");
            let elem_type = rust_type_to_sol(&ty.elem);
            quote! { #elem_type[#len_token] }
        } else {
            panic!("Unsupported array length literal")
        }
    } else {
        panic!("Unsupported array length expression");
    }
}

fn convert_paren_type(ty: &syn::TypeParen) -> proc_macro2::TokenStream {
    rust_type_to_sol(&ty.elem)
}

fn convert_slice_type(ty: &syn::TypeSlice) -> proc_macro2::TokenStream {
    let elem_type = rust_type_to_sol(&ty.elem);
    quote! { #elem_type[] }
}

fn convert_tuple_type(ty: &syn::TypeTuple) -> proc_macro2::TokenStream {
    let elems = ty.elems.iter().map(|elem| rust_type_to_sol(elem));
    quote! { (#(#elems),*)}
}

fn convert_path_type(type_path: &TypePath) -> proc_macro2::TokenStream {
    let ident = &type_path.path.segments.last().unwrap().ident;
    match ident.to_string().as_str() {
        "String" | "str" => quote! { string },
        "bool" => quote! { bool },
        "u8" => quote! { uint8 },
        "u16" => quote! { uint16 },
        "u32" => quote! { uint32 },
        "u64" => quote! { uint64 },
        "u128" => quote! { uint128 },
        "u256" | "uint" => quote! { uint256 },
        "U256" => quote! { uint256 },
        "i8" => quote! { int8 },
        "i16" => quote! { int16 },
        "i32" => quote! { int32 },
        "i64" => quote! { int64 },
        "i128" => quote! { int128 },
        "i256" | "int" => quote! { int256 },
        "Address" => quote! { address },
        "Bytes" => quote! { bytes },
        "Vec" => {
            if let syn::PathArguments::AngleBracketed(args) =
                &type_path.path.segments.last().unwrap().arguments
            {
                if let Some(syn::GenericArgument::Type(arg_ty)) = args.args.first() {
                    let elem_type = rust_type_to_sol(arg_ty);
                    quote! { #elem_type[] }
                } else {
                    panic!("Unsupported vector element type")
                }
            } else {
                panic!("Unsupported vector arguments")
            }
        }
        ident_str if ident_str.starts_with("bytes") => {
            if ident_str == "bytes" {
                quote! { bytes }
            } else if ident_str.len() > 5 && ident_str[5..].parse::<usize>().is_ok() {
                let bytes_len: usize = ident_str[5..].parse().unwrap();
                let bytes_ident = Ident::new(&format!("bytes{}", bytes_len), ident.span());
                quote! { #bytes_ident }
            } else {
                panic!("Unsupported bytes type: {}", ident_str)
            }
        }
        _ => panic!("Unsupported type: {}", ident),
    }
}

#[cfg(test)]
mod tests {
    use syn::{Ident, parse_quote, TypeArray, TypeParen, TypePath, TypeSlice, TypeTuple};

    use super::*;

    #[test]
    fn test_get_signatures_full_signature() {
        let item_impl: ItemImpl = parse_quote! {
            impl ExampleStruct {
                #[signature("function greeting(string message) external returns (string)")]
                fn greeting(&self, message: String) -> String {
                    message
                }
            }
        };

        let methods = item_impl
            .items
            .iter()
            .filter_map(|item| {
                if let ImplItem::Fn(func) = item {
                    Some(func)
                } else {
                    None
                }
            })
            .collect::<Vec<&ImplItemFn>>();

        let signatures = get_signatures(&methods);

        let expected = quote! {
            sol! {
                function greeting(string message) external returns (string);
            }
        };

        assert_eq!(signatures.to_string(), expected.to_string());
    }

    #[test]
    fn test_get_signatures_short_signature() {
        let item_impl: ItemImpl = parse_quote! {
            impl ExampleStruct {
                #[signature("customGreeting(string)")]
                fn custom_greeting(&self, message: String) -> String {
                    message
                }
            }
        };

        let methods = item_impl
            .items
            .iter()
            .filter_map(|item| {
                if let ImplItem::Fn(func) = item {
                    Some(func)
                } else {
                    None
                }
            })
            .collect::<Vec<&ImplItemFn>>();

        let signatures = get_signatures(&methods);

        let expected = quote! {
            sol! {
                function customGreeting(string message) external returns (string);
            }
        };

        assert_eq!(signatures.to_string(), expected.to_string());
    }

    #[test]
    fn test_derive_route_selector_arm() {
        let func: ImplItemFn = parse_quote! {
            pub fn greet(&self, msg: String) -> String {
                msg
            }
        };

        let expected = quote! {
            greetCall::SELECTOR => {
                let msg = match greetCall::abi_decode(&input, true) {
                    Ok(decoded) => decoded.msg,
                    Err(e) => {
                        panic!("Failed to decode input {:?}", e);
                    }
                };
                let output = self.greet(msg).abi_encode();
                SDK::write(&output);
            }
        };

        let actual = derive_route_selector_arm(&func);
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_get_signatures() {
        let item_impl: ItemImpl = parse_quote! {
            impl ExampleStruct {
                #[signature("function greeting() external view returns ()")]
                pub fn greeting(&self, msg: String) -> String {
                    msg
                }

                pub fn regular_fn(&self, msg: String) -> String {
                    msg
                }
                pub fn greeting_msg(&self, msg: String) -> String {
                    msg
                }
            }
        };

        let methods = get_public_methods(&item_impl);
        let signatures = get_signatures(&methods);

        let expected = quote! {
            sol! {
                function greeting() external view returns ();
                function regularFn(string msg) external returns (string);
                function greetingMsg(string msg) external returns (string);
            }
        };

        assert_eq!(signatures.to_string(), expected.to_string());
    }

    #[test]
    fn test_rust_name_to_sol() {
        let ident = Ident::new("test_function", proc_macro2::Span::call_site());
        let sol_ident = rust_name_to_sol(&ident);
        assert_eq!(sol_ident.to_string(), "testFunction");
    }

    #[test]
    fn test_get_method_call() {
        let method_name = Ident::new("test_function", proc_macro2::Span::call_site());
        let method_call_ident = sol_call_fn_name(&method_name);
        assert_eq!(method_call_ident.to_string(), "testFunctionCall");
    }

    #[test]
    fn test_convert_array_type() {
        let ty: TypeArray = parse_quote!([u8; 32]);
        let result = convert_array_type(&ty);
        assert_eq!(result.to_string(), "uint8 [32]");
    }

    #[test]
    fn test_convert_paren_type() {
        let ty: TypeParen = parse_quote!((u8));
        let result = convert_paren_type(&ty);
        assert_eq!(result.to_string(), "uint8");
    }

    #[test]
    fn test_convert_slice_type() {
        let ty: TypeSlice = parse_quote!([u8]);
        let result = convert_slice_type(&ty);
        assert_eq!(result.to_string(), "uint8 []");
    }

    #[test]
    fn test_convert_tuple_type() {
        let ty: TypeTuple = parse_quote!((u8, u16));
        let result = convert_tuple_type(&ty);
        assert_eq!(result.to_string(), "(uint8 , uint16)");
    }

    #[test]
    fn test_convert_path_type_string() {
        let ty: TypePath = parse_quote!(String);
        let result = convert_path_type(&ty);
        assert_eq!(result.to_string(), "string");
        let ty: TypePath = parse_quote!(str);
        let result = convert_path_type(&ty);
        assert_eq!(result.to_string(), "string");
    }

    #[test]
    fn test_convert_path_type_bool() {
        let ty: TypePath = parse_quote!(bool);
        let result = convert_path_type(&ty);
        assert_eq!(result.to_string(), "bool");
    }

    #[test]
    fn test_convert_path_type_uint() {
        let ty: TypePath = parse_quote!(u256);
        let result = convert_path_type(&ty);
        assert_eq!(result.to_string(), "uint256");
    }

    #[test]
    fn test_convert_path_type_vec() {
        let ty: TypePath = parse_quote!(Vec<u8>);
        let result = convert_path_type(&ty);
        assert_eq!(result.to_string(), "uint8 []");
    }

    #[test]
    fn test_convert_path_type_bytes() {
        let ty: TypePath = parse_quote!(bytes);
        let result = convert_path_type(&ty);
        assert_eq!(result.to_string(), "bytes");
    }

    #[test]
    fn test_convert_path_type_bytes_fixed() {
        let ty: TypePath = parse_quote!(bytes32);
        let result = convert_path_type(&ty);
        assert_eq!(result.to_string(), "bytes32");
    }
}
