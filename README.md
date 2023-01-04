<!-- markdownlint-disable MD014 -->

<div align="center">

  <h1><code>near-sdk-abi</code></h1>

  <p>
    <strong>Utility library for making typesafe cross-contract calls with <a href="https://github.com/near/near-sdk-rs">near-sdk-rs</a> smart contracts</strong>
  </p>

  <p>
    <a href="https://github.com/near/near-sdk-abi/actions/workflows/test.yml?query=branch%3Amain"><img src="https://github.com/near/near-sdk-abi/actions/workflows/test.yml/badge.svg" alt="Github CI Build" /></a>
    <a href="https://crates.io/crates/near-sdk-abi"><img src="https://img.shields.io/crates/v/near-sdk-abi.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/near-sdk-abi"><img src="https://img.shields.io/crates/d/near-sdk-abi.svg?style=flat-square" alt="Downloads" /></a>
  </p>

</div>

## Release notes

**Release notes and unreleased changes can be found in the [CHANGELOG](CHANGELOG.md)**

## Usage

This crate supports two sets of APIs for users with different needs:
* **Macro-driven**. Gives you a cross-contract binding in a single macro invocation.
* **Generation-based**. Gives you more control and is transparent about what code you end up using, but requires more setup.

### Macro API

Checkout the [`delegator-macro`](https://github.com/near/near-sdk-abi/tree/main/examples/delegator-macro) example for a standalone smart contract using macro API to make a cross-contract call.

To generate a trait named `ContractName` with ext interface named `ext_name` based on ABI located at `path/to/abi.json` (relative to the current file's directory):

```rust
near_abi_ext! { mod ext_name trait ContractName for "path/to/abi.json" }
```

Now, assuming you have an `ext_account_id: near_sdk::AccountId` representing the contract account id, you can make a cross-contract call like this:

```rust
let promise = ext_adder::ext(ext_account_id).my_method_name(arg1, arg2);
```

### Generation API

Checkout the [`delegator-generation`](https://github.com/near/near-sdk-abi/tree/main/examples/delegator-generation) example for a standalone project using generation API to make a cross-contract call.

First, we need our package to have a `build.rs` file that runs the generation step. The following snippet will generate the contract trait in `abi.rs` under `path/to/out/dir`:

```rust
fn main() -> anyhow::Result<()> {
    near_sdk_abi::Generator::new("path/to/out/dir".into())
        .file(near_sdk_abi::AbiFile::new("path/to/abi.json"))
        .generate()?;
    Ok(())
}
```

The resulting file, however, is not included in your source set by itself. You have to include it manually; the recommended way is to create a mod with a custom path:

```rust
#[path = "path/to/out/dir/abi.rs"]
mod mymod;
```

Now, assuming you have an `ext_account_id: near_sdk::AccountId` representing the contract account id, you can make a cross-contract call like this:

```rust
let promise = ext_adder::ext(ext_account_id).my_method_name(arg1, arg2);
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
