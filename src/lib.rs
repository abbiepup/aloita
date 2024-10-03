use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, Error, Item, ItemFn, LitInt};

macro_rules! compiler_error {
    ($tokens:expr, $($arg:tt)*) => {
        Error::new_spanned($tokens, format!($($arg)*))
            .to_compile_error()
            .into()
    };
}

#[proc_macro_attribute]
pub fn startup(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message = "The `#[startup]` attribute can only be applied to `fn`s";

    match parse_macro_input!(item as Item) {
        Item::Fn(item_fn) => {
            let ident = &item_fn.sig.ident;

            let body = quote! {
                #ident();
            };

            gen_func(attr, &item_fn, body, "constructor")
        }
        item => compiler_error!(item, "{}", message),
    }
}

#[proc_macro_attribute]
pub fn shutdown(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message = "The `#[shutdown]` attribute can only be applied to `fn`s";

    match parse_macro_input!(item as Item) {
        Item::Fn(item_fn) => {
            let ident = &item_fn.sig.ident;

            let body = quote! {
                extern "C" { fn atexit(function: unsafe extern "C" fn()); }
                unsafe extern "C" fn onexit() { #ident(); }
                atexit(onexit);
            };

            gen_func(attr, &item_fn, body, "destructor")
        }
        item => compiler_error!(item, "{}", message),
    }
}

fn gen_func(
    attr: TokenStream,
    function: &ItemFn,
    body: proc_macro2::TokenStream,
    section: &str,
) -> TokenStream {
    let order =
        parse::<LitInt>(attr).map_or_else(|_| 0, |lit| lit.base10_parse::<usize>().unwrap());
    let order = format!(
        ".{:0width$}",
        order,
        width = usize::MAX.ilog10() as usize + 1
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
            static DECL: unsafe extern "C" fn() = {
                #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = concat!(".text.", #section, #order))]
                unsafe extern "C" fn decl() { #body } decl
            };
        };
    }
    .into()
}
