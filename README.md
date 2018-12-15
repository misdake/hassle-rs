hassle-rs
========
[![Latest version](https://img.shields.io/crates/v/hassle-rs.svg)](https://crates.io/crates/hassle-rs)
[![Documentation](https://docs.rs/hassle-rs/badge.svg)](https://docs.rs/hassle-rs)
[![](https://tokei.rs/b1/github/Jasper-Bekkers/hassle-rs)](https://github.com/Jasper-Bekkers/hassle-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

This crate provides an FFI layer and idiomatic rust wrappers for the new [DirectXShaderCompiler](https://github.com/Microsoft/DirectXShaderCompiler) library.

- [Documentation](https://docs.rs/hassle-rs)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hassle-rs = "0.1.6"
```

and add this to your crate root:

```rust
extern crate hassle;
```

Then acquire `dxcompiler.dll` directly from [AppVeyor](https://ci.appveyor.com/project/antiagainst/directxshadercompiler/branch/master/artifacts) or compile it from source according to the instructions in the [DirectXShaderCompiler](https://github.com/Microsoft/DirectXShaderCompiler) GitHub repository and make sure it's in the executable enviroment.

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, shall be licensed as above, without any additional terms or conditions.

Contributions are always welcome; please look at the [issue tracker](https://github.com/Jasper-Bekkers/hassle-rs/issues) to see what known improvements are documented.
