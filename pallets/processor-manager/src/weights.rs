
//! Autogenerated weights for `pallet_acurast_processor_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `jenova`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acurast-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/acurast-node
// benchmark
// pallet
// --chain=acurast-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_acurast_processor_manager
// --extrinsic
// *
// --steps=50
// --repeat=20
// --output=./benchmarks/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_acurast_processor_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	/// Storage: Uniques Account (r:1 w:1)
	/// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager LastManagerId (r:1 w:1)
	/// Proof: AcurastProcessorManager LastManagerId (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ManagerCounter (r:1 w:1)
	/// Proof: AcurastProcessorManager ManagerCounter (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ProcessorToManagerIdIndex (r:20 w:20)
	/// Proof: AcurastProcessorManager ProcessorToManagerIdIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ManagedProcessors (r:0 w:20)
	/// Proof: AcurastProcessorManager ManagedProcessors (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `x` is `[1, 20]`.
	fn update_processor_pairings(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1994 + x * (12 ±0)`
		//  Estimated: `21817 + x * (2507 ±0)`
		// Minimum execution time: 63_000_000 picoseconds.
		Weight::from_parts(55_087_973, 0)
			.saturating_add(Weight::from_parts(0, 21817))
			// Standard Error: 10_940
			.saturating_add(Weight::from_parts(9_526_349, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2507).saturating_mul(x.into()))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:1 w:1)
	/// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager LastManagerId (r:1 w:1)
	/// Proof: AcurastProcessorManager LastManagerId (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ManagerCounter (r:1 w:1)
	/// Proof: AcurastProcessorManager ManagerCounter (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ProcessorToManagerIdIndex (r:1 w:1)
	/// Proof: AcurastProcessorManager ProcessorToManagerIdIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ManagedProcessors (r:0 w:1)
	/// Proof: AcurastProcessorManager ManagedProcessors (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn pair_with_manager() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1903`
		//  Estimated: `24324`
		// Minimum execution time: 60_000_000 picoseconds.
		Weight::from_parts(61_000_000, 0)
			.saturating_add(Weight::from_parts(0, 24324))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Uniques Account (r:1 w:0)
	/// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ProcessorToManagerIdIndex (r:1 w:0)
	/// Proof: AcurastProcessorManager ProcessorToManagerIdIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:3 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:0)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn recover_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1982`
		//  Estimated: `22263`
		// Minimum execution time: 64_000_000 picoseconds.
		Weight::from_parts(65_000_000, 0)
			.saturating_add(Weight::from_parts(0, 22263))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: AcurastProcessorManager ProcessorToManagerIdIndex (r:1 w:0)
	/// Proof: AcurastProcessorManager ProcessorToManagerIdIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ProcessorHeartbeat (r:0 w:1)
	/// Proof: AcurastProcessorManager ProcessorHeartbeat (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	fn heartbeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `684`
		//  Estimated: `4990`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4990))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: AcurastProcessorManager ProcessorToManagerIdIndex (r:1 w:0)
	/// Proof: AcurastProcessorManager ProcessorToManagerIdIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ProcessorHeartbeat (r:0 w:1)
	/// Proof: AcurastProcessorManager ProcessorHeartbeat (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	fn heartbeat_with_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `684`
		//  Estimated: `4990`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4990))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Account (r:1 w:0)
	/// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: AcurastProcessorManager ProcessorToManagerIdIndex (r:1 w:0)
	/// Proof: AcurastProcessorManager ProcessorToManagerIdIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: AcurastMarketplace StoredAdvertisementRestriction (r:1 w:1)
	/// Proof: AcurastMarketplace StoredAdvertisementRestriction (max_values: None, max_size: Some(3830), added: 6305, mode: MaxEncodedLen)
	/// Storage: AcurastMarketplace StoredReputation (r:1 w:1)
	/// Proof: AcurastMarketplace StoredReputation (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: AcurastMarketplace StoredStorageCapacity (r:0 w:1)
	/// Proof: AcurastMarketplace StoredStorageCapacity (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: AcurastMarketplace StoredAdvertisementPricing (r:0 w:1)
	/// Proof: AcurastMarketplace StoredAdvertisementPricing (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn advertise_for() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1263`
		//  Estimated: `17914`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 17914))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}

	fn update_binary_hash() -> Weight {
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4990))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}

	fn update_api_version() -> Weight {
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4990))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}

	fn set_processor_update_info(x: u32, ) -> Weight {
		Weight::from_parts(55_087_973, 0)
			.saturating_add(Weight::from_parts(0, 21817))
			// Standard Error: 10_940
			.saturating_add(Weight::from_parts(9_526_349, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2507).saturating_mul(x.into()))
	}

	fn update_reward_distribution_settings() -> Weight {
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4990))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
