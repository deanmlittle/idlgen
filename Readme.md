# IDLGen

Generate CPI and Introspection SDK files or Rust Crates using nothing but an Anchor IDL

# Usage

Simply install the CLI from source with the following command:
```sh
cargo install --git https://github.com/deanmlittle/idlgen
```

Then run it against an IDL file like so:
```sh
idlgen --filename jupiter.json --package crate
```

You can explore the different build options using the help command:
```sh
idlgen --help
```

# Credits

A special thanks to [@JuanMarchetto](https://github.com/JuanMarchetto) for his IDL types!
