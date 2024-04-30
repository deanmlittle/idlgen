# IDLGen

Generate CPI and Introspection SDK files or Rust Crates using nothing but an Anchor IDL

# Usage

Simply install the CLI from source with the following command:
`cargo install --git https://github.com/deanmlittle/anchor-idlgen`

Then run it against an IDL file like so:
`idlgen --filename jupiter.json --pacakge crate`

You can explore the different build options using the help command:
`idlgen --help`