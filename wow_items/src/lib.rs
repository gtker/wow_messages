//! `wow_items`
//!
//! Crate containing item definitions for World of Warcraft versions 1.12.x (`vanilla`), 2.4.3.8606 (`tbc`), and 3.3.5.x (`wrath`).
//!
//! ## Usage
//!
//! Add the crate with the required features:
//!
//! ```bash
//! cargo add --features 'vanilla tbc wrath' wow_items
//! ```
//!
//! Each expansion module contains an [`all_items`](vanilla::all_items) function that returns a slice to all items
//! and a [`lookup_item`](vanilla::lookup_item) function that searches all available items.
//! Prefer [`lookup_item`](vanilla::lookup_item) over manually searching [`all_items`](vanilla::all_items).
//!
//! ```rust
//! let item_id = 12640; // Lionheart Helm
//!
//! if let Some(item) = wow_items::vanilla::lookup_item(item_id) {
//!     println!("Lionheart has {} strength and {} agility.", item.strength(), item.agility());
//! } else {
//!     println!("Item not found.");
//! }
//!
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
