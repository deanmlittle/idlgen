use anchor_lang::prelude::*;

declare_id!("ZETAxsqBRek56DhiGXrn75yj2NHU3aYUnxvHXpkf3aD");

// Accounts
#[derive(Accounts)]
pub struct InitializeZetaPricing<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaPricingPubkeys<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeZetaGroup<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub underlying_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub perp_sync_queue: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub underlying: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub usdc_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct OverrideExpiry<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MigrateToNewCrossMarginAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub new_cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub old_cross_margin_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MigrateToCrossMarginAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCrossMarginAccountManager<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account_manager: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCrossMarginAccountManagerV2<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account_manager: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCrossMarginAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account_manager: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarginAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeSpreadAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub spread_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseCrossMarginAccountManager<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account_manager: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseCrossMarginAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account_manager: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseMarginAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseSpreadAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub spread_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeUnderlying<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub underlying: AccountInfo<'info>,
    /// CHECK: Skip check
    pub underlying_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializePerpSyncQueue<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub perp_sync_queue: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarketIndexes<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_indexes: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarketNode<'info> {
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Halt<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Unhalt<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateHaltState<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateVolatility<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateInterestRate<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddPerpMarketIndex<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub market_indexes: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddMarketIndexes<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub market_indexes: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeZetaState<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub treasury_wallet: AccountInfo<'info>,
    /// CHECK: Skip check
    pub referrals_admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub referrals_rewards_wallet: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub secondary_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeZetaTreasuryWallet<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub treasury_wallet: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeZetaReferralsRewardsWallet<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub referrals_rewards_wallet: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateSecondaryAdmin<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateTriggerAdmin<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMaTypeAdmin<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateReferralsAdmin<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePricingAdmin<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMakerRebatePercentage<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateTakeTriggerOrderFeePercentage<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaState<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateOracle<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateOracleBackupFeed<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePricingParameters<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMarginParameters<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaGroupMarginParameters<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePerpParameters<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaGroupPerpParameters<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaGroupExpiryParameters<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ToggleZetaGroupPerpsOnly<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CleanZetaMarkets<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CleanZetaMarketHalted<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub bids: AccountInfo<'info>,
    /// CHECK: Skip check
    pub asks: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SettlePositionsHalted<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarketStrikes<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExpireSeriesOverride {}

#[derive(Accounts)]
pub struct ExpireSeries {}

#[derive(Accounts)]
pub struct InitializeZetaMarket<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market_indexes: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub base_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub quote_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_base_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_quote_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub dex_base_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub dex_quote_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_owner: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarketTifEpochCycle<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePricingV2<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub perp_market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub perp_bids: AccountInfo<'info>,
    /// CHECK: Skip check
    pub perp_asks: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePricingV3<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub perp_market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub perp_bids: AccountInfo<'info>,
    /// CHECK: Skip check
    pub perp_asks: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub pricing_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApplyPerpFunding<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositV2<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositPermissionless<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub deposit_token_acc: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositInsuranceVault<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_deposit_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositInsuranceVaultV2<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_deposit_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ChooseAirdropCommunity<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account_manager: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawV2<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawInsuranceVault<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_deposit_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_token_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawInsuranceVaultV2<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_deposit_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_token_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrders<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders_map: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrdersV2<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders_map: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrdersV3<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders_map: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseOpenOrders<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders_map: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseOpenOrdersV2<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders_map: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseOpenOrdersV3<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders_map: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseOpenOrdersV4<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders_map: AccountInfo<'info>,
    /// CHECK: Skip check
    pub event_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeWhitelistDepositAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whitelist_deposit_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub user: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeWhitelistInsuranceAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whitelist_insurance_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub user: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeWhitelistTradingFeesAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whitelist_trading_fees_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub user: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeInsuranceDepositAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_deposit_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub whitelist_insurance_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCombinedInsuranceVault<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCombinedVault<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCombinedSocializedLossAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market_accounts: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrderV2<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market_accounts: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrderV3<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market_accounts: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrder<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market_accounts: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrderV2<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market_accounts: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrderV4<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market_accounts: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrderV3<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market_accounts: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrderV4<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub place_order_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceMultiOrders<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_base_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_quote_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_base_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_quote_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_base_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market_quote_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceTriggerOrder<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub trigger_order: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTriggerOrderV2<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub trigger_order: AccountInfo<'info>,
    /// CHECK: Skip check
    pub place_order_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TakeTriggerOrder<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub trigger_order: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub bids: AccountInfo<'info>,
    /// CHECK: Skip check
    pub asks: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub taker: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub taker_margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub order_margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTriggerOrder<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub trigger_order: AccountInfo<'info>,
    /// CHECK: Skip check
    pub place_order_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelTriggerOrder<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub trigger_order: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelTriggerOrderV2<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub trigger_order: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelTriggerOrder<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub trigger_order: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMinLot<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateTickSize<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMinLotsAndTickSizes<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct EditTriggerOrder<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub trigger_order: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct EditTriggerOrderV2<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub trigger_order: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrderNoError<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelAllMarketOrders<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrderHalted<'info> {
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrderByClientOrderId<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrderByClientOrderIdNoError<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PruneExpiredTifOrders<'info> {
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub event_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PruneExpiredTifOrdersV2<'info> {
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub event_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrderByOrderIdV2<'info> {
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrderByOrderId<'info> {
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrdersV2<'info> {
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrders<'info> {
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CrankEventQueue<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub event_queue: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CollectTreasuryFunds<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub treasury_wallet: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub collection_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TreasuryMovement<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub treasury_wallet: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub referrals_rewards_wallet: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RebalanceInsuranceVault<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub treasury_wallet: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub socialized_loss_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LiquidateV2<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub liquidator: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub liquidator_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub pricing: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub liquidated_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Liquidate<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub liquidator: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub liquidator_margin_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub liquidated_margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnVaultTokens<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SettleDexFunds<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_base_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub zeta_quote_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub dex_base_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub dex_quote_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_owner: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PositionMovement<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub spread_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub greeks: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_feed: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_backup_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferExcessSpreadBalance<'info> {
    /// CHECK: Skip check
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub spread_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ToggleMarketMaker<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeReferrerAccounts<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub referrer_id_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub referrer_pubkey_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseReferrerAccounts<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub referrer_id_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub referrer_pubkey_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct EditMaType<'info> {
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct EditDelegatedPubkey<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ResetNumFlexUnderlyings<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
}

// CPI
pub mod cpi {
    #![allow(unused)]
    use super::*;

    pub fn initialize_zeta_pricing<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeZetaPricing<'info>>,
        args: InitializeZetaPricingArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeZetaPricing { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeZetaPricing::DISCRIMINATOR);
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

    pub fn update_zeta_pricing_pubkeys<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateZetaPricingPubkeys<'info>>,
        args: UpdateZetaPricingPubkeysArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateZetaPricingPubkeys { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateZetaPricingPubkeys::DISCRIMINATOR);
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

    pub fn initialize_zeta_group<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeZetaGroup<'info>>,
        args: InitializeZetaGroupArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeZetaGroup { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeZetaGroup::DISCRIMINATOR);
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

    pub fn override_expiry<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, OverrideExpiry<'info>>,
        args: OverrideExpiryArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::OverrideExpiry { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::OverrideExpiry::DISCRIMINATOR);
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

    pub fn migrate_to_new_cross_margin_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, MigrateToNewCrossMarginAccount<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::MigrateToNewCrossMarginAccount {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::MigrateToNewCrossMarginAccount::DISCRIMINATOR);
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

    pub fn migrate_to_cross_margin_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, MigrateToCrossMarginAccount<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::MigrateToCrossMarginAccount {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::MigrateToCrossMarginAccount::DISCRIMINATOR);
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

    pub fn initialize_cross_margin_account_manager<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeCrossMarginAccountManager<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeCrossMarginAccountManager {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeCrossMarginAccountManager::DISCRIMINATOR);
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

