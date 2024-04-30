use convert_case::{Case, Casing};
use sha2::{Digest, Sha256};

use crate::{generators::{cpi::{make_cpi_accounts, make_cpi_ctxs}, i11n::make_i11n_ctxs}, types::Instruction, Sdk, IDL};

pub fn make_defined_types(idl: &IDL) -> String {
    idl.types.iter().map(|t| {
        let ty = if t.kind.kind == "enum" {
    format!("#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum {} {{
{}
}}", t.name, t.kind.variants.clone().unwrap_or(vec![]).iter().map(|n| format!("    {}", n.name.to_case(Case::Pascal))).collect::<Vec<String>>().join(",\n"))
        } else if t.kind.kind == "struct" {
            format!("#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct {} {{
{}
}}", t.name, t.kind.fields.clone().unwrap_or(vec![]).iter().map(|f| {
        format!("    pub {}: {},", f.name.to_case(Case::Snake), f.kind.to_string())
    }).collect::<Vec<String>>().join("\n"))
        } else {
            panic!("Unknown defined type: {}", t.kind.kind);
        };
        ty
    }).collect::<Vec<String>>().join("\n\n")
}

pub fn make_ixs(idl: &IDL) -> String {
    format!("pub mod instructions {{
    use super::*;

{}        
}}",
        idl.instructions.iter().map(|i| {
        let ix_name_pascal =  i.name.to_case(Case::Pascal);
        format!("    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct {} {{
{}
    }}", ix_name_pascal, i.args.iter().map(|a| format!("        pub {}: {},", a.name.to_case(Case::Snake), a.kind.to_string())).collect::<Vec<String>>().join("\n"))
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

pub fn make_ix_discriminator(ix: &Instruction) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", ix.name.to_case(Case::Snake)).as_bytes());
    let mut b: [u8;8] = [0u8;8];
    b.clone_from_slice(&hasher.finalize().to_vec()[..8]);
    format!("[{},{},{},{},{},{},{},{}]", b[0],b[1],b[2],b[3],b[4],b[5],b[6],b[7])
}

pub fn make_ix_has_info(ix: &Instruction) -> String {
    match ix.accounts.len() == 0 {
        true => String::new(),
        false => "<'info>".to_string()
    }
}

pub fn make_sdk(idl: &IDL, sdk: &Sdk) -> String {
    let cpi = match sdk {
        &Sdk::CPI | &Sdk::Full => format!("
// Accounts
{}

// CPI
{}
", make_cpi_accounts(idl), make_cpi_ctxs(idl)),
        &Sdk::I11n => String::new()
    };
    let i11n = match sdk {
        &Sdk::I11n | &Sdk::Full => format!("
// I11n
{}
", make_i11n_ctxs(idl)),
        &Sdk::CPI => String::new()
    };
    let ixs = make_ixs(idl);
    let defined_types = make_defined_types(idl);

    format!("use anchor_lang::prelude::*;

declare_id!(\"{}\");
{}{}
// Instructions
{}
        
// Defined types
{}", idl.metadata.address, cpi, i11n, ixs, defined_types)
}