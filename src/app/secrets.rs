// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum AppSecretService {
    /// The SFTP password to access the SFTP server hosting the repository.
    Sftp,
    /// The key for the restic repository.
    Restic,
}

/// Application secrets.
pub struct AppSecretsStorage {
    schema: secret::Schema,
}

impl From<AppSecretService> for &'static str {
    fn from(value: AppSecretService) -> Self {
        match value {
            AppSecretService::Sftp => "SFTP",
            AppSecretService::Restic => "restic",
        }
    }
}

impl AppSecretsStorage {
    pub fn new_for_id(id: &str) -> Self {
        let attributes = HashMap::from_iter([("service", secret::SchemaAttributeType::String)]);
        let schema = secret::Schema::new(id, secret::SchemaFlags::NONE, attributes);
        Self { schema }
    }

    pub async fn store_password(
        &self,
        label: &str,
        service: AppSecretService,
        password: &str,
    ) -> Result<(), glib::Error> {
        secret::password_store_future(
            Some(&self.schema),
            HashMap::from_iter([("service", service.into())]),
            Some(secret::COLLECTION_DEFAULT),
            label,
            password,
        )
        .await
    }

    pub async fn load_password(
        &self,
        service: AppSecretService,
    ) -> Result<Option<String>, glib::Error> {
        let password = secret::password_lookup_future(
            Some(&self.schema),
            HashMap::from_iter([("service", service.into())]),
        )
        .await?;

        Ok(password.map(String::from))
    }
}
