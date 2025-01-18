// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use gtk::gio::IOErrorEnum;

// TODO: Figure out backend situation.
//
// Obvious choice would be libsecret and libsecret-rs but the latter appears not
// to be maintained anymore, and it's unclear whether anyone can take over.
//
// The proposed alternative is oo7, but that crate has a huge dependency chain
// and adds about 500k lines to the cargo vet audit backlog, with many crates
// neither audited nor trusted by any of the registered audit sources.
//
// We may find ourselves ending up querying the secret portal directly and then
// encrypt secrets ourselves.

#[derive(Default)]
pub struct AppSecrets {
    pub repository_key: String,
    pub sftp_password: String,
}

/// Application secrets.
pub struct AppSecretsStorage {}

impl AppSecretsStorage {
    pub fn new_for_id(_id: &str) -> Self {
        Self {}
    }

    pub async fn store_secrets(&self, _secrets: AppSecrets) -> Result<(), glib::Error> {
        Err(glib::Error::new(
            IOErrorEnum::NotSupported,
            "Not implemented",
        ))
    }

    pub async fn load_secrets(&self) -> Result<AppSecrets, glib::Error> {
        Ok(AppSecrets {
            repository_key: "secret-repo-key".to_owned(),
            sftp_password: "super-secret-best-password".to_owned(),
        })
    }
}