    pub fn initialize_cross_margin_account_manager_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeCrossMarginAccountManagerV2<'info>>,
        referrer: Option<Pubkey>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeCrossMarginAccountManagerV2 { referrer };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeCrossMarginAccountManagerV2::DISCRIMINATOR);
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

    pub fn initialize_cross_margin_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeCrossMarginAccount<'info>>,
        subaccount_index: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeCrossMarginAccount { subaccount_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeCrossMarginAccount::DISCRIMINATOR);
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

    pub fn initialize_margin_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeMarginAccount<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeMarginAccount {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeMarginAccount::DISCRIMINATOR);
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

    pub fn initialize_spread_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeSpreadAccount<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeSpreadAccount {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeSpreadAccount::DISCRIMINATOR);
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

    pub fn close_cross_margin_account_manager<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseCrossMarginAccountManager<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseCrossMarginAccountManager {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseCrossMarginAccountManager::DISCRIMINATOR);
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

    pub fn close_cross_margin_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseCrossMarginAccount<'info>>,
        subaccount_index: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseCrossMarginAccount { subaccount_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseCrossMarginAccount::DISCRIMINATOR);
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

    pub fn close_margin_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseMarginAccount<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseMarginAccount {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseMarginAccount::DISCRIMINATOR);
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

    pub fn close_spread_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseSpreadAccount<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseSpreadAccount {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseSpreadAccount::DISCRIMINATOR);
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

    pub fn initialize_underlying<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeUnderlying<'info>>,
        flex_underlying: bool
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeUnderlying { flex_underlying };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeUnderlying::DISCRIMINATOR);
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

    pub fn initialize_perp_sync_queue<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializePerpSyncQueue<'info>>,
        nonce: u8,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializePerpSyncQueue { nonce, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializePerpSyncQueue::DISCRIMINATOR);
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

    pub fn initialize_market_indexes<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeMarketIndexes<'info>>,
        nonce: u8,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeMarketIndexes { nonce, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeMarketIndexes::DISCRIMINATOR);
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

    pub fn initialize_market_node<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeMarketNode<'info>>,
        args: InitializeMarketNodeArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeMarketNode { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeMarketNode::DISCRIMINATOR);
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

    pub fn halt<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Halt<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Halt { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Halt::DISCRIMINATOR);
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

    pub fn unhalt<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Unhalt<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Unhalt { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Unhalt::DISCRIMINATOR);
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

    pub fn update_halt_state<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateHaltState<'info>>,
        args: HaltStateArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateHaltState { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateHaltState::DISCRIMINATOR);
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

    pub fn update_volatility<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateVolatility<'info>>,
        args: UpdateVolatilityArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateVolatility { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateVolatility::DISCRIMINATOR);
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

    pub fn update_interest_rate<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateInterestRate<'info>>,
        args: UpdateInterestRateArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateInterestRate { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateInterestRate::DISCRIMINATOR);
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

    pub fn add_perp_market_index<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, AddPerpMarketIndex<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::AddPerpMarketIndex { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::AddPerpMarketIndex::DISCRIMINATOR);
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

    pub fn add_market_indexes<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, AddMarketIndexes<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::AddMarketIndexes {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::AddMarketIndexes::DISCRIMINATOR);
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

    pub fn initialize_zeta_state<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeZetaState<'info>>,
        args: InitializeStateArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeZetaState { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeZetaState::DISCRIMINATOR);
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

    pub fn initialize_zeta_treasury_wallet<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeZetaTreasuryWallet<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeZetaTreasuryWallet {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeZetaTreasuryWallet::DISCRIMINATOR);
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

    pub fn initialize_zeta_referrals_rewards_wallet<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeZetaReferralsRewardsWallet<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeZetaReferralsRewardsWallet {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeZetaReferralsRewardsWallet::DISCRIMINATOR);
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

    pub fn update_admin<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateAdmin<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateAdmin {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateAdmin::DISCRIMINATOR);
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

    pub fn update_secondary_admin<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateSecondaryAdmin<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateSecondaryAdmin {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateSecondaryAdmin::DISCRIMINATOR);
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

    pub fn update_trigger_admin<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateTriggerAdmin<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateTriggerAdmin {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateTriggerAdmin::DISCRIMINATOR);
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

    pub fn update_ma_type_admin<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateMaTypeAdmin<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateMaTypeAdmin {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateMaTypeAdmin::DISCRIMINATOR);
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

    pub fn update_referrals_admin<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateReferralsAdmin<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateReferralsAdmin {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateReferralsAdmin::DISCRIMINATOR);
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

    pub fn update_pricing_admin<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePricingAdmin<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdatePricingAdmin {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdatePricingAdmin::DISCRIMINATOR);
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

    pub fn update_maker_rebate_percentage<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateMakerRebatePercentage<'info>>,
        native_maker_rebate_percentage: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateMakerRebatePercentage { native_maker_rebate_percentage };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateMakerRebatePercentage::DISCRIMINATOR);
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

    pub fn update_take_trigger_order_fee_percentage<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateTakeTriggerOrderFeePercentage<'info>>,
        new_take_trigger_order_fee_percentage: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateTakeTriggerOrderFeePercentage { new_take_trigger_order_fee_percentage };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateTakeTriggerOrderFeePercentage::DISCRIMINATOR);
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

    pub fn update_zeta_state<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateZetaState<'info>>,
        args: UpdateStateArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateZetaState { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateZetaState::DISCRIMINATOR);
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

    pub fn update_oracle<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateOracle<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateOracle {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateOracle::DISCRIMINATOR);
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

    pub fn update_oracle_backup_feed<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateOracleBackupFeed<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateOracleBackupFeed {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateOracleBackupFeed::DISCRIMINATOR);
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

    pub fn update_pricing_parameters<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePricingParameters<'info>>,
        args: UpdatePricingParametersArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdatePricingParameters { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdatePricingParameters::DISCRIMINATOR);
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

    pub fn update_margin_parameters<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateMarginParameters<'info>>,
        args: UpdateMarginParametersArgs,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateMarginParameters { args, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateMarginParameters::DISCRIMINATOR);
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

    pub fn update_zeta_group_margin_parameters<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateZetaGroupMarginParameters<'info>>,
        args: UpdateMarginParametersArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateZetaGroupMarginParameters { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateZetaGroupMarginParameters::DISCRIMINATOR);
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

    pub fn update_perp_parameters<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePerpParameters<'info>>,
        args: UpdatePerpParametersArgs,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdatePerpParameters { args, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdatePerpParameters::DISCRIMINATOR);
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

    pub fn update_zeta_group_perp_parameters<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateZetaGroupPerpParameters<'info>>,
        args: UpdatePerpParametersArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateZetaGroupPerpParameters { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateZetaGroupPerpParameters::DISCRIMINATOR);
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

    pub fn update_zeta_group_expiry_parameters<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateZetaGroupExpiryParameters<'info>>,
        args: UpdateZetaGroupExpiryArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateZetaGroupExpiryParameters { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateZetaGroupExpiryParameters::DISCRIMINATOR);
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

    pub fn toggle_zeta_group_perps_only<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ToggleZetaGroupPerpsOnly<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ToggleZetaGroupPerpsOnly {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ToggleZetaGroupPerpsOnly::DISCRIMINATOR);
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

    pub fn clean_zeta_markets<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CleanZetaMarkets<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CleanZetaMarkets {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CleanZetaMarkets::DISCRIMINATOR);
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

    pub fn clean_zeta_market_halted<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CleanZetaMarketHalted<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CleanZetaMarketHalted { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CleanZetaMarketHalted::DISCRIMINATOR);
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

    pub fn settle_positions_halted<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SettlePositionsHalted<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SettlePositionsHalted { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SettlePositionsHalted::DISCRIMINATOR);
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

    pub fn initialize_market_strikes<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeMarketStrikes<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeMarketStrikes {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeMarketStrikes::DISCRIMINATOR);
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

    pub fn expire_series_override<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ExpireSeriesOverride>,
        args: ExpireSeriesOverrideArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ExpireSeriesOverride { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ExpireSeriesOverride::DISCRIMINATOR);
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

    pub fn expire_series<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ExpireSeries>,
        settlement_nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ExpireSeries { settlement_nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ExpireSeries::DISCRIMINATOR);
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

    pub fn initialize_zeta_market<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeZetaMarket<'info>>,
        args: InitializeMarketArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeZetaMarket { args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeZetaMarket::DISCRIMINATOR);
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

    pub fn initialize_market_tif_epoch_cycle<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeMarketTifEpochCycle<'info>>,
        epoch_length: u16
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeMarketTifEpochCycle { epoch_length };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeMarketTifEpochCycle::DISCRIMINATOR);
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

    pub fn update_pricing_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePricingV2<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdatePricingV2 { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdatePricingV2::DISCRIMINATOR);
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

    pub fn update_pricing_v_3<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePricingV3<'info>>,
        asset: Asset,
        price: u64,
        timestamp: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdatePricingV3 { asset, price, timestamp };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdatePricingV3::DISCRIMINATOR);
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

    pub fn apply_perp_funding<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ApplyPerpFunding<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ApplyPerpFunding { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ApplyPerpFunding::DISCRIMINATOR);
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

    pub fn deposit<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Deposit { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Deposit::DISCRIMINATOR);
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

    pub fn deposit_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, DepositV2<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::DepositV2 { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::DepositV2::DISCRIMINATOR);
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

    pub fn deposit_permissionless<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, DepositPermissionless<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::DepositPermissionless { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::DepositPermissionless::DISCRIMINATOR);
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

    pub fn deposit_insurance_vault<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, DepositInsuranceVault<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::DepositInsuranceVault { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::DepositInsuranceVault::DISCRIMINATOR);
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

    pub fn deposit_insurance_vault_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, DepositInsuranceVaultV2<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::DepositInsuranceVaultV2 { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::DepositInsuranceVaultV2::DISCRIMINATOR);
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

    pub fn choose_airdrop_community<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ChooseAirdropCommunity<'info>>,
        community: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ChooseAirdropCommunity { community };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ChooseAirdropCommunity::DISCRIMINATOR);
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

    pub fn withdraw<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Withdraw { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Withdraw::DISCRIMINATOR);
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

    pub fn withdraw_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, WithdrawV2<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::WithdrawV2 { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::WithdrawV2::DISCRIMINATOR);
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

    pub fn withdraw_insurance_vault<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, WithdrawInsuranceVault<'info>>,
        percentage_amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::WithdrawInsuranceVault { percentage_amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::WithdrawInsuranceVault::DISCRIMINATOR);
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

    pub fn withdraw_insurance_vault_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, WithdrawInsuranceVaultV2<'info>>,
        percentage_amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::WithdrawInsuranceVaultV2 { percentage_amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::WithdrawInsuranceVaultV2::DISCRIMINATOR);
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

    pub fn initialize_open_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeOpenOrders<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeOpenOrders {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeOpenOrders::DISCRIMINATOR);
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

    pub fn initialize_open_orders_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeOpenOrdersV2<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeOpenOrdersV2 {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeOpenOrdersV2::DISCRIMINATOR);
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

    pub fn initialize_open_orders_v_3<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeOpenOrdersV3<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeOpenOrdersV3 { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeOpenOrdersV3::DISCRIMINATOR);
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

    pub fn close_open_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseOpenOrders<'info>>,
        map_nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseOpenOrders { map_nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseOpenOrders::DISCRIMINATOR);
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

    pub fn close_open_orders_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseOpenOrdersV2<'info>>,
        map_nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseOpenOrdersV2 { map_nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseOpenOrdersV2::DISCRIMINATOR);
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

    pub fn close_open_orders_v_3<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseOpenOrdersV3<'info>>,
        map_nonce: u8,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseOpenOrdersV3 { map_nonce, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseOpenOrdersV3::DISCRIMINATOR);
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

    pub fn close_open_orders_v_4<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseOpenOrdersV4<'info>>,
        map_nonce: u8,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseOpenOrdersV4 { map_nonce, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseOpenOrdersV4::DISCRIMINATOR);
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

    pub fn initialize_whitelist_deposit_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeWhitelistDepositAccount<'info>>,
        nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeWhitelistDepositAccount { nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeWhitelistDepositAccount::DISCRIMINATOR);
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

    pub fn initialize_whitelist_insurance_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeWhitelistInsuranceAccount<'info>>,
        nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeWhitelistInsuranceAccount { nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeWhitelistInsuranceAccount::DISCRIMINATOR);
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

    pub fn initialize_whitelist_trading_fees_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeWhitelistTradingFeesAccount<'info>>,
        nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeWhitelistTradingFeesAccount { nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeWhitelistTradingFeesAccount::DISCRIMINATOR);
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

    pub fn initialize_insurance_deposit_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeInsuranceDepositAccount<'info>>,
        nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeInsuranceDepositAccount { nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeInsuranceDepositAccount::DISCRIMINATOR);
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

    pub fn initialize_combined_insurance_vault<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeCombinedInsuranceVault<'info>>,
        nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeCombinedInsuranceVault { nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeCombinedInsuranceVault::DISCRIMINATOR);
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

    pub fn initialize_combined_vault<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeCombinedVault<'info>>,
        nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeCombinedVault { nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeCombinedVault::DISCRIMINATOR);
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

    pub fn initialize_combined_socialized_loss_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeCombinedSocializedLossAccount<'info>>,
        nonce: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeCombinedSocializedLossAccount { nonce };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeCombinedSocializedLossAccount::DISCRIMINATOR);
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

    pub fn place_order<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlaceOrder<'info>>,
        price: u64,
        size: u64,
        side: Side,
        client_order_id: Option<u64>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlaceOrder { price, size, side, client_order_id };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlaceOrder::DISCRIMINATOR);
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

    pub fn place_order_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlaceOrderV2<'info>>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlaceOrderV2 { price, size, side, order_type, client_order_id };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlaceOrderV2::DISCRIMINATOR);
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

    pub fn place_order_v_3<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlaceOrderV3<'info>>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlaceOrderV3 { price, size, side, order_type, client_order_id, tag };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlaceOrderV3::DISCRIMINATOR);
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

    pub fn place_perp_order<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlacePerpOrder<'info>>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlacePerpOrder { price, size, side, order_type, client_order_id, tag };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlacePerpOrder::DISCRIMINATOR);
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

    pub fn place_perp_order_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlacePerpOrderV2<'info>>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>,
        tif_offset: Option<u16>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlacePerpOrderV2 { price, size, side, order_type, client_order_id, tag, tif_offset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlacePerpOrderV2::DISCRIMINATOR);
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

    pub fn place_order_v_4<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlaceOrderV4<'info>>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>,
        tif_offset: Option<u16>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlaceOrderV4 { price, size, side, order_type, client_order_id, tag, tif_offset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlaceOrderV4::DISCRIMINATOR);
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

    pub fn place_perp_order_v_3<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlacePerpOrderV3<'info>>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>,
        tif_offset: Option<u16>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlacePerpOrderV3 { price, size, side, order_type, client_order_id, tag, tif_offset, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlacePerpOrderV3::DISCRIMINATOR);
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

    pub fn place_perp_order_v_4<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlacePerpOrderV4<'info>>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        reduce_only: bool,
        client_order_id: Option<u64>,
        tag: Option<String>,
        tif_offset: Option<u16>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlacePerpOrderV4 { price, size, side, order_type, reduce_only, client_order_id, tag, tif_offset, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlacePerpOrderV4::DISCRIMINATOR);
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

    pub fn place_multi_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlaceMultiOrders<'info>>,
        asset: Asset,
        bid_orders: Vec<OrderArgs>,
        ask_orders: Vec<OrderArgs>,
        order_type: OrderType
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlaceMultiOrders { asset, bid_orders, ask_orders, order_type };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlaceMultiOrders::DISCRIMINATOR);
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

    pub fn place_trigger_order<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PlaceTriggerOrder<'info>>,
        trigger_order_bit: u8,
        order_price: u64,
        trigger_price: Option<u64>,
        trigger_direction: Option<TriggerDirection>,
        trigger_ts: Option<u64>,
        size: u64,
        side: Side,
        order_type: OrderType,
        reduce_only: bool,
        tag: Option<String>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PlaceTriggerOrder { trigger_order_bit, order_price, trigger_price, trigger_direction, trigger_ts, size, side, order_type, reduce_only, tag, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PlaceTriggerOrder::DISCRIMINATOR);
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

    pub fn execute_trigger_order_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ExecuteTriggerOrderV2<'info>>,
        trigger_order_bit: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ExecuteTriggerOrderV2 { trigger_order_bit };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ExecuteTriggerOrderV2::DISCRIMINATOR);
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

    pub fn take_trigger_order<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, TakeTriggerOrder<'info>>,
        trigger_order_bit: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::TakeTriggerOrder { trigger_order_bit };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::TakeTriggerOrder::DISCRIMINATOR);
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

    pub fn execute_trigger_order<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ExecuteTriggerOrder<'info>>,
        trigger_order_bit: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ExecuteTriggerOrder { trigger_order_bit };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ExecuteTriggerOrder::DISCRIMINATOR);
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

    pub fn force_cancel_trigger_order<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ForceCancelTriggerOrder<'info>>,
        trigger_order_bit: u8,
        enforce_tpsl_conditions: bool
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ForceCancelTriggerOrder { trigger_order_bit, enforce_tpsl_conditions };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ForceCancelTriggerOrder::DISCRIMINATOR);
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

    pub fn cancel_trigger_order_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CancelTriggerOrderV2<'info>>,
        trigger_order_bit: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CancelTriggerOrderV2 { trigger_order_bit };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CancelTriggerOrderV2::DISCRIMINATOR);
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

    pub fn cancel_trigger_order<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CancelTriggerOrder<'info>>,
        trigger_order_bit: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CancelTriggerOrder { trigger_order_bit };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CancelTriggerOrder::DISCRIMINATOR);
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

    pub fn update_min_lot<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateMinLot<'info>>,
        asset: Asset,
        min_lot_size: u32
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateMinLot { asset, min_lot_size };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateMinLot::DISCRIMINATOR);
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

    pub fn update_tick_size<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateTickSize<'info>>,
        asset: Asset,
        tick_size: u32
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateTickSize { asset, tick_size };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateTickSize::DISCRIMINATOR);
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

    pub fn initialize_min_lots_and_tick_sizes<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeMinLotsAndTickSizes<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeMinLotsAndTickSizes {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeMinLotsAndTickSizes::DISCRIMINATOR);
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

    pub fn edit_trigger_order<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, EditTriggerOrder<'info>>,
        order_price: u64,
        trigger_price: Option<u64>,
        trigger_direction: Option<TriggerDirection>,
        trigger_ts: Option<u64>,
        size: u64,
        side: Side,
        order_type: OrderType,
        reduce_only: bool
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::EditTriggerOrder { order_price, trigger_price, trigger_direction, trigger_ts, size, side, order_type, reduce_only };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::EditTriggerOrder::DISCRIMINATOR);
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

    pub fn edit_trigger_order_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, EditTriggerOrderV2<'info>>,
        order_price: u64,
        trigger_price: Option<u64>,
        trigger_direction: Option<TriggerDirection>,
        trigger_ts: Option<u64>,
        size: u64,
        side: Side,
        order_type: OrderType,
        reduce_only: bool
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::EditTriggerOrderV2 { order_price, trigger_price, trigger_direction, trigger_ts, size, side, order_type, reduce_only };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::EditTriggerOrderV2::DISCRIMINATOR);
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

    pub fn cancel_order<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CancelOrder<'info>>,
        side: Side,
        order_id: u128,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CancelOrder { side, order_id, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CancelOrder::DISCRIMINATOR);
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

    pub fn cancel_order_no_error<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CancelOrderNoError<'info>>,
        side: Side,
        order_id: u128,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CancelOrderNoError { side, order_id, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CancelOrderNoError::DISCRIMINATOR);
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

    pub fn cancel_all_market_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CancelAllMarketOrders<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CancelAllMarketOrders { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CancelAllMarketOrders::DISCRIMINATOR);
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

    pub fn cancel_order_halted<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CancelOrderHalted<'info>>,
        side: Side,
        order_id: u128,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CancelOrderHalted { side, order_id, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CancelOrderHalted::DISCRIMINATOR);
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

    pub fn cancel_order_by_client_order_id<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CancelOrderByClientOrderId<'info>>,
        client_order_id: u64,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CancelOrderByClientOrderId { client_order_id, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CancelOrderByClientOrderId::DISCRIMINATOR);
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

    pub fn cancel_order_by_client_order_id_no_error<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CancelOrderByClientOrderIdNoError<'info>>,
        client_order_id: u64,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CancelOrderByClientOrderIdNoError { client_order_id, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CancelOrderByClientOrderIdNoError::DISCRIMINATOR);
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

    pub fn prune_expired_tif_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PruneExpiredTifOrders<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PruneExpiredTifOrders {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PruneExpiredTifOrders::DISCRIMINATOR);
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

    pub fn prune_expired_tif_orders_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PruneExpiredTifOrdersV2<'info>>,
        limit: u16
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PruneExpiredTifOrdersV2 { limit };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PruneExpiredTifOrdersV2::DISCRIMINATOR);
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

    pub fn force_cancel_order_by_order_id_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ForceCancelOrderByOrderIdV2<'info>>,
        side: Side,
        order_id: u128,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ForceCancelOrderByOrderIdV2 { side, order_id, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ForceCancelOrderByOrderIdV2::DISCRIMINATOR);
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

    pub fn force_cancel_order_by_order_id<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ForceCancelOrderByOrderId<'info>>,
        side: Side,
        order_id: u128,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ForceCancelOrderByOrderId { side, order_id, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ForceCancelOrderByOrderId::DISCRIMINATOR);
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

    pub fn force_cancel_orders_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ForceCancelOrdersV2<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ForceCancelOrdersV2 { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ForceCancelOrdersV2::DISCRIMINATOR);
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

    pub fn force_cancel_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ForceCancelOrders<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ForceCancelOrders { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ForceCancelOrders::DISCRIMINATOR);
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

    pub fn crank_event_queue<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CrankEventQueue<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CrankEventQueue { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CrankEventQueue::DISCRIMINATOR);
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

    pub fn collect_treasury_funds<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CollectTreasuryFunds<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CollectTreasuryFunds { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CollectTreasuryFunds::DISCRIMINATOR);
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

    pub fn treasury_movement<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, TreasuryMovement<'info>>,
        treasury_movement_type: TreasuryMovementType,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::TreasuryMovement { treasury_movement_type, amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::TreasuryMovement::DISCRIMINATOR);
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

    pub fn rebalance_insurance_vault<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, RebalanceInsuranceVault<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::RebalanceInsuranceVault {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::RebalanceInsuranceVault::DISCRIMINATOR);
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

    pub fn liquidate_v_2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, LiquidateV2<'info>>,
        size: u64,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::LiquidateV2 { size, asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::LiquidateV2::DISCRIMINATOR);
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

    pub fn liquidate<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Liquidate<'info>>,
        size: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Liquidate { size };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Liquidate::DISCRIMINATOR);
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

    pub fn burn_vault_tokens<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, BurnVaultTokens<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::BurnVaultTokens {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::BurnVaultTokens::DISCRIMINATOR);
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

    pub fn settle_dex_funds<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SettleDexFunds<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SettleDexFunds {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SettleDexFunds::DISCRIMINATOR);
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

    pub fn position_movement<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, PositionMovement<'info>>,
        movement_type: MovementType,
        movements: Vec<PositionMovementArg>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::PositionMovement { movement_type, movements };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::PositionMovement::DISCRIMINATOR);
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

    pub fn transfer_excess_spread_balance<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, TransferExcessSpreadBalance<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::TransferExcessSpreadBalance {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::TransferExcessSpreadBalance::DISCRIMINATOR);
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

    pub fn toggle_market_maker<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ToggleMarketMaker<'info>>,
        is_market_maker: bool
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ToggleMarketMaker { is_market_maker };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ToggleMarketMaker::DISCRIMINATOR);
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

    pub fn initialize_referrer_accounts<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeReferrerAccounts<'info>>,
        referrer_id: String
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeReferrerAccounts { referrer_id };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeReferrerAccounts::DISCRIMINATOR);
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

    pub fn close_referrer_accounts<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseReferrerAccounts<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseReferrerAccounts {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseReferrerAccounts::DISCRIMINATOR);
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

    pub fn edit_ma_type<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, EditMaType<'info>>,
        ma_type: MarginAccountType
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::EditMaType { ma_type };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::EditMaType::DISCRIMINATOR);
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

    pub fn edit_delegated_pubkey<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, EditDelegatedPubkey<'info>>,
        new_key: Pubkey
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::EditDelegatedPubkey { new_key };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::EditDelegatedPubkey::DISCRIMINATOR);
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

    pub fn reset_num_flex_underlyings<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ResetNumFlexUnderlyings<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ResetNumFlexUnderlyings {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ResetNumFlexUnderlyings::DISCRIMINATOR);
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

