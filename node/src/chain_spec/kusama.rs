use acurast_runtime_common::types::{AccountId, AuraId};
use cumulus_primitives_core::ParaId;
use sc_service::ChainType;
use sc_telemetry::serde_json;
use sp_runtime::{app_crypto::Ss58Codec, traits::AccountIdConversion, AccountId32};
use std::str::FromStr;

use super::{accountid_from_str, ChainSpec, Extensions, KUSAMA_PARACHAIN_ID, SS58_FORMAT};
pub(crate) use acurast_kusama_runtime::{self as acurast_runtime, EXISTENTIAL_DEPOSIT};

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

const NATIVE_MIN_BALANCE: u128 = 1_000_000_000_000;
const NATIVE_TOKEN_SYMBOL: &str = "cACU";
const NATIVE_TOKEN_DECIMALS: u8 = 12;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn acurast_session_keys(keys: AuraId) -> acurast_runtime::SessionKeys {
	acurast_runtime::SessionKeys { aura: keys }
}

/// Returns the kusama [ChainSpec].
pub fn acurast_kusama_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), NATIVE_TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), NATIVE_TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS58_FORMAT.into());

	ChainSpec::builder(
		acurast_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "kusama".to_string(), para_id: KUSAMA_PARACHAIN_ID },
	)
	.with_name("Acurast Canary")
	.with_id("acurast-kusama")
	.with_chain_type(ChainType::Live)
	.with_properties(properties)
	.with_genesis_config_patch(genesis_config(
		vec![
			(
				AccountId32::from_str("5GsS2ABbr46mMNRiikVB28SL7Uixv5rnGPzQQJNwXVjnDmBh").unwrap(),
				AuraId::from_string("5GsS2ABbr46mMNRiikVB28SL7Uixv5rnGPzQQJNwXVjnDmBh").unwrap(),
			),
			(
				AccountId32::from_str("5HWM3CmrNvXTKCaZ53xXuxBtHCMHbXXR8fhaL1QeVMaVdGSw").unwrap(),
				AuraId::from_string("5HWM3CmrNvXTKCaZ53xXuxBtHCMHbXXR8fhaL1QeVMaVdGSw").unwrap(),
			),
			(
				AccountId32::from_str("5F7hAMcLn4TKku3jYK9orGCB76GujbMPXN8XAYaAbWwNf8JH").unwrap(),
				AuraId::from_string("5F7hAMcLn4TKku3jYK9orGCB76GujbMPXN8XAYaAbWwNf8JH").unwrap(),
			),
			(
				AccountId32::from_str("5GxSMqLQbWNuGTV6roRJbLR4Ysft7isphR4h7Z75g11fMSeh").unwrap(),
				AuraId::from_string("5GxSMqLQbWNuGTV6roRJbLR4Ysft7isphR4h7Z75g11fMSeh").unwrap(),
			),
		],
		vec![
			(acurast_pallet_account(), NATIVE_MIN_BALANCE),
			(fee_manager_pallet_account(), NATIVE_MIN_BALANCE),
			(acurast_sudo_account(), NATIVE_MIN_BALANCE * 1_000),
		],
		KUSAMA_PARACHAIN_ID.into(),
		acurast_sudo_account(),
	))
	.build()
}

fn genesis_config(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, acurast_runtime::Balance)>,
	id: ParaId,
	sudo_account: AccountId,
) -> serde_json::Value {
	serde_json::json!({
		"balances": {
			"balances": endowed_accounts,
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"collatorSelection": {
			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
		},
		"session": {
			"keys": invulnerables
				.into_iter()
				.map(|(acc, session_keys)| {
					(
						acc.clone(),                        // account id
						acc,                                // validator id
						acurast_session_keys(session_keys), // session keys
					)
				})
				.collect::<Vec<_>>(),
		},
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
		"sudo": {
			"key": Some(sudo_account)
		}
	})
}

/// Returns the pallet_acurast account id.
pub fn acurast_pallet_account() -> AccountId {
	acurast_runtime::AcurastPalletId::get().into_account_truncating()
}

/// Returns the pallet_fee_manager account id.
pub fn fee_manager_pallet_account() -> AccountId {
	acurast_runtime::FeeManagerPalletId::get().into_account_truncating()
}

/// returns the root account id.
pub fn acurast_sudo_account() -> AccountId {
	accountid_from_str("5CMcG3yoHxH6e4RqyZHx2QiTsZz4tTiHLXFQ5SmLiXKGcqgv")
}
