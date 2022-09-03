// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_child_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_child_bounties
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/pallet_child_bounties.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_child_bounties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_child_bounties::WeightInfo for WeightInfo<T> {
	// Storage: ChildBounties ParentChildBounties (r:1 w:1)
	// Storage: Bounties Bounties (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ChildBounties ChildBountyCount (r:1 w:1)
	// Storage: ChildBounties ChildBountyDescriptions (r:0 w:1)
	// Storage: ChildBounties ChildBounties (r:0 w:1)
	/// The range of component `d` is `[0, 16384]`.
	fn add_child_bounty(d: u32, ) -> Weight {
		(47_779_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:0)
	// Storage: ChildBounties ChildBounties (r:1 w:1)
	// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	fn propose_curator() -> Weight {
		(13_957_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:0)
	// Storage: ChildBounties ChildBounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		(26_939_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ChildBounties ChildBounties (r:1 w:1)
	// Storage: Bounties Bounties (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn unassign_curator() -> Weight {
		(39_860_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:0)
	// Storage: ChildBounties ChildBounties (r:1 w:1)
	fn award_child_bounty() -> Weight {
		(21_404_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ChildBounties ChildBounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: ChildBounties ParentChildBounties (r:1 w:1)
	// Storage: ChildBounties ChildBountyDescriptions (r:0 w:1)
	fn claim_child_bounty() -> Weight {
		(64_888_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:0)
	// Storage: ChildBounties ChildBounties (r:1 w:1)
	// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: ChildBounties ChildBountyDescriptions (r:0 w:1)
	fn close_child_bounty_added() -> Weight {
		(45_634_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:0)
	// Storage: ChildBounties ChildBounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:1)
	// Storage: ChildBounties ChildBountyDescriptions (r:0 w:1)
	fn close_child_bounty_active() -> Weight {
		(55_335_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
}
