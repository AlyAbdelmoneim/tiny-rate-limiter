use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{ItemFn, Lit, Meta, Token, parse_macro_input};

struct RateLimitArgs {
    key: String,
    capacity: u32,
    refill_rate: f64,
}

impl Parse for RateLimitArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;
        let mut key = None;
        let mut capacity = None;

        let mut refill_rate = None;
        for arg in args {
            if let Meta::NameValue(nv) = arg {
                let ident = nv.path.get_ident().unwrap().to_string();

                if let Expr::Lit(expr_lit) = &nv.value {
                    match &expr_lit.lit {
                        Lit::Str(s) if ident == "key" => key = Some(s.value()),
                        Lit::Int(i) if ident == "capacity" => capacity = Some(i.base10_parse()?),
                        Lit::Float(f) if ident == "refill_rate" => {
                            refill_rate = Some(f.base10_parse()?)
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(RateLimitArgs {
            key: key.ok_or_else(|| syn::Error::new(input.span(), "Missing key"))?,
            capacity: capacity.ok_or_else(|| syn::Error::new(input.span(), "Missing capacity"))?,
            refill_rate: refill_rate
                .ok_or_else(|| syn::Error::new(input.span(), "Missing refill_rate"))?,
        })
    }
}

#[proc_macro_attribute]
pub fn rate_limit(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as RateLimitArgs);
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;

    let key_name = args.key;
    let capacity = args.capacity;
    let refill_rate = args.refill_rate;

    let expanded = quote! {
        #fn_vis #fn_sig {
            let key = #key_name;

            let is_exceeded = rate_limiter_core::limiter::RATE_LIMITER.lock().unwrap()
                .try_consume(key, #capacity, #refill_rate)
                .is_err();

            if is_exceeded {
                panic!("Rate Limit Exceeded");
            }

            #fn_block

        }
    };

    TokenStream::from(expanded)
}
