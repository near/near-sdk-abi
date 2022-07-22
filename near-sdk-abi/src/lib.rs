use __private::{generate_ext, read_abi};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

// Private functions shared between macro & generation APIs, not stable to be used.
#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;

/// Configuration options for ABI code generation.
#[derive(Default)]
pub struct Generator {
    out_dir: Option<PathBuf>,
    abis: Vec<(PathBuf, Option<String>)>,
}

impl Generator {
    pub fn new(out_dir: PathBuf) -> Self {
        Generator {
            out_dir: Some(out_dir),
            abis: vec![],
        }
    }

    pub fn file(mut self, path: impl AsRef<Path>) -> Self {
        self.abis.push((path.as_ref().into(), None));
        self
    }

    pub fn file_with_name(mut self, path: impl AsRef<Path>, name: String) -> Self {
        self.abis.push((path.as_ref().into(), Some(name)));
        self
    }

    pub fn generate(self) -> Result<()> {
        let target: PathBuf = self.out_dir.map(Ok).unwrap_or_else(|| {
            env::var_os("OUT_DIR")
                .ok_or_else(|| anyhow!("OUT_DIR environment variable is not set"))
                .map(Into::into)
        })?;
        fs::create_dir_all(&target)?;

        for (abi_path, name) in self.abis {
            let abi_path_no_ext = abi_path.with_extension("");
            let abi_filename = abi_path_no_ext
                .file_name()
                .ok_or_else(|| anyhow!("{:?} is not a valid ABI path", abi_path.display()))?;
            let rust_path = target.join(abi_filename).with_extension("rs");

            let near_abi = read_abi(&abi_path);

            let contract_name = name
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
                        abi_path.display()
                    )
                })?;

            let token_stream = generate_ext(near_abi, contract_name);
            let token_stream = quote! {
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