// I11n
pub mod i11n {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use super::{instructions::*, ID};

    // InitializeZetaPricing
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaPricingI11n<'info> {
        pub accounts: InitializeZetaPricingAccountMetas<'info>,
        pub args: InitializeZetaPricing
    }

    // UpdateZetaPricingPubkeys
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaPricingPubkeysI11n<'info> {
        pub accounts: UpdateZetaPricingPubkeysAccountMetas<'info>,
        pub args: UpdateZetaPricingPubkeys
    }

    // InitializeZetaGroup
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaGroupI11n<'info> {
        pub accounts: InitializeZetaGroupAccountMetas<'info>,
        pub args: InitializeZetaGroup
    }

    // OverrideExpiry
    #[derive(TryFromInstruction)]
    pub struct OverrideExpiryI11n<'info> {
        pub accounts: OverrideExpiryAccountMetas<'info>,
        pub args: OverrideExpiry
    }

    // MigrateToNewCrossMarginAccount
    #[derive(TryFromInstruction)]
    pub struct MigrateToNewCrossMarginAccountI11n<'info> {
        pub accounts: MigrateToNewCrossMarginAccountAccountMetas<'info>,
        pub args: MigrateToNewCrossMarginAccount
    }

    // MigrateToCrossMarginAccount
    #[derive(TryFromInstruction)]
    pub struct MigrateToCrossMarginAccountI11n<'info> {
        pub accounts: MigrateToCrossMarginAccountAccountMetas<'info>,
        pub args: MigrateToCrossMarginAccount
    }

    // InitializeCrossMarginAccountManager
    #[derive(TryFromInstruction)]
    pub struct InitializeCrossMarginAccountManagerI11n<'info> {
        pub accounts: InitializeCrossMarginAccountManagerAccountMetas<'info>,
        pub args: InitializeCrossMarginAccountManager
    }

    // InitializeCrossMarginAccountManagerV2
    #[derive(TryFromInstruction)]
    pub struct InitializeCrossMarginAccountManagerV2I11n<'info> {
        pub accounts: InitializeCrossMarginAccountManagerV2AccountMetas<'info>,
        pub args: InitializeCrossMarginAccountManagerV2
    }

    // InitializeCrossMarginAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeCrossMarginAccountI11n<'info> {
        pub accounts: InitializeCrossMarginAccountAccountMetas<'info>,
        pub args: InitializeCrossMarginAccount
    }

    // InitializeMarginAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeMarginAccountI11n<'info> {
        pub accounts: InitializeMarginAccountAccountMetas<'info>,
        pub args: InitializeMarginAccount
    }

    // InitializeSpreadAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeSpreadAccountI11n<'info> {
        pub accounts: InitializeSpreadAccountAccountMetas<'info>,
        pub args: InitializeSpreadAccount
    }

    // CloseCrossMarginAccountManager
    #[derive(TryFromInstruction)]
    pub struct CloseCrossMarginAccountManagerI11n<'info> {
        pub accounts: CloseCrossMarginAccountManagerAccountMetas<'info>,
        pub args: CloseCrossMarginAccountManager
    }

    // CloseCrossMarginAccount
    #[derive(TryFromInstruction)]
    pub struct CloseCrossMarginAccountI11n<'info> {
        pub accounts: CloseCrossMarginAccountAccountMetas<'info>,
        pub args: CloseCrossMarginAccount
    }

    // CloseMarginAccount
    #[derive(TryFromInstruction)]
    pub struct CloseMarginAccountI11n<'info> {
        pub accounts: CloseMarginAccountAccountMetas<'info>,
        pub args: CloseMarginAccount
    }

    // CloseSpreadAccount
    #[derive(TryFromInstruction)]
    pub struct CloseSpreadAccountI11n<'info> {
        pub accounts: CloseSpreadAccountAccountMetas<'info>,
        pub args: CloseSpreadAccount
    }

    // InitializeUnderlying
    #[derive(TryFromInstruction)]
    pub struct InitializeUnderlyingI11n<'info> {
        pub accounts: InitializeUnderlyingAccountMetas<'info>,
        pub args: InitializeUnderlying
    }

    // InitializePerpSyncQueue
    #[derive(TryFromInstruction)]
    pub struct InitializePerpSyncQueueI11n<'info> {
        pub accounts: InitializePerpSyncQueueAccountMetas<'info>,
        pub args: InitializePerpSyncQueue
    }

    // InitializeMarketIndexes
    #[derive(TryFromInstruction)]
    pub struct InitializeMarketIndexesI11n<'info> {
        pub accounts: InitializeMarketIndexesAccountMetas<'info>,
        pub args: InitializeMarketIndexes
    }

    // InitializeMarketNode
    #[derive(TryFromInstruction)]
    pub struct InitializeMarketNodeI11n<'info> {
        pub accounts: InitializeMarketNodeAccountMetas<'info>,
        pub args: InitializeMarketNode
    }

    // Halt
    #[derive(TryFromInstruction)]
    pub struct HaltI11n<'info> {
        pub accounts: HaltAccountMetas<'info>,
        pub args: Halt
    }

    // Unhalt
    #[derive(TryFromInstruction)]
    pub struct UnhaltI11n<'info> {
        pub accounts: UnhaltAccountMetas<'info>,
        pub args: Unhalt
    }

    // UpdateHaltState
    #[derive(TryFromInstruction)]
    pub struct UpdateHaltStateI11n<'info> {
        pub accounts: UpdateHaltStateAccountMetas<'info>,
        pub args: UpdateHaltState
    }

    // UpdateVolatility
    #[derive(TryFromInstruction)]
    pub struct UpdateVolatilityI11n<'info> {
        pub accounts: UpdateVolatilityAccountMetas<'info>,
        pub args: UpdateVolatility
    }

    // UpdateInterestRate
    #[derive(TryFromInstruction)]
    pub struct UpdateInterestRateI11n<'info> {
        pub accounts: UpdateInterestRateAccountMetas<'info>,
        pub args: UpdateInterestRate
    }

    // AddPerpMarketIndex
    #[derive(TryFromInstruction)]
    pub struct AddPerpMarketIndexI11n<'info> {
        pub accounts: AddPerpMarketIndexAccountMetas<'info>,
        pub args: AddPerpMarketIndex
    }

    // AddMarketIndexes
    #[derive(TryFromInstruction)]
    pub struct AddMarketIndexesI11n<'info> {
        pub accounts: AddMarketIndexesAccountMetas<'info>,
        pub args: AddMarketIndexes
    }

    // InitializeZetaState
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaStateI11n<'info> {
        pub accounts: InitializeZetaStateAccountMetas<'info>,
        pub args: InitializeZetaState
    }

    // InitializeZetaTreasuryWallet
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaTreasuryWalletI11n<'info> {
        pub accounts: InitializeZetaTreasuryWalletAccountMetas<'info>,
        pub args: InitializeZetaTreasuryWallet
    }

    // InitializeZetaReferralsRewardsWallet
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaReferralsRewardsWalletI11n<'info> {
        pub accounts: InitializeZetaReferralsRewardsWalletAccountMetas<'info>,
        pub args: InitializeZetaReferralsRewardsWallet
    }

    // UpdateAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateAdminI11n<'info> {
        pub accounts: UpdateAdminAccountMetas<'info>,
        pub args: UpdateAdmin
    }

    // UpdateSecondaryAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateSecondaryAdminI11n<'info> {
        pub accounts: UpdateSecondaryAdminAccountMetas<'info>,
        pub args: UpdateSecondaryAdmin
    }

    // UpdateTriggerAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateTriggerAdminI11n<'info> {
        pub accounts: UpdateTriggerAdminAccountMetas<'info>,
        pub args: UpdateTriggerAdmin
    }

    // UpdateMaTypeAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateMaTypeAdminI11n<'info> {
        pub accounts: UpdateMaTypeAdminAccountMetas<'info>,
        pub args: UpdateMaTypeAdmin
    }

    // UpdateReferralsAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateReferralsAdminI11n<'info> {
        pub accounts: UpdateReferralsAdminAccountMetas<'info>,
        pub args: UpdateReferralsAdmin
    }

    // UpdatePricingAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdatePricingAdminI11n<'info> {
        pub accounts: UpdatePricingAdminAccountMetas<'info>,
        pub args: UpdatePricingAdmin
    }

    // UpdateMakerRebatePercentage
    #[derive(TryFromInstruction)]
    pub struct UpdateMakerRebatePercentageI11n<'info> {
        pub accounts: UpdateMakerRebatePercentageAccountMetas<'info>,
        pub args: UpdateMakerRebatePercentage
    }

    // UpdateTakeTriggerOrderFeePercentage
    #[derive(TryFromInstruction)]
    pub struct UpdateTakeTriggerOrderFeePercentageI11n<'info> {
        pub accounts: UpdateTakeTriggerOrderFeePercentageAccountMetas<'info>,
        pub args: UpdateTakeTriggerOrderFeePercentage
    }

    // UpdateZetaState
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaStateI11n<'info> {
        pub accounts: UpdateZetaStateAccountMetas<'info>,
        pub args: UpdateZetaState
    }

    // UpdateOracle
    #[derive(TryFromInstruction)]
    pub struct UpdateOracleI11n<'info> {
        pub accounts: UpdateOracleAccountMetas<'info>,
        pub args: UpdateOracle
    }

    // UpdateOracleBackupFeed
    #[derive(TryFromInstruction)]
    pub struct UpdateOracleBackupFeedI11n<'info> {
        pub accounts: UpdateOracleBackupFeedAccountMetas<'info>,
        pub args: UpdateOracleBackupFeed
    }

    // UpdatePricingParameters
    #[derive(TryFromInstruction)]
    pub struct UpdatePricingParametersI11n<'info> {
        pub accounts: UpdatePricingParametersAccountMetas<'info>,
        pub args: UpdatePricingParameters
    }

    // UpdateMarginParameters
    #[derive(TryFromInstruction)]
    pub struct UpdateMarginParametersI11n<'info> {
        pub accounts: UpdateMarginParametersAccountMetas<'info>,
        pub args: UpdateMarginParameters
    }

    // UpdateZetaGroupMarginParameters
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaGroupMarginParametersI11n<'info> {
        pub accounts: UpdateZetaGroupMarginParametersAccountMetas<'info>,
        pub args: UpdateZetaGroupMarginParameters
    }

    // UpdatePerpParameters
    #[derive(TryFromInstruction)]
    pub struct UpdatePerpParametersI11n<'info> {
        pub accounts: UpdatePerpParametersAccountMetas<'info>,
        pub args: UpdatePerpParameters
    }

    // UpdateZetaGroupPerpParameters
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaGroupPerpParametersI11n<'info> {
        pub accounts: UpdateZetaGroupPerpParametersAccountMetas<'info>,
        pub args: UpdateZetaGroupPerpParameters
    }

    // UpdateZetaGroupExpiryParameters
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaGroupExpiryParametersI11n<'info> {
        pub accounts: UpdateZetaGroupExpiryParametersAccountMetas<'info>,
        pub args: UpdateZetaGroupExpiryParameters
    }

    // ToggleZetaGroupPerpsOnly
    #[derive(TryFromInstruction)]
    pub struct ToggleZetaGroupPerpsOnlyI11n<'info> {
        pub accounts: ToggleZetaGroupPerpsOnlyAccountMetas<'info>,
        pub args: ToggleZetaGroupPerpsOnly
    }

    // CleanZetaMarkets
    #[derive(TryFromInstruction)]
    pub struct CleanZetaMarketsI11n<'info> {
        pub accounts: CleanZetaMarketsAccountMetas<'info>,
        pub args: CleanZetaMarkets
    }

    // CleanZetaMarketHalted
    #[derive(TryFromInstruction)]
    pub struct CleanZetaMarketHaltedI11n<'info> {
        pub accounts: CleanZetaMarketHaltedAccountMetas<'info>,
        pub args: CleanZetaMarketHalted
    }

    // SettlePositionsHalted
    #[derive(TryFromInstruction)]
    pub struct SettlePositionsHaltedI11n<'info> {
        pub accounts: SettlePositionsHaltedAccountMetas<'info>,
        pub args: SettlePositionsHalted
    }

    // InitializeMarketStrikes
    #[derive(TryFromInstruction)]
    pub struct InitializeMarketStrikesI11n<'info> {
        pub accounts: InitializeMarketStrikesAccountMetas<'info>,
        pub args: InitializeMarketStrikes
    }

    // ExpireSeriesOverride
    #[derive(TryFromInstruction)]
    pub struct ExpireSeriesOverrideI11n {
        pub accounts: ExpireSeriesOverrideAccountMetas,
        pub args: ExpireSeriesOverride
    }

    // ExpireSeries
    #[derive(TryFromInstruction)]
    pub struct ExpireSeriesI11n {
        pub accounts: ExpireSeriesAccountMetas,
        pub args: ExpireSeries
    }

    // InitializeZetaMarket
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaMarketI11n<'info> {
        pub accounts: InitializeZetaMarketAccountMetas<'info>,
        pub args: InitializeZetaMarket
    }

    // InitializeMarketTifEpochCycle
    #[derive(TryFromInstruction)]
    pub struct InitializeMarketTifEpochCycleI11n<'info> {
        pub accounts: InitializeMarketTifEpochCycleAccountMetas<'info>,
        pub args: InitializeMarketTifEpochCycle
    }

    // UpdatePricingV2
    #[derive(TryFromInstruction)]
    pub struct UpdatePricingV2I11n<'info> {
        pub accounts: UpdatePricingV2AccountMetas<'info>,
        pub args: UpdatePricingV2
    }

    // UpdatePricingV3
    #[derive(TryFromInstruction)]
    pub struct UpdatePricingV3I11n<'info> {
        pub accounts: UpdatePricingV3AccountMetas<'info>,
        pub args: UpdatePricingV3
    }

    // ApplyPerpFunding
    #[derive(TryFromInstruction)]
    pub struct ApplyPerpFundingI11n<'info> {
        pub accounts: ApplyPerpFundingAccountMetas<'info>,
        pub args: ApplyPerpFunding
    }

    // Deposit
    #[derive(TryFromInstruction)]
    pub struct DepositI11n<'info> {
        pub accounts: DepositAccountMetas<'info>,
        pub args: Deposit
    }

    // DepositV2
    #[derive(TryFromInstruction)]
    pub struct DepositV2I11n<'info> {
        pub accounts: DepositV2AccountMetas<'info>,
        pub args: DepositV2
    }

    // DepositPermissionless
    #[derive(TryFromInstruction)]
    pub struct DepositPermissionlessI11n<'info> {
        pub accounts: DepositPermissionlessAccountMetas<'info>,
        pub args: DepositPermissionless
    }

    // DepositInsuranceVault
    #[derive(TryFromInstruction)]
    pub struct DepositInsuranceVaultI11n<'info> {
        pub accounts: DepositInsuranceVaultAccountMetas<'info>,
        pub args: DepositInsuranceVault
    }

    // DepositInsuranceVaultV2
    #[derive(TryFromInstruction)]
    pub struct DepositInsuranceVaultV2I11n<'info> {
        pub accounts: DepositInsuranceVaultV2AccountMetas<'info>,
        pub args: DepositInsuranceVaultV2
    }

    // ChooseAirdropCommunity
    #[derive(TryFromInstruction)]
    pub struct ChooseAirdropCommunityI11n<'info> {
        pub accounts: ChooseAirdropCommunityAccountMetas<'info>,
        pub args: ChooseAirdropCommunity
    }

    // Withdraw
    #[derive(TryFromInstruction)]
    pub struct WithdrawI11n<'info> {
        pub accounts: WithdrawAccountMetas<'info>,
        pub args: Withdraw
    }

    // WithdrawV2
    #[derive(TryFromInstruction)]
    pub struct WithdrawV2I11n<'info> {
        pub accounts: WithdrawV2AccountMetas<'info>,
        pub args: WithdrawV2
    }

    // WithdrawInsuranceVault
    #[derive(TryFromInstruction)]
    pub struct WithdrawInsuranceVaultI11n<'info> {
        pub accounts: WithdrawInsuranceVaultAccountMetas<'info>,
        pub args: WithdrawInsuranceVault
    }

    // WithdrawInsuranceVaultV2
    #[derive(TryFromInstruction)]
    pub struct WithdrawInsuranceVaultV2I11n<'info> {
        pub accounts: WithdrawInsuranceVaultV2AccountMetas<'info>,
        pub args: WithdrawInsuranceVaultV2
    }

    // InitializeOpenOrders
    #[derive(TryFromInstruction)]
    pub struct InitializeOpenOrdersI11n<'info> {
        pub accounts: InitializeOpenOrdersAccountMetas<'info>,
        pub args: InitializeOpenOrders
    }

    // InitializeOpenOrdersV2
    #[derive(TryFromInstruction)]
    pub struct InitializeOpenOrdersV2I11n<'info> {
        pub accounts: InitializeOpenOrdersV2AccountMetas<'info>,
        pub args: InitializeOpenOrdersV2
    }

    // InitializeOpenOrdersV3
    #[derive(TryFromInstruction)]
    pub struct InitializeOpenOrdersV3I11n<'info> {
        pub accounts: InitializeOpenOrdersV3AccountMetas<'info>,
        pub args: InitializeOpenOrdersV3
    }

    // CloseOpenOrders
    #[derive(TryFromInstruction)]
    pub struct CloseOpenOrdersI11n<'info> {
        pub accounts: CloseOpenOrdersAccountMetas<'info>,
        pub args: CloseOpenOrders
    }

    // CloseOpenOrdersV2
    #[derive(TryFromInstruction)]
    pub struct CloseOpenOrdersV2I11n<'info> {
        pub accounts: CloseOpenOrdersV2AccountMetas<'info>,
        pub args: CloseOpenOrdersV2
    }

    // CloseOpenOrdersV3
    #[derive(TryFromInstruction)]
    pub struct CloseOpenOrdersV3I11n<'info> {
        pub accounts: CloseOpenOrdersV3AccountMetas<'info>,
        pub args: CloseOpenOrdersV3
    }

    // CloseOpenOrdersV4
    #[derive(TryFromInstruction)]
    pub struct CloseOpenOrdersV4I11n<'info> {
        pub accounts: CloseOpenOrdersV4AccountMetas<'info>,
        pub args: CloseOpenOrdersV4
    }

    // InitializeWhitelistDepositAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeWhitelistDepositAccountI11n<'info> {
        pub accounts: InitializeWhitelistDepositAccountAccountMetas<'info>,
        pub args: InitializeWhitelistDepositAccount
    }

    // InitializeWhitelistInsuranceAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeWhitelistInsuranceAccountI11n<'info> {
        pub accounts: InitializeWhitelistInsuranceAccountAccountMetas<'info>,
        pub args: InitializeWhitelistInsuranceAccount
    }

    // InitializeWhitelistTradingFeesAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeWhitelistTradingFeesAccountI11n<'info> {
        pub accounts: InitializeWhitelistTradingFeesAccountAccountMetas<'info>,
        pub args: InitializeWhitelistTradingFeesAccount
    }

    // InitializeInsuranceDepositAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeInsuranceDepositAccountI11n<'info> {
        pub accounts: InitializeInsuranceDepositAccountAccountMetas<'info>,
        pub args: InitializeInsuranceDepositAccount
    }

    // InitializeCombinedInsuranceVault
    #[derive(TryFromInstruction)]
    pub struct InitializeCombinedInsuranceVaultI11n<'info> {
        pub accounts: InitializeCombinedInsuranceVaultAccountMetas<'info>,
        pub args: InitializeCombinedInsuranceVault
    }

    // InitializeCombinedVault
    #[derive(TryFromInstruction)]
    pub struct InitializeCombinedVaultI11n<'info> {
        pub accounts: InitializeCombinedVaultAccountMetas<'info>,
        pub args: InitializeCombinedVault
    }

    // InitializeCombinedSocializedLossAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeCombinedSocializedLossAccountI11n<'info> {
        pub accounts: InitializeCombinedSocializedLossAccountAccountMetas<'info>,
        pub args: InitializeCombinedSocializedLossAccount
    }

    // PlaceOrder
    #[derive(TryFromInstruction)]
    pub struct PlaceOrderI11n<'info> {
        pub accounts: PlaceOrderAccountMetas<'info>,
        pub args: PlaceOrder
    }

    // PlaceOrderV2
    #[derive(TryFromInstruction)]
    pub struct PlaceOrderV2I11n<'info> {
        pub accounts: PlaceOrderV2AccountMetas<'info>,
        pub args: PlaceOrderV2
    }

    // PlaceOrderV3
    #[derive(TryFromInstruction)]
    pub struct PlaceOrderV3I11n<'info> {
        pub accounts: PlaceOrderV3AccountMetas<'info>,
        pub args: PlaceOrderV3
    }

    // PlacePerpOrder
    #[derive(TryFromInstruction)]
    pub struct PlacePerpOrderI11n<'info> {
        pub accounts: PlacePerpOrderAccountMetas<'info>,
        pub args: PlacePerpOrder
    }

    // PlacePerpOrderV2
    #[derive(TryFromInstruction)]
    pub struct PlacePerpOrderV2I11n<'info> {
        pub accounts: PlacePerpOrderV2AccountMetas<'info>,
        pub args: PlacePerpOrderV2
    }

    // PlaceOrderV4
    #[derive(TryFromInstruction)]
    pub struct PlaceOrderV4I11n<'info> {
        pub accounts: PlaceOrderV4AccountMetas<'info>,
        pub args: PlaceOrderV4
    }

    // PlacePerpOrderV3
    #[derive(TryFromInstruction)]
    pub struct PlacePerpOrderV3I11n<'info> {
        pub accounts: PlacePerpOrderV3AccountMetas<'info>,
        pub args: PlacePerpOrderV3
    }

    // PlacePerpOrderV4
    #[derive(TryFromInstruction)]
    pub struct PlacePerpOrderV4I11n<'info> {
        pub accounts: PlacePerpOrderV4AccountMetas<'info>,
        pub args: PlacePerpOrderV4
    }

    // PlaceMultiOrders
    #[derive(TryFromInstruction)]
    pub struct PlaceMultiOrdersI11n<'info> {
        pub accounts: PlaceMultiOrdersAccountMetas<'info>,
        pub args: PlaceMultiOrders
    }

    // PlaceTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct PlaceTriggerOrderI11n<'info> {
        pub accounts: PlaceTriggerOrderAccountMetas<'info>,
        pub args: PlaceTriggerOrder
    }

    // ExecuteTriggerOrderV2
    #[derive(TryFromInstruction)]
    pub struct ExecuteTriggerOrderV2I11n<'info> {
        pub accounts: ExecuteTriggerOrderV2AccountMetas<'info>,
        pub args: ExecuteTriggerOrderV2
    }

    // TakeTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct TakeTriggerOrderI11n<'info> {
        pub accounts: TakeTriggerOrderAccountMetas<'info>,
        pub args: TakeTriggerOrder
    }

    // ExecuteTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct ExecuteTriggerOrderI11n<'info> {
        pub accounts: ExecuteTriggerOrderAccountMetas<'info>,
        pub args: ExecuteTriggerOrder
    }

    // ForceCancelTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct ForceCancelTriggerOrderI11n<'info> {
        pub accounts: ForceCancelTriggerOrderAccountMetas<'info>,
        pub args: ForceCancelTriggerOrder
    }

    // CancelTriggerOrderV2
    #[derive(TryFromInstruction)]
    pub struct CancelTriggerOrderV2I11n<'info> {
        pub accounts: CancelTriggerOrderV2AccountMetas<'info>,
        pub args: CancelTriggerOrderV2
    }

    // CancelTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct CancelTriggerOrderI11n<'info> {
        pub accounts: CancelTriggerOrderAccountMetas<'info>,
        pub args: CancelTriggerOrder
    }

    // UpdateMinLot
    #[derive(TryFromInstruction)]
    pub struct UpdateMinLotI11n<'info> {
        pub accounts: UpdateMinLotAccountMetas<'info>,
        pub args: UpdateMinLot
    }

    // UpdateTickSize
    #[derive(TryFromInstruction)]
    pub struct UpdateTickSizeI11n<'info> {
        pub accounts: UpdateTickSizeAccountMetas<'info>,
        pub args: UpdateTickSize
    }

    // InitializeMinLotsAndTickSizes
    #[derive(TryFromInstruction)]
    pub struct InitializeMinLotsAndTickSizesI11n<'info> {
        pub accounts: InitializeMinLotsAndTickSizesAccountMetas<'info>,
        pub args: InitializeMinLotsAndTickSizes
    }

    // EditTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct EditTriggerOrderI11n<'info> {
        pub accounts: EditTriggerOrderAccountMetas<'info>,
        pub args: EditTriggerOrder
    }

    // EditTriggerOrderV2
    #[derive(TryFromInstruction)]
    pub struct EditTriggerOrderV2I11n<'info> {
        pub accounts: EditTriggerOrderV2AccountMetas<'info>,
        pub args: EditTriggerOrderV2
    }

    // CancelOrder
    #[derive(TryFromInstruction)]
    pub struct CancelOrderI11n<'info> {
        pub accounts: CancelOrderAccountMetas<'info>,
        pub args: CancelOrder
    }

    // CancelOrderNoError
    #[derive(TryFromInstruction)]
    pub struct CancelOrderNoErrorI11n<'info> {
        pub accounts: CancelOrderNoErrorAccountMetas<'info>,
        pub args: CancelOrderNoError
    }

    // CancelAllMarketOrders
    #[derive(TryFromInstruction)]
    pub struct CancelAllMarketOrdersI11n<'info> {
        pub accounts: CancelAllMarketOrdersAccountMetas<'info>,
        pub args: CancelAllMarketOrders
    }

    // CancelOrderHalted
    #[derive(TryFromInstruction)]
    pub struct CancelOrderHaltedI11n<'info> {
        pub accounts: CancelOrderHaltedAccountMetas<'info>,
        pub args: CancelOrderHalted
    }

    // CancelOrderByClientOrderId
    #[derive(TryFromInstruction)]
    pub struct CancelOrderByClientOrderIdI11n<'info> {
        pub accounts: CancelOrderByClientOrderIdAccountMetas<'info>,
        pub args: CancelOrderByClientOrderId
    }

    // CancelOrderByClientOrderIdNoError
    #[derive(TryFromInstruction)]
    pub struct CancelOrderByClientOrderIdNoErrorI11n<'info> {
        pub accounts: CancelOrderByClientOrderIdNoErrorAccountMetas<'info>,
        pub args: CancelOrderByClientOrderIdNoError
    }

    // PruneExpiredTifOrders
    #[derive(TryFromInstruction)]
    pub struct PruneExpiredTifOrdersI11n<'info> {
        pub accounts: PruneExpiredTifOrdersAccountMetas<'info>,
        pub args: PruneExpiredTifOrders
    }

    // PruneExpiredTifOrdersV2
    #[derive(TryFromInstruction)]
    pub struct PruneExpiredTifOrdersV2I11n<'info> {
        pub accounts: PruneExpiredTifOrdersV2AccountMetas<'info>,
        pub args: PruneExpiredTifOrdersV2
    }

    // ForceCancelOrderByOrderIdV2
    #[derive(TryFromInstruction)]
    pub struct ForceCancelOrderByOrderIdV2I11n<'info> {
        pub accounts: ForceCancelOrderByOrderIdV2AccountMetas<'info>,
        pub args: ForceCancelOrderByOrderIdV2
    }

    // ForceCancelOrderByOrderId
    #[derive(TryFromInstruction)]
    pub struct ForceCancelOrderByOrderIdI11n<'info> {
        pub accounts: ForceCancelOrderByOrderIdAccountMetas<'info>,
        pub args: ForceCancelOrderByOrderId
    }

    // ForceCancelOrdersV2
    #[derive(TryFromInstruction)]
    pub struct ForceCancelOrdersV2I11n<'info> {
        pub accounts: ForceCancelOrdersV2AccountMetas<'info>,
        pub args: ForceCancelOrdersV2
    }

    // ForceCancelOrders
    #[derive(TryFromInstruction)]
    pub struct ForceCancelOrdersI11n<'info> {
        pub accounts: ForceCancelOrdersAccountMetas<'info>,
        pub args: ForceCancelOrders
    }

    // CrankEventQueue
    #[derive(TryFromInstruction)]
    pub struct CrankEventQueueI11n<'info> {
        pub accounts: CrankEventQueueAccountMetas<'info>,
        pub args: CrankEventQueue
    }

    // CollectTreasuryFunds
    #[derive(TryFromInstruction)]
    pub struct CollectTreasuryFundsI11n<'info> {
        pub accounts: CollectTreasuryFundsAccountMetas<'info>,
        pub args: CollectTreasuryFunds
    }

    // TreasuryMovement
    #[derive(TryFromInstruction)]
    pub struct TreasuryMovementI11n<'info> {
        pub accounts: TreasuryMovementAccountMetas<'info>,
        pub args: TreasuryMovement
    }

    // RebalanceInsuranceVault
    #[derive(TryFromInstruction)]
    pub struct RebalanceInsuranceVaultI11n<'info> {
        pub accounts: RebalanceInsuranceVaultAccountMetas<'info>,
        pub args: RebalanceInsuranceVault
    }

    // LiquidateV2
    #[derive(TryFromInstruction)]
    pub struct LiquidateV2I11n<'info> {
        pub accounts: LiquidateV2AccountMetas<'info>,
        pub args: LiquidateV2
    }

    // Liquidate
    #[derive(TryFromInstruction)]
    pub struct LiquidateI11n<'info> {
        pub accounts: LiquidateAccountMetas<'info>,
        pub args: Liquidate
    }

    // BurnVaultTokens
    #[derive(TryFromInstruction)]
    pub struct BurnVaultTokensI11n<'info> {
        pub accounts: BurnVaultTokensAccountMetas<'info>,
        pub args: BurnVaultTokens
    }

    // SettleDexFunds
    #[derive(TryFromInstruction)]
    pub struct SettleDexFundsI11n<'info> {
        pub accounts: SettleDexFundsAccountMetas<'info>,
        pub args: SettleDexFunds
    }

    // PositionMovement
    #[derive(TryFromInstruction)]
    pub struct PositionMovementI11n<'info> {
        pub accounts: PositionMovementAccountMetas<'info>,
        pub args: PositionMovement
    }

    // TransferExcessSpreadBalance
    #[derive(TryFromInstruction)]
    pub struct TransferExcessSpreadBalanceI11n<'info> {
        pub accounts: TransferExcessSpreadBalanceAccountMetas<'info>,
        pub args: TransferExcessSpreadBalance
    }

    // ToggleMarketMaker
    #[derive(TryFromInstruction)]
    pub struct ToggleMarketMakerI11n<'info> {
        pub accounts: ToggleMarketMakerAccountMetas<'info>,
        pub args: ToggleMarketMaker
    }

    // InitializeReferrerAccounts
    #[derive(TryFromInstruction)]
    pub struct InitializeReferrerAccountsI11n<'info> {
        pub accounts: InitializeReferrerAccountsAccountMetas<'info>,
        pub args: InitializeReferrerAccounts
    }

    // CloseReferrerAccounts
    #[derive(TryFromInstruction)]
    pub struct CloseReferrerAccountsI11n<'info> {
        pub accounts: CloseReferrerAccountsAccountMetas<'info>,
        pub args: CloseReferrerAccounts
    }

    // EditMaType
    #[derive(TryFromInstruction)]
    pub struct EditMaTypeI11n<'info> {
        pub accounts: EditMaTypeAccountMetas<'info>,
        pub args: EditMaType
    }

    // EditDelegatedPubkey
    #[derive(TryFromInstruction)]
    pub struct EditDelegatedPubkeyI11n<'info> {
        pub accounts: EditDelegatedPubkeyAccountMetas<'info>,
        pub args: EditDelegatedPubkey
    }

    // ResetNumFlexUnderlyings
    #[derive(TryFromInstruction)]
    pub struct ResetNumFlexUnderlyingsI11n<'info> {
        pub accounts: ResetNumFlexUnderlyingsAccountMetas<'info>,
        pub args: ResetNumFlexUnderlyings
    }

    //Accounts
    #[derive(TryFromAccountMetas)]
    pub struct InitializeZetaPricingAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateZetaPricingPubkeysAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeZetaGroupAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub underlying_mint: &'info AccountMeta,
        pub zeta_program: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub perp_sync_queue: &'info AccountMeta,
        pub underlying: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub insurance_vault: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub usdc_mint: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct OverrideExpiryAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct MigrateToNewCrossMarginAccountAccountMetas<'info> {
        pub new_cross_margin_account: &'info AccountMeta,
        pub old_cross_margin_account: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct MigrateToCrossMarginAccountAccountMetas<'info> {
        pub cross_margin_account: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeCrossMarginAccountManagerAccountMetas<'info> {
        pub cross_margin_account_manager: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub zeta_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeCrossMarginAccountManagerV2AccountMetas<'info> {
        pub cross_margin_account_manager: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub zeta_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeCrossMarginAccountAccountMetas<'info> {
        pub cross_margin_account: &'info AccountMeta,
        pub cross_margin_account_manager: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub zeta_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeMarginAccountAccountMetas<'info> {
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub zeta_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeSpreadAccountAccountMetas<'info> {
        pub spread_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub zeta_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseCrossMarginAccountManagerAccountMetas<'info> {
        pub cross_margin_account_manager: &'info AccountMeta,
        pub authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseCrossMarginAccountAccountMetas<'info> {
        pub cross_margin_account: &'info AccountMeta,
        pub cross_margin_account_manager: &'info AccountMeta,
        pub authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseMarginAccountAccountMetas<'info> {
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseSpreadAccountAccountMetas<'info> {
        pub spread_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeUnderlyingAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub zeta_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub underlying: &'info AccountMeta,
        pub underlying_mint: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializePerpSyncQueueAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub zeta_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub perp_sync_queue: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeMarketIndexesAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub market_indexes: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeMarketNodeAccountMetas<'info> {
        pub zeta_group: &'info AccountMeta,
        pub market_node: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct HaltAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UnhaltAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateHaltStateAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateVolatilityAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateInterestRateAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct AddPerpMarketIndexAccountMetas<'info> {
        pub market_indexes: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct AddMarketIndexesAccountMetas<'info> {
        pub market_indexes: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeZetaStateAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub treasury_wallet: &'info AccountMeta,
        pub referrals_admin: &'info AccountMeta,
        pub referrals_rewards_wallet: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub usdc_mint: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub secondary_admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeZetaTreasuryWalletAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub treasury_wallet: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub usdc_mint: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeZetaReferralsRewardsWalletAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub referrals_rewards_wallet: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub usdc_mint: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateAdminAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub new_admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateSecondaryAdminAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub new_admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateTriggerAdminAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub new_admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateMaTypeAdminAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub new_admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateReferralsAdminAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub new_admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdatePricingAdminAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub new_admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateMakerRebatePercentageAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateTakeTriggerOrderFeePercentageAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateZetaStateAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateOracleAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateOracleBackupFeedAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdatePricingParametersAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateMarginParametersAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateZetaGroupMarginParametersAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdatePerpParametersAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateZetaGroupPerpParametersAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateZetaGroupExpiryParametersAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ToggleZetaGroupPerpsOnlyAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CleanZetaMarketsAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CleanZetaMarketHaltedAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub bids: &'info AccountMeta,
        pub asks: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SettlePositionsHaltedAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeMarketStrikesAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ExpireSeriesOverrideAccountMetas {

    }

    #[derive(TryFromAccountMetas)]
    pub struct ExpireSeriesAccountMetas {

    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeZetaMarketAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub market_indexes: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub request_queue: &'info AccountMeta,
        pub event_queue: &'info AccountMeta,
        pub bids: &'info AccountMeta,
        pub asks: &'info AccountMeta,
        pub base_mint: &'info AccountMeta,
        pub quote_mint: &'info AccountMeta,
        pub zeta_base_vault: &'info AccountMeta,
        pub zeta_quote_vault: &'info AccountMeta,
        pub dex_base_vault: &'info AccountMeta,
        pub dex_quote_vault: &'info AccountMeta,
        pub vault_owner: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeMarketTifEpochCycleAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdatePricingV2AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub perp_market: &'info AccountMeta,
        pub perp_bids: &'info AccountMeta,
        pub perp_asks: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdatePricingV3AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub perp_market: &'info AccountMeta,
        pub perp_bids: &'info AccountMeta,
        pub perp_asks: &'info AccountMeta,
        pub pricing_admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ApplyPerpFundingAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct DepositAccountMetas<'info> {
        pub zeta_group: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub user_token_account: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct DepositV2AccountMetas<'info> {
        pub margin_account: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub user_token_account: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct DepositPermissionlessAccountMetas<'info> {
        pub cross_margin_account: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub deposit_token_acc: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct DepositInsuranceVaultAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub insurance_vault: &'info AccountMeta,
        pub insurance_deposit_account: &'info AccountMeta,
        pub user_token_account: &'info AccountMeta,
        pub zeta_vault: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct DepositInsuranceVaultV2AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub insurance_vault: &'info AccountMeta,
        pub insurance_deposit_account: &'info AccountMeta,
        pub user_token_account: &'info AccountMeta,
        pub zeta_vault: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ChooseAirdropCommunityAccountMetas<'info> {
        pub cross_margin_account_manager: &'info AccountMeta,
        pub authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct WithdrawAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub user_token_account: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct WithdrawV2AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub user_token_account: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct WithdrawInsuranceVaultAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub insurance_vault: &'info AccountMeta,
        pub insurance_deposit_account: &'info AccountMeta,
        pub user_token_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct WithdrawInsuranceVaultV2AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub insurance_vault: &'info AccountMeta,
        pub insurance_deposit_account: &'info AccountMeta,
        pub user_token_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeOpenOrdersAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub open_orders_map: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeOpenOrdersV2AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub open_orders_map: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeOpenOrdersV3AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub cross_margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub open_orders_map: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseOpenOrdersAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub open_orders_map: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseOpenOrdersV2AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub open_orders_map: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseOpenOrdersV3AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub cross_margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub open_orders_map: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseOpenOrdersV4AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub cross_margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub open_orders_map: &'info AccountMeta,
        pub event_queue: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeWhitelistDepositAccountAccountMetas<'info> {
        pub whitelist_deposit_account: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub user: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeWhitelistInsuranceAccountAccountMetas<'info> {
        pub whitelist_insurance_account: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub user: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeWhitelistTradingFeesAccountAccountMetas<'info> {
        pub whitelist_trading_fees_account: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub user: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeInsuranceDepositAccountAccountMetas<'info> {
        pub insurance_deposit_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub whitelist_insurance_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeCombinedInsuranceVaultAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub insurance_vault: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub usdc_mint: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeCombinedVaultAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub usdc_mint: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeCombinedSocializedLossAccountAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub usdc_mint: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlaceOrderAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market_accounts: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market_node: &'info AccountMeta,
        pub market_mint: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlaceOrderV2AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market_accounts: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market_node: &'info AccountMeta,
        pub market_mint: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlaceOrderV3AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market_accounts: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market_node: &'info AccountMeta,
        pub market_mint: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlacePerpOrderAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market_accounts: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market_mint: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
        pub perp_sync_queue: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlacePerpOrderV2AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market_accounts: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market_mint: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
        pub perp_sync_queue: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlaceOrderV4AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market_accounts: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market_node: &'info AccountMeta,
        pub market_mint: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlacePerpOrderV3AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market_accounts: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market_mint: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
        pub perp_sync_queue: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlacePerpOrderV4AccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub place_order_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlaceMultiOrdersAccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub request_queue: &'info AccountMeta,
        pub event_queue: &'info AccountMeta,
        pub bids: &'info AccountMeta,
        pub asks: &'info AccountMeta,
        pub market_base_vault: &'info AccountMeta,
        pub market_quote_vault: &'info AccountMeta,
        pub zeta_base_vault: &'info AccountMeta,
        pub zeta_quote_vault: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market_base_mint: &'info AccountMeta,
        pub market_quote_mint: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
        pub perp_sync_queue: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PlaceTriggerOrderAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub open_orders: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub trigger_order: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub market: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ExecuteTriggerOrderV2AccountMetas<'info> {
        pub payer: &'info AccountMeta,
        pub trigger_order: &'info AccountMeta,
        pub place_order_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct TakeTriggerOrderAccountMetas<'info> {
        pub trigger_order: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub bids: &'info AccountMeta,
        pub asks: &'info AccountMeta,
        pub taker: &'info AccountMeta,
        pub taker_margin_account: &'info AccountMeta,
        pub order_margin_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ExecuteTriggerOrderAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub trigger_order: &'info AccountMeta,
        pub place_order_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ForceCancelTriggerOrderAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub trigger_order: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CancelTriggerOrderV2AccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub trigger_order: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CancelTriggerOrderAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub trigger_order: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateMinLotAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateTickSizeAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeMinLotsAndTickSizesAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct EditTriggerOrderAccountMetas<'info> {
        pub owner: &'info AccountMeta,
        pub trigger_order: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct EditTriggerOrderV2AccountMetas<'info> {
        pub owner: &'info AccountMeta,
        pub trigger_order: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CancelOrderAccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CancelOrderNoErrorAccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CancelAllMarketOrdersAccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CancelOrderHaltedAccountMetas<'info> {
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CancelOrderByClientOrderIdAccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CancelOrderByClientOrderIdNoErrorAccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PruneExpiredTifOrdersAccountMetas<'info> {
        pub dex_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub bids: &'info AccountMeta,
        pub asks: &'info AccountMeta,
        pub event_queue: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PruneExpiredTifOrdersV2AccountMetas<'info> {
        pub dex_program: &'info AccountMeta,
        pub state: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub bids: &'info AccountMeta,
        pub asks: &'info AccountMeta,
        pub event_queue: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ForceCancelOrderByOrderIdV2AccountMetas<'info> {
        pub pricing: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ForceCancelOrderByOrderIdAccountMetas<'info> {
        pub zeta_group: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ForceCancelOrdersV2AccountMetas<'info> {
        pub pricing: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ForceCancelOrdersAccountMetas<'info> {
        pub zeta_group: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub cancel_accounts: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CrankEventQueueAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub event_queue: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub perp_sync_queue: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CollectTreasuryFundsAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub treasury_wallet: &'info AccountMeta,
        pub collection_token_account: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct TreasuryMovementAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub insurance_vault: &'info AccountMeta,
        pub treasury_wallet: &'info AccountMeta,
        pub referrals_rewards_wallet: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct RebalanceInsuranceVaultAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_vault: &'info AccountMeta,
        pub insurance_vault: &'info AccountMeta,
        pub treasury_wallet: &'info AccountMeta,
        pub socialized_loss_account: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct LiquidateV2AccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub liquidator: &'info AccountMeta,
        pub liquidator_account: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub liquidated_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct LiquidateAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub liquidator: &'info AccountMeta,
        pub liquidator_margin_account: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub liquidated_margin_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct BurnVaultTokensAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub mint: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SettleDexFundsAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub zeta_base_vault: &'info AccountMeta,
        pub zeta_quote_vault: &'info AccountMeta,
        pub dex_base_vault: &'info AccountMeta,
        pub dex_quote_vault: &'info AccountMeta,
        pub vault_owner: &'info AccountMeta,
        pub mint_authority: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct PositionMovementAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub zeta_group: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub spread_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub greeks: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
        pub oracle_backup_feed: &'info AccountMeta,
        pub oracle_backup_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct TransferExcessSpreadBalanceAccountMetas<'info> {
        pub zeta_group: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
        pub spread_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ToggleMarketMakerAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeReferrerAccountsAccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub referrer_id_account: &'info AccountMeta,
        pub referrer_pubkey_account: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseReferrerAccountsAccountMetas<'info> {
        pub referrer_id_account: &'info AccountMeta,
        pub referrer_pubkey_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct EditMaTypeAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub margin_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct EditDelegatedPubkeyAccountMetas<'info> {
        pub margin_account: &'info AccountMeta,
        pub authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ResetNumFlexUnderlyingsAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub admin: &'info AccountMeta,
    }
}

