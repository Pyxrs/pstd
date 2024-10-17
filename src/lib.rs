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

pub trait Clamp {
    fn clamp(&mut self, min: Self, max: Self);
}

impl<T: PartialOrd> Clamp for T {
    fn clamp(&mut self, min: Self, max: Self) {
        clamp_ref(self, min, max);
    }
}
