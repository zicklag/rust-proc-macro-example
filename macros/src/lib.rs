extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn compile_time_print_hello(_input: TokenStream) -> TokenStream {
    println!("Hello");

    TokenStream::new()
}
