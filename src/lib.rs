//! # env-cast
//!
//! [`env_cast!()`](env_cast!) reads an environment variable just like [`env!()`](env!), but parses it into a specific type.
//!
//! Supported types are currently
//! `i8, u8, i16, u16, i32, u32, i64, u64, f32, f64`.
//!
//! ## Example
//! ```rust
//! use env_cast::env_cast;
//! let PKG_MAJOR: u32 = env_cast!("CARGO_PKG_VERSION_MAJOR" as u32);
//! ```
//!

use std::str::FromStr;

use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned};

/// Reads an environment variable just like [`env!("XXX")`](env!), but parses it into a specific type.
///
/// # Example
/// ```
/// use env_cast::env_cast;
/// let PKG_MAJOR: u32 = env_cast!("CARGO_PKG_VERSION_MAJOR" as u32);
/// ```
///
#[proc_macro]
pub fn env_cast(args: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let cast = parse_macro_input!(args as syn::Expr);

    let (var, ty) = if let syn::Expr::Cast(cast) = cast {
        (cast.expr, cast.ty)
    } else {
        return compile_error("Must be of the form <var> as <type>", cast.span());
    };

    let lit = if let syn::Expr::Lit(lit) = &*var {
        lit
    } else {
        return compile_error("Must be a string literal", var.span());
    };

    let lit_str = if let syn::Lit::Str(lit_str) = &lit.lit {
        lit_str.value()
    } else {
        return compile_error("Must be a string literal", lit.span());
    };

    let cast_as = match &*ty {
        syn::Type::Path(path) => {
            if path.path.is_ident("i8") {
                CastAs::I8
            } else if path.path.is_ident("u8") {
                CastAs::U8
            } else if path.path.is_ident("i16") {
                CastAs::I16
            } else if path.path.is_ident("u16") {
                CastAs::U16
            } else if path.path.is_ident("i32") {
                CastAs::I32
            } else if path.path.is_ident("u32") {
                CastAs::U32
            } else if path.path.is_ident("i64") {
                CastAs::I64
            } else if path.path.is_ident("u64") {
                CastAs::U64
            } else if path.path.is_ident("f32") {
                CastAs::F32
            } else if path.path.is_ident("f64") {
                CastAs::F64
            } else {
                return compile_error(
                    "Must be one of i8, u8, i16, u16, i32, u32, i64, u64, f32, f64",
                    ty.span(),
                );
            }
        }
        _ => {
            return compile_error(
                "Must be one of i8, u8, i16, u16, i32, u32, i64, u64, f32, f64",
                ty.span(),
            );
        }
    };

    let val_str = match std::env::var(lit_str) {
        Ok(val) => val,
        Err(err) => {
            return compile_error(&format!("Failed to read environment: {}", err), lit.span())
        }
    };

    match cast_as {
        CastAs::I8 => parse::<i8>(&val_str, lit.span()),
        CastAs::U8 => parse::<u8>(&val_str, lit.span()),
        CastAs::I16 => parse::<i16>(&val_str, lit.span()),
        CastAs::U16 => parse::<u16>(&val_str, lit.span()),
        CastAs::I32 => parse::<i32>(&val_str, lit.span()),
        CastAs::U32 => parse::<u32>(&val_str, lit.span()),
        CastAs::I64 => parse::<i64>(&val_str, lit.span()),
        CastAs::U64 => parse::<u64>(&val_str, lit.span()),
        CastAs::F32 => parse::<f32>(&val_str, lit.span()),
        CastAs::F64 => parse::<f64>(&val_str, lit.span()),
    }
}

fn compile_error(msg: &str, span: Span) -> proc_macro::TokenStream {
    quote_spanned! {span=>
        compile_error!(#msg);
    }
    .into()
}

enum CastAs {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

fn parse<T: FromStr + ToTokens>(str: &str, span: Span) -> proc_macro::TokenStream {
    match str.parse::<T>() {
        Ok(val) => quote! {#val}.into(),
        Err(_) => compile_error("Environment variable not parseable", span),
    }
}
