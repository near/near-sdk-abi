use std::path::{Path, PathBuf};

use near_abi::{AbiRoot, AbiType};
use quote::{format_ident, quote};
use schemafy_lib::{Expander, Generator};

pub fn generate_ext(
    near_abi: AbiRoot,
    contract_name: proc_macro2::Ident,
    mod_name: Option<proc_macro2::Ident>,
) -> proc_macro2::TokenStream {
    let schema_json = serde_json::to_string(&near_abi.body.root_schema).unwrap();

    let generator = Generator::builder().with_input_json(&schema_json).build();
    let (mut token_stream, schema) = generator.generate_with_schema();
    let mut expander = Expander::new(None, "", &schema);

    let methods = near_abi
        .body
        .functions
        .iter()
        .map(|m| {
            let name = format_ident!("{}", m.name);
            let result_type = m
                .result
                .clone()
                .map(|r_param| {
                    let r_type = match &r_param {
                        AbiType::Json { type_schema } => {
                            expand_subschema(&mut expander, type_schema)
                        }
                        AbiType::Borsh { type_schema: _ } => {
                            panic!("Borsh is currently unsupported")
                        }
                    };
                    quote! { -> #r_type }
                })
                .unwrap_or_else(|| quote! {});
            let args = m
                .params
                .iter()
                .map(|a_param| {
                    let a_type = match &a_param.typ {
                        AbiType::Json { type_schema } => {
                            expand_subschema(&mut expander, type_schema)
                        }
                        AbiType::Borsh { type_schema: _ } => {
                            panic!("Borsh is currently unsupported")
                        }
                    };
                    let a_name = format_ident!("{}", &a_param.name);
                    quote! { #a_name: #a_type }
                })
                .collect::<Vec<_>>();
            quote! { fn #name(&self, #(#args),*) #result_type; }
        })
        .collect::<Vec<_>>();

    let ext_contract = mod_name.map_or_else(
        || quote! { #[near_sdk::ext_contract] },
        |n| quote! { #[near_sdk::ext_contract(#n)] },
    );

    token_stream.extend(quote! {
        #ext_contract
        pub trait #contract_name {
            #(#methods)*
        }
    });

    token_stream
}

pub fn read_abi(abi_path: impl AsRef<Path>) -> AbiRoot {
    let abi_path = if abi_path.as_ref().is_relative() {
        let crate_root = get_crate_root().unwrap();
        crate_root.join(&abi_path)
    } else {
        PathBuf::from(abi_path.as_ref())
    };

    let abi_json = std::fs::read_to_string(&abi_path)
        .unwrap_or_else(|err| panic!("Unable to read `{}`: {}", abi_path.to_string_lossy(), err));

    serde_json::from_str::<AbiRoot>(&abi_json).unwrap_or_else(|err| {
        panic!(
            "Cannot parse `{}` as ABI: {}",
            abi_path.to_string_lossy(),
            err
        )
    })
}

pub fn get_crate_root() -> std::io::Result<PathBuf> {
    if let Ok(path) = std::env::var("CARGO_MANIFEST_DIR") {
        return Ok(PathBuf::from(path));
    }

    let current_dir = std::env::current_dir()?;

    for p in current_dir.ancestors() {
        if std::fs::read_dir(p)?
            .into_iter()
            .filter_map(Result::ok)
            .any(|p| p.file_name().eq("Cargo.toml"))
        {
            return Ok(PathBuf::from(p));
        }
    }

    Ok(current_dir)
}

fn schemars_schema_to_schemafy(schema: &schemars::schema::Schema) -> schemafy_lib::Schema {
    let schema_json = serde_json::to_string(&schema).unwrap();
    serde_json::from_str(&schema_json).unwrap_or_else(|err| {
        panic!(
            "Could not convert schemars schema to schemafy model: {}",
            err
        )
    })
}

fn expand_subschema(expander: &mut Expander, schema: &schemars::schema::Schema) -> syn::Ident {
    let schemafy_schema = schemars_schema_to_schemafy(schema);
    format_ident!("{}", expander.expand_type_from_schema(&schemafy_schema).typ)
}
