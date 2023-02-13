#![feature(iterator_try_collect)]

#[cfg(feature = "random")]
pub use generate_random;

#[cfg(feature = "random")]
pub use rand;

#[cfg(feature = "serde")]
pub use serde;

pub use mt_ser;

use enumset::{EnumSet, EnumSetType};
use mt_ser::mt_derive;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[cfg(any(feature = "client", feature = "server"))]
use mt_ser::{DefCfg, DeserializeError, MtCfg, MtDeserialize, MtSerialize, SerializeError, Utf16};

#[cfg(feature = "random")]
use generate_random::GenerateRandom;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod to_clt;
mod to_srv;

pub use to_clt::*;
pub use to_srv::*;
