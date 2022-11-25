use proc_macro::TokenStream;

mod elapsed;

/// A proc macro for calculating the elapsed time of the function
#[proc_macro_attribute]
#[cfg(not(test))]
pub fn elapsed(args: TokenStream, func: TokenStream) -> TokenStream {
    elapsed::elapsed(args, func)
}
