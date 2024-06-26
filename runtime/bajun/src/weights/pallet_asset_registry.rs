// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_asset_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-nbnwcyh-project-647-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("trappist-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/trappist-node
// benchmark
// pallet
// --chain=trappist-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_asset_registry
// --extrinsic=*
// --wasm-execution=compiled
// --header=./templates/file_header.txt
// --output=./runtime/trappist/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_asset_registry`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_registry::WeightInfo for WeightInfo<T> {
    /// Storage: `Assets::Asset` (r:1 w:0)
    /// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
    /// Storage: `AssetRegistry::AssetIdLocation` (r:1 w:1)
    /// Proof: `AssetRegistry::AssetIdLocation` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
    /// Storage: `AssetRegistry::AssetLocationId` (r:0 w:1)
    /// Proof: `AssetRegistry::AssetLocationId` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
    fn register_reserve_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `123`
        //  Estimated: `4087`
        // Minimum execution time: 19_271_000 picoseconds.
        Weight::from_parts(19_590_000, 0)
            .saturating_add(Weight::from_parts(0, 4087))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `AssetRegistry::AssetIdLocation` (r:1 w:1)
    /// Proof: `AssetRegistry::AssetIdLocation` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
    /// Storage: `AssetRegistry::AssetLocationId` (r:0 w:1)
    /// Proof: `AssetRegistry::AssetLocationId` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
    fn unregister_reserve_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `107`
        //  Estimated: `4087`
        // Minimum execution time: 15_981_000 picoseconds.
        Weight::from_parts(16_448_000, 0)
            .saturating_add(Weight::from_parts(0, 4087))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
}
