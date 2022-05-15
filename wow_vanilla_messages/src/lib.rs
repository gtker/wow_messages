#![allow(dead_code)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate core;

use crate::util::read_u8_le;
use std::fmt::{Display, Formatter};
use std::io::{Error, Read, Write};

use wow_srp::header_crypto::{Decrypter, Encrypter};

pub mod helper;
mod traits;
pub(crate) mod util;
mod world;

pub use traits::*;
pub use world::*;

pub use helper::aura_mask::AuraMask;
pub use helper::guid::Guid;
pub use helper::update_mask::UpdateMask;

pub const DEFAULT_PORT: u16 = 8085;
