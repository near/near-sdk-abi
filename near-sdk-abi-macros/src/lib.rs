use near_sdk_abi::__private::{generate_ext, read_abi};
use std::path::PathBuf;

#[proc_macro]
pub fn near_abi_ext(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let abi_def = syn::parse_macro_input!(tokens as AbiDef);
    let near_abi = read_abi(PathBuf::from(&abi_def.path.value()));

    generate_ext(near_abi, abi_def.trait_name, Some(abi_def.mod_name)).into()
}

struct AbiDef {
    /// Resulting mod name.
    mod_name: syn::Ident,
    /// Resulting ext contract trait name.
    trait_name: syn::Ident,
    /// Path to the ABI file.
    path: syn::LitStr,
}

impl syn::parse::Parse for AbiDef {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse::<syn::Token![mod]>()?;
        let mod_name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![trait]>()?;
        let trait_name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![for]>()?;
        let path = input.parse()?;
        Ok(AbiDef {
            mod_name,
            trait_name,
            path,
        })
    }
}
