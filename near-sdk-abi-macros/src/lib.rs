use convert_case::{Case, Casing};
use near_sdk_abi::__private::{generate_ext, read_abi};
use quote::format_ident;
use std::path::PathBuf;

#[proc_macro]
pub fn near_abi_ext(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let abi_def = syn::parse_macro_input!(tokens as AbiDef);

    let near_abi = read_abi(PathBuf::from(&abi_def.abi_path.value()));

    let contract_name = abi_def.name.map(|n| format_ident!("{}", n)).or_else(|| {
        near_abi
            .metadata
            .name
            .clone()
            .map(|n| format_ident!("Ext{}", n.to_case(Case::UpperCamel)))
    });
    let contract_name = if let Some(name) = contract_name {
        name
    } else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "ABI metadata does not contain a contract name. Please supply the name as a `name` parameter to the macro.",
        )
        .to_compile_error()
        .into();
    };

    generate_ext(near_abi, contract_name).into()
}

struct AbiDef {
    /// Contract name to be used for the resulting trait name.
    /// If missing will try to pull the name from ABI metadata and use `Ext<ContractName>`.
    name: Option<String>,
    /// Path to the ABI file.
    abi_path: syn::LitStr,
}

impl syn::parse::Parse for AbiDef {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let name = if input.peek(syn::Ident) {
            let name_ident: syn::Ident = input.parse()?;
            if name_ident != "name" {
                return Err(syn::Error::new(
                    name_ident.span(),
                    format!("Expected `name`, but got `{}`", name_ident),
                ));
            }
            input.parse::<syn::Token![:]>()?;
            Some(input.parse::<syn::Ident>()?.to_string())
        } else {
            None
        };
        Ok(AbiDef {
            name,
            abi_path: input.parse()?,
        })
    }
}
