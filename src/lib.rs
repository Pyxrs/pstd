pub use chrono as time;
pub use fastrand;
pub use flume as mpmc;
pub use futures;
pub use log;
pub use parking_lot as sync;
pub use rand;
pub use rayon;
pub use scc;

pub mod prelude {
    pub use chrono::prelude as time;
    pub use closure::closure;
    pub use log::{debug, error, info, trace, warn};
    pub use std::time::{Duration, Instant};
}

pub mod anyhow {
    pub use anyhow::{anyhow, bail, ensure, Chain, Context, Error, Ok};
    pub type AnyResult<T> = anyhow::Result<T>;
}

pub mod asyn {
    pub use futures::*;
    pub use futures::{executor::block_on, future::*};
}
