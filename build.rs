#![allow(clippy::ptr_arg, clippy::collapsible_if)]

use std::{io::Write, ops::Not};

use heck::ToSnakeCase;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, ItemStruct};

fn main() {
    // std::fs::write("build_logs.txt", "").unwrap();

    let objects = std::fs::read_to_string("./src/objects/chart_objects.rs").unwrap();

    let symbols = syn::parse_file(&objects).unwrap();

    let use_ = quote! {
        '\n'
        use {
            super::{
                chart_objects::*,
                helper_objects::*
            },
            crate::{Annotation, FnWithArgs},
            std::collections::*
        };
    };

    let impl_blocks = symbols
        .items
        .into_iter()
        .filter_map(|item| match item {
            syn::Item::Struct(item_struct) => Some(item_struct),
            _ => None,
        })
        .map(|s| {
            let s_name = &s.ident;

            let generics = &s.generics;

            let type_param = s
                .generics
                .type_params()
                .map(|t| t.ident.clone())
                .collect_vec();

            let type_params = if type_param.is_empty().not() {
                quote! { < #(#type_param),* > }
            } else {
                quote! {}
            };

            let new = quote! {
                pub fn new() -> Self {
                    Self::default()
                }
                '\n'
            };

            let methods = s.clone()
                .fields
                .into_iter()
                .map(|field| {
                    let name = &field.ident;

                    let l_name = name.as_ref().unwrap().to_string().to_snake_case();
                    let set_name =
                        &if l_name.eq("r_type") {
                            format!(
                                "{}_{}", 
                                s_name.to_string()
                                    .to_snake_case()
                                    .split("_")
                                    .last()
                                    .unwrap_or_default(),
                                l_name
                            ).replace("r_", "")
                        }
                        else {
                            l_name
                        };
                    let get_name = syn::Ident::new(
                        &format!("get_{}", set_name),
                        proc_macro2::Span::call_site(),
                    );
                    let set_name = ident(set_name).unwrap();

                    let type_ = field.ty.clone();

                    let (type_segments, type_seperators) = type_segments(&type_);

                    let default_set_fn = if type_segments[0].0 == "Option" {
                        let mut seps = type_seperators.iter().skip(1).rev().skip(1).rev();
                        let mut type_vec = Vec::new();
                        for (seg, _) in type_segments.iter().skip(1) {
                            type_vec.push(seg);
                            if let Some(s) = seps.next() {
                                type_vec.push(s);
                            }
                        }
                        // append_log(type_vec.clone().into_iter().join(""));
                        let type_ = ident(&type_vec.into_iter().join("")).unwrap().to_token_stream();

                        quote!{
                            pub fn #set_name(mut self, value: impl Into<#type_>) -> #s_name #type_params {
                                self.#name = Some(value.into());
                                self
                            }
                        }
                    }
                    else {
                        quote!{
                            pub fn #set_name(mut self, value: impl Into<#type_>) -> #s_name #type_params {
                                self.#name = value.into();
                                self
                            }
                        }
                    };

                    let override_fn = override_set_fn(&s, &field);

                    let set_fn = override_fn.unwrap_or(default_set_fn);

                    quote! {
                        pub fn #get_name(&mut self) -> &mut #type_ {
                            &mut self.#name
                        }
                        #set_fn
                        '\n'
                    }
                    .to_token_stream()
                })
                .collect_vec();

            quote! {
                '\n'
                impl #generics #s_name #type_params {
                    #new

                    #(#methods)*
                }
            }
        })
        .collect();

    let code = [Vec::from([use_]), impl_blocks]
        .concat()
        .into_iter()
        .map(|token| token.to_string())
        .collect_vec()
        .join("\n")
        .replace("'\\n'", "\n\n");

    let mut formatted_code = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    formatted_code
        .stdin
        .take()
        .map(|mut s| s.write_all(code.as_bytes()));

    std::fs::write(
        format!("{}/{}", std::env::var("OUT_DIR").unwrap(), "methods.rs"),
        String::from_utf8_lossy(&formatted_code.wait_with_output().unwrap().stdout).to_string(),
    )
    .unwrap();
}

