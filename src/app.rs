// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use glib::Object;
use gtk::{gio::ActionEntry, prelude::*};

pub mod widgets;

glib::wrapper! {
    pub struct UmbrellaApplication(ObjectSubclass<imp::UmbrellaApplication>)
        @extends adw::Application, gtk::Application, gtk::gio::Application,
        @implements gtk::gio::ActionGroup, gtk::gio::ActionMap;
}

impl UmbrellaApplication {
    fn setup_actions(&self) {
        let actions = [ActionEntry::builder("quit")
            .activate(|app: &UmbrellaApplication, _, _| app.quit())
            .build()];
        self.add_action_entries(actions);
        self.set_accels_for_action("app.quit", &["<Control>q"]);
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
    use adw::prelude::*;
    use adw::subclass::prelude::*;

    use crate::config::G_LOG_DOMAIN;

    use super::widgets::UmbrellaApplicationWindow;

    #[derive(Default)]
    pub struct UmbrellaApplication {}

    impl UmbrellaApplication {
        fn create_application_window(&self) -> UmbrellaApplicationWindow {
            glib::debug!("Creating new application window");
            let window = UmbrellaApplicationWindow::new(&*self.obj(), crate::config::APP_ID);
            if crate::config::is_development() {
                window.add_css_class("devel");
            }
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
