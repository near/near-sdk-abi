use anyhow::{Result, anyhow};
use convert_case::{Case, Casing};
use near_sdk_abi_impl::{generate_ext, read_abi};
use quote::{format_ident, quote};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

pub use near_sdk_abi_macros::near_abi_ext;

pub struct AbiFile {
    /// Path to the ABI JSON file.
    pub path: PathBuf,
    /// Contract name to be used for the resulting trait name.
    /// If missing will try to pull the name from ABI metadata and use `Ext<ContractName>`.
    pub contract_name: Option<String>,
    /// mod name to be used for the resulting ext mod.
    /// If missing will be derived by applying snake case to the contract name, e.g. ext_status_message.
    pub mod_name: Option<String>,
}

impl AbiFile {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        AbiFile {
            path: path.into(),
            contract_name: None,
            mod_name: None,
        }
    }
}

/// Configuration options for ABI code generation.
#[derive(Default)]
pub struct Generator {
    out_dir: Option<PathBuf>,
    abis: Vec<AbiFile>,
}

impl Generator {
    pub fn new(out_dir: PathBuf) -> Self {
        Generator {
            out_dir: Some(out_dir),
            abis: vec![],
        }
    }

    pub fn file(mut self, abi_file: AbiFile) -> Self {
        self.abis.push(abi_file);
        self
    }

    pub fn generate(self) -> Result<()> {
        let target: PathBuf = self.out_dir.map(Ok).unwrap_or_else(|| {
            env::var_os("OUT_DIR")
                .ok_or_else(|| anyhow!("OUT_DIR environment variable is not set"))
                .map(Into::into)
        })?;
        fs::create_dir_all(&target)?;

        for AbiFile {
            path,
            contract_name,
            mod_name,
        } in self.abis
        {
            let abi_path_no_ext = path.with_extension("");
            let abi_filename = abi_path_no_ext
                .file_name()
                .ok_or_else(|| anyhow!("{:?} is not a valid ABI path", path.display()))?;
            let rust_path = target.join(abi_filename).with_extension("rs");

            let near_abi = read_abi(&path);

            let contract_name = contract_name
                .as_ref()
                .map(|n| format_ident!("{}", n))
                .or_else(|| {
                    near_abi
                        .metadata
                        .name
                        .clone()
                        .map(|n| format_ident!("Ext{}", n.to_case(Case::UpperCamel)))
                })
                .ok_or_else(|| {
                    anyhow!(
                        "ABI file '{}' does not contain a contract name. Please supply the name via `file_with_name`.",
                        path.display()
                    )
                })?;

            let token_stream = generate_ext(
                near_abi,
                contract_name,
                mod_name.map(|n| format_ident!("{}", n)),
            );
            let token_stream = quote! {
                #![allow(unused_imports)]
                use serde::{Deserialize, Serialize};
                #token_stream
            };
            let syntax_tree = syn::parse_file(&token_stream.to_string()).unwrap();
            let formatted = prettyplease::unparse(&syntax_tree);

            let mut rust_file = File::create(rust_path)?;
            write!(rust_file, "{}", formatted)?;
        }

        Ok(())
    }
}