// Instructions
pub mod instructions {
    use super::*;

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaPricing {
        pub args: InitializeZetaPricingArgs,
    }
    
    impl Discriminator for InitializeZetaPricing {
        const DISCRIMINATOR: [u8; 8] = [[35,209,180,29,245,199,125,16]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaPricingPubkeys {
        pub args: UpdateZetaPricingPubkeysArgs,
    }
    
    impl Discriminator for UpdateZetaPricingPubkeys {
        const DISCRIMINATOR: [u8; 8] = [[169,221,23,248,219,122,142,158]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaGroup {
        pub args: InitializeZetaGroupArgs,
    }
    
    impl Discriminator for InitializeZetaGroup {
        const DISCRIMINATOR: [u8; 8] = [[6,135,36,232,35,39,250,71]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct OverrideExpiry {
        pub args: OverrideExpiryArgs,
    }
    
    impl Discriminator for OverrideExpiry {
        const DISCRIMINATOR: [u8; 8] = [[129,197,117,114,108,119,207,136]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct MigrateToNewCrossMarginAccount {

    }
    
    impl Discriminator for MigrateToNewCrossMarginAccount {
        const DISCRIMINATOR: [u8; 8] = [[183,45,251,109,134,108,191,243]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct MigrateToCrossMarginAccount {

    }
    
    impl Discriminator for MigrateToCrossMarginAccount {
        const DISCRIMINATOR: [u8; 8] = [[157,53,107,104,184,189,100,220]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCrossMarginAccountManager {

    }
    
    impl Discriminator for InitializeCrossMarginAccountManager {
        const DISCRIMINATOR: [u8; 8] = [[72,154,15,28,165,215,209,199]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCrossMarginAccountManagerV2 {
        pub referrer: Option<Pubkey>,
    }
    
    impl Discriminator for InitializeCrossMarginAccountManagerV2 {
        const DISCRIMINATOR: [u8; 8] = [[192,5,219,118,210,135,23,158]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCrossMarginAccount {
        pub subaccount_index: u8,
    }
    
    impl Discriminator for InitializeCrossMarginAccount {
        const DISCRIMINATOR: [u8; 8] = [[27,26,228,50,210,211,205,94]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarginAccount {

    }
    
    impl Discriminator for InitializeMarginAccount {
        const DISCRIMINATOR: [u8; 8] = [[67,235,66,102,167,171,120,197]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeSpreadAccount {

    }
    
    impl Discriminator for InitializeSpreadAccount {
        const DISCRIMINATOR: [u8; 8] = [[206,86,251,27,91,111,23,211]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseCrossMarginAccountManager {

    }
    
    impl Discriminator for CloseCrossMarginAccountManager {
        const DISCRIMINATOR: [u8; 8] = [[232,182,182,137,86,88,118,252]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseCrossMarginAccount {
        pub subaccount_index: u8,
    }
    
    impl Discriminator for CloseCrossMarginAccount {
        const DISCRIMINATOR: [u8; 8] = [[203,196,187,60,13,170,190,69]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseMarginAccount {

    }
    
    impl Discriminator for CloseMarginAccount {
        const DISCRIMINATOR: [u8; 8] = [[105,215,41,239,166,207,1,103]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseSpreadAccount {

    }
    
    impl Discriminator for CloseSpreadAccount {
        const DISCRIMINATOR: [u8; 8] = [[190,228,253,16,201,148,161,240]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeUnderlying {
        pub flex_underlying: bool,
    }
    
    impl Discriminator for InitializeUnderlying {
        const DISCRIMINATOR: [u8; 8] = [[114,108,213,92,175,124,43,19]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePerpSyncQueue {
        pub nonce: u8,
        pub asset: Asset,
    }
    
    impl Discriminator for InitializePerpSyncQueue {
        const DISCRIMINATOR: [u8; 8] = [[10,55,154,224,129,174,161,8]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarketIndexes {
        pub nonce: u8,
        pub asset: Asset,
    }
    
    impl Discriminator for InitializeMarketIndexes {
        const DISCRIMINATOR: [u8; 8] = [[91,63,205,144,20,83,177,120]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarketNode {
        pub args: InitializeMarketNodeArgs,
    }
    
    impl Discriminator for InitializeMarketNode {
        const DISCRIMINATOR: [u8; 8] = [[50,118,21,21,179,248,23,128]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Halt {
        pub asset: Asset,
    }
    
    impl Discriminator for Halt {
        const DISCRIMINATOR: [u8; 8] = [[24,156,8,121,65,3,5,82]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Unhalt {
        pub asset: Asset,
    }
    
    impl Discriminator for Unhalt {
        const DISCRIMINATOR: [u8; 8] = [[249,140,27,213,128,130,207,113]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateHaltState {
        pub args: HaltStateArgs,
    }
    
    impl Discriminator for UpdateHaltState {
        const DISCRIMINATOR: [u8; 8] = [[215,45,53,162,149,138,5,63]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateVolatility {
        pub args: UpdateVolatilityArgs,
    }
    
    impl Discriminator for UpdateVolatility {
        const DISCRIMINATOR: [u8; 8] = [[190,105,116,221,229,198,208,83]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateInterestRate {
        pub args: UpdateInterestRateArgs,
    }
    
    impl Discriminator for UpdateInterestRate {
        const DISCRIMINATOR: [u8; 8] = [[75,8,255,41,123,59,135,238]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct AddPerpMarketIndex {
        pub asset: Asset,
    }
    
    impl Discriminator for AddPerpMarketIndex {
        const DISCRIMINATOR: [u8; 8] = [[122,40,14,64,169,18,231,136]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct AddMarketIndexes {

    }
    
    impl Discriminator for AddMarketIndexes {
        const DISCRIMINATOR: [u8; 8] = [[94,246,144,175,4,164,233,252]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaState {
        pub args: InitializeStateArgs,
    }
    
    impl Discriminator for InitializeZetaState {
        const DISCRIMINATOR: [u8; 8] = [[68,39,75,142,191,146,94,222]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaTreasuryWallet {

    }
    
    impl Discriminator for InitializeZetaTreasuryWallet {
        const DISCRIMINATOR: [u8; 8] = [[249,57,187,102,184,104,37,231]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaReferralsRewardsWallet {

    }
    
    impl Discriminator for InitializeZetaReferralsRewardsWallet {
        const DISCRIMINATOR: [u8; 8] = [[245,229,223,120,7,134,247,248]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateAdmin {

    }
    
    impl Discriminator for UpdateAdmin {
        const DISCRIMINATOR: [u8; 8] = [[161,176,40,213,60,184,179,228]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSecondaryAdmin {

    }
    
    impl Discriminator for UpdateSecondaryAdmin {
        const DISCRIMINATOR: [u8; 8] = [[84,230,26,75,2,179,175,234]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateTriggerAdmin {

    }
    
    impl Discriminator for UpdateTriggerAdmin {
        const DISCRIMINATOR: [u8; 8] = [[241,100,110,210,57,121,119,108]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateMaTypeAdmin {

    }
    
    impl Discriminator for UpdateMaTypeAdmin {
        const DISCRIMINATOR: [u8; 8] = [[44,185,150,102,112,28,129,239]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateReferralsAdmin {

    }
    
    impl Discriminator for UpdateReferralsAdmin {
        const DISCRIMINATOR: [u8; 8] = [[73,144,92,119,74,106,16,200]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePricingAdmin {

    }
    
    impl Discriminator for UpdatePricingAdmin {
        const DISCRIMINATOR: [u8; 8] = [[73,24,156,28,110,88,123,175]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateMakerRebatePercentage {
        pub native_maker_rebate_percentage: u64,
    }
    
    impl Discriminator for UpdateMakerRebatePercentage {
        const DISCRIMINATOR: [u8; 8] = [[180,236,253,19,231,231,220,65]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateTakeTriggerOrderFeePercentage {
        pub new_take_trigger_order_fee_percentage: u64,
    }
    
    impl Discriminator for UpdateTakeTriggerOrderFeePercentage {
        const DISCRIMINATOR: [u8; 8] = [[227,234,157,246,128,74,233,54]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaState {
        pub args: UpdateStateArgs,
    }
    
    impl Discriminator for UpdateZetaState {
        const DISCRIMINATOR: [u8; 8] = [[104,182,20,187,3,164,60,3]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateOracle {

    }
    
    impl Discriminator for UpdateOracle {
        const DISCRIMINATOR: [u8; 8] = [[112,41,209,18,248,226,252,188]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateOracleBackupFeed {

    }
    
    impl Discriminator for UpdateOracleBackupFeed {
        const DISCRIMINATOR: [u8; 8] = [[230,9,33,202,228,209,180,98]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePricingParameters {
        pub args: UpdatePricingParametersArgs,
    }
    
    impl Discriminator for UpdatePricingParameters {
        const DISCRIMINATOR: [u8; 8] = [[105,127,208,134,61,61,113,247]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateMarginParameters {
        pub args: UpdateMarginParametersArgs,
        pub asset: Asset,
    }
    
    impl Discriminator for UpdateMarginParameters {
        const DISCRIMINATOR: [u8; 8] = [[69,50,174,197,123,196,72,236]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaGroupMarginParameters {
        pub args: UpdateMarginParametersArgs,
    }
    
    impl Discriminator for UpdateZetaGroupMarginParameters {
        const DISCRIMINATOR: [u8; 8] = [[60,208,121,147,242,106,11,254]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpParameters {
        pub args: UpdatePerpParametersArgs,
        pub asset: Asset,
    }
    
    impl Discriminator for UpdatePerpParameters {
        const DISCRIMINATOR: [u8; 8] = [[90,135,219,42,164,134,97,174]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaGroupPerpParameters {
        pub args: UpdatePerpParametersArgs,
    }
    
    impl Discriminator for UpdateZetaGroupPerpParameters {
        const DISCRIMINATOR: [u8; 8] = [[72,152,140,158,195,93,247,31]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaGroupExpiryParameters {
        pub args: UpdateZetaGroupExpiryArgs,
    }
    
    impl Discriminator for UpdateZetaGroupExpiryParameters {
        const DISCRIMINATOR: [u8; 8] = [[17,69,121,104,225,206,140,215]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ToggleZetaGroupPerpsOnly {

    }
    
    impl Discriminator for ToggleZetaGroupPerpsOnly {
        const DISCRIMINATOR: [u8; 8] = [[170,115,77,11,161,157,247,169]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CleanZetaMarkets {

    }
    
    impl Discriminator for CleanZetaMarkets {
        const DISCRIMINATOR: [u8; 8] = [[122,127,49,89,68,228,85,157]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CleanZetaMarketHalted {
        pub asset: Asset,
    }
    
    impl Discriminator for CleanZetaMarketHalted {
        const DISCRIMINATOR: [u8; 8] = [[137,140,94,18,231,232,217,204]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct SettlePositionsHalted {
        pub asset: Asset,
    }
    
    impl Discriminator for SettlePositionsHalted {
        const DISCRIMINATOR: [u8; 8] = [[170,147,139,163,19,104,167,77]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarketStrikes {

    }
    
    impl Discriminator for InitializeMarketStrikes {
        const DISCRIMINATOR: [u8; 8] = [[189,46,255,33,126,133,43,171]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ExpireSeriesOverride {
        pub args: ExpireSeriesOverrideArgs,
    }
    
    impl Discriminator for ExpireSeriesOverride {
        const DISCRIMINATOR: [u8; 8] = [[104,22,34,123,86,224,130,70]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ExpireSeries {
        pub settlement_nonce: u8,
    }
    
    impl Discriminator for ExpireSeries {
        const DISCRIMINATOR: [u8; 8] = [[45,162,105,98,44,21,171,127]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaMarket {
        pub args: InitializeMarketArgs,
    }
    
    impl Discriminator for InitializeZetaMarket {
        const DISCRIMINATOR: [u8; 8] = [[116,239,226,149,46,163,221,3]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarketTifEpochCycle {
        pub epoch_length: u16,
    }
    
    impl Discriminator for InitializeMarketTifEpochCycle {
        const DISCRIMINATOR: [u8; 8] = [[199,143,173,147,202,204,64,204]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePricingV2 {
        pub asset: Asset,
    }
    
    impl Discriminator for UpdatePricingV2 {
        const DISCRIMINATOR: [u8; 8] = [[242,238,253,175,233,110,36,24]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePricingV3 {
        pub asset: Asset,
        pub price: u64,
        pub timestamp: u64,
    }
    
    impl Discriminator for UpdatePricingV3 {
        const DISCRIMINATOR: [u8; 8] = [[85,100,25,56,11,58,77,196]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ApplyPerpFunding {
        pub asset: Asset,
    }
    
    impl Discriminator for ApplyPerpFunding {
        const DISCRIMINATOR: [u8; 8] = [[23,82,225,222,219,122,230,251]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Deposit {
        pub amount: u64,
    }
    
    impl Discriminator for Deposit {
        const DISCRIMINATOR: [u8; 8] = [[242,35,198,137,82,225,242,182]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DepositV2 {
        pub amount: u64,
    }
    
    impl Discriminator for DepositV2 {
        const DISCRIMINATOR: [u8; 8] = [[189,215,40,239,176,56,3,69]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DepositPermissionless {
        pub amount: u64,
    }
    
    impl Discriminator for DepositPermissionless {
        const DISCRIMINATOR: [u8; 8] = [[235,247,9,248,204,52,9,50]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DepositInsuranceVault {
        pub amount: u64,
    }
    
    impl Discriminator for DepositInsuranceVault {
        const DISCRIMINATOR: [u8; 8] = [[47,53,25,47,109,122,22,22]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DepositInsuranceVaultV2 {
        pub amount: u64,
    }
    
    impl Discriminator for DepositInsuranceVaultV2 {
        const DISCRIMINATOR: [u8; 8] = [[175,99,163,101,112,206,31,17]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ChooseAirdropCommunity {
        pub community: u8,
    }
    
    impl Discriminator for ChooseAirdropCommunity {
        const DISCRIMINATOR: [u8; 8] = [[116,156,192,82,248,41,115,186]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Withdraw {
        pub amount: u64,
    }
    
    impl Discriminator for Withdraw {
        const DISCRIMINATOR: [u8; 8] = [[183,18,70,156,148,109,161,34]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct WithdrawV2 {
        pub amount: u64,
    }
    
    impl Discriminator for WithdrawV2 {
        const DISCRIMINATOR: [u8; 8] = [[227,234,204,176,201,98,87,69]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct WithdrawInsuranceVault {
        pub percentage_amount: u64,
    }
    
    impl Discriminator for WithdrawInsuranceVault {
        const DISCRIMINATOR: [u8; 8] = [[17,250,213,45,172,117,81,225]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct WithdrawInsuranceVaultV2 {
        pub percentage_amount: u64,
    }
    
    impl Discriminator for WithdrawInsuranceVaultV2 {
        const DISCRIMINATOR: [u8; 8] = [[153,147,202,154,136,112,37,231]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeOpenOrders {

    }
    
    impl Discriminator for InitializeOpenOrders {
        const DISCRIMINATOR: [u8; 8] = [[55,234,16,82,100,42,126,192]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeOpenOrdersV2 {

    }
    
    impl Discriminator for InitializeOpenOrdersV2 {
        const DISCRIMINATOR: [u8; 8] = [[110,93,101,187,58,250,156,7]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeOpenOrdersV3 {
        pub asset: Asset,
    }
    
    impl Discriminator for InitializeOpenOrdersV3 {
        const DISCRIMINATOR: [u8; 8] = [[44,87,3,195,52,192,180,46]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseOpenOrders {
        pub map_nonce: u8,
    }
    
    impl Discriminator for CloseOpenOrders {
        const DISCRIMINATOR: [u8; 8] = [[200,216,63,239,7,230,255,20]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseOpenOrdersV2 {
        pub map_nonce: u8,
    }
    
    impl Discriminator for CloseOpenOrdersV2 {
        const DISCRIMINATOR: [u8; 8] = [[230,9,144,21,73,145,156,101]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseOpenOrdersV3 {
        pub map_nonce: u8,
        pub asset: Asset,
    }
    
    impl Discriminator for CloseOpenOrdersV3 {
        const DISCRIMINATOR: [u8; 8] = [[141,13,107,77,193,255,16,233]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseOpenOrdersV4 {
        pub map_nonce: u8,
        pub asset: Asset,
    }
    
    impl Discriminator for CloseOpenOrdersV4 {
        const DISCRIMINATOR: [u8; 8] = [[131,117,17,123,177,105,107,188]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeWhitelistDepositAccount {
        pub nonce: u8,
    }
    
    impl Discriminator for InitializeWhitelistDepositAccount {
        const DISCRIMINATOR: [u8; 8] = [[61,231,115,219,81,243,158,138]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeWhitelistInsuranceAccount {
        pub nonce: u8,
    }
    
    impl Discriminator for InitializeWhitelistInsuranceAccount {
        const DISCRIMINATOR: [u8; 8] = [[43,46,240,155,80,4,86,102]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeWhitelistTradingFeesAccount {
        pub nonce: u8,
    }
    
    impl Discriminator for InitializeWhitelistTradingFeesAccount {
        const DISCRIMINATOR: [u8; 8] = [[198,129,216,185,247,29,105,190]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeInsuranceDepositAccount {
        pub nonce: u8,
    }
    
    impl Discriminator for InitializeInsuranceDepositAccount {
        const DISCRIMINATOR: [u8; 8] = [[85,163,114,121,139,167,41,37]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCombinedInsuranceVault {
        pub nonce: u8,
    }
    
    impl Discriminator for InitializeCombinedInsuranceVault {
        const DISCRIMINATOR: [u8; 8] = [[77,18,181,144,219,84,6,106]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCombinedVault {
        pub nonce: u8,
    }
    
    impl Discriminator for InitializeCombinedVault {
        const DISCRIMINATOR: [u8; 8] = [[59,99,105,17,73,119,229,252]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCombinedSocializedLossAccount {
        pub nonce: u8,
    }
    
    impl Discriminator for InitializeCombinedSocializedLossAccount {
        const DISCRIMINATOR: [u8; 8] = [[136,108,88,245,230,224,101,82]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceOrder {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub client_order_id: Option<u64>,
    }
    
    impl Discriminator for PlaceOrder {
        const DISCRIMINATOR: [u8; 8] = [[51,194,155,175,109,130,96,106]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceOrderV2 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
    }
    
    impl Discriminator for PlaceOrderV2 {
        const DISCRIMINATOR: [u8; 8] = [[32,43,50,46,206,151,233,186]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceOrderV3 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
    }
    
    impl Discriminator for PlaceOrderV3 {
        const DISCRIMINATOR: [u8; 8] = [[139,178,86,95,92,147,223,229]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlacePerpOrder {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
    }
    
    impl Discriminator for PlacePerpOrder {
        const DISCRIMINATOR: [u8; 8] = [[69,161,93,202,120,126,76,185]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlacePerpOrderV2 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
        pub tif_offset: Option<u16>,
    }
    
    impl Discriminator for PlacePerpOrderV2 {
        const DISCRIMINATOR: [u8; 8] = [[132,84,7,188,12,6,220,192]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceOrderV4 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
        pub tif_offset: Option<u16>,
    }
    
    impl Discriminator for PlaceOrderV4 {
        const DISCRIMINATOR: [u8; 8] = [[233,18,198,25,140,238,176,69]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlacePerpOrderV3 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
        pub tif_offset: Option<u16>,
        pub asset: Asset,
    }
    
    impl Discriminator for PlacePerpOrderV3 {
        const DISCRIMINATOR: [u8; 8] = [[52,208,31,213,51,190,221,85]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlacePerpOrderV4 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub reduce_only: bool,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
        pub tif_offset: Option<u16>,
        pub asset: Asset,
    }
    
    impl Discriminator for PlacePerpOrderV4 {
        const DISCRIMINATOR: [u8; 8] = [[254,229,1,43,140,159,246,129]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceMultiOrders {
        pub asset: Asset,
        pub bid_orders: Vec<OrderArgs>,
        pub ask_orders: Vec<OrderArgs>,
        pub order_type: OrderType,
    }
    
    impl Discriminator for PlaceMultiOrders {
        const DISCRIMINATOR: [u8; 8] = [[204,215,243,243,59,234,225,121]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceTriggerOrder {
        pub trigger_order_bit: u8,
        pub order_price: u64,
        pub trigger_price: Option<u64>,
        pub trigger_direction: Option<TriggerDirection>,
        pub trigger_ts: Option<u64>,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub reduce_only: bool,
        pub tag: Option<String>,
        pub asset: Asset,
    }
    
    impl Discriminator for PlaceTriggerOrder {
        const DISCRIMINATOR: [u8; 8] = [[32,156,50,188,232,159,112,236]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ExecuteTriggerOrderV2 {
        pub trigger_order_bit: u8,
    }
    
    impl Discriminator for ExecuteTriggerOrderV2 {
        const DISCRIMINATOR: [u8; 8] = [[163,243,175,234,135,157,148,216]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct TakeTriggerOrder {
        pub trigger_order_bit: u8,
    }
    
    impl Discriminator for TakeTriggerOrder {
        const DISCRIMINATOR: [u8; 8] = [[107,207,59,226,25,23,31,161]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ExecuteTriggerOrder {
        pub trigger_order_bit: u8,
    }
    
    impl Discriminator for ExecuteTriggerOrder {
        const DISCRIMINATOR: [u8; 8] = [[105,10,104,136,215,134,84,171]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelTriggerOrder {
        pub trigger_order_bit: u8,
        pub enforce_tpsl_conditions: bool,
    }
    
    impl Discriminator for ForceCancelTriggerOrder {
        const DISCRIMINATOR: [u8; 8] = [[120,236,216,28,192,79,255,188]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelTriggerOrderV2 {
        pub trigger_order_bit: u8,
    }
    
    impl Discriminator for CancelTriggerOrderV2 {
        const DISCRIMINATOR: [u8; 8] = [[26,109,79,244,126,182,86,53]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelTriggerOrder {
        pub trigger_order_bit: u8,
    }
    
    impl Discriminator for CancelTriggerOrder {
        const DISCRIMINATOR: [u8; 8] = [[144,84,67,39,27,25,202,141]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateMinLot {
        pub asset: Asset,
        pub min_lot_size: u32,
    }
    
    impl Discriminator for UpdateMinLot {
        const DISCRIMINATOR: [u8; 8] = [[6,136,5,12,229,146,102,89]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateTickSize {
        pub asset: Asset,
        pub tick_size: u32,
    }
    
    impl Discriminator for UpdateTickSize {
        const DISCRIMINATOR: [u8; 8] = [[222,122,1,221,123,116,143,110]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMinLotsAndTickSizes {

    }
    
    impl Discriminator for InitializeMinLotsAndTickSizes {
        const DISCRIMINATOR: [u8; 8] = [[68,25,51,43,126,171,80,87]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct EditTriggerOrder {
        pub order_price: u64,
        pub trigger_price: Option<u64>,
        pub trigger_direction: Option<TriggerDirection>,
        pub trigger_ts: Option<u64>,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub reduce_only: bool,
    }
    
    impl Discriminator for EditTriggerOrder {
        const DISCRIMINATOR: [u8; 8] = [[180,43,215,112,254,116,20,133]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct EditTriggerOrderV2 {
        pub order_price: u64,
        pub trigger_price: Option<u64>,
        pub trigger_direction: Option<TriggerDirection>,
        pub trigger_ts: Option<u64>,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub reduce_only: bool,
    }
    
    impl Discriminator for EditTriggerOrderV2 {
        const DISCRIMINATOR: [u8; 8] = [[168,165,107,3,97,12,212,191]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrder {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl Discriminator for CancelOrder {
        const DISCRIMINATOR: [u8; 8] = [[95,129,237,240,8,49,223,132]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrderNoError {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl Discriminator for CancelOrderNoError {
        const DISCRIMINATOR: [u8; 8] = [[95,97,215,204,111,51,204,184]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelAllMarketOrders {
        pub asset: Asset,
    }
    
    impl Discriminator for CancelAllMarketOrders {
        const DISCRIMINATOR: [u8; 8] = [[139,190,230,249,77,160,206,4]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrderHalted {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl Discriminator for CancelOrderHalted {
        const DISCRIMINATOR: [u8; 8] = [[0,192,233,2,252,251,130,169]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrderByClientOrderId {
        pub client_order_id: u64,
        pub asset: Asset,
    }
    
    impl Discriminator for CancelOrderByClientOrderId {
        const DISCRIMINATOR: [u8; 8] = [[115,178,201,8,175,183,123,119]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrderByClientOrderIdNoError {
        pub client_order_id: u64,
        pub asset: Asset,
    }
    
    impl Discriminator for CancelOrderByClientOrderIdNoError {
        const DISCRIMINATOR: [u8; 8] = [[53,77,167,157,175,131,144,171]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PruneExpiredTifOrders {

    }
    
    impl Discriminator for PruneExpiredTifOrders {
        const DISCRIMINATOR: [u8; 8] = [[24,227,226,212,93,26,242,230]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PruneExpiredTifOrdersV2 {
        pub limit: u16,
    }
    
    impl Discriminator for PruneExpiredTifOrdersV2 {
        const DISCRIMINATOR: [u8; 8] = [[42,235,144,91,67,127,89,77]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelOrderByOrderIdV2 {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl Discriminator for ForceCancelOrderByOrderIdV2 {
        const DISCRIMINATOR: [u8; 8] = [[95,114,98,46,56,255,216,131]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelOrderByOrderId {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl Discriminator for ForceCancelOrderByOrderId {
        const DISCRIMINATOR: [u8; 8] = [[182,235,48,179,248,133,210,240]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelOrdersV2 {
        pub asset: Asset,
    }
    
    impl Discriminator for ForceCancelOrdersV2 {
        const DISCRIMINATOR: [u8; 8] = [[35,210,136,152,28,38,213,77]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelOrders {
        pub asset: Asset,
    }
    
    impl Discriminator for ForceCancelOrders {
        const DISCRIMINATOR: [u8; 8] = [[64,181,196,63,222,72,64,232]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CrankEventQueue {
        pub asset: Asset,
    }
    
    impl Discriminator for CrankEventQueue {
        const DISCRIMINATOR: [u8; 8] = [[67,133,97,223,178,188,235,181]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CollectTreasuryFunds {
        pub amount: u64,
    }
    
    impl Discriminator for CollectTreasuryFunds {
        const DISCRIMINATOR: [u8; 8] = [[243,213,4,236,26,246,180,174]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct TreasuryMovement {
        pub treasury_movement_type: TreasuryMovementType,
        pub amount: u64,
    }
    
    impl Discriminator for TreasuryMovement {
        const DISCRIMINATOR: [u8; 8] = [[1,34,242,105,215,211,157,18]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct RebalanceInsuranceVault {

    }
    
    impl Discriminator for RebalanceInsuranceVault {
        const DISCRIMINATOR: [u8; 8] = [[11,196,66,235,59,237,223,111]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct LiquidateV2 {
        pub size: u64,
        pub asset: Asset,
    }
    
    impl Discriminator for LiquidateV2 {
        const DISCRIMINATOR: [u8; 8] = [[167,18,205,205,111,156,207,228]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Liquidate {
        pub size: u64,
    }
    
    impl Discriminator for Liquidate {
        const DISCRIMINATOR: [u8; 8] = [[223,179,226,125,48,46,39,74]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct BurnVaultTokens {

    }
    
    impl Discriminator for BurnVaultTokens {
        const DISCRIMINATOR: [u8; 8] = [[233,203,165,201,175,43,188,159]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct SettleDexFunds {

    }
    
    impl Discriminator for SettleDexFunds {
        const DISCRIMINATOR: [u8; 8] = [[165,103,142,38,211,166,14,226]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PositionMovement {
        pub movement_type: MovementType,
        pub movements: Vec<PositionMovementArg>,
    }
    
    impl Discriminator for PositionMovement {
        const DISCRIMINATOR: [u8; 8] = [[117,16,75,249,179,127,171,147]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct TransferExcessSpreadBalance {

    }
    
    impl Discriminator for TransferExcessSpreadBalance {
        const DISCRIMINATOR: [u8; 8] = [[172,184,12,10,52,105,64,213]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ToggleMarketMaker {
        pub is_market_maker: bool,
    }
    
    impl Discriminator for ToggleMarketMaker {
        const DISCRIMINATOR: [u8; 8] = [[203,247,84,159,104,253,148,80]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeReferrerAccounts {
        pub referrer_id: String,
    }
    
    impl Discriminator for InitializeReferrerAccounts {
        const DISCRIMINATOR: [u8; 8] = [[105,228,72,221,218,18,179,117]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseReferrerAccounts {

    }
    
    impl Discriminator for CloseReferrerAccounts {
        const DISCRIMINATOR: [u8; 8] = [[224,78,55,139,203,236,62,78]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct EditMaType {
        pub ma_type: MarginAccountType,
    }
    
    impl Discriminator for EditMaType {
        const DISCRIMINATOR: [u8; 8] = [[231,208,51,50,222,147,76,78]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct EditDelegatedPubkey {
        pub new_key: Pubkey,
    }
    
    impl Discriminator for EditDelegatedPubkey {
        const DISCRIMINATOR: [u8; 8] = [[137,245,71,89,46,249,22,53]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ResetNumFlexUnderlyings {

    }
    
    impl Discriminator for ResetNumFlexUnderlyings {
        const DISCRIMINATOR: [u8; 8] = [[48,19,254,209,200,211,49,61]];
        fn discriminator() -> [u8; 8] {
            Self::DISCRIMINATOR
        }
    }        
}
        
// Defined types
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProductGreeks {
    pub delta: u64,
    pub vega: AnchorDecimal,
    pub volatility: AnchorDecimal,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AnchorDecimal {
    pub flags: u32,
    pub hi: u32,
    pub lo: u32,
    pub mid: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HaltStateV2 {
    pub halted: bool,
    pub timestamp: u64,
    pub spot_price: u64,
    pub market_cleaned: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HaltState {
    pub halted: bool,
    pub spot_price: u64,
    pub timestamp: u64,
    pub mark_prices_set: [bool;2],
    pub mark_prices_set_padding: [bool;3],
    pub perp_mark_price_set: bool,
    pub market_nodes_cleaned: [bool;2],
    pub market_nodes_cleaned_padding: [bool;4],
    pub market_cleaned: [bool;46],
    pub market_cleaned_padding: [bool;91],
    pub perp_market_cleaned: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PricingParameters {
    pub option_trade_normalizer: AnchorDecimal,
    pub future_trade_normalizer: AnchorDecimal,
    pub max_volatility_retreat: AnchorDecimal,
    pub max_interest_retreat: AnchorDecimal,
    pub max_delta: u64,
    pub min_delta: u64,
    pub min_volatility: u64,
    pub max_volatility: u64,
    pub min_interest_rate: i64,
    pub max_interest_rate: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MarginParameters {
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PerpParameters {
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub impact_cash_delta: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ExpirySeries {
    pub active_ts: u64,
    pub expiry_ts: u64,
    pub dirty: bool,
    pub padding: [u8;15],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Strike {
    pub is_set: bool,
    pub value: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Product {
    pub market: Pubkey,
    pub strike: Strike,
    pub dirty: bool,
    pub kind: Kind,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Position {
    pub size: i64,
    pub cost_of_trades: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OrderState {
    pub closing_orders: u64,
    pub opening_orders: [u64;2],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProductLedger {
    pub position: Position,
    pub order_state: OrderState,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CrossMarginAccountInfo {
    pub initialized: bool,
    pub name: [u8;10],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OrderArgs {
    pub price: u64,
    pub size: u64,
    pub client_order_id: Option<u64>,
    pub tif_offset: Option<u16>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HaltStateArgs {
    pub asset: Asset,
    pub spot_price: u64,
    pub timestamp: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HaltArgs {
    pub spot_prices: [u64;15],
    pub timestamp: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateVolatilityArgs {
    pub expiry_index: u8,
    pub volatility: [u64;5],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateInterestRateArgs {
    pub expiry_index: u8,
    pub interest_rate: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ExpireSeriesOverrideArgs {
    pub settlement_nonce: u8,
    pub settlement_price: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeMarketArgs {
    pub asset: Asset,
    pub market_nonce: u8,
    pub base_mint_nonce: u8,
    pub quote_mint_nonce: u8,
    pub zeta_base_vault_nonce: u8,
    pub zeta_quote_vault_nonce: u8,
    pub dex_base_vault_nonce: u8,
    pub dex_quote_vault_nonce: u8,
    pub vault_signer_nonce: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeStateArgs {
    pub state_nonce: u8,
    pub serum_nonce: u8,
    pub mint_auth_nonce: u8,
    pub strike_initialization_threshold_seconds: u32,
    pub pricing_frequency_seconds: u32,
    pub liquidator_liquidation_percentage: u32,
    pub insurance_vault_liquidation_percentage: u32,
    pub native_deposit_limit: u64,
    pub expiration_threshold_seconds: u32,
    pub position_movement_fee_bps: u8,
    pub margin_concession_percentage: u8,
    pub max_perp_delta_age_seconds: u16,
    pub native_withdraw_limit: u64,
    pub withdraw_limit_epoch_seconds: u32,
    pub native_open_interest_limit: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeMarketNodeArgs {
    pub nonce: u8,
    pub index: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OverrideExpiryArgs {
    pub expiry_index: u8,
    pub active_ts: u64,
    pub expiry_ts: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateStateArgs {
    pub strike_initialization_threshold_seconds: u32,
    pub pricing_frequency_seconds: u32,
    pub liquidator_liquidation_percentage: u32,
    pub insurance_vault_liquidation_percentage: u32,
    pub native_deposit_limit: u64,
    pub expiration_threshold_seconds: u32,
    pub position_movement_fee_bps: u8,
    pub margin_concession_percentage: u8,
    pub max_perp_delta_age_seconds: u16,
    pub native_withdraw_limit: u64,
    pub withdraw_limit_epoch_seconds: u32,
    pub native_open_interest_limit: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdatePricingParametersArgs {
    pub option_trade_normalizer: u64,
    pub future_trade_normalizer: u64,
    pub max_volatility_retreat: u64,
    pub max_interest_retreat: u64,
    pub min_delta: u64,
    pub max_delta: u64,
    pub min_interest_rate: i64,
    pub max_interest_rate: i64,
    pub min_volatility: u64,
    pub max_volatility: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateMarginParametersArgs {
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdatePerpParametersArgs {
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub perp_impact_cash_delta: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeZetaGroupArgs {
    pub perps_only: bool,
    pub flex_underlying: bool,
    pub asset_override: Option<Asset>,
    pub zeta_group_nonce: u8,
    pub underlying_nonce: u8,
    pub greeks_nonce: u8,
    pub vault_nonce: u8,
    pub insurance_vault_nonce: u8,
    pub socialized_loss_account_nonce: u8,
    pub perp_sync_queue_nonce: u8,
    pub interest_rate: i64,
    pub volatility: [u64;5],
    pub option_trade_normalizer: u64,
    pub future_trade_normalizer: u64,
    pub max_volatility_retreat: u64,
    pub max_interest_retreat: u64,
    pub max_delta: u64,
    pub min_delta: u64,
    pub min_interest_rate: i64,
    pub max_interest_rate: i64,
    pub min_volatility: u64,
    pub max_volatility: u64,
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
    pub expiry_interval_seconds: u32,
    pub new_expiry_threshold_seconds: u32,
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub perp_impact_cash_delta: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateZetaGroupExpiryArgs {
    pub expiry_interval_seconds: u32,
    pub new_expiry_threshold_seconds: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateGreeksArgs {
    pub index: u8,
    pub theo: u64,
    pub delta: u32,
    pub gamma: u32,
    pub volatility: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PositionMovementArg {
    pub index: u8,
    pub size: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateZetaPricingPubkeysArgs {
    pub asset: Asset,
    pub oracle: Pubkey,
    pub oracle_backup_feed: Pubkey,
    pub market: Pubkey,
    pub perp_sync_queue: Pubkey,
    pub zeta_group_key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeZetaPricingArgs {
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub perp_impact_cash_delta: u64,
    pub margin_initial: u64,
    pub margin_maintenance: u64,
    pub pricing_nonce: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum ExpirySeriesStatus {
    Uninitialized,
    Initialized,
    Live,
    Expired,
    ExpiredDirty
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Kind {
    Uninitialized,
    Call,
    Put,
    Future,
    Perp
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum OrderType {
    Limit,
    PostOnly,
    FillOrKill,
    ImmediateOrCancel,
    PostOnlySlide,
    PostOnlyFront
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Side {
    Uninitialized,
    Bid,
    Ask
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum TriggerDirection {
    Uninitialized,
    LessThanOrEqual,
    GreaterThanOrEqual
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Asset {
    Sol,
    Btc,
    Eth,
    Apt,
    Arb,
    Bnb,
    Pyth,
    Tia,
    Jto,
    Onembonk,
    Sei,
    Jup,
    Dym,
    Strk,
    Wif,
    Undefined
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum MovementType {
    Undefined,
    Lock,
    Unlock
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum TreasuryMovementType {
    Undefined,
    ToTreasuryFromInsurance,
    ToInsuranceFromTreasury,
    ToTreasuryFromReferralsRewards,
    ToReferralsRewardsFromTreasury
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum OrderCompleteType {
    Cancel,
    Fill,
    Booted
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum MarginRequirement {
    Initial,
    Maintenance,
    MaintenanceIncludingOrders,
    MarketMakerConcession
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum MarginAccountType {
    Normal,
    MarketMaker,
    MarketMakerT1,
    MarketMakerT0,
    MarketMakerT2,
    MarketMakerT3,
    MarketMakerT4,
    MarketMakerT5,
    MarketMakerT6,
    MarketMakerT7,
    MarketMakerT8,
    MarketMakerT9,
    NormalT1,
    NormalT2,
    NormalT3,
    NormalT4,
    NormalT5,
    NormalT6,
    NormalT7,
    NormalT8,
    NormalT9
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum PlaceOrderType {
    PlaceOrder,
    PlacePerpOrder
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum ValidationType {
    Place,
    Cancel,
    OpenOrders,
    Liquidate
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum TraitType {
    MarginAccount,
    CrossMarginAccount
}