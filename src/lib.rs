use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, ItemFn, LitInt};

#[proc_macro_attribute]
pub fn startup(attr: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);
    let ident = &function.sig.ident;

    gen_func(&function, attr, "ctor", quote! { #ident(); })
}

#[proc_macro_attribute]
pub fn shutdown(attr: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);
    let ident = &function.sig.ident;

    gen_func(&function, attr, "dtor", quote! {
        extern "C" { fn atexit(function: unsafe extern "C" fn()); }
        unsafe extern "C" fn _onexit() { #ident(); }
        atexit(_onexit);
    })
}

fn gen_func(
    function: &ItemFn,
    attr: TokenStream,
    subsection: &str,
    body: proc_macro2::TokenStream,
) -> TokenStream {
    let order = parse::<LitInt>(attr).map_or_else(|_| String::new(), |lit| format!(".{}", lit));

    quote! {
        #function

        const _: () = {
            #[used]
            #[cfg_attr(
                any(
                    target_os = "none",
                    target_os = "linux",
                    target_os = "netbsd",
                    target_os = "android",
                    target_os = "freebsd",
                    target_os = "openbsd",
                    target_os = "dragonfly",
                ), 
                link_section = concat!(".init_array.", #subsection, #order)
            )]
            #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
            #[cfg_attr(target_os = "windows", link_section = concat!(".CRT$XCU.", #subsection, #order))]
            static _DECL: unsafe extern "C" fn() = {
                #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = concat!(".text.", #subsection, #order))]
                unsafe extern "C" fn _decl() { #body } _decl
            };
        };
    }
    .into()
}
