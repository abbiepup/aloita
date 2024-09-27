use core::fmt::Display;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Error, Item, ItemFn, LitInt};

#[proc_macro_attribute]
pub fn startup(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message = "The `#[startup]` attribute can only be applied to `fn`s";

    match parse_macro_input!(item as Item) {
        Item::Fn(item_fn) => startup_impl(attr, item_fn),
        item => compile_error(item, message),
    }
}

#[proc_macro_attribute]
pub fn shutdown(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message = "The `#[shutdown]` attribute can only be applied to `fn`s";

    match parse_macro_input!(item as Item) {
        Item::Fn(item_fn) => shutdown_impl(attr, item_fn),
        item => compile_error(item, message),
    }
}

fn startup_impl(attr: TokenStream, item_fn: ItemFn) -> TokenStream {
    let ident = &item_fn.sig.ident;
    gen_func(&item_fn, attr, "constructor", quote! { #ident(); })
}

fn shutdown_impl(attr: TokenStream, item_fn: ItemFn) -> TokenStream {
    let ident = &item_fn.sig.ident;

    gen_func(
        &item_fn,
        attr,
        "destructor",
        quote! {
            extern "C" { fn atexit(function: unsafe extern "C" fn()); }
            unsafe extern "C" fn _onexit() { #ident(); }
            atexit(_onexit);
        },
    )
}

fn compile_error(tokens: impl ToTokens, message: impl Display) -> TokenStream {
    Error::new_spanned(tokens, message)
        .to_compile_error()
        .into()
}

const fn decimal_digit_length_of_usize() -> usize {
    let mut count = 0;
    let mut value = usize::MAX;

    while value > 0 {
        value /= 10;
        count += 1;
    }

    count
}

fn gen_func(
    function: &ItemFn,
    attr: TokenStream,
    section: &str,
    body: proc_macro2::TokenStream,
) -> TokenStream {
    let order = syn::parse::<LitInt>(attr).map_or_else(
        |_| String::new(),
        |lit| {
            let num = lit.base10_parse::<usize>().unwrap();
            format!(".{:0width$}", num, width = decimal_digit_length_of_usize())
        },
    );

    quote! {
        #function

        const _: () = {
            #[used]
            #[cfg_attr(target_os = "windows", link_section = concat!(".CRT$XCU.", #section, #order))]
            #[cfg_attr(target_os = "dragonfly", link_section = concat!(".init_array.", #section, #order))]
            #[cfg_attr(target_os = "openbsd", link_section = concat!(".init_array.", #section, #order))]
            #[cfg_attr(target_os = "freebsd", link_section = concat!(".init_array.", #section, #order))]
            #[cfg_attr(target_os = "netbsd", link_section = concat!(".init_array.", #section, #order))]
            #[cfg_attr(target_os = "android", link_section = concat!(".init_array.", #section, #order))]
            #[cfg_attr(target_os = "linux", link_section = concat!(".init_array.", #section, #order))]
            #[cfg_attr(target_os = "none", link_section = concat!(".init_array.", #section, #order))]
            #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
            static _DECL: unsafe extern "C" fn() = {
                #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = concat!(".text.", #section, #order))]
                unsafe extern "C" fn _decl() { #body } _decl
            };
        };
    }
    .into()
}
