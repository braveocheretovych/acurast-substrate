#![cfg_attr(not(feature = "std"), no_std)]

use acurast_p256_crypto::MultiSignature;
pub use pallet_acurast;
pub use pallet_acurast_assets_manager;
pub use parachains_common::Balance;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::H256;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
};
pub use sp_runtime::{MultiAddress, Perbill, Permill};
#[cfg(not(feature = "std"))]
use sp_std::alloc::string;

pub mod constants;
pub mod weights;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

// the base number of indivisible units for balances
pub const PICOUNIT: Balance = 1;
pub const NANOUNIT: Balance = 1_000;
pub const MICROUNIT: Balance = 1_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const UNIT: Balance = 1_000_000_000_000;
pub const KILOUNIT: Balance = 1_000_000_000_000_000;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	use super::*;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}
