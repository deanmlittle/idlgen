use crate::{generators::common::{make_ix_arg_names, make_ix_args, make_ix_has_info}, IDL};
use convert_case::{Casing, Case};

pub fn make_cpi_ctxs(idl: &IDL) -> String {
    format!("pub mod cpi {{
    #![allow(unused)]
    use anchor_i11n::Discriminator;
    use super::*;

{}  
}}",
    idl.instructions.iter().map(|ix| {
        let ix_name_pascal =  ix.name.to_case(Case::Pascal);
        let ix_name_snake =  ix.name.to_case(Case::Snake);
        let ix_has_info = make_ix_has_info(ix);
        let ix_args = make_ix_args(ix);
        let ix_arg_names = make_ix_arg_names(ix);

        format!("    pub fn {}<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, {}{}>{}
    ) -> anchor_lang::Result<()> {{
        let ix = {{
            let ix = instructions::{} {{ {} }};
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::{}::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {{
                program_id: ctx.program.key(),
                accounts,
                data,
            }}
        }};
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }}", ix_name_snake, ix_name_pascal, ix_has_info, ix_args, ix_name_pascal, ix_arg_names, ix_name_pascal)
        }).collect::<Vec<String>>().join("\n\n")
    )
}

pub fn make_cpi_accounts(idl: &IDL) -> String {
    idl.instructions.iter().map(|ix| {
        let ix_name_pascal =  ix.name.to_case(Case::Pascal);
        if ix.accounts.len() == 0 {
            return format!("#[derive(Accounts)]
pub struct {} {{}}", ix_name_pascal)
        }
        format!("#[derive(Accounts)]
pub struct {}<'info> {{
{}
}}", ix_name_pascal, ix.accounts.iter().map(|a| {
        let mut kind = "AccountInfo<'info>".to_string();
        if a.isOptional {
            kind = format!("Option<{}>", kind);
        }
        let constraints = match (a.isMut, a.isSigner) {
            (true, true) | (false, true) => "    #[account(mut, signer)]\n",
            (true, false) => "    #[account(mut)]\n",
            (false, false) => ""
        };
        format!("{}    /// CHECK: Skip check\n    pub {}: {},", constraints, a.name.to_case(Case::Snake), kind)
        }).collect::<Vec<String>>().join("\n"))
    }).collect::<Vec<String>>().join("\n\n")
}