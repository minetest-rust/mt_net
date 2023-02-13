#![feature(iterator_try_collect)]

pub use mt_ser;

#[cfg(feature = "random")]
pub use generate_random;

#[cfg(feature = "random")]
pub use rand;

#[cfg(feature = "serde")]
pub use serde;

use enumset::{EnumSet, EnumSetType};
use mt_ser::mt_derive;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[cfg(any(feature = "client", feature = "server"))]
use mt_ser::{DefCfg, MtCfg, MtDeserialize, MtSerialize, Utf16};

#[cfg(feature = "random")]
use generate_random::GenerateRandom;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod to_clt;
mod to_srv;

pub use to_clt::*;
pub use to_srv::*;
