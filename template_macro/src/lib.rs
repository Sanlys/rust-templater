use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, Ident, LitStr};
use std::fs;
use std::env::current_dir;

#[proc_macro]
pub fn template_macro(input: TokenStream) -> TokenStream {
    let raw_filename = parse_macro_input!(input as LitStr);
    let filename = raw_filename.value();
    let file_contents = match fs::read_to_string(&filename) {
        Ok(val) => {
            val
        },
        _ => {
            match current_dir() {
                Ok(dir) => {
                    dir.to_string_lossy().to_string()
                },
                _ => {
                    "kill me".to_string()
                }
            }
        }
    };

    let input_tokens = file_contents;

    let re = Regex::new(r"\{\ *\{(\s*[^}]+\s*)\} *\}").unwrap();
    let formatted = re.replace_all(&input_tokens, "{}");
    let captured_variables: Vec<Ident> = re
        .captures_iter(&input_tokens)
        .filter_map(|cap| {
            cap.get(1).map(|m| {
                let trimmed = m.as_str().trim().to_string();
                return Ident::new(&trimmed, Span::call_site());
            })
        })
        .collect();

    let expanded = if captured_variables.is_empty() {
        quote! {
            format!(#formatted)
        }
    } else {
        quote! {
            format!(#formatted, #(#captured_variables),*)
        }
    };

    expanded.into()
}
