use crate::IDL;
use convert_case::{Casing, Case};

pub fn make_rpc_accounts(idl: &IDL) -> String {
    idl.instructions.iter().map(|ix| {
        // Create the account struct
        let ix_name_pascal = ix.name.to_case(Case::Pascal);
        let account_names = ix.accounts.iter().map(|a| {
            format!("        pub {}: Pubkey,", a.name.to_case(Case::Snake))
        }).collect::<Vec<String>>().join("\n");
        let account_impls = ix.accounts.iter().map(|a| {
            match a.isMut {
                true => format!("        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.{},
            {},
        ));", a.name.to_case(Case::Snake), a.isSigner),
                false => format!("        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.{},
            {},
        ));", a.name.to_case(Case::Snake), a.isSigner),
            }
        }).collect::<Vec<String>>().join("\n");
        let rpc_account = match account_names.len() == 0 {
            true => {
                format!("#[cfg_attr(not(target_os=\"solana\"), derive(Debug))]
#[derive(AnchorSerialize)]
pub struct {} {{}}", ix_name_pascal)
        },
        false => {
            format!("#[cfg_attr(not(target_os=\"solana\"), derive(Debug))]
#[derive(AnchorSerialize)]
pub struct {} {{
{}
}}", ix_name_pascal, account_names)
            }
        };

        format!("{}

    impl anchor_lang::ToAccountMetas for {} {{
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {{
            let mut account_metas = vec![];
    {}
            account_metas
        }}
    }}", rpc_account, ix_name_pascal, account_impls)
}).collect::<Vec<String>>().join("\n\n")
}