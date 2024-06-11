use convert_case::{Case, Casing};

use crate::{generators::{accounts::make_accounts, cpi::{make_cpi_accounts, make_cpi_ctxs}, events::make_events, i11n::make_i11n_ctxs, rpc::make_rpc_accounts}, types::{Instruction, Type, Types}, IDL};

pub fn make_defined_types(idl: &IDL) -> String {
    idl.types.iter().map(|t| {
        if t.kind.kind == "enum" {
            make_defined_types_enum(t.clone())
        } else if t.kind.kind == "struct" {
            make_defined_types_struct(t.clone())
        } else {
            panic!("Unknown defined type: {}", t.kind.kind);
        }
    }).collect::<Vec<String>>().join("\n\n")
}

pub fn make_defined_types_enum(t: Types) -> String {
    format!("#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum {} {{
{}
}}", t.name, t.kind.variants.clone().unwrap_or(vec![]).iter().map(|n| format!("    {}", n.name.to_case(Case::Pascal))).collect::<Vec<String>>().join(",\n"))
}

pub fn make_defined_types_struct(t: Types) -> String {
    format!("#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct {} {{
{}
}}", t.name, make_defined_types_fields(t.clone()))
}

pub fn make_defined_types_fields(t: Types) -> String {
    t.kind.fields.clone().unwrap_or(vec![]).iter().map(|f| {
        format!("    pub {}: {},", f.name.to_case(Case::Snake), f.kind.to_string())
    }).collect::<Vec<String>>().join("\n")
}

pub fn make_ixs(idl: &IDL) -> String {
    format!("pub mod instructions {{
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use anchor_lang::Discriminator;
    use super::*;

{}        
}}",
        idl.instructions.iter().map(|ix| {
        let ix_name_pascal =  ix.name.to_case(Case::Pascal);
        format!("    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct {} {{
{}
    }}
    
    impl anchor_lang::InstructionData for {} {{
        fn data(&self) -> Vec<u8> {{
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }}
    
        fn write_to(&self, mut data: &mut Vec<u8>) {{
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }}
    }}
    ", 
    ix_name_pascal, 
    ix.args.iter().map(|a| format!("        pub {}: {},", a.name.to_case(Case::Snake), a.kind.to_string())).collect::<Vec<String>>().join("\n"),
    ix_name_pascal
    )
        }).collect::<Vec<String>>().join("\n\n")
    )
}

pub fn make_ix_args(ix: &Instruction) -> String {
    if ix.args.len() > 0 {
        format!(",\n{}", ix.args.iter().map(|a| format!("        {}: {}", a.name.to_case(Case::Snake), a.kind.to_string())).collect::<Vec<String>>().join(",\n"))
    } else {
        String::new()
    }
}

pub fn make_ix_arg_names(ix: &Instruction) -> String {
    ix.args.iter().map(|a| a.name.to_case(Case::Snake)).collect::<Vec<String>>().join(", ")
}

pub fn make_ix_has_info(ix: &Instruction) -> String {
    match ix.accounts.len() == 0 {
        true => String::new(),
        false => "<'info>".to_string()
    }
}

pub fn indent(s: String) -> String {
    s.lines().into_iter().map(|s| format!("    {}", s)).collect::<Vec<String>>().join("\n")
}

pub fn make_cargo_toml(idl: &IDL) -> String {
    format!("[package]
name = \"{}-sdk\"
version = \"{}\"
description = \"Created with IDLGen\"
edition = \"2021\"

[lib]
crate-type = [\"cdylib\", \"lib\"]
name = \"{}_sdk\"

[features]
rpc = []
i11n = [\"anchor-i11n\"]
cpi = []
events = []
default = [\"rpc\", \"i11n\", \"cpi\", \"events\"]

[dependencies]
anchor-lang = \"0.30.0\"
anchor-i11n = {{ optional = true, version = \"0.1.0\"}}", idl.get_name().to_case(Case::Kebab), idl.get_version(), idl.get_name().to_case(Case::Snake))
}

pub fn make_lib_rs(idl: &IDL) -> String {

format!("use anchor_lang::prelude::*;

declare_id!(\"{}\");

// Accounts
{}

// CPI
#[cfg(all(target_os = \"solana\", feature=\"cpi\"))]
{}

// RPC
#[cfg(all(not(target_os = \"solana\"), feature=\"cpi\"))]
pub mod rpc {{
    #![allow(unused)]
    use anchor_lang::prelude::*;
{}
}}

// I11n
#[cfg(all(target_os = \"solana\", feature=\"i11n\"))]
{}

// Instructions
{}

// Events
#[cfg(feature=\"events\")]
pub mod events {{
    use super::*;
    use anchor_i11n::AnchorDiscriminator;
    use anchor_lang::Discriminator;

{}
}}

// Accounts
{}
        
// Defined types
{}", idl.get_address(), make_cpi_accounts(idl), make_cpi_ctxs(idl), make_rpc_accounts(idl), make_i11n_ctxs(idl), make_ixs(idl), make_events(idl), make_accounts(idl), make_defined_types(idl))
}