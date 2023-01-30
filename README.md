# Obfustring

[<img alt="github" src="https://img.shields.io/badge/github-Retoon/obfustring-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Retoon/obfustring) [<img alt="crates.io" src="https://img.shields.io/crates/v/obfustring?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/obfustring) [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-obfustring-66a2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/obfustring/latest/obfustring)
[<img alt="license" src="https://img.shields.io/badge/license-MIT-ddfdf?style=for-the-badge&badge=" height="20">](https://github.com/Retoon/obfustring/blob/master/LICENSE)

This crate provides a obfuscation macro for string literals. This makes it easy to protect them from common reverse engineering attacks like string reference lookup in something like a debugger or hex editor.

A string literal `""` is given as the input and converted into a `[u8; str_size*2]` array literal that is then stored inside the binary. Every character is offset by a random amount. This offset is stored alongside the original data so it can be reconstructed.

## Installation

```toml
[dependencies]
obfustring = "0.5.0"
```

<br><br>

# Syntax & Usage

The crate provides a `obfustring!()` macro that takes in a single string literal.

```rs
use obfustring::obfustring;

let obfuscated_string = obfustring!("Hello obfustring!"); // <-- Won't show up in binaries or hex editors
let generic_string = String::from("Hello regular string!"); // <-- Will show up in binaries or hex editors

println!("obfuscated_string: {}", obfuscated_string);
println!("generic_string: {}", generic_string);
```

<br>

# Disclaimer
Note that you should **never** have any encryption/api keys or
sensetive data hardcoded into your program. Though this macro
would make it harder, it wouldn't absolutely hide it from
someone looking hard enough. **Stick to environment variables.**

<br><br>

## License

This project is licensed under the [MIT license].

[mit license]: https://github.com/Retoon/obfustring/blob/master/LICENSE
