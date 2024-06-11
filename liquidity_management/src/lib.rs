use anchor_lang::prelude::*;

declare_id!("83GGsxVtkoSSSsJhBCUCVbeZXUreqwc6LTW3nfQNhYuV");

// Accounts
#[derive(Accounts)]
pub struct AddAllocateRecord<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddRequest<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Allocate<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreatePosition<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    /// CHECK: Skip check
    pub holding: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program_2022: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct IncreasePosition<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    /// CHECK: Skip check
    pub holding: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program_2022: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct IncreaseReserve<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub reserve: AccountInfo<'info>,
    /// CHECK: Skip check
    pub proposal: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_manager: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program_2022: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub reserve: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_manager: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program_2022: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InvokeWithdrawAndBurnProposal<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RemoveAllocateRecord<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RemoveRequest<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RequestWithdrawal<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
}

// CPI
#[cfg(all(target_os = "solana", feature="cpi"))]
pub mod cpi {
    #![allow(unused)]
    use anchor_lang::Discriminator;
    use super::*;

    pub fn add_allocate_record<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, AddAllocateRecord<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::AddAllocateRecord {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::AddAllocateRecord::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn add_request<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, AddRequest<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::AddRequest {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::AddRequest::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn allocate<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Allocate<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Allocate {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Allocate::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn close_position<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ClosePosition<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ClosePosition {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ClosePosition::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn create_position<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreatePosition<'info>>,
        owner: Pubkey
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreatePosition { owner };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreatePosition::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn increase_position<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, IncreasePosition<'info>>,
        owner: Pubkey
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::IncreasePosition { owner };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::IncreasePosition::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn increase_reserve<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, IncreaseReserve<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::IncreaseReserve {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::IncreaseReserve::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn initialize<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Initialize<'info>>,
        seed: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Initialize { seed };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Initialize::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn invoke_withdraw_and_burn_proposal<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InvokeWithdrawAndBurnProposal<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InvokeWithdrawAndBurnProposal {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InvokeWithdrawAndBurnProposal::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn remove_allocate_record<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, RemoveAllocateRecord<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::RemoveAllocateRecord {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::RemoveAllocateRecord::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn remove_request<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, RemoveRequest<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::RemoveRequest {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::RemoveRequest::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn request_withdrawal<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, RequestWithdrawal<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::RequestWithdrawal {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::RequestWithdrawal::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }  
}

// RPC
#[cfg(all(not(target_os = "solana"), feature="cpi"))]
pub mod rpc {
    #![allow(unused)]
    use anchor_lang::prelude::*;
    #[derive(AnchorSerialize)]
    pub struct AddAllocateRecord {
        pub signer: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for AddAllocateRecord {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct AddRequest {
        pub signer: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for AddRequest {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct Allocate {
        pub signer: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for Allocate {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ClosePosition {
        pub signer: Pubkey,
        pub position: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ClosePosition {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.position,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CreatePosition {
        pub signer: Pubkey,
        pub position: Pubkey,
        pub holding: Pubkey,
        pub mint: Pubkey,
        pub system_program: Pubkey,
        pub token_program_2022: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CreatePosition {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.position,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.holding,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program_2022,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct IncreasePosition {
        pub signer: Pubkey,
        pub position: Pubkey,
        pub holding: Pubkey,
        pub mint: Pubkey,
        pub system_program: Pubkey,
        pub token_program_2022: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for IncreasePosition {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.position,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.holding,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program_2022,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct IncreaseReserve {
        pub signer: Pubkey,
        pub reserve: Pubkey,
        pub proposal: Pubkey,
        pub vault_manager: Pubkey,
        pub mint: Pubkey,
        pub system_program: Pubkey,
        pub token_program_2022: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for IncreaseReserve {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.reserve,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.proposal,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.vault_manager,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program_2022,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct Initialize {
        pub signer: Pubkey,
        pub reserve: Pubkey,
        pub vault_manager: Pubkey,
        pub vault: Pubkey,
        pub mint: Pubkey,
        pub token_program_2022: Pubkey,
        pub associated_token_program: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for Initialize {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.reserve,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.vault_manager,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program_2022,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InvokeWithdrawAndBurnProposal {
        pub signer: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InvokeWithdrawAndBurnProposal {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct RemoveAllocateRecord {
        pub signer: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for RemoveAllocateRecord {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct RemoveRequest {
        pub signer: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for RemoveRequest {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct RequestWithdrawal {
        pub signer: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for RequestWithdrawal {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            false,
        ));
            account_metas
        }
    }
}

// I11n
#[cfg(all(target_os = "solana", feature="i11n"))]
pub mod i11n {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use anchor_lang::Discriminator;
    use super::{instructions::*, ID};

    // AddAllocateRecord
    #[derive(TryFromInstruction)]
    pub struct AddAllocateRecordI11n<'info> {
        pub accounts: AddAllocateRecordAccountMetas<'info>,
        pub args: AddAllocateRecord,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // AddRequest
    #[derive(TryFromInstruction)]
    pub struct AddRequestI11n<'info> {
        pub accounts: AddRequestAccountMetas<'info>,
        pub args: AddRequest,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Allocate
    #[derive(TryFromInstruction)]
    pub struct AllocateI11n<'info> {
        pub accounts: AllocateAccountMetas<'info>,
        pub args: Allocate,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ClosePosition
    #[derive(TryFromInstruction)]
    pub struct ClosePositionI11n<'info> {
        pub accounts: ClosePositionAccountMetas<'info>,
        pub args: ClosePosition,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CreatePosition
    #[derive(TryFromInstruction)]
    pub struct CreatePositionI11n<'info> {
        pub accounts: CreatePositionAccountMetas<'info>,
        pub args: CreatePosition,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // IncreasePosition
    #[derive(TryFromInstruction)]
    pub struct IncreasePositionI11n<'info> {
        pub accounts: IncreasePositionAccountMetas<'info>,
        pub args: IncreasePosition,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // IncreaseReserve
    #[derive(TryFromInstruction)]
    pub struct IncreaseReserveI11n<'info> {
        pub accounts: IncreaseReserveAccountMetas<'info>,
        pub args: IncreaseReserve,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Initialize
    #[derive(TryFromInstruction)]
    pub struct InitializeI11n<'info> {
        pub accounts: InitializeAccountMetas<'info>,
        pub args: Initialize,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InvokeWithdrawAndBurnProposal
    #[derive(TryFromInstruction)]
    pub struct InvokeWithdrawAndBurnProposalI11n<'info> {
        pub accounts: InvokeWithdrawAndBurnProposalAccountMetas<'info>,
        pub args: InvokeWithdrawAndBurnProposal,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // RemoveAllocateRecord
    #[derive(TryFromInstruction)]
    pub struct RemoveAllocateRecordI11n<'info> {
        pub accounts: RemoveAllocateRecordAccountMetas<'info>,
        pub args: RemoveAllocateRecord,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // RemoveRequest
    #[derive(TryFromInstruction)]
    pub struct RemoveRequestI11n<'info> {
        pub accounts: RemoveRequestAccountMetas<'info>,
        pub args: RemoveRequest,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // RequestWithdrawal
    #[derive(TryFromInstruction)]
    pub struct RequestWithdrawalI11n<'info> {
        pub accounts: RequestWithdrawalAccountMetas<'info>,
        pub args: RequestWithdrawal,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    //Accounts
    #[derive(TryFromAccountMetas)]
    pub struct AddAllocateRecordAccountMetas<'info> {
        pub signer: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct AddRequestAccountMetas<'info> {
        pub signer: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct AllocateAccountMetas<'info> {
        pub signer: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ClosePositionAccountMetas<'info> {
        pub signer: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CreatePositionAccountMetas<'info> {
        pub signer: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub holding: &'info AccountMeta,
        pub mint: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program_2022: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct IncreasePositionAccountMetas<'info> {
        pub signer: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub holding: &'info AccountMeta,
        pub mint: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program_2022: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct IncreaseReserveAccountMetas<'info> {
        pub signer: &'info AccountMeta,
        pub reserve: &'info AccountMeta,
        pub proposal: &'info AccountMeta,
        pub vault_manager: &'info AccountMeta,
        pub mint: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program_2022: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeAccountMetas<'info> {
        pub signer: &'info AccountMeta,
        pub reserve: &'info AccountMeta,
        pub vault_manager: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub mint: &'info AccountMeta,
        pub token_program_2022: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InvokeWithdrawAndBurnProposalAccountMetas<'info> {
        pub signer: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct RemoveAllocateRecordAccountMetas<'info> {
        pub signer: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct RemoveRequestAccountMetas<'info> {
        pub signer: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct RequestWithdrawalAccountMetas<'info> {
        pub signer: &'info AccountMeta,
    }
}

// Instructions
pub mod instructions {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use anchor_lang::Discriminator;
    use super::*;

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct AddAllocateRecord {

    }
    
    impl anchor_lang::InstructionData for AddAllocateRecord {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct AddRequest {

    }
    
    impl anchor_lang::InstructionData for AddRequest {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Allocate {

    }
    
    impl anchor_lang::InstructionData for Allocate {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct ClosePosition {

    }
    
    impl anchor_lang::InstructionData for ClosePosition {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CreatePosition {
        pub owner: Pubkey,
    }
    
    impl anchor_lang::InstructionData for CreatePosition {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct IncreasePosition {
        pub owner: Pubkey,
    }
    
    impl anchor_lang::InstructionData for IncreasePosition {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct IncreaseReserve {

    }
    
    impl anchor_lang::InstructionData for IncreaseReserve {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Initialize {
        pub seed: u64,
    }
    
    impl anchor_lang::InstructionData for Initialize {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct InvokeWithdrawAndBurnProposal {

    }
    
    impl anchor_lang::InstructionData for InvokeWithdrawAndBurnProposal {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct RemoveAllocateRecord {

    }
    
    impl anchor_lang::InstructionData for RemoveAllocateRecord {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct RemoveRequest {

    }
    
    impl anchor_lang::InstructionData for RemoveRequest {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct RequestWithdrawal {

    }
    
    impl anchor_lang::InstructionData for RequestWithdrawal {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
            
}

// Events
#[cfg(feature="events")]
pub mod events {
    use super::*;
    use anchor_i11n::AnchorDiscriminator;
    use anchor_lang::Discriminator;

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct PositionCreated {

    }
    
    impl anchor_lang::Event for PositionCreated {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct PositionUpdated {

    }
    
    impl anchor_lang::Event for PositionUpdated {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct ReserveUpdated {

    }
    
    impl anchor_lang::Event for ReserveUpdated {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }
}

// Accounts
pub mod accounts {
    #![allow(unused)]
    use super::*;

   #[account]
    pub struct PositionAccount {

    }

   #[account]
    pub struct ReserveAccount {

    }

   #[account]
    pub struct VaultManagerAccount {

    }  
}
        
// Defined types
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Position {
    pub owner: Pubkey,
    pub amount: u64,
    pub mint: Pubkey,
    pub updated_at: i64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PositionAccount {
    pub fields: Position,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PositionCreated {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub updated_at: i64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PositionUpdated {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub updated_at: i64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Reserve {
    pub mint: Pubkey,
    pub amount: u64,
    pub updated_at: i64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ReserveAccount {
    pub fields: Reserve,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ReserveUpdated {
    pub mint: Pubkey,
    pub amount: u64,
    pub updated_at: i64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VaultManager {
    pub seed: u64,
    pub bump: u8,
    pub mint: Pubkey,
    pub amount: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VaultManagerAccount {
    pub fields: VaultManager,
}