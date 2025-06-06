// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Mocks for the transaction pause module.

#![cfg(test)]

use frame_support::{
    construct_runtime, derive_impl, ord_parameter_types, parameter_types,
    traits::{ConstU128, ConstU64, Everything},
};
use frame_system::EnsureSignedBy;
use sp_core::H256;
use sp_runtime::{traits::IdentityLookup, BuildStorage};

use super::*;

pub type AccountId = u128;
pub type Balance = u128;

pub const ALICE: AccountId = 1;
mod transaction_pause {
    pub use super::super::*;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
    type AccountData = pallet_balances::AccountData<Balance>;
    type AccountId = AccountId;
    type BaseCallFilter = Everything;
    type Block = Block;
    type BlockHashCount = ConstU64<250>;
    type BlockLength = ();
    type BlockWeights = ();
    type DbWeight = ();
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type Nonce = u64;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type PalletInfo = PalletInfo;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type SS58Prefix = ();
    type SystemWeightInfo = ();
    type Version = ();
}

impl pallet_balances::Config for Runtime {
    type AccountStore = System;
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<10>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();

    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type WeightInfo = ();
}

parameter_types! {
  pub const MaxOracleKeyLength: u32 = 100;
  pub const MaxOracleValueLength: u32 = 100;
}

impl pallet_oracle::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxOracleKeyLength = MaxOracleKeyLength;
    type MaxOracleValueLength = MaxOracleValueLength;
    type WeightInfo = ();
}

parameter_types! {
  pub const MaxBytecodeLength: u32 = 3;
  pub const ProgramDepositPerByte: u32 = 5;
  pub const MaxOwnedPrograms: u32 = 5;
  pub const MaxOracleLookups: u32 = 5;
}

impl pallet_programs::Config for Runtime {
    type Currency = ();
    type MaxBytecodeLength = MaxBytecodeLength;
    type ProgramDepositPerByte = ProgramDepositPerByte;
    type MaxOwnedPrograms = MaxOwnedPrograms;
    type MaxOracleLookups = MaxOracleLookups;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}
ord_parameter_types! {
  pub const One: AccountId = 1;
}

impl Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type UpdateOrigin = EnsureSignedBy<One, AccountId>;
    type WeightInfo = ();
}

type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
  pub enum Runtime
  {
    System: frame_system,
    TransactionPause: transaction_pause,
    Balances: pallet_balances,
    ProgramsPallet: pallet_programs,
    Oracle: pallet_oracle,
  }
);

pub struct ExtBuilder;

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap().into()
    }
}
