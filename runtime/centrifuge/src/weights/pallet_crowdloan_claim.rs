//! Autogenerated weights for pallet_crowdloan_claim
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_crowdloan_claim
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_crowdloan_claim.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use pallet_crowdloan_claim::weights::WeightInfo;
use sp_std::marker::PhantomData;

/// Weights for pallet_crowdloan_claim using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn claim_reward_ed25519() -> Weight {
		(297_651_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn claim_reward_sr25519() -> Weight {
		(305_570_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn claim_reward_ecdsa() -> Weight {
		(476_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn initialize() -> Weight {
		(38_207_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn set_lease_start() -> Weight {
		(19_026_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_lease_period() -> Weight {
		(18_939_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_contributions_root() -> Weight {
		(20_202_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_locked_at() -> Weight {
		(19_033_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_crowdloan_trie_index() -> Weight {
		(19_331_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}