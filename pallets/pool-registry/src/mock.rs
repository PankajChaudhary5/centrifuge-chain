// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
use std::marker::PhantomData;

use cfg_primitives::{BlockNumber, PoolEpochId, Moment};
use cfg_traits::{OrderManager, UpdateState, TrancheCurrency as TrancheCurrencyT, PreConditions, Permissions as PermissionsT};
use cfg_types::{
	fixed_point::Rate,
	permissions::{PermissionScope, PermissionRoles, Role, PoolRole, UNION},
	time::TimeProvider,
	tokens::{CustomMetadata, CurrencyId, TrancheCurrency},
};
use pallet_restricted_tokens::TransferDetails;
use orml_traits::{asset_registry::AssetMetadata, parameter_type_with_key};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	parameter_types,
	traits::{Hooks, SortedMembers, PalletInfoAccess},
};
use frame_system::{EnsureSigned, EnsureSignedBy};
use pallet_pool_system::{pool_types::PoolChanges, tranches::TrancheInput};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use pallet_pool_system::*;

use crate::{self as pallet_pool_registry, Config, PoolMutate};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type TrancheId = [u8; 16];

const CURRENCY: Balance = 1_000_000_000_000_000_000;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type AccountData = ();
	type AccountId = u64;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = ();
	type Version = ();
}

impl pallet_timestamp::Config for Test {
	type MinimumPeriod = ();
	type Moment = Moment;
	type OnTimestampSet = ();
	type WeightInfo = ();
}

impl SortedMembers<u64> for One {
	fn sorted_members() -> Vec<u64> {
		vec![1]
	}
}

cfg_test_utils::mocks::orml_asset_registry::impl_mock_registry! {
	RegistryMock,
	CurrencyId,
	Balance,
	CustomMetadata
}

parameter_types! {
	pub const PoolPalletId: frame_support::PalletId = cfg_types::ids::POOLS_PALLET_ID;

	/// The index with which this pallet is instantiated in this runtime.
	pub PoolPalletIndex: u8 = <PoolSystem as PalletInfoAccess>::index() as u8;

	#[derive(scale_info::TypeInfo, Eq, PartialEq, Debug, Clone, Copy )]
	pub const MaxTranches: u32 = 5;

	pub const MinUpdateDelay: u64 = 0; // no delay
	pub const ChallengeTime: BlockNumber = 0;
	// Defaults for pool parameters
	pub const DefaultMinEpochTime: u64 = 1;
	pub const DefaultMaxNAVAge: u64 = 24 * 60 * 60;

	// Runtime-defined constraints for pool parameters
	pub const MinEpochTimeLowerBound: u64 = 1;
	pub const MinEpochTimeUpperBound: u64 = 24 * 60 * 60;
	pub const MaxNAVAgeUpperBound: u64 = 24 * 60 * 60;

	#[derive(scale_info::TypeInfo, Eq, PartialEq, Debug, Clone, Copy )]
	pub const MaxTokenNameLength: u32 = 128;

	#[derive(scale_info::TypeInfo, Eq, PartialEq, Debug, Clone, Copy )]
	pub const MaxTokenSymbolLength: u32 = 32;

	pub const PoolDeposit: Balance = 1 * CURRENCY;
}

impl pallet_pool_system::Config for Test {
	type AssetRegistry = RegistryMock;
	type Balance = Balance;
	type ChallengeTime = ChallengeTime;
	type Currency = Balances;
	type CurrencyId = CurrencyId;
	type DefaultMaxNAVAge = DefaultMaxNAVAge;
	type DefaultMinEpochTime = DefaultMinEpochTime;
	type EpochId = PoolEpochId;
	type Investments = Investments;
	type MaxNAVAgeUpperBound = MaxNAVAgeUpperBound;
	type MaxSizeMetadata = MaxSizeMetadata;
	type MaxTokenNameLength = MaxTokenNameLength;
	type MaxTokenSymbolLength = MaxTokenSymbolLength;
	type MaxTranches = MaxTranches;
	type MinEpochTimeLowerBound = MinEpochTimeLowerBound;
	type MinEpochTimeUpperBound = MinEpochTimeUpperBound;
	type MinUpdateDelay = MinUpdateDelay;
	type NAV = FakeNav;
	type PalletId = PoolPalletId;
	type PalletIndex = PoolPalletIndex;
	type ParachainId = ParachainInfo;
	type Permission = Permissions;
	type PoolCreateOrigin = EnsureSigned<u64>;
	type PoolCurrency = PoolCurrency;
	type PoolDeposit = PoolDeposit;
	type PoolId = PoolId;
	type Rate = Rate;
	type RuntimeEvent = RuntimeEvent;
	type Time = Timestamp;
	type Tokens = Tokens;
	type TrancheCurrency = TrancheCurrency;
	type TrancheId = TrancheId;
	type TrancheWeight = TrancheWeight;
	type UpdateGuard = UpdateGuard;
	type WeightInfo = ();
}

pub type Balance = u128;

