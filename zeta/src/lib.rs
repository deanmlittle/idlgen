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
pub struct InitializeMarketPda<'info> {
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
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeZetaSpecificMarketVaults<'info> {
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
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
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
    /// CHECK: Skip check
    pub serum_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

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
#[cfg(all(target_os = "solana", feature="cpi"))]
pub mod cpi {
    #![allow(unused)]
    use anchor_lang::Discriminator;
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

    pub fn initialize_market_pda<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeMarketPda<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeMarketPda { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeMarketPda::DISCRIMINATOR);
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

    pub fn initialize_zeta_specific_market_vaults<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeZetaSpecificMarketVaults<'info>>,
        asset: Asset
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeZetaSpecificMarketVaults { asset };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeZetaSpecificMarketVaults::DISCRIMINATOR);
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

// RPC
#[cfg(all(not(target_os = "solana"), feature="cpi"))]
pub mod rpc {
    #![allow(unused)]
    use anchor_lang::prelude::*;
    #[derive(AnchorSerialize)]
    pub struct InitializeZetaPricing {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeZetaPricing {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateZetaPricingPubkeys {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateZetaPricingPubkeys {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeZetaGroup {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub system_program: Pubkey,
        pub underlying_mint: Pubkey,
        pub zeta_program: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub zeta_group: Pubkey,
        pub greeks: Pubkey,
        pub perp_sync_queue: Pubkey,
        pub underlying: Pubkey,
        pub vault: Pubkey,
        pub insurance_vault: Pubkey,
        pub socialized_loss_account: Pubkey,
        pub token_program: Pubkey,
        pub usdc_mint: Pubkey,
        pub rent: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeZetaGroup {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.underlying_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.perp_sync_queue,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.underlying,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.usdc_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct OverrideExpiry {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub zeta_group: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for OverrideExpiry {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct MigrateToNewCrossMarginAccount {
        pub new_cross_margin_account: Pubkey,
        pub old_cross_margin_account: Pubkey,
        pub pricing: Pubkey,
        pub authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for MigrateToNewCrossMarginAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.new_cross_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.old_cross_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct MigrateToCrossMarginAccount {
        pub cross_margin_account: Pubkey,
        pub pricing: Pubkey,
        pub authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for MigrateToCrossMarginAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeCrossMarginAccountManager {
        pub cross_margin_account_manager: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub zeta_program: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeCrossMarginAccountManager {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account_manager,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_program,
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
    pub struct InitializeCrossMarginAccountManagerV2 {
        pub cross_margin_account_manager: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub zeta_program: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeCrossMarginAccountManagerV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account_manager,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_program,
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
    pub struct InitializeCrossMarginAccount {
        pub cross_margin_account: Pubkey,
        pub cross_margin_account_manager: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub zeta_program: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeCrossMarginAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account_manager,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_program,
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
    pub struct InitializeMarginAccount {
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub zeta_program: Pubkey,
        pub system_program: Pubkey,
        pub zeta_group: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeMarginAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeSpreadAccount {
        pub spread_account: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub zeta_program: Pubkey,
        pub system_program: Pubkey,
        pub zeta_group: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeSpreadAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.spread_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CloseCrossMarginAccountManager {
        pub cross_margin_account_manager: Pubkey,
        pub authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CloseCrossMarginAccountManager {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account_manager,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CloseCrossMarginAccount {
        pub cross_margin_account: Pubkey,
        pub cross_margin_account_manager: Pubkey,
        pub authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CloseCrossMarginAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account_manager,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CloseMarginAccount {
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub zeta_group: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CloseMarginAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CloseSpreadAccount {
        pub spread_account: Pubkey,
        pub authority: Pubkey,
        pub zeta_group: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CloseSpreadAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.spread_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeUnderlying {
        pub admin: Pubkey,
        pub zeta_program: Pubkey,
        pub state: Pubkey,
        pub system_program: Pubkey,
        pub underlying: Pubkey,
        pub underlying_mint: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeUnderlying {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.underlying,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.underlying_mint,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializePerpSyncQueue {
        pub admin: Pubkey,
        pub zeta_program: Pubkey,
        pub state: Pubkey,
        pub perp_sync_queue: Pubkey,
        pub pricing: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializePerpSyncQueue {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.perp_sync_queue,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
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
    pub struct InitializeMarketIndexes {
        pub state: Pubkey,
        pub market_indexes: Pubkey,
        pub admin: Pubkey,
        pub system_program: Pubkey,
        pub pricing: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeMarketIndexes {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_indexes,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeMarketNode {
        pub zeta_group: Pubkey,
        pub market_node: Pubkey,
        pub greeks: Pubkey,
        pub payer: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeMarketNode {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_node,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct Halt {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for Halt {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct Unhalt {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for Unhalt {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateHaltState {
        pub state: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateHaltState {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateVolatility {
        pub state: Pubkey,
        pub greeks: Pubkey,
        pub zeta_group: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateVolatility {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateInterestRate {
        pub state: Pubkey,
        pub greeks: Pubkey,
        pub zeta_group: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateInterestRate {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct AddPerpMarketIndex {
        pub market_indexes: Pubkey,
        pub pricing: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for AddPerpMarketIndex {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_indexes,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct AddMarketIndexes {
        pub market_indexes: Pubkey,
        pub zeta_group: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for AddMarketIndexes {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_indexes,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeZetaState {
        pub state: Pubkey,
        pub mint_authority: Pubkey,
        pub serum_authority: Pubkey,
        pub treasury_wallet: Pubkey,
        pub referrals_admin: Pubkey,
        pub referrals_rewards_wallet: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
        pub usdc_mint: Pubkey,
        pub admin: Pubkey,
        pub secondary_admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeZetaState {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.treasury_wallet,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.referrals_admin,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.referrals_rewards_wallet,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.usdc_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.secondary_admin,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeZetaTreasuryWallet {
        pub state: Pubkey,
        pub treasury_wallet: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
        pub usdc_mint: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeZetaTreasuryWallet {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.treasury_wallet,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.usdc_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeZetaReferralsRewardsWallet {
        pub state: Pubkey,
        pub referrals_rewards_wallet: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
        pub usdc_mint: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeZetaReferralsRewardsWallet {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.referrals_rewards_wallet,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.usdc_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateAdmin {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub new_admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateAdmin {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.new_admin,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateSecondaryAdmin {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub new_admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateSecondaryAdmin {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.new_admin,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateTriggerAdmin {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub new_admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateTriggerAdmin {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.new_admin,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateMaTypeAdmin {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub new_admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateMaTypeAdmin {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.new_admin,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateReferralsAdmin {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub new_admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateReferralsAdmin {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.new_admin,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdatePricingAdmin {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub new_admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdatePricingAdmin {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.new_admin,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateMakerRebatePercentage {
        pub state: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateMakerRebatePercentage {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateTakeTriggerOrderFeePercentage {
        pub state: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateTakeTriggerOrderFeePercentage {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateZetaState {
        pub state: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateZetaState {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateOracle {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub admin: Pubkey,
        pub oracle: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateOracle {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateOracleBackupFeed {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub admin: Pubkey,
        pub oracle: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateOracleBackupFeed {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdatePricingParameters {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdatePricingParameters {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateMarginParameters {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateMarginParameters {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateZetaGroupMarginParameters {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateZetaGroupMarginParameters {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdatePerpParameters {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdatePerpParameters {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateZetaGroupPerpParameters {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateZetaGroupPerpParameters {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateZetaGroupExpiryParameters {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateZetaGroupExpiryParameters {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ToggleZetaGroupPerpsOnly {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ToggleZetaGroupPerpsOnly {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CleanZetaMarkets {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CleanZetaMarkets {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CleanZetaMarketHalted {
        pub state: Pubkey,
        pub market: Pubkey,
        pub bids: Pubkey,
        pub asks: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CleanZetaMarketHalted {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.bids,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.asks,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct SettlePositionsHalted {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for SettlePositionsHalted {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeMarketStrikes {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeMarketStrikes {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ExpireSeriesOverride {}

    impl anchor_lang::ToAccountMetas for ExpireSeriesOverride {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
    
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ExpireSeries {}

    impl anchor_lang::ToAccountMetas for ExpireSeries {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
    
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeMarketPda {
        pub state: Pubkey,
        pub market_indexes: Pubkey,
        pub pricing: Pubkey,
        pub admin: Pubkey,
        pub market: Pubkey,
        pub system_program: Pubkey,
        pub rent: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeMarketPda {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_indexes,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeZetaSpecificMarketVaults {
        pub state: Pubkey,
        pub market_indexes: Pubkey,
        pub pricing: Pubkey,
        pub admin: Pubkey,
        pub market: Pubkey,
        pub base_mint: Pubkey,
        pub quote_mint: Pubkey,
        pub zeta_base_vault: Pubkey,
        pub zeta_quote_vault: Pubkey,
        pub serum_authority: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
        pub rent: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeZetaSpecificMarketVaults {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_indexes,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.base_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.quote_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_base_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_quote_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeZetaMarket {
        pub state: Pubkey,
        pub market_indexes: Pubkey,
        pub pricing: Pubkey,
        pub admin: Pubkey,
        pub market: Pubkey,
        pub request_queue: Pubkey,
        pub event_queue: Pubkey,
        pub bids: Pubkey,
        pub asks: Pubkey,
        pub base_mint: Pubkey,
        pub quote_mint: Pubkey,
        pub dex_base_vault: Pubkey,
        pub dex_quote_vault: Pubkey,
        pub vault_owner: Pubkey,
        pub mint_authority: Pubkey,
        pub serum_authority: Pubkey,
        pub dex_program: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
        pub rent: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeZetaMarket {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_indexes,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.request_queue,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.event_queue,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.bids,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.asks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.base_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.quote_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.dex_base_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.dex_quote_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.vault_owner,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeMarketTifEpochCycle {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub market: Pubkey,
        pub serum_authority: Pubkey,
        pub dex_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeMarketTifEpochCycle {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdatePricingV2 {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub perp_market: Pubkey,
        pub perp_bids: Pubkey,
        pub perp_asks: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdatePricingV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.perp_market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.perp_bids,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.perp_asks,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdatePricingV3 {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub oracle: Pubkey,
        pub perp_market: Pubkey,
        pub perp_bids: Pubkey,
        pub perp_asks: Pubkey,
        pub pricing_admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdatePricingV3 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.perp_market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.perp_bids,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.perp_asks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing_admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ApplyPerpFunding {
        pub state: Pubkey,
        pub pricing: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ApplyPerpFunding {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct Deposit {
        pub zeta_group: Pubkey,
        pub margin_account: Pubkey,
        pub vault: Pubkey,
        pub user_token_account: Pubkey,
        pub socialized_loss_account: Pubkey,
        pub authority: Pubkey,
        pub token_program: Pubkey,
        pub state: Pubkey,
        pub greeks: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for Deposit {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct DepositV2 {
        pub margin_account: Pubkey,
        pub vault: Pubkey,
        pub user_token_account: Pubkey,
        pub socialized_loss_account: Pubkey,
        pub authority: Pubkey,
        pub token_program: Pubkey,
        pub state: Pubkey,
        pub pricing: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for DepositV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct DepositPermissionless {
        pub cross_margin_account: Pubkey,
        pub vault: Pubkey,
        pub deposit_token_acc: Pubkey,
        pub socialized_loss_account: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub token_program: Pubkey,
        pub state: Pubkey,
        pub pricing: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for DepositPermissionless {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.deposit_token_acc,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct DepositInsuranceVault {
        pub state: Pubkey,
        pub insurance_vault: Pubkey,
        pub insurance_deposit_account: Pubkey,
        pub user_token_account: Pubkey,
        pub zeta_vault: Pubkey,
        pub socialized_loss_account: Pubkey,
        pub authority: Pubkey,
        pub token_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for DepositInsuranceVault {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_deposit_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct DepositInsuranceVaultV2 {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub insurance_vault: Pubkey,
        pub insurance_deposit_account: Pubkey,
        pub user_token_account: Pubkey,
        pub zeta_vault: Pubkey,
        pub socialized_loss_account: Pubkey,
        pub authority: Pubkey,
        pub token_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for DepositInsuranceVaultV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_deposit_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ChooseAirdropCommunity {
        pub cross_margin_account_manager: Pubkey,
        pub authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ChooseAirdropCommunity {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account_manager,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct Withdraw {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub vault: Pubkey,
        pub margin_account: Pubkey,
        pub user_token_account: Pubkey,
        pub token_program: Pubkey,
        pub authority: Pubkey,
        pub greeks: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub socialized_loss_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for Withdraw {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct WithdrawV2 {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub vault: Pubkey,
        pub margin_account: Pubkey,
        pub user_token_account: Pubkey,
        pub token_program: Pubkey,
        pub authority: Pubkey,
        pub socialized_loss_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for WithdrawV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct WithdrawInsuranceVault {
        pub state: Pubkey,
        pub insurance_vault: Pubkey,
        pub insurance_deposit_account: Pubkey,
        pub user_token_account: Pubkey,
        pub authority: Pubkey,
        pub token_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for WithdrawInsuranceVault {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_deposit_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct WithdrawInsuranceVaultV2 {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub insurance_vault: Pubkey,
        pub insurance_deposit_account: Pubkey,
        pub user_token_account: Pubkey,
        pub authority: Pubkey,
        pub token_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for WithdrawInsuranceVaultV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_deposit_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeOpenOrders {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub dex_program: Pubkey,
        pub system_program: Pubkey,
        pub open_orders: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub market: Pubkey,
        pub serum_authority: Pubkey,
        pub open_orders_map: Pubkey,
        pub rent: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeOpenOrders {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders_map,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeOpenOrdersV2 {
        pub state: Pubkey,
        pub dex_program: Pubkey,
        pub system_program: Pubkey,
        pub open_orders: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub market: Pubkey,
        pub serum_authority: Pubkey,
        pub open_orders_map: Pubkey,
        pub rent: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeOpenOrdersV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders_map,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeOpenOrdersV3 {
        pub state: Pubkey,
        pub dex_program: Pubkey,
        pub system_program: Pubkey,
        pub open_orders: Pubkey,
        pub cross_margin_account: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub market: Pubkey,
        pub serum_authority: Pubkey,
        pub open_orders_map: Pubkey,
        pub rent: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeOpenOrdersV3 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders_map,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CloseOpenOrders {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub dex_program: Pubkey,
        pub open_orders: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub market: Pubkey,
        pub serum_authority: Pubkey,
        pub open_orders_map: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CloseOpenOrders {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders_map,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CloseOpenOrdersV2 {
        pub state: Pubkey,
        pub dex_program: Pubkey,
        pub open_orders: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub market: Pubkey,
        pub serum_authority: Pubkey,
        pub open_orders_map: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CloseOpenOrdersV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders_map,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CloseOpenOrdersV3 {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub dex_program: Pubkey,
        pub open_orders: Pubkey,
        pub cross_margin_account: Pubkey,
        pub authority: Pubkey,
        pub market: Pubkey,
        pub serum_authority: Pubkey,
        pub open_orders_map: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CloseOpenOrdersV3 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders_map,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CloseOpenOrdersV4 {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub dex_program: Pubkey,
        pub open_orders: Pubkey,
        pub cross_margin_account: Pubkey,
        pub authority: Pubkey,
        pub market: Pubkey,
        pub serum_authority: Pubkey,
        pub open_orders_map: Pubkey,
        pub event_queue: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CloseOpenOrdersV4 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.cross_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders_map,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.event_queue,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeWhitelistDepositAccount {
        pub whitelist_deposit_account: Pubkey,
        pub admin: Pubkey,
        pub user: Pubkey,
        pub system_program: Pubkey,
        pub state: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeWhitelistDepositAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.whitelist_deposit_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.user,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeWhitelistInsuranceAccount {
        pub whitelist_insurance_account: Pubkey,
        pub admin: Pubkey,
        pub user: Pubkey,
        pub system_program: Pubkey,
        pub state: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeWhitelistInsuranceAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.whitelist_insurance_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.user,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeWhitelistTradingFeesAccount {
        pub whitelist_trading_fees_account: Pubkey,
        pub admin: Pubkey,
        pub user: Pubkey,
        pub system_program: Pubkey,
        pub state: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeWhitelistTradingFeesAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.whitelist_trading_fees_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.user,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeInsuranceDepositAccount {
        pub insurance_deposit_account: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub system_program: Pubkey,
        pub whitelist_insurance_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeInsuranceDepositAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_deposit_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.whitelist_insurance_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeCombinedInsuranceVault {
        pub state: Pubkey,
        pub insurance_vault: Pubkey,
        pub token_program: Pubkey,
        pub usdc_mint: Pubkey,
        pub admin: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeCombinedInsuranceVault {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.usdc_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeCombinedVault {
        pub state: Pubkey,
        pub vault: Pubkey,
        pub token_program: Pubkey,
        pub usdc_mint: Pubkey,
        pub admin: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeCombinedVault {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.usdc_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeCombinedSocializedLossAccount {
        pub state: Pubkey,
        pub socialized_loss_account: Pubkey,
        pub token_program: Pubkey,
        pub usdc_mint: Pubkey,
        pub admin: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeCombinedSocializedLossAccount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.usdc_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlaceOrder {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub dex_program: Pubkey,
        pub token_program: Pubkey,
        pub serum_authority: Pubkey,
        pub greeks: Pubkey,
        pub open_orders: Pubkey,
        pub rent: Pubkey,
        pub market_accounts: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market_node: Pubkey,
        pub market_mint: Pubkey,
        pub mint_authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlaceOrder {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_accounts,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_node,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlaceOrderV2 {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub dex_program: Pubkey,
        pub token_program: Pubkey,
        pub serum_authority: Pubkey,
        pub greeks: Pubkey,
        pub open_orders: Pubkey,
        pub rent: Pubkey,
        pub market_accounts: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market_node: Pubkey,
        pub market_mint: Pubkey,
        pub mint_authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlaceOrderV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_accounts,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_node,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlaceOrderV3 {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub dex_program: Pubkey,
        pub token_program: Pubkey,
        pub serum_authority: Pubkey,
        pub greeks: Pubkey,
        pub open_orders: Pubkey,
        pub rent: Pubkey,
        pub market_accounts: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market_node: Pubkey,
        pub market_mint: Pubkey,
        pub mint_authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlaceOrderV3 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_accounts,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_node,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlacePerpOrder {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub dex_program: Pubkey,
        pub token_program: Pubkey,
        pub serum_authority: Pubkey,
        pub greeks: Pubkey,
        pub open_orders: Pubkey,
        pub rent: Pubkey,
        pub market_accounts: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market_mint: Pubkey,
        pub mint_authority: Pubkey,
        pub perp_sync_queue: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlacePerpOrder {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_accounts,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.perp_sync_queue,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlacePerpOrderV2 {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub dex_program: Pubkey,
        pub token_program: Pubkey,
        pub serum_authority: Pubkey,
        pub greeks: Pubkey,
        pub open_orders: Pubkey,
        pub rent: Pubkey,
        pub market_accounts: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market_mint: Pubkey,
        pub mint_authority: Pubkey,
        pub perp_sync_queue: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlacePerpOrderV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_accounts,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.perp_sync_queue,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlaceOrderV4 {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub dex_program: Pubkey,
        pub token_program: Pubkey,
        pub serum_authority: Pubkey,
        pub greeks: Pubkey,
        pub open_orders: Pubkey,
        pub rent: Pubkey,
        pub market_accounts: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market_node: Pubkey,
        pub market_mint: Pubkey,
        pub mint_authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlaceOrderV4 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_accounts,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_node,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlacePerpOrderV3 {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub margin_account: Pubkey,
        pub authority: Pubkey,
        pub dex_program: Pubkey,
        pub token_program: Pubkey,
        pub serum_authority: Pubkey,
        pub open_orders: Pubkey,
        pub rent: Pubkey,
        pub market_accounts: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market_mint: Pubkey,
        pub mint_authority: Pubkey,
        pub perp_sync_queue: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlacePerpOrderV3 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market_accounts,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.perp_sync_queue,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlacePerpOrderV4 {
        pub authority: Pubkey,
        pub place_order_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlacePerpOrderV4 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.place_order_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlaceMultiOrders {
        pub authority: Pubkey,
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub margin_account: Pubkey,
        pub dex_program: Pubkey,
        pub token_program: Pubkey,
        pub serum_authority: Pubkey,
        pub open_orders: Pubkey,
        pub rent: Pubkey,
        pub market: Pubkey,
        pub request_queue: Pubkey,
        pub event_queue: Pubkey,
        pub bids: Pubkey,
        pub asks: Pubkey,
        pub market_base_vault: Pubkey,
        pub market_quote_vault: Pubkey,
        pub zeta_base_vault: Pubkey,
        pub zeta_quote_vault: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market_base_mint: Pubkey,
        pub market_quote_mint: Pubkey,
        pub mint_authority: Pubkey,
        pub perp_sync_queue: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlaceMultiOrders {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.request_queue,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.event_queue,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.bids,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.asks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_base_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_quote_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_base_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_quote_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_base_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market_quote_mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.perp_sync_queue,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PlaceTriggerOrder {
        pub state: Pubkey,
        pub open_orders: Pubkey,
        pub authority: Pubkey,
        pub margin_account: Pubkey,
        pub pricing: Pubkey,
        pub trigger_order: Pubkey,
        pub system_program: Pubkey,
        pub dex_program: Pubkey,
        pub market: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PlaceTriggerOrder {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.open_orders,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.trigger_order,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ExecuteTriggerOrderV2 {
        pub payer: Pubkey,
        pub trigger_order: Pubkey,
        pub place_order_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ExecuteTriggerOrderV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.trigger_order,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.place_order_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct TakeTriggerOrder {
        pub trigger_order: Pubkey,
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub bids: Pubkey,
        pub asks: Pubkey,
        pub taker: Pubkey,
        pub taker_margin_account: Pubkey,
        pub order_margin_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for TakeTriggerOrder {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.trigger_order,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.bids,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.asks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.taker,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.taker_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.order_margin_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ExecuteTriggerOrder {
        pub admin: Pubkey,
        pub trigger_order: Pubkey,
        pub place_order_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ExecuteTriggerOrder {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.trigger_order,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.place_order_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ForceCancelTriggerOrder {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub trigger_order: Pubkey,
        pub margin_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ForceCancelTriggerOrder {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.trigger_order,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CancelTriggerOrderV2 {
        pub authority: Pubkey,
        pub trigger_order: Pubkey,
        pub margin_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CancelTriggerOrderV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.trigger_order,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CancelTriggerOrder {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub payer: Pubkey,
        pub trigger_order: Pubkey,
        pub margin_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CancelTriggerOrder {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.admin,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.payer,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.trigger_order,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateMinLot {
        pub state: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateMinLot {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct UpdateTickSize {
        pub state: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for UpdateTickSize {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeMinLotsAndTickSizes {
        pub state: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeMinLotsAndTickSizes {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct EditTriggerOrder {
        pub owner: Pubkey,
        pub trigger_order: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for EditTriggerOrder {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.owner,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.trigger_order,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct EditTriggerOrderV2 {
        pub owner: Pubkey,
        pub trigger_order: Pubkey,
        pub state: Pubkey,
        pub margin_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for EditTriggerOrderV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.owner,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.trigger_order,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CancelOrder {
        pub authority: Pubkey,
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CancelOrder {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CancelOrderNoError {
        pub authority: Pubkey,
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CancelOrderNoError {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CancelAllMarketOrders {
        pub authority: Pubkey,
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CancelAllMarketOrders {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CancelOrderHalted {
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CancelOrderHalted {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CancelOrderByClientOrderId {
        pub authority: Pubkey,
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CancelOrderByClientOrderId {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CancelOrderByClientOrderIdNoError {
        pub authority: Pubkey,
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CancelOrderByClientOrderIdNoError {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PruneExpiredTifOrders {
        pub dex_program: Pubkey,
        pub state: Pubkey,
        pub serum_authority: Pubkey,
        pub market: Pubkey,
        pub bids: Pubkey,
        pub asks: Pubkey,
        pub event_queue: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PruneExpiredTifOrders {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.bids,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.asks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.event_queue,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PruneExpiredTifOrdersV2 {
        pub dex_program: Pubkey,
        pub state: Pubkey,
        pub serum_authority: Pubkey,
        pub market: Pubkey,
        pub bids: Pubkey,
        pub asks: Pubkey,
        pub event_queue: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PruneExpiredTifOrdersV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.bids,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.asks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.event_queue,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ForceCancelOrderByOrderIdV2 {
        pub pricing: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ForceCancelOrderByOrderIdV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ForceCancelOrderByOrderId {
        pub zeta_group: Pubkey,
        pub greeks: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ForceCancelOrderByOrderId {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ForceCancelOrdersV2 {
        pub pricing: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ForceCancelOrdersV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ForceCancelOrders {
        pub zeta_group: Pubkey,
        pub greeks: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub cancel_accounts: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ForceCancelOrders {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.cancel_accounts,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CrankEventQueue {
        pub state: Pubkey,
        pub pricing: Pubkey,
        pub market: Pubkey,
        pub event_queue: Pubkey,
        pub dex_program: Pubkey,
        pub serum_authority: Pubkey,
        pub perp_sync_queue: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CrankEventQueue {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.event_queue,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.perp_sync_queue,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct CollectTreasuryFunds {
        pub state: Pubkey,
        pub treasury_wallet: Pubkey,
        pub collection_token_account: Pubkey,
        pub token_program: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CollectTreasuryFunds {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.treasury_wallet,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.collection_token_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct TreasuryMovement {
        pub state: Pubkey,
        pub insurance_vault: Pubkey,
        pub treasury_wallet: Pubkey,
        pub referrals_rewards_wallet: Pubkey,
        pub token_program: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for TreasuryMovement {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.treasury_wallet,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.referrals_rewards_wallet,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct RebalanceInsuranceVault {
        pub state: Pubkey,
        pub zeta_vault: Pubkey,
        pub insurance_vault: Pubkey,
        pub treasury_wallet: Pubkey,
        pub socialized_loss_account: Pubkey,
        pub token_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for RebalanceInsuranceVault {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.treasury_wallet,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct LiquidateV2 {
        pub state: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_account: Pubkey,
        pub pricing: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market: Pubkey,
        pub liquidated_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for LiquidateV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.liquidator,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.liquidator_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.pricing,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.liquidated_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct Liquidate {
        pub state: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_margin_account: Pubkey,
        pub greeks: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
        pub market: Pubkey,
        pub zeta_group: Pubkey,
        pub liquidated_margin_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for Liquidate {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.liquidator,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.liquidator_margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.liquidated_margin_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct BurnVaultTokens {
        pub state: Pubkey,
        pub mint: Pubkey,
        pub vault: Pubkey,
        pub serum_authority: Pubkey,
        pub token_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for BurnVaultTokens {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.mint,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct SettleDexFunds {
        pub state: Pubkey,
        pub market: Pubkey,
        pub zeta_base_vault: Pubkey,
        pub zeta_quote_vault: Pubkey,
        pub dex_base_vault: Pubkey,
        pub dex_quote_vault: Pubkey,
        pub vault_owner: Pubkey,
        pub mint_authority: Pubkey,
        pub serum_authority: Pubkey,
        pub dex_program: Pubkey,
        pub token_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for SettleDexFunds {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_base_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.zeta_quote_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.dex_base_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.dex_quote_vault,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.vault_owner,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct PositionMovement {
        pub state: Pubkey,
        pub zeta_group: Pubkey,
        pub margin_account: Pubkey,
        pub spread_account: Pubkey,
        pub authority: Pubkey,
        pub greeks: Pubkey,
        pub oracle: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub oracle_backup_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for PositionMovement {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.spread_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct TransferExcessSpreadBalance {
        pub zeta_group: Pubkey,
        pub margin_account: Pubkey,
        pub spread_account: Pubkey,
        pub authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for TransferExcessSpreadBalance {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.spread_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ToggleMarketMaker {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub margin_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ToggleMarketMaker {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct InitializeReferrerAccounts {
        pub authority: Pubkey,
        pub referrer_id_account: Pubkey,
        pub referrer_pubkey_account: Pubkey,
        pub system_program: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for InitializeReferrerAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.referrer_id_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.referrer_pubkey_account,
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
    pub struct CloseReferrerAccounts {
        pub referrer_id_account: Pubkey,
        pub referrer_pubkey_account: Pubkey,
        pub authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for CloseReferrerAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.referrer_id_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.referrer_pubkey_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct EditMaType {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub margin_account: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for EditMaType {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct EditDelegatedPubkey {
        pub margin_account: Pubkey,
        pub authority: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for EditDelegatedPubkey {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
            account_metas
        }
    }

    #[derive(AnchorSerialize)]
    pub struct ResetNumFlexUnderlyings {
        pub state: Pubkey,
        pub admin: Pubkey,
    }

    impl anchor_lang::ToAccountMetas for ResetNumFlexUnderlyings {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
            self.state,
            false,
        ));
        account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
            self.admin,
            true,
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

    // InitializeZetaPricing
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaPricingI11n<'info> {
        pub accounts: InitializeZetaPricingAccountMetas<'info>,
        pub args: InitializeZetaPricing,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateZetaPricingPubkeys
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaPricingPubkeysI11n<'info> {
        pub accounts: UpdateZetaPricingPubkeysAccountMetas<'info>,
        pub args: UpdateZetaPricingPubkeys,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeZetaGroup
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaGroupI11n<'info> {
        pub accounts: InitializeZetaGroupAccountMetas<'info>,
        pub args: InitializeZetaGroup,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // OverrideExpiry
    #[derive(TryFromInstruction)]
    pub struct OverrideExpiryI11n<'info> {
        pub accounts: OverrideExpiryAccountMetas<'info>,
        pub args: OverrideExpiry,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // MigrateToNewCrossMarginAccount
    #[derive(TryFromInstruction)]
    pub struct MigrateToNewCrossMarginAccountI11n<'info> {
        pub accounts: MigrateToNewCrossMarginAccountAccountMetas<'info>,
        pub args: MigrateToNewCrossMarginAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // MigrateToCrossMarginAccount
    #[derive(TryFromInstruction)]
    pub struct MigrateToCrossMarginAccountI11n<'info> {
        pub accounts: MigrateToCrossMarginAccountAccountMetas<'info>,
        pub args: MigrateToCrossMarginAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeCrossMarginAccountManager
    #[derive(TryFromInstruction)]
    pub struct InitializeCrossMarginAccountManagerI11n<'info> {
        pub accounts: InitializeCrossMarginAccountManagerAccountMetas<'info>,
        pub args: InitializeCrossMarginAccountManager,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeCrossMarginAccountManagerV2
    #[derive(TryFromInstruction)]
    pub struct InitializeCrossMarginAccountManagerV2I11n<'info> {
        pub accounts: InitializeCrossMarginAccountManagerV2AccountMetas<'info>,
        pub args: InitializeCrossMarginAccountManagerV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeCrossMarginAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeCrossMarginAccountI11n<'info> {
        pub accounts: InitializeCrossMarginAccountAccountMetas<'info>,
        pub args: InitializeCrossMarginAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeMarginAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeMarginAccountI11n<'info> {
        pub accounts: InitializeMarginAccountAccountMetas<'info>,
        pub args: InitializeMarginAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeSpreadAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeSpreadAccountI11n<'info> {
        pub accounts: InitializeSpreadAccountAccountMetas<'info>,
        pub args: InitializeSpreadAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseCrossMarginAccountManager
    #[derive(TryFromInstruction)]
    pub struct CloseCrossMarginAccountManagerI11n<'info> {
        pub accounts: CloseCrossMarginAccountManagerAccountMetas<'info>,
        pub args: CloseCrossMarginAccountManager,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseCrossMarginAccount
    #[derive(TryFromInstruction)]
    pub struct CloseCrossMarginAccountI11n<'info> {
        pub accounts: CloseCrossMarginAccountAccountMetas<'info>,
        pub args: CloseCrossMarginAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseMarginAccount
    #[derive(TryFromInstruction)]
    pub struct CloseMarginAccountI11n<'info> {
        pub accounts: CloseMarginAccountAccountMetas<'info>,
        pub args: CloseMarginAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseSpreadAccount
    #[derive(TryFromInstruction)]
    pub struct CloseSpreadAccountI11n<'info> {
        pub accounts: CloseSpreadAccountAccountMetas<'info>,
        pub args: CloseSpreadAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeUnderlying
    #[derive(TryFromInstruction)]
    pub struct InitializeUnderlyingI11n<'info> {
        pub accounts: InitializeUnderlyingAccountMetas<'info>,
        pub args: InitializeUnderlying,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializePerpSyncQueue
    #[derive(TryFromInstruction)]
    pub struct InitializePerpSyncQueueI11n<'info> {
        pub accounts: InitializePerpSyncQueueAccountMetas<'info>,
        pub args: InitializePerpSyncQueue,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeMarketIndexes
    #[derive(TryFromInstruction)]
    pub struct InitializeMarketIndexesI11n<'info> {
        pub accounts: InitializeMarketIndexesAccountMetas<'info>,
        pub args: InitializeMarketIndexes,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeMarketNode
    #[derive(TryFromInstruction)]
    pub struct InitializeMarketNodeI11n<'info> {
        pub accounts: InitializeMarketNodeAccountMetas<'info>,
        pub args: InitializeMarketNode,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Halt
    #[derive(TryFromInstruction)]
    pub struct HaltI11n<'info> {
        pub accounts: HaltAccountMetas<'info>,
        pub args: Halt,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Unhalt
    #[derive(TryFromInstruction)]
    pub struct UnhaltI11n<'info> {
        pub accounts: UnhaltAccountMetas<'info>,
        pub args: Unhalt,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateHaltState
    #[derive(TryFromInstruction)]
    pub struct UpdateHaltStateI11n<'info> {
        pub accounts: UpdateHaltStateAccountMetas<'info>,
        pub args: UpdateHaltState,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateVolatility
    #[derive(TryFromInstruction)]
    pub struct UpdateVolatilityI11n<'info> {
        pub accounts: UpdateVolatilityAccountMetas<'info>,
        pub args: UpdateVolatility,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateInterestRate
    #[derive(TryFromInstruction)]
    pub struct UpdateInterestRateI11n<'info> {
        pub accounts: UpdateInterestRateAccountMetas<'info>,
        pub args: UpdateInterestRate,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // AddPerpMarketIndex
    #[derive(TryFromInstruction)]
    pub struct AddPerpMarketIndexI11n<'info> {
        pub accounts: AddPerpMarketIndexAccountMetas<'info>,
        pub args: AddPerpMarketIndex,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // AddMarketIndexes
    #[derive(TryFromInstruction)]
    pub struct AddMarketIndexesI11n<'info> {
        pub accounts: AddMarketIndexesAccountMetas<'info>,
        pub args: AddMarketIndexes,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeZetaState
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaStateI11n<'info> {
        pub accounts: InitializeZetaStateAccountMetas<'info>,
        pub args: InitializeZetaState,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeZetaTreasuryWallet
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaTreasuryWalletI11n<'info> {
        pub accounts: InitializeZetaTreasuryWalletAccountMetas<'info>,
        pub args: InitializeZetaTreasuryWallet,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeZetaReferralsRewardsWallet
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaReferralsRewardsWalletI11n<'info> {
        pub accounts: InitializeZetaReferralsRewardsWalletAccountMetas<'info>,
        pub args: InitializeZetaReferralsRewardsWallet,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateAdminI11n<'info> {
        pub accounts: UpdateAdminAccountMetas<'info>,
        pub args: UpdateAdmin,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateSecondaryAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateSecondaryAdminI11n<'info> {
        pub accounts: UpdateSecondaryAdminAccountMetas<'info>,
        pub args: UpdateSecondaryAdmin,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateTriggerAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateTriggerAdminI11n<'info> {
        pub accounts: UpdateTriggerAdminAccountMetas<'info>,
        pub args: UpdateTriggerAdmin,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateMaTypeAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateMaTypeAdminI11n<'info> {
        pub accounts: UpdateMaTypeAdminAccountMetas<'info>,
        pub args: UpdateMaTypeAdmin,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateReferralsAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdateReferralsAdminI11n<'info> {
        pub accounts: UpdateReferralsAdminAccountMetas<'info>,
        pub args: UpdateReferralsAdmin,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdatePricingAdmin
    #[derive(TryFromInstruction)]
    pub struct UpdatePricingAdminI11n<'info> {
        pub accounts: UpdatePricingAdminAccountMetas<'info>,
        pub args: UpdatePricingAdmin,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateMakerRebatePercentage
    #[derive(TryFromInstruction)]
    pub struct UpdateMakerRebatePercentageI11n<'info> {
        pub accounts: UpdateMakerRebatePercentageAccountMetas<'info>,
        pub args: UpdateMakerRebatePercentage,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateTakeTriggerOrderFeePercentage
    #[derive(TryFromInstruction)]
    pub struct UpdateTakeTriggerOrderFeePercentageI11n<'info> {
        pub accounts: UpdateTakeTriggerOrderFeePercentageAccountMetas<'info>,
        pub args: UpdateTakeTriggerOrderFeePercentage,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateZetaState
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaStateI11n<'info> {
        pub accounts: UpdateZetaStateAccountMetas<'info>,
        pub args: UpdateZetaState,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateOracle
    #[derive(TryFromInstruction)]
    pub struct UpdateOracleI11n<'info> {
        pub accounts: UpdateOracleAccountMetas<'info>,
        pub args: UpdateOracle,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateOracleBackupFeed
    #[derive(TryFromInstruction)]
    pub struct UpdateOracleBackupFeedI11n<'info> {
        pub accounts: UpdateOracleBackupFeedAccountMetas<'info>,
        pub args: UpdateOracleBackupFeed,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdatePricingParameters
    #[derive(TryFromInstruction)]
    pub struct UpdatePricingParametersI11n<'info> {
        pub accounts: UpdatePricingParametersAccountMetas<'info>,
        pub args: UpdatePricingParameters,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateMarginParameters
    #[derive(TryFromInstruction)]
    pub struct UpdateMarginParametersI11n<'info> {
        pub accounts: UpdateMarginParametersAccountMetas<'info>,
        pub args: UpdateMarginParameters,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateZetaGroupMarginParameters
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaGroupMarginParametersI11n<'info> {
        pub accounts: UpdateZetaGroupMarginParametersAccountMetas<'info>,
        pub args: UpdateZetaGroupMarginParameters,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdatePerpParameters
    #[derive(TryFromInstruction)]
    pub struct UpdatePerpParametersI11n<'info> {
        pub accounts: UpdatePerpParametersAccountMetas<'info>,
        pub args: UpdatePerpParameters,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateZetaGroupPerpParameters
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaGroupPerpParametersI11n<'info> {
        pub accounts: UpdateZetaGroupPerpParametersAccountMetas<'info>,
        pub args: UpdateZetaGroupPerpParameters,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateZetaGroupExpiryParameters
    #[derive(TryFromInstruction)]
    pub struct UpdateZetaGroupExpiryParametersI11n<'info> {
        pub accounts: UpdateZetaGroupExpiryParametersAccountMetas<'info>,
        pub args: UpdateZetaGroupExpiryParameters,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ToggleZetaGroupPerpsOnly
    #[derive(TryFromInstruction)]
    pub struct ToggleZetaGroupPerpsOnlyI11n<'info> {
        pub accounts: ToggleZetaGroupPerpsOnlyAccountMetas<'info>,
        pub args: ToggleZetaGroupPerpsOnly,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CleanZetaMarkets
    #[derive(TryFromInstruction)]
    pub struct CleanZetaMarketsI11n<'info> {
        pub accounts: CleanZetaMarketsAccountMetas<'info>,
        pub args: CleanZetaMarkets,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CleanZetaMarketHalted
    #[derive(TryFromInstruction)]
    pub struct CleanZetaMarketHaltedI11n<'info> {
        pub accounts: CleanZetaMarketHaltedAccountMetas<'info>,
        pub args: CleanZetaMarketHalted,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SettlePositionsHalted
    #[derive(TryFromInstruction)]
    pub struct SettlePositionsHaltedI11n<'info> {
        pub accounts: SettlePositionsHaltedAccountMetas<'info>,
        pub args: SettlePositionsHalted,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeMarketStrikes
    #[derive(TryFromInstruction)]
    pub struct InitializeMarketStrikesI11n<'info> {
        pub accounts: InitializeMarketStrikesAccountMetas<'info>,
        pub args: InitializeMarketStrikes,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ExpireSeriesOverride
    #[derive(TryFromInstruction)]
    pub struct ExpireSeriesOverrideI11n<'info> {
        pub accounts: ExpireSeriesOverrideAccountMetas,
        pub args: ExpireSeriesOverride,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ExpireSeries
    #[derive(TryFromInstruction)]
    pub struct ExpireSeriesI11n<'info> {
        pub accounts: ExpireSeriesAccountMetas,
        pub args: ExpireSeries,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeMarketPda
    #[derive(TryFromInstruction)]
    pub struct InitializeMarketPdaI11n<'info> {
        pub accounts: InitializeMarketPdaAccountMetas<'info>,
        pub args: InitializeMarketPda,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeZetaSpecificMarketVaults
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaSpecificMarketVaultsI11n<'info> {
        pub accounts: InitializeZetaSpecificMarketVaultsAccountMetas<'info>,
        pub args: InitializeZetaSpecificMarketVaults,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeZetaMarket
    #[derive(TryFromInstruction)]
    pub struct InitializeZetaMarketI11n<'info> {
        pub accounts: InitializeZetaMarketAccountMetas<'info>,
        pub args: InitializeZetaMarket,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeMarketTifEpochCycle
    #[derive(TryFromInstruction)]
    pub struct InitializeMarketTifEpochCycleI11n<'info> {
        pub accounts: InitializeMarketTifEpochCycleAccountMetas<'info>,
        pub args: InitializeMarketTifEpochCycle,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdatePricingV2
    #[derive(TryFromInstruction)]
    pub struct UpdatePricingV2I11n<'info> {
        pub accounts: UpdatePricingV2AccountMetas<'info>,
        pub args: UpdatePricingV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdatePricingV3
    #[derive(TryFromInstruction)]
    pub struct UpdatePricingV3I11n<'info> {
        pub accounts: UpdatePricingV3AccountMetas<'info>,
        pub args: UpdatePricingV3,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ApplyPerpFunding
    #[derive(TryFromInstruction)]
    pub struct ApplyPerpFundingI11n<'info> {
        pub accounts: ApplyPerpFundingAccountMetas<'info>,
        pub args: ApplyPerpFunding,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Deposit
    #[derive(TryFromInstruction)]
    pub struct DepositI11n<'info> {
        pub accounts: DepositAccountMetas<'info>,
        pub args: Deposit,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // DepositV2
    #[derive(TryFromInstruction)]
    pub struct DepositV2I11n<'info> {
        pub accounts: DepositV2AccountMetas<'info>,
        pub args: DepositV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // DepositPermissionless
    #[derive(TryFromInstruction)]
    pub struct DepositPermissionlessI11n<'info> {
        pub accounts: DepositPermissionlessAccountMetas<'info>,
        pub args: DepositPermissionless,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // DepositInsuranceVault
    #[derive(TryFromInstruction)]
    pub struct DepositInsuranceVaultI11n<'info> {
        pub accounts: DepositInsuranceVaultAccountMetas<'info>,
        pub args: DepositInsuranceVault,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // DepositInsuranceVaultV2
    #[derive(TryFromInstruction)]
    pub struct DepositInsuranceVaultV2I11n<'info> {
        pub accounts: DepositInsuranceVaultV2AccountMetas<'info>,
        pub args: DepositInsuranceVaultV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ChooseAirdropCommunity
    #[derive(TryFromInstruction)]
    pub struct ChooseAirdropCommunityI11n<'info> {
        pub accounts: ChooseAirdropCommunityAccountMetas<'info>,
        pub args: ChooseAirdropCommunity,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Withdraw
    #[derive(TryFromInstruction)]
    pub struct WithdrawI11n<'info> {
        pub accounts: WithdrawAccountMetas<'info>,
        pub args: Withdraw,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // WithdrawV2
    #[derive(TryFromInstruction)]
    pub struct WithdrawV2I11n<'info> {
        pub accounts: WithdrawV2AccountMetas<'info>,
        pub args: WithdrawV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // WithdrawInsuranceVault
    #[derive(TryFromInstruction)]
    pub struct WithdrawInsuranceVaultI11n<'info> {
        pub accounts: WithdrawInsuranceVaultAccountMetas<'info>,
        pub args: WithdrawInsuranceVault,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // WithdrawInsuranceVaultV2
    #[derive(TryFromInstruction)]
    pub struct WithdrawInsuranceVaultV2I11n<'info> {
        pub accounts: WithdrawInsuranceVaultV2AccountMetas<'info>,
        pub args: WithdrawInsuranceVaultV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeOpenOrders
    #[derive(TryFromInstruction)]
    pub struct InitializeOpenOrdersI11n<'info> {
        pub accounts: InitializeOpenOrdersAccountMetas<'info>,
        pub args: InitializeOpenOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeOpenOrdersV2
    #[derive(TryFromInstruction)]
    pub struct InitializeOpenOrdersV2I11n<'info> {
        pub accounts: InitializeOpenOrdersV2AccountMetas<'info>,
        pub args: InitializeOpenOrdersV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeOpenOrdersV3
    #[derive(TryFromInstruction)]
    pub struct InitializeOpenOrdersV3I11n<'info> {
        pub accounts: InitializeOpenOrdersV3AccountMetas<'info>,
        pub args: InitializeOpenOrdersV3,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseOpenOrders
    #[derive(TryFromInstruction)]
    pub struct CloseOpenOrdersI11n<'info> {
        pub accounts: CloseOpenOrdersAccountMetas<'info>,
        pub args: CloseOpenOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseOpenOrdersV2
    #[derive(TryFromInstruction)]
    pub struct CloseOpenOrdersV2I11n<'info> {
        pub accounts: CloseOpenOrdersV2AccountMetas<'info>,
        pub args: CloseOpenOrdersV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseOpenOrdersV3
    #[derive(TryFromInstruction)]
    pub struct CloseOpenOrdersV3I11n<'info> {
        pub accounts: CloseOpenOrdersV3AccountMetas<'info>,
        pub args: CloseOpenOrdersV3,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseOpenOrdersV4
    #[derive(TryFromInstruction)]
    pub struct CloseOpenOrdersV4I11n<'info> {
        pub accounts: CloseOpenOrdersV4AccountMetas<'info>,
        pub args: CloseOpenOrdersV4,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeWhitelistDepositAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeWhitelistDepositAccountI11n<'info> {
        pub accounts: InitializeWhitelistDepositAccountAccountMetas<'info>,
        pub args: InitializeWhitelistDepositAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeWhitelistInsuranceAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeWhitelistInsuranceAccountI11n<'info> {
        pub accounts: InitializeWhitelistInsuranceAccountAccountMetas<'info>,
        pub args: InitializeWhitelistInsuranceAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeWhitelistTradingFeesAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeWhitelistTradingFeesAccountI11n<'info> {
        pub accounts: InitializeWhitelistTradingFeesAccountAccountMetas<'info>,
        pub args: InitializeWhitelistTradingFeesAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeInsuranceDepositAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeInsuranceDepositAccountI11n<'info> {
        pub accounts: InitializeInsuranceDepositAccountAccountMetas<'info>,
        pub args: InitializeInsuranceDepositAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeCombinedInsuranceVault
    #[derive(TryFromInstruction)]
    pub struct InitializeCombinedInsuranceVaultI11n<'info> {
        pub accounts: InitializeCombinedInsuranceVaultAccountMetas<'info>,
        pub args: InitializeCombinedInsuranceVault,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeCombinedVault
    #[derive(TryFromInstruction)]
    pub struct InitializeCombinedVaultI11n<'info> {
        pub accounts: InitializeCombinedVaultAccountMetas<'info>,
        pub args: InitializeCombinedVault,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeCombinedSocializedLossAccount
    #[derive(TryFromInstruction)]
    pub struct InitializeCombinedSocializedLossAccountI11n<'info> {
        pub accounts: InitializeCombinedSocializedLossAccountAccountMetas<'info>,
        pub args: InitializeCombinedSocializedLossAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlaceOrder
    #[derive(TryFromInstruction)]
    pub struct PlaceOrderI11n<'info> {
        pub accounts: PlaceOrderAccountMetas<'info>,
        pub args: PlaceOrder,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlaceOrderV2
    #[derive(TryFromInstruction)]
    pub struct PlaceOrderV2I11n<'info> {
        pub accounts: PlaceOrderV2AccountMetas<'info>,
        pub args: PlaceOrderV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlaceOrderV3
    #[derive(TryFromInstruction)]
    pub struct PlaceOrderV3I11n<'info> {
        pub accounts: PlaceOrderV3AccountMetas<'info>,
        pub args: PlaceOrderV3,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlacePerpOrder
    #[derive(TryFromInstruction)]
    pub struct PlacePerpOrderI11n<'info> {
        pub accounts: PlacePerpOrderAccountMetas<'info>,
        pub args: PlacePerpOrder,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlacePerpOrderV2
    #[derive(TryFromInstruction)]
    pub struct PlacePerpOrderV2I11n<'info> {
        pub accounts: PlacePerpOrderV2AccountMetas<'info>,
        pub args: PlacePerpOrderV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlaceOrderV4
    #[derive(TryFromInstruction)]
    pub struct PlaceOrderV4I11n<'info> {
        pub accounts: PlaceOrderV4AccountMetas<'info>,
        pub args: PlaceOrderV4,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlacePerpOrderV3
    #[derive(TryFromInstruction)]
    pub struct PlacePerpOrderV3I11n<'info> {
        pub accounts: PlacePerpOrderV3AccountMetas<'info>,
        pub args: PlacePerpOrderV3,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlacePerpOrderV4
    #[derive(TryFromInstruction)]
    pub struct PlacePerpOrderV4I11n<'info> {
        pub accounts: PlacePerpOrderV4AccountMetas<'info>,
        pub args: PlacePerpOrderV4,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlaceMultiOrders
    #[derive(TryFromInstruction)]
    pub struct PlaceMultiOrdersI11n<'info> {
        pub accounts: PlaceMultiOrdersAccountMetas<'info>,
        pub args: PlaceMultiOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PlaceTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct PlaceTriggerOrderI11n<'info> {
        pub accounts: PlaceTriggerOrderAccountMetas<'info>,
        pub args: PlaceTriggerOrder,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ExecuteTriggerOrderV2
    #[derive(TryFromInstruction)]
    pub struct ExecuteTriggerOrderV2I11n<'info> {
        pub accounts: ExecuteTriggerOrderV2AccountMetas<'info>,
        pub args: ExecuteTriggerOrderV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // TakeTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct TakeTriggerOrderI11n<'info> {
        pub accounts: TakeTriggerOrderAccountMetas<'info>,
        pub args: TakeTriggerOrder,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ExecuteTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct ExecuteTriggerOrderI11n<'info> {
        pub accounts: ExecuteTriggerOrderAccountMetas<'info>,
        pub args: ExecuteTriggerOrder,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ForceCancelTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct ForceCancelTriggerOrderI11n<'info> {
        pub accounts: ForceCancelTriggerOrderAccountMetas<'info>,
        pub args: ForceCancelTriggerOrder,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CancelTriggerOrderV2
    #[derive(TryFromInstruction)]
    pub struct CancelTriggerOrderV2I11n<'info> {
        pub accounts: CancelTriggerOrderV2AccountMetas<'info>,
        pub args: CancelTriggerOrderV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CancelTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct CancelTriggerOrderI11n<'info> {
        pub accounts: CancelTriggerOrderAccountMetas<'info>,
        pub args: CancelTriggerOrder,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateMinLot
    #[derive(TryFromInstruction)]
    pub struct UpdateMinLotI11n<'info> {
        pub accounts: UpdateMinLotAccountMetas<'info>,
        pub args: UpdateMinLot,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateTickSize
    #[derive(TryFromInstruction)]
    pub struct UpdateTickSizeI11n<'info> {
        pub accounts: UpdateTickSizeAccountMetas<'info>,
        pub args: UpdateTickSize,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeMinLotsAndTickSizes
    #[derive(TryFromInstruction)]
    pub struct InitializeMinLotsAndTickSizesI11n<'info> {
        pub accounts: InitializeMinLotsAndTickSizesAccountMetas<'info>,
        pub args: InitializeMinLotsAndTickSizes,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // EditTriggerOrder
    #[derive(TryFromInstruction)]
    pub struct EditTriggerOrderI11n<'info> {
        pub accounts: EditTriggerOrderAccountMetas<'info>,
        pub args: EditTriggerOrder,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // EditTriggerOrderV2
    #[derive(TryFromInstruction)]
    pub struct EditTriggerOrderV2I11n<'info> {
        pub accounts: EditTriggerOrderV2AccountMetas<'info>,
        pub args: EditTriggerOrderV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CancelOrder
    #[derive(TryFromInstruction)]
    pub struct CancelOrderI11n<'info> {
        pub accounts: CancelOrderAccountMetas<'info>,
        pub args: CancelOrder,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CancelOrderNoError
    #[derive(TryFromInstruction)]
    pub struct CancelOrderNoErrorI11n<'info> {
        pub accounts: CancelOrderNoErrorAccountMetas<'info>,
        pub args: CancelOrderNoError,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CancelAllMarketOrders
    #[derive(TryFromInstruction)]
    pub struct CancelAllMarketOrdersI11n<'info> {
        pub accounts: CancelAllMarketOrdersAccountMetas<'info>,
        pub args: CancelAllMarketOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CancelOrderHalted
    #[derive(TryFromInstruction)]
    pub struct CancelOrderHaltedI11n<'info> {
        pub accounts: CancelOrderHaltedAccountMetas<'info>,
        pub args: CancelOrderHalted,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CancelOrderByClientOrderId
    #[derive(TryFromInstruction)]
    pub struct CancelOrderByClientOrderIdI11n<'info> {
        pub accounts: CancelOrderByClientOrderIdAccountMetas<'info>,
        pub args: CancelOrderByClientOrderId,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CancelOrderByClientOrderIdNoError
    #[derive(TryFromInstruction)]
    pub struct CancelOrderByClientOrderIdNoErrorI11n<'info> {
        pub accounts: CancelOrderByClientOrderIdNoErrorAccountMetas<'info>,
        pub args: CancelOrderByClientOrderIdNoError,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PruneExpiredTifOrders
    #[derive(TryFromInstruction)]
    pub struct PruneExpiredTifOrdersI11n<'info> {
        pub accounts: PruneExpiredTifOrdersAccountMetas<'info>,
        pub args: PruneExpiredTifOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PruneExpiredTifOrdersV2
    #[derive(TryFromInstruction)]
    pub struct PruneExpiredTifOrdersV2I11n<'info> {
        pub accounts: PruneExpiredTifOrdersV2AccountMetas<'info>,
        pub args: PruneExpiredTifOrdersV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ForceCancelOrderByOrderIdV2
    #[derive(TryFromInstruction)]
    pub struct ForceCancelOrderByOrderIdV2I11n<'info> {
        pub accounts: ForceCancelOrderByOrderIdV2AccountMetas<'info>,
        pub args: ForceCancelOrderByOrderIdV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ForceCancelOrderByOrderId
    #[derive(TryFromInstruction)]
    pub struct ForceCancelOrderByOrderIdI11n<'info> {
        pub accounts: ForceCancelOrderByOrderIdAccountMetas<'info>,
        pub args: ForceCancelOrderByOrderId,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ForceCancelOrdersV2
    #[derive(TryFromInstruction)]
    pub struct ForceCancelOrdersV2I11n<'info> {
        pub accounts: ForceCancelOrdersV2AccountMetas<'info>,
        pub args: ForceCancelOrdersV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ForceCancelOrders
    #[derive(TryFromInstruction)]
    pub struct ForceCancelOrdersI11n<'info> {
        pub accounts: ForceCancelOrdersAccountMetas<'info>,
        pub args: ForceCancelOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CrankEventQueue
    #[derive(TryFromInstruction)]
    pub struct CrankEventQueueI11n<'info> {
        pub accounts: CrankEventQueueAccountMetas<'info>,
        pub args: CrankEventQueue,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CollectTreasuryFunds
    #[derive(TryFromInstruction)]
    pub struct CollectTreasuryFundsI11n<'info> {
        pub accounts: CollectTreasuryFundsAccountMetas<'info>,
        pub args: CollectTreasuryFunds,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // TreasuryMovement
    #[derive(TryFromInstruction)]
    pub struct TreasuryMovementI11n<'info> {
        pub accounts: TreasuryMovementAccountMetas<'info>,
        pub args: TreasuryMovement,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // RebalanceInsuranceVault
    #[derive(TryFromInstruction)]
    pub struct RebalanceInsuranceVaultI11n<'info> {
        pub accounts: RebalanceInsuranceVaultAccountMetas<'info>,
        pub args: RebalanceInsuranceVault,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // LiquidateV2
    #[derive(TryFromInstruction)]
    pub struct LiquidateV2I11n<'info> {
        pub accounts: LiquidateV2AccountMetas<'info>,
        pub args: LiquidateV2,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Liquidate
    #[derive(TryFromInstruction)]
    pub struct LiquidateI11n<'info> {
        pub accounts: LiquidateAccountMetas<'info>,
        pub args: Liquidate,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // BurnVaultTokens
    #[derive(TryFromInstruction)]
    pub struct BurnVaultTokensI11n<'info> {
        pub accounts: BurnVaultTokensAccountMetas<'info>,
        pub args: BurnVaultTokens,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SettleDexFunds
    #[derive(TryFromInstruction)]
    pub struct SettleDexFundsI11n<'info> {
        pub accounts: SettleDexFundsAccountMetas<'info>,
        pub args: SettleDexFunds,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // PositionMovement
    #[derive(TryFromInstruction)]
    pub struct PositionMovementI11n<'info> {
        pub accounts: PositionMovementAccountMetas<'info>,
        pub args: PositionMovement,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // TransferExcessSpreadBalance
    #[derive(TryFromInstruction)]
    pub struct TransferExcessSpreadBalanceI11n<'info> {
        pub accounts: TransferExcessSpreadBalanceAccountMetas<'info>,
        pub args: TransferExcessSpreadBalance,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ToggleMarketMaker
    #[derive(TryFromInstruction)]
    pub struct ToggleMarketMakerI11n<'info> {
        pub accounts: ToggleMarketMakerAccountMetas<'info>,
        pub args: ToggleMarketMaker,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeReferrerAccounts
    #[derive(TryFromInstruction)]
    pub struct InitializeReferrerAccountsI11n<'info> {
        pub accounts: InitializeReferrerAccountsAccountMetas<'info>,
        pub args: InitializeReferrerAccounts,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseReferrerAccounts
    #[derive(TryFromInstruction)]
    pub struct CloseReferrerAccountsI11n<'info> {
        pub accounts: CloseReferrerAccountsAccountMetas<'info>,
        pub args: CloseReferrerAccounts,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // EditMaType
    #[derive(TryFromInstruction)]
    pub struct EditMaTypeI11n<'info> {
        pub accounts: EditMaTypeAccountMetas<'info>,
        pub args: EditMaType,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // EditDelegatedPubkey
    #[derive(TryFromInstruction)]
    pub struct EditDelegatedPubkeyI11n<'info> {
        pub accounts: EditDelegatedPubkeyAccountMetas<'info>,
        pub args: EditDelegatedPubkey,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ResetNumFlexUnderlyings
    #[derive(TryFromInstruction)]
    pub struct ResetNumFlexUnderlyingsI11n<'info> {
        pub accounts: ResetNumFlexUnderlyingsAccountMetas<'info>,
        pub args: ResetNumFlexUnderlyings,
        pub remaining_accounts: Vec<&'info AccountMeta>,
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
    pub struct InitializeMarketPdaAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub market_indexes: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeZetaSpecificMarketVaultsAccountMetas<'info> {
        pub state: &'info AccountMeta,
        pub market_indexes: &'info AccountMeta,
        pub pricing: &'info AccountMeta,
        pub admin: &'info AccountMeta,
        pub market: &'info AccountMeta,
        pub base_mint: &'info AccountMeta,
        pub quote_mint: &'info AccountMeta,
        pub zeta_base_vault: &'info AccountMeta,
        pub zeta_quote_vault: &'info AccountMeta,
        pub serum_authority: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
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
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use anchor_lang::Discriminator;
    use super::*;

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaPricing {
        pub args: InitializeZetaPricingArgs,
    }
    
    impl anchor_lang::InstructionData for InitializeZetaPricing {
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
    pub struct UpdateZetaPricingPubkeys {
        pub args: UpdateZetaPricingPubkeysArgs,
    }
    
    impl anchor_lang::InstructionData for UpdateZetaPricingPubkeys {
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
    pub struct InitializeZetaGroup {
        pub args: InitializeZetaGroupArgs,
    }
    
    impl anchor_lang::InstructionData for InitializeZetaGroup {
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
    pub struct OverrideExpiry {
        pub args: OverrideExpiryArgs,
    }
    
    impl anchor_lang::InstructionData for OverrideExpiry {
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
    pub struct MigrateToNewCrossMarginAccount {

    }
    
    impl anchor_lang::InstructionData for MigrateToNewCrossMarginAccount {
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
    pub struct MigrateToCrossMarginAccount {

    }
    
    impl anchor_lang::InstructionData for MigrateToCrossMarginAccount {
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
    pub struct InitializeCrossMarginAccountManager {

    }
    
    impl anchor_lang::InstructionData for InitializeCrossMarginAccountManager {
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
    pub struct InitializeCrossMarginAccountManagerV2 {
        pub referrer: Option<Pubkey>,
    }
    
    impl anchor_lang::InstructionData for InitializeCrossMarginAccountManagerV2 {
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
    pub struct InitializeCrossMarginAccount {
        pub subaccount_index: u8,
    }
    
    impl anchor_lang::InstructionData for InitializeCrossMarginAccount {
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
    pub struct InitializeMarginAccount {

    }
    
    impl anchor_lang::InstructionData for InitializeMarginAccount {
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
    pub struct InitializeSpreadAccount {

    }
    
    impl anchor_lang::InstructionData for InitializeSpreadAccount {
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
    pub struct CloseCrossMarginAccountManager {

    }
    
    impl anchor_lang::InstructionData for CloseCrossMarginAccountManager {
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
    pub struct CloseCrossMarginAccount {
        pub subaccount_index: u8,
    }
    
    impl anchor_lang::InstructionData for CloseCrossMarginAccount {
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
    pub struct CloseMarginAccount {

    }
    
    impl anchor_lang::InstructionData for CloseMarginAccount {
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
    pub struct CloseSpreadAccount {

    }
    
    impl anchor_lang::InstructionData for CloseSpreadAccount {
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
    pub struct InitializeUnderlying {
        pub flex_underlying: bool,
    }
    
    impl anchor_lang::InstructionData for InitializeUnderlying {
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
    pub struct InitializePerpSyncQueue {
        pub nonce: u8,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for InitializePerpSyncQueue {
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
    pub struct InitializeMarketIndexes {
        pub nonce: u8,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for InitializeMarketIndexes {
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
    pub struct InitializeMarketNode {
        pub args: InitializeMarketNodeArgs,
    }
    
    impl anchor_lang::InstructionData for InitializeMarketNode {
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
    pub struct Halt {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for Halt {
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
    pub struct Unhalt {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for Unhalt {
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
    pub struct UpdateHaltState {
        pub args: HaltStateArgs,
    }
    
    impl anchor_lang::InstructionData for UpdateHaltState {
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
    pub struct UpdateVolatility {
        pub args: UpdateVolatilityArgs,
    }
    
    impl anchor_lang::InstructionData for UpdateVolatility {
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
    pub struct UpdateInterestRate {
        pub args: UpdateInterestRateArgs,
    }
    
    impl anchor_lang::InstructionData for UpdateInterestRate {
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
    pub struct AddPerpMarketIndex {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for AddPerpMarketIndex {
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
    pub struct AddMarketIndexes {

    }
    
    impl anchor_lang::InstructionData for AddMarketIndexes {
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
    pub struct InitializeZetaState {
        pub args: InitializeStateArgs,
    }
    
    impl anchor_lang::InstructionData for InitializeZetaState {
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
    pub struct InitializeZetaTreasuryWallet {

    }
    
    impl anchor_lang::InstructionData for InitializeZetaTreasuryWallet {
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
    pub struct InitializeZetaReferralsRewardsWallet {

    }
    
    impl anchor_lang::InstructionData for InitializeZetaReferralsRewardsWallet {
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
    pub struct UpdateAdmin {

    }
    
    impl anchor_lang::InstructionData for UpdateAdmin {
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
    pub struct UpdateSecondaryAdmin {

    }
    
    impl anchor_lang::InstructionData for UpdateSecondaryAdmin {
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
    pub struct UpdateTriggerAdmin {

    }
    
    impl anchor_lang::InstructionData for UpdateTriggerAdmin {
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
    pub struct UpdateMaTypeAdmin {

    }
    
    impl anchor_lang::InstructionData for UpdateMaTypeAdmin {
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
    pub struct UpdateReferralsAdmin {

    }
    
    impl anchor_lang::InstructionData for UpdateReferralsAdmin {
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
    pub struct UpdatePricingAdmin {

    }
    
    impl anchor_lang::InstructionData for UpdatePricingAdmin {
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
    pub struct UpdateMakerRebatePercentage {
        pub native_maker_rebate_percentage: u64,
    }
    
    impl anchor_lang::InstructionData for UpdateMakerRebatePercentage {
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
    pub struct UpdateTakeTriggerOrderFeePercentage {
        pub new_take_trigger_order_fee_percentage: u64,
    }
    
    impl anchor_lang::InstructionData for UpdateTakeTriggerOrderFeePercentage {
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
    pub struct UpdateZetaState {
        pub args: UpdateStateArgs,
    }
    
    impl anchor_lang::InstructionData for UpdateZetaState {
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
    pub struct UpdateOracle {

    }
    
    impl anchor_lang::InstructionData for UpdateOracle {
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
    pub struct UpdateOracleBackupFeed {

    }
    
    impl anchor_lang::InstructionData for UpdateOracleBackupFeed {
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
    pub struct UpdatePricingParameters {
        pub args: UpdatePricingParametersArgs,
    }
    
    impl anchor_lang::InstructionData for UpdatePricingParameters {
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
    pub struct UpdateMarginParameters {
        pub args: UpdateMarginParametersArgs,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for UpdateMarginParameters {
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
    pub struct UpdateZetaGroupMarginParameters {
        pub args: UpdateMarginParametersArgs,
    }
    
    impl anchor_lang::InstructionData for UpdateZetaGroupMarginParameters {
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
    pub struct UpdatePerpParameters {
        pub args: UpdatePerpParametersArgs,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for UpdatePerpParameters {
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
    pub struct UpdateZetaGroupPerpParameters {
        pub args: UpdatePerpParametersArgs,
    }
    
    impl anchor_lang::InstructionData for UpdateZetaGroupPerpParameters {
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
    pub struct UpdateZetaGroupExpiryParameters {
        pub args: UpdateZetaGroupExpiryArgs,
    }
    
    impl anchor_lang::InstructionData for UpdateZetaGroupExpiryParameters {
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
    pub struct ToggleZetaGroupPerpsOnly {

    }
    
    impl anchor_lang::InstructionData for ToggleZetaGroupPerpsOnly {
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
    pub struct CleanZetaMarkets {

    }
    
    impl anchor_lang::InstructionData for CleanZetaMarkets {
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
    pub struct CleanZetaMarketHalted {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CleanZetaMarketHalted {
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
    pub struct SettlePositionsHalted {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for SettlePositionsHalted {
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
    pub struct InitializeMarketStrikes {

    }
    
    impl anchor_lang::InstructionData for InitializeMarketStrikes {
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
    pub struct ExpireSeriesOverride {
        pub args: ExpireSeriesOverrideArgs,
    }
    
    impl anchor_lang::InstructionData for ExpireSeriesOverride {
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
    pub struct ExpireSeries {
        pub settlement_nonce: u8,
    }
    
    impl anchor_lang::InstructionData for ExpireSeries {
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
    pub struct InitializeMarketPda {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for InitializeMarketPda {
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
    pub struct InitializeZetaSpecificMarketVaults {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for InitializeZetaSpecificMarketVaults {
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
    pub struct InitializeZetaMarket {
        pub args: InitializeMarketArgs,
    }
    
    impl anchor_lang::InstructionData for InitializeZetaMarket {
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
    pub struct InitializeMarketTifEpochCycle {
        pub epoch_length: u16,
    }
    
    impl anchor_lang::InstructionData for InitializeMarketTifEpochCycle {
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
    pub struct UpdatePricingV2 {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for UpdatePricingV2 {
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
    pub struct UpdatePricingV3 {
        pub asset: Asset,
        pub price: u64,
        pub timestamp: u64,
    }
    
    impl anchor_lang::InstructionData for UpdatePricingV3 {
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
    pub struct ApplyPerpFunding {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for ApplyPerpFunding {
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
    pub struct Deposit {
        pub amount: u64,
    }
    
    impl anchor_lang::InstructionData for Deposit {
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
    pub struct DepositV2 {
        pub amount: u64,
    }
    
    impl anchor_lang::InstructionData for DepositV2 {
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
    pub struct DepositPermissionless {
        pub amount: u64,
    }
    
    impl anchor_lang::InstructionData for DepositPermissionless {
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
    pub struct DepositInsuranceVault {
        pub amount: u64,
    }
    
    impl anchor_lang::InstructionData for DepositInsuranceVault {
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
    pub struct DepositInsuranceVaultV2 {
        pub amount: u64,
    }
    
    impl anchor_lang::InstructionData for DepositInsuranceVaultV2 {
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
    pub struct ChooseAirdropCommunity {
        pub community: u8,
    }
    
    impl anchor_lang::InstructionData for ChooseAirdropCommunity {
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
    pub struct Withdraw {
        pub amount: u64,
    }
    
    impl anchor_lang::InstructionData for Withdraw {
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
    pub struct WithdrawV2 {
        pub amount: u64,
    }
    
    impl anchor_lang::InstructionData for WithdrawV2 {
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
    pub struct WithdrawInsuranceVault {
        pub percentage_amount: u64,
    }
    
    impl anchor_lang::InstructionData for WithdrawInsuranceVault {
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
    pub struct WithdrawInsuranceVaultV2 {
        pub percentage_amount: u64,
    }
    
    impl anchor_lang::InstructionData for WithdrawInsuranceVaultV2 {
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
    pub struct InitializeOpenOrders {

    }
    
    impl anchor_lang::InstructionData for InitializeOpenOrders {
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
    pub struct InitializeOpenOrdersV2 {

    }
    
    impl anchor_lang::InstructionData for InitializeOpenOrdersV2 {
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
    pub struct InitializeOpenOrdersV3 {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for InitializeOpenOrdersV3 {
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
    pub struct CloseOpenOrders {
        pub map_nonce: u8,
    }
    
    impl anchor_lang::InstructionData for CloseOpenOrders {
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
    pub struct CloseOpenOrdersV2 {
        pub map_nonce: u8,
    }
    
    impl anchor_lang::InstructionData for CloseOpenOrdersV2 {
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
    pub struct CloseOpenOrdersV3 {
        pub map_nonce: u8,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CloseOpenOrdersV3 {
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
    pub struct CloseOpenOrdersV4 {
        pub map_nonce: u8,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CloseOpenOrdersV4 {
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
    pub struct InitializeWhitelistDepositAccount {
        pub nonce: u8,
    }
    
    impl anchor_lang::InstructionData for InitializeWhitelistDepositAccount {
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
    pub struct InitializeWhitelistInsuranceAccount {
        pub nonce: u8,
    }
    
    impl anchor_lang::InstructionData for InitializeWhitelistInsuranceAccount {
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
    pub struct InitializeWhitelistTradingFeesAccount {
        pub nonce: u8,
    }
    
    impl anchor_lang::InstructionData for InitializeWhitelistTradingFeesAccount {
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
    pub struct InitializeInsuranceDepositAccount {
        pub nonce: u8,
    }
    
    impl anchor_lang::InstructionData for InitializeInsuranceDepositAccount {
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
    pub struct InitializeCombinedInsuranceVault {
        pub nonce: u8,
    }
    
    impl anchor_lang::InstructionData for InitializeCombinedInsuranceVault {
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
    pub struct InitializeCombinedVault {
        pub nonce: u8,
    }
    
    impl anchor_lang::InstructionData for InitializeCombinedVault {
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
    pub struct InitializeCombinedSocializedLossAccount {
        pub nonce: u8,
    }
    
    impl anchor_lang::InstructionData for InitializeCombinedSocializedLossAccount {
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
    pub struct PlaceOrder {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub client_order_id: Option<u64>,
    }
    
    impl anchor_lang::InstructionData for PlaceOrder {
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
    pub struct PlaceOrderV2 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
    }
    
    impl anchor_lang::InstructionData for PlaceOrderV2 {
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
    pub struct PlaceOrderV3 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
    }
    
    impl anchor_lang::InstructionData for PlaceOrderV3 {
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
    pub struct PlacePerpOrder {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
    }
    
    impl anchor_lang::InstructionData for PlacePerpOrder {
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
    pub struct PlacePerpOrderV2 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
        pub tif_offset: Option<u16>,
    }
    
    impl anchor_lang::InstructionData for PlacePerpOrderV2 {
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
    pub struct PlaceOrderV4 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
        pub tif_offset: Option<u16>,
    }
    
    impl anchor_lang::InstructionData for PlaceOrderV4 {
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
    
    impl anchor_lang::InstructionData for PlacePerpOrderV3 {
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
    
    impl anchor_lang::InstructionData for PlacePerpOrderV4 {
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
    pub struct PlaceMultiOrders {
        pub asset: Asset,
        pub bid_orders: Vec<OrderArgs>,
        pub ask_orders: Vec<OrderArgs>,
        pub order_type: OrderType,
    }
    
    impl anchor_lang::InstructionData for PlaceMultiOrders {
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
    
    impl anchor_lang::InstructionData for PlaceTriggerOrder {
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
    pub struct ExecuteTriggerOrderV2 {
        pub trigger_order_bit: u8,
    }
    
    impl anchor_lang::InstructionData for ExecuteTriggerOrderV2 {
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
    pub struct TakeTriggerOrder {
        pub trigger_order_bit: u8,
    }
    
    impl anchor_lang::InstructionData for TakeTriggerOrder {
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
    pub struct ExecuteTriggerOrder {
        pub trigger_order_bit: u8,
    }
    
    impl anchor_lang::InstructionData for ExecuteTriggerOrder {
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
    pub struct ForceCancelTriggerOrder {
        pub trigger_order_bit: u8,
        pub enforce_tpsl_conditions: bool,
    }
    
    impl anchor_lang::InstructionData for ForceCancelTriggerOrder {
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
    pub struct CancelTriggerOrderV2 {
        pub trigger_order_bit: u8,
    }
    
    impl anchor_lang::InstructionData for CancelTriggerOrderV2 {
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
    pub struct CancelTriggerOrder {
        pub trigger_order_bit: u8,
    }
    
    impl anchor_lang::InstructionData for CancelTriggerOrder {
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
    pub struct UpdateMinLot {
        pub asset: Asset,
        pub min_lot_size: u32,
    }
    
    impl anchor_lang::InstructionData for UpdateMinLot {
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
    pub struct UpdateTickSize {
        pub asset: Asset,
        pub tick_size: u32,
    }
    
    impl anchor_lang::InstructionData for UpdateTickSize {
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
    pub struct InitializeMinLotsAndTickSizes {

    }
    
    impl anchor_lang::InstructionData for InitializeMinLotsAndTickSizes {
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
    
    impl anchor_lang::InstructionData for EditTriggerOrder {
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
    
    impl anchor_lang::InstructionData for EditTriggerOrderV2 {
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
    pub struct CancelOrder {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CancelOrder {
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
    pub struct CancelOrderNoError {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CancelOrderNoError {
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
    pub struct CancelAllMarketOrders {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CancelAllMarketOrders {
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
    pub struct CancelOrderHalted {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CancelOrderHalted {
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
    pub struct CancelOrderByClientOrderId {
        pub client_order_id: u64,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CancelOrderByClientOrderId {
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
    pub struct CancelOrderByClientOrderIdNoError {
        pub client_order_id: u64,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CancelOrderByClientOrderIdNoError {
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
    pub struct PruneExpiredTifOrders {

    }
    
    impl anchor_lang::InstructionData for PruneExpiredTifOrders {
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
    pub struct PruneExpiredTifOrdersV2 {
        pub limit: u16,
    }
    
    impl anchor_lang::InstructionData for PruneExpiredTifOrdersV2 {
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
    pub struct ForceCancelOrderByOrderIdV2 {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for ForceCancelOrderByOrderIdV2 {
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
    pub struct ForceCancelOrderByOrderId {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for ForceCancelOrderByOrderId {
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
    pub struct ForceCancelOrdersV2 {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for ForceCancelOrdersV2 {
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
    pub struct ForceCancelOrders {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for ForceCancelOrders {
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
    pub struct CrankEventQueue {
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for CrankEventQueue {
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
    pub struct CollectTreasuryFunds {
        pub amount: u64,
    }
    
    impl anchor_lang::InstructionData for CollectTreasuryFunds {
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
    pub struct TreasuryMovement {
        pub treasury_movement_type: TreasuryMovementType,
        pub amount: u64,
    }
    
    impl anchor_lang::InstructionData for TreasuryMovement {
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
    pub struct RebalanceInsuranceVault {

    }
    
    impl anchor_lang::InstructionData for RebalanceInsuranceVault {
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
    pub struct LiquidateV2 {
        pub size: u64,
        pub asset: Asset,
    }
    
    impl anchor_lang::InstructionData for LiquidateV2 {
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
    pub struct Liquidate {
        pub size: u64,
    }
    
    impl anchor_lang::InstructionData for Liquidate {
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
    pub struct BurnVaultTokens {

    }
    
    impl anchor_lang::InstructionData for BurnVaultTokens {
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
    pub struct SettleDexFunds {

    }
    
    impl anchor_lang::InstructionData for SettleDexFunds {
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
    pub struct PositionMovement {
        pub movement_type: MovementType,
        pub movements: Vec<PositionMovementArg>,
    }
    
    impl anchor_lang::InstructionData for PositionMovement {
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
    pub struct TransferExcessSpreadBalance {

    }
    
    impl anchor_lang::InstructionData for TransferExcessSpreadBalance {
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
    pub struct ToggleMarketMaker {
        pub is_market_maker: bool,
    }
    
    impl anchor_lang::InstructionData for ToggleMarketMaker {
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
    pub struct InitializeReferrerAccounts {
        pub referrer_id: String,
    }
    
    impl anchor_lang::InstructionData for InitializeReferrerAccounts {
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
    pub struct CloseReferrerAccounts {

    }
    
    impl anchor_lang::InstructionData for CloseReferrerAccounts {
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
    pub struct EditMaType {
        pub ma_type: MarginAccountType,
    }
    
    impl anchor_lang::InstructionData for EditMaType {
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
    pub struct EditDelegatedPubkey {
        pub new_key: Pubkey,
    }
    
    impl anchor_lang::InstructionData for EditDelegatedPubkey {
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
    pub struct ResetNumFlexUnderlyings {

    }
    
    impl anchor_lang::InstructionData for ResetNumFlexUnderlyings {
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
    pub struct TradeEvent {
        pub margin_account: Pubkey,
        pub index: u8,
        pub size: u64,
        pub cost_of_trades: u64,
        pub is_bid: bool,
        pub client_order_id: u64,
        pub order_id: u128
    }
    
    impl anchor_lang::Event for TradeEvent {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct TradeEventV2 {
        pub margin_account: Pubkey,
        pub index: u8,
        pub size: u64,
        pub cost_of_trades: u64,
        pub is_bid: bool,
        pub client_order_id: u64,
        pub order_id: u128,
        pub asset: u8,
        pub user: Pubkey,
        pub is_taker: bool,
        pub sequence_number: u64
    }
    
    impl anchor_lang::Event for TradeEventV2 {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct TradeEventV3 {
        pub margin_account: Pubkey,
        pub index: u8,
        pub size: u64,
        pub cost_of_trades: u64,
        pub is_bid: bool,
        pub client_order_id: u64,
        pub order_id: u128,
        pub asset: Asset,
        pub user: Pubkey,
        pub is_taker: bool,
        pub sequence_number: u64,
        pub fee: u64,
        pub price: u64,
        pub pnl: i64,
        pub rebate: u64
    }
    
    impl anchor_lang::Event for TradeEventV3 {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct PositionMovementEvent {
        pub net_balance_transfer: i64,
        pub margin_account_balance: u64,
        pub spread_account_balance: u64,
        pub movement_fees: u64
    }
    
    impl anchor_lang::Event for PositionMovementEvent {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceOrderEvent {
        pub fee: u64,
        pub oracle_price: u64,
        pub order_id: u128,
        pub expiry_ts: u64,
        pub asset: Asset,
        pub margin_account: Pubkey,
        pub client_order_id: u64,
        pub user: Pubkey
    }
    
    impl anchor_lang::Event for PlaceOrderEvent {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct LiquidationEvent {
        pub liquidator_reward: u64,
        pub insurance_reward: u64,
        pub cost_of_trades: u64,
        pub size: i64,
        pub remaining_liquidatee_balance: u64,
        pub remaining_liquidator_balance: u64,
        pub mark_price: u64,
        pub underlying_price: u64,
        pub liquidatee: Pubkey,
        pub liquidator: Pubkey,
        pub asset: Asset,
        pub liquidatee_margin_account: Pubkey,
        pub liquidator_margin_account: Pubkey
    }
    
    impl anchor_lang::Event for LiquidationEvent {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct OrderCompleteEvent {
        pub margin_account: Pubkey,
        pub user: Pubkey,
        pub asset: Asset,
        pub market_index: u8,
        pub side: Side,
        pub unfilled_size: u64,
        pub order_id: u128,
        pub client_order_id: u64,
        pub order_complete_type: OrderCompleteType
    }
    
    impl anchor_lang::Event for OrderCompleteEvent {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct ApplyFundingEvent {
        pub margin_account: Pubkey,
        pub user: Pubkey,
        pub asset: Asset,
        pub balance_change: i64,
        pub remaining_balance: u64,
        pub funding_rate: i64,
        pub oracle_price: u64,
        pub position_size: i64
    }
    
    impl anchor_lang::Event for ApplyFundingEvent {
        fn data(&self) -> Vec<u8> {
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceMultiOrdersEvent {
        pub oracle_price: u64,
        pub order_ids: Vec<u128>,
        pub expiry_tss: Vec<u64>,
        pub asset: Asset,
        pub margin_account: Pubkey,
        pub client_order_ids: Vec<u64>,
        pub user: Pubkey
    }
    
    impl anchor_lang::Event for PlaceMultiOrdersEvent {
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
    pub struct Pricing {
        pub nonce: u8,
        pub mark_prices: [u64;16],
        pub mark_prices_padding: [u64;9],
        pub update_timestamps: [u64;16],
        pub update_timestamps_padding: [u64;9],
        pub funding_deltas: [AnchorDecimal;16],
        pub funding_deltas_padding: [AnchorDecimal;9],
        pub latest_funding_rates: [AnchorDecimal;16],
        pub latest_funding_rates_padding: [AnchorDecimal;9],
        pub latest_midpoints: [u64;16],
        pub latest_midpoints_padding: [u64;9],
        pub oracles: [Pubkey;16],
        pub oracles_padding: [Pubkey;9],
        pub oracle_backup_feeds: [Pubkey;16],
        pub oracle_backup_feeds_padding: [Pubkey;9],
        pub markets: [Pubkey;16],
        pub markets_padding: [Pubkey;9],
        pub perp_sync_queues: [Pubkey;16],
        pub perp_sync_queues_padding: [Pubkey;9],
        pub perp_parameters: [PerpParameters;16],
        pub perp_parameters_padding: [PerpParameters;9],
        pub margin_parameters: [MarginParameters;16],
        pub margin_parameters_padding: [MarginParameters;9],
        pub products: [Product;16],
        pub products_padding: [Product;9],
        pub zeta_group_keys: [Pubkey;16],
        pub zeta_group_keys_padding: [Pubkey;9],
        pub total_insurance_vault_deposits: u64,
        pub last_withdraw_timestamp: u64,
        pub net_outflow_sum: i64,
        pub halt_force_pricing: [bool;16],
        pub halt_force_pricing_padding: [bool;9],
        pub padding: [u8;2707]
    }

   #[account]
    pub struct Greeks {
        pub nonce: u8,
        pub mark_prices: [u64;46],
        pub mark_prices_padding: [u64;91],
        pub perp_mark_price: u64,
        pub product_greeks: [ProductGreeks;22],
        pub product_greeks_padding: [ProductGreeks;44],
        pub update_timestamp: [u64;2],
        pub update_timestamp_padding: [u64;4],
        pub retreat_expiration_timestamp: [u64;2],
        pub retreat_expiration_timestamp_padding: [u64;4],
        pub interest_rate: [i64;2],
        pub interest_rate_padding: [i64;4],
        pub nodes: [u64;5],
        pub volatility: [u64;10],
        pub volatility_padding: [u64;20],
        pub node_keys: [Pubkey;138],
        pub halt_force_pricing: [bool;6],
        pub perp_update_timestamp: u64,
        pub perp_funding_delta: AnchorDecimal,
        pub perp_latest_funding_rate: AnchorDecimal,
        pub perp_latest_midpoint: u64,
        pub padding: [u8;1593]
    }

   #[account]
    pub struct MarketIndexes {
        pub nonce: u8,
        pub initialized: bool,
        pub indexes: [u8;138]
    }

   #[account]
    pub struct OpenOrdersMap {
        pub user_key: Pubkey
    }

   #[account]
    pub struct CrossOpenOrdersMap {
        pub user_key: Pubkey,
        pub subaccount_index: u8
    }

   #[account]
    pub struct State {
        pub admin: Pubkey,
        pub state_nonce: u8,
        pub serum_nonce: u8,
        pub mint_auth_nonce: u8,
        pub num_underlyings: u8,
        pub num_flex_underlyings: u8,
        pub null: [u8;7],
        pub strike_initialization_threshold_seconds: u32,
        pub pricing_frequency_seconds: u32,
        pub liquidator_liquidation_percentage: u32,
        pub insurance_vault_liquidation_percentage: u32,
        pub deprecated_fee_values: [u64;3],
        pub native_deposit_limit: u64,
        pub expiration_threshold_seconds: u32,
        pub position_movement_fee_bps: u8,
        pub margin_concession_percentage: u8,
        pub treasury_wallet_nonce: u8,
        pub deprecated_option_fee_values: [u64;2],
        pub referrals_admin: Pubkey,
        pub referrals_rewards_wallet_nonce: u8,
        pub max_perp_delta_age: u16,
        pub secondary_admin: Pubkey,
        pub vault_nonce: u8,
        pub insurance_vault_nonce: u8,
        pub deprecated_total_insurance_vault_deposits: u64,
        pub native_withdraw_limit: u64,
        pub withdraw_limit_epoch_seconds: u32,
        pub native_open_interest_limit: u64,
        pub halt_states: [HaltStateV2;16],
        pub halt_states_padding: [HaltStateV2;9],
        pub trigger_admin: Pubkey,
        pub min_lot_sizes: [u32;16],
        pub min_lot_sizes_padding: [u32;9],
        pub tick_sizes: [u32;16],
        pub tick_sizes_padding: [u32;9],
        pub deprecated_maker_fee_value: u64,
        pub native_take_trigger_order_fee_percentage: u64,
        pub native_maker_rebate_percentage: u64,
        pub ma_type_admin: Pubkey,
        pub pricing_admin: Pubkey,
        pub padding: [u8;18]
    }

   #[account]
    pub struct Underlying {
        pub mint: Pubkey
    }

   #[account]
    pub struct SettlementAccount {
        pub settlement_price: u64,
        pub strikes: [u64;23]
    }

   #[account]
    pub struct PerpSyncQueue {
        pub nonce: u8,
        pub head: u16,
        pub length: u16,
        pub queue: [AnchorDecimal;600]
    }

   #[account]
    pub struct ZetaGroup {
        pub nonce: u8,
        pub nonce_padding: [u8;2],
        pub front_expiry_index: u8,
        pub halt_state: HaltState,
        pub underlying_mint: Pubkey,
        pub oracle: Pubkey,
        pub greeks: Pubkey,
        pub pricing_parameters: PricingParameters,
        pub margin_parameters: MarginParameters,
        pub margin_parameters_padding: [u8;104],
        pub products: [Product;46],
        pub products_padding: [Product;91],
        pub perp: Product,
        pub expiry_series: [ExpirySeries;2],
        pub expiry_series_padding: [ExpirySeries;4],
        pub deprecated_padding: [u8;8],
        pub asset: Asset,
        pub expiry_interval_seconds: u32,
        pub new_expiry_threshold_seconds: u32,
        pub perp_parameters: PerpParameters,
        pub perp_sync_queue: Pubkey,
        pub oracle_backup_feed: Pubkey,
        pub perps_only: bool,
        pub flex_underlying: bool,
        pub padding: [u8;964]
    }

   #[account]
    pub struct MarketNode {
        pub index: u8,
        pub nonce: u8,
        pub node_updates: [i64;5],
        pub interest_update: i64
    }

   #[account]
    pub struct SpreadAccount {
        pub authority: Pubkey,
        pub nonce: u8,
        pub balance: u64,
        pub series_expiry: [u64;5],
        pub series_expiry_padding: u64,
        pub positions: [Position;46],
        pub positions_padding: [Position;92],
        pub asset: Asset,
        pub padding: [u8;262]
    }

   #[account]
    pub struct CrossMarginAccountManager {
        pub nonce: u8,
        pub authority: Pubkey,
        pub accounts: [CrossMarginAccountInfo;20],
        pub referrer: Pubkey,
        pub airdrop_community: u8,
        pub padding: [u8;22]
    }

   #[account]
    pub struct CrossMarginAccount {
        pub authority: Pubkey,
        pub delegated_pubkey: Pubkey,
        pub balance: u64,
        pub subaccount_index: u8,
        pub nonce: u8,
        pub force_cancel_flag: bool,
        pub account_type: MarginAccountType,
        pub open_orders_nonces: [u8;16],
        pub open_orders_nonces_padding: [u8;9],
        pub rebalance_amount: i64,
        pub last_funding_deltas: [AnchorDecimal;16],
        pub last_funding_deltas_padding: [AnchorDecimal;9],
        pub product_ledgers: [ProductLedger;16],
        pub product_ledgers_padding: [ProductLedger;9],
        pub trigger_order_bits: u128,
        pub rebate_rebalance_amount: u64,
        pub potential_order_loss: [u64;16],
        pub potential_order_loss_padding: [u64;9],
        pub padding: [u8;1776]
    }

   #[account]
    pub struct MarginAccount {
        pub authority: Pubkey,
        pub nonce: u8,
        pub balance: u64,
        pub force_cancel_flag: bool,
        pub open_orders_nonce: [u8;138],
        pub series_expiry: [u64;5],
        pub series_expiry_padding: u64,
        pub product_ledgers: [ProductLedger;46],
        pub product_ledgers_padding: [ProductLedger;91],
        pub perp_product_ledger: ProductLedger,
        pub rebalance_amount: i64,
        pub asset: Asset,
        pub account_type: MarginAccountType,
        pub last_funding_delta: AnchorDecimal,
        pub delegated_pubkey: Pubkey,
        pub rebate_rebalance_amount: u64,
        pub padding: [u8;330]
    }

   #[account]
    pub struct TriggerOrder {
        pub owner: Pubkey,
        pub margin_account: Pubkey,
        pub open_orders: Pubkey,
        pub order_price: u64,
        pub trigger_price: Option<u64>,
        pub trigger_ts: Option<u64>,
        pub size: u64,
        pub creation_ts: u64,
        pub trigger_direction: Option<TriggerDirection>,
        pub side: Side,
        pub asset: Asset,
        pub order_type: OrderType,
        pub bit: u8,
        pub reduce_only: bool
    }

   #[account]
    pub struct SocializedLossAccount {
        pub nonce: u8,
        pub overbankrupt_amount: u64
    }

   #[account]
    pub struct WhitelistDepositAccount {
        pub nonce: u8,
        pub user_key: Pubkey
    }

   #[account]
    pub struct WhitelistInsuranceAccount {
        pub nonce: u8,
        pub user_key: Pubkey
    }

   #[account]
    pub struct InsuranceDepositAccount {
        pub nonce: u8,
        pub amount: u64
    }

   #[account]
    pub struct WhitelistTradingFeesAccount {
        pub nonce: u8,
        pub user_key: Pubkey
    }

   #[account]
    pub struct ReferrerIdAccount {
        pub referrer_id: [u8;6],
        pub referrer_pubkey: Pubkey
    }

   #[account]
    pub struct ReferrerPubkeyAccount {
        pub referrer_id: [u8;6]
    }  
}
        
// Defined types
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProductGreeks {
    pub delta: u64,
    pub vega: AnchorDecimal,
    pub volatility: AnchorDecimal,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AnchorDecimal {
    pub flags: u32,
    pub hi: u32,
    pub lo: u32,
    pub mid: u32,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct HaltStateV2 {
    pub halted: bool,
    pub timestamp: u64,
    pub spot_price: u64,
    pub market_cleaned: bool,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MarginParameters {
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PerpParameters {
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub impact_cash_delta: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ExpirySeries {
    pub active_ts: u64,
    pub expiry_ts: u64,
    pub dirty: bool,
    pub padding: [u8;15],
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Strike {
    pub is_set: bool,
    pub value: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Product {
    pub market: Pubkey,
    pub strike: Strike,
    pub dirty: bool,
    pub kind: Kind,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Position {
    pub size: i64,
    pub cost_of_trades: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OrderState {
    pub closing_orders: u64,
    pub opening_orders: [u64;2],
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProductLedger {
    pub position: Position,
    pub order_state: OrderState,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CrossMarginAccountInfo {
    pub initialized: bool,
    pub name: [u8;10],
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OrderArgs {
    pub price: u64,
    pub size: u64,
    pub client_order_id: Option<u64>,
    pub tif_offset: Option<u16>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct HaltStateArgs {
    pub asset: Asset,
    pub spot_price: u64,
    pub timestamp: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct HaltArgs {
    pub spot_prices: [u64;16],
    pub timestamp: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateVolatilityArgs {
    pub expiry_index: u8,
    pub volatility: [u64;5],
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateInterestRateArgs {
    pub expiry_index: u8,
    pub interest_rate: i64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ExpireSeriesOverrideArgs {
    pub settlement_nonce: u8,
    pub settlement_price: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeMarketArgs {
    pub asset: Asset,
    pub vault_signer_nonce: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeMarketNodeArgs {
    pub nonce: u8,
    pub index: u8,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OverrideExpiryArgs {
    pub expiry_index: u8,
    pub active_ts: u64,
    pub expiry_ts: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateMarginParametersArgs {
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdatePerpParametersArgs {
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub perp_impact_cash_delta: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateZetaGroupExpiryArgs {
    pub expiry_interval_seconds: u32,
    pub new_expiry_threshold_seconds: u32,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateGreeksArgs {
    pub index: u8,
    pub theo: u64,
    pub delta: u32,
    pub gamma: u32,
    pub volatility: u32,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PositionMovementArg {
    pub index: u8,
    pub size: i64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateZetaPricingPubkeysArgs {
    pub asset: Asset,
    pub oracle: Pubkey,
    pub market: Pubkey,
    pub perp_sync_queue: Pubkey,
    pub zeta_group_key: Pubkey,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeZetaPricingArgs {
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub perp_impact_cash_delta: u64,
    pub margin_initial: u64,
    pub margin_maintenance: u64,
    pub pricing_nonce: u8,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum ExpirySeriesStatus {
    Uninitialized,
    Initialized,
    Live,
    Expired,
    ExpiredDirty
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum Kind {
    Uninitialized,
    Call,
    Put,
    Future,
    Perp
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    PostOnly,
    FillOrKill,
    ImmediateOrCancel,
    PostOnlySlide,
    PostOnlyFront
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum Side {
    Uninitialized,
    Bid,
    Ask
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum TriggerDirection {
    Uninitialized,
    LessThanOrEqual,
    GreaterThanOrEqual
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
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
    Rndr,
    Undefined
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum MovementType {
    Undefined,
    Lock,
    Unlock
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum TreasuryMovementType {
    Undefined,
    ToTreasuryFromInsurance,
    ToInsuranceFromTreasury,
    ToTreasuryFromReferralsRewards,
    ToReferralsRewardsFromTreasury
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum OrderCompleteType {
    Cancel,
    Fill,
    Booted
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum MarginRequirement {
    Initial,
    Maintenance,
    MaintenanceIncludingOrders,
    MarketMakerConcession
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum PlaceOrderType {
    PlaceOrder,
    PlacePerpOrder
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum ValidationType {
    Place,
    Cancel,
    OpenOrders,
    Liquidate
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum TraitType {
    MarginAccount,
    CrossMarginAccount
}