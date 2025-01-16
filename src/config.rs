// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use glib::{gstr, GStr};

/// The app ID to use.
pub static APP_ID: &GStr = gstr!("de.swsnr.umbrella");

/// The glib log domain of our application logs.
pub const G_LOG_DOMAIN: &str = "Umbrella";

/// The Cargo package verson.
///
/// This provides the full version from `Cargo.toml`.
pub static CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
