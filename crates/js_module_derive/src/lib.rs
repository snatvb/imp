use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprClosure, Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

enum ModuleItem {
    Flat {
        name: LitStr,
        func: Expr,
    },
    Sub {
        name: LitStr,
        methods: Vec<(Ident, Expr)>,
    },
    Declare {
        closure: ExprClosure,
    },
    Evaluate {
        closure: ExprClosure,
    },
}

struct ModuleInput {
    name: Ident,
    items: Vec<ModuleItem>,
}

impl Parse for ModuleInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut items = Vec::new();

        while !input.is_empty() {
            let item: ModuleItem = input.parse()?;
            items.push(item);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(ModuleInput { name, items })
    }
}

impl Parse for ModuleItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            let name: LitStr = input.parse()?;
            input.parse::<Token![=>]>()?;

            if input.peek(syn::token::Brace) {
                let content;
                syn::braced!(content in input);
                let methods: Punctuated<(Ident, Expr), Token![,]> =
                    content.parse_terminated(parse_method, Token![,])?;
                Ok(ModuleItem::Sub {
                    name,
                    methods: methods.into_iter().collect(),
                })
            } else {
                let func: Expr = input.parse()?;
                Ok(ModuleItem::Flat { name, func })
            }
        } else if lookahead.peek(kw::declare) {
            input.parse::<kw::declare>()?;
            input.parse::<Token![:]>()?;
            let closure: ExprClosure = input.parse()?;
            Ok(ModuleItem::Declare { closure })
        } else if lookahead.peek(kw::evaluate) {
            input.parse::<kw::evaluate>()?;
            input.parse::<Token![:]>()?;
            let closure: ExprClosure = input.parse()?;
            Ok(ModuleItem::Evaluate { closure })
        } else {
            Err(lookahead.error())
        }
    }
}

fn parse_method(input: ParseStream) -> syn::Result<(Ident, Expr)> {
    let method: Ident = input.parse()?;
    input.parse::<Token![=>]>()?;
    let func: Expr = input.parse()?;
    Ok((method, func))
}

mod kw {
    syn::custom_keyword!(declare);
    syn::custom_keyword!(evaluate);
}

fn extract_ident(pat: &syn::Pat) -> Option<&Ident> {
    match pat {
        syn::Pat::Ident(pat_ident) => Some(&pat_ident.ident),
        _ => None,
    }
}

#[proc_macro]
pub fn impl_module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ModuleInput);
    let expanded = expand(input);
    TokenStream::from(expanded)
}

fn expand(input: ModuleInput) -> proc_macro2::TokenStream {
    let name = &input.name;

    let flat_items: Vec<_> = input
        .items
        .iter()
        .filter_map(|item| match item {
            ModuleItem::Flat { name, func } => Some((name, func)),
            _ => None,
        })
        .collect();

    let sub_items: Vec<_> = input
        .items
        .iter()
        .filter_map(|item| match item {
            ModuleItem::Sub { name, methods } => Some((name, methods)),
            _ => None,
        })
        .collect();

    let declare_closure = input.items.iter().find_map(|item| match item {
        ModuleItem::Declare { closure } => Some(closure),
        _ => None,
    });

    let evaluate_closure = input.items.iter().find_map(|item| match item {
        ModuleItem::Evaluate { closure } => Some(closure),
        _ => None,
    });

    let declare_fn = gen_declare(&flat_items, &sub_items, declare_closure);
    let evaluate_fn = gen_evaluate(&flat_items, &sub_items, evaluate_closure);

    quote! {
        pub struct #name;

        impl js_core::js::module::ModuleDef for #name {
            #declare_fn
            #evaluate_fn
        }
    }
}

fn gen_declare(
    flat_items: &[(&LitStr, &Expr)],
    sub_items: &[(&LitStr, &Vec<(Ident, Expr)>)],
    declare_closure: Option<&ExprClosure>,
) -> proc_macro2::TokenStream {
    let all_names: Vec<_> = flat_items
        .iter()
        .map(|(name, _)| name)
        .chain(sub_items.iter().map(|(name, _)| name))
        .collect();

    if let Some(closure) = declare_closure {
        let closure_params = &closure.inputs;
        let closure_body = &closure.body;

        let first_param = closure_params.first().and_then(extract_ident);
        let second_param = closure_params.iter().nth(1).and_then(extract_ident);

        if let (Some(first), Some(declare_all_fn_name)) = (first_param, second_param) {
            let declare_calls = all_names.iter().map(|name| {
                quote! {
                    decl.declare(#name)?;
                }
            });

            quote! {
                fn declare<'js>(decl: &js_core::js::module::Declarations<'js>) -> js_core::js::Result<()> {
                    fn #declare_all_fn_name(decl: &js_core::js::module::Declarations<'_>) -> js_core::js::Result<()> {
                        #(#declare_calls)*
                        Ok(())
                    }
                    let #first = decl;
                    #closure_body
                }
            }
        } else {
            quote! {
                fn declare<'js>(decl: &js_core::js::module::Declarations<'js>) -> js_core::js::Result<()> {
                    #closure
                }
            }
        }
    } else {
        let declare_calls = all_names.iter().map(|name| {
            quote! {
                decl.declare(#name)?;
            }
        });

        quote! {
            fn declare<'js>(decl: &js_core::js::module::Declarations<'js>) -> js_core::js::Result<()> {
                #(#declare_calls)*
                decl.declare("default")?;
                Ok(())
            }
        }
    }
}

