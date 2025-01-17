// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use glib::{gstr, GStr};
use gtk::gio;

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

/// Whether the app is running in flatpak.
pub fn running_in_flatpak() -> bool {
    std::fs::exists("/.flatpak-info").unwrap_or_default()
}

/// Get the locale directory.
///
/// Return the flatpak locale directory when in
pub fn locale_directory() -> PathBuf {
    if let Some(dir) = std::env::var_os("UMBRELLA_LOCALE_DIR") {
        dir.into()
    } else if running_in_flatpak() {
        "/app/share/locale".into()
    } else {
        "/usr/share/locale".into()
    }
}

/// Get a schema source for this application.
///
/// In a debug build load compiled schemas from the manifest directory, to allow
/// running the application uninstalled.
///
/// In a release build only use the default schema source.
pub fn schema_source() -> gio::SettingsSchemaSource {
    let default = gio::SettingsSchemaSource::default().unwrap();
    if cfg!(debug_assertions) {
        let directory = concat!(env!("CARGO_MANIFEST_DIR"), "/schemas");
        if std::fs::exists(directory).unwrap_or_default() {
            gio::SettingsSchemaSource::from_directory(directory, Some(&default), false).unwrap()
        } else {
            default
        }
    } else {
        default
    }
}
