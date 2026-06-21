use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(JsError, attributes(js))]
pub fn derive_js_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("JsError can only be derived for enums"),
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let attr = variant
            .attrs
            .iter()
            .find(|a| a.path().is_ident("js"))
            .expect("Each variant must have #[js(...)] attribute");

        let kind = attr
            .parse_args::<syn::Ident>()
            .expect("Expected #[js(system)], #[js(type_error)], or #[js(error)]");
            let kind_str = kind.to_string();

            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let body = match kind_str.as_str() {
                        "system" => quote! { js_core::error::JsError::into_js(e, ctx) },
                        "type_error" => quote! { js_core::error::make_type_error(ctx, e.to_string()) },
                        "range_error" => quote! { js_core::error::make_range_error(ctx, e.to_string()) },
                        "error" => quote! { js_core::error::make_error(ctx, e.to_string()) },
                        "abort" => quote! { js_core::error::make_abort_error(ctx, e.to_string()) },
                        _ => panic!("Expected #[js(system)], #[js(type_error)], #[js(range_error)], #[js(error)], or #[js(abort)]"),
                    };
                    quote! {
                        #name::#variant_name(e) => #body,
                    }
                }
                _ => panic!("JsError variants must have exactly one unnamed field"),
            }
        });

    let expanded = quote! {
        impl js_core::error::JsError for #name {
            fn into_js<'js>(self, ctx: &js_core::js::Ctx<'js>) -> js_core::js::Result<js_core::js::Value<'js>> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
