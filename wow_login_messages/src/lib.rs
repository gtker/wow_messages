//! Implementation of the network protocol used between World of Warcraft
//! [authentication/login](https://wowdev.wiki/Login_Packet) servers
//! and clients for game versions 1.0.0 (Vanilla) to 3.3.5 (Wrath of the Lich King).
//! See the [WoWDev Login page](https://wowdev.wiki/Login) for details.
//!
//! Clients will send the
//! [protocol version](all::CMD_AUTH_LOGON_CHALLENGE_Client::protocol_version)
//! they're using. So if the client sends a `protocol_version` of `2` they will be using
//! messages from [`version_2`].
//!
//! In each version module there is an [`opcodes`](version_2::opcodes) module that contains
//! enums for possible client and server messages.
//!
//! Notice that the same game version _may_ use different protocol versions for login/reconnecting.
//! See the
//! [table on the WoWDev wiki](https://wowdev.wiki/CMD_AUTH_LOGON_CHALLENGE_Client#Protocol_Versions)
//! or the table below for further information.
//!
//! | Game Version | Login | Reconnect |
//! | ------------ | ----- | --------- |
//! | `1.1.2.4125` | 2     | 2         |
//! | `1.12.1.5875`| 3     | 2         |
//! | `2.4.3.8606` | 8     | 8         |
//! | `3.3.5.12340`| 8     | 8         |
//!
//! The [`helper`] module contains utility functions for common operations.
//!
//! The message definitions in this crate are auto generated from the `wowm`
//! files in [the `wow_messages` Github repository](https://github.com/gtker/wow_messages).
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! wow_login_messages = { version = "0.1" }
//! ```
//!
//! Or add it with [cargo edit](https://github.com/killercup/cargo-edit):
//! ```bash
//! cargo add wow_login_messages
//! ```
//!
//! And then use the structs from the relevant version module.
//!
//! ## Features
//!
//! Tokio and async-std support are gated behind the `async_tokio` and `async_std` features.
//! Both of these are disabled by default.
//!
//! ## Design Decisions
//!
//! Types have been named the ugly `CMD_SCREAMING_SNAKE_CASE` way because that's
//! what other (mostly C++) emulators call them. This makes it significantly easier
//! to search through other emulators or other documentation.
//!
//! ## Other Work
//!
//! * [vMaNGOS (C++)](https://github.com/vmangos/core/blob/ce164f3eb32c75b244482070fbaf3ada1110e6be/src/realmd/AuthSocket.cpp#L65)
//! and MaNGOS derivatives in general have a relatively complete list of messages for popular versions,
//! but they are not available as a library.
//! * [Ember (C++)](https://github.com/EmberEmu/Ember/blob/418aaac1d32a65384cfb399c97640c1f25afa69c/src/login/grunt/client/LoginChallenge.h#L37)
//! has all messages for 1.12, although some work will probably be required in order to use it as a standalone library.
//! * [Shadowburn (Erlang)](https://gitlab.com/shadowburn/shadowburn/-/blob/ac905fabf56579b3bda6f16689c74f544da043e2/apps/logind/lib/authenticator.ex#L173)
//! has messages for 1.12, although they are not in the form of a library.
//! * [gophercraft (Go)](https://github.com/superp00t/gophercraft/blob/382259f45bc9bfc4209af87ae1cd174d76fd4ce2/auth/AuthLogonChallenge_C.go#L13)
//! seems to have most messages in a library format.
//!

#![doc(html_root_url = "https://docs.rs/wow_login_messages/0.1.0")]
#![forbid(unsafe_code)]
#![allow(
    unused,
    non_camel_case_types,
    non_snake_case,
    clippy::missing_errors_doc
)]
#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::missing_panics_doc,
    clippy::style,
    clippy::missing_const_for_fn,
    clippy::doc_markdown,
    clippy::unseparated_literal_suffix
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

pub mod helper;
pub(crate) mod logon;
pub(crate) mod util;

pub use logon::*;

/// Default used by the auth server.
/// Clients will automatically connect to this when no port is specified in the `realmlist.wtf`.
pub const DEFAULT_PORT: u16 = 3724;

pub trait ServerMessage: ReadableAndWritable {
    const OPCODE: u8;
}

pub trait ClientMessage: ReadableAndWritable {
    const OPCODE: u8;
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait ReadableAndWritable: Sized {
    type Error;
    fn read<R: std::io::Read>(r: &mut R) -> Result<Self, Self::Error>;
    fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error>;
    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W)
        -> Result<(), std::io::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;
}

pub trait ConstantSized: MaximumPossibleSized {
    fn size() -> usize;
}

pub trait VariableSized: MaximumPossibleSized {
    fn size(&self) -> usize;
}

pub trait MaximumPossibleSized {
    fn maximum_possible_size() -> usize;
}
