use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Error, ItemFn};

#[proc_macro_attribute]
pub fn startup(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);

    if !function.sig.inputs.is_empty() {
        return Error::new_spanned(
            &function,
            "Functions marked with #[startup] must have zero arguments.",
        )
        .to_compile_error()
        .into();
    }

    let ident = &function.sig.ident;

    quote! {
        #function

        const _: () = {
            #[used]
            #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
            #[cfg_attr(target_os = "none", link_section = ".init_array")]
            #[cfg_attr(target_os = "linux", link_section = ".init_array")]
            #[cfg_attr(target_os = "netbsd", link_section = ".init_array")]
            #[cfg_attr(target_os = "android", link_section = ".init_array")]
            #[cfg_attr(target_os = "freebsd", link_section = ".init_array")]
            #[cfg_attr(target_os = "openbsd", link_section = ".init_array")]
            #[cfg_attr(target_os = "dragonfly", link_section = ".init_array")]
            #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
            static _DECL: unsafe extern "C" fn() = {
                #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
                unsafe extern "C" fn _decl() { 
                    #ident(); 
                }
                _decl
            };
        };
    }
    .into()
}

#[proc_macro_attribute]
pub fn atexit(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);

    quote! {
        #function
    }
    .into()
}


