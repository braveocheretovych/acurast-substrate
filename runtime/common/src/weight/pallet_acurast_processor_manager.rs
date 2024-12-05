
//! Autogenerated weights for `pallet_acurast_processor_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2024-12-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `acurast-benchmark`, CPU: `AMD EPYC 7B13`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("acurast-kusama")`, DB CACHE: 1024

// Executed Command:
// /acurast-node
// benchmark
// pallet
// --chain=acurast-kusama
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps=50
// --repeat=20
// --output=/benchmarks/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_acurast_processor_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_acurast_processor_manager::WeightInfo for WeightInfo<T> {
	/// Storage: `Uniques::Account` (r:1 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::LastManagerId` (r:1 w:1)
	/// Proof: `AcurastProcessorManager::LastManagerId` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	/// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ManagerCounter` (r:1 w:1)
	/// Proof: `AcurastProcessorManager::ManagerCounter` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorToManagerIdIndex` (r:20 w:20)
	/// Proof: `AcurastProcessorManager::ProcessorToManagerIdIndex` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ClassAccount` (r:0 w:1)
	/// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ManagedProcessors` (r:0 w:20)
	/// Proof: `AcurastProcessorManager::ManagedProcessors` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 20]`.
	fn update_processor_pairings(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `249`
		//  Estimated: `3655 + x * (2507 ±0)`
		// Minimum execution time: 72_660_000 picoseconds.
		Weight::from_parts(65_642_571, 0)
			.saturating_add(Weight::from_parts(0, 3655))
			// Standard Error: 15_881
			.saturating_add(Weight::from_parts(10_635_244, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2507).saturating_mul(x.into()))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:1 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::LastManagerId` (r:1 w:1)
	/// Proof: `AcurastProcessorManager::LastManagerId` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	/// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ManagerCounter` (r:1 w:1)
	/// Proof: `AcurastProcessorManager::ManagerCounter` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorToManagerIdIndex` (r:1 w:1)
	/// Proof: `AcurastProcessorManager::ProcessorToManagerIdIndex` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ClassAccount` (r:0 w:1)
	/// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ManagedProcessors` (r:0 w:1)
	/// Proof: `AcurastProcessorManager::ManagedProcessors` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn pair_with_manager() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `249`
		//  Estimated: `3655`
		// Minimum execution time: 71_280_000 picoseconds.
		Weight::from_parts(73_900_000, 0)
			.saturating_add(Weight::from_parts(0, 3655))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `AcurastProcessorManager::ProcessorToManagerIdIndex` (r:1 w:0)
	/// Proof: `AcurastProcessorManager::ProcessorToManagerIdIndex` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:0)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn recover_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `3611`
		// Minimum execution time: 27_410_000 picoseconds.
		Weight::from_parts(28_090_000, 0)
			.saturating_add(Weight::from_parts(0, 3611))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: `AcurastProcessorManager::ProcessorToManagerIdIndex` (r:1 w:0)
	/// Proof: `AcurastProcessorManager::ProcessorToManagerIdIndex` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorHeartbeat` (r:0 w:1)
	/// Proof: `AcurastProcessorManager::ProcessorHeartbeat` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn heartbeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `254`
		//  Estimated: `3497`
		// Minimum execution time: 22_610_000 picoseconds.
		Weight::from_parts(23_420_000, 0)
			.saturating_add(Weight::from_parts(0, 3497))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AcurastProcessorManager::ProcessorToManagerIdIndex` (r:1 w:0)
	/// Proof: `AcurastProcessorManager::ProcessorToManagerIdIndex` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:0)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `AcurastMarketplace::StoredAdvertisementRestriction` (r:1 w:1)
	/// Proof: `AcurastMarketplace::StoredAdvertisementRestriction` (`max_values`: None, `max_size`: Some(3830), added: 6305, mode: `MaxEncodedLen`)
	/// Storage: `AcurastMarketplace::StoredReputation` (r:1 w:1)
	/// Proof: `AcurastMarketplace::StoredReputation` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `AcurastMarketplace::StoredStorageCapacity` (r:0 w:1)
	/// Proof: `AcurastMarketplace::StoredStorageCapacity` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `AcurastMarketplace::StoredAdvertisementPricing` (r:0 w:1)
	/// Proof: `AcurastMarketplace::StoredAdvertisementPricing` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn advertise_for() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `520`
		//  Estimated: `7295`
		// Minimum execution time: 37_940_000 picoseconds.
		Weight::from_parts(39_560_000, 0)
			.saturating_add(Weight::from_parts(0, 7295))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AcurastProcessorManager::ProcessorToManagerIdIndex` (r:1 w:0)
	/// Proof: `AcurastProcessorManager::ProcessorToManagerIdIndex` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Acurast::StoredAttestation` (r:1 w:0)
	/// Proof: `Acurast::StoredAttestation` (`max_values`: None, `max_size`: Some(11623), added: 14098, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorMinVersionForReward` (r:1 w:0)
	/// Proof: `AcurastProcessorManager::ProcessorMinVersionForReward` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorRewardDistributionSettings` (r:1 w:0)
	/// Proof: `AcurastProcessorManager::ProcessorRewardDistributionSettings` (`max_values`: Some(1), `max_size`: Some(60), added: 555, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:0)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorRewardDistributionWindow` (r:1 w:1)
	/// Proof: `AcurastProcessorManager::ProcessorRewardDistributionWindow` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorVersion` (r:0 w:1)
	/// Proof: `AcurastProcessorManager::ProcessorVersion` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorHeartbeat` (r:0 w:1)
	/// Proof: `AcurastProcessorManager::ProcessorHeartbeat` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn heartbeat_with_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1073`
		//  Estimated: `15088`
		// Minimum execution time: 113_830_000 picoseconds.
		Weight::from_parts(117_250_000, 0)
			.saturating_add(Weight::from_parts(0, 15088))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AcurastProcessorManager::KnownBinaryHash` (r:0 w:1)
	/// Proof: `AcurastProcessorManager::KnownBinaryHash` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn update_binary_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_710_000 picoseconds.
		Weight::from_parts(11_420_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AcurastProcessorManager::ApiVersion` (r:0 w:1)
	/// Proof: `AcurastProcessorManager::ApiVersion` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn update_api_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_460_000 picoseconds.
		Weight::from_parts(7_930_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AcurastProcessorManager::KnownBinaryHash` (r:1 w:0)
	/// Proof: `AcurastProcessorManager::KnownBinaryHash` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorToManagerIdIndex` (r:100 w:0)
	/// Proof: `AcurastProcessorManager::ProcessorToManagerIdIndex` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:0)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `AcurastProcessorManager::ProcessorUpdateInfo` (r:0 w:100)
	/// Proof: `AcurastProcessorManager::ProcessorUpdateInfo` (`max_values`: None, `max_size`: Some(258), added: 2733, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 100]`.
	fn set_processor_update_info(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `529 + x * (36 ±0)`
		//  Estimated: `3611 + x * (2507 ±0)`
		// Minimum execution time: 32_380_000 picoseconds.
		Weight::from_parts(23_427_627, 0)
			.saturating_add(Weight::from_parts(0, 3611))
			// Standard Error: 7_639
			.saturating_add(Weight::from_parts(8_763_595, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2507).saturating_mul(x.into()))
	}
	/// Storage: `AcurastProcessorManager::ProcessorRewardDistributionSettings` (r:0 w:1)
	/// Proof: `AcurastProcessorManager::ProcessorRewardDistributionSettings` (`max_values`: Some(1), `max_size`: Some(60), added: 555, mode: `MaxEncodedLen`)
	fn update_reward_distribution_settings() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_880_000 picoseconds.
		Weight::from_parts(4_200_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AcurastProcessorManager::ProcessorMinVersionForReward` (r:0 w:1)
	/// Proof: `AcurastProcessorManager::ProcessorMinVersionForReward` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn update_min_processor_version_for_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_430_000 picoseconds.
		Weight::from_parts(10_140_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
