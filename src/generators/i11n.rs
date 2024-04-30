use convert_case::{Case, Casing};
use crate::{generators::common::make_ix_has_info, IDL};

pub fn make_i11n_accounts(idl: &IDL) -> String {
    idl.instructions.iter().map(|ix| {
        let ix_name_pascal =  ix.name.to_case(Case::Pascal);
        format!("    #[derive(TryFromAccountMetas)]
    pub struct {}AccountMetas{} {{
{}
    }}", ix_name_pascal, make_ix_has_info(ix), ix.accounts.iter().map(|a| {
        let kind = match a.isOptional {
            true => "Option<&'info AccountMeta>",
            false => "&'info AccountMeta"
        };
        format!("        pub {}: {},", a.name.to_case(Case::Snake), kind)
        }).collect::<Vec<String>>().join("\n"))
    }).collect::<Vec<String>>().join("\n\n")
}

pub fn make_i11n_ctxs(idl: &IDL) -> String {
    format!("pub mod i11n {{
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use super::{{instructions::*, ID}};
{}

    //Accounts
{}
}}",

idl.instructions.iter().map(|ix| {
        let ix_name_pascal = ix.name.to_case(Case::Pascal);
        format!("
    // {}
    #[derive(TryFromInstruction)]
    pub struct {}I11n{} {{
        pub accounts: {}AccountMetas{},
        pub args: {}
    }}", ix_name_pascal, ix_name_pascal, make_ix_has_info(ix), ix_name_pascal, make_ix_has_info(ix), ix_name_pascal)
        }).collect::<Vec<String>>().join("\n"),
        make_i11n_accounts(idl)
    )
}