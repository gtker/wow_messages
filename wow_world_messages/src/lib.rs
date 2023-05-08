//! # `wow_world_messages`
//!
//!  Implementation of the network protocol used between World of
//!  Warcraft [game servers and clients](https://wowdev.wiki/World_Packet) for game versions 1.12.x (Vanilla), 2.4.3 (The Burning Crusade), and 3.3.5 (Wrath of the Lich King).
//!  See the [WoWDev Login page](https://wowdev.wiki/Login) for details on getting to this message exchange.
//!
//!  ## Usage
//!
//!  Remove the expansions you don't need from the following command:
//!
//!  ```bash
//!  cargo add wow_world_messages --features 'vanilla tbc wrath'
//!  ```
//!
//!  And then [check out the docs](https://docs.rs/wow_world_messages/latest/).
//!
//!  ## Design Decisions
//!
//!  Types have been named the ugly `CMD_SCREAMING_SNAKE_CASE` way because that's
//!  what other (mostly C++) emulators call them. This makes it significantly easier
//!  to search through other emulators or other documentation.
//!
//!  ## Examples
//!
//!  The [`wow_messages` repo](https://github.com/gtker/wow_messages/tree/main/examples) has a few example applications that showcase the library.
//!
//!  ## Other Work
//!
//!  * [`vMaNGOS` (C++)](https://github.com/vmangos/core)
//!    and `MaNGOS` derivatives in general have a relatively complete list of messages for popular versions,
//!    but they are not available as a library.
//!
//!  ## License
//!
//!  Licensed under either of
//!
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//!  * MIT license
//!    ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//!  at your option.
//!
//!  ## Contribution
//!
//!  Unless you explicitly state otherwise, any contribution intentionally submitted
//!  for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//!  dual licensed as above, without any additional terms or conditions.
#![forbid(unsafe_code)]
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::missing_panics_doc,
    clippy::style,
    clippy::missing_const_for_fn,
    clippy::doc_markdown,
    clippy::unseparated_literal_suffix
)]
#![allow(non_snake_case, clippy::missing_errors_doc)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod errors;
pub(crate) mod helper;
pub(crate) mod manual;
mod traits;
pub(crate) mod util;
#[allow(
    unused,
    clippy::complexity,
    clippy::bool_to_int_with_if,
    dead_code,
    clippy::single_match,
    clippy::large_enum_variant,
    clippy::enum_variant_names,
    clippy::approx_constant,
    clippy::upper_case_acronyms,
    clippy::needless_borrow,
    clippy::derive_partial_eq_without_eq,
    non_camel_case_types
)]
mod world;

pub use traits::*;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use world::*;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use wow_world_base::shared::datetime_vanilla_tbc_wrath::*;
pub use wow_world_base::shared::guid_vanilla_tbc_wrath::Guid;

pub const DEFAULT_PORT: u16 = 8085;
