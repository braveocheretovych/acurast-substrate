extern crate core;

#[cfg(test)]
mod tests {
	use cumulus_primitives_core::ParaId;
	use frame_support::{
		assert_ok,
		traits::{GenesisBuild, Hooks},
	};
	use hex_literal::hex;
	use sp_runtime::{
		traits::{AccountIdConversion, ConstU32},
		AccountId32, BoundedVec,
	};
	use xcm_simulator::{decl_test_parachain, TestExt};

	use acurast_runtime::{
		pallet_acurast::Schedule,
		pallet_acurast_marketplace::{
			Advertisement, FeeManager, JobRequirements, Pricing, SchedulingWindow,
		},
		AccountId, Balance, FeeManagement,
	};
	// parent re-exports
	use super::polkadot_ext;
	use emulations::{
		emulators::{
			xcm_simulator,
			xcm_simulator::{decl_test_network, decl_test_relay_chain},
		},
		runtimes::{
			acurast_runtime,
			acurast_runtime::{
				pallet_acurast, pallet_acurast_marketplace,
				pallet_acurast_marketplace::ExecutionOperationHash,
			},
			polkadot_runtime,
		},
	};

	mod jobs;

	decl_test_relay_chain! {
		pub struct PolkadotRelay {
			Runtime = polkadot_runtime::Runtime,
			XcmConfig = polkadot_runtime::xcm_config::XcmConfig,
			new_ext = polkadot_ext(),
		}
	}

	decl_test_parachain! {
		pub struct AcurastParachain {
			Runtime = acurast_runtime::Runtime,
			XcmpMessageHandler = acurast_runtime::XcmpQueue,
			DmpMessageHandler = acurast_runtime::DmpQueue,
			new_ext = acurast_ext(ACURAST_CHAIN_ID),
		}
	}

	decl_test_network! {
		pub struct Network {
			relay_chain = PolkadotRelay,
			parachains = vec![
				(2001, AcurastParachain),
			],
		}
	}

	// make this match parachains in decl_test_network!
	pub const ACURAST_CHAIN_ID: u32 = 2001;

	pub const ALICE: AccountId32 = AccountId32::new([4u8; 32]);
	pub const BOB: AccountId32 = AccountId32::new([8u8; 32]);
	pub const FERDIE: AccountId32 = AccountId32::new([5u8; 32]);

	pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

	pub fn acurast_ext(para_id: u32) -> sp_io::TestExternalities {
		use acurast_runtime::{Runtime, System};

		let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		let parachain_info_config =
			parachain_info::GenesisConfig { parachain_id: ParaId::from(para_id) };

		<parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(
			&parachain_info_config,
			&mut t,
		)
		.unwrap();

		let pallet_assets_account: <Runtime as frame_system::Config>::AccountId =
			<Runtime as acurast_runtime::pallet_acurast::Config>::PalletId::get()
				.into_account_truncating();

		let fee_manager_account: <Runtime as frame_system::Config>::AccountId =
			acurast_runtime::FeeManagerPalletId::get().into_account_truncating();

		pallet_balances::GenesisConfig::<Runtime> {
			balances: vec![
				(ALICE, INITIAL_BALANCE),
				(BOB, INITIAL_BALANCE),
				(FERDIE, INITIAL_BALANCE),
				(pallet_assets_account, INITIAL_BALANCE),
				(fee_manager_account, INITIAL_BALANCE),
			],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let pallet_xcm_config = pallet_xcm::GenesisConfig::default();
		<pallet_xcm::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(
			&pallet_xcm_config,
			&mut t,
		)
		.unwrap();

		acurast_runtime::pallet_acurast::GenesisConfig::<Runtime> {
			attestations: vec![(BOB, None)],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		acurast_runtime::pallet_acurast_processor_manager::GenesisConfig::<Runtime> {
			managers: vec![(ALICE, vec![BOB])],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}

	type Acurast = pallet_acurast::Pallet<acurast_runtime::Runtime>;
	type AcurastMarketplace = pallet_acurast_marketplace::Pallet<acurast_runtime::Runtime>;
	type AcurastBalances = pallet_balances::Pallet<acurast_runtime::Runtime>;

	/// Type representing the utf8 bytes of a string containing the value of an ipfs url.
	/// The ipfs url is expected to point to a script.
	pub type Script = BoundedVec<u8, ConstU32<53>>;

	const SCRIPT_BYTES: [u8; 53] = hex!("697066733A2F2F00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
	pub const OPERATION_HASH: [u8; 32] =
		hex!("a3f18e4c6f0cdd0d8666f407610351cacb9a263678cf058294be9977b69f2cb3");

	pub fn script() -> Script {
		SCRIPT_BYTES.to_vec().try_into().unwrap()
	}

	pub fn operation_hash() -> ExecutionOperationHash {
		OPERATION_HASH.to_vec().try_into().unwrap()
	}

	pub fn advertisement(
		fee_per_millisecond: u128,
		fee_per_storage_byte: u128,
		storage_capacity: u32,
		max_memory: u32,
		network_request_quota: u8,
		scheduling_window: SchedulingWindow,
	) -> Advertisement<AccountId, Balance, pallet_acurast::CU32<100>> {
		Advertisement {
			pricing: Pricing {
				fee_per_millisecond,
				fee_per_storage_byte,
				base_fee_per_execution: 0,
				scheduling_window,
			},
			allowed_consumers: None,
			storage_capacity,
			max_memory,
			network_request_quota,
			available_modules: vec![].try_into().unwrap(),
		}
	}
}

// add arg paras: Vec<u32>
pub fn polkadot_ext() -> sp_io::TestExternalities {
	use emulations::runtimes::polkadot_runtime::{Runtime, System};

	let t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}