parameter_types! {
	// Pool metadata limit
	#[derive(scale_info::TypeInfo, Eq, PartialEq, Debug, Clone, Copy )]
	pub const MaxSizeMetadata: u32 = 100;
}

pub struct ModifyPoolMock<T> {
	phantom: PhantomData<T>,
}

impl<T: Config + pallet_pool_registry::Config> PoolMutate<T::AccountId, T::PoolId>
	for ModifyPoolMock<T>
{
	type Balance = T::Balance;
	type CurrencyId = T::CurrencyId;
	type MaxTokenNameLength = T::MaxTokenNameLength;
	type MaxTokenSymbolLength = T::MaxTokenSymbolLength;
	type MaxTranches = T::MaxTranches;
	type PoolChanges =
		PoolChanges<T::Rate, T::MaxTokenNameLength, T::MaxTokenSymbolLength, T::MaxTranches>;
	type Rate = T::Rate;
	type TrancheInput = TrancheInput<T::Rate, T::MaxTokenNameLength, T::MaxTokenSymbolLength>;

	fn create(
		_admin: T::AccountId,
		_depositor: T::AccountId,
		_pool_id: T::PoolId,
		_tranche_inputs: Vec<Self::TrancheInput>,
		_currency: T::CurrencyId,
		_max_reserve: T::Balance,
		_metadata: Option<Vec<u8>>,
	) -> DispatchResult {
		Ok(())
	}

	fn update(
		_pool_id: T::PoolId,
		_changes: Self::PoolChanges,
	) -> Result<UpdateState, DispatchError> {
		Ok(UpdateState::Executed(5))
	}

	fn execute_update(_: T::PoolId) -> Result<u32, DispatchError> {
		todo!()
	}
}

parameter_types! {
	pub const MaxOutstandingCollects: u32 = 10;
}
impl pallet_investments::Config for Test {
	type Accountant = PoolSystem;
	type Amount = Balance;
	type BalanceRatio = Rate;
	type InvestmentId = TrancheCurrency;
	type MaxOutstandingCollects = MaxOutstandingCollects;
	type PreConditions = Always;
	type RuntimeEvent = RuntimeEvent;
	type Tokens = Tokens;
	type WeightInfo = ();
}

pub struct Always;
impl<T> PreConditions<T> for Always {
	type Result = DispatchResult;

	fn check(_: T) -> Self::Result {
		Ok(())
	}
}


impl Config for Test {
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type InterestRate = Rate;
	type MaxSizeMetadata = MaxSizeMetadata;
	type MaxTokenNameLength = ();
	type MaxTokenSymbolLength = ();
	type MaxTranches = ();
	type ModifyPool = ModifyPoolMock<Self>;
	type Permission = PermissionsMock;
	type PoolCreateOrigin = EnsureSigned<u64>;
	type PoolId = u64;
	type Rate = Rate;
	type RuntimeEvent = RuntimeEvent;
	type TrancheId = TrancheId;
	type WeightInfo = ();
}

parameter_types! {
	pub MaxLocks: u32 = 2;
	pub const MaxReserves: u32 = 50;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl orml_tokens::Config for Test {
	type Amount = i64;
	type Balance = Balance;
	type CurrencyHooks = ();
	type CurrencyId = CurrencyId;
	type DustRemovalWhitelist = frame_support::traits::Nothing;
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const NativeToken: CurrencyId = CurrencyId::Native;
}

impl pallet_restricted_tokens::Config for Test {
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type Fungibles = OrmlTokens;
	type NativeFungible = Balances;
	type NativeToken = NativeToken;
	type PreCurrency = cfg_traits::Always;
	type PreExtrTransfer = RestrictedTokens<Permissions>;
	type PreFungibleInspect = pallet_restricted_tokens::FungibleInspectPassthrough;
	type PreFungibleInspectHold = cfg_traits::Always;
	type PreFungibleMutate = cfg_traits::Always;
	type PreFungibleMutateHold = cfg_traits::Always;
	type PreFungibleTransfer = cfg_traits::Always;
	type PreFungiblesInspect = pallet_restricted_tokens::FungiblesInspectPassthrough;
	type PreFungiblesInspectHold = cfg_traits::Always;
	type PreFungiblesMutate = cfg_traits::Always;
	type PreFungiblesMutateHold = cfg_traits::Always;
	type PreFungiblesTransfer = cfg_traits::Always;
	type PreReservableCurrency = cfg_traits::Always;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		Balances: pallet_balances::{Pallet, Storage, Event<T>},
		FakeNav: cfg_test_utils::mocks::nav::{Pallet, Storage},
		Investments: pallet_investments::{Pallet, Call, Storage, Event<T>},
		OrmlTokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		ParachainInfo: parachain_info::{Pallet, Storage},
		Permissions: pallet_permissions::{Pallet, Call, Storage, Event<T>},
		PoolRegistry: pallet_pool_registry::{Pallet, Call, Storage, Event<T>},
		PoolSystem: pallet_pool_system::{Pallet, Call, Storage, Event<T>},
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		Tokens: pallet_restricted_tokens::{Pallet, Call, Event<T>},
	}
);

