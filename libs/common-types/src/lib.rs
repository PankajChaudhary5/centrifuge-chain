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

// Ensure we're `no_std` when compiling for WebAssembly.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use common_traits::Properties;
use frame_support::scale_info::build::Fields;
use frame_support::scale_info::Path;
use frame_support::scale_info::Type;
use frame_support::sp_runtime::traits::Saturating;
use frame_support::traits::{Get, UnixTime};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::TypeId;
///! Common-types of the Centrifuge chain.
use sp_std::cmp::{Ord, PartialEq, PartialOrd};
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

// Pub exports
pub use tokens::*;

#[cfg(test)]
mod tests;
mod tokens;

/// PoolRole can hold any type of role specific functions a user can do on a given pool.
// NOTE: In order to not carry around the Moment and TranchId all the time, we give it defaults.
//       In case the PoolRole we provide does not match what we expect. I.e. if we change the Moment
//       and/or the TrancheId type in our actual runtimes, then the compiler complains about it anyways.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, TypeInfo, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PoolRole<TrancheId = [u8; 16], Moment = u64> {
	PoolAdmin,
	Borrower,
	PricingAdmin,
	LiquidityAdmin,
	MemberListAdmin,
	RiskAdmin,
	TrancheInvestor(TrancheId, Moment),
}

bitflags::bitflags! {
	/// The current admin roles we support
	#[derive(codec::Encode, codec::Decode,  TypeInfo)]
	pub struct AdminRoles: u32 {
		const POOL_ADMIN = 0b00000001;
		const BORROWER  = 0b00000010;
		const PRICING_ADMIN = 0b00000100;
		const LIQUIDITY_ADMIN = 0b00001000;
		const MEMBER_LIST_ADMIN = 0b00010000;
		const RISK_ADMIN = 0b00100000;
	}
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone, Eq, PartialEq)]
pub struct TrancheInvestors<Now, MinDelay, TrancheId, Moment> {
	info: Vec<TrancheInvestorInfo<TrancheId, Moment>>,
	_phantom: PhantomData<(Now, MinDelay)>,
}

/// The structure that we store in the pallet-permissions storage
/// This here implements trait Properties.
#[derive(Encode, Decode, TypeInfo, Clone, Eq, PartialEq, Debug)]
pub struct PermissionRoles<Now, MinDelay, TrancheId, Moment = u64> {
	admin: AdminRoles,
	tranche_investor: TrancheInvestors<Now, MinDelay, TrancheId, Moment>,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone, Eq, PartialEq)]
pub struct TrancheInvestorInfo<TrancheId, Moment> {
	tranche_id: TrancheId,
	permissioned_till: Moment,
}

impl<Now, MinDelay, TrancheId, Moment> Default
	for TrancheInvestors<Now, MinDelay, TrancheId, Moment>
where
	Now: UnixTime,
	MinDelay: Get<Moment>,
	Moment: From<u64> + PartialEq + PartialOrd + Saturating + Ord,
	TrancheId: PartialEq + PartialOrd,
{
	fn default() -> Self {
		Self {
			info: Vec::default(),
			_phantom: Default::default(),
		}
	}
}

impl<Now, MinDelay, TrancheId, Moment> Default for PermissionRoles<Now, MinDelay, TrancheId, Moment>
where
	Now: UnixTime,
	MinDelay: Get<Moment>,
	Moment: From<u64> + PartialEq + PartialOrd + Saturating + Ord,
	TrancheId: PartialEq + PartialOrd,
{
	fn default() -> Self {
		Self {
			admin: AdminRoles::empty(),
			tranche_investor: TrancheInvestors::<Now, MinDelay, TrancheId, Moment>::default(),
		}
	}
}

/// The implementation of trait Properties for our PermissionsRoles does not care which Moment
/// is passed to the PoolRole::TrancheInvestor(TrancheId, Moment) variant.
/// This UNION shall reflect that and explain to the reader why it is passed here.
pub const UNION: u64 = 0;

impl<Now, MinDelay, TrancheId, Moment> Properties
	for PermissionRoles<Now, MinDelay, TrancheId, Moment>
