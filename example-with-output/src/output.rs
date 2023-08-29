#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::Pubkey;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        5u8, 135u8, 132u8, 191u8, 20u8, 139u8, 164u8, 40u8, 47u8, 176u8, 18u8, 87u8, 72u8, 136u8,
        169u8, 241u8, 83u8, 160u8, 125u8, 173u8, 247u8, 101u8, 192u8, 69u8, 92u8, 154u8, 151u8,
        3u8, 128u8, 0u8, 0u8, 0u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
pub mod types {
    use anchor_lang::prelude::*;
    pub struct OracleGuardRails {
        pub price_divergence: PriceDivergenceGuardRails,
        pub validity: ValidityGuardRails,
    }
    #[automatically_derived]
    impl ::core::default::Default for OracleGuardRails {
        #[inline]
        fn default() -> OracleGuardRails {
            OracleGuardRails {
                price_divergence: ::core::default::Default::default(),
                validity: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OracleGuardRails {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "OracleGuardRails",
                "price_divergence",
                &self.price_divergence,
                "validity",
                &&self.validity,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for OracleGuardRails {}
    impl borsh::de::BorshDeserialize for OracleGuardRails
    where
        PriceDivergenceGuardRails: borsh::BorshDeserialize,
        ValidityGuardRails: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                price_divergence: borsh::BorshDeserialize::deserialize_reader(reader)?,
                validity: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for OracleGuardRails
    where
        PriceDivergenceGuardRails: borsh::ser::BorshSerialize,
        ValidityGuardRails: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.price_divergence, writer)?;
            borsh::BorshSerialize::serialize(&self.validity, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OracleGuardRails {
        #[inline]
        fn clone(&self) -> OracleGuardRails {
            OracleGuardRails {
                price_divergence: ::core::clone::Clone::clone(&self.price_divergence),
                validity: ::core::clone::Clone::clone(&self.validity),
            }
        }
    }
    pub struct PriceDivergenceGuardRails {
        pub mark_oracle_divergence_numerator: u64,
        pub mark_oracle_divergence_denominator: u64,
    }
    #[automatically_derived]
    impl ::core::default::Default for PriceDivergenceGuardRails {
        #[inline]
        fn default() -> PriceDivergenceGuardRails {
            PriceDivergenceGuardRails {
                mark_oracle_divergence_numerator: ::core::default::Default::default(),
                mark_oracle_divergence_denominator: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PriceDivergenceGuardRails {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "PriceDivergenceGuardRails",
                "mark_oracle_divergence_numerator",
                &self.mark_oracle_divergence_numerator,
                "mark_oracle_divergence_denominator",
                &&self.mark_oracle_divergence_denominator,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for PriceDivergenceGuardRails {}
    impl borsh::de::BorshDeserialize for PriceDivergenceGuardRails
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                mark_oracle_divergence_numerator: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                mark_oracle_divergence_denominator: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for PriceDivergenceGuardRails
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.mark_oracle_divergence_numerator, writer)?;
            borsh::BorshSerialize::serialize(&self.mark_oracle_divergence_denominator, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PriceDivergenceGuardRails {
        #[inline]
        fn clone(&self) -> PriceDivergenceGuardRails {
            PriceDivergenceGuardRails {
                mark_oracle_divergence_numerator: ::core::clone::Clone::clone(
                    &self.mark_oracle_divergence_numerator,
                ),
                mark_oracle_divergence_denominator: ::core::clone::Clone::clone(
                    &self.mark_oracle_divergence_denominator,
                ),
            }
        }
    }
    pub struct ValidityGuardRails {
        pub slots_before_stale_for_amm: i64,
        pub slots_before_stale_for_margin: i64,
        pub confidence_interval_max_size: u64,
        pub too_volatile_ratio: i64,
    }
    #[automatically_derived]
    impl ::core::default::Default for ValidityGuardRails {
        #[inline]
        fn default() -> ValidityGuardRails {
            ValidityGuardRails {
                slots_before_stale_for_amm: ::core::default::Default::default(),
                slots_before_stale_for_margin: ::core::default::Default::default(),
                confidence_interval_max_size: ::core::default::Default::default(),
                too_volatile_ratio: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ValidityGuardRails {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "ValidityGuardRails",
                "slots_before_stale_for_amm",
                &self.slots_before_stale_for_amm,
                "slots_before_stale_for_margin",
                &self.slots_before_stale_for_margin,
                "confidence_interval_max_size",
                &self.confidence_interval_max_size,
                "too_volatile_ratio",
                &&self.too_volatile_ratio,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ValidityGuardRails {}
    impl borsh::de::BorshDeserialize for ValidityGuardRails
    where
        i64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                slots_before_stale_for_amm: borsh::BorshDeserialize::deserialize_reader(reader)?,
                slots_before_stale_for_margin: borsh::BorshDeserialize::deserialize_reader(reader)?,
                confidence_interval_max_size: borsh::BorshDeserialize::deserialize_reader(reader)?,
                too_volatile_ratio: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for ValidityGuardRails
    where
        i64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.slots_before_stale_for_amm, writer)?;
            borsh::BorshSerialize::serialize(&self.slots_before_stale_for_margin, writer)?;
            borsh::BorshSerialize::serialize(&self.confidence_interval_max_size, writer)?;
            borsh::BorshSerialize::serialize(&self.too_volatile_ratio, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ValidityGuardRails {
        #[inline]
        fn clone(&self) -> ValidityGuardRails {
            ValidityGuardRails {
                slots_before_stale_for_amm: ::core::clone::Clone::clone(
                    &self.slots_before_stale_for_amm,
                ),
                slots_before_stale_for_margin: ::core::clone::Clone::clone(
                    &self.slots_before_stale_for_margin,
                ),
                confidence_interval_max_size: ::core::clone::Clone::clone(
                    &self.confidence_interval_max_size,
                ),
                too_volatile_ratio: ::core::clone::Clone::clone(&self.too_volatile_ratio),
            }
        }
    }
    #[repr(C)]
    pub struct FeeStructure {
        pub fee_tiers: [FeeTier; 10],
        pub filler_reward_structure: OrderFillerRewardStructure,
        pub referrer_reward_epoch_upper_bound: u64,
        pub flat_filler_fee: u64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FeeStructure {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "FeeStructure",
                "fee_tiers",
                &self.fee_tiers,
                "filler_reward_structure",
                &self.filler_reward_structure,
                "referrer_reward_epoch_upper_bound",
                &self.referrer_reward_epoch_upper_bound,
                "flat_filler_fee",
                &&self.flat_filler_fee,
            )
        }
    }
    impl borsh::de::BorshDeserialize for FeeStructure
    where
        [FeeTier; 10]: borsh::BorshDeserialize,
        OrderFillerRewardStructure: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                fee_tiers: borsh::BorshDeserialize::deserialize_reader(reader)?,
                filler_reward_structure: borsh::BorshDeserialize::deserialize_reader(reader)?,
                referrer_reward_epoch_upper_bound: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                flat_filler_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for FeeStructure
    where
        [FeeTier; 10]: borsh::ser::BorshSerialize,
        OrderFillerRewardStructure: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.fee_tiers, writer)?;
            borsh::BorshSerialize::serialize(&self.filler_reward_structure, writer)?;
            borsh::BorshSerialize::serialize(&self.referrer_reward_epoch_upper_bound, writer)?;
            borsh::BorshSerialize::serialize(&self.flat_filler_fee, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl FeeStructure {}
    #[automatically_derived]
    impl ::core::marker::Copy for FeeStructure {}
    #[automatically_derived]
    impl ::core::clone::Clone for FeeStructure {
        #[inline]
        fn clone(&self) -> FeeStructure {
            let _: ::core::clone::AssertParamIsClone<[FeeTier; 10]>;
            let _: ::core::clone::AssertParamIsClone<OrderFillerRewardStructure>;
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    pub struct FeeTier {
        pub fee_numerator: u32,
        pub fee_denominator: u32,
        pub maker_rebate_numerator: u32,
        pub maker_rebate_denominator: u32,
        pub referrer_reward_numerator: u32,
        pub referrer_reward_denominator: u32,
        pub referee_fee_numerator: u32,
        pub referee_fee_denominator: u32,
    }
    #[automatically_derived]
    impl ::core::default::Default for FeeTier {
        #[inline]
        fn default() -> FeeTier {
            FeeTier {
                fee_numerator: ::core::default::Default::default(),
                fee_denominator: ::core::default::Default::default(),
                maker_rebate_numerator: ::core::default::Default::default(),
                maker_rebate_denominator: ::core::default::Default::default(),
                referrer_reward_numerator: ::core::default::Default::default(),
                referrer_reward_denominator: ::core::default::Default::default(),
                referee_fee_numerator: ::core::default::Default::default(),
                referee_fee_denominator: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FeeTier {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "fee_numerator",
                "fee_denominator",
                "maker_rebate_numerator",
                "maker_rebate_denominator",
                "referrer_reward_numerator",
                "referrer_reward_denominator",
                "referee_fee_numerator",
                "referee_fee_denominator",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.fee_numerator,
                &self.fee_denominator,
                &self.maker_rebate_numerator,
                &self.maker_rebate_denominator,
                &self.referrer_reward_numerator,
                &self.referrer_reward_denominator,
                &self.referee_fee_numerator,
                &&self.referee_fee_denominator,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "FeeTier", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FeeTier {}
    impl borsh::de::BorshDeserialize for FeeTier
    where
        u32: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                fee_numerator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                fee_denominator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                maker_rebate_numerator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                maker_rebate_denominator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                referrer_reward_numerator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                referrer_reward_denominator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                referee_fee_numerator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                referee_fee_denominator: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for FeeTier
    where
        u32: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.fee_numerator, writer)?;
            borsh::BorshSerialize::serialize(&self.fee_denominator, writer)?;
            borsh::BorshSerialize::serialize(&self.maker_rebate_numerator, writer)?;
            borsh::BorshSerialize::serialize(&self.maker_rebate_denominator, writer)?;
            borsh::BorshSerialize::serialize(&self.referrer_reward_numerator, writer)?;
            borsh::BorshSerialize::serialize(&self.referrer_reward_denominator, writer)?;
            borsh::BorshSerialize::serialize(&self.referee_fee_numerator, writer)?;
            borsh::BorshSerialize::serialize(&self.referee_fee_denominator, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FeeTier {
        #[inline]
        fn clone(&self) -> FeeTier {
            FeeTier {
                fee_numerator: ::core::clone::Clone::clone(&self.fee_numerator),
                fee_denominator: ::core::clone::Clone::clone(&self.fee_denominator),
                maker_rebate_numerator: ::core::clone::Clone::clone(&self.maker_rebate_numerator),
                maker_rebate_denominator: ::core::clone::Clone::clone(
                    &self.maker_rebate_denominator,
                ),
                referrer_reward_numerator: ::core::clone::Clone::clone(
                    &self.referrer_reward_numerator,
                ),
                referrer_reward_denominator: ::core::clone::Clone::clone(
                    &self.referrer_reward_denominator,
                ),
                referee_fee_numerator: ::core::clone::Clone::clone(&self.referee_fee_numerator),
                referee_fee_denominator: ::core::clone::Clone::clone(&self.referee_fee_denominator),
            }
        }
    }
    pub struct OrderFillerRewardStructure {
        pub reward_numerator: u32,
        pub reward_denominator: u32,
        pub time_based_reward_lower_bound: u128,
    }
    #[automatically_derived]
    impl ::core::default::Default for OrderFillerRewardStructure {
        #[inline]
        fn default() -> OrderFillerRewardStructure {
            OrderFillerRewardStructure {
                reward_numerator: ::core::default::Default::default(),
                reward_denominator: ::core::default::Default::default(),
                time_based_reward_lower_bound: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OrderFillerRewardStructure {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "OrderFillerRewardStructure",
                "reward_numerator",
                &self.reward_numerator,
                "reward_denominator",
                &self.reward_denominator,
                "time_based_reward_lower_bound",
                &&self.time_based_reward_lower_bound,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for OrderFillerRewardStructure {}
    impl borsh::de::BorshDeserialize for OrderFillerRewardStructure
    where
        u32: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u128: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                reward_numerator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                reward_denominator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                time_based_reward_lower_bound: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for OrderFillerRewardStructure
    where
        u32: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u128: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.reward_numerator, writer)?;
            borsh::BorshSerialize::serialize(&self.reward_denominator, writer)?;
            borsh::BorshSerialize::serialize(&self.time_based_reward_lower_bound, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OrderFillerRewardStructure {
        #[inline]
        fn clone(&self) -> OrderFillerRewardStructure {
            OrderFillerRewardStructure {
                reward_numerator: ::core::clone::Clone::clone(&self.reward_numerator),
                reward_denominator: ::core::clone::Clone::clone(&self.reward_denominator),
                time_based_reward_lower_bound: ::core::clone::Clone::clone(
                    &self.time_based_reward_lower_bound,
                ),
            }
        }
    }
}
pub mod accounts {
    use anchor_lang::prelude::*;
    #[repr(C)]
    pub struct State {
        pub admin: ::anchor_lang::prelude::Pubkey,
        pub whitelist_mint: ::anchor_lang::prelude::Pubkey,
        pub discount_mint: ::anchor_lang::prelude::Pubkey,
        pub signer: ::anchor_lang::prelude::Pubkey,
        pub srm_vault: ::anchor_lang::prelude::Pubkey,
        pub perp_fee_structure: crate::types::FeeStructure,
        pub spot_fee_structure: crate::types::FeeStructure,
        pub oracle_guard_rails: crate::types::OracleGuardRails,
        pub number_of_authorities: u64,
        pub number_of_sub_accounts: u64,
        pub lp_cooldown_time: u64,
        pub liquidation_margin_buffer_ratio: u32,
        pub settlement_duration: u16,
        pub number_of_markets: u16,
        pub number_of_spot_markets: u16,
        pub signer_nonce: u8,
        pub min_perp_auction_duration: u8,
        pub default_market_order_time_in_force: u8,
        pub default_spot_auction_duration: u8,
        pub exchange_status: u8,
        pub liquidation_duration: u8,
        pub initial_pct_to_liquidate: u16,
        pub padding: [u8; 14],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for State {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "admin",
                "whitelist_mint",
                "discount_mint",
                "signer",
                "srm_vault",
                "perp_fee_structure",
                "spot_fee_structure",
                "oracle_guard_rails",
                "number_of_authorities",
                "number_of_sub_accounts",
                "lp_cooldown_time",
                "liquidation_margin_buffer_ratio",
                "settlement_duration",
                "number_of_markets",
                "number_of_spot_markets",
                "signer_nonce",
                "min_perp_auction_duration",
                "default_market_order_time_in_force",
                "default_spot_auction_duration",
                "exchange_status",
                "liquidation_duration",
                "initial_pct_to_liquidate",
                "padding",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.admin,
                &self.whitelist_mint,
                &self.discount_mint,
                &self.signer,
                &self.srm_vault,
                &self.perp_fee_structure,
                &self.spot_fee_structure,
                &self.oracle_guard_rails,
                &self.number_of_authorities,
                &self.number_of_sub_accounts,
                &self.lp_cooldown_time,
                &self.liquidation_margin_buffer_ratio,
                &self.settlement_duration,
                &self.number_of_markets,
                &self.number_of_spot_markets,
                &self.signer_nonce,
                &self.min_perp_auction_duration,
                &self.default_market_order_time_in_force,
                &self.default_spot_auction_duration,
                &self.exchange_status,
                &self.liquidation_duration,
                &self.initial_pct_to_liquidate,
                &&self.padding,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "State", names, values)
        }
    }
    impl borsh::de::BorshDeserialize for State
    where
        ::anchor_lang::prelude::Pubkey: borsh::BorshDeserialize,
        ::anchor_lang::prelude::Pubkey: borsh::BorshDeserialize,
        ::anchor_lang::prelude::Pubkey: borsh::BorshDeserialize,
        ::anchor_lang::prelude::Pubkey: borsh::BorshDeserialize,
        ::anchor_lang::prelude::Pubkey: borsh::BorshDeserialize,
        crate::types::FeeStructure: borsh::BorshDeserialize,
        crate::types::FeeStructure: borsh::BorshDeserialize,
        crate::types::OracleGuardRails: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u16: borsh::BorshDeserialize,
        u16: borsh::BorshDeserialize,
        u16: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        u16: borsh::BorshDeserialize,
        [u8; 14]: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
                whitelist_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                discount_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                signer: borsh::BorshDeserialize::deserialize_reader(reader)?,
                srm_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                perp_fee_structure: borsh::BorshDeserialize::deserialize_reader(reader)?,
                spot_fee_structure: borsh::BorshDeserialize::deserialize_reader(reader)?,
                oracle_guard_rails: borsh::BorshDeserialize::deserialize_reader(reader)?,
                number_of_authorities: borsh::BorshDeserialize::deserialize_reader(reader)?,
                number_of_sub_accounts: borsh::BorshDeserialize::deserialize_reader(reader)?,
                lp_cooldown_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                liquidation_margin_buffer_ratio: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                settlement_duration: borsh::BorshDeserialize::deserialize_reader(reader)?,
                number_of_markets: borsh::BorshDeserialize::deserialize_reader(reader)?,
                number_of_spot_markets: borsh::BorshDeserialize::deserialize_reader(reader)?,
                signer_nonce: borsh::BorshDeserialize::deserialize_reader(reader)?,
                min_perp_auction_duration: borsh::BorshDeserialize::deserialize_reader(reader)?,
                default_market_order_time_in_force: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                default_spot_auction_duration: borsh::BorshDeserialize::deserialize_reader(reader)?,
                exchange_status: borsh::BorshDeserialize::deserialize_reader(reader)?,
                liquidation_duration: borsh::BorshDeserialize::deserialize_reader(reader)?,
                initial_pct_to_liquidate: borsh::BorshDeserialize::deserialize_reader(reader)?,
                padding: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for State
    where
        ::anchor_lang::prelude::Pubkey: borsh::ser::BorshSerialize,
        ::anchor_lang::prelude::Pubkey: borsh::ser::BorshSerialize,
        ::anchor_lang::prelude::Pubkey: borsh::ser::BorshSerialize,
        ::anchor_lang::prelude::Pubkey: borsh::ser::BorshSerialize,
        ::anchor_lang::prelude::Pubkey: borsh::ser::BorshSerialize,
        crate::types::FeeStructure: borsh::ser::BorshSerialize,
        crate::types::FeeStructure: borsh::ser::BorshSerialize,
        crate::types::OracleGuardRails: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u16: borsh::ser::BorshSerialize,
        u16: borsh::ser::BorshSerialize,
        u16: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        u16: borsh::ser::BorshSerialize,
        [u8; 14]: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.admin, writer)?;
            borsh::BorshSerialize::serialize(&self.whitelist_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.discount_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.signer, writer)?;
            borsh::BorshSerialize::serialize(&self.srm_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.perp_fee_structure, writer)?;
            borsh::BorshSerialize::serialize(&self.spot_fee_structure, writer)?;
            borsh::BorshSerialize::serialize(&self.oracle_guard_rails, writer)?;
            borsh::BorshSerialize::serialize(&self.number_of_authorities, writer)?;
            borsh::BorshSerialize::serialize(&self.number_of_sub_accounts, writer)?;
            borsh::BorshSerialize::serialize(&self.lp_cooldown_time, writer)?;
            borsh::BorshSerialize::serialize(&self.liquidation_margin_buffer_ratio, writer)?;
            borsh::BorshSerialize::serialize(&self.settlement_duration, writer)?;
            borsh::BorshSerialize::serialize(&self.number_of_markets, writer)?;
            borsh::BorshSerialize::serialize(&self.number_of_spot_markets, writer)?;
            borsh::BorshSerialize::serialize(&self.signer_nonce, writer)?;
            borsh::BorshSerialize::serialize(&self.min_perp_auction_duration, writer)?;
            borsh::BorshSerialize::serialize(&self.default_market_order_time_in_force, writer)?;
            borsh::BorshSerialize::serialize(&self.default_spot_auction_duration, writer)?;
            borsh::BorshSerialize::serialize(&self.exchange_status, writer)?;
            borsh::BorshSerialize::serialize(&self.liquidation_duration, writer)?;
            borsh::BorshSerialize::serialize(&self.initial_pct_to_liquidate, writer)?;
            borsh::BorshSerialize::serialize(&self.padding, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl State {}
    #[automatically_derived]
    impl ::core::marker::Copy for State {}
    #[automatically_derived]
    impl ::core::clone::Clone for State {
        #[inline]
        fn clone(&self) -> State {
            let _: ::core::clone::AssertParamIsClone<::anchor_lang::prelude::Pubkey>;
            let _: ::core::clone::AssertParamIsClone<::anchor_lang::prelude::Pubkey>;
            let _: ::core::clone::AssertParamIsClone<::anchor_lang::prelude::Pubkey>;
            let _: ::core::clone::AssertParamIsClone<::anchor_lang::prelude::Pubkey>;
            let _: ::core::clone::AssertParamIsClone<::anchor_lang::prelude::Pubkey>;
            let _: ::core::clone::AssertParamIsClone<crate::types::FeeStructure>;
            let _: ::core::clone::AssertParamIsClone<crate::types::FeeStructure>;
            let _: ::core::clone::AssertParamIsClone<crate::types::OracleGuardRails>;
            let _: ::core::clone::AssertParamIsClone<u64>;
            let _: ::core::clone::AssertParamIsClone<u32>;
            let _: ::core::clone::AssertParamIsClone<u16>;
            let _: ::core::clone::AssertParamIsClone<u8>;
            let _: ::core::clone::AssertParamIsClone<[u8; 14]>;
            *self
        }
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for State {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for State {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for State {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for State {
        const DISCRIMINATOR: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for State {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [216, 146, 107, 94, 104, 75, 182, 177].len() {
                return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
            }
            let given_disc = &buf[..8];
            if &[216, 146, 107, 94, 104, 75, 182, 177] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                            .name(),
                        error_code_number:
                            anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
                        error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                            .to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "example-with-output/src/lib.rs",
                                line: 3u32,
                            },
                        )),
                        compared_values: None,
                    })
                    .with_account_name("State"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let data: &[u8] = &buf[8..];
            let account = anchor_lang::__private::bytemuck::from_bytes(data);
            Ok(*account)
        }
    }
    #[automatically_derived]
    impl anchor_lang::Owner for State {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
}
pub mod instructions {
    use anchor_lang::{
        prelude::{borsh, AccountMeta},
        solana_program::instruction::Instruction,
    };
    pub struct InitializeUserAccounts {
        pub user: ::anchor_lang::prelude::Pubkey,
        pub user_stats: ::anchor_lang::prelude::Pubkey,
        pub state: ::anchor_lang::prelude::Pubkey,
        pub authority: ::anchor_lang::prelude::Pubkey,
        pub payer: ::anchor_lang::prelude::Pubkey,
        pub rent: ::anchor_lang::prelude::Pubkey,
        pub system_program: ::anchor_lang::prelude::Pubkey,
    }
    #[automatically_derived]
    impl ::core::default::Default for InitializeUserAccounts {
        #[inline]
        fn default() -> InitializeUserAccounts {
            InitializeUserAccounts {
                user: ::core::default::Default::default(),
                user_stats: ::core::default::Default::default(),
                state: ::core::default::Default::default(),
                authority: ::core::default::Default::default(),
                payer: ::core::default::Default::default(),
                rent: ::core::default::Default::default(),
                system_program: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InitializeUserAccounts {
        #[inline]
        fn clone(&self) -> InitializeUserAccounts {
            InitializeUserAccounts {
                user: ::core::clone::Clone::clone(&self.user),
                user_stats: ::core::clone::Clone::clone(&self.user_stats),
                state: ::core::clone::Clone::clone(&self.state),
                authority: ::core::clone::Clone::clone(&self.authority),
                payer: ::core::clone::Clone::clone(&self.payer),
                rent: ::core::clone::Clone::clone(&self.rent),
                system_program: ::core::clone::Clone::clone(&self.system_program),
            }
        }
    }
    struct InitializeUserData {
        discriminator: [u8; 8],
        sub_account_id: u16,
        name: [u8; 32],
    }
    impl borsh::ser::BorshSerialize for InitializeUserData
    where
        [u8; 8]: borsh::ser::BorshSerialize,
        u16: borsh::ser::BorshSerialize,
        [u8; 32]: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.discriminator, writer)?;
            borsh::BorshSerialize::serialize(&self.sub_account_id, writer)?;
            borsh::BorshSerialize::serialize(&self.name, writer)?;
            Ok(())
        }
    }
    pub struct InitializeUser();
    impl InitializeUser {
        pub const DISCRIMINATOR: [u8; 8] = [111, 17, 185, 250, 60, 122, 38, 254];
        pub fn new(
            accounts: InitializeUserAccounts,
            sub_account_id: u16,
            name: [u8; 32],
        ) -> Instruction {
            let data = InitializeUserData {
                discriminator: Self::DISCRIMINATOR,
                sub_account_id,
                name,
            };
            let accounts_metas: Vec<AccountMeta> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta {
                        pubkey: accounts.user,
                        is_writable: true,
                        is_signer: false,
                    },
                    AccountMeta {
                        pubkey: accounts.user_stats,
                        is_writable: true,
                        is_signer: false,
                    },
                    AccountMeta {
                        pubkey: accounts.state,
                        is_writable: true,
                        is_signer: false,
                    },
                    AccountMeta {
                        pubkey: accounts.authority,
                        is_writable: false,
                        is_signer: true,
                    },
                    AccountMeta {
                        pubkey: accounts.payer,
                        is_writable: true,
                        is_signer: true,
                    },
                    AccountMeta {
                        pubkey: accounts.rent,
                        is_writable: false,
                        is_signer: false,
                    },
                    AccountMeta {
                        pubkey: accounts.system_program,
                        is_writable: false,
                        is_signer: false,
                    },
                ]),
            );
            Instruction::new_with_borsh(crate::id(), &data, accounts_metas)
        }
        pub fn new_with_remaining_accounts(
            accounts: InitializeUserAccounts,
            sub_account_id: u16,
            name: [u8; 32],
            remaining_accounts: &Vec<AccountMeta>,
        ) -> Instruction {
            let data = InitializeUserData {
                discriminator: Self::DISCRIMINATOR,
                sub_account_id,
                name,
            };
            let mut accounts_metas: Vec<AccountMeta> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta {
                        pubkey: accounts.user,
                        is_writable: true,
                        is_signer: false,
                    },
                    AccountMeta {
                        pubkey: accounts.user_stats,
                        is_writable: true,
                        is_signer: false,
                    },
                    AccountMeta {
                        pubkey: accounts.state,
                        is_writable: true,
                        is_signer: false,
                    },
                    AccountMeta {
                        pubkey: accounts.authority,
                        is_writable: false,
                        is_signer: true,
                    },
                    AccountMeta {
                        pubkey: accounts.payer,
                        is_writable: true,
                        is_signer: true,
                    },
                    AccountMeta {
                        pubkey: accounts.rent,
                        is_writable: false,
                        is_signer: false,
                    },
                    AccountMeta {
                        pubkey: accounts.system_program,
                        is_writable: false,
                        is_signer: false,
                    },
                ]),
            );
            remaining_accounts.iter().for_each(|meta| {
                accounts_metas.push(meta.clone());
            });
            Instruction::new_with_borsh(crate::id(), &data, accounts_metas)
        }
    }
}
pub mod events {
    use anchor_lang::prelude::*;
    pub struct NewUserRecord {
        pub ts: i64,
        pub user_authority: ::anchor_lang::prelude::Pubkey,
        pub user: ::anchor_lang::prelude::Pubkey,
        pub sub_account_id: u16,
        pub name: [u8; 32],
        pub referrer: ::anchor_lang::prelude::Pubkey,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NewUserRecord {
        #[inline]
        fn clone(&self) -> NewUserRecord {
            NewUserRecord {
                ts: ::core::clone::Clone::clone(&self.ts),
                user_authority: ::core::clone::Clone::clone(&self.user_authority),
                user: ::core::clone::Clone::clone(&self.user),
                sub_account_id: ::core::clone::Clone::clone(&self.sub_account_id),
                name: ::core::clone::Clone::clone(&self.name),
                referrer: ::core::clone::Clone::clone(&self.referrer),
            }
        }
    }
    impl borsh::ser::BorshSerialize for NewUserRecord
    where
        i64: borsh::ser::BorshSerialize,
        ::anchor_lang::prelude::Pubkey: borsh::ser::BorshSerialize,
        ::anchor_lang::prelude::Pubkey: borsh::ser::BorshSerialize,
        u16: borsh::ser::BorshSerialize,
        [u8; 32]: borsh::ser::BorshSerialize,
        ::anchor_lang::prelude::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.ts, writer)?;
            borsh::BorshSerialize::serialize(&self.user_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.user, writer)?;
            borsh::BorshSerialize::serialize(&self.sub_account_id, writer)?;
            borsh::BorshSerialize::serialize(&self.name, writer)?;
            borsh::BorshSerialize::serialize(&self.referrer, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for NewUserRecord
    where
        i64: borsh::BorshDeserialize,
        ::anchor_lang::prelude::Pubkey: borsh::BorshDeserialize,
        ::anchor_lang::prelude::Pubkey: borsh::BorshDeserialize,
        u16: borsh::BorshDeserialize,
        [u8; 32]: borsh::BorshDeserialize,
        ::anchor_lang::prelude::Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                ts: borsh::BorshDeserialize::deserialize_reader(reader)?,
                user_authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                user: borsh::BorshDeserialize::deserialize_reader(reader)?,
                sub_account_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
                name: borsh::BorshDeserialize::deserialize_reader(reader)?,
                referrer: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Event for NewUserRecord {
        fn data(&self) -> Vec<u8> {
            let mut d = [236, 186, 113, 219, 42, 51, 149, 249].to_vec();
            d.append(&mut self.try_to_vec().unwrap());
            d
        }
    }
    impl anchor_lang::Discriminator for NewUserRecord {
        const DISCRIMINATOR: [u8; 8] = [236, 186, 113, 219, 42, 51, 149, 249];
    }
}
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
