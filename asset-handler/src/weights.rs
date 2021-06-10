// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{Weight, constants::RocksDbWeight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for xpallet_assets.
pub trait WeightInfo {
    fn register_resource_id() -> Weight;
    fn remove_resource_id() -> Weight;
    fn redeem() -> Weight;
    fn deposit() -> Weight;
}

/// Weights for asset_handler using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn register_resource_id() -> Weight {
        0 as Weight
    }
    fn remove_resource_id() -> Weight {
        0 as Weight
    }
    fn redeem() -> Weight {
        0 as Weight
    }
    fn deposit() -> Weight {
        0 as Weight
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn register_resource_id() -> Weight {
        0 as Weight
    }
    fn remove_resource_id() -> Weight {
        0 as Weight
    }
    fn redeem() -> Weight {
        0 as Weight
    }
    fn deposit() -> Weight {
        0 as Weight
    }
}
