use proc_macro::TokenStream;

#[proc_macro]
pub fn generate(args: TokenStream) -> TokenStream {
    let args = utils::Args::parse(args.to_string());
    let generated = utils::generate(args);
    TokenStream::from(generated)
}
