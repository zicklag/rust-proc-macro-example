extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// This defines a new procedural macro. Every procedural macro takes an input TokenStream and
/// returns a token stream.
///
/// The input token stream will contain all the Rust code that you pass in as an argument to the
/// macro. In this case, we just ignore the input because we just want to print a message.
///
/// Macros can also output any Rust code they want. In this case we output a println statement. The
/// Rust code that is output by the macro will be run when you run the program.
#[proc_macro]
pub fn compile_time_print_hello(_input: TokenStream) -> TokenStream {
    // This will be printed while you are compiling the crate
    println!("Hello from a macro while compiling!!!");

    // The quote macro let's us create a TokenStream by typing rust code into the macro
    quote! {
        // This will be printed when you run the program, not when you compile the program
        println!("Hello from a macro while running!!")

    }
    // We have to call into to convert the return value of the quote macro to the type of tokens
    // stream that the proc macro returns.
    .into()
}
