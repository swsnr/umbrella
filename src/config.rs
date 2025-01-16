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

/// Get [`CARGO_PKG_VERSION`] parsed.
fn cargo_pkg_version() -> semver::Version {
    semver::Version::parse(CARGO_PKG_VERSION).unwrap()
}

/// Whether this is a development/nightly build.
pub fn is_development() -> bool {
    APP_ID.ends_with(".Devel")
}

/// The version to use for release notes.
///
/// For nightly builds (see [`is_development`]) this returns `next`, otherwise
/// it returns [`CARGO_PKG_VERSION`] but with patch set to 0, and all pre and
/// build parts emptied.
///
/// This follows our versioning policy which uses patch releases only for
/// translation updates.
pub fn release_notes_version() -> semver::Version {
    let mut version = cargo_pkg_version();
    version.patch = 0;
    version.pre = semver::Prerelease::EMPTY;
    version.build = semver::BuildMetadata::EMPTY;
    version
}
