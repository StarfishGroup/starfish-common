#[cfg(feature = "encrypt")]
pub mod encrypt;
#[cfg(feature = "encrypt")]
pub use base64;
#[cfg(feature = "encrypt")]
pub use ring;

#[cfg(feature = "env")]
pub mod env;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "log")]
pub use tracing::{self, debug, error, info, instrument, warn};

#[cfg(feature = "orm")]
pub mod orm;
#[cfg(feature = "orm")]
pub use sea_orm;

#[cfg(feature = "uuid")]
pub mod uuid;
#[cfg(feature = "uuid")]
pub use snowflaked;

#[cfg(feature = "web")]
pub use axum;
#[cfg(feature = "web")]
pub use tower_http;
#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "rpc")]
pub mod rpc;
#[cfg(feature = "rpc")]
pub use prost;
#[cfg(feature = "rpc")]
pub use prost_wkt;
#[cfg(feature = "rpc")]
pub use prost_wkt_types;
#[cfg(feature = "rpc")]
pub use tonic;

pub mod runtime;

pub use anyhow::{anyhow, bail, ensure, Error as AnyError, Ok as AnyOk, Result};
pub use async_trait::async_trait;
pub use chrono::{self, prelude::*};
pub use derive_more;
pub use futures::{self, prelude::*};
pub use num_cpus;
pub use rust_decimal::{self, prelude::*};
pub use rust_decimal_macros::{self, dec};
pub use serde;
pub use serde_json::{self, json};
pub use serde_urlencoded;
pub use serde_with;
pub use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    result::Result as StdResult,
    sync::Arc,
    time::Duration as StdDuration,
};
pub use strum;
pub use tokio::{
    self, join, select,
    signal::ctrl_c,
    spawn,
    sync::{
        mpsc::{channel, unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
        Mutex, MutexGuard, RwLock, RwLockReadGuard,
    },
    task::yield_now,
    time::{interval, interval_at, sleep, sleep_until, timeout, timeout_at, Duration, Instant},
};
