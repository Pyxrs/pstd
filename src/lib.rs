pub use chrono as time;
pub use fastrand;
pub use flume as mpmc;
pub use futures;
pub use log;
pub use parking_lot as sync;
pub use rand;
pub use rayon;
pub use relative_path as path;

pub mod prelude {
    pub use chrono::prelude as time;
    pub use closure::closure;
    pub use log::{debug, error, info, trace, warn};
    pub use std::time::{Duration, Instant};
}

/// Synchronous insecure hash types
pub mod ahash {
    pub use ahash::{AHashMap, AHashSet, AHasher};
}

/// Concurrent hash types
pub mod chash {
    pub use scc::{
        Bag as CBag, HashCache as CHashCache, HashIndex as CHashIndex, HashMap as CHashMap,
        HashSet as CHashSet, LinkedEntry as CLinkedEntry, LinkedList as CLinkedList,
        Queue as CQueue, Stack as CStack, TreeIndex as CTreeIndex,
    };
}

pub mod anyhow {
    pub use anyhow::{anyhow, bail, ensure, Chain, Context, Error, Ok};
    pub type AnyResult<T> = anyhow::Result<T>;
}

pub mod asyn {
    pub use futures::*;
    pub use futures::{executor::block_on, future::*};
}

/// Returns input value clamped to the interval `[min, max]`.
#[must_use]
#[inline]
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val > min {
        if val < max {
            val
        } else {
            max
        }
    } else {
        min
    }
}

/// Returns input value clamped to the interval `[min, max]`.
//#[inline]
pub fn clamp_ref<T: PartialOrd>(val: &mut T, min: T, max: T) {
    if *val > min {
        if *val > max {
            *val = max;
        }
    } else {
        *val = min;
    }
}

pub trait ClampRef {
    fn clamp_ref(&mut self, min: Self, max: Self);
}

impl<T: PartialOrd> ClampRef for T {
    fn clamp_ref(&mut self, min: Self, max: Self) {
        clamp_ref(self, min, max);
    }
}

#[cfg(feature = "game")]
pub mod game {
    use std::f32::consts::PI;

    pub fn sun_direction(time_of_day: f32, latitude: f32, time_of_year: f32) -> (f32, f32, f32) {
        // Convert time of day from [0, 1] to [-π, π]
        let solar_time_rads = 2.0 * PI * (time_of_day - 0.5);

        // Convert time of year from [0, 1] to [-0.41, 0.41]
        // offset by 0.25 so declination is 0 at equinoxes
        let declination_rads = (2.0 * PI * (time_of_year - 0.25)).sin() * 23.45f32.to_radians();

        // Convert latitude from [0, 1] to [-π/2, π/2]
        let latitude_rads = PI * (latitude - 0.5);

        // equations adapted from https://stackoverflow.com/questions/8708048/position-of-the-sun-given-time-of-day-latitude-and-longitude
        let zenith_rads = (latitude_rads.sin() * declination_rads.sin()
            + latitude_rads.cos() * declination_rads.cos() * solar_time_rads.cos())
        .acos();

        let mut azimuth_rads = ((latitude_rads.sin() * zenith_rads.cos() - declination_rads.sin())
            / (latitude_rads.cos() * zenith_rads.sin()))
        .acos();

        let elevation_rads = (declination_rads.sin() * latitude_rads.sin()
            + declination_rads.cos() * latitude_rads.cos() * solar_time_rads.cos())
        .asin();

        if solar_time_rads > 0.0f32 {
            azimuth_rads += PI;
        } else {
            azimuth_rads = 3.0 * PI - azimuth_rads;
        }

        let x = azimuth_rads.sin() * elevation_rads.cos();
        let y = elevation_rads.sin();
        let z = azimuth_rads.cos() * elevation_rads.cos();

        (x, y, z)
    }
}
