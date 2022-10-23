// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Polkadot chain configurations.

use beefy_primitives::crypto::AuthorityId as BeefyId;
use frame_support::weights::Weight;
use grandpa::AuthorityId as GrandpaId;
#[cfg(feature = "kusama-native")]
use kusama_runtime as kusama;
#[cfg(feature = "kusama-native")]
use kusama_runtime_constants::currency::UNITS as KSM;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::Forcing;
use polkadot_primitives::v2::{AccountId, AccountPublic, AssignmentId, ValidatorId};
#[cfg(feature = "polkadot-native")]
use polkadot_runtime as polkadot;
#[cfg(feature = "polkadot-native")]
use polkadot_runtime_constants::currency::UNITS as DOT;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;

#[cfg(feature = "rococo-native")]
use rococo_runtime as rococo;
#[cfg(feature = "rococo-native")]
use rococo_runtime_constants::currency::UNITS as ROC;
use sc_chain_spec::{ChainSpecExtension, ChainType};
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{traits::IdentifyAccount, Perbill};
use telemetry::TelemetryEndpoints;
#[cfg(feature = "westend-native")]
use westend_runtime as westend;
#[cfg(feature = "westend-native")]
use westend_runtime_constants::currency::UNITS as WND;

#[cfg(feature = "westend-native")]
const WESTEND_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
#[cfg(feature = "rococo-native")]
const ROCOCO_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
#[cfg(feature = "rococo-native")]
const VERSI_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "dot";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<polkadot_primitives::v2::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<polkadot_primitives::v2::Block>,
	/// The light sync state.
	///
	/// This value will be set by the `sync-state rpc` implementation.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// The `ChainSpec` parameterized for the polkadot runtime.
#[cfg(feature = "polkadot-native")]
pub type PolkadotChainSpec = service::GenericChainSpec<polkadot::GenesisConfig, Extensions>;

// Dummy chain spec, in case when we don't have the native runtime.
pub type DummyChainSpec = service::GenericChainSpec<(), Extensions>;

// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "polkadot-native"))]
pub type PolkadotChainSpec = DummyChainSpec;

/// The `ChainSpec` parameterized for the kusama runtime.
#[cfg(feature = "kusama-native")]
pub type KusamaChainSpec = service::GenericChainSpec<kusama::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the kusama runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "kusama-native"))]
pub type KusamaChainSpec = DummyChainSpec;

/// The `ChainSpec` parameterized for the westend runtime.
#[cfg(feature = "westend-native")]
pub type WestendChainSpec = service::GenericChainSpec<westend::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the westend runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "westend-native"))]
pub type WestendChainSpec = DummyChainSpec;

/// The `ChainSpec` parameterized for the rococo runtime.
#[cfg(feature = "rococo-native")]
pub type RococoChainSpec = service::GenericChainSpec<RococoGenesisExt, Extensions>;

/// The `ChainSpec` parameterized for the `versi` runtime.
///
/// As of now `Versi` will just be a clone of `Rococo`, until we need it to differ.
pub type VersiChainSpec = RococoChainSpec;

/// The `ChainSpec` parameterized for the rococo runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "rococo-native"))]
pub type RococoChainSpec = DummyChainSpec;

/// Extension for the Rococo genesis config to support a custom changes to the genesis state.
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg(feature = "rococo-native")]
pub struct RococoGenesisExt {
	/// The runtime genesis config.
	runtime_genesis_config: rococo::GenesisConfig,
	/// The session length in blocks.
	///
	/// If `None` is supplied, the default value is used.
	session_length_in_blocks: Option<u32>,
}

#[cfg(feature = "rococo-native")]
impl sp_runtime::BuildStorage for RococoGenesisExt {
	fn assimilate_storage(&self, storage: &mut sp_core::storage::Storage) -> Result<(), String> {
		sp_state_machine::BasicExternalities::execute_with_storage(storage, || {
			if let Some(length) = self.session_length_in_blocks.as_ref() {
				rococo_runtime_constants::time::EpochDurationInBlocks::set(length);
			}
		});
		self.runtime_genesis_config.assimilate_storage(storage)
	}
}

pub fn polkadot_config() -> Result<PolkadotChainSpec, String> {
	PolkadotChainSpec::from_json_bytes(&include_bytes!("../chain-specs/polkadot.json")[..])
}

pub fn kusama_config() -> Result<KusamaChainSpec, String> {
	KusamaChainSpec::from_json_bytes(&include_bytes!("../chain-specs/kusama.json")[..])
}

pub fn westend_config() -> Result<WestendChainSpec, String> {
	WestendChainSpec::from_json_bytes(&include_bytes!("../chain-specs/westend.json")[..])
}

pub fn rococo_config() -> Result<RococoChainSpec, String> {
	RococoChainSpec::from_json_bytes(&include_bytes!("../chain-specs/rococo.json")[..])
}

/// This is a temporary testnet that uses the same runtime as rococo.
pub fn wococo_config() -> Result<RococoChainSpec, String> {
	RococoChainSpec::from_json_bytes(&include_bytes!("../chain-specs/wococo.json")[..])
}

/// The default parachains host configuration.
#[cfg(any(
	feature = "rococo-native",
	feature = "kusama-native",
	feature = "westend-native",
	feature = "polkadot-native"
))]
fn default_parachains_host_configuration(
) -> polkadot_runtime_parachains::configuration::HostConfiguration<
	polkadot_primitives::v2::BlockNumber,
> {
	use polkadot_primitives::v2::{MAX_CODE_SIZE, MAX_POV_SIZE};

	polkadot_runtime_parachains::configuration::HostConfiguration {
		validation_upgrade_cooldown: 2u32,
		validation_upgrade_delay: 2,
		code_retention_period: 1200,
		max_code_size: MAX_CODE_SIZE,
		max_pov_size: MAX_POV_SIZE,
		max_head_data_size: 32 * 1024,
		group_rotation_frequency: 20,
		chain_availability_period: 4,
		thread_availability_period: 4,
		max_upward_queue_count: 8,
		max_upward_queue_size: 1024 * 1024,
		max_downward_message_size: 1024 * 1024,
		ump_service_total_weight: Weight::from_ref_time(100_000_000_000)
			.set_proof_size(MAX_POV_SIZE as u64),
		max_upward_message_size: 50 * 1024,
		max_upward_message_num_per_candidate: 5,
		hrmp_sender_deposit: 0,
		hrmp_recipient_deposit: 0,
		hrmp_channel_max_capacity: 8,
		hrmp_channel_max_total_size: 8 * 1024,
		hrmp_max_parachain_inbound_channels: 4,
		hrmp_max_parathread_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_parachain_outbound_channels: 4,
		hrmp_max_parathread_outbound_channels: 4,
		hrmp_max_message_num_per_candidate: 5,
		dispute_period: 6,
		no_show_slots: 2,
		n_delay_tranches: 25,
		needed_approvals: 2,
		relay_vrf_modulo_samples: 2,
		zeroth_delay_tranche_width: 0,
		minimum_validation_upgrade_delay: 5,
		..Default::default()
	}
}

#[cfg(any(
	feature = "rococo-native",
	feature = "kusama-native",
	feature = "westend-native",
	feature = "polkadot-native"
))]
#[test]
fn default_parachains_host_configuration_is_consistent() {
	default_parachains_host_configuration().panic_if_not_consistent();
}

#[cfg(feature = "polkadot-native")]
fn polkadot_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> polkadot::SessionKeys {
	polkadot::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "kusama-native")]
fn kusama_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> kusama::SessionKeys {
	kusama::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "westend-native")]
fn westend_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> westend::SessionKeys {
	westend::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "rococo-native")]
fn rococo_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
	beefy: BeefyId,
) -> rococo_runtime::SessionKeys {
	rococo_runtime::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
		beefy,
	}
}