fn gen_evaluate(
    flat_items: &[(&LitStr, &Expr)],
    sub_items: &[(&LitStr, &Vec<(Ident, Expr)>)],
    evaluate_closure: Option<&ExprClosure>,
) -> proc_macro2::TokenStream {
    if let Some(closure) = evaluate_closure {
        let closure_params = &closure.inputs;
        let closure_body = &closure.body;

        let first_param = closure_params.first().and_then(extract_ident);
        let second_param = closure_params.iter().nth(1).and_then(extract_ident);
        let third_param = closure_params.iter().nth(2).and_then(extract_ident);

        if let (Some(first), Some(second), Some(_export_all_fn_name)) =
            (first_param, second_param, third_param)
        {
            let export_all_body = gen_export_all_body(flat_items, sub_items);

            quote! {
                fn evaluate<'js>(ctx: &js_core::js::Ctx<'js>, exports: &js_core::js::module::Exports<'js>) -> js_core::js::Result<()> {
                    fn export_all<'js>(ctx: &js_core::js::Ctx<'js>, exports: &js_core::js::module::Exports<'js>) -> js_core::js::Result<js_core::js::Object<'js>> {
                        #export_all_body
                        Ok(ns)
                    }
                    let #first = ctx;
                    let #second = exports;
                    #closure_body
                }
            }
        } else {
            quote! {
                fn evaluate<'js>(ctx: &js_core::js::Ctx<'js>, exports: &js_core::js::module::Exports<'js>) -> js_core::js::Result<()> {
                    #closure
                }
            }
        }
    } else {
        let export_body = gen_export_body_no_return(flat_items, sub_items);

        quote! {
            fn evaluate<'js>(ctx: &js_core::js::Ctx<'js>, exports: &js_core::js::module::Exports<'js>) -> js_core::js::Result<()> {
                #export_body
                exports.export("default", ns)?;
                Ok(())
            }
        }
    }
}

fn gen_export_body_no_return(
    flat_items: &[(&LitStr, &Expr)],
    sub_items: &[(&LitStr, &Vec<(Ident, Expr)>)],
) -> proc_macro2::TokenStream {
    let flat_exports = flat_items.iter().map(|(name, func)| {
        quote! {
            let val = js_core::js::IntoJs::into_js(#func, ctx)?;
            ns.set(#name, val.clone())?;
            exports.export(#name, val)?;
        }
    });

    let sub_exports = sub_items.iter().map(|(name, methods)| {
        let method_sets = methods.iter().map(|(method, func)| {
            quote! {
                obj.set(stringify!(#method), #func)?;
            }
        });

        quote! {
            let obj = js_core::js::Object::new(ctx.clone())?;
            #(#method_sets)*
            ns.set(#name, obj.clone())?;
            exports.export(#name, obj)?;
        }
    });

    quote! {
        let ns = js_core::js::Object::new(ctx.clone())?;
        #(#flat_exports)*
        #(#sub_exports)*
    }
}

fn gen_export_all_body(
    flat_items: &[(&LitStr, &Expr)],
    sub_items: &[(&LitStr, &Vec<(Ident, Expr)>)],
) -> proc_macro2::TokenStream {
    let flat_exports = flat_items.iter().map(|(name, func)| {
        quote! {
            let val = js_core::js::IntoJs::into_js(#func, ctx)?;
            ns.set(#name, val.clone())?;
            exports.export(#name, val)?;
        }
    });

    let sub_exports = sub_items.iter().map(|(name, methods)| {
        let method_sets = methods.iter().map(|(method, func)| {
            quote! {
                obj.set(stringify!(#method), #func)?;
            }
        });

        quote! {
            let obj = js_core::js::Object::new(ctx.clone())?;
            #(#method_sets)*
            ns.set(#name, obj.clone())?;
            exports.export(#name, obj)?;
        }
    });

    quote! {
        let ns = js_core::js::Object::new(ctx.clone())?;
        #(#flat_exports)*
        #(#sub_exports)*
    }
}
