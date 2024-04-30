use anchor_lang::prelude::*;

declare_id!("ZETAxsqBRek56DhiGXrn75yj2NHU3aYUnxvHXpkf3aD");

// Accounts
#[derive(Accounts)]
pub struct InitializeZetaPricing<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaPricingPubkeys<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeZetaGroup<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub underlying_mint: AccountInfo<'info>,
    pub zeta_program: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    pub perp_sync_queue: AccountInfo<'info>,
    #[account(mut)]
    pub underlying: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub usdc_mint: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct OverrideExpiry<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MigrateToNewCrossMarginAccount<'info> {
    #[account(mut)]
    pub new_cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub old_cross_margin_account: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MigrateToCrossMarginAccount<'info> {
    #[account(mut)]
    pub cross_margin_account: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCrossMarginAccountManager<'info> {
    #[account(mut)]
    pub cross_margin_account_manager: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub zeta_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCrossMarginAccountManagerV2<'info> {
    #[account(mut)]
    pub cross_margin_account_manager: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub zeta_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCrossMarginAccount<'info> {
    #[account(mut)]
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub cross_margin_account_manager: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub zeta_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarginAccount<'info> {
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub zeta_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeSpreadAccount<'info> {
    #[account(mut)]
    pub spread_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub zeta_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseCrossMarginAccountManager<'info> {
    #[account(mut)]
    pub cross_margin_account_manager: AccountInfo<'info>,
    #[account(mut)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseCrossMarginAccount<'info> {
    #[account(mut)]
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub cross_margin_account_manager: AccountInfo<'info>,
    #[account(mut)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseMarginAccount<'info> {
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseSpreadAccount<'info> {
    #[account(mut)]
    pub spread_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeUnderlying<'info> {
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub zeta_program: AccountInfo<'info>,
    #[account(mut)]
    pub state: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    #[account(mut)]
    pub underlying: AccountInfo<'info>,
    pub underlying_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializePerpSyncQueue<'info> {
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub zeta_program: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub perp_sync_queue: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarketIndexes<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub market_indexes: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarketNode<'info> {
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    pub greeks: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Halt<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Unhalt<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateHaltState<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateVolatility<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub greeks: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateInterestRate<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddPerpMarketIndex<'info> {
    #[account(mut)]
    pub market_indexes: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddMarketIndexes<'info> {
    #[account(mut)]
    pub market_indexes: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeZetaState<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub treasury_wallet: AccountInfo<'info>,
    pub referrals_admin: AccountInfo<'info>,
    #[account(mut)]
    pub referrals_rewards_wallet: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub secondary_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeZetaTreasuryWallet<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub treasury_wallet: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeZetaReferralsRewardsWallet<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub referrals_rewards_wallet: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateSecondaryAdmin<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateTriggerAdmin<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMaTypeAdmin<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateReferralsAdmin<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePricingAdmin<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMakerRebatePercentage<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateTakeTriggerOrderFeePercentage<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaState<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateOracle<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateOracleBackupFeed<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePricingParameters<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMarginParameters<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaGroupMarginParameters<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePerpParameters<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaGroupPerpParameters<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateZetaGroupExpiryParameters<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ToggleZetaGroupPerpsOnly<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CleanZetaMarkets<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CleanZetaMarketHalted<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    pub bids: AccountInfo<'info>,
    pub asks: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SettlePositionsHalted<'info> {
    pub state: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarketStrikes<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_group: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExpireSeriesOverride {}

#[derive(Accounts)]
pub struct ExpireSeries {}