#[cfg(feature = "polkadot-native")]
fn polkadot_staging_testnet_config_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;
	let endowed_accounts = vec![
		// v1
		hex!["087ff8cccf3d6f89a35dc11cf655c166a68611e99bcf3503e69ae9d4cb18b07c"].into(),
		hex!["7afb2a4a1d8ef7bb20b733919dd0bdfc5aef0034cc9f1effd42ae8e3ca144629"].into(),
		// v2
		hex!["347c9f9cd22bc929b749be6606d637908c8fb430eea1bc62ecd6db9b22c8ec0a"].into(),
		hex!["9888049cef2f2b7edea8cedcc46bd7a387a6a7adb5c3ed516710226c1560715f"].into(),
		// v3
		hex!["488ed7b179c6430e5becb5edf65cbc56376aecb6a49f5b13ba9987bf07f07c33"].into(),
		hex!["082e441456ca3350883a922856c9cda9a7638226dcd6132ffb6859aaca51b26b"].into(),
		// v4
		hex!["68f5a90fdf1932004a4c34bc4f32e8cb6068fc64d6e3a300817397a7d38f7e48"].into(),
		hex!["e43f9da17409d47378a3324c1107681a17abf0a8a9ef10329de32e310e34283a"].into(),
		// v5
		hex!["c4065fdb164a5dfda6ed4664ab947e17285d70f5e96e0ad6c134cbd50dfa652b"].into(),
		hex!["24c8d06f7e6ace74a00e469153daf48aa3bb8f4929d9dbfaf44df2d2d8bef77a"].into(),
		// v6
		hex!["2a7615ff7c8b09bb17d293d9b1c46b5dcda6ce7e6b864a740e6dc1b34402ea55"].into(),
		hex!["a23672b15485355fb6c74a1e8175c275a1ae9f6d899c691e20e10dfab34d7054"].into(),
		// v7
		hex!["ec583517a759c7de3488f29659027d598645db48b7c1b3273783e6a69465e043"].into(),
		hex!["f26e02d34ab82589868e9168d618c3e3a6f8f1f02f060c4120e0a57dc92ea72f"].into(),
		// v8
		hex!["827332176cfab08ad737b0b245c2c5803493e717e9f4a6a67ab0715a5696d312"].into(),
		hex!["b0c5c8b7055108bc09f6d91982c1db6820aa04d15ca010481a9f2bd3ce8f4867"].into(),
	];

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// v1-1
			hex!["087ff8cccf3d6f89a35dc11cf655c166a68611e99bcf3503e69ae9d4cb18b07c"].into(),
			hex!["087ff8cccf3d6f89a35dc11cf655c166a68611e99bcf3503e69ae9d4cb18b07c"].into(),
			hex!["087ff8cccf3d6f89a35dc11cf655c166a68611e99bcf3503e69ae9d4cb18b07c"]
				.unchecked_into(),
			hex!["a0433041612825ee64649294fb0cea04e1749dd1b5343e4f576db9f705506829"]
				.unchecked_into(),
			hex!["087ff8cccf3d6f89a35dc11cf655c166a68611e99bcf3503e69ae9d4cb18b07c"]
				.unchecked_into(),
			hex!["087ff8cccf3d6f89a35dc11cf655c166a68611e99bcf3503e69ae9d4cb18b07c"]
				.unchecked_into(),
			hex!["087ff8cccf3d6f89a35dc11cf655c166a68611e99bcf3503e69ae9d4cb18b07c"]
				.unchecked_into(),
			hex!["087ff8cccf3d6f89a35dc11cf655c166a68611e99bcf3503e69ae9d4cb18b07c"]
				.unchecked_into(),
		),
		(
			// v1-2
			hex!["7afb2a4a1d8ef7bb20b733919dd0bdfc5aef0034cc9f1effd42ae8e3ca144629"].into(),
			hex!["7afb2a4a1d8ef7bb20b733919dd0bdfc5aef0034cc9f1effd42ae8e3ca144629"].into(),
			hex!["7afb2a4a1d8ef7bb20b733919dd0bdfc5aef0034cc9f1effd42ae8e3ca144629"]
				.unchecked_into(),
			hex!["d4a81b83fd729bc1bdff881c80df05d2c1c16cb27f2dbe1f618adbc07c892f8d"]
				.unchecked_into(),
			hex!["7afb2a4a1d8ef7bb20b733919dd0bdfc5aef0034cc9f1effd42ae8e3ca144629"]
				.unchecked_into(),
			hex!["7afb2a4a1d8ef7bb20b733919dd0bdfc5aef0034cc9f1effd42ae8e3ca144629"]
				.unchecked_into(),
			hex!["7afb2a4a1d8ef7bb20b733919dd0bdfc5aef0034cc9f1effd42ae8e3ca144629"]
				.unchecked_into(),
			hex!["7afb2a4a1d8ef7bb20b733919dd0bdfc5aef0034cc9f1effd42ae8e3ca144629"]
				.unchecked_into(),
		),
		(
			// v2-1
			hex!["347c9f9cd22bc929b749be6606d637908c8fb430eea1bc62ecd6db9b22c8ec0a"].into(),
			hex!["347c9f9cd22bc929b749be6606d637908c8fb430eea1bc62ecd6db9b22c8ec0a"].into(),
			hex!["347c9f9cd22bc929b749be6606d637908c8fb430eea1bc62ecd6db9b22c8ec0a"]
				.unchecked_into(),
			hex!["77b009a0068754fcd38fe41b31dea89c103a29a53269ab2f35818bc53b51406c"]
				.unchecked_into(),
			hex!["347c9f9cd22bc929b749be6606d637908c8fb430eea1bc62ecd6db9b22c8ec0a"]
				.unchecked_into(),
			hex!["347c9f9cd22bc929b749be6606d637908c8fb430eea1bc62ecd6db9b22c8ec0a"]
				.unchecked_into(),
			hex!["347c9f9cd22bc929b749be6606d637908c8fb430eea1bc62ecd6db9b22c8ec0a"]
				.unchecked_into(),
			hex!["347c9f9cd22bc929b749be6606d637908c8fb430eea1bc62ecd6db9b22c8ec0a"]
				.unchecked_into(),
		),
		(
			// v2-2
			hex!["9888049cef2f2b7edea8cedcc46bd7a387a6a7adb5c3ed516710226c1560715f"].into(),
			hex!["9888049cef2f2b7edea8cedcc46bd7a387a6a7adb5c3ed516710226c1560715f"].into(),
			hex!["9888049cef2f2b7edea8cedcc46bd7a387a6a7adb5c3ed516710226c1560715f"]
				.unchecked_into(),
			hex!["aa294a9647a0e77c2cfbb39e03d09f945cee21e32e5ac59b69c0297ecbb68d09"]
				.unchecked_into(),
			hex!["9888049cef2f2b7edea8cedcc46bd7a387a6a7adb5c3ed516710226c1560715f"]
				.unchecked_into(),
			hex!["9888049cef2f2b7edea8cedcc46bd7a387a6a7adb5c3ed516710226c1560715f"]
				.unchecked_into(),
			hex!["9888049cef2f2b7edea8cedcc46bd7a387a6a7adb5c3ed516710226c1560715f"]
				.unchecked_into(),
			hex!["9888049cef2f2b7edea8cedcc46bd7a387a6a7adb5c3ed516710226c1560715f"]
				.unchecked_into(),
		),
		(
			// v3-1
			hex!["488ed7b179c6430e5becb5edf65cbc56376aecb6a49f5b13ba9987bf07f07c33"].into(),
			hex!["488ed7b179c6430e5becb5edf65cbc56376aecb6a49f5b13ba9987bf07f07c33"].into(),
			hex!["488ed7b179c6430e5becb5edf65cbc56376aecb6a49f5b13ba9987bf07f07c33"]
				.unchecked_into(),
			hex!["64b7bd2b031daf60d712dd6dafdcb941d8defdd1952e0fad11d1cd59a89f008c"]
				.unchecked_into(),
			hex!["488ed7b179c6430e5becb5edf65cbc56376aecb6a49f5b13ba9987bf07f07c33"]
				.unchecked_into(),
			hex!["488ed7b179c6430e5becb5edf65cbc56376aecb6a49f5b13ba9987bf07f07c33"]
				.unchecked_into(),
			hex!["488ed7b179c6430e5becb5edf65cbc56376aecb6a49f5b13ba9987bf07f07c33"]
				.unchecked_into(),
			hex!["488ed7b179c6430e5becb5edf65cbc56376aecb6a49f5b13ba9987bf07f07c33"]
				.unchecked_into(),
		),
		(
			// v3-2
			hex!["082e441456ca3350883a922856c9cda9a7638226dcd6132ffb6859aaca51b26b"].into(),
			hex!["082e441456ca3350883a922856c9cda9a7638226dcd6132ffb6859aaca51b26b"].into(),
			hex!["082e441456ca3350883a922856c9cda9a7638226dcd6132ffb6859aaca51b26b"]
				.unchecked_into(),
			hex!["1478c68963ed006eba4e49b89951dc8024da50c80b68d01e14f18cbc4e5f9908"]
				.unchecked_into(),
			hex!["082e441456ca3350883a922856c9cda9a7638226dcd6132ffb6859aaca51b26b"]
				.unchecked_into(),
			hex!["082e441456ca3350883a922856c9cda9a7638226dcd6132ffb6859aaca51b26b"]
				.unchecked_into(),
			hex!["082e441456ca3350883a922856c9cda9a7638226dcd6132ffb6859aaca51b26b"]
				.unchecked_into(),
			hex!["082e441456ca3350883a922856c9cda9a7638226dcd6132ffb6859aaca51b26b"]
				.unchecked_into(),
		),
		(
			// v4-1
			hex!["68f5a90fdf1932004a4c34bc4f32e8cb6068fc64d6e3a300817397a7d38f7e48"].into(),
			hex!["68f5a90fdf1932004a4c34bc4f32e8cb6068fc64d6e3a300817397a7d38f7e48"].into(),
			hex!["68f5a90fdf1932004a4c34bc4f32e8cb6068fc64d6e3a300817397a7d38f7e48"]
				.unchecked_into(),
			hex!["3994f26539057f0ad5f632dd8db7c72289e8f74d48863a081fb01597e52784c6"]
				.unchecked_into(),
			hex!["68f5a90fdf1932004a4c34bc4f32e8cb6068fc64d6e3a300817397a7d38f7e48"]
				.unchecked_into(),
			hex!["68f5a90fdf1932004a4c34bc4f32e8cb6068fc64d6e3a300817397a7d38f7e48"]
				.unchecked_into(),
			hex!["68f5a90fdf1932004a4c34bc4f32e8cb6068fc64d6e3a300817397a7d38f7e48"]
				.unchecked_into(),
			hex!["68f5a90fdf1932004a4c34bc4f32e8cb6068fc64d6e3a300817397a7d38f7e48"]
				.unchecked_into(),
		),
		(
			// v4-2
			hex!["e43f9da17409d47378a3324c1107681a17abf0a8a9ef10329de32e310e34283a"].into(),
			hex!["e43f9da17409d47378a3324c1107681a17abf0a8a9ef10329de32e310e34283a"].into(),
			hex!["e43f9da17409d47378a3324c1107681a17abf0a8a9ef10329de32e310e34283a"]
				.unchecked_into(),
			hex!["7ffa2b51db9bd90a0d9acbbe7712e8a23acdff4f59f0641484efcf0f1ed52e76"]
				.unchecked_into(),
			hex!["e43f9da17409d47378a3324c1107681a17abf0a8a9ef10329de32e310e34283a"]
				.unchecked_into(),
			hex!["e43f9da17409d47378a3324c1107681a17abf0a8a9ef10329de32e310e34283a"]
				.unchecked_into(),
			hex!["e43f9da17409d47378a3324c1107681a17abf0a8a9ef10329de32e310e34283a"]
				.unchecked_into(),
			hex!["e43f9da17409d47378a3324c1107681a17abf0a8a9ef10329de32e310e34283a"]
				.unchecked_into(),
		),
		(
			// v5-1
			hex!["c4065fdb164a5dfda6ed4664ab947e17285d70f5e96e0ad6c134cbd50dfa652b"].into(),
			hex!["c4065fdb164a5dfda6ed4664ab947e17285d70f5e96e0ad6c134cbd50dfa652b"].into(),
			hex!["c4065fdb164a5dfda6ed4664ab947e17285d70f5e96e0ad6c134cbd50dfa652b"]
				.unchecked_into(),
			hex!["dfc2e8a24120871c6dfa8772a056130e78bd8045ce407668057b86a023d644ab"]
				.unchecked_into(),
			hex!["c4065fdb164a5dfda6ed4664ab947e17285d70f5e96e0ad6c134cbd50dfa652b"]
				.unchecked_into(),
			hex!["c4065fdb164a5dfda6ed4664ab947e17285d70f5e96e0ad6c134cbd50dfa652b"]
				.unchecked_into(),
			hex!["c4065fdb164a5dfda6ed4664ab947e17285d70f5e96e0ad6c134cbd50dfa652b"]
				.unchecked_into(),
			hex!["c4065fdb164a5dfda6ed4664ab947e17285d70f5e96e0ad6c134cbd50dfa652b"]
				.unchecked_into(),
		),
		(
			// v5-2
			hex!["24c8d06f7e6ace74a00e469153daf48aa3bb8f4929d9dbfaf44df2d2d8bef77a"].into(),
			hex!["24c8d06f7e6ace74a00e469153daf48aa3bb8f4929d9dbfaf44df2d2d8bef77a"].into(),
			hex!["24c8d06f7e6ace74a00e469153daf48aa3bb8f4929d9dbfaf44df2d2d8bef77a"]
				.unchecked_into(),
			hex!["867b395b2beb0f9e3994728f5ff692ef709363e30f6eb77abb4cd49be1301013"]
				.unchecked_into(),
			hex!["24c8d06f7e6ace74a00e469153daf48aa3bb8f4929d9dbfaf44df2d2d8bef77a"]
				.unchecked_into(),
			hex!["24c8d06f7e6ace74a00e469153daf48aa3bb8f4929d9dbfaf44df2d2d8bef77a"]
				.unchecked_into(),
			hex!["24c8d06f7e6ace74a00e469153daf48aa3bb8f4929d9dbfaf44df2d2d8bef77a"]
				.unchecked_into(),
			hex!["24c8d06f7e6ace74a00e469153daf48aa3bb8f4929d9dbfaf44df2d2d8bef77a"]
				.unchecked_into(),
		),
		(
			// v6-1
			hex!["2a7615ff7c8b09bb17d293d9b1c46b5dcda6ce7e6b864a740e6dc1b34402ea55"].into(),
			hex!["2a7615ff7c8b09bb17d293d9b1c46b5dcda6ce7e6b864a740e6dc1b34402ea55"].into(),
			hex!["2a7615ff7c8b09bb17d293d9b1c46b5dcda6ce7e6b864a740e6dc1b34402ea55"]
				.unchecked_into(),
			hex!["ce8c0a1350ec919877bcb9aa649005f2faaab5f480a3a8864f7fb0240d0e3025"]
				.unchecked_into(),
			hex!["2a7615ff7c8b09bb17d293d9b1c46b5dcda6ce7e6b864a740e6dc1b34402ea55"]
				.unchecked_into(),
			hex!["2a7615ff7c8b09bb17d293d9b1c46b5dcda6ce7e6b864a740e6dc1b34402ea55"]
				.unchecked_into(),
			hex!["2a7615ff7c8b09bb17d293d9b1c46b5dcda6ce7e6b864a740e6dc1b34402ea55"]
				.unchecked_into(),
			hex!["2a7615ff7c8b09bb17d293d9b1c46b5dcda6ce7e6b864a740e6dc1b34402ea55"]
				.unchecked_into(),
		),
		(
			// v6-2
			hex!["a23672b15485355fb6c74a1e8175c275a1ae9f6d899c691e20e10dfab34d7054"].into(),
			hex!["a23672b15485355fb6c74a1e8175c275a1ae9f6d899c691e20e10dfab34d7054"].into(),
			hex!["a23672b15485355fb6c74a1e8175c275a1ae9f6d899c691e20e10dfab34d7054"]
				.unchecked_into(),
			hex!["87519c9ffa550b31a427bfd3b817f3057dee257f7ed259214c76ee765a7d0c25"]
				.unchecked_into(),
			hex!["a23672b15485355fb6c74a1e8175c275a1ae9f6d899c691e20e10dfab34d7054"]
				.unchecked_into(),
			hex!["a23672b15485355fb6c74a1e8175c275a1ae9f6d899c691e20e10dfab34d7054"]
				.unchecked_into(),
			hex!["a23672b15485355fb6c74a1e8175c275a1ae9f6d899c691e20e10dfab34d7054"]
				.unchecked_into(),
			hex!["a23672b15485355fb6c74a1e8175c275a1ae9f6d899c691e20e10dfab34d7054"]
				.unchecked_into(),
		),
		(
			// v7-1
			hex!["ec583517a759c7de3488f29659027d598645db48b7c1b3273783e6a69465e043"].into(),
			hex!["ec583517a759c7de3488f29659027d598645db48b7c1b3273783e6a69465e043"].into(),
			hex!["ec583517a759c7de3488f29659027d598645db48b7c1b3273783e6a69465e043"]
				.unchecked_into(),
			hex!["bbba0b446a9e6c6faf99dd1cb72891bfecd80fe76e9c40b0e9c23b714eb59a71"]
				.unchecked_into(),
			hex!["ec583517a759c7de3488f29659027d598645db48b7c1b3273783e6a69465e043"]
				.unchecked_into(),
			hex!["ec583517a759c7de3488f29659027d598645db48b7c1b3273783e6a69465e043"]
				.unchecked_into(),
			hex!["ec583517a759c7de3488f29659027d598645db48b7c1b3273783e6a69465e043"]
				.unchecked_into(),
			hex!["ec583517a759c7de3488f29659027d598645db48b7c1b3273783e6a69465e043"]
				.unchecked_into(),
		),
		(
			// v7-2
			hex!["f26e02d34ab82589868e9168d618c3e3a6f8f1f02f060c4120e0a57dc92ea72f"].into(),
			hex!["f26e02d34ab82589868e9168d618c3e3a6f8f1f02f060c4120e0a57dc92ea72f"].into(),
			hex!["f26e02d34ab82589868e9168d618c3e3a6f8f1f02f060c4120e0a57dc92ea72f"]
				.unchecked_into(),
			hex!["73a236761661771b978ad99db0b621dd35e980d372fb0cf39aa4a17489b25274"]
				.unchecked_into(),
			hex!["f26e02d34ab82589868e9168d618c3e3a6f8f1f02f060c4120e0a57dc92ea72f"]
				.unchecked_into(),
			hex!["f26e02d34ab82589868e9168d618c3e3a6f8f1f02f060c4120e0a57dc92ea72f"]
				.unchecked_into(),
			hex!["f26e02d34ab82589868e9168d618c3e3a6f8f1f02f060c4120e0a57dc92ea72f"]
				.unchecked_into(),
			hex!["f26e02d34ab82589868e9168d618c3e3a6f8f1f02f060c4120e0a57dc92ea72f"]
				.unchecked_into(),
		),
		(
			// v8-1
			hex!["827332176cfab08ad737b0b245c2c5803493e717e9f4a6a67ab0715a5696d312"].into(),
			hex!["827332176cfab08ad737b0b245c2c5803493e717e9f4a6a67ab0715a5696d312"].into(),
			hex!["827332176cfab08ad737b0b245c2c5803493e717e9f4a6a67ab0715a5696d312"]
				.unchecked_into(),
			hex!["6a0fd3eea26f74e9c85394f92ca73c9b2de50be34967fbf76d680cdfd91c3e99"]
				.unchecked_into(),
			hex!["827332176cfab08ad737b0b245c2c5803493e717e9f4a6a67ab0715a5696d312"]
				.unchecked_into(),
			hex!["827332176cfab08ad737b0b245c2c5803493e717e9f4a6a67ab0715a5696d312"]
				.unchecked_into(),
			hex!["827332176cfab08ad737b0b245c2c5803493e717e9f4a6a67ab0715a5696d312"]
				.unchecked_into(),
			hex!["827332176cfab08ad737b0b245c2c5803493e717e9f4a6a67ab0715a5696d312"]
				.unchecked_into(),
		),
		(
			// v8-2
			hex!["b0c5c8b7055108bc09f6d91982c1db6820aa04d15ca010481a9f2bd3ce8f4867"].into(),
			hex!["b0c5c8b7055108bc09f6d91982c1db6820aa04d15ca010481a9f2bd3ce8f4867"].into(),
			hex!["b0c5c8b7055108bc09f6d91982c1db6820aa04d15ca010481a9f2bd3ce8f4867"]
				.unchecked_into(),
			hex!["d55e75f8e94bb57878381afa56b16d80b68156567c4dfcb85a36b77fb7c54cf4"]
				.unchecked_into(),
			hex!["b0c5c8b7055108bc09f6d91982c1db6820aa04d15ca010481a9f2bd3ce8f4867"]
				.unchecked_into(),
			hex!["b0c5c8b7055108bc09f6d91982c1db6820aa04d15ca010481a9f2bd3ce8f4867"]
				.unchecked_into(),
			hex!["b0c5c8b7055108bc09f6d91982c1db6820aa04d15ca010481a9f2bd3ce8f4867"]
				.unchecked_into(),
			hex!["b0c5c8b7055108bc09f6d91982c1db6820aa04d15ca010481a9f2bd3ce8f4867"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * DOT;
	// const STASH: u128 = 100 * DOT;

	polkadot::GenesisConfig {
		system: polkadot::SystemConfig { code: wasm_binary.to_vec() },
		balances: polkadot::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				// .chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		indices: polkadot::IndicesConfig { indices: vec![] },
		session: polkadot::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						polkadot_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: polkadot::StakingConfig {
			validator_count: 20,
			minimum_validator_count: 1,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), 500_000 * DOT, polkadot::StakerStatus::Validator)
				})
				.collect(),
			invulnerables: vec![],
			force_era: Forcing::ForceAlways,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: Default::default(),
		council: polkadot::CouncilConfig {
			members: vec![endowed_accounts[0].clone()],
			phantom: Default::default(),
		},
		technical_committee: polkadot::TechnicalCommitteeConfig {
			members: vec![endowed_accounts[0].clone()],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: polkadot::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(polkadot::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: polkadot::AuthorityDiscoveryConfig { keys: vec![] },
		claims: polkadot::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: polkadot::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: polkadot::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		xcm_pallet: Default::default(),
		nomination_pools: Default::default(),
	}
}

