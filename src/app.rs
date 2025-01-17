// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use adw::prelude::*;
use glib::{dgettext, dpgettext2, Object};
use gtk::gio::ActionEntry;
use widgets::UmbrellaPreferencesDialog;

mod widgets;

glib::wrapper! {
    pub struct UmbrellaApplication(ObjectSubclass<imp::UmbrellaApplication>)
        @extends adw::Application, gtk::Application, gtk::gio::Application,
        @implements gtk::gio::ActionGroup, gtk::gio::ActionMap;
}

impl UmbrellaApplication {
    fn setup_actions(&self) {
        let actions = [
            ActionEntry::builder("quit")
                .activate(|app: &UmbrellaApplication, _, _| app.quit())
                .build(),
            ActionEntry::builder("about")
                .activate(|app: &UmbrellaApplication, _, _| {
                    app.show_about_dialog();
                })
                .build(),
            ActionEntry::builder("preferences")
                .activate(|app: &UmbrellaApplication, _, _| {
                    let dialog = UmbrellaPreferencesDialog::default();
                    dialog.present(app.active_window().as_ref());
                })
                .build(),
        ];
        self.add_action_entries(actions);
        self.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    fn show_about_dialog(&self) {
        let dialog = adw::AboutDialog::from_appdata(
            "/de/swsnr/umbrella/de.swsnr.umbrella.metainfo.xml",
            Some(&crate::config::release_notes_version().to_string()),
        );
        dialog.set_version(crate::config::CARGO_PKG_VERSION);

        dialog.add_link(
            &dpgettext2(None, "about-dialog.link.label", "Translations"),
            "https://translate.codeberg.org/engage/de-swsnr-umbrella/",
        );

        dialog.set_developers(&["Sebastian Wiesner https://swsnr.de"]);
        dialog.set_designers(&["Sebastian Wiesner https://swsnr.de"]);
        // Credits for the translator to the current language.
        // Translators: Add your name here, as "Jane Doe <jdoe@example.com>" or "Jane Doe https://jdoe.example.com"
        // Mail address or URL are optional.  Separate multiple translators with a newline, i.e. \n
        dialog.set_translator_credits(&dgettext(None, "translator-credits"));

        dialog.add_acknowledgement_section(
            Some(&dpgettext2(
                None,
                "about-dialog.acknowledgment-section",
                "Helpful services",
            )),
            &[
                "Flathub https://flathub.org/",
                "Open Build Service https://build.opensuse.org/",
                "GitHub actions https://github.com/features/actions",
            ],
        );

        dialog.present(self.active_window().as_ref());
    }
}

impl Default for UmbrellaApplication {
    fn default() -> Self {
        Object::builder()
            .property("application-id", crate::config::APP_ID)
            .property("resource-base-path", "/de/swsnr/umbrella")
            .build()
    }
}

mod imp {
    use std::cell::{Ref, RefCell};

    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use gtk::gio::{Settings, SettingsBackend};

    use crate::config::G_LOG_DOMAIN;

    use super::widgets::UmbrellaApplicationWindow;

    #[derive(Default)]
    pub struct UmbrellaApplication {
        settings: RefCell<Option<Settings>>,
    }

    impl UmbrellaApplication {
        /// Get application settings.
        ///
        /// Panic if settings weren't loaded yet; only call this after `startup`!
        fn settings(&self) -> Ref<Settings> {
            Ref::map(self.settings.borrow(), |v| v.as_ref().unwrap())
        }

        fn create_application_window(&self) -> UmbrellaApplicationWindow {
            glib::debug!("Creating new application window");
            let window = UmbrellaApplicationWindow::new(&*self.obj(), crate::config::APP_ID);
            if crate::config::is_development() {
                window.add_css_class("devel");
            }

            let settings = self.settings();
            settings
                .bind("main-window-width", &window, "default-width")
                .build();
            settings
                .bind("main-window-height", &window, "default-height")
                .build();
            settings
                .bind("main-window-maximized", &window, "maximized")
                .build();
            settings
                .bind("main-window-fullscreen", &window, "fullscreened")
                .build();

            window
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UmbrellaApplication {
        const NAME: &'static str = "UmbrellaApplication";

        type Type = super::UmbrellaApplication;

        type ParentType = adw::Application;
    }

    impl ObjectImpl for UmbrellaApplication {}

    impl ApplicationImpl for UmbrellaApplication {
        /// Start the application.
        ///
        /// Set the default icon name for all Gtk windows.
        fn startup(&self) {
            self.parent_startup();
            let app = self.obj();
            glib::debug!("Application starting");
            app.setup_actions();

            // Load app settings
            self.settings.replace(Some(Settings::new_full(
                &crate::config::schema_source()
                    .lookup(crate::config::APP_ID, true)
                    .unwrap(),
                SettingsBackend::NONE,
                None,
            )));
        }

        /// Activate the application.
        ///
        /// Presents the current active window of the application, or creates a
        /// new application window and presents it, if the application doesn't
        /// have an active window currently.
        fn activate(&self) {
            glib::debug!("Activating application");
            self.parent_activate();

            let window = self
                .obj()
                .active_window()
                .unwrap_or_else(|| self.create_application_window().upcast());
            window.present();
        }
    }

    impl GtkApplicationImpl for UmbrellaApplication {}

    impl AdwApplicationImpl for UmbrellaApplication {}
}
