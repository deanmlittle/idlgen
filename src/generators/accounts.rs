use crate::{types::Accounts, IDL};
use convert_case::{Casing, Case};

pub fn make_accounts(idl: &IDL) -> String {
    format!("pub mod accounts {{
    #![allow(unused)]
    use super::*;

{}  
}}",
    idl.accounts.iter().map(|account| {
        let account_name_pascal = account.name.to_case(Case::Pascal);
        
        format!("   #[account]
    pub struct {} {{
{}
    }}", account_name_pascal, make_account_props(account))
        }).collect::<Vec<String>>().join("\n\n")
    )
}

pub fn make_account_props(account: &Accounts) -> String {
    account.kind.fields.iter().map(|t| format!("        pub {}: {}", t.name.to_case(Case::Snake), t.kind.to_string())).collect::<Vec<String>>().join(",\n")
}