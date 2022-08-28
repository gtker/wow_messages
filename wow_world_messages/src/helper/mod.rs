use std::fmt::{Display, Formatter};
use std::io;
use std::io::{Error, Read, Write};
use wow_srp::header_crypto::{Decrypter, CLIENT_HEADER_LENGTH};

use crate::errors::ExpectedOpcodeError;
use crate::util::{read_u16_le, read_u32_le, read_u64_le, read_u8_le};
use crate::ClientMessage;
#[cfg(feature = "vanilla")]
pub use expected::*;

#[cfg(feature = "vanilla")]
pub(crate) mod aura_mask;
#[cfg(feature = "vanilla")]
pub(crate) mod expected;
pub(crate) mod guid;
#[cfg(feature = "vanilla")]
pub(crate) mod update_mask;
