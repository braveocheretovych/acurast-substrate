
//! Autogenerated weights for `pallet_acurast_hyperdrive`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `acurast-benchmark`, CPU: `AMD EPYC 7B13`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acurast-kusama"), DB CACHE: 1024

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

/// Weight functions for `pallet_acurast_hyperdrive`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_acurast_hyperdrive::WeightInfo for WeightInfo<T> {
	/// Storage: AcurastHyperdriveTezos CurrentAlephZeroContract (r:0 w:1)
	/// Proof: AcurastHyperdriveTezos CurrentAlephZeroContract (max_values: Some(1), max_size: Some(66), added: 561, mode: MaxEncodedLen)
	fn update_aleph_zero_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_710_000 picoseconds.
		Weight::from_parts(14_410_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