where
	Now: UnixTime,
	MinDelay: Get<Moment>,
	Moment: From<u64> + PartialEq + PartialOrd + Saturating + Ord + Copy,
	TrancheId: PartialEq + PartialOrd,
{
	type Property = PoolRole<TrancheId, Moment>;
	type Error = ();
	type Ok = ();

	fn exists(&self, property: Self::Property) -> bool {
		match property {
			PoolRole::Borrower => self.admin.contains(AdminRoles::BORROWER),
			PoolRole::LiquidityAdmin => self.admin.contains(AdminRoles::LIQUIDITY_ADMIN),
			PoolRole::PoolAdmin => self.admin.contains(AdminRoles::POOL_ADMIN),
			PoolRole::PricingAdmin => self.admin.contains(AdminRoles::PRICING_ADMIN),
			PoolRole::MemberListAdmin => self.admin.contains(AdminRoles::MEMBER_LIST_ADMIN),
			PoolRole::RiskAdmin => self.admin.contains(AdminRoles::RISK_ADMIN),
			PoolRole::TrancheInvestor(id, _) => self.tranche_investor.contains(id),
		}
	}

	fn empty(&self) -> bool {
		self.admin.is_empty() && self.tranche_investor.is_empty()
	}

	fn rm(&mut self, property: Self::Property) -> Result<(), ()> {
		match property {
			PoolRole::Borrower => Ok(self.admin.remove(AdminRoles::BORROWER)),
			PoolRole::LiquidityAdmin => Ok(self.admin.remove(AdminRoles::LIQUIDITY_ADMIN)),
			PoolRole::PoolAdmin => Ok(self.admin.remove(AdminRoles::POOL_ADMIN)),
			PoolRole::PricingAdmin => Ok(self.admin.remove(AdminRoles::PRICING_ADMIN)),
			PoolRole::MemberListAdmin => Ok(self.admin.remove(AdminRoles::MEMBER_LIST_ADMIN)),
			PoolRole::RiskAdmin => Ok(self.admin.remove(AdminRoles::RISK_ADMIN)),
			PoolRole::TrancheInvestor(id, delta) => self.tranche_investor.remove(id, delta),
		}
	}

	fn add(&mut self, property: Self::Property) -> Result<(), ()> {
		match property {
			PoolRole::Borrower => Ok(self.admin.insert(AdminRoles::BORROWER)),
			PoolRole::LiquidityAdmin => Ok(self.admin.insert(AdminRoles::LIQUIDITY_ADMIN)),
			PoolRole::PoolAdmin => Ok(self.admin.insert(AdminRoles::POOL_ADMIN)),
			PoolRole::PricingAdmin => Ok(self.admin.insert(AdminRoles::PRICING_ADMIN)),
			PoolRole::MemberListAdmin => Ok(self.admin.insert(AdminRoles::MEMBER_LIST_ADMIN)),
			PoolRole::RiskAdmin => Ok(self.admin.insert(AdminRoles::RISK_ADMIN)),
			PoolRole::TrancheInvestor(id, delta) => self.tranche_investor.insert(id, delta),
		}
	}
}

impl<Now, MinDelay, TrancheId, Moment> TrancheInvestors<Now, MinDelay, TrancheId, Moment>
where
	Now: UnixTime,
	MinDelay: Get<Moment>,
	Moment: From<u64> + PartialEq + PartialOrd + Saturating + Ord + Copy,
	TrancheId: PartialEq + PartialOrd,
{
	pub fn empty() -> Self {
		Self::default()
	}

	pub fn is_empty(&self) -> bool {
		self.info.is_empty()
	}

	fn validity(&self, delta: Moment) -> Result<Moment, ()> {
		let now: Moment = Now::now().as_secs().into();
		let min_validity = now.saturating_add(MinDelay::get());
		let req_validity = now.saturating_add(delta);

		if req_validity < min_validity {
			Err(())
		} else {
			Ok(req_validity)
		}
	}

	pub fn contains(&self, tranche: TrancheId) -> bool {
		self.info
			.iter()
			.position(|info| {
				info.tranche_id == tranche && info.permissioned_till >= Now::now().as_secs().into()
			})
			.is_some()
	}

	pub fn remove(&mut self, tranche: TrancheId, delta: Moment) -> Result<(), ()> {
		if let Some(index) = self.info.iter().position(|info| info.tranche_id == tranche) {
			let valid_till = &self.info[index].permissioned_till;
			let now = Now::now().as_secs().into();

			if *valid_till <= now {
				// The account is already invalid. Hence no more grace period
				Err(())
			} else {
				// Ensure that permissioned_till is at least now + min_delay.
				Ok(self.info[index].permissioned_till = self.validity(delta)?)
			}
		} else {
			Err(())
		}
	}

	pub fn insert(&mut self, tranche: TrancheId, delta: Moment) -> Result<(), ()> {
		let validity = self.validity(delta)?;

		if let Some(index) = self.info.iter().position(|info| info.tranche_id == tranche) {
			if self.info[index].permissioned_till > validity {
				Err(())
			} else {
				Ok(self.info[index].permissioned_till = validity)
			}
		} else {
			Ok(self.info.push(TrancheInvestorInfo {
				tranche_id: tranche,
				permissioned_till: validity,
			}))
		}
	}
}

/// A struct we need as the pallets implementing trait Time
/// do not implement TypeInfo. This wraps this and implements everything manually.
#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone)]
pub struct TimeProvider<T>(PhantomData<T>);

impl<T> UnixTime for TimeProvider<T>
where
	T: UnixTime,
{
	fn now() -> core::time::Duration {
		<T as UnixTime>::now()
	}
}

impl<T> TypeInfo for TimeProvider<T> {
	type Identity = ();

	fn type_info() -> Type {
		Type::builder()
			.path(Path::new("TimeProvider", module_path!()))
			.docs(&["A wrapper around a T that provides a trait Time implementation. Should be filtered out."])
			.composite(Fields::unit())
	}
}

/// A representation of a pool identifier that can be converted to an account address
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PoolLocator<PoolId> {
	pub pool_id: PoolId,
}

impl<PoolId> TypeId for PoolLocator<PoolId> {
	const TYPE_ID: [u8; 4] = *b"pool";
}

// Type that indicates a point in time
pub type Moment = u64;