use anchor_lang::prelude::Pubkey;

anchor_client_gen::generate!(
    idl_path = "idl.json",
    program_id = "NativeLoader1111111111111111111111111111111",
    skip_errors,
    zero_copy_unsafe(State, FeeStructure),
    repr_c(State, FeeStructure)
);

impl Default for types::FeeStructure {
    fn default() -> Self {
        Self {
            fee_tiers: [types::FeeTier::default(); 10],
            filler_reward_structure: types::OrderFillerRewardStructure::default(),
            referrer_reward_epoch_upper_bound: 0,
            flat_filler_fee: 0,
        }
    }
}

impl Default for accounts::State {
    fn default() -> Self {
        Self {
            admin: Pubkey::default(),
            whitelist_mint: Pubkey::default(),
            discount_mint: Pubkey::default(),
            signer: Pubkey::default(),
            srm_vault: Pubkey::default(),
            perp_fee_structure: types::FeeStructure::default(),
            spot_fee_structure: types::FeeStructure::default(),
            oracle_guard_rails: types::OracleGuardRails::default(),
            number_of_authorities: 0,
            number_of_sub_accounts: 0,
            lp_cooldown_time: 0,
            liquidation_margin_buffer_ratio: 0,
            settlement_duration: 0,
            number_of_markets: 0,
            number_of_spot_markets: 0,
            signer_nonce: 0,
            min_perp_auction_duration: 0,
            default_market_order_time_in_force: 0,
            default_spot_auction_duration: 0,
            exchange_status: 0,
            liquidation_duration: 0,
            initial_pct_to_liquidate: 0,
            padding: [0; 14],
        }
    }
}