#[derive(Accounts)]
pub struct InitializeZetaMarket<'info> {
    pub state: AccountInfo<'info>,
    pub market_indexes: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub base_mint: AccountInfo<'info>,
    #[account(mut)]
    pub quote_mint: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_base_vault: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_quote_vault: AccountInfo<'info>,
    #[account(mut)]
    pub dex_base_vault: AccountInfo<'info>,
    #[account(mut)]
    pub dex_quote_vault: AccountInfo<'info>,
    pub vault_owner: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarketTifEpochCycle<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePricingV2<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub perp_market: AccountInfo<'info>,
    pub perp_bids: AccountInfo<'info>,
    pub perp_asks: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePricingV3<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub perp_market: AccountInfo<'info>,
    pub perp_bids: AccountInfo<'info>,
    pub perp_asks: AccountInfo<'info>,
    #[account(mut, signer)]
    pub pricing_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApplyPerpFunding<'info> {
    pub state: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositV2<'info> {
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositPermissionless<'info> {
    #[account(mut)]
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub deposit_token_acc: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositInsuranceVault<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_deposit_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_vault: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositInsuranceVaultV2<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_deposit_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_vault: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ChooseAirdropCommunity<'info> {
    #[account(mut)]
    pub cross_margin_account_manager: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawV2<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawInsuranceVault<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_deposit_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawInsuranceVaultV2<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_deposit_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrders<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders_map: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrdersV2<'info> {
    pub state: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders_map: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrdersV3<'info> {
    pub state: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders_map: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseOpenOrders<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders_map: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseOpenOrdersV2<'info> {
    pub state: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders_map: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseOpenOrdersV3<'info> {
    pub state: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders_map: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseOpenOrdersV4<'info> {
    pub state: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub cross_margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub authority: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders_map: AccountInfo<'info>,
    pub event_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeWhitelistDepositAccount<'info> {
    #[account(mut)]
    pub whitelist_deposit_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeWhitelistInsuranceAccount<'info> {
    #[account(mut)]
    pub whitelist_insurance_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeWhitelistTradingFeesAccount<'info> {
    #[account(mut)]
    pub whitelist_trading_fees_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeInsuranceDepositAccount<'info> {
    #[account(mut)]
    pub insurance_deposit_account: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub whitelist_insurance_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCombinedInsuranceVault<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_vault: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCombinedVault<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeCombinedSocializedLossAccount<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub usdc_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub market_accounts: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    pub market_mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrderV2<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub market_accounts: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    pub market_mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrderV3<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub market_accounts: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    pub market_mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrder<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub market_accounts: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub market_mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrderV2<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub market_accounts: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub market_mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrderV4<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub market_accounts: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    pub market_mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrderV3<'info> {
    pub state: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub market_accounts: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub market_mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrderV4<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub place_order_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceMultiOrders<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub market_base_vault: AccountInfo<'info>,
    #[account(mut)]
    pub market_quote_vault: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_base_vault: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_quote_vault: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub market_base_mint: AccountInfo<'info>,
    #[account(mut)]
    pub market_quote_mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceTriggerOrder<'info> {
    pub state: AccountInfo<'info>,
    pub open_orders: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    pub trigger_order: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTriggerOrderV2<'info> {
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub trigger_order: AccountInfo<'info>,
    pub place_order_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TakeTriggerOrder<'info> {
    #[account(mut)]
    pub trigger_order: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub bids: AccountInfo<'info>,
    pub asks: AccountInfo<'info>,
    #[account(mut, signer)]
    pub taker: AccountInfo<'info>,
    #[account(mut)]
    pub taker_margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub order_margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTriggerOrder<'info> {
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub trigger_order: AccountInfo<'info>,
    pub place_order_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelTriggerOrder<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub trigger_order: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelTriggerOrderV2<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub trigger_order: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelTriggerOrder<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub trigger_order: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMinLot<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateTickSize<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMinLotsAndTickSizes<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct EditTriggerOrder<'info> {
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub trigger_order: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct EditTriggerOrderV2<'info> {
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub trigger_order: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrderNoError<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelAllMarketOrders<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrderHalted<'info> {
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrderByClientOrderId<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrderByClientOrderIdNoError<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PruneExpiredTifOrders<'info> {
    pub dex_program: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PruneExpiredTifOrdersV2<'info> {
    pub dex_program: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrderByOrderIdV2<'info> {
    pub pricing: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrderByOrderId<'info> {
    pub zeta_group: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrdersV2<'info> {
    pub pricing: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrders<'info> {
    pub zeta_group: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub cancel_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CrankEventQueue<'info> {
    pub state: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub perp_sync_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CollectTreasuryFunds<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub treasury_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub collection_token_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TreasuryMovement<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    pub treasury_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub referrals_rewards_wallet: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RebalanceInsuranceVault<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_vault: AccountInfo<'info>,
    #[account(mut)]
    pub insurance_vault: AccountInfo<'info>,
    #[account(mut)]
    pub treasury_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub socialized_loss_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LiquidateV2<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub liquidator: AccountInfo<'info>,
    #[account(mut)]
    pub liquidator_account: AccountInfo<'info>,
    pub pricing: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub liquidated_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Liquidate<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub liquidator: AccountInfo<'info>,
    #[account(mut)]
    pub liquidator_margin_account: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub liquidated_margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnVaultTokens<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SettleDexFunds<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_base_vault: AccountInfo<'info>,
    #[account(mut)]
    pub zeta_quote_vault: AccountInfo<'info>,
    #[account(mut)]
    pub dex_base_vault: AccountInfo<'info>,
    #[account(mut)]
    pub dex_quote_vault: AccountInfo<'info>,
    pub vault_owner: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PositionMovement<'info> {
    pub state: AccountInfo<'info>,
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub spread_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub greeks: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferExcessSpreadBalance<'info> {
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    pub spread_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ToggleMarketMaker<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeReferrerAccounts<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub referrer_id_account: AccountInfo<'info>,
    #[account(mut)]
    pub referrer_pubkey_account: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseReferrerAccounts<'info> {
    #[account(mut)]
    pub referrer_id_account: AccountInfo<'info>,
    #[account(mut)]
    pub referrer_pubkey_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct EditMaType<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct EditDelegatedPubkey<'info> {
    #[account(mut)]
    pub margin_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ResetNumFlexUnderlyings<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
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

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaPricingPubkeys {
        pub args: UpdateZetaPricingPubkeysArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaGroup {
        pub args: InitializeZetaGroupArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct OverrideExpiry {
        pub args: OverrideExpiryArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct MigrateToNewCrossMarginAccount {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct MigrateToCrossMarginAccount {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCrossMarginAccountManager {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCrossMarginAccountManagerV2 {
        pub referrer: Option<Pubkey>,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCrossMarginAccount {
        pub subaccount_index: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarginAccount {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeSpreadAccount {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseCrossMarginAccountManager {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseCrossMarginAccount {
        pub subaccount_index: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseMarginAccount {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseSpreadAccount {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeUnderlying {
        pub flex_underlying: bool,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePerpSyncQueue {
        pub nonce: u8,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarketIndexes {
        pub nonce: u8,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarketNode {
        pub args: InitializeMarketNodeArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Halt {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Unhalt {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateHaltState {
        pub args: HaltStateArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateVolatility {
        pub args: UpdateVolatilityArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateInterestRate {
        pub args: UpdateInterestRateArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct AddPerpMarketIndex {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct AddMarketIndexes {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaState {
        pub args: InitializeStateArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaTreasuryWallet {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaReferralsRewardsWallet {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateAdmin {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSecondaryAdmin {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateTriggerAdmin {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateMaTypeAdmin {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateReferralsAdmin {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePricingAdmin {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateMakerRebatePercentage {
        pub native_maker_rebate_percentage: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateTakeTriggerOrderFeePercentage {
        pub new_take_trigger_order_fee_percentage: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaState {
        pub args: UpdateStateArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateOracle {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateOracleBackupFeed {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePricingParameters {
        pub args: UpdatePricingParametersArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateMarginParameters {
        pub args: UpdateMarginParametersArgs,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaGroupMarginParameters {
        pub args: UpdateMarginParametersArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpParameters {
        pub args: UpdatePerpParametersArgs,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaGroupPerpParameters {
        pub args: UpdatePerpParametersArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateZetaGroupExpiryParameters {
        pub args: UpdateZetaGroupExpiryArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ToggleZetaGroupPerpsOnly {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CleanZetaMarkets {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CleanZetaMarketHalted {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct SettlePositionsHalted {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarketStrikes {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ExpireSeriesOverride {
        pub args: ExpireSeriesOverrideArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ExpireSeries {
        pub settlement_nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeZetaMarket {
        pub args: InitializeMarketArgs,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMarketTifEpochCycle {
        pub epoch_length: u16,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePricingV2 {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePricingV3 {
        pub asset: Asset,
        pub price: u64,
        pub timestamp: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ApplyPerpFunding {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Deposit {
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DepositV2 {
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DepositPermissionless {
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DepositInsuranceVault {
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DepositInsuranceVaultV2 {
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ChooseAirdropCommunity {
        pub community: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Withdraw {
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct WithdrawV2 {
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct WithdrawInsuranceVault {
        pub percentage_amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct WithdrawInsuranceVaultV2 {
        pub percentage_amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeOpenOrders {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeOpenOrdersV2 {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeOpenOrdersV3 {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseOpenOrders {
        pub map_nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseOpenOrdersV2 {
        pub map_nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseOpenOrdersV3 {
        pub map_nonce: u8,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseOpenOrdersV4 {
        pub map_nonce: u8,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeWhitelistDepositAccount {
        pub nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeWhitelistInsuranceAccount {
        pub nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeWhitelistTradingFeesAccount {
        pub nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeInsuranceDepositAccount {
        pub nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCombinedInsuranceVault {
        pub nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCombinedVault {
        pub nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeCombinedSocializedLossAccount {
        pub nonce: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceOrder {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub client_order_id: Option<u64>,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceOrderV2 {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
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

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlacePerpOrder {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub order_type: OrderType,
        pub client_order_id: Option<u64>,
        pub tag: Option<String>,
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

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceMultiOrders {
        pub asset: Asset,
        pub bid_orders: Vec<OrderArgs>,
        pub ask_orders: Vec<OrderArgs>,
        pub order_type: OrderType,
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

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ExecuteTriggerOrderV2 {
        pub trigger_order_bit: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct TakeTriggerOrder {
        pub trigger_order_bit: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ExecuteTriggerOrder {
        pub trigger_order_bit: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelTriggerOrder {
        pub trigger_order_bit: u8,
        pub enforce_tpsl_conditions: bool,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelTriggerOrderV2 {
        pub trigger_order_bit: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelTriggerOrder {
        pub trigger_order_bit: u8,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateMinLot {
        pub asset: Asset,
        pub min_lot_size: u32,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateTickSize {
        pub asset: Asset,
        pub tick_size: u32,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeMinLotsAndTickSizes {

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

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrder {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrderNoError {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelAllMarketOrders {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrderHalted {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrderByClientOrderId {
        pub client_order_id: u64,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrderByClientOrderIdNoError {
        pub client_order_id: u64,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PruneExpiredTifOrders {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PruneExpiredTifOrdersV2 {
        pub limit: u16,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelOrderByOrderIdV2 {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelOrderByOrderId {
        pub side: Side,
        pub order_id: u128,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelOrdersV2 {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelOrders {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CrankEventQueue {
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CollectTreasuryFunds {
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct TreasuryMovement {
        pub treasury_movement_type: TreasuryMovementType,
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct RebalanceInsuranceVault {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct LiquidateV2 {
        pub size: u64,
        pub asset: Asset,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Liquidate {
        pub size: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct BurnVaultTokens {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct SettleDexFunds {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct PositionMovement {
        pub movement_type: MovementType,
        pub movements: Vec<PositionMovementArg>,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct TransferExcessSpreadBalance {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ToggleMarketMaker {
        pub is_market_maker: bool,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeReferrerAccounts {
        pub referrer_id: String,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CloseReferrerAccounts {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct EditMaType {
        pub ma_type: MarginAccountType,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct EditDelegatedPubkey {
        pub new_key: Pubkey,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct ResetNumFlexUnderlyings {

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