#[cfg(feature = "westend-native")]
fn westend_staging_testnet_config_genesis(wasm_binary: &[u8]) -> westend::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5DaVh5WRfazkGaKhx1jUu6hjz7EmRe4dtW6PKeVLim84KLe8
		hex!["42f4a4b3e0a89c835ee696205caa90dd85c8ea1d7364b646328ee919a6b2fc1e"].into(),
	];
	// SECRET='...' ./scripts/prepare-test-net.sh 4
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			//5ERCqy118nnXDai8g4t3MjdX7ZC5PrQzQpe9vwex5cELWqbt
			hex!["681af4f93073484e1acd6b27395d0d258f1a6b158c808846c8fd05ee2435056e"].into(),
			//5GTS114cfQNBgpQULhMaNCPXGds6NokegCnikxDe1vqANhtn
			hex!["c2463372598ebabd21ee5bc33e1d7e77f391d2df29ce2fbe6bed0d13be629a45"].into(),
			//5FhGbceKeH7fuGogcBwd28ZCkAwDGYBADCTeHiYrvx2ztyRd
			hex!["a097bfc6a33499ed843b711f52f523f8a7174f798a9f98620e52f4170dbe2948"]
				.unchecked_into(),
			//5Es7nDkJt2by5qVCCD7PZJdp76KJw1LdRCiNst5S5f4eecnz
			hex!["7bde49dda82c2c9f082b807ef3ceebff96437d67b3e630c584db7a220ecafacf"]
				.unchecked_into(),
			//5D4e8zRjaYzFamqChGPPtu26PcKbKgUrhb7WqcNbKa2RDFUR
			hex!["2c2fb730a7d9138e6d62fcf516f9ecc2d712af3f2f03ca330c9564b8c0c1bb33"]
				.unchecked_into(),
			//5DD3JY5ENkjcgVFbVSgUbZv7WmrnyJ8bxxu56ee6hZFiRdnh
			hex!["3297a8622988cc23dd9c131e3fb8746d49e007f6e58a81d43420cd539e250e4c"]
				.unchecked_into(),
			//5Gpodowhud8FG9xENXR5YwTFbUAWyoEtw7sYFytFsG4z7SU6
			hex!["d2932edf775088bd088dc5a112ad867c24cc95858f77f8a1ab014de8d4f96a3f"]
				.unchecked_into(),
			//5GUMj8tnjL3PJZgXoiWtgLCaMVNHBNeSeTqDsvcxmaVAjKn9
			hex!["c2fb0f74591a00555a292bc4882d3158bafc4c632124cb60681f164ef81bcf72"]
				.unchecked_into(),
		),
		(
			//5HgDCznTkHKUjzPkQoTZGWbvbyqB7sqHDBPDKdF1FyVYM7Er
			hex!["f8418f189f84814fd40cc1b2e90873e72ea789487f3b98ed42811ba76d10fc37"].into(),
			//5GQTryeFwuvgmZ2tH5ZeAKZHRM9ch5WGVGo6ND9P8f9uMsNY
			hex!["c002bb4af4a1bd2f33d104aef8a41878fe1ac94ba007029c4dfdefa8b698d043"].into(),
			//5C7YkWSVH1zrpsE5KwW1ua1qatyphzYxiZrL24mjkxz7mUbn
			hex!["022b14fbcf65a93b81f453105b9892c3fc4aa74c22c53b4abab019e1d58fbd41"]
				.unchecked_into(),
			//5GwFC6Tmg4fhj4PxSqHycgJxi3PDfnC9RGDsNHoRwAvXvpnZ
			hex!["d77cafd3b32c8b52b0e2780a586a6e527c94f1bdec117c4e4acb0a491461ffa3"]
				.unchecked_into(),
			//5DSVrGURuDuh8Luzo8FYq7o2NWiUSLSN6QAVNrj9BtswWH6R
			hex!["3cdb36a5a14715999faffd06c5b9e5dcdc24d4b46bc3e4df1aaad266112a7b49"]
				.unchecked_into(),
			//5DLEG2AupawCXGwhJtrzBRc3zAhuP8V662dDrUTzAsCiB9Ec
			hex!["38134245c9919ecb20bf2eedbe943b69ba92ceb9eb5477b92b0afd3cb6ce2858"]
				.unchecked_into(),
			//5D83o9fDgnHxaKPkSx59hk8zYzqcgzN2mrf7cp8fiVEi7V4E
			hex!["2ec917690dc1d676002e3504c530b2595490aa5a4603d9cc579b9485b8d0d854"]
				.unchecked_into(),
			//5DwBJquZgncRWXFxj2ydbF8LBUPPUbiq86sXWXgm8Z38m8L2
			hex!["52bae9b8dedb8058dda93ec6f57d7e5a517c4c9f002a4636fada70fed0acf376"]
				.unchecked_into(),
		),
		(
			//5DMHpkRpQV7NWJFfn2zQxCLiAKv7R12PWFRPHKKk5X3JkYfP
			hex!["38e280b35d08db46019a210a944e4b7177665232ab679df12d6a8bbb317a2276"].into(),
			//5FbJpSHmFDe5FN3DVGe1R345ZePL9nhcC9V2Cczxo7q8q6rN
			hex!["9c0bc0e2469924d718ae683737f818a47c46b0612376ecca06a2ac059fe1f870"].into(),
			//5E5Pm3Udzxy26KGkLE5pc8JPfQrvkYHiaXWtuEfmQsBSgep9
			hex!["58fecadc2df8182a27e999e7e1fd7c99f8ec18f2a81f9a0db38b3653613f3f4d"]
				.unchecked_into(),
			//5FxcystSLHtaWoy2HEgRNerj9PrUs452B6AvHVnQZm5ZQmqE
			hex!["ac4d0c5e8f8486de05135c10a707f58aa29126d5eb28fdaaba00f9a505f5249d"]
				.unchecked_into(),
			//5E7KqVXaVGuAqiqMigpuH8oXHLVh4tmijmpJABLYANpjMkem
			hex!["5a781385a0235fe8594dd101ec55ef9ba01883f8563a0cdd37b89e0303f6a578"]
				.unchecked_into(),
			//5H9AybjkpyZ79yN5nHuBqs6RKuZPgM7aAVVvTQsDFovgXb2A
			hex!["e09570f62a062450d4406b4eb43e7f775ff954e37606646cd590d1818189501f"]
				.unchecked_into(),
			//5Ccgs7VwJKBawMbwMENDmj2eFAxhFdGksVHdk8aTAf4w7xox
			hex!["1864832dae34df30846d5cc65973f58a2d01b337d094b1284ec3466ecc90251d"]
				.unchecked_into(),
			//5EsSaZZ7niJs7hmAtp4QeK19AcAuTp7WXB7N7gRipVooerq4
			hex!["7c1d92535e6d94e21cffea6633a855a7e3c9684cd2f209e5ddbdeaf5111e395b"]
				.unchecked_into(),
		),
		(
			//5Ea11qhmGRntQ7pyEkEydbwxvfrYwGMKW6rPERU4UiSBB6rd
			hex!["6ed057d2c833c45629de2f14b9f6ce6df1edbf9421b7a638e1fb4828c2bd2651"].into(),
			//5CZomCZwPB78BZMZsCiy7WSpkpHhdrN8QTSyjcK3FFEZHBor
			hex!["1631ff446b3534d031adfc37b7f7aed26d2a6b3938d10496aab3345c54707429"].into(),
			//5CSM6vppouFHzAVPkVFWN76DPRUG7B9qwJe892ccfSfJ8M5f
			hex!["108188c43a7521e1abe737b343341c2179a3a89626c7b017c09a5b10df6f1c42"]
				.unchecked_into(),
			//5GwkG4std9KcjYi3ThSC7QWfhqokmYVvWEqTU9h7iswjhLnr
			hex!["d7de8a43f7ee49fa3b3aaf32fb12617ec9ff7b246a46ab14e9c9d259261117fa"]
				.unchecked_into(),
			//5CoUk3wrCGJAWbiJEcsVjYhnd2JAHvR59jBRbSw77YrBtRL1
			hex!["209f680bc501f9b59358efe3636c51fd61238a8659bac146db909aea2595284b"]
				.unchecked_into(),
			//5EcSu96wprFM7G2HfJTjYu8kMParnYGznSUNTsoEKXywEsgG
			hex!["70adf80395b3f59e4cab5d9da66d5a286a0b6e138652a06f72542e46912df922"]
				.unchecked_into(),
			//5Ge3sjpD43Cuy7rNoJQmE9WctgCn6Faw89Pe7xPs3i55eHwJ
			hex!["ca5f6b970b373b303f64801a0c2cadc4fc05272c6047a2560a27d0c65589ca1d"]
				.unchecked_into(),
			//5EFcjHLvB2z5vd5g63n4gABmhzP5iPsKvTwd8sjfvTehNNrk
			hex!["60cae7fa5a079d9fc8061d715fbcc35ef57c3b00005694c2badce22dcc5a9f1b"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * WND;
	const STASH: u128 = 100 * WND;

	westend::GenesisConfig {
		system: westend::SystemConfig { code: wasm_binary.to_vec() },
		balances: westend::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		indices: westend::IndicesConfig { indices: vec![] },
		session: westend::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						westend_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: westend::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, westend::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		babe: westend::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(westend::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: westend::AuthorityDiscoveryConfig { keys: vec![] },
		vesting: westend::VestingConfig { vesting: vec![] },
		sudo: westend::SudoConfig { key: Some(endowed_accounts[0].clone()) },
		hrmp: Default::default(),
		configuration: westend::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		registrar: westend_runtime::RegistrarConfig {
			next_free_para_id: polkadot_primitives::v2::LOWEST_PUBLIC_ID,
		},
		xcm_pallet: Default::default(),
		nomination_pools: Default::default(),
	}
}

#[cfg(feature = "kusama-native")]
fn kusama_staging_testnet_config_genesis(wasm_binary: &[u8]) -> kusama::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	let endowed_accounts = vec![
		// v1
		hex!["209dcb067214a185e173a0ed214e51af000d6eef4dd643001a180f2c90fec820"].into(),
		hex!["c43ab2103302a4d46e7c41b6d146180cb874418b22f1e350af4b242ab528644f"].into(),
		// v2
		hex!["74c84ff35bd3a01e0cf5a1045d9e00c427c2f941d60a44e92474924883a9ef25"].into(),
		hex!["6ca782e79167c330cecd4beb0fcc239fe94f582720b8a603214ca606be5ef87b"].into(),
		// v3
		hex!["30b7dad7196ea53ff15cc731714160b0c0b0bad4b852dd96c91fddf2a30e1368"].into(),
		hex!["dca0964b7c936ad7e6961c1c5d5bfd53a646a970a17ade0f8556029465bc7c6f"].into(),
		// v4
		hex!["469e24763b72ca09007e1ca8d2f7030c54890c058506bb96a436794fc622c91c"].into(),
		hex!["560dc5205fb8c9191530557d58c5fbcbd7acdf1cd6a8d07ea0d612b32ca65244"].into(),
		// v5
		hex!["ce2129456edc979a34c43a6140eb077160028db7f3e0ae53bdbf17599d232834"].into(),
		hex!["6e59a058d9ae78161ff2598e3b3d0ab13ad01256edcdb28aea80ce3c45ef1274"].into(),
		// a1/v6
		hex!["f6cc4f8aaad00f837aaced5b4c4fddb7fb44b145c6714e34ea80fdc11411711d"].into(),
		hex!["4cb5475e5dafce0617467e87abb1cb908ced8805495f23c4e28f9c6183a0d450"].into(),
		// a2/v7
		hex!["b456043dd6f8ca1a015eb77c14c0c5c5502703084826543824af7ec53d176d64"].into(),
		hex!["ce8515151fcca9ca877ca39896771ffbea6d54bdf04990158eca7b8660544464"].into(),
		// a3/v8
		hex!["10ca1b0f57f16cb77b82fd55a5c4da8e47ef3699a39b41c3d5176e1ca74a2b63"].into(),
		hex!["f27208b2c79ae83b77cd9d53cccc6c4c79ab4f3e3abbd5c12fad5a9a30168d1f"].into(),
	];

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// v1-1
			hex!["209dcb067214a185e173a0ed214e51af000d6eef4dd643001a180f2c90fec820"].into(),
			hex!["209dcb067214a185e173a0ed214e51af000d6eef4dd643001a180f2c90fec820"].into(),
			hex!["209dcb067214a185e173a0ed214e51af000d6eef4dd643001a180f2c90fec820"]
				.unchecked_into(),
			hex!["7af5f760d086b7efb287e1e41534840d1842b9d9a35c501ecddfda18d2998daa"]
				.unchecked_into(),
			hex!["209dcb067214a185e173a0ed214e51af000d6eef4dd643001a180f2c90fec820"]
				.unchecked_into(),
			hex!["209dcb067214a185e173a0ed214e51af000d6eef4dd643001a180f2c90fec820"]
				.unchecked_into(),
			hex!["209dcb067214a185e173a0ed214e51af000d6eef4dd643001a180f2c90fec820"]
				.unchecked_into(),
			hex!["209dcb067214a185e173a0ed214e51af000d6eef4dd643001a180f2c90fec820"]
				.unchecked_into(),
		),
		(
			// v1-2
			hex!["c43ab2103302a4d46e7c41b6d146180cb874418b22f1e350af4b242ab528644f"].into(),
			hex!["c43ab2103302a4d46e7c41b6d146180cb874418b22f1e350af4b242ab528644f"].into(),
			hex!["c43ab2103302a4d46e7c41b6d146180cb874418b22f1e350af4b242ab528644f"]
				.unchecked_into(),
			hex!["83088261ee28c45029d7b4b4e1743b4a661402d2871a32a9e513b0626c838051"]
				.unchecked_into(),
			hex!["c43ab2103302a4d46e7c41b6d146180cb874418b22f1e350af4b242ab528644f"]
				.unchecked_into(),
			hex!["c43ab2103302a4d46e7c41b6d146180cb874418b22f1e350af4b242ab528644f"]
				.unchecked_into(),
			hex!["c43ab2103302a4d46e7c41b6d146180cb874418b22f1e350af4b242ab528644f"]
				.unchecked_into(),
			hex!["c43ab2103302a4d46e7c41b6d146180cb874418b22f1e350af4b242ab528644f"]
				.unchecked_into(),
		),
		(
			// v2-1
			hex!["74c84ff35bd3a01e0cf5a1045d9e00c427c2f941d60a44e92474924883a9ef25"].into(),
			hex!["74c84ff35bd3a01e0cf5a1045d9e00c427c2f941d60a44e92474924883a9ef25"].into(),
			hex!["74c84ff35bd3a01e0cf5a1045d9e00c427c2f941d60a44e92474924883a9ef25"]
				.unchecked_into(),
			hex!["09365475e83516fe8a52712aecddb5b0d2e79391507ac40ce12ce235d7038131"]
				.unchecked_into(),
			hex!["74c84ff35bd3a01e0cf5a1045d9e00c427c2f941d60a44e92474924883a9ef25"]
				.unchecked_into(),
			hex!["74c84ff35bd3a01e0cf5a1045d9e00c427c2f941d60a44e92474924883a9ef25"]
				.unchecked_into(),
			hex!["74c84ff35bd3a01e0cf5a1045d9e00c427c2f941d60a44e92474924883a9ef25"]
				.unchecked_into(),
			hex!["74c84ff35bd3a01e0cf5a1045d9e00c427c2f941d60a44e92474924883a9ef25"]
				.unchecked_into(),
		),
		(
			// v2-2
			hex!["6ca782e79167c330cecd4beb0fcc239fe94f582720b8a603214ca606be5ef87b"].into(),
			hex!["6ca782e79167c330cecd4beb0fcc239fe94f582720b8a603214ca606be5ef87b"].into(),
			hex!["6ca782e79167c330cecd4beb0fcc239fe94f582720b8a603214ca606be5ef87b"]
				.unchecked_into(),
			hex!["68f7253b3c462aea28ba1278264e3cff745fd910206787f01d562450bea47af4"]
				.unchecked_into(),
			hex!["6ca782e79167c330cecd4beb0fcc239fe94f582720b8a603214ca606be5ef87b"]
				.unchecked_into(),
			hex!["6ca782e79167c330cecd4beb0fcc239fe94f582720b8a603214ca606be5ef87b"]
				.unchecked_into(),
			hex!["6ca782e79167c330cecd4beb0fcc239fe94f582720b8a603214ca606be5ef87b"]
				.unchecked_into(),
			hex!["6ca782e79167c330cecd4beb0fcc239fe94f582720b8a603214ca606be5ef87b"]
				.unchecked_into(),
		),
		(
			// v3-1
			hex!["30b7dad7196ea53ff15cc731714160b0c0b0bad4b852dd96c91fddf2a30e1368"].into(),
			hex!["30b7dad7196ea53ff15cc731714160b0c0b0bad4b852dd96c91fddf2a30e1368"].into(),
			hex!["30b7dad7196ea53ff15cc731714160b0c0b0bad4b852dd96c91fddf2a30e1368"]
				.unchecked_into(),
			hex!["6f9506893bbbca7b1d0c8b9a3e8ebb7b8c8d5bf6911601bb9361a8ec6d26339d"]
				.unchecked_into(),
			hex!["30b7dad7196ea53ff15cc731714160b0c0b0bad4b852dd96c91fddf2a30e1368"]
				.unchecked_into(),
			hex!["30b7dad7196ea53ff15cc731714160b0c0b0bad4b852dd96c91fddf2a30e1368"]
				.unchecked_into(),
			hex!["30b7dad7196ea53ff15cc731714160b0c0b0bad4b852dd96c91fddf2a30e1368"]
				.unchecked_into(),
			hex!["30b7dad7196ea53ff15cc731714160b0c0b0bad4b852dd96c91fddf2a30e1368"]
				.unchecked_into(),
		),
		(
			// v3-2
			hex!["dca0964b7c936ad7e6961c1c5d5bfd53a646a970a17ade0f8556029465bc7c6f"].into(),
			hex!["dca0964b7c936ad7e6961c1c5d5bfd53a646a970a17ade0f8556029465bc7c6f"].into(),
			hex!["dca0964b7c936ad7e6961c1c5d5bfd53a646a970a17ade0f8556029465bc7c6f"]
				.unchecked_into(),
			hex!["f4244947370830e00209a1d5c48a7f8f52278f5c16ee593e34e5fa4e0bba3cc0"]
				.unchecked_into(),
			hex!["dca0964b7c936ad7e6961c1c5d5bfd53a646a970a17ade0f8556029465bc7c6f"]
				.unchecked_into(),
			hex!["dca0964b7c936ad7e6961c1c5d5bfd53a646a970a17ade0f8556029465bc7c6f"]
				.unchecked_into(),
			hex!["dca0964b7c936ad7e6961c1c5d5bfd53a646a970a17ade0f8556029465bc7c6f"]
				.unchecked_into(),
			hex!["dca0964b7c936ad7e6961c1c5d5bfd53a646a970a17ade0f8556029465bc7c6f"]
				.unchecked_into(),
		),
		(
			// v4-1
			hex!["469e24763b72ca09007e1ca8d2f7030c54890c058506bb96a436794fc622c91c"].into(),
			hex!["469e24763b72ca09007e1ca8d2f7030c54890c058506bb96a436794fc622c91c"].into(),
			hex!["469e24763b72ca09007e1ca8d2f7030c54890c058506bb96a436794fc622c91c"]
				.unchecked_into(),
			hex!["8d0c28e0667a1e6c5bdd3a542434ad1dee66908451eedb87cd5fb4544d99887a"]
				.unchecked_into(),
			hex!["469e24763b72ca09007e1ca8d2f7030c54890c058506bb96a436794fc622c91c"]
				.unchecked_into(),
			hex!["469e24763b72ca09007e1ca8d2f7030c54890c058506bb96a436794fc622c91c"]
				.unchecked_into(),
			hex!["469e24763b72ca09007e1ca8d2f7030c54890c058506bb96a436794fc622c91c"]
				.unchecked_into(),
			hex!["469e24763b72ca09007e1ca8d2f7030c54890c058506bb96a436794fc622c91c"]
				.unchecked_into(),
		),
		(
			// v4-2
			hex!["560dc5205fb8c9191530557d58c5fbcbd7acdf1cd6a8d07ea0d612b32ca65244"].into(),
			hex!["560dc5205fb8c9191530557d58c5fbcbd7acdf1cd6a8d07ea0d612b32ca65244"].into(),
			hex!["560dc5205fb8c9191530557d58c5fbcbd7acdf1cd6a8d07ea0d612b32ca65244"]
				.unchecked_into(),
			hex!["8b6a204a006af7d175a39a5c135f9f33cab80dbdbc9ed82e2e0731dd576ba985"]
				.unchecked_into(),
			hex!["560dc5205fb8c9191530557d58c5fbcbd7acdf1cd6a8d07ea0d612b32ca65244"]
				.unchecked_into(),
			hex!["560dc5205fb8c9191530557d58c5fbcbd7acdf1cd6a8d07ea0d612b32ca65244"]
				.unchecked_into(),
			hex!["560dc5205fb8c9191530557d58c5fbcbd7acdf1cd6a8d07ea0d612b32ca65244"]
				.unchecked_into(),
			hex!["560dc5205fb8c9191530557d58c5fbcbd7acdf1cd6a8d07ea0d612b32ca65244"]
				.unchecked_into(),
		),
		(
			// v5-1
			hex!["ce2129456edc979a34c43a6140eb077160028db7f3e0ae53bdbf17599d232834"].into(),
			hex!["ce2129456edc979a34c43a6140eb077160028db7f3e0ae53bdbf17599d232834"].into(),
			hex!["ce2129456edc979a34c43a6140eb077160028db7f3e0ae53bdbf17599d232834"]
				.unchecked_into(),
			hex!["1a70dd8d7c3452153f7f0774277ef69cbf4d53100f6d7eb1d2af37c2d8c0e6e1"]
				.unchecked_into(),
			hex!["ce2129456edc979a34c43a6140eb077160028db7f3e0ae53bdbf17599d232834"]
				.unchecked_into(),
			hex!["ce2129456edc979a34c43a6140eb077160028db7f3e0ae53bdbf17599d232834"]
				.unchecked_into(),
			hex!["ce2129456edc979a34c43a6140eb077160028db7f3e0ae53bdbf17599d232834"]
				.unchecked_into(),
			hex!["ce2129456edc979a34c43a6140eb077160028db7f3e0ae53bdbf17599d232834"]
				.unchecked_into(),
		),
		(
			// v5-2
			hex!["6e59a058d9ae78161ff2598e3b3d0ab13ad01256edcdb28aea80ce3c45ef1274"].into(),
			hex!["6e59a058d9ae78161ff2598e3b3d0ab13ad01256edcdb28aea80ce3c45ef1274"].into(),
			hex!["6e59a058d9ae78161ff2598e3b3d0ab13ad01256edcdb28aea80ce3c45ef1274"]
				.unchecked_into(),
			hex!["78cc1f2066af2ea827486b59ade33b9073f029a8e180bc9d2dd968efc10db1ca"]
				.unchecked_into(),
			hex!["6e59a058d9ae78161ff2598e3b3d0ab13ad01256edcdb28aea80ce3c45ef1274"]
				.unchecked_into(),
			hex!["6e59a058d9ae78161ff2598e3b3d0ab13ad01256edcdb28aea80ce3c45ef1274"]
				.unchecked_into(),
			hex!["6e59a058d9ae78161ff2598e3b3d0ab13ad01256edcdb28aea80ce3c45ef1274"]
				.unchecked_into(),
			hex!["6e59a058d9ae78161ff2598e3b3d0ab13ad01256edcdb28aea80ce3c45ef1274"]
				.unchecked_into(),
		),
		(
			// v6-1
			hex!["f6cc4f8aaad00f837aaced5b4c4fddb7fb44b145c6714e34ea80fdc11411711d"].into(),
			hex!["f6cc4f8aaad00f837aaced5b4c4fddb7fb44b145c6714e34ea80fdc11411711d"].into(),
			hex!["f6cc4f8aaad00f837aaced5b4c4fddb7fb44b145c6714e34ea80fdc11411711d"]
				.unchecked_into(),
			hex!["c54828e25a1defb99c596584166dda156aff76a2222d221000afa0049c8c1b4a"]
				.unchecked_into(),
			hex!["f6cc4f8aaad00f837aaced5b4c4fddb7fb44b145c6714e34ea80fdc11411711d"]
				.unchecked_into(),
			hex!["f6cc4f8aaad00f837aaced5b4c4fddb7fb44b145c6714e34ea80fdc11411711d"]
				.unchecked_into(),
			hex!["f6cc4f8aaad00f837aaced5b4c4fddb7fb44b145c6714e34ea80fdc11411711d"]
				.unchecked_into(),
			hex!["f6cc4f8aaad00f837aaced5b4c4fddb7fb44b145c6714e34ea80fdc11411711d"]
				.unchecked_into(),
		),
		(
			// v6-2
			hex!["4cb5475e5dafce0617467e87abb1cb908ced8805495f23c4e28f9c6183a0d450"].into(),
			hex!["4cb5475e5dafce0617467e87abb1cb908ced8805495f23c4e28f9c6183a0d450"].into(),
			hex!["4cb5475e5dafce0617467e87abb1cb908ced8805495f23c4e28f9c6183a0d450"]
				.unchecked_into(),
			hex!["777aa6bff811a4ab54614af138080c41d05bb1e9807863897ab23fbcde14a792"]
				.unchecked_into(),
			hex!["4cb5475e5dafce0617467e87abb1cb908ced8805495f23c4e28f9c6183a0d450"]
				.unchecked_into(),
			hex!["4cb5475e5dafce0617467e87abb1cb908ced8805495f23c4e28f9c6183a0d450"]
				.unchecked_into(),
			hex!["4cb5475e5dafce0617467e87abb1cb908ced8805495f23c4e28f9c6183a0d450"]
				.unchecked_into(),
			hex!["4cb5475e5dafce0617467e87abb1cb908ced8805495f23c4e28f9c6183a0d450"]
				.unchecked_into(),
		),
		(
			// v7-1
			hex!["b456043dd6f8ca1a015eb77c14c0c5c5502703084826543824af7ec53d176d64"].into(),
			hex!["b456043dd6f8ca1a015eb77c14c0c5c5502703084826543824af7ec53d176d64"].into(),
			hex!["b456043dd6f8ca1a015eb77c14c0c5c5502703084826543824af7ec53d176d64"]
				.unchecked_into(),
			hex!["2e80bc6bf7855b6d16b9e8f82ee8421b75714b6d1ed97690590ab8ab56b58fc1"]
				.unchecked_into(),
			hex!["b456043dd6f8ca1a015eb77c14c0c5c5502703084826543824af7ec53d176d64"]
				.unchecked_into(),
			hex!["b456043dd6f8ca1a015eb77c14c0c5c5502703084826543824af7ec53d176d64"]
				.unchecked_into(),
			hex!["b456043dd6f8ca1a015eb77c14c0c5c5502703084826543824af7ec53d176d64"]
				.unchecked_into(),
			hex!["b456043dd6f8ca1a015eb77c14c0c5c5502703084826543824af7ec53d176d64"]
				.unchecked_into(),
		),
		(
			// v7-2
			hex!["ce8515151fcca9ca877ca39896771ffbea6d54bdf04990158eca7b8660544464"].into(),
			hex!["ce8515151fcca9ca877ca39896771ffbea6d54bdf04990158eca7b8660544464"].into(),
			hex!["ce8515151fcca9ca877ca39896771ffbea6d54bdf04990158eca7b8660544464"]
				.unchecked_into(),
			hex!["87ba1200e908805623fc8a3051c274d631bf533cbd144d8eead1d606a56b5c41"]
				.unchecked_into(),
			hex!["ce8515151fcca9ca877ca39896771ffbea6d54bdf04990158eca7b8660544464"]
				.unchecked_into(),
			hex!["ce8515151fcca9ca877ca39896771ffbea6d54bdf04990158eca7b8660544464"]
				.unchecked_into(),
			hex!["ce8515151fcca9ca877ca39896771ffbea6d54bdf04990158eca7b8660544464"]
				.unchecked_into(),
			hex!["ce8515151fcca9ca877ca39896771ffbea6d54bdf04990158eca7b8660544464"]
				.unchecked_into(),
		),
		(
			// v8-1
			hex!["10ca1b0f57f16cb77b82fd55a5c4da8e47ef3699a39b41c3d5176e1ca74a2b63"].into(),
			hex!["10ca1b0f57f16cb77b82fd55a5c4da8e47ef3699a39b41c3d5176e1ca74a2b63"].into(),
			hex!["10ca1b0f57f16cb77b82fd55a5c4da8e47ef3699a39b41c3d5176e1ca74a2b63"]
				.unchecked_into(),
			hex!["688498b98a8d0246bc7d4575163f8d559b2bf76555c5624bca97c0b103ab6728"]
				.unchecked_into(),
			hex!["10ca1b0f57f16cb77b82fd55a5c4da8e47ef3699a39b41c3d5176e1ca74a2b63"]
				.unchecked_into(),
			hex!["10ca1b0f57f16cb77b82fd55a5c4da8e47ef3699a39b41c3d5176e1ca74a2b63"]
				.unchecked_into(),
			hex!["10ca1b0f57f16cb77b82fd55a5c4da8e47ef3699a39b41c3d5176e1ca74a2b63"]
				.unchecked_into(),
			hex!["10ca1b0f57f16cb77b82fd55a5c4da8e47ef3699a39b41c3d5176e1ca74a2b63"]
				.unchecked_into(),
		),
		(
			// v8-2
			hex!["f27208b2c79ae83b77cd9d53cccc6c4c79ab4f3e3abbd5c12fad5a9a30168d1f"].into(),
			hex!["f27208b2c79ae83b77cd9d53cccc6c4c79ab4f3e3abbd5c12fad5a9a30168d1f"].into(),
			hex!["f27208b2c79ae83b77cd9d53cccc6c4c79ab4f3e3abbd5c12fad5a9a30168d1f"]
				.unchecked_into(),
			hex!["eb4600d8ecc7226eddda893edf1bf1fcecd1c4b7db4391bd6fe4ca2ef5d02782"]
				.unchecked_into(),
			hex!["f27208b2c79ae83b77cd9d53cccc6c4c79ab4f3e3abbd5c12fad5a9a30168d1f"]
				.unchecked_into(),
			hex!["f27208b2c79ae83b77cd9d53cccc6c4c79ab4f3e3abbd5c12fad5a9a30168d1f"]
				.unchecked_into(),
			hex!["f27208b2c79ae83b77cd9d53cccc6c4c79ab4f3e3abbd5c12fad5a9a30168d1f"]
				.unchecked_into(),
			hex!["f27208b2c79ae83b77cd9d53cccc6c4c79ab4f3e3abbd5c12fad5a9a30168d1f"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * KSM;

	kusama::GenesisConfig {
		system: kusama::SystemConfig { code: wasm_binary.to_vec() },
		balances: kusama::BalancesConfig {
			balances: endowed_accounts.iter().map(|k: &AccountId| (k.clone(), ENDOWMENT)).collect(),
		},
		indices: kusama::IndicesConfig { indices: vec![] },
		session: kusama::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						kusama_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: kusama::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 1,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						50_000_000_000_000_000,
						kusama::StakerStatus::Validator,
					)
				})
				.collect(),
			// invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			invulnerables: vec![],
			force_era: Forcing::ForceAlways,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: Default::default(),
		council: kusama::CouncilConfig {
			members: vec![endowed_accounts[0].clone()],
			phantom: Default::default(),
		},
		technical_committee: kusama::TechnicalCommitteeConfig {
			members: vec![endowed_accounts[0].clone()],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: kusama::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(kusama::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: kusama::AuthorityDiscoveryConfig { keys: vec![] },
		claims: kusama::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: kusama::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: kusama::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		xcm_pallet: Default::default(),
		nomination_pools: Default::default(),
		nis_counterpart_balances: Default::default(),
	}
}

#[cfg(feature = "rococo-native")]
fn rococo_staging_testnet_config_genesis(wasm_binary: &[u8]) -> rococo_runtime::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5DwBmEFPXRESyEam5SsQF1zbWSCn2kCjyLW51hJHXe9vW4xs
		hex!["52bc71c1eca5353749542dfdf0af97bf764f9c2f44e860cd485f1cd86400f649"].into(),
	];

	// ./scripts/prepare-test-net.sh 8
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)> = vec![
		(
			//5EHZkbp22djdbuMFH9qt1DVzSCvqi3zWpj6DAYfANa828oei
			hex!["62475fe5406a7cb6a64c51d0af9d3ab5c2151bcae982fb812f7a76b706914d6a"].into(),
			//5FeSEpi9UYYaWwXXb3tV88qtZkmSdB3mvgj3pXkxKyYLGhcd
			hex!["9e6e781a76810fe93187af44c79272c290c2b9e2b8b92ee11466cd79d8023f50"].into(),
			//5Fh6rDpMDhM363o1Z3Y9twtaCPfizGQWCi55BSykTQjGbP7H
			hex!["a076ef1280d768051f21d060623da3ab5b56944d681d303ed2d4bf658c5bed35"]
				.unchecked_into(),
			//5CPd3zoV9Aaah4xWucuDivMHJ2nEEmpdi864nPTiyRZp4t87
			hex!["0e6d7d1afbcc6547b92995a394ba0daed07a2420be08220a5a1336c6731f0bfa"]
				.unchecked_into(),
			//5F7BEa1LGFksUihyatf3dCDYneB8pWzVyavnByCsm5nBgezi
			hex!["86975a37211f8704e947a365b720f7a3e2757988eaa7d0f197e83dba355ef743"]
				.unchecked_into(),
			//5CP6oGfwqbEfML8efqm1tCZsUgRsJztp9L8ZkEUxA16W8PPz
			hex!["0e07a51d3213842f8e9363ce8e444255990a225f87e80a3d651db7841e1a0205"]
				.unchecked_into(),
			//5HQdwiDh8Qtd5dSNWajNYpwDvoyNWWA16Y43aEkCNactFc2b
			hex!["ec60e71fe4a567ef9fef99d4bbf37ffae70564b41aa6f94ef0317c13e0a5477b"]
				.unchecked_into(),
			//5HbSgM72xVuscsopsdeG3sCSCYdAeM1Tay9p79N6ky6vwDGq
			hex!["f49eae66a0ac9f610316906ec8f1a0928e20d7059d76a5ca53cbcb5a9b50dd3c"]
				.unchecked_into(),
			//5DPSWdgw38Spu315r6LSvYCggeeieBAJtP5A1qzuzKhqmjVu
			hex!["034f68c5661a41930c82f26a662276bf89f33467e1c850f2fb8ef687fe43d62276"]
				.unchecked_into(),
		),
		(
			//5DvH8oEjQPYhzCoQVo7WDU91qmQfLZvxe9wJcrojmJKebCmG
			hex!["520b48452969f6ddf263b664de0adb0c729d0e0ad3b0e5f3cb636c541bc9022a"].into(),
			//5ENZvCRzyXJJYup8bM6yEzb2kQHEb1NDpY2ZEyVGBkCfRdj3
			hex!["6618289af7ae8621981ffab34591e7a6486e12745dfa3fd3b0f7e6a3994c7b5b"].into(),
			//5DLjSUfqZVNAADbwYLgRvHvdzXypiV1DAEaDMjcESKTcqMoM
			hex!["38757d0de00a0c739e7d7984ef4bc01161bd61e198b7c01b618425c16bb5bd5f"]
				.unchecked_into(),
			//5HnDVBN9mD6mXyx8oryhDbJtezwNSj1VRXgLoYCBA6uEkiao
			hex!["fcd5f87a6fd5707a25122a01b4dac0a8482259df7d42a9a096606df1320df08d"]
				.unchecked_into(),
			//5DhyXZiuB1LvqYKFgT5tRpgGsN3is2cM9QxgW7FikvakbAZP
			hex!["48a910c0af90898f11bd57d37ceaea53c78994f8e1833a7ade483c9a84bde055"]
				.unchecked_into(),
			//5EPEWRecy2ApL5n18n3aHyU1956zXTRqaJpzDa9DoqiggNwF
			hex!["669a10892119453e9feb4e3f1ee8e028916cc3240022920ad643846fbdbee816"]
				.unchecked_into(),
			//5ES3fw5X4bndSgLNmtPfSbM2J1kLqApVB2CCLS4CBpM1UxUZ
			hex!["68bf52c482630a8d1511f2edd14f34127a7d7082219cccf7fd4c6ecdb535f80d"]
				.unchecked_into(),
			//5HeXbwb5PxtcRoopPZTp5CQun38atn2UudQ8p2AxR5BzoaXw
			hex!["f6f8fe475130d21165446a02fb1dbce3a7bf36412e5d98f4f0473aed9252f349"]
				.unchecked_into(),
			//5F7nTtN8MyJV4UsXpjg7tHSnfANXZ5KRPJmkASc1ZSH2Xoa5
			hex!["03a90c2bb6d3b7000020f6152fe2e5002fa970fd1f42aafb6c8edda8dacc2ea77e"]
				.unchecked_into(),
		),
		(
			//5FPMzsezo1PRxYbVpJMWK7HNbR2kUxidsAAxH4BosHa4wd6S
			hex!["92ef83665b39d7a565e11bf8d18d41d45a8011601c339e57a8ea88c8ff7bba6f"].into(),
			//5G6NQidFG7YiXsvV7hQTLGArir9tsYqD4JDxByhgxKvSKwRx
			hex!["b235f57244230589523271c27b8a490922ffd7dccc83b044feaf22273c1dc735"].into(),
			//5GpZhzAVg7SAtzLvaAC777pjquPEcNy1FbNUAG2nZvhmd6eY
			hex!["d2644c1ab2c63a3ad8d40ad70d4b260969e3abfe6d7e6665f50dc9f6365c9d2a"]
				.unchecked_into(),
			//5HAes2RQYPbYKbLBfKb88f4zoXv6pPA6Ke8CjN7dob3GpmSP
			hex!["e1b68fbd84333e31486c08e6153d9a1415b2e7e71b413702b7d64e9b631184a1"]
				.unchecked_into(),
			//5HTXBf36LXmkFWJLokNUK6fPxVpkr2ToUnB1pvaagdGu4c1T
			hex!["ee93e26259decb89afcf17ef2aa0fa2db2e1042fb8f56ecfb24d19eae8629878"]
				.unchecked_into(),
			//5FtAGDZYJKXkhVhAxCQrXmaP7EE2mGbBMfmKDHjfYDgq2BiU
			hex!["a8e61ffacafaf546283dc92d14d7cc70ea0151a5dd81fdf73ff5a2951f2b6037"]
				.unchecked_into(),
			//5CtK7JHv3h6UQZ44y54skxdwSVBRtuxwPE1FYm7UZVhg8rJV
			hex!["244f3421b310c68646e99cdbf4963e02067601f57756b072a4b19431448c186e"]
				.unchecked_into(),
			//5D4r6YaB6F7A7nvMRHNFNF6zrR9g39bqDJFenrcaFmTCRwfa
			hex!["2c57f81fd311c1ab53813c6817fe67f8947f8d39258252663b3384ab4195494d"]
				.unchecked_into(),
			//5EPoHj8uV4fFKQHYThc6Z9fDkU7B6ih2ncVzQuDdNFb8UyhF
			hex!["039d065fe4f9234f0a4f13cc3ae585f2691e9c25afa469618abb6645111f607a53"]
				.unchecked_into(),
		),
		(
			//5DMNx7RoX6d7JQ38NEM7DWRcW2THu92LBYZEWvBRhJeqcWgR
			hex!["38f3c2f38f6d47f161e98c697bbe3ca0e47c033460afda0dda314ab4222a0404"].into(),
			//5GGdKNDr9P47dpVnmtq3m8Tvowwf1ot1abw6tPsTYYFoKm2v
			hex!["ba0898c1964196474c0be08d364cdf4e9e1d47088287f5235f70b0590dfe1704"].into(),
			//5EjkyPCzR2SjhDZq8f7ufsw6TfkvgNRepjCRQFc4TcdXdaB1
			hex!["764186bc30fd5a02477f19948dc723d6d57ab174debd4f80ed6038ec960bfe21"]
				.unchecked_into(),
			//5DJV3zCBTJBLGNDCcdWrYxWDacSz84goGTa4pFeKVvehEBte
			hex!["36be9069cdb4a8a07ecd51f257875150f0a8a1be44a10d9d98dabf10a030aef4"]
				.unchecked_into(),
			//5FHf8kpK4fPjEJeYcYon2gAPwEBubRvtwpzkUbhMWSweKPUY
			hex!["8e95b9b5b4dc69790b67b566567ca8bf8cdef3a3a8bb65393c0d1d1c87cd2d2c"]
				.unchecked_into(),
			//5F9FsRjpecP9GonktmtFL3kjqNAMKjHVFjyjRdTPa4hbQRZA
			hex!["882d72965e642677583b333b2d173ac94b5fd6c405c76184bb14293be748a13b"]
				.unchecked_into(),
			//5F1FZWZSj3JyTLs8sRBxU6QWyGLSL9BMRtmSKDmVEoiKFxSP
			hex!["821271c99c958b9220f1771d9f5e29af969edfa865631dba31e1ab7bc0582b75"]
				.unchecked_into(),
			//5CtgRR74VypK4h154s369abs78hDUxZSJqcbWsfXvsjcHJNA
			hex!["2496f28d887d84705c6dae98aee8bf90fc5ad10bb5545eca1de6b68425b70f7c"]
				.unchecked_into(),
			//5CPx6dsr11SCJHKFkcAQ9jpparS7FwXQBrrMznRo4Hqv1PXz
			hex!["0307d29bbf6a5c4061c2157b44fda33b7bb4ec52a5a0305668c74688cedf288d58"]
				.unchecked_into(),
		),
		(
			//5C8AL1Zb4bVazgT3EgDxFgcow1L4SJjVu44XcLC9CrYqFN4N
			hex!["02a2d8cfcf75dda85fafc04ace3bcb73160034ed1964c43098fb1fe831de1b16"].into(),
			//5FLYy3YKsAnooqE4hCudttAsoGKbVG3hYYBtVzwMjJQrevPa
			hex!["90cab33f0bb501727faa8319f0845faef7d31008f178b65054b6629fe531b772"].into(),
			//5Et3tfbVf1ByFThNAuUq5pBssdaPPskip5yob5GNyUFojXC7
			hex!["7c94715e5dd8ab54221b1b6b2bfa5666f593f28a92a18e28052531de1bd80813"]
				.unchecked_into(),
			//5EX1JBghGbQqWohTPU6msR9qZ2nYPhK9r3RTQ2oD1K8TCxaG
			hex!["6c878e33b83c20324238d22240f735457b6fba544b383e70bb62a27b57380c81"]
				.unchecked_into(),
			//5GqL8RbVAuNXpDhjQi1KrS1MyNuKhvus2AbmQwRGjpuGZmFu
			hex!["d2f9d537ffa59919a4028afdb627c14c14c97a1547e13e8e82203d2049b15b1a"]
				.unchecked_into(),
			//5EUNaBpX9mJgcmLQHyG5Pkms6tbDiKuLbeTEJS924Js9cA1N
			hex!["6a8570b9c6408e54bacf123cc2bb1b0f087f9c149147d0005badba63a5a4ac01"]
				.unchecked_into(),
			//5CaZuueRVpMATZG4hkcrgDoF4WGixuz7zu83jeBdY3bgWGaG
			hex!["16c69ea8d595e80b6736f44be1eaeeef2ac9c04a803cc4fd944364cb0d617a33"]
				.unchecked_into(),
			//5DABsdQCDUGuhzVGWe5xXzYQ9rtrVxRygW7RXf9Tsjsw1aGJ
			hex!["306ac5c772fe858942f92b6e28bd82fb7dd8cdd25f9a4626c1b0eee075fcb531"]
				.unchecked_into(),
			//5H91T5mHhoCw9JJG4NjghDdQyhC6L7XcSuBWKD3q3TAhEVvQ
			hex!["02fb0330356e63a35dd930bc74525edf28b3bf5eb44aab9e9e4962c8309aaba6a6"]
				.unchecked_into(),
		),
		(
			//5C8XbDXdMNKJrZSrQURwVCxdNdk8AzG6xgLggbzuA399bBBF
			hex!["02ea6bfa8b23b92fe4b5db1063a1f9475e3acd0ab61e6b4f454ed6ba00b5f864"].into(),
			//5GsyzFP8qtF8tXPSsjhjxAeU1v7D1PZofuQKN9TdCc7Dp1JM
			hex!["d4ffc4c05b47d1115ad200f7f86e307b20b46c50e1b72a912ec4f6f7db46b616"].into(),
			//5GHWB8ZDzegLcMW7Gdd1BS6WHVwDdStfkkE4G7KjPjZNJBtD
			hex!["bab3cccdcc34401e9b3971b96a662686cf755aa869a5c4b762199ce531b12c5b"]
				.unchecked_into(),
			//5GzDPGbUM9uH52ZEwydasTj8edokGUJ7vEpoFWp9FE1YNuFB
			hex!["d9c056c98ca0e6b4eb7f5c58c007c1db7be0fe1f3776108f797dd4990d1ccc33"]
				.unchecked_into(),
			//5GWZbVkJEfWZ7fRca39YAQeqri2Z7pkeHyd7rUctUHyQifLp
			hex!["c4a980da30939d5bb9e4a734d12bf81259ae286aa21fa4b65405347fa40eff35"]
				.unchecked_into(),
			//5CmLCFeSurRXXtwMmLcVo7sdJ9EqDguvJbuCYDcHkr3cpqyE
			hex!["1efc23c0b51ad609ab670ecf45807e31acbd8e7e5cb7c07cf49ee42992d2867c"]
				.unchecked_into(),
			//5DnsSy8a8pfE2aFjKBDtKw7WM1V4nfE5sLzP15MNTka53GqS
			hex!["4c64d3f06d28adeb36a892fdaccecace150bec891f04694448a60b74fa469c22"]
				.unchecked_into(),
			//5CZdFnyzZvKetZTeUwj5APAYskVJe4QFiTezo5dQNsrnehGd
			hex!["160ea09c5717270e958a3da42673fa011613a9539b2e4ebcad8626bc117ca04a"]
				.unchecked_into(),
			//5HgoR9JJkdBusxKrrs3zgd3ToppgNoGj1rDyAJp4e7eZiYyT
			hex!["020019a8bb188f8145d02fa855e9c36e9914457d37c500e03634b5223aa5702474"]
				.unchecked_into(),
		),
		(
			//5HinEonzr8MywkqedcpsmwpxKje2jqr9miEwuzyFXEBCvVXM
			hex!["fa373e25a1c4fe19c7148acde13bc3db1811cf656dc086820f3dda736b9c4a00"].into(),
			//5EHJbj6Td6ks5HDnyfN4ttTSi57osxcQsQexm7XpazdeqtV7
			hex!["62145d721967bd88622d08625f0f5681463c0f1b8bcd97eb3c2c53f7660fd513"].into(),
			//5EeCsC58XgJ1DFaoYA1WktEpP27jvwGpKdxPMFjicpLeYu96
			hex!["720537e2c1c554654d73b3889c3ef4c3c2f95a65dd3f7c185ebe4afebed78372"]
				.unchecked_into(),
			//5DnEySxbnppWEyN8cCLqvGjAorGdLRg2VmkY96dbJ1LHFK8N
			hex!["4bea0b37e0cce9bddd80835fa2bfd5606f5dcfb8388bbb10b10c483f0856cf14"]
				.unchecked_into(),
			//5E1Y1FJ7dVP7qtE3wm241pTm72rTMcDT5Jd8Czv7Pwp7N3AH
			hex!["560d90ca51e9c9481b8a9810060e04d0708d246714960439f804e5c6f40ca651"]
				.unchecked_into(),
			//5CAC278tFCHAeHYqE51FTWYxHmeLcENSS1RG77EFRTvPZMJT
			hex!["042f07fc5268f13c026bbe199d63e6ac77a0c2a780f71cda05cee5a6f1b3f11f"]
				.unchecked_into(),
			//5HjRTLWcQjZzN3JDvaj1UzjNSayg5ZD9ZGWMstaL7Ab2jjAa
			hex!["fab485e87ed1537d089df521edf983a777c57065a702d7ed2b6a2926f31da74f"]
				.unchecked_into(),
			//5ELv74v7QcsS6FdzvG4vL2NnYDGWmRnJUSMKYwdyJD7Xcdi7
			hex!["64d59feddb3d00316a55906953fb3db8985797472bd2e6c7ea1ab730cc339d7f"]
				.unchecked_into(),
			//5FaUcPt4fPz93vBhcrCJqmDkjYZ7jCbzAF56QJoCmvPaKrmx
			hex!["033f1a6d47fe86f88934e4b83b9fae903b92b5dcf4fec97d5e3e8bf4f39df03685"]
				.unchecked_into(),
		),
		(
			//5Ey3NQ3dfabaDc16NUv7wRLsFCMDFJSqZFzKVycAsWuUC6Di
			hex!["8062e9c21f1d92926103119f7e8153cebdb1e5ab3e52d6f395be80bb193eab47"].into(),
			//5HiWsuSBqt8nS9pnggexXuHageUifVPKPHDE2arTKqhTp1dV
			hex!["fa0388fa88f3f0cb43d583e2571fbc0edad57dff3a6fd89775451dd2c2b8ea00"].into(),
			//5H168nKX2Yrfo3bxj7rkcg25326Uv3CCCnKUGK6uHdKMdPt8
			hex!["da6b2df18f0f9001a6dcf1d301b92534fe9b1f3ccfa10c49449fee93adaa8349"]
				.unchecked_into(),
			//5DrA2fZdzmNqT5j6DXNwVxPBjDV9jhkAqvjt6Us3bQHKy3cF
			hex!["4ee66173993dd0db5d628c4c9cb61a27b76611ad3c3925947f0d0011ee2c5dcc"]
				.unchecked_into(),
			//5FNFDUGNLUtqg5LgrwYLNmBiGoP8KRxsvQpBkc7GQP6qaBUG
			hex!["92156f54a114ee191415898f2da013d9db6a5362d6b36330d5fc23e27360ab66"]
				.unchecked_into(),
			//5Gx6YeNhynqn8qkda9QKpc9S7oDr4sBrfAu516d3sPpEt26F
			hex!["d822d4088b20dca29a580a577a97d6f024bb24c9550bebdfd7d2d18e946a1c7d"]
				.unchecked_into(),
			//5DhDcHqwxoes5s89AyudGMjtZXx1nEgrk5P45X88oSTR3iyx
			hex!["481538f8c2c011a76d7d57db11c2789a5e83b0f9680dc6d26211d2f9c021ae4c"]
				.unchecked_into(),
			//5DqAvikdpfRdk5rR35ZobZhqaC5bJXZcEuvzGtexAZP1hU3T
			hex!["4e262811acdfe94528bfc3c65036080426a0e1301b9ada8d687a70ffcae99c26"]
				.unchecked_into(),
			//5E41Znrr2YtZu8bZp3nvRuLVHg3jFksfQ3tXuviLku4wsao7
			hex!["025e84e95ed043e387ddb8668176b42f8e2773ddd84f7f58a6d9bf436a4b527986"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * ROC;
	const STASH: u128 = 100 * ROC;

	rococo_runtime::GenesisConfig {
		system: rococo_runtime::SystemConfig { code: wasm_binary.to_vec() },
		balances: rococo_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		beefy: Default::default(),
		indices: rococo_runtime::IndicesConfig { indices: vec![] },
		session: rococo_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						rococo_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
							x.8.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		phragmen_election: Default::default(),
		babe: rococo_runtime::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(rococo_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		democracy: rococo_runtime::DemocracyConfig::default(),
		council: rococo::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: rococo::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		authority_discovery: rococo_runtime::AuthorityDiscoveryConfig { keys: vec![] },
		claims: rococo::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: rococo::VestingConfig { vesting: vec![] },
		sudo: rococo_runtime::SudoConfig { key: Some(endowed_accounts[0].clone()) },
		paras: rococo_runtime::ParasConfig { paras: vec![] },
		hrmp: Default::default(),
		configuration: rococo_runtime::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		registrar: rococo_runtime::RegistrarConfig {
			next_free_para_id: polkadot_primitives::v2::LOWEST_PUBLIC_ID,
		},
		xcm_pallet: Default::default(),
		nis_counterpart_balances: Default::default(),
	}
}

/// Returns the properties for the [`PolkadotChainSpec`].
pub fn polkadot_chain_spec_properties() -> serde_json::map::Map<String, serde_json::Value> {
	serde_json::json!({
		"tokenDecimals": 10,
	})
	.as_object()
	.expect("Map given; qed")
	.clone()
}

/// Polkadot staging testnet config.
#[cfg(feature = "polkadot-native")]
pub fn polkadot_staging_testnet_config() -> Result<PolkadotChainSpec, String> {
	let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;
	let boot_nodes = vec![ // use vN-1 as bootnodes
		"/dns/v1.internal.kusama.systems/tcp/30233/p2p/12D3KooWQfwm7u78JHMPZNrhp1TKSoAsSrkiTL8F5boXFuHabGPY".parse().unwrap(),
		"/dns/v2.internal.kusama.systems/tcp/30233/p2p/12D3KooWDoUwVFtZuQaVPk5zqTo53SbKs2oJGMnpkS1JEkR8vUyq".parse().unwrap(),
		"/dns/v3.internal.kusama.systems/tcp/30233/p2p/12D3KooWJMuL2GLoytZwBtgmuyVRTggykYyfqHH8BVQ7srCgSp8F".parse().unwrap(),
		"/dns/v4.internal.kusama.systems/tcp/30233/p2p/12D3KooWPxP6ZTTPNeT9mMG6sMoTLs7xaHqkBny9VXchiZ1HeWvc".parse().unwrap(),
		"/dns/v5.internal.kusama.systems/tcp/30233/p2p/12D3KooWBfM6QpsVJvUgu7LxZ3mXivaURUFDfpx5My3GbSQA3Ya9".parse().unwrap(),
	];

	Ok(PolkadotChainSpec::from_genesis(
		"Polkadot Staging Testnet",
		"polkadot_staging_testnet",
		ChainType::Live,
		move || polkadot_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(polkadot_chain_spec_properties()),
		Default::default(),
	))
}

/// Staging testnet config.
#[cfg(feature = "kusama-native")]
pub fn kusama_staging_testnet_config() -> Result<KusamaChainSpec, String> {
	let wasm_binary = kusama::WASM_BINARY.ok_or("Kusama development wasm not available")?;
	let boot_nodes = vec![
		"/dns/v1.internal.kusama.systems/tcp/30333/p2p/12D3KooWSW965mkWrzvFdsVeSerWAv5rm1SFfJuncYk2EfPUCsya".parse().unwrap(),
		"/dns/v3.internal.kusama.systems/tcp/30333/p2p/12D3KooWE6qZYa7pQhZHYxsRNTL1LJWunn8zFwRYcVBh24eQ16RP".parse().unwrap(),
		"/dns/v5.internal.kusama.systems/tcp/30333/p2p/12D3KooWGBphxCswn7izrjgLdywaVN6Z6qCfAdUcrjzvZr9X1uFa".parse().unwrap(),
		"/dns/v7.internal.kusama.systems/tcp/30333/p2p/12D3KooWF9GV2oNsSfug5oK4FbdhKUAVK2f8CE4ZfdkmsPFmMVLE".parse().unwrap(),
	];

	Ok(KusamaChainSpec::from_genesis(
		"Kusama Staging Testnet",
		"kusama_staging_testnet",
		ChainType::Live,
		move || kusama_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(
				"/dns/api.telemetry.pelagos.systems/tcp/443/x-parity-wss/%2Fsubmit%2F".to_string(),
				0,
			)])
			.unwrap(),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Westend staging testnet config.
#[cfg(feature = "westend-native")]
pub fn westend_staging_testnet_config() -> Result<WestendChainSpec, String> {
	let wasm_binary = westend::WASM_BINARY.ok_or("Westend development wasm not available")?;
	let boot_nodes = vec![];

	Ok(WestendChainSpec::from_genesis(
		"Westend Staging Testnet",
		"westend_staging_testnet",
		ChainType::Live,
		move || westend_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(WESTEND_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Westend Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Rococo staging testnet config.
#[cfg(feature = "rococo-native")]
pub fn rococo_staging_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?;
	let boot_nodes = vec![];

	Ok(RococoChainSpec::from_genesis(
		"Rococo Staging Testnet",
		"rococo_staging_testnet",
		ChainType::Live,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_staging_testnet_config_genesis(wasm_binary),
			session_length_in_blocks: None,
		},
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(ROCOCO_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Rococo Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

pub fn versi_chain_spec_properties() -> serde_json::map::Map<String, serde_json::Value> {
	serde_json::json!({
		"ss58Format": 42,
		"tokenDecimals": 12,
		"tokenSymbol": "VRS",
	})
	.as_object()
	.expect("Map given; qed")
	.clone()
}

/// Versi staging testnet config.
#[cfg(feature = "rococo-native")]
pub fn versi_staging_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Versi development wasm not available")?;
	let boot_nodes = vec![];

	Ok(RococoChainSpec::from_genesis(
		"Versi Staging Testnet",
		"versi_staging_testnet",
		ChainType::Live,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_staging_testnet_config_genesis(wasm_binary),
			session_length_in_blocks: Some(100),
		},
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(VERSI_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Versi Staging telemetry url is valid; qed"),
		),
		Some("versi"),
		None,
		Some(versi_chain_spec_properties()),
		Default::default(),
	))
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
	BeefyId,
) {
	let keys = get_authority_keys_from_seed_no_beefy(seed);
	(keys.0, keys.1, keys.2, keys.3, keys.4, keys.5, keys.6, keys.7, get_from_seed::<BeefyId>(seed))
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<ValidatorId>(seed),
		get_from_seed::<AssignmentId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
	vec![
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Charlie"),
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
		get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
	]
}

/// Helper function to create polkadot `GenesisConfig` for testing
#[cfg(feature = "polkadot-native")]
pub fn polkadot_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> polkadot::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOT;
	const STASH: u128 = 100 * DOT;

	polkadot::GenesisConfig {
		system: polkadot::SystemConfig { code: wasm_binary.to_vec() },
		indices: polkadot::IndicesConfig { indices: vec![] },
		balances: polkadot::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: polkadot::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						polkadot_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: polkadot::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, polkadot::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: polkadot::DemocracyConfig::default(),
		council: polkadot::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: polkadot::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: polkadot::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(polkadot::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: polkadot::AuthorityDiscoveryConfig { keys: vec![] },
		claims: polkadot::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: polkadot::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: polkadot::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		xcm_pallet: Default::default(),
		nomination_pools: Default::default(),
	}
}

/// Helper function to create kusama `GenesisConfig` for testing
#[cfg(feature = "kusama-native")]
pub fn kusama_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> kusama::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * KSM;
	const STASH: u128 = 100 * KSM;

	kusama::GenesisConfig {
		system: kusama::SystemConfig { code: wasm_binary.to_vec() },
		indices: kusama::IndicesConfig { indices: vec![] },
		balances: kusama::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: kusama::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.1.clone(),
						x.1.clone(),
						kusama_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: kusama::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), x.1.clone(), STASH, kusama::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: kusama::DemocracyConfig::default(),
		council: kusama::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: kusama::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: kusama::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(kusama::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: kusama::AuthorityDiscoveryConfig { keys: vec![] },
		claims: kusama::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: kusama::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: kusama::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		xcm_pallet: Default::default(),
		nomination_pools: Default::default(),
		nis_counterpart_balances: Default::default(),
	}
}

/// Helper function to create westend `GenesisConfig` for testing
#[cfg(feature = "westend-native")]
pub fn westend_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> westend::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * WND;
	const STASH: u128 = 100 * WND;

	westend::GenesisConfig {
		system: westend::SystemConfig { code: wasm_binary.to_vec() },
		indices: westend::IndicesConfig { indices: vec![] },
		balances: westend::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: westend::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						westend_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: westend::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, westend::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		babe: westend::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(westend::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: westend::AuthorityDiscoveryConfig { keys: vec![] },
		vesting: westend::VestingConfig { vesting: vec![] },
		sudo: westend::SudoConfig { key: Some(root_key) },
		hrmp: Default::default(),
		configuration: westend::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		registrar: westend_runtime::RegistrarConfig {
			next_free_para_id: polkadot_primitives::v2::LOWEST_PUBLIC_ID,
		},
		xcm_pallet: Default::default(),
		nomination_pools: Default::default(),
	}
}

/// Helper function to create rococo `GenesisConfig` for testing
#[cfg(feature = "rococo-native")]
pub fn rococo_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> rococo_runtime::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * ROC;

	rococo_runtime::GenesisConfig {
		system: rococo_runtime::SystemConfig { code: wasm_binary.to_vec() },
		beefy: Default::default(),
		indices: rococo_runtime::IndicesConfig { indices: vec![] },
		balances: rococo_runtime::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: rococo_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						rococo_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
							x.8.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		babe: rococo_runtime::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(rococo_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		phragmen_election: Default::default(),
		democracy: rococo::DemocracyConfig::default(),
		council: rococo::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: rococo::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		claims: rococo::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: rococo::VestingConfig { vesting: vec![] },
		authority_discovery: rococo_runtime::AuthorityDiscoveryConfig { keys: vec![] },
		sudo: rococo_runtime::SudoConfig { key: Some(root_key.clone()) },
		hrmp: Default::default(),
		configuration: rococo_runtime::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: rococo_runtime::ParasConfig { paras: vec![] },
		registrar: rococo_runtime::RegistrarConfig {
			next_free_para_id: polkadot_primitives::v2::LOWEST_PUBLIC_ID,
		},
		xcm_pallet: Default::default(),
		nis_counterpart_balances: Default::default(),
	}
}

#[cfg(feature = "polkadot-native")]
fn polkadot_development_config_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
	polkadot_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

#[cfg(feature = "kusama-native")]
fn kusama_development_config_genesis(wasm_binary: &[u8]) -> kusama::GenesisConfig {
	kusama_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

#[cfg(feature = "westend-native")]
fn westend_development_config_genesis(wasm_binary: &[u8]) -> westend::GenesisConfig {
	westend_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

#[cfg(feature = "rococo-native")]
fn rococo_development_config_genesis(wasm_binary: &[u8]) -> rococo_runtime::GenesisConfig {
	rococo_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Polkadot development config (single validator Alice)
#[cfg(feature = "polkadot-native")]
pub fn polkadot_development_config() -> Result<PolkadotChainSpec, String> {
	let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;

	Ok(PolkadotChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || polkadot_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(polkadot_chain_spec_properties()),
		Default::default(),
	))
}

/// Kusama development config (single validator Alice)
#[cfg(feature = "kusama-native")]
pub fn kusama_development_config() -> Result<KusamaChainSpec, String> {
	let wasm_binary = kusama::WASM_BINARY.ok_or("Kusama development wasm not available")?;

	Ok(KusamaChainSpec::from_genesis(
		"Development",
		"kusama_dev",
		ChainType::Development,
		move || kusama_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Westend development config (single validator Alice)
#[cfg(feature = "westend-native")]
pub fn westend_development_config() -> Result<WestendChainSpec, String> {
	let wasm_binary = westend::WASM_BINARY.ok_or("Westend development wasm not available")?;

	Ok(WestendChainSpec::from_genesis(
		"Development",
		"westend_dev",
		ChainType::Development,
		move || westend_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Rococo development config (single validator Alice)
#[cfg(feature = "rococo-native")]
pub fn rococo_development_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Development",
		"rococo_dev",
		ChainType::Development,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_development_config_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// `Versi` development config (single validator Alice)
#[cfg(feature = "rococo-native")]
pub fn versi_development_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Versi development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Development",
		"versi_dev",
		ChainType::Development,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_development_config_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some("versi"),
		None,
		None,
		Default::default(),
	))
}

/// Wococo development config (single validator Alice)
#[cfg(feature = "rococo-native")]
pub fn wococo_development_config() -> Result<RococoChainSpec, String> {
	const WOCOCO_DEV_PROTOCOL_ID: &str = "woco";
	let wasm_binary = rococo::WASM_BINARY.ok_or("Wococo development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Development",
		"wococo_dev",
		ChainType::Development,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_development_config_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(WOCOCO_DEV_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

#[cfg(feature = "polkadot-native")]
fn polkadot_local_testnet_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
	polkadot_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Polkadot local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "polkadot-native")]
pub fn polkadot_local_testnet_config() -> Result<PolkadotChainSpec, String> {
	let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;

	Ok(PolkadotChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || polkadot_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(polkadot_chain_spec_properties()),
		Default::default(),
	))
}

#[cfg(feature = "kusama-native")]
fn kusama_local_testnet_genesis(wasm_binary: &[u8]) -> kusama::GenesisConfig {
	kusama_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
			get_authority_keys_from_seed_no_beefy("Charlie"),
			get_authority_keys_from_seed_no_beefy("Dave"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		Some(vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
		]),
	)
}

/// Kusama local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "kusama-native")]
pub fn kusama_local_testnet_config() -> Result<KusamaChainSpec, String> {
	let wasm_binary = kusama::WASM_BINARY.ok_or("Kusama development wasm not available")?;

	Ok(KusamaChainSpec::from_genesis(
		"Kusama Local Testnet",
		"kusama_local_testnet",
		ChainType::Local,
		move || kusama_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

#[cfg(feature = "westend-native")]
fn westend_local_testnet_genesis(wasm_binary: &[u8]) -> westend::GenesisConfig {
	westend_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Westend local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "westend-native")]
pub fn westend_local_testnet_config() -> Result<WestendChainSpec, String> {
	let wasm_binary = westend::WASM_BINARY.ok_or("Westend development wasm not available")?;

	Ok(WestendChainSpec::from_genesis(
		"Westend Local Testnet",
		"westend_local_testnet",
		ChainType::Local,
		move || westend_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

#[cfg(feature = "rococo-native")]
fn rococo_local_testnet_genesis(wasm_binary: &[u8]) -> rococo_runtime::GenesisConfig {
	rococo_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
			get_authority_keys_from_seed("Charlie"),
			get_authority_keys_from_seed("Dave"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		Some(vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
		]),
	)
}

/// Rococo local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "rococo-native")]
pub fn rococo_local_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?;
	let boot_nodes = vec![
		"/dns/arangatuy.baikal.manta.systems/tcp/30333/p2p/12D3KooWHgRk27dyj4F3Hqa4AnDLHXue8AtDrGLps5FFwpzqGnHT".parse().unwrap(),
		"/dns/frolikha.baikal.manta.systems/tcp/30333/p2p/12D3KooWSZwMTXRACHC9n5xavxBcbMJKv4PjvEUTbACbtF4RvAT3".parse().unwrap(),
		"/dns/olkhon.baikal.manta.systems/tcp/30333/p2p/12D3KooWKopKfTvSRUUrzvK2YYaFp958tmxhdTDhGrf9b6umFpbp".parse().unwrap(),
		"/dns/ushkan.baikal.manta.systems/tcp/30333/p2p/12D3KooWBD12k1h3SQxwnNA5TLUccdrUNMTyYjRcZfWePnrjcckt".parse().unwrap(),
	];
	Ok(RococoChainSpec::from_genesis(
		"Baikal Rococo Testnet",
		"baikal_rococo_testnet",
		ChainType::Live,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_local_testnet_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(
				"/dns/api.telemetry.pelagos.systems/tcp/443/x-parity-wss/%2Fsubmit%2F".to_string(),
				0,
			)])
			.unwrap(),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Wococo is a temporary testnet that uses almost the same runtime as rococo.
#[cfg(feature = "rococo-native")]
fn wococo_local_testnet_genesis(wasm_binary: &[u8]) -> rococo_runtime::GenesisConfig {
	rococo_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
			get_authority_keys_from_seed("Charlie"),
			get_authority_keys_from_seed("Dave"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Wococo local testnet config (multivalidator Alice + Bob + Charlie + Dave)
#[cfg(feature = "rococo-native")]
pub fn wococo_local_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Wococo development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Wococo Local Testnet",
		"wococo_local_testnet",
		ChainType::Local,
		move || RococoGenesisExt {
			runtime_genesis_config: wococo_local_testnet_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// `Versi` is a temporary testnet that uses the same runtime as rococo.
#[cfg(feature = "rococo-native")]
fn versi_local_testnet_genesis(wasm_binary: &[u8]) -> rococo_runtime::GenesisConfig {
	rococo_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
			get_authority_keys_from_seed("Charlie"),
			get_authority_keys_from_seed("Dave"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// `Versi` local testnet config (multivalidator Alice + Bob + Charlie + Dave)
#[cfg(feature = "rococo-native")]
pub fn versi_local_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Versi development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Versi Local Testnet",
		"versi_local_testnet",
		ChainType::Local,
		move || RococoGenesisExt {
			runtime_genesis_config: versi_local_testnet_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some("versi"),
		None,
		None,
		Default::default(),
	))
}
