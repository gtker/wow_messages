//! `wow_spells`
//!
//! Crate containing spell definitions for World of Warcraft versions 1.12.x (`vanilla`), 2.4.3.8606 (`tbc`), and 3.3.5.x (`wrath`).
//!
//! ## Usage
//!
//! Add the crate with the required features:
//!
//! ```bash
//! cargo add --features 'vanilla tbc wrath' wow_spells
//! ```
//!
//! Each expansion module contains an [`all_spells`](vanilla::all_spells) function that returns a slice to all spells
//! and a [`lookup_spell`](vanilla::lookup_spell) function that searches all available spells.
//! Prefer [`lookup_spell`](vanilla::lookup_spell) over manually searching [`all_spells`](vanilla::all_spells).
//!
//! ```rust
//! let spell_id = 7598; // Increased Critical 2
//!
//! # #[cfg(feature = "vanilla")]
//! if let Some(spell) = wow_spells::vanilla::lookup_spell(spell_id) {
//!     println!("Spell is named '{}'.", spell.spell_name());
//! } else {
//!     println!("Spell not found.");
//! }
//! ```
//!
//! ## Notice
//!
//! This crate contains very large constant arrays which can cause out-of-memory errors during compilation.
//! Try reducing the amount of cores used for compilation if this is the case.
//!
//! ## Auto Generation
//!
//! This crate is partially auto generated through sqlite databases in the
//! [`wow_messages` repository](https://github.com/gtker/wow_messages/).
//!
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Version 2.4.3.8606.
#[cfg(feature = "tbc")]
pub mod tbc;
/// Version 1.12.x.
#[cfg(feature = "vanilla")]
pub mod vanilla;
/// Version 3.3.5.x.
#[cfg(feature = "wrath")]
pub mod wrath;
