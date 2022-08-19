use crate::{self as pallet_fees, *};
use frame_support::{
	parameter_types,
	traits::{Everything, FindAuthor, SortedMembers},
	ConsensusEngineId, PalletId, RuntimeDebug,
};
use frame_system::EnsureSignedBy;
use sp_core::H256;
use sp_runtime::{
	serde::{Deserialize, Serialize},
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use codec::{Decode, Encode};
use scale_info::TypeInfo;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u64;

// For testing the pallet, we construct a mock runtime.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Fees: pallet_fees::{Pallet, Call, Config<T>, Storage, Event<T>},
		Treasury: pallet_treasury::{Pallet, Call, Storage, Config, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub struct AuthorGiven;

impl FindAuthor<u64> for AuthorGiven {
	fn find_author<'a, I>(_digests: I) -> Option<u64>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		Some(100)
	}
}

impl pallet_authorship::Config for Test {
	type FindAuthor = AuthorGiven;
	type UncleGenerations = ();
	type FilterUncle = ();
	type EventHandler = ();
}

parameter_types! {
	pub const TreasuryPalletId: PalletId = PalletId(*b"treasury");
}

impl pallet_treasury::Config for Test {
	type Currency = Balances;
	type ApproveOrigin = EnsureSignedBy<Admin, u64>;
	type RejectOrigin = EnsureSignedBy<Admin, u64>;
	type Event = ();
	type OnSlash = Treasury;
	type ProposalBond = ();
	type ProposalBondMinimum = ();
	type ProposalBondMaximum = ();
	type SpendPeriod = ();
	type Burn = ();
	type PalletId = TreasuryPalletId;
	type BurnDestination = ();
	type WeightInfo = ();
	type SpendFunds = ();
	type MaxApprovals = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

parameter_types! {
	pub const Admin: u64 = 1;
}

impl SortedMembers<u64> for Admin {
	fn sorted_members() -> Vec<u64> {
		vec![1]
	}
}

#[derive(
	Encode, Decode, Clone, Copy, PartialEq, RuntimeDebug, TypeInfo, Serialize, Deserialize,
)]
pub enum TestFeeKey {
	Key1,
	Key2,
}

impl Config for Test {
	type FeeKey = TestFeeKey;
	type Currency = Balances;
	type Treasury = Treasury;
	type Event = ();
	type FeeChangeOrigin = EnsureSignedBy<Admin, u64>;
	type WeightInfo = ();
}

pub const USER_ACCOUNT: u64 = 2;
pub const USER_INITIAL_BALANCE: u64 = 50;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(USER_ACCOUNT, USER_INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}