fn override_set_fn(s: &ItemStruct, field: &Field) -> Option<TokenStream> {
    let s_name = &s.ident;
    let name = &field.ident;
    let type_ = &field.ty;
    let type_param = s
        .generics
        .type_params()
        .map(|t| t.ident.clone())
        .collect_vec();
    let type_params = if type_param.is_empty().not() {
        quote! { < #(#type_param),* > }
    } else {
        quote! {}
    };
    let set_name = ident(&name.clone().unwrap().to_string().to_snake_case()).unwrap();

    let (type_segments, _type_seperators) = type_segments(type_);

    // append_log(type_.to_token_stream().to_string());
    // append_log(
    //     type_segments
    //         .clone()
    //         .into_iter()
    //         .map(|(seg, _)| seg)
    //         .collect_vec()
    //         .join(" | "),
    // );

    if let Some((seg, _type_)) = type_segments.first() {
        // for Vec<T>
        if seg == "Vec" {
            let inner_t = &type_segments[1].1;
            let iterator_set_fn = quote! {
                pub fn #set_name<T: Into<#inner_t>>(mut self, value: impl IntoIterator<Item = T>) -> #s_name #type_params {
                    self.#name = value.into_iter().map(Into::into).collect();
                    self
                }
            };
            return Some(iterator_set_fn);
        }

        if seg == "Option" {
            // for Option<HashMap<T, U>>
            if type_segments[1].0 == "HashMap" {
                let inner_t = &type_segments[2].1;
                let inner_u = &type_segments[3].1;
                let iterator_set_fn = quote! {
                    pub fn #set_name<T: Into<#inner_t>, U: IntoIterator<Item = (T, #inner_u)>>(mut self, value: U) -> #s_name #type_params {
                        self.#name = Some(value.into_iter().map(|(k, v)| (k.into(), v)).collect());
                        self
                    }
                };
                return Some(iterator_set_fn);
            }

            if type_segments[1].0 == "u32" {
                let type_ = &type_segments[1].1;

                return Some(quote! {
                    pub fn #set_name(mut self, value: #type_) -> #s_name #type_params {
                        self.#name = Some(value);
                        self
                    }
                });
            }
        }
    }

    None
}

fn type_segments(type_: &syn::Type) -> (Vec<(String, syn::Type)>, Vec<String>) {
    let type_segments = type_.to_token_stream().to_string();
    // append_log(&type_segments);
    let segs = type_segments
        .split("<")
        .flat_map(|seg| seg.split(","))
        .flat_map(|seg| seg.split(">"))
        .map(|seg| seg.trim().to_string())
        .filter_map(|seg| Some((seg.clone(), ident(seg.trim()).ok()?)))
        .collect_vec();

    let mut chars = type_segments
        .chars()
        .filter(|c| [':', '<', '>', ','].contains(c))
        .collect_vec();
    if chars.len() % 2 != 0 {
        chars.push(' ');
    }
    let seps = chars
        .into_iter()
        .tuples()
        .fold(Vec::new(), |mut acc, (c1, c2)| {
            if c1 == ':' && c2 == ':' {
                acc.push("::".to_string());
            }

            if ['<', '>', ','].contains(&c1) {
                acc.push(c1.to_string());
            }
            if ['<', '>', ','].contains(&c2) {
                acc.push(c2.to_string());
            }

            acc
        });

    // append_log(&seps);

    (segs, seps)
}

fn ident(i: &str) -> Result<syn::Type, syn::Error> {
    syn::parse_str(i)
}

// fn append_log(s: impl std::fmt::Debug) {
//     let mut file = std::fs::OpenOptions::new()
//         .append(true)
//         .open("build_logs.txt")
//         .unwrap();

//     file.write_all(format!("{:#?}", s).as_bytes()).unwrap();
//     file.write_all(b"\n").unwrap();
// }
