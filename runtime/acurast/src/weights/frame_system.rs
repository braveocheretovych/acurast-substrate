
//! Autogenerated weights for `frame_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `dhcp-19-130.guest.tezos.foundation`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acurast-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/acurast-node
// benchmark
// pallet
// --chain=acurast-dev
// --steps=50
// --repeat=20
// --pallet=frame_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./runtime/acurast/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(_b: u32, ) -> Weight {
		Weight::from_ref_time(2_101_000 as u64)
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(b as u64))
	}
	// Storage: System Digest (r:1 w:1)
	// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
	fn set_heap_pages() -> Weight {
		Weight::from_ref_time(7_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[1, 1000]`.
	fn set_storage(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(604_000 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[1, 1000]`.
	fn kill_storage(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(503_000 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `p` is `[1, 1000]`.
	fn kill_prefix(p: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_163_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
}