pub struct PoolCurrency;
impl Contains<CurrencyId> for PoolCurrency {
	fn contains(id: &CurrencyId) -> bool {
		match id {
			CurrencyId::Tranche(_, _) | CurrencyId::Native | CurrencyId::KSM => false,
			_ => true,
		}
	}
}


pub struct UpdateGuard;
impl PoolUpdateGuard for UpdateGuard {
	type Moment = Moment;
	type PoolDetails = PoolDetails<
		CurrencyId,
		TrancheCurrency,
		u32,
		Balance,
		Rate,
		MaxSizeMetadata,
		TrancheWeight,
		TrancheId,
		u64,
	>;
	type ScheduledUpdateDetails =
	ScheduledUpdateDetails<Rate, MaxTokenNameLength, MaxTokenSymbolLength, MaxTranches>;

	fn released(
		pool: &Self::PoolDetails,
		update: &Self::ScheduledUpdateDetails,
		now: Self::Moment,
	) -> bool {
		if now < update.scheduled_time {
			return false;
		}

		// The epoch in which the redemptions were fulfilled,
		// should have closed after the scheduled time already,
		// to ensure that investors had the `MinUpdateDelay`
		// to submit their redemption orders.
		if now < pool.epoch.last_closed {
			return false;
		}

		// There should be no outstanding redemption orders.
		if pool
			.tranches
			.tranches
			.iter()
			.map(|tranche| Investments::redeem_orders(tranche.currency).amount)
			.any(|redemption| redemption != Zero::zero())
		{
			return false;
		}
		return true;
	}
}


parameter_types! {
	pub const One: u64 = 1;
	#[derive(Debug, Eq, PartialEq, scale_info::TypeInfo, Clone)]
	pub const MinDelay: Moment = 0;

	pub const MaxRoles: u32 = u32::MAX;
}
impl pallet_permissions::Config for Test {
	type AdminOrigin = EnsureSignedBy<One, u64>;
	type Editors = frame_support::traits::Everything;
	type MaxRolesPerScope = MaxRoles;
	type Role = Role<TrancheId, Moment>;
	type RuntimeEvent = RuntimeEvent;
	type Scope = PermissionScope<u64, CurrencyId>;
	type Storage = PermissionRoles<TimeProvider<Timestamp>, MinDelay, TrancheId, Moment>;
	type WeightInfo = ();
}


pub struct RestrictedTokens<P>(PhantomData<P>);
impl<P> PreConditions<TransferDetails<u64, CurrencyId, Balance>> for RestrictedTokens<P>
	where
		P: PermissionsT<u64, Scope = PermissionScope<u64, CurrencyId>, Role = Role<TrancheId>>,
{
	type Result = bool;

	fn check(details: TransferDetails<u64, CurrencyId, Balance>) -> bool {
		let TransferDetails {
			send,
			recv,
			id,
			amount: _amount,
		} = details.clone();

		match id {
			CurrencyId::Tranche(pool_id, tranche_id) => {
				P::has(
					PermissionScope::Pool(pool_id),
					send,
					Role::PoolRole(PoolRole::TrancheInvestor(tranche_id, UNION)),
				) && P::has(
					PermissionScope::Pool(pool_id),
					recv,
					Role::PoolRole(PoolRole::TrancheInvestor(tranche_id, UNION)),
				)
			}
			_ => true,
		}
	}
}

// Parameterize balances pallet
parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}


// Implement balances pallet configuration for mock runtime
impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = ();
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

type AccountId = u64;
type PoolId = u64;

pub struct PermissionsMock {}

impl cfg_traits::Permissions<AccountId> for PermissionsMock {
	type Error = sp_runtime::DispatchError;
	type Ok = ();
	type Role = Role;
	type Scope = PermissionScope<PoolId, CurrencyId>;

	fn has(_scope: Self::Scope, _who: AccountId, _role: Self::Role) -> bool {
		true
	}

	fn add(
		_scope: Self::Scope,
		_who: AccountId,
		_role: Self::Role,
	) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn remove(
		_scope: Self::Scope,
		_who: AccountId,
		_role: Self::Role,
	) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

// Test externalities builder
//
// This type is mainly used for mocking storage in tests. It is the type alias
// for an in-memory, hashmap-based externalities implementation.
pub struct TestExternalitiesBuilder {}

// Default trait implementation for test externalities builder
impl Default for TestExternalitiesBuilder {
	fn default() -> Self {
		Self {}
	}
}

pub const SECONDS: u64 = 1000;
pub const START_DATE: u64 = 1640995200;

impl TestExternalitiesBuilder {
	// Build a genesis storage key/value store
	pub fn build(self) -> sp_io::TestExternalities {
		let storage = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap();
		let mut externalities = sp_io::TestExternalities::new(storage);
		externalities.execute_with(|| {
			System::set_block_number(1);
			System::on_initialize(System::block_number());
			Timestamp::on_initialize(System::block_number());
			Timestamp::set(RuntimeOrigin::none(), START_DATE * SECONDS).unwrap();
		});
		externalities
	}
}
